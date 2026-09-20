import { createServer } from 'node:http';
import { readFile, stat } from 'node:fs/promises';
import { extname, resolve } from 'node:path';

const root = resolve(import.meta.dirname, '../../../site');
const base = process.env.CEREBRI_BASE_PATH ?? '';
const port = Number(process.env.PORT ?? 4173);
const mime = {
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.css': 'text/css; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
  '.png': 'image/png',
  '.svg': 'image/svg+xml',
  '.woff2': 'font/woff2'
};

createServer(async (req, res) => {
  try {
    let pathname = new URL(req.url ?? '/', 'http://localhost').pathname;
    if (base) {
      if (!pathname.startsWith(base)) {
        res.writeHead(404).end('Not found');
        return;
      }
      pathname = pathname.slice(base.length) || '/';
    }
    if (pathname.endsWith('/')) pathname += 'index.html';
    const file = resolve(root, '.' + pathname);
    if (!file.startsWith(root)) {
      res.writeHead(403).end('Forbidden');
      return;
    }
    const info = await stat(file);
    if (!info.isFile()) throw new Error('not file');
    const body = await readFile(file);
    res.writeHead(200, {
      'content-type': mime[extname(file)] ?? 'application/octet-stream',
      'cache-control': 'no-store'
    });
    res.end(body);
  } catch {
    res.writeHead(404, { 'content-type': 'text/plain; charset=utf-8' });
    res.end('Not found');
  }
}).listen(port, '127.0.0.1');
