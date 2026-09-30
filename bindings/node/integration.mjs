// Host-side transport only. Rust owns admission, planning and all domain evidence.
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const defaultBinary = fileURLToPath(new URL(
  '../../target/debug/cerebri-node-bridge' + (process.platform === 'win32' ? '.exe' : ''), import.meta.url));
const messages = {
  invalid_request: 'Request must be serializable JSON.',
  request_too_large: 'Request exceeds 256 KiB.',
  invalid_options: 'Invalid bridge options.',
  aborted: 'Planning call was cancelled.',
  timeout: 'Planning bridge deadline exceeded.',
  bridge_unavailable: 'Planning bridge could not be started.',
  bridge_failed: 'Planning bridge failed.',
  response_too_large: 'Response exceeds 16 MiB.',
  invalid_response: 'Planning bridge returned an invalid response.',
  incompatible_bridge: 'Planning bridge integration contract is incompatible.',
};

export class CerebriBridgeError extends Error {
  constructor(code) {
    super(messages[code]);
    this.name = 'CerebriBridgeError';
    this.code = code;
  }
}

const object = value => value !== null && typeof value === 'object' && !Array.isArray(value);
const version = (value, minor) => object(value) && value.major === 0 && value.minor === minor;
const rejectionCodes = new Set([
  'invalid_request', 'request_too_large', 'unsupported_integration_version',
  'unsupported_cpir_version', 'unsupported_operation', 'mutation_authority_forbidden',
  'unsupported_deployment_mode', 'prospective_event_required', 'temporal_coverage_required',
  'uncertain_duration_forbidden',
]);

function checkVersion(value) {
  if (!object(value)) throw new CerebriBridgeError('invalid_response');
  if (!version(value.integration_version, 1)) throw new CerebriBridgeError('incompatible_bridge');
}

export async function describeIntegration(options) {
  const value = await call('--describe-integration', '', options);
  checkVersion(value);
  if (value.profile !== 'single_event_suggestion' || !version(value.cpir_schema_version, 2)
      || value.execution_supported !== false || value.temporal_coverage_required !== true
      || value.max_request_bytes !== 256 * 1024
      || JSON.stringify(value.operations) !== '["FIND_SLOT"]'
      || JSON.stringify(value.deployment_modes) !== '["Test","Shadow","Suggestion"]'
      || typeof value.software_version !== 'string') {
    throw new CerebriBridgeError('incompatible_bridge');
  }
  return value;
}

export async function suggest(request, options) {
  let input;
  try { input = JSON.stringify(request); } catch { throw new CerebriBridgeError('invalid_request'); }
  if (typeof input !== 'string') throw new CerebriBridgeError('invalid_request');
  if (Buffer.byteLength(input) > 256 * 1024) throw new CerebriBridgeError('request_too_large');
  const value = await call('--suggest', input, options);
  checkVersion(value);
  if (value.status === 'rejected' && rejectionCodes.has(value.code)) return value;
  // Only the transport envelope is decoded here. Consumers must decode the CPIR result
  // fields they use; this transport must not reconstruct solver or lifecycle truth.
  const sent = JSON.parse(input);
  if (value.status !== 'planned' || !object(value.result)
      || typeof value.request_id !== 'string' || typeof value.trace_id !== 'string'
      || !Number.isSafeInteger(value.context_revision) || value.context_revision < 0
      || value.request_id !== sent?.request?.request_id
      || value.trace_id !== sent?.request?.trace_id
      || value.context_revision !== sent?.request?.context?.revision) {
    throw new CerebriBridgeError('invalid_response');
  }
  return value;
}

function call(command, input, options = {}) {
  if (!object(options)) return Promise.reject(new CerebriBridgeError('invalid_options'));
  const { binary = defaultBinary, timeoutMs = 10_000, signal } = options;
  if (typeof binary !== 'string' || !binary || !Number.isInteger(timeoutMs)
      || timeoutMs < 1 || timeoutMs > 60_000
      || (signal !== undefined && !(signal instanceof AbortSignal))) {
    return Promise.reject(new CerebriBridgeError('invalid_options'));
  }
  if (signal?.aborted) return Promise.reject(new CerebriBridgeError('aborted'));
  return new Promise((resolve, reject) => {
    let child;
    let timer;
    let settled = false;
    let size = 0;
    const chunks = [];
    const cleanup = () => {
      clearTimeout(timer);
      signal?.removeEventListener('abort', abort);
      chunks.length = 0;
    };
    const fail = code => {
      if (settled) return;
      settled = true;
      cleanup();
      child?.stdin.destroy();
      child?.kill('SIGKILL');
      reject(new CerebriBridgeError(code));
    };
    const abort = () => fail('aborted');
    try {
      child = spawn(binary, [command], { windowsHide: true, stdio: ['pipe', 'pipe', 'pipe'] });
    } catch { fail('bridge_unavailable'); return; }
    child.on('error', () => fail('bridge_unavailable'));
    child.stdin.on('error', () => fail('bridge_failed'));
    child.stdout.on('error', () => fail('bridge_failed'));
    child.stderr.on('error', () => fail('bridge_failed'));
    child.stderr.resume(); // Never forward parser output or application content to logs.
    child.stdout.on('data', chunk => {
      if (settled) return;
      size += chunk.length;
      if (size > 16 * 1024 * 1024) fail('response_too_large');
      else chunks.push(chunk);
    });
    child.on('close', code => {
      if (settled) return;
      if (code !== 0) { fail('bridge_failed'); return; }
      let value;
      try { value = JSON.parse(Buffer.concat(chunks).toString('utf8')); }
      catch { fail('invalid_response'); return; }
      settled = true;
      cleanup();
      resolve(value);
    });
    signal?.addEventListener('abort', abort, { once: true });
    timer = setTimeout(() => fail('timeout'), timeoutMs);
    child.stdin.end(input);
  });
}
