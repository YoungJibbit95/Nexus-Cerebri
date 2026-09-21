import type { TimeRange } from './contracts.ts';
import type { AvailabilityReport, ExpansionReport, Occurrence, RecurrenceRule } from './temporal.ts';
import { parseAvailability, parseExpansion, parseOccurrence, parseRecurrence } from './temporal.ts';
import { shape } from './shape.ts';
const { object, array, string, number, range, member } = shape;

export type Provenance = 'USER_EXPLICIT' | 'INTEGRATION_FACT' | 'SYSTEM_FACT' | 'PERSONAL_LEARNED' | 'GLOBAL_LEARNED' | 'MODEL_INFERENCE' | 'DEFAULT';
export interface TemporalSeries {
  id: string;
  state: { kind: 'Existing'; revision: number } | { kind: 'Prospective' };
  provenance: Provenance;
  evidence: string[];
  rule: RecurrenceRule;
}
export interface TemporalContext {
  horizon: TimeRange; coverage: 'Complete' | 'Incomplete';
  limits: { max_occurrences: number; max_dates: number };
  series: TemporalSeries[];
}
export interface CompiledContextSnapshot {
  source_revision: number;
  horizon: TimeRange;
  occurrences: { id: string; series_id: string; source_revision: number; provenance: Provenance; evidence: string[]; occurrence: Occurrence }[];
  series: { series: TemporalSeries; expansion: ExpansionReport }[];
  availability: AvailabilityReport;
}
function source(value: Record<string, unknown>, path: string): void {
  member(value.provenance, ['USER_EXPLICIT', 'INTEGRATION_FACT', 'SYSTEM_FACT', 'PERSONAL_LEARNED', 'GLOBAL_LEARNED', 'MODEL_INFERENCE', 'DEFAULT'], `${path}.provenance`);
  array(value.evidence, `${path}.evidence`).forEach((id) => string(id, `${path}.evidence.id`));
}
function parseSeries(value: unknown): void {
  const series = object(value, 'series');
  string(series.id, 'series.id');
  const state = object(series.state, 'series.state');
  member(state.kind, ['Existing', 'Prospective'], 'series.state.kind');
  if (state.kind === 'Existing') number(state.revision, 'series.state.revision');
  source(series, 'series');
  parseRecurrence(series.rule);
}
export function parseTemporalContext(value: unknown): TemporalContext {
  const temporal = object(value, 'context.temporal');
  range(temporal.horizon, 'context.temporal.horizon');
  member(temporal.coverage, ['Complete', 'Incomplete'], 'context.temporal.coverage');
  const limits = object(temporal.limits, 'context.temporal.limits');
  number(limits.max_occurrences, 'limits.max_occurrences');
  number(limits.max_dates, 'limits.max_dates');
  array(temporal.series, 'context.temporal.series').forEach(parseSeries);
  return value as TemporalContext;
}
export function parseCompiledContext(value: unknown): CompiledContextSnapshot {
  const compiled = object(value, 'compiled_context');
  number(compiled.source_revision, 'compiled_context.source_revision');
  range(compiled.horizon, 'compiled_context.horizon');
  parseAvailability(compiled.availability);
  for (const raw of array(compiled.series, 'compiled_context.series')) {
    const item = object(raw, 'series_expansion');
    parseSeries(item.series);
    parseExpansion(item.expansion);
  }
  for (const raw of array(compiled.occurrences, 'compiled_context.occurrences')) {
    const item = object(raw, 'materialized_occurrence');
    shape.identifier(item.id, 'materialized_occurrence.id');
    shape.identifier(item.series_id, 'materialized_occurrence.series_id');
    number(item.source_revision, 'materialized_occurrence.source_revision');
    source(item, 'materialized_occurrence');
    parseOccurrence(item.occurrence);
  }
  return value as CompiledContextSnapshot;
}
