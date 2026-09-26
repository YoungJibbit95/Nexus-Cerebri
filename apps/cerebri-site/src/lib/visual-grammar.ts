export type SemanticKind =
  | 'fact'
  | 'constraint'
  | 'preference'
  | 'scope'
  | 'candidate'
  | 'violation'
  | 'result'
  | 'authority'
  | 'research';

export type SemanticLegendItem = {
  kind: SemanticKind;
  label: string;
  detail?: string;
};

export const plannerSemanticLegend: SemanticLegendItem[] = [
  { kind: 'scope', label: 'Scope', detail: 'where search may occur' },
  { kind: 'fact', label: 'Known state', detail: 'supplied reality' },
  { kind: 'constraint', label: 'Hard constraint', detail: 'must not be violated' },
  { kind: 'preference', label: 'Preference', detail: 'orders valid candidates' },
  { kind: 'candidate', label: 'Candidate', detail: 'considered placement' },
  { kind: 'violation', label: 'Rejected', detail: 'failed validity' },
  { kind: 'result', label: 'First result', detail: 'Rust-produced order' }
];
