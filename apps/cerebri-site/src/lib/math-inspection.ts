import { runtimeData } from '$lib/generated/runtime-data';

export const mathData = runtimeData.mathInspection;

// Presentation coordinates and units only. Domain observations come from Rust.
export const timeLabel = (instant: string) => instant.slice(11, 19);
export const secondsLabel = (milliseconds: number) => String(milliseconds / 1000);
export function position(instant: string, start: string, end: string) {
  return 100 * (Date.parse(instant) - Date.parse(start)) / (Date.parse(end) - Date.parse(start));
}

export const orderingDimensions = [
  { key: 'preference_distance_seconds', label: 'Preferred distance', symbol: 'd', description: 'Legacy numeric projection of the preferred-start observation. Absent evidence projects to zero.' },
  { key: 'mutation_count', label: 'Mutations', symbol: 'm', description: 'Mutation count supplied by the Rust planner for this request.' },
  { key: 'shifted_seconds', label: 'Shift', symbol: 's', description: 'Ordering-key field shifted_seconds carries the observation shift_seconds.' },
  { key: 'start', label: 'Start', symbol: 't', description: 'Exact candidate instant, including subseconds where present.' },
  { key: 'object_id', label: 'Object ID', symbol: 'id', description: 'The final deterministic tie-break dimension.' }
] as const;
