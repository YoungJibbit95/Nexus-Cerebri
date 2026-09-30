import type { CandidateOrderingKey, PlanningRequest, PlanningResult, RankedCandidate } from './contracts.ts';
import { known, readable } from './presentation.ts';

/** Geometry and disclosure only. Never generates, filters or sorts planner candidates. */
export function visibleCandidates(result: PlanningResult | null, selected: number, limit = 8) {
  const items = result?.candidates.slice(0, limit).map((candidate, index) => ({ candidate, index })) ?? [];
  if (result?.candidates[selected] && selected >= limit) items.push({ candidate: result.candidates[selected], index: selected });
  return items;
}
export const orderingFields: { key: keyof CandidateOrderingKey; label: string; explanation: string }[] = [
  { key: 'preference_distance_seconds', label: 'Preferred distance', explanation: 'Closer to the supplied preferred time' },
  { key: 'mutation_count', label: 'Mutations', explanation: 'Fewer proposed changes' },
  { key: 'shifted_seconds', label: 'Shift', explanation: 'Less movement from the original placement' },
  { key: 'start', label: 'Start', explanation: 'An earlier start breaks the tie' },
  { key: 'object_id', label: 'Object identity', explanation: 'Object identity breaks the remaining tie' },
];
function instantIdentity(value: string): string {
  // Date geometry is millisecond-based; equality of ordering instants retains submilliseconds.
  return `${Date.parse(value)}:${(/\.(\d+)/.exec(value)?.[1] ?? '').padEnd(9, '0').slice(3)}`;
}
export function firstDifference(a: RankedCandidate, b: RankedCandidate): number {
  return orderingFields.findIndex(({ key }) => key === 'start'
    ? instantIdentity(a.ordering_key.start) !== instantIdentity(b.ordering_key.start)
    : a.ordering_key[key] !== b.ordering_key[key]);
}
export function requestSentence(request: PlanningRequest): string {
  const duration = known(request.duration);
  const verb = request.operation === 'MOVE' ? 'Move' : request.operation === 'CREATE' ? 'Place' : 'Plan';
  return duration === undefined ? `${verb} the target. Its duration is ${readable(request.duration.value.processing === 'UNRESOLVED' ? 'UNRESOLVED' : request.duration.value.knowledge.state).toLowerCase()}.`
    : `${verb} ${duration / 60} minutes within this time window.`;
}
export function runCounts(result: PlanningResult) {
  return { evaluated: result.search_space.evaluated, rejected: result.conflicts.rejections.length, feasible: result.candidates.length };
}
export function blockerIds(result: PlanningResult | null, index: number | null): string[] {
  if (index === null) return [];
  return result?.conflicts.rejections[index]?.reasons.flatMap((reason) => typeof reason !== 'string' && 'HardConstraint' in reason
    ? [...reason.HardConstraint.evidence.blocking_objects, ...reason.HardConstraint.evidence.blocking_occurrences] : []) ?? [];
}
