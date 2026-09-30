import assert from 'node:assert/strict';
import { test } from 'node:test';
import { createRequire } from 'node:module';
import { EventEmitter } from 'node:events';
import { PassThrough } from 'node:stream';
import { mkdtempSync, mkdirSync, writeFileSync, rmSync, existsSync, readFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
const require = createRequire(import.meta.url);
const { executable, layout, verifyLayout, listenOrigin, allowedNavigation, OwnedCore } = require('../desktop/runtime.cjs');
const { stage } = require('../desktop/stage.cjs');

function resourceFixture() {
  const directory = mkdtempSync(join(tmpdir(), 'cerebri desktop test '));
  const paths = layout(directory);
  mkdirSync(join(paths.lab, 'assets'), { recursive: true });
  mkdirSync(join(paths.root, 'api'), { recursive: true });
  writeFileSync(paths.api, 'test executable', { mode: 0o755 });
  writeFileSync(join(paths.lab, 'index.html'), '<html>test</html>');
  return { directory, paths, cleanup: () => rmSync(directory, { recursive: true, force: true }) };
}
class Child extends EventEmitter {
  stdout = new PassThrough(); stderr = new PassThrough(); pid = 123456;
  exitCode: number | null = null; signalCode: string | null = null;
  kills: string[] = [];
  kill(signal: string) { this.kills.push(signal); this.signalCode = signal; queueMicrotask(() => { this.emit('exit', null, signal); this.emit('close', null, signal); }); return true; }
}
const healthy = async () => ({ ok: true, json: async () => ({ status: 'ok', software_version: '0.2.0' }) });

test('desktop address parsing accepts only the complete printed dynamic IPv4 loopback contract', () => {
  assert.equal(listenOrigin('CEREBRI_LISTEN_ADDR=127.0.0.1:43210\r'), 'http://127.0.0.1:43210');
  for (const value of ['127.0.0.1:3000', 'CEREBRI_LISTEN_ADDR=0.0.0.0:3000', 'CEREBRI_LISTEN_ADDR=localhost:3000', 'CEREBRI_LISTEN_ADDR=127.0.0.1:0', 'CEREBRI_LISTEN_ADDR=127.0.0.1:65536', 'CEREBRI_LISTEN_ADDR=127.0.0.1:3000/other']) assert.throws(() => listenOrigin(value));
});
test('runtime resources are absolute and platform-native, including paths with spaces', () => {
  assert.equal(executable('win32'), 'cerebri-api.exe');
  assert.equal(executable('linux'), 'cerebri-api'); assert.equal(executable('darwin'), 'cerebri-api');
  assert.throws(() => executable('unknown')); assert.throws(() => layout('relative/resources'));
  const fixture = resourceFixture();
  try {
    verifyLayout(fixture.paths);
    rmSync(join(fixture.paths.lab, 'index.html')); assert.throws(() => verifyLayout(fixture.paths), /Lab assets are missing/);
    writeFileSync(join(fixture.paths.lab, 'index.html'), 'Lab');
    rmSync(fixture.paths.api); assert.throws(() => verifyLayout(fixture.paths), /executable is missing/);
  }
  finally { fixture.cleanup(); }
});
test('navigation accepts only the selected Lab origin and exact controlled pages', () => {
  const origin = 'http://127.0.0.1:43210', pages = new Set(['data:text/html,controlled']);
  assert.equal(allowedNavigation(origin + '/lab/#workspace', origin, pages), true);
  assert.equal(allowedNavigation('data:text/html,controlled', origin, pages), true);
  for (const target of ['http://127.0.0.1:3000/lab/', origin + '/v1/plan', origin + '/lab/../health', origin + '/laboratory', 'http://user@127.0.0.1:43210/lab/', 'https://example.com/', 'data:text/html,other']) assert.equal(allowedNavigation(target, origin, pages), false);
});
test('owned startup parses split stdout, verifies health and stops its one child', async () => {
  const fixture = resourceFixture(), child = new Child();
  let spawned = 0;
  const core = new OwnedCore(fixture.paths, { spawnProcess: (binary: string, args: string[], options: { shell: boolean; windowsHide: boolean; env: Record<string, string> }) => {
    spawned++; assert.equal(binary, fixture.paths.api); assert.deepEqual(args, []); assert.equal(options.shell, false); assert.equal(options.windowsHide, true);
    assert.equal(options.env.CEREBRI_BIND_ADDR, '127.0.0.1:0'); assert.equal(options.env.CEREBRI_LAB_DIST, fixture.paths.lab);
    queueMicrotask(() => { child.stdout.write('CEREBRI_LISTEN_'); child.stdout.write('ADDR=127.0.0.1:45678\n'); }); return child;
  }, fetchHealth: async (url: string) => { assert.equal(url, 'http://127.0.0.1:45678/health'); return healthy(); } });
  try {
    const [a, b] = await Promise.all([core.start(), core.start()]); assert.equal(a, b); assert.equal(spawned, 1); assert.equal(core.state, 'ready');
    await core.stop(); await core.stop(); assert.equal(core.state, 'stopped'); assert.deepEqual(child.kills, ['SIGTERM']);
  } finally { fixture.cleanup(); }
});
test('startup timeout and malformed address terminate the child and fail closed', async () => {
  for (const output of ['', 'CEREBRI_LISTEN_ADDR=0.0.0.0:1234\n']) {
    const fixture = resourceFixture(), child = new Child();
    const core = new OwnedCore(fixture.paths, { startupMs: 20, spawnProcess: () => { queueMicrotask(() => child.stdout.write(output)); return child; }, fetchHealth: healthy });
    try { await assert.rejects(core.start(), /timed out|invalid loopback/); assert.equal(core.state, 'failed'); assert.equal(core.child, null); assert.deepEqual(child.kills, ['SIGTERM']); }
    finally { fixture.cleanup(); }
  }
});
test('health failure, unexpected exit and cancellation retain diagnostics and clean up', async () => {
  const fixture = resourceFixture(), child = new Child(); let crash = '';
  const core = new OwnedCore(fixture.paths, { spawnProcess: () => { queueMicrotask(() => child.stdout.write('CEREBRI_LISTEN_ADDR=127.0.0.1:43210\n')); return child; }, fetchHealth: healthy, onCrash: (error: Error) => crash = error.message });
  try {
    await core.start(); child.stderr.write('diagnostic'); child.exitCode = 2; child.emit('exit', 2, null);
    assert.match(crash, /unexpectedly/); assert.equal(core.origin, null); assert.equal(core.stderr, 'diagnostic'); await core.stop();
    const bad = new Child();
    const failed = new OwnedCore(fixture.paths, { spawnProcess: () => { queueMicrotask(() => bad.stdout.write('CEREBRI_LISTEN_ADDR=127.0.0.1:43210\n')); return bad; }, fetchHealth: async () => ({ ok: false, json: async () => ({}) }) });
    await assert.rejects(failed.start(), /health check failed/); assert.equal(failed.child, null);
    const pending = new Child(), cancelled = new OwnedCore(fixture.paths, { spawnProcess: () => pending, fetchHealth: healthy });
    const starting = cancelled.start(); const assertion = assert.rejects(starting, /cancelled/); await cancelled.stop(); await assertion;
    assert.equal(cancelled.child, null);
  } finally { fixture.cleanup(); }
});
test('staging copies a deterministic native API and Lab layout and rejects missing inputs', () => {
  const directory = mkdtempSync(join(tmpdir(), 'cerebri staging test '));
  const labRoot = join(directory, 'apps/cerebri-lab'), binary = join(directory, executable());
  mkdirSync(join(labRoot, 'dist/assets'), { recursive: true }); mkdirSync(join(directory, 'examples'), { recursive: true });
  writeFileSync(join(labRoot, 'dist/index.html'), 'Lab'); writeFileSync(join(labRoot, 'dist/assets/test.js'), 'asset');
  writeFileSync(join(directory, 'examples/request.json'), '{}'); writeFileSync(join(directory, 'examples/temporal-request.json'), '{}');
  try {
    assert.throws(() => stage({ labRoot, apiPath: binary }), /release cerebri-api/);
    writeFileSync(binary, 'binary');
    const paths = stage({ labRoot, apiPath: binary });
    assert.equal(readFileSync(paths.api, 'utf8'), 'binary'); assert.equal(readFileSync(join(paths.lab, 'assets/test.js'), 'utf8'), 'asset');
    writeFileSync(join(paths.root, 'obsolete'), 'remove'); stage({ labRoot, apiPath: binary });
    assert.equal(existsSync(join(paths.root, 'obsolete')), false);
    assert.ok(existsSync(join(paths.root, 'smoke-inputs.json')));
  } finally { rmSync(directory, { recursive: true, force: true }); }
});
