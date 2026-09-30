import test from 'node:test';
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { copyFile, mkdtemp, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { CerebriBridgeError, describeIntegration, plan, suggest } from './index.mjs';

const fixture = JSON.parse(await readFile(new URL('../../examples/integration/suggestion.json', import.meta.url), 'utf8'));
const hasCode = code => error => error instanceof CerebriBridgeError && error.code === code
  && !error.message.includes('private-') && error.cause === undefined;

test('integration manifest and result agree with the real Rust bridge', async () => {
  const manifest = await describeIntegration();
  assert.deepEqual(manifest.integration_version, { major: 0, minor: 1 });
  assert.deepEqual(manifest.cpir_schema_version, { major: 0, minor: 2 });
  assert.equal(manifest.execution_supported, false);
  const actual = await suggest(fixture);
  assert.equal(actual.status, 'planned');
  assert.equal(actual.request_id, fixture.request.request_id);
  assert.equal(actual.trace_id, fixture.request.trace_id);
  assert.equal(actual.context_revision, fixture.request.context.revision);
  assert.deepEqual(actual.result, await plan(fixture.request));
  assert.equal(actual.result.outcome, 'Solution');
  assert.deepEqual(actual.result.candidates.map(c => c.start), [
    '2026-10-01T10:00:00Z', '2026-10-01T10:15:00Z', '2026-10-01T10:30:00Z',
    '2026-10-01T10:45:00Z', '2026-10-01T11:00:00Z', '2026-10-01T11:15:00Z', '2026-10-01T11:30:00Z',
  ]);
});

test('integration distinguishes rejection, missing coverage, no solution and bounded search', async () => {
  for (const [path, replacement, code] of [
    [['integration_version', 'minor'], 99, 'unsupported_integration_version'],
    [['request', 'operation'], 'CREATE', 'unsupported_operation'],
    [['request', 'context', 'temporal'], null, 'temporal_coverage_required'],
  ]) {
    const input = structuredClone(fixture);
    let target = input;
    for (const key of path.slice(0, -1)) target = target[key];
    target[path.at(-1)] = replacement;
    assert.deepEqual(await suggest(input), { integration_version: { major: 0, minor: 1 }, status: 'rejected', code });
  }
  const incomplete = structuredClone(fixture);
  incomplete.request.context.temporal.coverage = 'Incomplete';
  const unknown = await suggest(incomplete);
  assert.equal(unknown.status, 'planned');
  assert.equal(unknown.result.outcome, 'InsufficientInformation');
  assert.deepEqual(unknown.result.candidates, []);
  assert.deepEqual(unknown.result.compilation.availability.free, []);
  const busy = structuredClone(fixture);
  busy.request.context.objects[1].time.value.knowledge.data.end = '2026-10-01T12:00:00Z';
  const none = await suggest(busy);
  assert.equal(none.result.outcome, 'NoSolution');
  assert.equal(none.result.assessment, 'Complete');
  const bounded = structuredClone(fixture);
  bounded.request.budget.max_candidates = 6;
  const partial = await suggest(bounded);
  assert.equal(partial.result.outcome, 'Solution');
  assert.equal(partial.result.assessment, 'BestFound');
  assert.equal(partial.result.search_space.exhausted, false);
});

test('new client bounds input and rejects invalid host configuration', async () => {
  assert.equal((await suggest({})).code, 'invalid_request');
  await assert.rejects(suggest(undefined), hasCode('invalid_request'));
  const cycle = {}; cycle.self = cycle;
  await assert.rejects(suggest(cycle), hasCode('invalid_request'));
  await assert.rejects(suggest({ text: 'x'.repeat(256 * 1024) }), hasCode('request_too_large'));
  for (const timeoutMs of [0, -1, Infinity, 60_001]) {
    await assert.rejects(describeIntegration({ timeoutMs }), hasCode('invalid_options'));
  }
  await assert.rejects(describeIntegration(null), hasCode('invalid_options'));
  await assert.rejects(describeIntegration({ binary: 'absent-cerebri-test-binary' }), hasCode('bridge_unavailable'));
  const controller = new AbortController(); controller.abort();
  await assert.rejects(suggest(fixture, { signal: controller.signal }), hasCode('aborted'));
});

test('transport failures stay bounded and sanitize child output; cancellation kills the child', async t => {
  const directory = await mkdtemp(join(tmpdir(), 'cerebri-integration-test-'));
  const suffix = process.platform === 'win32' ? '.exe' : '';
  const binary = join(directory, 'fixture' + suffix);
  t.after(() => rm(directory, { recursive: true, force: true }));
  execFileSync('rustc', ['--edition=2024', fileURLToPath(new URL('./test-fixtures/bridge.rs', import.meta.url)), '-o', binary], { windowsHide: true });
  for (const [mode, code] of [
    ['malformed', 'invalid_response'], ['failure', 'bridge_failed'], ['oversize', 'response_too_large'],
    ['incompatible', 'incompatible_bridge'], ['mismatch', 'invalid_response'],
  ]) {
    const program = join(directory, mode + suffix);
    await copyFile(binary, program);
    await assert.rejects(suggest(fixture, { binary: program }), hasCode(code));
  }
  const hanging = join(directory, 'hang' + suffix);
  await copyFile(binary, hanging);
  const pidFile = join(directory, 'hang.pid');
  const delay = ms => new Promise(resolve => setTimeout(resolve, ms));
  const waitForPid = async () => {
    for (let tries = 0; tries < 200; tries++) {
      try { return Number(await readFile(pidFile, 'utf8')); } catch { await delay(10); }
    }
    throw new Error('test child did not start');
  };
  const waitForExit = async pid => {
    for (let tries = 0; tries < 200; tries++) {
      try { process.kill(pid, 0); } catch (error) {
        if (error.code === 'ESRCH') return;
        throw error;
      }
      await delay(10);
    }
    throw new Error('bridge did not kill child');
  };
  const controller = new AbortController();
  const aborted = assert.rejects(suggest(fixture, { binary: hanging, signal: controller.signal }), hasCode('aborted'));
  const pid = await waitForPid();
  controller.abort();
  await aborted;
  await waitForExit(pid);
  await rm(pidFile);
  const timedOut = assert.rejects(suggest(fixture, { binary: hanging, timeoutMs: 1000 }), hasCode('timeout'));
  const timedOutPid = await waitForPid();
  await timedOut;
  await waitForExit(timedOutPid);
});
