import assert from 'node:assert/strict';
import { before, after, test } from 'node:test';
import { execFileSync, spawn } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { createServer, type ViteDevServer } from 'vite';
import { parseResult, parseRequest } from '../src/lib/transport.ts';
import { parseTemporalResult, type TemporalRequest, type TemporalResult } from '../src/lib/temporal.ts';
import { nominalEvidence, traceSections } from '../src/lib/l3-presentation.ts';
import { blockerIds } from '../src/lib/planner-presentation.ts';
import type { PlanningResult, ExplanationMode } from '../src/lib/contracts.ts';
const root = fileURLToPath(new URL('../../../', import.meta.url));
const fixture = (name: string) => JSON.parse(readFileSync(new URL('../../../examples/' + name, import.meta.url), 'utf8'));
const request: TemporalRequest = fixture('temporal-request.json');
let vite: ViteDevServer, api: ReturnType<typeof spawn>, base: string;
let temporal: TemporalResult, incomplete: TemporalResult, folded: TemporalResult, compiled: PlanningResult, skipped: PlanningResult;
before(async () => {
  const output = execFileSync('cargo', ['build', '-p', 'cerebri-api', '--locked', '--message-format=json'], { cwd: root, encoding: 'utf8', windowsHide: true, timeout: 120000, maxBuffer: 16 * 1024 * 1024 });
  const binary = output.trim().split('\n').map(line => JSON.parse(line)).find(item => item.reason === 'compiler-artifact' && item.target?.name === 'cerebri-api' && item.executable)?.executable;
  assert.ok(binary);
  api = spawn(binary, [], { cwd: root, windowsHide: true, env: { ...process.env, CEREBRI_BIND_ADDR: '127.0.0.1:0' }, stdio: ['ignore', 'pipe', 'pipe'] });
  base = await new Promise<string>((resolve, reject) => {
    let output = ''; const timer = setTimeout(() => reject(new Error('API startup timed out')), 15000);
    api.stdout!.on('data', chunk => { output += chunk; const address = /CEREBRI_LISTEN_ADDR=(127\.0\.0\.1:\d+)/.exec(output)?.[1]; if (address) { clearTimeout(timer); resolve('http://' + address); } });
    api.once('error', error => { clearTimeout(timer); reject(error); });
    api.once('exit', code => { clearTimeout(timer); reject(new Error('API exited ' + code)); });
  });
  async function post(route: string, input: unknown) { const response = await fetch(base + route, { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify(input), signal: AbortSignal.timeout(10000) }); assert.equal(response.status, 200); return response.json(); }
  temporal = parseTemporalResult(await post('/v1/temporal', request));
  incomplete = parseTemporalResult(await post('/v1/temporal', { ...request, coverage: 'Incomplete' }));
  const foldRequest = structuredClone(request); foldRequest.horizon = { start: '2026-10-24T00:00:00Z', end: '2026-10-27T00:00:00Z' }; foldRequest.recurrences[0].start_date = '2026-10-24'; foldRequest.recurrences[0].fold_policy = 'Later';
  folded = parseTemporalResult(await post('/v1/temporal', foldRequest));
  const clipRequest = fixture('planner/recurrence-busy.json'); clipRequest.scope.time_range.start = '2026-10-01T10:30:00Z'; clipRequest.context.temporal.horizon.start = clipRequest.scope.time_range.start;
  compiled = parseResult(await post('/v1/plan', clipRequest)); skipped = parseResult(await post('/v1/plan', fixture('planner/dst-skipped-planning.json')));
  vite = await createServer({ root: fileURLToPath(new URL('../', import.meta.url)), server: { middlewareMode: true }, appType: 'custom', ssr: { noExternal: ['svelte'] } });
});
after(async () => { await vite?.close(); if (api?.exitCode === null && api.signalCode === null) await new Promise<void>(resolve => { api.once('close', () => resolve()); api.kill(); }); });
async function html(name: string, props: Record<string, unknown>, mode: ExplanationMode) {
  const component = await vite.ssrLoadModule('/src/components/' + name + '.svelte');
  const { render } = await vite.ssrLoadModule('svelte/server'); const { DEPTH_CONTEXT } = await vite.ssrLoadModule('/src/lib/depth.ts');
  return render(component.default, { props, context: new Map([[DEPTH_CONTEXT, () => mode]]) }).body;
}
test('Temporal depths explain the same real gap/skip data with manual stages and complete Research evidence', async () => {
  assert.equal(temporal.status, 'Complete'); if (temporal.status !== 'Complete') return;
  const original = structuredClone(temporal);
  for (const mode of ['Simple', 'Technical', 'Research'] as const) {
    const output = await html('TemporalStory', { request, report: temporal.data, mode }, mode);
    assert.match(output, /Temporal explanation steps/); assert.match(output, /SKIPPED/); assert.match(output, /2026-03-29/); assert.match(output, /02:30/);
    assert.equal((output.match(/aria-pressed=/g) ?? []).length, 6);
    if (mode === 'Simple') assert.doesNotMatch(output, /<pre/);
    else { assert.match(output, /Gap: Skip/); assert.match(output, /Fold: Earlier/); assert.match(output, /Nonexistent Local Time/); }
    if (mode === 'Research') assert.match(output, /class="json-panel" open/);
    assert.deepEqual(temporal, original);
  }
});
test('coverage and fold resolution remain actual returned distinctions', async () => {
  assert.equal(incomplete.status, 'Complete'); assert.equal(folded.status, 'Complete');
  if (incomplete.status !== 'Complete' || folded.status !== 'Complete') return;
  assert.equal(incomplete.data.availability.free.length, 0); assert.ok(incomplete.data.availability.unknown.length > 0);
  const availability = await html('AvailabilityTimeline', { availability: incomplete.data.availability }, 'Simple');
  assert.match(availability, /Unknown/); assert.match(availability, /Incomplete coverage/);
  const fold = folded.data.expansions[0].occurrences.find(item => item.resolution === 'LaterFold'); assert.ok(fold);
  const events = nominalEvidence(folded.data.expansions[0], { ...request.recurrences[0], fold_policy: 'Later' });
  assert.equal(events.find(item => item.sequence === fold.sequence)?.occurrence?.resolution, 'LaterFold');
});
test('Compilation uses real materialization, skip reason and distinct full/visible intervals', async () => {
  assert.ok(compiled.compilation); assert.ok(skipped.compilation);
  const occurrence = compiled.compilation.occurrences[0];
  assert.notEqual(occurrence.occurrence.range.start, occurrence.occurrence.visible_range.start);
  const output = await html('CompilationStory', { compiled: compiled.compilation, mode: 'Technical' }, 'Technical');
  assert.match(output, new RegExp(compiled.compilation.occurrences.length + ' identified occurrences'));
  assert.match(output, /horizon clips this occurrence/); assert.match(output, /2026-10-01T10:00:00Z/); assert.match(output, /2026-10-01T10:30:00Z/);
  const skip = await html('CompilationPanel', { compiled: skipped.compilation, mode: 'Research' }, 'Research');
  assert.match(skip, /SKIPPED/); assert.match(skip, /Nonexistent Local Time/); assert.match(skip, /Compiled snapshot \/ complete source data/);
  const simple = await html('CompilationPanel', { compiled: compiled.compilation, mode: 'Simple' }, 'Simple'); assert.doesNotMatch(simple, /<table|<pre/);
});
test('Research exposes every occurrence table row beyond the bounded visual and Technical layers', async () => {
  const many = structuredClone(compiled.compilation!); many.occurrences = Array.from({ length: 110 }, (_, index) => ({ ...many.occurrences[0], id: 'occurrence-' + index }));
  const research = await html('CompilationPanel', { compiled: many, mode: 'Research' }, 'Research');
  assert.match(research, /title="occurrence-109"/);
  const technical = await html('CompilationPanel', { compiled: many, mode: 'Technical' }, 'Technical');
  assert.doesNotMatch(technical, /title="occurrence-109"/);
});
test('Trace uses only attached sections and returned blockers without inventing runtime evidence', async () => {
  const plannerRequest = parseRequest(fixture('planner/recurrence-busy.json'));
  const sections = traceSections(compiled, plannerRequest); assert.ok(sections.some(item => item.label === 'Compilation'));
  assert.equal(traceSections(compiled, null).some(item => item.label === 'CPIR'), false);
  assert.equal(traceSections({ ...compiled, compilation: null }, plannerRequest).some(item => item.label === 'Compilation'), false);
  const index = compiled.conflicts.rejections.findIndex(item => item.reasons.some(reason => typeof reason !== 'string' && 'HardConstraint' in reason)); assert.ok(index >= 0);
  assert.ok(blockerIds(compiled, index).length > 0);
  const output = await html('TraceView', { result: compiled, request: plannerRequest, mode: 'Research', selected: 0, onselect: () => {}, rejected: index, onreject: () => {} }, 'Research');
  assert.match(output, /structural explanation of a completed result/); assert.match(output, /Selected rejection \/ exact evidence/); assert.match(output, /Complete conflict set/); assert.match(output, /Complete search-space data/);
  const simple = await html('TraceView', { result: compiled, request: null, mode: 'Simple', selected: 0, onselect: () => {}, rejected: index, onreject: () => {} }, 'Simple');
  assert.doesNotMatch(simple, /<pre/); assert.match(simple, /source CPIR is not attached/);
});
