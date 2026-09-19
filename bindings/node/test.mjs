import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { plan } from './index.mjs';
const request = JSON.parse(await readFile(new URL('../../examples/request.json', import.meta.url), 'utf8'));
test('Node bridge calls the Rust planner deterministically', async () => {
  const result = await plan(request);
  assert.equal(result.outcome,'Solution');
  assert.equal(result.assessment,'ProvenOptimal');
  assert.equal(result.candidates.length,7);
  assert.deepEqual(await plan(request),result);
});
test('Node rejects malformed and oversized input', async () => {
  await assert.rejects(plan({}));
  await assert.rejects(plan({ padding:'x'.repeat(256*1024) }),RangeError);
});

