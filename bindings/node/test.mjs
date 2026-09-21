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
  assert.deepEqual(await plan(request),result);
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

