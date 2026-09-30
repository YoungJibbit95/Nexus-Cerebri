const { app, BrowserWindow, session } = require('electron');
const fs = require('node:fs');
const path = require('node:path');
const { layout, OwnedCore, allowedNavigation } = require('./runtime.cjs');

const smoke = process.argv.includes('--smoke-test');
const smokeSecond = process.argv.includes('--smoke-second-instance');
let window, core, origin, quitting = false, shutdown, retryTimer, connecting = false;
let smokeVerifying = false, secondObserved = false, failureShown;
const controlledPages = new Set();
function escape(value) { return String(value).replace(/[&<>"']/g, (char) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[char])); }
function localPage(title, message, diagnostic = '', restart = false) {
  const url = 'data:text/html;charset=utf-8,' + encodeURIComponent(`<!doctype html><html lang="en"><meta charset="utf-8"><title>Cerebri Lab</title><meta name="viewport" content="width=device-width,initial-scale=1"><meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'"><style>body{margin:0;min-height:100vh;display:grid;place-items:center;background:#101827;color:#edf5ff;font:16px system-ui}main{max-width:42rem;padding:2rem}p{line-height:1.6;color:#bdcbe0}a{color:#8ce5ec}a:focus-visible,summary:focus-visible{outline:2px solid #8ce5ec;outline-offset:5px}pre{white-space:pre-wrap;overflow-wrap:anywhere;font-size:13px}small{display:block;margin-top:2rem;color:#bdcbe0}</style><main><h1>${escape(title)}</h1><p>${escape(message)}</p>${restart ? '<p><a href="cerebri-local:restart">Restart local core</a></p>' : ''}${diagnostic ? `<details><summary>Technical diagnostics</summary><pre>${escape(diagnostic.slice(-4000))}</pre></details>` : ''}<small>Local research runtime · Proposal ≠ execution · No telemetry</small></main></html>`);
  controlledPages.clear(); controlledPages.add(url); return url;
}
async function showFailure(error) {
  if (quitting || !window || window.isDestroyed()) return;
  origin = null;
  await window.loadURL(localPage('The local core is unavailable', 'The Lab cannot plan until its local core is running. Restart the core to try again; no calendar has been changed.', error.message + '\n' + (core?.stderr ?? ''), app.isPackaged));
}
function createWindow() {
  if (quitting) return;
  window = new BrowserWindow({ width: 1440, height: 900, minWidth: 960, minHeight: 640, show: !smoke,
    title: 'Cerebri Lab', backgroundColor: '#101827',
    webPreferences: { nodeIntegration: false, contextIsolation: true, sandbox: true, backgroundThrottling: !smoke },
  });
  window.webContents.setWindowOpenHandler(() => ({ action: 'deny' }));
  const navigation = (event, target) => {
    if (target === 'cerebri-local:restart' && controlledPages.has(window.webContents.getURL()) && app.isPackaged) {
      event.preventDefault(); if (!connecting) void connect(); return;
    }
    if (!allowedNavigation(target, origin, controlledPages)) event.preventDefault();
  };
  window.webContents.on('will-navigate', navigation);
  window.webContents.on('will-redirect', navigation);
  window.on('closed', () => { window = null; });
  if (origin) void window.loadURL(origin + '/lab/').catch(showFailure);
  else void connect();
}
async function connect() {
  if (connecting || quitting || !window) return;
  connecting = true; clearTimeout(retryTimer);
  try {
    await window.loadURL(localPage(app.isPackaged ? 'Starting the local core' : 'Waiting for the development API', app.isPackaged
      ? 'Cerebri Lab is starting its bundled Rust core on a private loopback port.'
      : 'Start the development API at 127.0.0.1:3000. The Lab will reconnect while this window remains open.'));
    if (smoke) await capture('electron-startup.png');
    if (app.isPackaged) {
      core ??= new OwnedCore(layout(process.resourcesPath), { onCrash: (error) => { failureShown = showFailure(error); } });
      origin = await core.start();
      if (smoke) console.log('CEREBRI_DESKTOP_CORE_PID=' + core.pid);
    } else {
      origin = 'http://127.0.0.1:3000';
      const response = await fetch(origin + '/health', { signal: AbortSignal.timeout(2000), redirect: 'error' });
      if (!response.ok || (await response.json()).status !== 'ok') throw new Error('Development API health check failed.');
    }
    if (!quitting && window && !window.isDestroyed()) {
      await window.loadURL(origin + '/lab/');
      if (smoke && !smokeVerifying) await runSmoke();
    }
  } catch (error) {
    await showFailure(error);
    if (smoke) { console.error('CEREBRI_DESKTOP_SMOKE_FAILED=' + error.message); await exit(1); }
    else if (!app.isPackaged && !quitting && window) retryTimer = setTimeout(() => void connect(), 2000);
  } finally { connecting = false; }
}
async function capture(name) {
  if (!smoke || !process.env.CEREBRI_SMOKE_EVIDENCE || !window) return;
  const directory = path.resolve(process.env.CEREBRI_SMOKE_EVIDENCE);
  fs.mkdirSync(directory, { recursive: true });
  await window.webContents.executeJavaScript('new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)))');
  fs.writeFileSync(path.join(directory, name), (await window.capturePage(undefined, { stayHidden: true, stayAwake: true })).toPNG());
}
async function runSmoke() {
  if (!app.isPackaged) throw new Error('Smoke requires an unpacked packaged application.');
  smokeVerifying = true;
  const wait = async (condition) => {
    const deadline = Date.now() + 15000;
    while (!condition()) { if (Date.now() > deadline) throw new Error('Desktop lifecycle verification timed out.'); await new Promise(resolve => setTimeout(resolve, 50)); }
  };
  const inputs = JSON.parse(fs.readFileSync(path.join(layout(process.resourcesPath).root, 'smoke-inputs.json'), 'utf8'));
  const proof = await window.webContents.executeJavaScript(`(async () => {
    const wait = async (condition) => { const deadline = Date.now() + 15000; while (!condition()) { if (Date.now() > deadline) throw new Error('Renderer readiness timed out'); await new Promise(resolve => setTimeout(resolve, 50)); } };
    const button = name => [...document.querySelectorAll('button')].find(item => item.textContent.includes(name) || item.getAttribute('aria-label') === name);
    const post = async (route, input) => { const response = await fetch(route, {method:'POST',headers:{'content-type':'application/json'},body:JSON.stringify(input)}); if (!response.ok) throw new Error(route + ' failed'); return response.json(); };
    await wait(() => document.querySelector('.app-shell') && button('Run planner'));
    if (typeof require !== 'undefined') throw new Error('Node leaked into renderer');
    const health = await (await fetch('/health')).json();
    const planner = await post('/v1/plan', ${JSON.stringify(inputs.planner)});
    button('Understand').click(); button('Run planner').click();
    await wait(() => document.querySelector('.run-flow strong')?.textContent === String(planner.search_space.evaluated));
    if (document.querySelectorAll('pre').length) throw new Error('Understand exposes raw JSON');
    if (!document.querySelector('.authority-rail').textContent.includes('Proposal ≠ execution')) throw new Error('Authority boundary missing');
    const counts = [...document.querySelectorAll('.run-flow strong')].map(item => item.textContent);
    button('Research').click();
    await wait(() => [...document.querySelectorAll('details.json-panel')].some(item => item.querySelector('summary').textContent.includes('Complete PlanningResult')));
    const rendered = JSON.parse([...document.querySelectorAll('details.json-panel')].find(item => item.querySelector('summary').textContent.includes('Complete PlanningResult')).querySelector('pre').textContent);
    if (JSON.stringify(rendered) !== JSON.stringify(planner)) throw new Error('Renderer/planner parity failed');
    button('Understand').click();
    return { health, planner, counts, renderer: true, depthParity: true };
  })()`);
  await capture('electron-planner-understand.png');
  let linuxDesktop = null;
  if (process.platform === 'linux') {
    const metadata = JSON.parse(fs.readFileSync(path.join(app.getAppPath(), 'package.json'), 'utf8'));
    const handle = window.getNativeWindowHandle();
    const id = '0x' + handle.readUInt32LE(0).toString(16);
    const wmClass = require('node:child_process').execFileSync('xprop', ['-id', id, 'WM_CLASS'], { encoding: 'utf8', timeout: 5000 });
    if (!wmClass.includes(metadata.desktopName.replace(/\.desktop$/, ''))) throw new Error('Linux window association does not match desktopName.');
    linuxDesktop = { desktopName: metadata.desktopName, wmClass };
  }
  const temporal = await window.webContents.executeJavaScript(`(async () => {
    const button = name => [...document.querySelectorAll('button')].find(item => item.getAttribute('aria-label') === name || item.textContent.includes(name));
    button('Temporal').click(); await new Promise(resolve => setTimeout(resolve, 0)); button('Inspect temporal data').click();
    const expected = await (await fetch('/v1/temporal',{method:'POST',headers:{'content-type':'application/json'},body:${JSON.stringify(JSON.stringify(inputs.temporal))}})).json();
    const deadline = Date.now() + 15000;
    while (!document.querySelector('.temporal-summary')) { if(Date.now()>deadline) throw new Error('Temporal renderer timed out'); await new Promise(resolve=>setTimeout(resolve,50)); }
    button('Research').click(); await new Promise(resolve=>setTimeout(resolve,0));
    const payload = [...document.querySelectorAll('details.json-panel')].find(item=>item.querySelector('summary').textContent.includes('Complete TemporalResult'));
    if (!payload || JSON.stringify(JSON.parse(payload.querySelector('pre').textContent)) !== JSON.stringify(expected)) throw new Error('Temporal renderer parity failed');
    return expected;
  })()`);
  await wait(() => secondObserved);
  const firstPid = core.pid;
  let macActivation = null;
  if (process.platform === 'darwin') {
    window.close();
    await wait(() => !window);
    if (core.state !== 'ready' || core.pid !== firstPid) throw new Error('Window close stopped the application-owned API.');
    app.emit('activate');
    await wait(() => window && window.webContents.getURL() === origin + '/lab/' && !window.webContents.isLoading());
    if (core.pid !== firstPid) throw new Error('Activation restarted the application-owned API.');
    macActivation = true;
  }
  // Explicit smoke fault injection, followed by the same restart action used by users.
  await new Promise(resolve => { core.child.once('close', resolve); core.child.kill('SIGTERM'); });
  await failureShown;
  if (origin || core.state !== 'failed') throw new Error('Unexpected API exit did not enter the error state.');
  await capture('electron-api-failure.png');
  connecting = false;
  await window.webContents.executeJavaScript("document.querySelector('a[href=\"cerebri-local:restart\"]').click()");
  await wait(() => !connecting && core.state === 'ready' && window.webContents.getURL() === origin + '/lab/');
  await window.webContents.executeJavaScript(`new Promise((resolve, reject) => { const deadline = Date.now() + 15000; const check = () => { if (document.querySelector('.app-shell')) resolve(); else if (Date.now() > deadline) reject(new Error('Recovery renderer timed out')); else setTimeout(check, 50); }; check(); })`);
  if (core.pid === firstPid) throw new Error('Restart did not create a fresh API process.');
  await capture('electron-recovered.png');
  const pid = core.pid;
  await core.stop();
  console.log('CEREBRI_DESKTOP_SMOKE_OK=' + JSON.stringify({ ...proof, temporal, pid, pids: [firstPid, pid], singleInstance: secondObserved, crashRecovery: true, macActivation, linuxDesktop, childStopped: core.state === 'stopped', origin }));
  await exit(0);
}
async function exit(code = 0) {
  if (shutdown) return shutdown;
  quitting = true; clearTimeout(retryTimer);
  shutdown = (async () => {
    try { await core?.stop(); } catch (error) { console.error(error.message); code = 1; }
    app.exit(code);
  })();
  return shutdown;
}

if (!app.requestSingleInstanceLock()) { if (smokeSecond) console.log('CEREBRI_DESKTOP_SECOND_INSTANCE_OK'); app.quit(); }
else if (smokeSecond) { console.error('Second instance unexpectedly acquired the lock.'); app.exit(1); }
else {
  app.on('second-instance', () => { secondObserved = true; if (quitting) return; if (!window) createWindow(); else { if (window.isMinimized()) window.restore(); if (!smoke) { window.show(); window.focus(); } } });
  app.whenReady().then(() => {
    session.defaultSession.setPermissionRequestHandler((_contents, _permission, callback) => callback(false));
    session.defaultSession.setPermissionCheckHandler(() => false);
    session.defaultSession.webRequest.onBeforeRequest((details, callback) => {
      let allowed = controlledPages.has(details.url);
      try { const url = new URL(details.url); allowed ||= Boolean(origin) && url.origin === origin; } catch { /* Fail closed. */ }
      callback({ cancel: !allowed });
    });
    createWindow();
    app.on('activate', () => { if (!quitting && BrowserWindow.getAllWindows().length === 0) createWindow(); });
  });
  app.on('before-quit', (event) => { if (!quitting) { event.preventDefault(); void exit(); } });
  app.on('window-all-closed', () => { if (process.platform !== 'darwin') app.quit(); });
  process.on('SIGTERM', () => void exit());
  process.on('SIGINT', () => void exit());
}
