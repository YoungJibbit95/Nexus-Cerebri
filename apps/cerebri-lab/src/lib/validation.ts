// Presentation shape checks only; validity and planning decisions remain in Rust.
import { shape } from './shape.ts';
const { object, array, member, identifier } = shape;
function tagged(value: unknown, path: string): [string, unknown] {
  const item = object(value, path);
  const keys = Object.keys(item);
  if (keys.length !== 1) throw new Error(`${path}: expected exactly one variant.`);
  return [keys[0], item[keys[0]]];
}
export function parseEdge(value: unknown): void {
  const edge = object(value, 'edge');
  identifier(edge.predecessor, 'edge.predecessor');
  identifier(edge.dependent, 'edge.dependent');
}
export function parseDependencyIssue(value: unknown): void {
  if (typeof value === 'string') { member(value, ['InputLimit'], 'dependency.issue'); return; }
  const [kind, data] = tagged(value, 'dependency.issue');
  member(kind, ['MissingReference', 'Cycle'], 'dependency.issue');
  if (kind === 'MissingReference') parseEdge(data);
  else {
    const cycle = object(data, 'cycle');
    array(cycle.members, 'cycle.members').forEach((v) => identifier(v, 'cycle.member'));
    array(cycle.edges, 'cycle.edges').forEach(parseEdge);
  }
}
function required(value: unknown): void {
  member(value, ['Missing', 'Unknown', 'Uncertain', 'Ambiguous', 'Unresolved'], 'required_field');
}
function compilation(value: unknown): void {
  if (typeof value === 'string') { member(value, ['HorizonMismatch', 'InputLimit'], 'compilation.error'); return; }
  const [kind, data] = tagged(value, 'compilation.error');
  member(kind, ['DuplicateSeries', 'IdentityCollision', 'ProspectiveSeries', 'InvalidProvenance', 'UnknownObjectTime', 'Temporal'], 'compilation.error');
  if (kind === 'Temporal') member(data, ['InvalidRange', 'InvalidDuration', 'Overflow', 'AmbiguousLocalTime', 'NonexistentLocalTime', 'InvalidRecurrence', 'InputLimit', 'OccurrenceLimitExceeded', 'DateLimitExceeded'], 'temporal.error');
  else identifier(data, 'compilation.identity');
}
export function parseValidationIssue(value: unknown): void {
  if (typeof value === 'string') {
    member(value, ['UnsupportedSchema', 'PlanningPermissionDenied', 'NoTargets', 'UnsupportedSearchBudget', 'UnsupportedTargetCount', 'InputLimit', 'IncompleteCoverage'], 'validation.issue');
    return;
  }
  const [kind, data] = tagged(value, 'validation.issue');
  switch (kind) {
    case 'RequiredDuration': required(data); break;
    case 'RequiredTime': {
      const time = object(data, 'required_time'); identifier(time.object_id, 'required_time.object_id'); required(time.reason); break;
    }
    case 'UnsupportedOperation': member(data, ['CREATE', 'MOVE', 'UPDATE', 'CANCEL', 'FIND_SLOT', 'RESCHEDULE', 'OPTIMIZE', 'PLAN', 'ANALYZE'], 'operation'); break;
    case 'Compilation': compilation(data); break;
    case 'Dependency': parseDependencyIssue(data); break;
    default:
      member(kind, ['UnsupportedObject', 'DuplicateObject', 'DuplicateFact', 'DuplicateTarget', 'UnknownObject', 'ContradictoryFact', 'InvalidFactSource', 'ScopeViolation', 'MissingPlanningCapability', 'PolicyDenied', 'InvalidTargetState'], 'validation.issue');
      identifier(data, 'validation.identity');
  }
}
