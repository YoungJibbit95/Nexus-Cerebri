import test from 'node:test';
import assert from 'node:assert/strict';
import { resolve } from 'node:path';
import { readAuthoritySources, checkVersionDeclarations } from './version-authorities.mjs';

const sources = await readAuthoritySources(resolve(import.meta.dirname, '..'));
test('repository declarations and current/legacy fixtures match their authorities', () => {
  checkVersionDeclarations(sources);
});

// Keep the correct value elsewhere: presence-only checks would incorrectly accept these edits.
for (const [name, key, before, after] of [
  ['stale software badge', 'readme', 'software-0.2.0-', 'software-0.1.0-'],
  ['stale software declaration', 'readme', '> **Software:** `0.2.0`', '> **Software:** `0.1.0`'],
  ['stale specification badge', 'readme', 'specification-0.4-', 'specification-0.3-'],
  ['stale specification declaration', 'readme', '> **Specification:** `0.4`', '> **Specification:** `0.3`'],
  ['stale CPIR badge', 'readme', '/CPIR-0.2-', '/CPIR-0.1-'],
  ['stale current CPIR', 'readme', '> **Current CPIR:** `0.2`', '> **Current CPIR:** `0.1`'],
  ['incorrect legacy compatibility', 'readme', '> **Legacy CPIR:** `0.1`', '> **Legacy CPIR:** `0.2`'],
  ['published status badge', 'readme', 'status-unreleased-', 'status-released-'],
  ['published status declaration', 'readme', '> **Release status:** `Unreleased`', '> **Release status:** `Released`'],
  ['publication claim', 'readme', '> **Published release:** `None`', '> **Published release:** `0.2.0`'],
  ['formal RC wording', 'readme', 'The current implementation', 'First public release candidate. The current implementation'],
  ['contradictory prose', 'readme', 'The current implementation', 'Published 0.2.0. The current implementation'],
  ['duplicate authority', 'readme', '> **Software:** `0.2.0`', '> **Software:** `0.2.0`\n> **Software:** `0.1.0`'],
  ['stale footer', 'readme', '<sub>Software 0.2.0', '<sub>Software 0.1.0'],
  ['stale Changelog badge', 'changelog', 'software_target-0.2.0-', 'software_target-0.1.0-'],
  ['contradictory qualification', 'changelog', '**Release qualification:** `Undergoing release qualification`', '**Release qualification:** `Qualified`'],
  ['incorrect current fixture', 'currentFixture', '"minor": 2', '"minor": 1'],
  ['incorrect legacy fixture', 'legacyFixture', '"minor": 1', '"minor": 2'],
  ['schema translation drift', 'cpirDe', '**Current CPIR:** `0.2`', '**Current CPIR:** `0.1`'],
  ['Master header/path drift', 'master', '**Specification version:** 0.4', '**Specification version:** 0.3']
]) {
  test(name + ' is rejected even with correct versions elsewhere', () => {
    assert.ok(sources[key].includes(before), 'mutation must hit its intended field');
    assert.throws(() => checkVersionDeclarations({ ...sources, [key]: sources[key].replace(before, after) }));
  });
}

test('dated historical records do not override current release authority', () => {
  checkVersionDeclarations({
    ...sources,
    changelog: sources.changelog + '\n## 2020-01-01 — Historical record\n**Software target:** `0.1.0`\nNo release is implied.\n'
  });
});
