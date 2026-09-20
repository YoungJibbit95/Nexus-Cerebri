import manifest from '../../../../examples/planner/manifest.json';
import { parseRequest } from './transport.ts';

const inputs = import.meta.glob('../../../../examples/planner/*.json', { eager: true, import: 'default' });
/** Only checked-in source requests are bundled. Every result still comes from the Rust API. */
export const plannerScenarios = manifest.scenarios.filter((scenario) => scenario.route === '/v1/plan');
export function scenarioRequest(file: string) {
  if (!plannerScenarios.some((scenario) => scenario.file === file)) throw new Error('Unknown planner scenario.');
  return parseRequest(structuredClone(inputs[`../../../../examples/planner/${file}`]));
}
