export type JsonValue = null | boolean | number | string | JsonValue[] | JsonObject;
export interface JsonObject { [key: string]: JsonValue }
export interface BridgeOptions {
  /** Trusted host configuration only; never supplied by a renderer or remote request. */
  binary?: string;
  /** Process lifetime deadline in milliseconds, 1..60000; default 10000. Not a planner budget. */
  timeoutMs?: number;
  signal?: AbortSignal;
}
export type BridgeErrorCode = 'invalid_request' | 'request_too_large' | 'invalid_options'
  | 'aborted' | 'timeout' | 'bridge_unavailable' | 'bridge_failed' | 'response_too_large'
  | 'invalid_response' | 'incompatible_bridge';
export class CerebriBridgeError extends Error {
  readonly code: BridgeErrorCode;
  constructor(code: BridgeErrorCode);
}
export interface IntegrationManifest {
  integration_version: { major: 0; minor: 1 };
  software_version: string;
  profile: 'single_event_suggestion';
  cpir_schema_version: { major: 0; minor: 2 };
  operations: ['FIND_SLOT'];
  deployment_modes: ['Test', 'Shadow', 'Suggestion'];
  temporal_coverage_required: true;
  execution_supported: false;
  max_request_bytes: 262144;
}
export interface SuggestionRequest {
  integration_version: { major: 0; minor: 1 };
  /** CPIR 0.2 JSON; the Rust decoder and planner own domain validation. */
  request: JsonObject;
}
export type SuggestionRejection = 'invalid_request' | 'request_too_large'
  | 'unsupported_integration_version' | 'unsupported_cpir_version' | 'unsupported_operation'
  | 'mutation_authority_forbidden' | 'unsupported_deployment_mode' | 'prospective_event_required'
  | 'temporal_coverage_required' | 'uncertain_duration_forbidden';
export type SuggestionResponse = {
  integration_version: { major: 0; minor: 1 };
} & ({
  status: 'rejected'; code: SuggestionRejection;
} | {
  status: 'planned'; request_id: string; trace_id: string; context_revision: number;
  /** Full, unchanged Rust PlanningResult. Decode consumed fields before presentation.
   * This envelope declaration deliberately does not assert a checked domain DTO. */
  result: JsonObject;
});
export function describeIntegration(options?: BridgeOptions): Promise<IntegrationManifest>;
export function suggest(request: SuggestionRequest, options?: BridgeOptions): Promise<SuggestionResponse>;
