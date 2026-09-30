const { spawn } = require('node:child_process');
const fs = require('node:fs');
const path = require('node:path');

function executable(platform = process.platform) {
  if (!['win32', 'linux', 'darwin'].includes(platform)) throw new Error('Unsupported desktop platform.');
  return 'cerebri-api' + (platform === 'win32' ? '.exe' : '');
}
function layout(resourcesPath, platform = process.platform) {
  if (!path.isAbsolute(resourcesPath)) throw new Error('Resources path must be absolute.');
  const root = path.join(resourcesPath, 'cerebri-runtime');
  return { root, api: path.join(root, 'api', executable(platform)), lab: path.join(root, 'lab') };
}
function verifyLayout(paths) {
  if (!fs.statSync(paths.api, { throwIfNoEntry: false })?.isFile()) throw new Error('Bundled local core executable is missing.');
  if (!fs.statSync(path.join(paths.lab, 'index.html'), { throwIfNoEntry: false })?.isFile() ||
      !fs.statSync(path.join(paths.lab, 'assets'), { throwIfNoEntry: false })?.isDirectory()) throw new Error('Bundled Lab assets are missing.');
  if (process.platform !== 'win32') fs.accessSync(paths.api, fs.constants.X_OK);
}
function listenOrigin(line) {
  const match = /^CEREBRI_LISTEN_ADDR=127\.0\.0\.1:([1-9]\d{0,4})$/.exec(line.trim());
  if (!match || Number(match[1]) > 65535) throw new Error('Local core reported an invalid loopback address.');
  return 'http://127.0.0.1:' + match[1];
}
function allowedNavigation(target, origin, controlledPages = new Set()) {
  if (controlledPages.has(target)) return true;
  try {
    const url = new URL(target);
    return Boolean(origin) && url.origin === origin && !url.username && !url.password && url.pathname.startsWith('/lab/');
  } catch { return false; }
}

class OwnedCore {
  constructor(paths, { spawnProcess = spawn, fetchHealth = fetch, startupMs = 15000, shutdownMs = 3000, onCrash = () => {} } = {}) {
    this.paths = paths; this.spawnProcess = spawnProcess; this.fetchHealth = fetchHealth;
    this.startupMs = startupMs; this.shutdownMs = shutdownMs; this.onCrash = onCrash;
    this.state = 'idle'; this.child = null; this.origin = null; this.stderr = ''; this.generation = 0;
  }
  async start() {
    if (this.state === 'ready') return this.origin;
    if (this.starting) return this.starting;
    this.starting = this.launch();
    try { return await this.starting; } finally { this.starting = null; }
  }
  async launch() {
    verifyLayout(this.paths);
    this.state = 'starting'; this.stderr = ''; this.origin = null;
    const generation = ++this.generation;
    const deadline = Date.now() + this.startupMs;
    this.child = this.spawnProcess(this.paths.api, [], {
      cwd: this.paths.root, shell: false, windowsHide: true, stdio: ['ignore', 'pipe', 'pipe'],
      env: { ...process.env, CEREBRI_BIND_ADDR: '127.0.0.1:0', CEREBRI_LAB_DIST: this.paths.lab },
    });
    const child = this.child;
    this.pid = child.pid;
    try {
      const origin = await new Promise((resolve, reject) => {
        let buffer = '', discovered = false;
        const timeout = setTimeout(() => reject(new Error('Local core startup timed out.')), this.startupMs);
        const done = (error, address) => { clearTimeout(timeout); this.abortStartup = null; error ? reject(error) : resolve(address); };
        this.abortStartup = () => done(new Error('Local core startup cancelled.'));
        child.stderr.on('data', (chunk) => { this.stderr = (this.stderr + chunk).slice(-4000); });
        child.stdout.on('data', (chunk) => {
          if (discovered) return;
          buffer += chunk;
          if (buffer.length > 16384) { discovered = true; done(new Error('Local core startup output exceeds its bound.')); return; }
          const lines = buffer.split(/\r?\n/); buffer = lines.pop();
          for (const line of lines) if (line.startsWith('CEREBRI_LISTEN_ADDR=')) {
            discovered = true;
            try { done(null, listenOrigin(line)); } catch (error) { done(error); }
            break;
          }
        });
        child.once('error', (error) => done(error));
        child.once('exit', (code, signal) => {
          if (this.state === 'starting') done(new Error(`Local core exited during startup (${code ?? signal}).`));
          else if (this.state === 'ready') {
            this.state = 'failed'; this.origin = null;
            this.onCrash(new Error(`Local core stopped unexpectedly (${code ?? signal}).`));
          }
        });
      });
      const response = await this.fetchHealth(origin + '/health', { signal: AbortSignal.timeout(Math.max(1, Math.min(4000, deadline - Date.now()))), redirect: 'error' });
      const health = await response.json();
      if (!response.ok || health.status !== 'ok' || typeof health.software_version !== 'string') throw new Error('Local core health check failed.');
      if (Date.now() > deadline) throw new Error('Local core startup timed out.');
      if (generation !== this.generation || child.exitCode !== null || child.signalCode !== null) throw new Error('Local core stopped before readiness.');
      this.health = health; this.origin = origin; this.state = 'ready'; return origin;
    } catch (error) {
      await this.stop(); this.state = 'failed'; throw error;
    }
  }
  async stop() {
    if (this.stopping) return this.stopping;
    this.state = 'stopping'; ++this.generation; this.origin = null;
    this.abortStartup?.(); this.abortStartup = null;
    this.stopping = (async () => {
      const child = this.child;
      if (child && child.exitCode === null && child.signalCode === null) {
        await new Promise((resolve, reject) => {
          let force;
          const timeout = setTimeout(() => {
            child.kill('SIGKILL');
            force = setTimeout(() => reject(new Error('Owned local core did not terminate.')), this.shutdownMs);
          }, this.shutdownMs);
          child.once('close', () => { clearTimeout(timeout); clearTimeout(force); resolve(); });
          child.kill('SIGTERM');
        });
      }
      this.child = null; this.state = 'stopped';
    })();
    try { await this.stopping; } finally { this.stopping = null; }
  }
}
module.exports = { executable, layout, verifyLayout, listenOrigin, allowedNavigation, OwnedCore };
