import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';

export const authorityPaths = {
  cargo: 'Cargo.toml',
  master: 'docs/architecture/specifications/master-v0.4.md',
  cpir: 'docs/en/cpir.md',
  cpirDe: 'docs/de/cpir.md',
  types: 'crates/cerebri-types/src/lib.rs',
  changelog: 'CHANGELOG.md',
  readme: 'README.md',
  currentFixture: 'examples/request.json',
  legacyFixture: 'examples/legacy-cpir-0.1.json'
};

export async function readAuthoritySources(root) {
  return Object.fromEntries(await Promise.all(Object.entries(authorityPaths).map(async ([key, path]) =>
    [key, await readFile(resolve(root, path), 'utf8')]
  )));
}

function one(source, pattern, name) {
  const matches = [...source.matchAll(pattern)];
  assert.equal(matches.length, 1, name + ': expected exactly one declaration');
  return matches[0][1];
}

function field(source, name) {
  return one(source, new RegExp('^\\*\\*' + name + ':\\*\\* `([^`]+)`\\r?$', 'gm'), name);
}

function line(source, prefix, expected) {
  assert.deepEqual(source.split(/\r?\n/).filter(value => value.startsWith(prefix)), [expected], prefix);
}

// Existing authorities: Cargo, Master header, schema reference and current Changelog header.
// Historical dated records below the current header deliberately do not define present status.
export function versionAuthorities(sources) {
  const workspace = one(sources.cargo, /^\[workspace\.package\]\r?\n([^]*?)(?=^\[|$(?![^]))/gm, 'workspace.package');
  const softwareVersion = one(workspace, /^version = "([^"]+)"\r?$/gm, 'software version');
  const specVersion = one(sources.master, /^\*\*Specification version:\*\* ([0-9.]+)\\\r?$/gm, 'Master revision');
  assert.ok(authorityPaths.master.endsWith('master-v' + specVersion + '.md'), 'Master path/revision mismatch');
  const cpirVersion = field(sources.cpir, 'Current CPIR');
  const legacyCpirVersion = field(sources.cpir, 'Legacy CPIR');
  for (const version of [cpirVersion, legacyCpirVersion]) {
    assert.match(version, /^\d+\.\d+$/);
    const [major, minor] = version.split('.');
    line(sources.types, '    pub const CPIR_' + major + '_' + minor + ':',
      '    pub const CPIR_' + major + '_' + minor + ': Self = Self { major: ' + major + ', minor: ' + minor + ' };');
  }
  assert.notEqual(cpirVersion, legacyCpirVersion, 'current CPIR must remain distinct from legacy');
  assert.equal(field(sources.cpirDe, 'Current CPIR'), cpirVersion);
  assert.equal(field(sources.cpirDe, 'Legacy CPIR'), legacyCpirVersion);
  const currentChangelog = sources.changelog.split(/^## \d{4}-\d{2}-\d{2} /m)[0];
  assert.equal(field(currentChangelog, 'Software target'), softwareVersion);
  const releaseStatus = field(currentChangelog, 'Release status');
  const qualificationStatus = field(currentChangelog, 'Release qualification');
  const publishedRelease = field(currentChangelog, 'Published release');
  // This branch qualifies an unreleased workspace. Publication requires an explicit later decision.
  assert.equal(releaseStatus, 'Unreleased');
  assert.equal(qualificationStatus, 'Undergoing release qualification');
  assert.equal(publishedRelease, 'None');
  return { softwareVersion, specVersion, cpirVersion, legacyCpirVersion, releaseStatus, qualificationStatus, publishedRelease };
}

export function checkVersionDeclarations(sources) {
  const versions = versionAuthorities(sources);
  const { softwareVersion, specVersion, cpirVersion, legacyCpirVersion, releaseStatus, qualificationStatus, publishedRelease } = versions;
  for (const [label, value] of [
    ['Software', softwareVersion], ['Specification', specVersion], ['Current CPIR', cpirVersion],
    ['Legacy CPIR', legacyCpirVersion], ['Release status', releaseStatus],
    ['Release qualification', qualificationStatus], ['Published release', publishedRelease]
  ]) line(sources.readme, '> **' + label + ':**', '> **' + label + ':** `' + value + '`');
  for (const [label, slug, value, color] of [
    ['Software', 'software', softwareVersion, '0969da'],
    ['Specification', 'specification', specVersion, '8250df'],
    ['CPIR', 'CPIR', cpirVersion, '1f883d'],
    ['Release status', 'status', releaseStatus.toLowerCase(), 'd29922']
  ]) line(sources.readme, '[![' + label + ']',
    '[![' + label + '](https://img.shields.io/badge/' + slug + '-' + value + '-' + color + '?style=for-the-badge)](#)');
  line(sources.readme, '<sub>Software ',
    '<sub>Software ' + softwareVersion + ' · Specification ' + specVersion + ' · CPIR ' + cpirVersion +
    ' (legacy ' + legacyCpirVersion + ') · REST /v1 · Unreleased</sub>');
  const currentChangelog = sources.changelog.split(/^## \d{4}-\d{2}-\d{2} /m)[0];
  for (const [label, slug, value, color] of [
    ['Software Target', 'software_target', softwareVersion, '0969da'],
    ['Specification', 'specification', specVersion, '8250df'],
    ['Status', 'status', releaseStatus.toLowerCase(), 'd29922']
  ]) line(currentChangelog, '![' + label + ']',
    '![' + label + '](https://img.shields.io/badge/' + slug + '-' + value + '-' + color + '?style=flat-square)');
  assert.doesNotMatch(sources.readme, /first public release candidate|\b(?:published|released)\s+(?:software\s+)?v?\d+\.\d+\.\d+/i,
    'README contains a contradictory publication claim');
  for (const [key, version] of [['currentFixture', cpirVersion], ['legacyFixture', legacyCpirVersion]]) {
    const [major, minor] = version.split('.').map(Number);
    assert.deepEqual(JSON.parse(sources[key]).schema_version, { major, minor }, key + ' schema mismatch');
  }
  return versions;
}
