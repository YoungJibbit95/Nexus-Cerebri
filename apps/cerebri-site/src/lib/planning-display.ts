import { runtimeData } from '$lib/generated/runtime-data';

// Display coordinates and identity only; classification and order remain Rust output.
export const planner = runtimeData.plannerResult;
export const request = runtimeData.request;
export const scopeStart = Date.parse(request.scope.time_range.start);
export const scopeEnd = Date.parse(request.scope.time_range.end);
const span = scopeEnd - scopeStart;
export const timeLabel = (instant: number | string) => new Date(instant).toISOString().slice(11, 16);
export const timePosition = (instant: number | string) => 100 * (new Date(instant).getTime() - scopeStart) / span;
export const durationPosition = 100 * request.duration.value.knowledge.data * 1000 / span;
export const ranked = planner.candidates;
const rejected = new Set<string>(planner.conflicts.rejections.map(entry => entry.start));
const valid = new Set<string>(ranked.map(entry => entry.start));
export const candidates = Array.from({ length: planner.search_space.evaluated }, (_, index) => {
  const instant = scopeStart + index * request.granularity * 1000;
  const start = new Date(instant).toISOString().replace('.000Z', 'Z');
  return { id: 'C' + String(index + 1).padStart(2, '0'), start, instant,
    label: timeLabel(instant), position: timePosition(instant),
    state: start === ranked[0]?.start ? 'selected' : rejected.has(start) ? 'rejected' : valid.has(start) ? 'valid' : 'unseen' };
});
export const candidateIdentity = (start: string) => candidates.find(candidate => candidate.start === start)?.id ?? start;
const busyRange = request.context.objects.find(object => object.id === 'busy')?.time.value.knowledge.data;
export const busy = busyRange ? { position: timePosition(busyRange.start), width: timePosition(busyRange.end) - timePosition(busyRange.start) } : null;
export const keyFields = [
  { key: 'preference_distance_seconds', label: 'preferred distance', render: (value: string | number) => value + 's' },
  { key: 'mutation_count', label: 'mutation count', render: String },
  { key: 'shifted_seconds', label: 'shift seconds', render: (value: string | number) => value + 's' },
  { key: 'start', label: 'start', render: (value: string | number) => timeLabel(value) + ' UTC' },
  { key: 'object_id', label: 'object id', render: String }
] as const;
// Inspect the first difference in two already ordered records; never sort or rank.
export const decisiveIndex = keyFields.findIndex(field => ranked[0]?.ordering_key[field.key] !== ranked[1]?.ordering_key[field.key]);
