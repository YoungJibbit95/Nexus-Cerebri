const { spawn, execFileSync } = require('node:child_process');
const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');
const root = path.resolve(__dirname, '../desktop-dist');
let binary;
if (process.platform === 'win32') binary = path.join(root, 'win-unpacked/cerebri-lab.exe');
else if (process.platform === 'linux') binary = path.join(root, 'linux-unpacked/cerebri-lab');
else {
  const directory = fs.readdirSync(root).find((name) => /^mac(?:-|$)/.test(name) && fs.statSync(path.join(root, name)).isDirectory());
  if (!directory) throw new Error('Unpacked macOS application is missing.');
  const bundle = fs.readdirSync(path.join(root, directory)).find((name) => name.endsWith('.app'));
  const contents = path.join(root, directory, bundle, 'Contents');
  const executable = execFileSync('/usr/libexec/PlistBuddy', ['-c', 'Print :CFBundleExecutable', path.join(contents, 'Info.plist')], { encoding: 'utf8' }).trim();
  if (!executable || path.basename(executable) !== executable) throw new Error('Invalid macOS bundle executable.');
  binary = path.join(contents, 'MacOS', executable);
}
const evidence = path.resolve(process.env.CEREBRI_SMOKE_EVIDENCE || path.join(root, 'smoke', process.platform));
fs.mkdirSync(evidence, { recursive: true });
const child = spawn(binary, ['--smoke-test'], { shell: false, windowsHide: true, cwd: evidence,
  env: { ...process.env, CEREBRI_SMOKE_EVIDENCE: evidence }, stdio: ['ignore', 'pipe', 'pipe'] });
let stdout = '', stderr = '', apiPid, secondary;
const apiPids = new Set();
const timer = setTimeout(() => child.kill('SIGTERM'), 75000);
child.stdout.on('data', (chunk) => {
  stdout = (stdout + chunk).slice(-1024 * 1024);
  apiPid = Number(/CEREBRI_DESKTOP_CORE_PID=(\d+)/.exec(stdout)?.[1]) || apiPid;
  for (const match of stdout.matchAll(/CEREBRI_DESKTOP_CORE_PID=(\d+)/g)) apiPids.add(Number(match[1]));
  if (apiPid && !secondary) secondary = new Promise((resolve) => {
    const second = spawn(binary, ['--smoke-second-instance'], { shell: false, windowsHide: true, cwd: evidence, stdio: ['ignore', 'pipe', 'pipe'] });
    let output = '';
    const timeout = setTimeout(() => second.kill('SIGTERM'), 15000);
    second.stdout.on('data', chunk => output += chunk);
    second.once('error', () => { clearTimeout(timeout); resolve(false); });
    second.once('close', code => { clearTimeout(timeout); resolve(code === 0 && output.includes('CEREBRI_DESKTOP_SECOND_INSTANCE_OK')); });
  });
});
child.stderr.on('data', (chunk) => { stderr = (stderr + chunk).slice(-128 * 1024); });
child.once('error', (error) => { clearTimeout(timer); console.error(error); process.exitCode = 1; });
child.once('close', async (code) => {
  clearTimeout(timer);
  fs.writeFileSync(path.join(evidence, 'stdout.log'), stdout);
  fs.writeFileSync(path.join(evidence, 'stderr.log'), stderr);
  try {
    assert.equal(code, 0, stderr + '\n' + stdout);
    const marker = /^CEREBRI_DESKTOP_SMOKE_OK=(.*)$/m.exec(stdout);
    assert.ok(marker, 'Packaged renderer/API success marker is missing.');
    const proof = JSON.parse(marker[1]);
    apiPid = proof.pid;
    assert.equal(proof.childStopped, true);
    assert.equal(proof.renderer, true);
    assert.equal(proof.depthParity, true);
    assert.equal(await secondary, true, 'Second launch was not rejected by the single-instance lock.');
    assert.equal(proof.singleInstance, true);
    assert.equal(proof.crashRecovery, true);
    if (process.platform === 'darwin') assert.equal(proof.macActivation, true);
    assert.match(proof.origin, /^http:\/\/127\.0\.0\.1:\d+$/);
    assert.equal(proof.planner.outcome, 'Solution');
    assert.equal(proof.temporal.status, 'Complete');
    assert.deepEqual(proof.counts, [String(proof.planner.search_space.evaluated), String(proof.planner.conflicts.rejections.length), String(proof.planner.candidates.length), '#1']);
    for (const pid of proof.pids) {
      let alive = false;
      try { process.kill(pid, 0); alive = true; } catch (error) { if (error.code !== 'ESRCH') throw error; }
      assert.equal(alive, false, 'Owned API remained alive after Electron shutdown: ' + pid);
    }
    fs.writeFileSync(path.join(evidence, 'proof.json'), JSON.stringify(proof, null, 2));
    console.log('CEREBRI_DESKTOP_SMOKE_OK ' + JSON.stringify({ platform: process.platform, binary, evidence, childStopped: true, evaluated: proof.planner.search_space.evaluated, candidates: proof.planner.candidates.length, skipped: proof.temporal.data.expansions.flatMap(item => item.skipped).length }));
  } catch (error) {
    console.error(error.message); process.exitCode = 1;
    // Cleanup only the PID reported by this owned, explicitly requested smoke run.
    for (const pid of apiPids) { try { process.kill(pid, 'SIGTERM'); } catch { /* It already exited. */ } }
  }
});
