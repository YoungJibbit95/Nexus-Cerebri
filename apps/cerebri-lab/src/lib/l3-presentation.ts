import type { TimeRange, PlanningRequest, PlanningResult } from './contracts.ts';
import type { ExpansionReport, RecurrenceRule } from './temporal.ts';

/** Presentation selectors only: no expansion, timezone resolution or planning decisions. */
export function nominalEvidence(expansion: ExpansionReport, rule?: RecurrenceRule) {
  return [
    ...expansion.occurrences.map((occurrence) => ({ key: 'occurrence-' + occurrence.sequence, sequence: occurrence.sequence, date: occurrence.date, localTime: occurrence.local_time, occurrence, skip: null })),
    ...expansion.skipped.map((skip) => ({ key: 'skip-' + skip.sequence, sequence: skip.sequence, date: skip.date, localTime: rule?.local_time ?? null, occurrence: null, skip })),
  ].sort((a, b) => a.sequence - b.sequence);
}
export function localPosition(value: string) {
  const [hours, minutes, seconds] = value.split(':').map(Number);
  return (hours * 3600 + minutes * 60 + seconds) / 86400 * 100;
}
export function comparisonFrame(full: TimeRange, horizon: TimeRange): TimeRange {
  return { start: Date.parse(full.start) < Date.parse(horizon.start) ? full.start : horizon.start,
    end: Date.parse(full.end) > Date.parse(horizon.end) ? full.end : horizon.end };
}
export function traceSections(result: PlanningResult, request: PlanningRequest | null) {
  return [
    ...(request ? [{ label: 'CPIR', value: request, meaning: 'The exact request sent to the Rust core.' }] : []),
    { label: 'Validation', value: result.validation, meaning: 'Can this request be considered? Validation is separate from execution authorization.' },
    ...(result.compilation ? [{ label: 'Compilation', value: result.compilation, meaning: 'Existing temporal sources became a bounded occupancy snapshot.' }] : []),
    { label: 'Search', value: result.search_space, meaning: `${result.search_space.evaluated} positions were evaluated within the declared grid.` },
    { label: 'Constraint checks', value: result.conflicts, meaning: `${result.conflicts.rejections.length} positions were rejected. Hard failures are excluded before ranking.` },
    { label: 'Feasible set', value: result.candidates, meaning: `${result.candidates.length} feasible candidates survived.` },
    { label: 'Deterministic order', value: result.candidates.map(({ ordering_key, ranking_features }) => ({ ordering_key, ranking_features })), meaning: 'Returned lexicographic keys order the feasible set. The first unequal field decides.' },
    { label: 'PlanningResult', value: result, meaning: result.candidates.length ? 'Candidate 1 is the first proposal. It has not been executed.' : 'This completed result has no feasible proposal.' },
  ];
}
