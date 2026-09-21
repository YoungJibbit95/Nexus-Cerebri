function object(value: unknown, path: string): Record<string, unknown> {
  if (typeof value !== 'object' || value === null || Array.isArray(value)) throw new Error(`${path}: expected an object.`);
  return value as Record<string, unknown>;
}
function array(value: unknown, path: string): unknown[] {
  if (!Array.isArray(value)) throw new Error(`${path}: expected an array.`);
  return value;
}
function string(value: unknown, path: string): string {
  if (typeof value !== 'string') throw new Error(`${path}: expected text.`);
  return value;
}
function number(value: unknown, path: string): number {
  if (typeof value !== 'number' || !Number.isSafeInteger(value) || value < 0) throw new Error(`${path}: expected a nonnegative safe integer.`);
  return value;
}
function identifier(value: unknown, path: string): string {
  const text = string(value, path);
  if (!/^[A-Za-z0-9._-]{1,128}$/.test(text)) throw new Error(`${path}: expected a bounded domain identifier.`);
  return text;
}
function instant(value: unknown, path: string): void {
  const text = string(value, path);
  if (!/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?(Z|[+-]\d{2}:\d{2})$/i.test(text) || !Number.isFinite(Date.parse(text))) throw new Error(`${path}: expected an ISO timestamp with an explicit UTC offset.`);
}
function range(value: unknown, path: string): void {
  const item = object(value, path);
  instant(item.start, `${path}.start`);
  instant(item.end, `${path}.end`);
  if (Date.parse(item.start as string) >= Date.parse(item.end as string)) throw new Error(`${path}: expected a positive interval.`);
}
function member(value: unknown, choices: string[], path: string): void {
  if (!choices.includes(string(value, path))) throw new Error(`${path}: unsupported value ${String(value)}.`);
}
export const shape = { object, array, string, number, instant, range, member, identifier };
