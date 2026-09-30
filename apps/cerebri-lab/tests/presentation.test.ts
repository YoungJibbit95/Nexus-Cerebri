import assert from 'node:assert/strict';
import { after, before, test } from 'node:test';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { createServer } from 'vite';
import type { ViteDevServer } from 'vite';
import { depths, storedDepth } from '../src/lib/depth.ts';
import { createStoryPlayback, STORY_LAST_STAGE } from '../src/lib/motion.ts';
import { firstDifference, runCounts, visibleCandidates } from '../src/lib/planner-presentation.ts';
import { parseRequest } from '../src/lib/transport.ts';
import type { ExplanationMode, PlanningResult, RankedCandidate } from '../src/lib/contracts.ts';

const root = fileURLToPath(new URL('../', import.meta.url));
let server: ViteDevServer;
before(async () => { server = await createServer({ root, server: { middlewareMode: true }, appType: 'custom', ssr: { noExternal: ['svelte'] } }); });
after(async () => { await server?.close(); });
const request = parseRequest(JSON.parse(readFileSync(new URL('../../../examples/request.json', import.meta.url), 'utf8')));
function candidate(start: string, distance: number): RankedCandidate {
  return {
    proposed: { id: 'test-plan-' + distance, source_revision: 1, placements: [{ object_id: 'new-event', range: { start, end: '2026-10-01T12:00:00Z' } }] },
    cost: distance, mutation_count: 1, shifted_seconds: 0, start, object_id: 'new-event', explanation: ['FeasibleWithinScope', 'AnalysisOnly'].map((reason) => ({ reason: reason as 'FeasibleWithinScope' | 'AnalysisOnly', cost: 0 })),
    ordering_key: { preference_distance_seconds: distance, mutation_count: 1, shifted_seconds: 0, start, object_id: 'new-event' },
    ranking_features: { schema_version: { major: 0, minor: 1 }, preferred_start_distance_seconds: distance, preferred_start_source: 'SessionContext', mutation_count: 1, shift_seconds: 0 },
  };
}
const result: PlanningResult = {
  outcome: 'Solution', assessment: 'BestFound', validation: { state: 'Valid', issues: [] },
  candidates: [candidate('2026-10-01T10:45:00Z', 0), candidate('2026-10-01T11:00:00Z', 900)],
  conflicts: { rejections: [{ start: '2026-10-01T09:15:00Z', reasons: [{ HardConstraint: { constraint: 'NoOverlap', object_id: 'new-event', reason: 'Overlap', evidence: { facts: [], blocking_objects: ['busy'], blocking_occurrences: [] } } }] }] },
  search_space: { horizon: request.scope.time_range, granularity: 900, objective: 'Test objective', evaluated: 3, exhausted: false },
  compilation: null, dependency_graph: { nodes: ['new-event', 'busy'], edges: [], order: ['busy', 'new-event'], issues: [] },
};
async function html(component: string, props: Record<string, unknown>, mode: ExplanationMode) {
  const module = await server.ssrLoadModule('/src/components/' + component + '.svelte');
  const { render } = await server.ssrLoadModule('svelte/server');
  const { DEPTH_CONTEXT } = await server.ssrLoadModule('/src/lib/depth.ts');
  return render(module.default, { props, context: new Map([[DEPTH_CONTEXT, () => mode]]) }).body;
}

test('three global depths render the same completed result; Understand hides JSON, Research exposes it', async () => {
  const before = structuredClone(result);
  for (const { mode, label } of depths) {
    const control = await html('DepthControl', { mode, onchange: () => {} }, mode);
    assert.match(control, new RegExp(label));
    assert.match(control, /aria-pressed="true"/);
    const output = await html('PlannerScene', { request, result, selected: 0, onselect: () => {}, mode, runId: 0 }, mode);
    assert.match(output, /3<\/strong>.*?evaluated/s);
    assert.match(output, /1<\/strong>.*?rejected/s);
    assert.match(output, /2<\/strong>.*?feasible/s);
    assert.match(output, /Proposal ≠ execution/);
    assert.match(output, /data-stage="5"/);
    assert.match(output, /Best Found/);
    if (mode === 'Simple') assert.doesNotMatch(output, /<pre/);
    else assert.match(output, /ranking_features/);
    if (mode === 'Research') assert.match(output, /class="json-panel" open/);
    assert.deepEqual(result, before);
  }
  assert.equal(storedDepth('invalid'), 'Simple');
  assert.equal(storedDepth('Research'), 'Research');
  assert.deepEqual(runCounts(result), { evaluated: 3, rejected: 1, feasible: 2 });
});

test('the shared candidate selection is reflected in timeline, comparison and placement inspector', async () => {
  for (const component of ['Timeline', 'ScoreChart', 'CandidateDetail']) {
    const output = await html(component, { request, result, selected: 1, onselect: () => {}, mode: 'Technical', candidate: result.candidates[1], rank: 2 }, 'Technical');
    assert.match(output, /data-selected-candidate="2"/);
    assert.match(output, /11:00/);
  }
  const many = structuredClone(result);
  many.candidates = Array.from({ length: 20 }, (_, i) => candidate('2026-10-01T11:00:00Z', i));
  assert.deepEqual(visibleCandidates(many, 17).map(({ index }) => index), [0, 1, 2, 3, 4, 5, 6, 7, 17]);
});

test('comparison walks tied fields and preserves exact instant identity, including submilliseconds', () => {
  const a = result.candidates[0];
  const b = structuredClone(a);
  assert.equal(firstDifference(a, b), -1);
  b.ordering_key.object_id = 'another'; assert.equal(firstDifference(a, b), 4);
  b.ordering_key.start = '2026-10-01T10:45:00.000000001Z'; assert.equal(firstDifference(a, b), 3);
  b.ordering_key.start = '2026-10-01T12:45:00+02:00'; assert.equal(firstDifference(a, b), 4);
  b.ordering_key.shifted_seconds = 1; assert.equal(firstDifference(a, b), 2);
  b.ordering_key.mutation_count = 2; assert.equal(firstDifference(a, b), 1);
  b.ordering_key.preference_distance_seconds = 900; assert.equal(firstDifference(a, b), 0);
});

test('reduced motion snaps to the same semantic final stage; cancelled callbacks cannot revive an old story', () => {
  const stages: number[] = [];
  const callbacks: (() => void)[] = [];
  const cancelled: unknown[] = [];
  const story = createStoryPlayback((stage) => stages.push(stage), { schedule: (callback) => { callbacks.push(callback); return callbacks.length; }, cancel: (handle) => cancelled.push(handle) });
  story.play();
  const obsolete = callbacks[0];
  callbacks.shift()!();
  story.finish();
  const afterFinish = [...stages];
  obsolete(); callbacks.shift()!();
  assert.deepEqual(stages, afterFinish);
  assert.ok(cancelled.length > 0);
  story.play(true);
  assert.equal(stages.at(-1), STORY_LAST_STAGE);
  assert.equal(callbacks.length, 0);
  story.play();
  while (callbacks.length) callbacks.shift()!();
  assert.deepEqual(stages.slice(-6), [0, 1, 2, 3, 4, 5]);
});

test('preferences highlight returned provenance rather than assuming the explicit-request source', async () => {
  const output = await html('InspectorViews', { view: 'Preferences', request, result, mode: 'Technical', sourceAttached: true }, 'Technical');
  assert.match(output, /class="accent".*?Session Context/s);
  assert.doesNotMatch(output, /class="accent".*?<strong>Explicit Current Request<\/strong>/s);
  const imported = await html('InspectorViews', { view: 'Preferences', request, result, mode: 'Research', sourceAttached: false }, 'Research');
  assert.match(imported, /source preference evidence is not attached/);
  assert.doesNotMatch(imported, /Preference profile/);
});

test('console starts as a small activity drawer and exposes raw output only at deeper levels', async () => {
  const entries = [{ id: 1, at: '2026-09-30T12:00:00Z', source: 'planner', level: 'success', message: 'Completed', data: result }];
  const simple = await html('OutputConsole', { entries, mode: 'Simple', onclear: () => {} }, 'Simple');
  assert.match(simple, /Session activity/); assert.doesNotMatch(simple, /<pre/);
  assert.doesNotMatch(simple, /activity-drawer" open/);
  const research = await html('OutputConsole', { entries, mode: 'Research', onclear: () => {} }, 'Research');
  assert.match(research, /activity-drawer" open/); assert.match(research, /ranking_features/);
});

test('an out-of-horizon supplied preference is labelled rather than painted at a false position', async () => {
  const outside = structuredClone(request);
  outside.preferences.preferences = [{ source: 'ExplicitCurrentRequest', preferred_start: '2026-10-02T10:45:00Z', evidence: [] }];
  const output = await html('Timeline', { request: outside, result: null, selected: 0, onselect: () => {}, mode: 'Simple' }, 'Simple');
  assert.match(output, /outside the horizon/);
  assert.doesNotMatch(output, /class="preference-marker"/);
});

test('Research keeps complete dependency tables beyond the Technical disclosure limit', async () => {
  const graph = { nodes: ['a'], edges: Array.from({ length: 70 }, (_, i) => ({ predecessor: 'a', dependent: 'node-' + i })), order: [], issues: [] };
  const technical = await html('DependencyPanel', { graph, mode: 'Technical' }, 'Technical');
  const research = await html('DependencyPanel', { graph, mode: 'Research' }, 'Research');
  assert.doesNotMatch(technical, /<td>node-69<\/td>/);
  assert.match(research, /<td>node-69<\/td>/);
});

test('completed runs without candidates do not imply a pending or selected proposal', async () => {
  const empty = { ...result, outcome: 'NoSolution', assessment: 'Complete', candidates: [] };
  const output = await html('PlannerScene', { request, result: empty, selected: 0, onselect: () => {}, mode: 'Simple', runId: 0 }, 'Simple');
  assert.match(output, /No feasible proposal/);
  assert.match(output, /No feasible candidates to compare/);
  assert.doesNotMatch(output, /Awaiting proposal|first proposal<\/span>/);
});
