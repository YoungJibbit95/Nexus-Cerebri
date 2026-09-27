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
  { kind: 'scope', label: 'Time window', detail: 'limits this search' },
  { kind: 'fact', label: 'Known appointment', detail: 'occupies this time' },
  { kind: 'constraint', label: 'Required rule', detail: 'every option must pass' },
  { kind: 'preference', label: 'Preference', detail: 'helps compare valid options' },
  { kind: 'candidate', label: 'Candidate', detail: 'one possible appointment' },
  { kind: 'violation', label: 'Rejected', detail: 'fails a required check' },
  { kind: 'result', label: 'First proposal', detail: 'first in the planner’s order' }
];
