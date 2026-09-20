/** Transport shapes for the internal CPIR 0.1 and Rust planning API.
 * These types describe data, never confer validation or execution authority. */
export type Json = null | boolean | number | string | Json[] | { [key: string]: Json };
export type JsonObject = { [key: string]: Json };
export interface TimeRange { start: string; end: string }
export type Knowledge<T> =
  | { state: 'KNOWN'; data: T }
  | { state: 'MISSING' | 'UNKNOWN' }
  | { state: 'UNCERTAIN'; data: { value: T; confidence: number } }
  | { state: 'AMBIGUOUS'; data: T[] };
export interface EvidenceField<T> {
  value: { processing: 'UNRESOLVED' } | { processing: 'RESOLVED'; knowledge: Knowledge<T> };
  provenance: string;
  confidence: number | null;
  evidence: string[];
}
export interface PlanningObject {
  id: string;
  revision: number | null;
  kind: { kind: string; details?: Json };
  timezone: string;
  time: EvidenceField<TimeRange>;
  semantics: Record<string, EvidenceField<number>> | null;
}
export interface PlanningRequest {
  schema_version: { major: number; minor: number };
  request_id: string;
  trace_id: string;
  operation: string;
  scope: { time_range: TimeRange; max_mutations: number };
  context: { revision: number; objects: PlanningObject[] };
  duration: EvidenceField<number>;
  target_ids: string[];
  granularity: number;
  budget: { max_candidates: number };
  preferences: { preferences: { source: string; preferred_start: string; evidence: string[] }[] };
}
export type PlanReason = 'FeasibleWithinScope' | 'EarliestTieBreak' | 'AnalysisOnly' | 'DurationUncertain' | { PreferredStart: string };
export interface ScoreComponent { reason: PlanReason; cost: number }
export interface RankedCandidate {
  proposed: { id: string; source_revision: number; placements: { object_id: string; range: TimeRange }[] };
  cost: number;
  mutation_count: number;
  shifted_seconds: number;
  start: string;
  object_id: string;
  explanation: ScoreComponent[];
}
export interface ConstraintViolation {
  constraint: Json;
  object_id: string;
  reason: string;
  evidence: { facts: string[]; blocking_objects: string[] };
}
export type RejectionReason = 'MutationLimit' | 'MovedObjectLimit' | 'InvalidTargets'
  | { Scope: string } | { InvalidDuration: string } | { Capability: string } | { Policy: string }
  | { HardConstraint: ConstraintViolation };
export interface CandidateRejection { start: string; reasons: RejectionReason[] }
export interface ValidationReport {
  state: 'Valid' | 'ValidWithUncertainty' | 'InsufficientInformation';
  issues: Json[];
}
export interface PlanningResult {
  outcome: 'Solution' | 'NoSolution' | 'NeedsRelaxation' | 'InsufficientInformation';
  assessment: 'ProvenOptimal' | 'Complete' | 'BestFound';
  validation: ValidationReport;
  candidates: RankedCandidate[];
  conflicts: { rejections: CandidateRejection[] };
  search_space: { horizon: TimeRange; granularity: number; objective: string; evaluated: number; exhausted: boolean };
}
export type ExplanationMode = 'Simple' | 'Technical' | 'Research';
export type LabView = 'Planner' | 'Temporal' | 'Trace' | 'Semantics' | 'Preferences' | 'ML' | 'Dataset';
export interface ConsoleEntry {
  id: number;
  at: string;
  level: 'info' | 'success' | 'error';
  source: 'lab' | 'planner' | 'validator' | 'temporal';
  message: string;
  data?: unknown;
}
