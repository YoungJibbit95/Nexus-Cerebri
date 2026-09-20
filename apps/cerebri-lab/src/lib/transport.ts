import type { PlanningRequest, PlanningResult, ValidationReport } from './contracts.ts';

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
export function parseValidation(value: unknown): ValidationReport {
  const item = object(value, 'validation');
  member(item.state, ['Valid', 'ValidWithUncertainty', 'InsufficientInformation'], 'validation.state');
  array(item.issues, 'validation.issues');
  return value as ValidationReport;
}
/** Presentation guards protect views from malformed imports; Rust remains the CPIR validator. */
export function parseRequest(value: unknown): PlanningRequest {
  const item = object(value, 'request');
  const version = object(item.schema_version, 'schema_version');
  number(version.major, 'schema_version.major');
  number(version.minor, 'schema_version.minor');
  for (const key of ['request_id', 'trace_id', 'operation']) string(item[key], key);
  const scope = object(item.scope, 'scope');
  range(scope.time_range, 'scope.time_range');
  number(scope.max_mutations, 'scope.max_mutations');
  number(item.granularity, 'granularity');
  number(object(item.budget, 'budget').max_candidates, 'budget.max_candidates');
  array(item.target_ids, 'target_ids').forEach((id) => string(id, 'target_id'));
  const context = object(item.context, 'context');
  number(context.revision, 'context.revision');
  for (const raw of array(context.objects, 'context.objects')) {
    const entry = object(raw, 'object');
    for (const key of ['id', 'timezone']) string(entry[key], `object.${key}`);
    string(object(entry.kind, 'object.kind').kind, 'object.kind.kind');
    checkEvidence(entry.time, 'object.time', range);
    if (entry.semantics != null) {
      for (const [key, field] of Object.entries(object(entry.semantics, 'object.semantics'))) {
        checkEvidence(field, `semantics.${key}`, (v, p) => {
          if (typeof v !== 'number' || !Number.isFinite(v)) throw new Error(`${p}: expected a finite number.`);
        });
      }
    }
  }
  checkEvidence(item.duration, 'duration', number);
  for (const raw of array(object(item.preferences, 'preferences').preferences, 'preferences.preferences')) {
    const pref = object(raw, 'preference');
    string(pref.source, 'preference.source');
    instant(pref.preferred_start, 'preference.preferred_start');
    array(pref.evidence, 'preference.evidence');
  }
  return value as PlanningRequest;
}
function checkEvidence(value: unknown, path: string, checkData: (value: unknown, path: string) => void): void {
  const field = object(value, path);
  string(field.provenance, `${path}.provenance`);
  array(field.evidence, `${path}.evidence`);
  const state = object(field.value, `${path}.value`);
  member(state.processing, ['UNRESOLVED', 'RESOLVED'], `${path}.processing`);
  if (state.processing === 'RESOLVED') {
    const knowledge = object(state.knowledge, `${path}.knowledge`);
    member(knowledge.state, ['KNOWN', 'MISSING', 'UNKNOWN', 'UNCERTAIN', 'AMBIGUOUS'], `${path}.knowledge.state`);
    if (knowledge.state === 'KNOWN') checkData(knowledge.data, `${path}.data`);
    if (knowledge.state === 'UNCERTAIN') checkData(object(knowledge.data, `${path}.data`).value, `${path}.data.value`);
    if (knowledge.state === 'AMBIGUOUS') array(knowledge.data, `${path}.data`).forEach((v) => checkData(v, `${path}.candidate`));
  }
}
export function parseResult(value: unknown): PlanningResult {
  const item = object(value, 'result');
  member(item.outcome, ['Solution', 'NoSolution', 'NeedsRelaxation', 'InsufficientInformation'], 'outcome');
  member(item.assessment, ['ProvenOptimal', 'Complete', 'BestFound'], 'assessment');
  parseValidation(item.validation);
  const search = object(item.search_space, 'search_space');
  range(search.horizon, 'search_space.horizon');
  number(search.granularity, 'search_space.granularity');
  number(search.evaluated, 'search_space.evaluated');
  string(search.objective, 'search_space.objective');
  if (typeof search.exhausted !== 'boolean') throw new Error('search_space.exhausted: expected a boolean.');
  for (const raw of array(item.candidates, 'candidates')) {
    const candidate = object(raw, 'candidate');
    instant(candidate.start, 'candidate.start');
    string(candidate.object_id, 'candidate.object_id');
    for (const key of ['cost', 'mutation_count', 'shifted_seconds']) number(candidate[key], `candidate.${key}`);
    const proposed = object(candidate.proposed, 'candidate.proposed');
    string(proposed.id, 'proposed.id');
    number(proposed.source_revision, 'proposed.source_revision');
    for (const rawPlacement of array(proposed.placements, 'placements')) {
      const placement = object(rawPlacement, 'placement');
      string(placement.object_id, 'placement.object_id');
      range(placement.range, 'placement.range');
    }
    for (const rawScore of array(candidate.explanation, 'explanation')) {
      const score = object(rawScore, 'score');
      number(score.cost, 'score.cost');
      if (typeof score.reason === 'string') member(score.reason, ['FeasibleWithinScope', 'EarliestTieBreak', 'AnalysisOnly', 'DurationUncertain'], 'score.reason');
      else string(object(score.reason, 'score.reason').PreferredStart, 'score.reason.PreferredStart');
    }
  }
  const conflicts = object(item.conflicts, 'conflicts');
  for (const raw of array(conflicts.rejections, 'conflicts.rejections')) {
    const rejection = object(raw, 'rejection');
    instant(rejection.start, 'rejection.start');
    for (const reason of array(rejection.reasons, 'rejection.reasons')) {
      if (typeof reason === 'string') member(reason, ['MutationLimit', 'MovedObjectLimit', 'InvalidTargets'], 'rejection.reason');
      else {
        const tagged = object(reason, 'rejection.reason');
        if ('HardConstraint' in tagged) {
          const violation = object(tagged.HardConstraint, 'HardConstraint');
          string(violation.reason, 'violation.reason');
          string(violation.object_id, 'violation.object_id');
          const evidence = object(violation.evidence, 'violation.evidence');
          for (const key of ['facts', 'blocking_objects']) array(evidence[key], `evidence.${key}`).forEach((v) => string(v, key));
        } else {
          const keys = Object.keys(tagged);
          if (keys.length !== 1) throw new Error('Expected a tagged rejection reason.');
          member(keys[0], ['Scope', 'InvalidDuration', 'Capability', 'Policy'], 'rejection.reason');
          string(tagged[keys[0]], 'rejection.object_id');
        }
      }
    }
  }
  return value as PlanningResult;
}
export async function postJson(path: string, value: unknown): Promise<unknown> {
  const body = JSON.stringify(value);
  if (new TextEncoder().encode(body).length > 256 * 1024) throw new Error('Request exceeds the API limit of 256 KiB.');
  const response = await fetch(path, {
    method: 'POST', headers: { 'content-type': 'application/json' }, body,
    signal: AbortSignal.timeout(20_000),
  });
  if (!response.ok) {
    const detail = (await response.text()).slice(0, 1200);
    throw new Error(`API ${response.status}: ${detail || response.statusText}`);
  }
  return response.json();
}
export async function readJsonFile(file: File, maxBytes = 8 * 1024 * 1024): Promise<unknown> {
  if (file.size > maxBytes) throw new Error(`File exceeds the ${Math.round(maxBytes / 1024)} KiB import limit.`);
  return JSON.parse(await file.text()) as unknown;
}
export function downloadJson(value: unknown, filename: string): void {
  const url = URL.createObjectURL(new Blob([JSON.stringify(value, null, 2)], { type: 'application/json' }));
  const link = document.createElement('a');
  link.href = url;
  link.download = filename;
  link.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}

export const shape = { object, array, string, number, instant, range, member };
