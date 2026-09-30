/** Provisional process bridge: Rust validates input and produces the output contract. */
export function plan(request: unknown, options?: { binary?: string }): Promise<unknown>;
export { CerebriBridgeError, describeIntegration, suggest } from './integration.mjs';
export type { BridgeOptions, IntegrationManifest, SuggestionRequest, SuggestionResponse, SuggestionRejection, JsonValue, JsonObject } from './integration.mjs';
