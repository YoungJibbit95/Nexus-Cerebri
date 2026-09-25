import type { PlanningRequest, PlanningResult, ValidationReport } from './contracts.ts';
import { parseCompiledContext, parseTemporalContext } from './compilation.ts';
import { parseValidationIssue, parseDependencyIssue, parseEdge } from './validation.ts';

import { shape } from './shape.ts';
const { object, array, string, number, instant, range, member } = shape;
export function parseValidation(value: unknown): ValidationReport {
  const item = object(value, 'validation');
  member(item.state, ['Valid', 'ValidWithUncertainty', 'InsufficientInformation'], 'validation.state');
  array(item.issues, 'validation.issues').forEach(parseValidationIssue);
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
  if (context.temporal != null) parseTemporalContext(context.temporal);
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
/** Inspect the wire contract only; never recompute features or rank in the client. */
function parseRankingFeatures(value: unknown): void {
  const item = object(value, 'ranking_features');
  const fields = ['schema_version', 'preferred_start_distance_seconds', 'preferred_start_source', 'mutation_count', 'shift_seconds'];
  if (Object.keys(item).length !== fields.length || !fields.every((key) => Object.hasOwn(item, key))) throw new Error('ranking_features: expected every v0.1 field, without extras.');
  const version = object(item.schema_version, 'ranking_features.schema_version');
  if (Object.keys(version).length !== 2 || version.major !== 0 || version.minor !== 1) throw new Error('ranking_features: unsupported schema version.');
  if (item.preferred_start_distance_seconds !== null) number(item.preferred_start_distance_seconds, 'ranking_features.preferred_start_distance_seconds');
  if (item.preferred_start_source !== null) member(item.preferred_start_source, ['ExplicitCurrentRequest', 'SessionContext', 'PersonalLearned', 'GlobalLearned', 'Default'], 'ranking_features.preferred_start_source');
  if ((item.preferred_start_distance_seconds === null) !== (item.preferred_start_source === null)) throw new Error('ranking_features: distance and source must both be null or both have values.');
  if (number(item.mutation_count, 'ranking_features.mutation_count') > 0xffffffff) throw new Error('ranking_features.mutation_count: exceeds u32.');
  number(item.shift_seconds, 'ranking_features.shift_seconds');
}
export function parseResult(value: unknown): PlanningResult {
  const item = object(value, 'result');
  member(item.outcome, ['Solution', 'NoSolution', 'NeedsRelaxation', 'InsufficientInformation'], 'outcome');
  member(item.assessment, ['ProvenOptimal', 'Complete', 'BestFound'], 'assessment');
  parseValidation(item.validation);
  if (item.compilation !== null) parseCompiledContext(item.compilation);
  const graph = object(item.dependency_graph, 'dependency_graph');
  for (const key of ['nodes', 'order']) array(graph[key], `dependency_graph.${key}`).forEach((id) => string(id, `dependency_graph.${key}.id`));
  array(graph.edges, 'dependency_graph.edges').forEach(parseEdge);
  array(graph.issues, 'dependency_graph.issues').forEach(parseDependencyIssue);
  const search = object(item.search_space, 'search_space');
  range(search.horizon, 'search_space.horizon');
  number(search.granularity, 'search_space.granularity');
  number(search.evaluated, 'search_space.evaluated');
  string(search.objective, 'search_space.objective');
  if (typeof search.exhausted !== 'boolean') throw new Error('search_space.exhausted: expected a boolean.');
  for (const raw of array(item.candidates, 'candidates')) {
    const candidate = object(raw, 'candidate');
    parseRankingFeatures(candidate.ranking_features);
    instant(candidate.start, 'candidate.start');
    string(candidate.object_id, 'candidate.object_id');
    for (const key of ['cost', 'mutation_count', 'shifted_seconds']) number(candidate[key], `candidate.${key}`);
    const ordering = object(candidate.ordering_key, 'candidate.ordering_key');
    for (const key of ['preference_distance_seconds', 'mutation_count', 'shifted_seconds']) number(ordering[key], `ordering_key.${key}`);
    instant(ordering.start, 'ordering_key.start');
    shape.identifier(ordering.object_id, 'ordering_key.object_id');
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
          for (const key of ['facts', 'blocking_objects', 'blocking_occurrences']) array(evidence[key], `evidence.${key}`).forEach((v) => string(v, key));
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

export { shape } from './shape.ts';
