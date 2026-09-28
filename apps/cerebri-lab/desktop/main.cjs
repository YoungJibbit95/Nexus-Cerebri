const { app, BrowserWindow, session } = require('electron');

const apiOrigin = 'http://127.0.0.1:3000';
const labUrl = `${apiOrigin}/lab/`;
const waitingPage = `data:text/html;charset=utf-8,${encodeURIComponent(`
<!doctype html><html lang="en"><meta charset="utf-8"><title>Cerebri Lab</title>
<meta name="viewport" content="width=device-width,initial-scale=1">
<style>body{margin:0;min-height:100vh;display:grid;place-items:center;background:#101827;color:#edf5ff;font:16px system-ui,sans-serif}main{max-width:32rem;padding:2rem}h1{font-size:1.5rem}p{line-height:1.5;color:#bdcbe0}</style>
<main><h1>Waiting for Cerebri API</h1><p>Start the local API on 127.0.0.1:3000. Cerebri Lab will connect automatically.</p></main>
</html>`)}`;

async function labIsReady() {
  try {
    const response = await fetch(labUrl, { signal: AbortSignal.timeout(2000) });
    return response.ok && response.headers.get('content-type')?.includes('text/html');
  } catch {
    return false;
  }
}

function createWindow() {
  const window = new BrowserWindow({
    width: 1440,
    height: 900,
    minWidth: 960,
    minHeight: 640,
    title: 'Cerebri Lab',
    backgroundColor: '#101827',
    webPreferences: { nodeIntegration: false, contextIsolation: true, sandbox: true },
  });

  window.webContents.setWindowOpenHandler(() => ({ action: 'deny' }));
  window.webContents.on('will-navigate', (event, target) => {
    if (!target.startsWith(labUrl)) event.preventDefault();
  });

  let loading = false;
  let connected = false;
  void window.loadURL(waitingPage);
  const retry = async () => {
    if (loading || connected || window.isDestroyed()) return;
    loading = true;
    if (await labIsReady() && !window.isDestroyed()) {
      try {
        await window.loadURL(labUrl);
        connected = true;
      } catch {
        if (!window.isDestroyed()) void window.loadURL(waitingPage);
      }
    }
    loading = false;
  };
  const timer = setInterval(() => void retry(), 2000);
  window.on('closed', () => clearInterval(timer));
  void retry();
}

app.whenReady().then(() => {
  session.defaultSession.setPermissionRequestHandler((_contents, _permission, callback) => callback(false));
  createWindow();
  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit();
});
