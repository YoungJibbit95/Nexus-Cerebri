// Provisional process bridge, with all planning delegated to the Rust core.
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';
const defaultBinary = fileURLToPath(new URL(
  '../../target/debug/cerebri-node-bridge' + (process.platform === 'win32' ? '.exe' : ''), import.meta.url));

export async function plan(request, { binary = defaultBinary } = {}) {
  const input = JSON.stringify(request);
  if (Buffer.byteLength(input) > 256 * 1024) throw new RangeError('request exceeds 256 KiB');
  return new Promise((resolve, reject) => {
    const child = spawn(binary, [], { windowsHide: true, stdio: ['pipe','pipe','pipe'] });
    const chunks = [];
    let size = 0;
    child.on('error', reject);
    child.stdin.on('error', reject);
    child.stdout.on('data', chunk => {
      size += chunk.length;
      if (size > 16 * 1024 * 1024) { child.kill(); reject(new RangeError('response exceeds 16 MiB')); }
      else chunks.push(chunk);
    });
    // Do not forward raw input/parser diagnostics into application logs.
    child.stderr.resume();
    child.on('close', code => {
      if (code !== 0) { reject(new Error('Rust planning bridge failed')); return; }
      try { resolve(JSON.parse(Buffer.concat(chunks).toString('utf8'))); } catch (error) { reject(error); }
    });
    child.stdin.end(input);
  });
}

