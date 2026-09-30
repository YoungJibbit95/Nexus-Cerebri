const fs = require('node:fs');
const path = require('node:path');
const { executable, layout, verifyLayout } = require('./runtime.cjs');

function stage({ labRoot = path.resolve(__dirname, '..'), apiPath = path.resolve(__dirname, '../../../target/release', executable()) } = {}) {
  const dist = path.join(labRoot, 'dist');
  const staging = path.resolve(labRoot, 'desktop/.stage');
  // Only this fixed generated directory may be recursively removed.
  if (staging !== path.join(path.resolve(labRoot), 'desktop', '.stage')) throw new Error('Invalid staging target.');
  if (!fs.existsSync(path.join(dist, 'index.html')) || !fs.existsSync(path.join(dist, 'assets'))) throw new Error('Build Lab production assets before staging.');
  if (!fs.statSync(apiPath, { throwIfNoEntry: false })?.isFile()) throw new Error('Build the release cerebri-api before staging.');
  fs.rmSync(staging, { recursive: true, force: true });
  const paths = layout(staging);
  fs.mkdirSync(path.dirname(paths.api), { recursive: true });
  fs.copyFileSync(apiPath, paths.api);
  if (process.platform !== 'win32') fs.chmodSync(paths.api, 0o755);
  fs.cpSync(dist, paths.lab, { recursive: true });
  verifyLayout(paths);
  const repository = path.resolve(labRoot, '../..');
  const planner = JSON.parse(fs.readFileSync(path.join(repository, 'examples/request.json'), 'utf8'));
  planner.preferences = { preferences: [{ source: 'ExplicitCurrentRequest', preferred_start: '2026-10-01T10:45:00Z', evidence: [] }] };
  fs.writeFileSync(path.join(paths.root, 'smoke-inputs.json'), JSON.stringify({ planner,
    temporal: JSON.parse(fs.readFileSync(path.join(repository, 'examples/temporal-request.json'), 'utf8')) }));
  console.log('CEREBRI_DESKTOP_STAGED=' + paths.root);
  return paths;
}
if (require.main === module) stage();
module.exports = { stage };
