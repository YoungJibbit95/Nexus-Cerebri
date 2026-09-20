import type { EvidenceField, PlanReason, RejectionReason, TimeRange } from './contracts.ts';

export function readable(value: string): string {
  return value.replace(/([a-z])([A-Z])/g, '$1 $2').replaceAll('_', ' ');
}
export function time(value: string, zone = 'UTC'): string {
  return new Intl.DateTimeFormat('en-GB', { hour: '2-digit', minute: '2-digit', timeZone: zone }).format(new Date(value));
}
export function date(value: string): string {
  return new Intl.DateTimeFormat('en-GB', { day: '2-digit', month: 'short', year: 'numeric', timeZone: 'UTC' }).format(new Date(value));
}
export function known<T>(field: EvidenceField<T>): T | undefined {
  return field.value.processing === 'RESOLVED' && field.value.knowledge.state === 'KNOWN' ? field.value.knowledge.data : undefined;
}
export function knowledgeLabel<T>(field: EvidenceField<T>): string {
  return field.value.processing === 'UNRESOLVED' ? 'UNRESOLVED' : field.value.knowledge.state;
}
/** Clip only the painted shape to the display viewport; the source interval is unchanged. */
export function intervalStyle(range: TimeRange, horizon: TimeRange): string {
  const start = Date.parse(horizon.start);
  const width = Date.parse(horizon.end) - start;
  const left = Math.max(0, Math.min(100, (Date.parse(range.start) - start) / width * 100));
  const right = Math.max(0, Math.min(100, (Date.parse(range.end) - start) / width * 100));
  return `left:${left}%;width:${Math.max(0, right - left)}%`;
}
export function reasonLabel(reason: PlanReason): string {
  return typeof reason === 'string' ? readable(reason) : `Preferred start · ${readable(reason.PreferredStart)}`;
}
export function rejectionLabel(reason: RejectionReason): string {
  if (typeof reason === 'string') return readable(reason);
  if ('HardConstraint' in reason) {
    const violation = reason.HardConstraint;
    const blockers = violation.evidence.blocking_objects;
    return `${readable(violation.reason)}${blockers.length ? ` · ${blockers.join(', ')}` : ''}`;
  }
  const [kind, id] = Object.entries(reason)[0];
  return `${readable(kind)} · ${id}`;
}
