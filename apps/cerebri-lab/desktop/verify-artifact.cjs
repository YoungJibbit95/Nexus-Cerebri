// Build evidence only: never executes on ordinary application launch.
const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');
const { execFileSync } = require('node:child_process');
const root = path.resolve(__dirname, '../desktop-dist');
const metadata = require('../package.json');
const extension = { win32: '.exe', linux: '.AppImage', darwin: '.dmg' }[process.platform];
const artifacts = fs.readdirSync(root).filter(name => name.endsWith(extension) && fs.statSync(path.join(root, name)).isFile());
assert.ok(artifacts.length, 'Native installer artifact is missing.');
if (process.platform === 'linux') {
  const directory = path.join(root, 'appimage-evidence');
  fs.mkdirSync(directory, { recursive: true });
  execFileSync(path.join(root, artifacts[0]), ['--appimage-extract'], { cwd: directory, stdio: 'ignore', timeout: 60000 });
  const extracted = path.join(directory, 'squashfs-root');
  const entry = metadata.desktopName;
  assert.ok(entry.endsWith('.desktop'));
  const text = fs.readFileSync(path.join(extracted, entry), 'utf8');
  const identity = entry.replace(/\.desktop$/, '');
  assert.match(text, /^Exec=AppRun %U$/m);
  assert.ok(!text.includes('--no-sandbox'), 'Installer entry must preserve Chromium sandboxing.');
  assert.ok(text.split('\n').includes('StartupWMClass=' + identity));
  assert.ok(fs.statSync(path.join(extracted, 'cerebri-lab')).isFile());
  const proof = JSON.parse(fs.readFileSync(path.join(root, 'smoke/linux/proof.json'), 'utf8'));
  assert.equal(proof.linuxDesktop.desktopName, entry);
  assert.ok(proof.linuxDesktop.wmClass.includes(identity));
  fs.writeFileSync(path.join(root, 'smoke/linux/desktop-entry.txt'), text);
  console.log('CEREBRI_LINUX_IDENTITY_OK ' + JSON.stringify({ entry, identity, executable: 'cerebri-lab', wmClass: proof.linuxDesktop.wmClass }));
}
console.log('CEREBRI_INSTALLER_OK ' + JSON.stringify({ platform: process.platform, artifacts }));
