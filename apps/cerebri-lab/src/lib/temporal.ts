import type { Json, TimeRange } from './contracts.ts';
import { shape } from './shape.ts';
const { object, array, string, number, range, member } = shape;

export interface TemporalRequest {
  horizon: TimeRange;
  coverage: 'Complete' | 'Incomplete';
  busy: { range: TimeRange; buffers: { before_seconds: number; after_seconds: number; travel_seconds: number } }[];
  recurrences: RecurrenceRule[];
  limits: { max_occurrences: number; max_dates: number };
}
export interface RecurrenceRule {
  start_date: string; local_time: string; timezone: string; duration: number;
  pattern: { frequency: 'DAILY'; every: number } | { frequency: 'WEEKLY'; every: number; weekdays: string[] };
  until?: string | null; count?: number | null;
  gap_policy: 'Reject' | 'Skip'; fold_policy: 'Reject' | 'Earlier' | 'Later';
}
export type Occurrence = ExpansionReport['occurrences'][number];
export interface ExpansionReport {
  horizon: TimeRange;
  timezone: string;
  occurrences: { sequence: number; date: string; local_time: string; range: TimeRange; visible_range: TimeRange; resolution: 'Unique' | 'EarlierFold' | 'LaterFold' }[];
  skipped: { sequence: number; date: string; reason: 'NonexistentLocalTime' }[];
  examined_dates: number;
}
export interface AvailabilityReport {
  horizon: TimeRange; coverage: 'Complete' | 'Incomplete'; busy: TimeRange[]; free: TimeRange[]; unknown: TimeRange[];
  trace: { input_index: number; original: TimeRange; buffered: TimeRange; clipped: TimeRange | null }[];
}
export interface TemporalReport {
  availability: AvailabilityReport;
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
  array(item.recurrences, 'recurrences').forEach(parseRecurrence);
  const limits = object(item.limits, 'limits');
  number(limits.max_occurrences, 'max_occurrences');
  number(limits.max_dates, 'max_dates');
  return value as TemporalRequest;
}
export function parseRecurrence(value: unknown): RecurrenceRule {
  const rule = object(value, 'recurrence');
  for (const key of ['start_date', 'local_time', 'timezone']) string(rule[key], `recurrence.${key}`);
  number(rule.duration, 'recurrence.duration');
  if (rule.until != null) string(rule.until, 'recurrence.until');
  if (rule.count != null) number(rule.count, 'recurrence.count');
  member(rule.gap_policy, ['Reject', 'Skip'], 'recurrence.gap_policy');
  member(rule.fold_policy, ['Reject', 'Earlier', 'Later'], 'recurrence.fold_policy');
  const pattern = object(rule.pattern, 'recurrence.pattern');
  member(pattern.frequency, ['DAILY', 'WEEKLY'], 'recurrence.pattern.frequency');
  number(pattern.every, 'recurrence.pattern.every');
  if (pattern.frequency === 'WEEKLY') array(pattern.weekdays, 'recurrence.pattern.weekdays').forEach((day) => member(day, ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'], 'recurrence.weekday'));
  return value as RecurrenceRule;
}
export function parseTemporalResult(value: unknown): TemporalResult {
  const item = object(value, 'temporal result');
  member(item.status, ['Complete', 'Rejected'], 'status');
  if (item.status === 'Rejected') {
    if (!('data' in item)) throw new Error('Rejected temporal result requires error data.');
    return value as TemporalResult;
  }
  const data = object(item.data, 'temporal data');
  parseAvailability(data.availability);
  array(data.expansions, 'expansions').forEach(parseExpansion);
  return value as TemporalResult;
}
export function parseAvailability(value: unknown): AvailabilityReport {
  const availability = object(value, 'availability');
  range(availability.horizon, 'horizon');
  member(availability.coverage, ['Complete', 'Incomplete'], 'coverage');
  for (const key of ['busy', 'free', 'unknown']) array(availability[key], key).forEach((entry) => range(entry, key));
  for (const raw of array(availability.trace, 'availability.trace')) {
    const entry = object(raw, 'interval trace');
    number(entry.input_index, 'input_index');
    range(entry.original, 'original'); range(entry.buffered, 'buffered');
    if (entry.clipped !== null) range(entry.clipped, 'clipped');
  }
  return value as AvailabilityReport;
}
export function parseExpansion(value: unknown): ExpansionReport {
  const expansion = object(value, 'expansion');
  range(expansion.horizon, 'expansion.horizon');
  string(expansion.timezone, 'timezone');
  number(expansion.examined_dates, 'examined_dates');
  array(expansion.occurrences, 'occurrences').forEach(parseOccurrence);
  for (const skipped of array(expansion.skipped, 'skipped')) {
    const skip = object(skipped, 'skip');
    number(skip.sequence, 'sequence'); string(skip.date, 'date');
    member(skip.reason, ['NonexistentLocalTime'], 'reason');
  }
  return value as ExpansionReport;
}
export function parseOccurrence(value: unknown): Occurrence {
  const occurrence = object(value, 'occurrence');
  number(occurrence.sequence, 'occurrence.sequence');
  string(occurrence.date, 'occurrence.date'); string(occurrence.local_time, 'occurrence.local_time');
  range(occurrence.range, 'occurrence.range'); range(occurrence.visible_range, 'occurrence.visible_range');
  member(occurrence.resolution, ['Unique', 'EarlierFold', 'LaterFold'], 'occurrence.resolution');
  return value as Occurrence;
}
