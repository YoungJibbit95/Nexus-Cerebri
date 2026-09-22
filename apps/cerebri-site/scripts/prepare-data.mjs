import { existsSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { resolve } from 'node:path';

const upstream = resolve(process.cwd(), '../../scripts/build-site-data.mjs');
if (existsSync(upstream)) {
  const result = spawnSync(process.execPath, [upstream], { stdio: 'inherit' });
  process.exit(result.status ?? 1);
}

for (const path of ['src/lib/generated/runtime-data.ts', 'src/lib/generated/docs-data.ts', 'static/nexus-cerebri-logo.png']) {
  if (!existsSync(resolve(process.cwd(), path))) {
    console.error(`Missing generated site input: ${path}`);
    process.exit(1);
  }
}
console.log('Using bundled generated site data (standalone site package).');
