import type { ExplanationMode } from './contracts.ts';

export const DEPTH_CONTEXT = Symbol('lab-presentation-depth');
export const DEPTH_STORAGE_KEY = 'cerebri-lab-depth';
export const depths: { mode: ExplanationMode; label: string; hint: string; number: string }[] = [
  { mode: 'Simple', label: 'Understand', hint: 'What happened, and why', number: '01' },
  { mode: 'Technical', label: 'Technical', hint: 'Rules, mechanics, evidence', number: '02' },
  { mode: 'Research', label: 'Research', hint: 'Complete state and source data', number: '03' },
];
export function storedDepth(value: string | null): ExplanationMode {
  return depths.find((depth) => depth.mode === value)?.mode ?? 'Simple';
}
