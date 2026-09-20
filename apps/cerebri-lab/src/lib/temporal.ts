import type { Json, TimeRange } from './contracts.ts';
import { shape } from './transport.ts';
const { object, array, string, number, range, member } = shape;

export interface TemporalRequest {
  horizon: TimeRange;
  coverage: 'Complete' | 'Incomplete';
  busy: { range: TimeRange; buffers: { before_seconds: number; after_seconds: number; travel_seconds: number } }[];
  recurrences: Json[];
  limits: { max_occurrences: number; max_dates: number };
}
export interface ExpansionReport {
  horizon: TimeRange;
  timezone: string;
  occurrences: { sequence: number; date: string; local_time: string; range: TimeRange; visible_range: TimeRange; resolution: 'Unique' | 'EarlierFold' | 'LaterFold' }[];
  skipped: { sequence: number; date: string; reason: 'NonexistentLocalTime' }[];
  examined_dates: number;
}
export interface TemporalReport {
  availability: {
    horizon: TimeRange; coverage: 'Complete' | 'Incomplete'; busy: TimeRange[]; free: TimeRange[]; unknown: TimeRange[];
    trace: { input_index: number; original: TimeRange; buffered: TimeRange; clipped: TimeRange | null }[];
  };
  expansions: ExpansionReport[];
}
export type TemporalResult = { status: 'Complete'; data: TemporalReport } | { status: 'Rejected'; data: Json };
export function parseTemporalRequest(value: unknown): TemporalRequest {
  const item = object(value, 'temporal');
  range(item.horizon, 'horizon');
  member(item.coverage, ['Complete', 'Incomplete'], 'coverage');
  for (const raw of array(item.busy, 'busy')) {
    const entry = object(raw, 'busy interval');
    range(entry.range, 'busy.range');
    const buffers = object(entry.buffers, 'buffers');
    for (const key of ['before_seconds', 'after_seconds', 'travel_seconds']) number(buffers[key], key);
  }
  array(item.recurrences, 'recurrences');
  const limits = object(item.limits, 'limits');
  number(limits.max_occurrences, 'max_occurrences');
  number(limits.max_dates, 'max_dates');
  return value as TemporalRequest;
}
export function parseTemporalResult(value: unknown): TemporalResult {
  const item = object(value, 'temporal result');
  member(item.status, ['Complete', 'Rejected'], 'status');
  if (item.status === 'Rejected') {
    if (!('data' in item)) throw new Error('Rejected temporal result requires error data.');
    return value as TemporalResult;
  }
  const data = object(item.data, 'temporal data');
  const availability = object(data.availability, 'availability');
  range(availability.horizon, 'horizon');
  member(availability.coverage, ['Complete', 'Incomplete'], 'coverage');
  for (const key of ['busy', 'free', 'unknown']) array(availability[key], key).forEach((entry) => range(entry, key));
  for (const raw of array(availability.trace, 'availability.trace')) {
    const entry = object(raw, 'interval trace');
    number(entry.input_index, 'input_index');
    range(entry.original, 'original'); range(entry.buffered, 'buffered');
    if (entry.clipped !== null) range(entry.clipped, 'clipped');
  }
  for (const raw of array(data.expansions, 'expansions')) {
    const expansion = object(raw, 'expansion');
    range(expansion.horizon, 'expansion.horizon');
    string(expansion.timezone, 'timezone');
    number(expansion.examined_dates, 'examined_dates');
    for (const occurrence of array(expansion.occurrences, 'occurrences')) {
      const occurrenceData = object(occurrence, 'occurrence');
      number(occurrenceData.sequence, 'sequence');
      string(occurrenceData.date, 'date'); string(occurrenceData.local_time, 'local_time');
      range(occurrenceData.range, 'range'); range(occurrenceData.visible_range, 'visible_range');
      member(occurrenceData.resolution, ['Unique', 'EarlierFold', 'LaterFold'], 'resolution');
    }
    for (const skipped of array(expansion.skipped, 'skipped')) {
      const skip = object(skipped, 'skip');
      number(skip.sequence, 'sequence'); string(skip.date, 'date');
      member(skip.reason, ['NonexistentLocalTime'], 'reason');
    }
  }
  return value as TemporalResult;
}
