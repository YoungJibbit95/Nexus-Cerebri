import assert from 'node:assert/strict';
import { execFileSync, spawn } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';
import { parseRequest, parseResult, parseValidation } from '../src/lib/transport.ts';
import { parseTemporalRequest, parseTemporalResult } from '../src/lib/temporal.ts';
import type { PlanningResult } from '../src/lib/contracts.ts';

const root = fileURLToPath(new URL('../../../', import.meta.url));
const fixture = (path: string): unknown => JSON.parse(readFileSync(new URL(`../../../examples/${path}`, import.meta.url), 'utf8'));
interface Scenario {
  name: string; file: string; route: string;
  expected: { outcome?: string; assessment?: string; candidates?: number; first_start?: string; occurrences?: number; dependency_order?: string[]; graph_issues?: number; compilation?: null; unknown_ranges?: number; skipped?: number; status?: string };
}

test('real Rust API responses satisfy Lab contracts and retain independent scenario expectations', { timeout: 180_000 }, async (t) => {
  const build = execFileSync('cargo', ['build', '-p', 'cerebri-api', '--locked', '--message-format=json'], {
    cwd: root, encoding: 'utf8', windowsHide: true, timeout: 120_000, maxBuffer: 16 * 1024 * 1024,
  });
  const artifacts = build.trim().split('\n').map((line) => JSON.parse(line) as { reason: string; executable?: string; target?: { name: string } });
  const executable = artifacts.reverse().find((item) => item.reason === 'compiler-artifact' && item.target?.name === 'cerebri-api' && item.executable)?.executable;
  assert.ok(executable, 'cargo must report the actual API executable');
  const server = spawn(executable, [], { cwd: root, windowsHide: true, env: { ...process.env, CEREBRI_BIND_ADDR: '127.0.0.1:0' }, stdio: ['ignore', 'pipe', 'pipe'] });
  t.after(async () => {
    if (server.exitCode === null && server.signalCode === null) {
      await new Promise<void>((resolve) => { server.once('close', () => resolve()); server.kill(); });
    }
  });
  const base = await new Promise<string>((resolve, reject) => {
    let output = '';
    let errors = '';
    const timer = setTimeout(() => reject(new Error(`API startup timed out: ${errors}`)), 15_000);
    server.stderr.on('data', (chunk: Buffer) => { errors = (errors + chunk.toString()).slice(-4000); });
    server.stdout.on('data', (chunk: Buffer) => {
      output += chunk.toString();
      const address = /CEREBRI_LISTEN_ADDR=(127\.0\.0\.1:\d+)/.exec(output)?.[1];
      if (address) { clearTimeout(timer); resolve(`http://${address}`); }
    });
    server.once('error', (error) => { clearTimeout(timer); reject(error); });
    server.once('exit', (code) => { clearTimeout(timer); reject(new Error(`API exited ${code}: ${errors}`)); });
  });
  async function post(path: string, input: unknown): Promise<unknown> {
    const response = await fetch(`${base}${path}`, { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify(input), signal: AbortSignal.timeout(10_000) });
    assert.equal(response.status, 200, `${path}: ${await response.clone().text()}`);
    return response.json();
  }

  await t.test('legacy CPIR preserves the absence of a compilation claim and explicit ranking fields', async () => {
    const request = parseRequest(fixture('request.json'));
    const result = parseResult(await post('/v1/plan', request));
    assert.equal(result.outcome, 'Solution');
    assert.equal(result.compilation, null);
    assert.ok(result.candidates.length > 0);
    assert.equal(parseValidation(await post('/v1/validate', request)).state, 'Valid');
    assert.deepEqual(result.dependency_graph.issues, []);
    const candidate = result.candidates[0];
    assert.equal(candidate.ordering_key.preference_distance_seconds, candidate.cost);
    for (const mutate of [
      (value: any) => { delete value.compilation; },
      (value: any) => { delete value.dependency_graph; },
      (value: any) => { delete value.candidates[0].ordering_key; },
      (value: any) => { value.candidates[0].ordering_key.shifted_seconds = '0'; },
      (value: any) => { value.dependency_graph.edges = [{ predecessor: 'a' }]; },
      (value: any) => { value.dependency_graph.issues = [{ Cycle: { members: [], edges: [{}] } }]; },
      (value: any) => { value.validation.issues = [null]; },
      (value: any) => { value.validation.issues = ['InventedIssue']; },
      (value: any) => { value.validation.issues = [{ RequiredTime: { object_id: 'busy', reason: 5 } }]; },
      (value: any) => { value.validation.issues = [{ Compilation: { Temporal: 'InventedError' } }]; },
      (value: any) => { value.validation.issues = [{ Dependency: { Cycle: { members: ['busy'], edges: [null] } } }]; },
      (value: any) => { value.candidates[0].ordering_key.object_id = ''; },
    ]) {
      const changed = structuredClone(result); mutate(changed);
      assert.throws(() => parseResult(changed));
    }
  });

  await t.test('DST diagnostic and incomplete coverage come from the real temporal core', async () => {
    const request = parseTemporalRequest(fixture('temporal-request.json'));
    const result = parseTemporalResult(await post('/v1/temporal', request));
    assert.equal(result.status, 'Complete');
    assert.ok(result.status === 'Complete');
    assert.deepEqual(result.data.expansions[0].skipped, [{ sequence: 2, date: '2026-03-29', reason: 'NonexistentLocalTime' }]);
    request.coverage = 'Incomplete';
    const incomplete = parseTemporalResult(await post('/v1/temporal', request));
    assert.ok(incomplete.status === 'Complete');
    assert.deepEqual(incomplete.data.availability.free, []);
    assert.ok(incomplete.data.availability.unknown.length > 0);
  });

  const manifest = fixture('planner/manifest.json') as { scenarios: Scenario[] };
  const results = new Map<string, PlanningResult>();
  for (const scenario of manifest.scenarios) {
    await t.test(`CPIR → API → Core → typed Lab: ${scenario.name}`, async () => {
      const input = fixture(`planner/${scenario.file}`);
      const expected = scenario.expected;
      // New manifest assertions cannot silently be omitted by this client.
      for (const key of Object.keys(expected)) assert.ok(['outcome', 'assessment', 'candidates', 'first_start', 'occurrences', 'dependency_order', 'graph_issues', 'compilation', 'unknown_ranges', 'skipped', 'status'].includes(key), `Uncovered expectation: ${key}`);
      if (scenario.route === '/v1/temporal') {
        const result = parseTemporalResult(await post(scenario.route, parseTemporalRequest(input)));
        assert.equal(result.status, expected.status);
        assert.ok(result.status === 'Complete');
        assert.equal(result.data.expansions.reduce((total, expansion) => total + expansion.skipped.length, 0), expected.skipped);
      } else {
        assert.equal(scenario.route, '/v1/plan');
        const request = parseRequest(input);
        const result = parseResult(await post(scenario.route, request));
        results.set(scenario.name, result);
        assert.equal(result.outcome, expected.outcome);
        assert.equal(result.assessment, expected.assessment);
        assert.equal(result.candidates.length, expected.candidates);
        assert.deepEqual(parseValidation(await post('/v1/validate', request)), result.validation);
        if (expected.first_start !== undefined) assert.equal(Date.parse(result.candidates[0].start), Date.parse(expected.first_start));
        if (expected.occurrences !== undefined) assert.equal(result.compilation?.occurrences.length, expected.occurrences);
        if (expected.dependency_order !== undefined) assert.deepEqual(result.dependency_graph.order, expected.dependency_order);
        if (expected.graph_issues !== undefined) assert.equal(result.dependency_graph.issues.length, expected.graph_issues);
        if ('compilation' in expected) assert.equal(result.compilation, expected.compilation);
        if (expected.unknown_ranges !== undefined) {
          assert.equal(result.compilation?.availability.unknown.length, expected.unknown_ranges);
          assert.deepEqual(result.compilation?.availability.free, []);
        }
        if (expected.skipped !== undefined) assert.equal(result.compilation?.series.reduce((total, item) => total + item.expansion.skipped.length, 0), expected.skipped);
      }
    });
  }

  await t.test('compiler, occurrence evidence and cycle contract drift is rejected before rendering', () => {
    const recurrence = results.get('recurrence-busy');
    assert.ok(recurrence?.compilation);
    assert.equal(recurrence.compilation.series[0].series.state.kind, 'Existing');
    assert.equal(recurrence.compilation.occurrences[0].provenance, 'INTEGRATION_FACT');
    const recurrenceBlocker = recurrence.conflicts.rejections.flatMap((rejection) => rejection.reasons).find((reason) => typeof reason !== 'string' && 'HardConstraint' in reason && reason.HardConstraint.evidence.blocking_occurrences.length > 0);
    assert.ok(recurrenceBlocker && typeof recurrenceBlocker !== 'string' && 'HardConstraint' in recurrenceBlocker);
    assert.ok(recurrenceBlocker.HardConstraint.evidence.blocking_occurrences.includes(recurrence.compilation.occurrences[0].id));
    for (const mutate of [
      (value: any) => { delete value.compilation.horizon; },
      (value: any) => { value.compilation.source_revision = '1'; },
      (value: any) => { value.compilation.series[0].series.state.kind = 'Remote'; },
      (value: any) => { value.compilation.series[0].series.rule.gap_policy = 'Assume'; },
      (value: any) => { delete value.compilation.occurrences[0].series_id; },
      (value: any) => { value.compilation.occurrences[0].provenance = 'PROVIDER'; },
      (value: any) => { delete value.compilation.occurrences[0].occurrence.visible_range; },
      (value: any) => { value.compilation.occurrences[0].occurrence.resolution = 'Guessed'; },
      (value: any) => { value.compilation.availability.unknown = null; },
      (value: any) => { value.compilation.availability.coverage = 'Assumed'; },
      (value: any) => { value.compilation.occurrences[0].id = ''; },
      (value: any) => { value.compilation.occurrences[0].id = 'bad identity'; },
      (value: any) => { value.compilation.occurrences[0].id = 'x'.repeat(129); },
      (value: any) => { value.compilation.occurrences[0].evidence = [null]; },
      (value: any) => { value.conflicts.rejections.flatMap((rejection: any) => rejection.reasons).find((reason: any) => reason.HardConstraint).HardConstraint.evidence.blocking_occurrences = [1]; },
    ]) {
      const changed = structuredClone(recurrence); mutate(changed);
      assert.throws(() => parseResult(changed));
    }
    const cycle = results.get('dependency-cycle');
    assert.ok(cycle);
    const issue = cycle.dependency_graph.issues[0];
    assert.ok(typeof issue !== 'string' && 'Cycle' in issue);
    assert.deepEqual(issue.Cycle.members, ['busy', 'new-event']);
    assert.deepEqual(cycle.dependency_graph.order, []);
    const dst = structuredClone(results.get('dst-skipped-planning'));
    assert.ok(dst?.compilation);
    (dst.compilation.series[0].expansion.skipped[0] as { reason: string }).reason = 'Unknown';
    assert.throws(() => parseResult(dst));
  });
});
