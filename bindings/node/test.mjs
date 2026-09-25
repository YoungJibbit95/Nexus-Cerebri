import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { plan } from './index.mjs';
const request = JSON.parse(await readFile(new URL('../../examples/request.json', import.meta.url), 'utf8'));
test('Node bridge calls the Rust planner deterministically', async () => {
  assert.deepEqual(request.schema_version, { major: 0, minor: 2 });
  const result = await plan(request);
  assert.equal(result.outcome,'Solution');
  assert.equal(result.assessment,'ProvenOptimal');
  assert.equal(result.candidates.length,7);
  assert.deepEqual(result.candidates[0].ranking_features, {
    schema_version: { major: 0, minor: 1 }, preferred_start_distance_seconds: null,
    preferred_start_source: null, mutation_count: 1, shift_seconds: 0,
  });
  assert.deepEqual(await plan(request),result);
});

test('Node retains preferred-start presence, provenance and second quantization', async () => {
  for (const [offset, distance] of [['00', 0], ['00.800', 0], ['01', 1]]) {
    const input = structuredClone(request);
    input.preferences.preferences = [
      { source: 'PersonalLearned', preferred_start: '2026-10-01T10:00:00Z', evidence: [] },
      { source: 'ExplicitCurrentRequest', preferred_start: `2026-10-01T11:00:${offset}Z`, evidence: [] },
    ];
    const result = await plan(input);
    assert.equal(result.candidates[0].start, '2026-10-01T11:00:00Z');
    assert.deepEqual(result.candidates[0].ranking_features, {
      schema_version: { major: 0, minor: 1 }, preferred_start_distance_seconds: distance,
      preferred_start_source: 'ExplicitCurrentRequest', mutation_count: 1, shift_seconds: 0,
    });
  }
});

test('Node bridge accepts the explicit CPIR 0.1 legacy fixture', async () => {
  const legacy = JSON.parse(await readFile(new URL('../../examples/legacy-cpir-0.1.json', import.meta.url), 'utf8'));
  assert.deepEqual(legacy.schema_version, { major: 0, minor: 1 });
  const result = await plan(legacy);
  assert.equal(result.outcome, 'Solution');
  assert.equal(result.candidates.length, 7);
});
test('Node rejects malformed and oversized input', async () => {
  await assert.rejects(plan({}));
  await assert.rejects(plan({ padding:'x'.repeat(256*1024) }),RangeError);
});

