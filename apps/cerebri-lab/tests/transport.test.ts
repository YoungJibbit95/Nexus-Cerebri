import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { intervalStyle, known, knowledgeLabel, rejectionLabel } from '../src/lib/presentation.ts';
import { parseRequest, parseResult, parseValidation, postJson } from '../src/lib/transport.ts';
import { parseTemporalRequest, parseTemporalResult } from '../src/lib/temporal.ts';
import type { EvidenceField, PlanningResult, RankedCandidate } from '../src/lib/contracts.ts';

const readFixture = (name: string): unknown => JSON.parse(readFileSync(new URL(`../../../examples/${name}.json`, import.meta.url), 'utf8'));
const horizon = { start: '2026-10-01T09:00:00Z', end: '2026-10-01T12:00:00Z' };
function resultFixture(): PlanningResult {
  // Small transport fixture; it is never presented as a planner run in the UI.
  return {
    compilation: null, dependency_graph: { nodes: ['busy', 'new-event'], edges: [], order: ['busy', 'new-event'], issues: [] },
    outcome: 'Solution', assessment: 'BestFound', validation: { state: 'Valid', issues: [] },
    candidates: [{ ranking_features: { schema_version: { major: 0, minor: 1 }, preferred_start_distance_seconds: null, preferred_start_source: null, mutation_count: 1, shift_seconds: 0 }, proposed: { id: 'transport-fixture', source_revision: 1, placements: [{ object_id: 'new-event', range: { start: '2026-10-01T10:00:00Z', end: '2026-10-01T10:30:00Z' } }] }, cost: 0, mutation_count: 1, shifted_seconds: 0, start: '2026-10-01T10:00:00Z', object_id: 'new-event', explanation: [{ reason: 'EarliestTieBreak', cost: 0 }], ordering_key: { preference_distance_seconds: 0, mutation_count: 1, shifted_seconds: 0, start: '2026-10-01T10:00:00Z', object_id: 'new-event' } }],
    conflicts: { rejections: [{ start: horizon.start, reasons: [{ HardConstraint: { constraint: { kind: 'NO_OVERLAP' }, object_id: 'new-event', reason: 'Overlap', evidence: { facts: [], blocking_objects: ['busy'], blocking_occurrences: [] } } }] }] },
    search_space: { horizon: { ...horizon }, granularity: 900, objective: 'transport fixture', evaluated: 2, exhausted: false },
  };
}

test('canonical CPIR parses without dropping unknown transport fields or granting validity', () => {
  const fixture = readFixture('request');
  const request = parseRequest(fixture);
  assert.strictEqual(request, fixture);
  assert.equal(request.schema_version.minor, 2);
  assert.equal(known(request.duration), 1800);
  assert.equal(knowledgeLabel(request.context.objects[0].time), 'MISSING');
});

test('all epistemic states remain distinct and uncertain data does not become known', () => {
  const values: EvidenceField<number>['value'][] = [
    { processing: 'RESOLVED', knowledge: { state: 'KNOWN', data: 0 } },
    { processing: 'RESOLVED', knowledge: { state: 'MISSING' } },
    { processing: 'RESOLVED', knowledge: { state: 'UNKNOWN' } },
    { processing: 'RESOLVED', knowledge: { state: 'UNCERTAIN', data: { value: 1800, confidence: 0.4 } } },
    { processing: 'RESOLVED', knowledge: { state: 'AMBIGUOUS', data: [1800, 3600] } },
    { processing: 'UNRESOLVED' },
  ];
  assert.deepEqual(values.map((value) => {
    const request = parseRequest(readFixture('request'));
    request.duration.value = value;
    parseRequest(request);
    assert.equal(known(request.duration), value === values[0] ? 0 : undefined);
    return knowledgeLabel(request.duration);
  }), ['KNOWN', 'MISSING', 'UNKNOWN', 'UNCERTAIN', 'AMBIGUOUS', 'UNRESOLVED']);
});

test('result guard retains separate outcome and search proof with structured rejection evidence', () => {
  const fixture = resultFixture();
  assert.strictEqual(parseResult(fixture), fixture);
  assert.equal(fixture.assessment, 'BestFound');
  assert.equal(rejectionLabel(fixture.conflicts.rejections[0].reasons[0]), 'Overlap · busy');
  assert.deepEqual(parseValidation({ state: 'InsufficientInformation', issues: [{ RequiredDuration: 'Unknown' }] }).issues, [{ RequiredDuration: 'Unknown' }]);
});

test('ranking wire guards distinguish absent, null and value and reject incompatible imports', () => {
  for (const distance of [undefined, null, 0]) {
    for (const source of [undefined, null, 'ExplicitCurrentRequest']) {
      const result = resultFixture();
      const features = result.candidates[0].ranking_features as unknown as Record<string, unknown>;
      delete features.preferred_start_distance_seconds;
      delete features.preferred_start_source;
      if (distance !== undefined) features.preferred_start_distance_seconds = distance;
      if (source !== undefined) features.preferred_start_source = source;
      const wire: unknown = JSON.parse(JSON.stringify(result));
      if (distance !== undefined && source !== undefined && (distance === null) === (source === null)) assert.deepEqual(parseResult(wire), result);
      else assert.throws(() => parseResult(wire));
    }
  }
  for (const [key, value] of [
    ['schema_version', { major: 1, minor: 1 }], ['schema_version', { major: 0, minor: 2 }],
    ['schema_version', { major: 0, minor: 1, extra: 0 }], ['schema_version', null],
    ['preferred_start_distance_seconds', -1], ['preferred_start_distance_seconds', 0.5],
    ['preferred_start_distance_seconds', '0'], ['preferred_start_source', 'Unknown'],
    ['mutation_count', 0x100000000], ['shift_seconds', -1], ['extra', 0],
  ] as const) {
    const result = resultFixture();
    (result.candidates[0].ranking_features as unknown as Record<string, unknown>)[key] = value;
    assert.throws(() => parseResult(result), key);
  }
  for (const key of Object.keys(resultFixture().candidates[0].ranking_features)) {
    const result = resultFixture();
    delete (result.candidates[0].ranking_features as unknown as Record<string, unknown>)[key];
    assert.throws(() => parseResult(result), key);
  }
});

test('ranking imports reject cross-field disagreements without changing consistent observations', () => {
  for (const distance of [null, 0, 3600]) {
    for (const mutationCount of [0, 1]) {
      const result = resultFixture();
      const candidate = result.candidates[0];
      candidate.ranking_features = {
        schema_version: { major: 0, minor: 1 }, mutation_count: mutationCount, shift_seconds: 900,
        ...(distance === null
          ? { preferred_start_distance_seconds: null, preferred_start_source: null }
          : { preferred_start_distance_seconds: distance, preferred_start_source: 'ExplicitCurrentRequest' as const }),
      };
      candidate.mutation_count = candidate.ordering_key.mutation_count = mutationCount;
      candidate.shifted_seconds = candidate.ordering_key.shifted_seconds = 900;
      candidate.cost = candidate.ordering_key.preference_distance_seconds = distance ?? 0;
      // An additional candidate ensures the guard checks every entry, not only the first.
      result.candidates.push(structuredClone(candidate));
      const wire: unknown = JSON.parse(JSON.stringify(result));
      assert.strictEqual(parseResult(wire), wire);
      assert.deepEqual(wire, result);
      const mutations: [string, (value: RankedCandidate) => void][] = [
        ['feature mutation count', (value) => { value.ranking_features.mutation_count += 1; }],
        ['candidate mutation count', (value) => { value.mutation_count += 1; }],
        ['ordering mutation count', (value) => { value.ordering_key.mutation_count += 1; }],
        ['feature shift', (value) => { value.ranking_features.shift_seconds += 1; }],
        ['candidate shift', (value) => { value.shifted_seconds += 1; }],
        ['ordering shift', (value) => { value.ordering_key.shifted_seconds += 1; }],
        ['candidate distance projection', (value) => { value.cost += 1; }],
        ['ordering distance projection', (value) => { value.ordering_key.preference_distance_seconds += 1; }],
      ];
      if (distance !== null) mutations.push(['feature distance', (value) => { value.ranking_features.preferred_start_distance_seconds = distance + 1; }]);
      for (const [label, mutate] of mutations) {
        const changed = structuredClone(result);
        mutate(changed.candidates[1]);
        assert.throws(() => parseResult(JSON.parse(JSON.stringify(changed))), /ranking_features\..*: inconsistent/, `${label}, distance=${distance}, mutations=${mutationCount}`);
      }
    }
  }
});

test('malformed imported results fail before visualization', () => {
  for (const mutate of [
    (value: PlanningResult) => { value.candidates[0].cost = Number.NaN; },
    (value: PlanningResult) => { value.candidates[0].cost = Number.MAX_SAFE_INTEGER + 1; },
    (value: PlanningResult) => { value.candidates[0].proposed.placements[0].range.end = 'invalid'; },
    (value: PlanningResult) => { value.search_space.horizon.end = value.search_space.horizon.start; },
    (value: PlanningResult) => { value.candidates[0].start = '2026-10-01T10:00:00'; },
  ]) {
    const value = resultFixture(); mutate(value); assert.throws(() => parseResult(value));
  }
  assert.throws(() => parseResult({ candidates: [] }));
  assert.throws(() => parseValidation({ state: 'Valid', issues: null }));
});

test('timeline clipping changes only painted bounds', () => {
  const range = { start: '2026-10-01T08:00:00Z', end: '2026-10-01T13:00:00Z' };
  assert.equal(intervalStyle(range, horizon), 'left:0%;width:100%');
  assert.equal(intervalStyle({ start: '2026-10-01T13:00:00Z', end: '2026-10-01T14:00:00Z' }, horizon), 'left:100%;width:0%');
  assert.equal(range.start, '2026-10-01T08:00:00Z');
});

test('temporal import preserves incomplete coverage and explicit core rejections', () => {
  const request = parseTemporalRequest(readFixture('temporal-request'));
  assert.equal(request.recurrences.length, 1);
  request.coverage = 'Incomplete';
  assert.equal(parseTemporalRequest(request).coverage, 'Incomplete');
  const output = { status: 'Complete', data: { availability: { horizon, coverage: 'Incomplete', busy: [], free: [], unknown: [horizon], trace: [] }, expansions: [] } };
  const result = parseTemporalResult(output);
  assert.strictEqual(result, output);
  assert.ok(result.status === 'Complete');
  assert.deepEqual(result.data.availability.free, []);
  assert.deepEqual(result.data.availability.unknown, [horizon]);
  assert.deepEqual(parseTemporalResult({ status: 'Rejected', data: 'DateLimitExceeded' }), { status: 'Rejected', data: 'DateLimitExceeded' });
  assert.throws(() => parseTemporalResult({ status: 'Complete', data: { availability: { horizon, coverage: 'Complete' } } }));
});

test('transport returns actual server JSON and exposes non-success responses', async () => {
  const original = globalThis.fetch;
  try {
    let body = '';
    globalThis.fetch = async (_url, options) => { body = String(options?.body); return new Response(JSON.stringify(resultFixture()), { status: 200, headers: { 'content-type': 'application/json' } }); };
    const input = readFixture('request');
    assert.deepEqual(await postJson('/v1/plan', input), resultFixture());
    assert.deepEqual(JSON.parse(body), input);
    globalThis.fetch = async () => new Response('invalid CPIR', { status: 422 });
    await assert.rejects(postJson('/v1/plan', {}), /API 422: invalid CPIR/);
    await assert.rejects(postJson('/v1/plan', { oversized: 'x'.repeat(256 * 1024) }), /256 KiB/);
  } finally { globalThis.fetch = original; }
});
