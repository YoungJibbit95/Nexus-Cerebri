# ADR-0016: Application-independent suggestion boundary

Date: 2026-09-30
Status: Proposed; bounded implementation for review. Independent Architecture/Security qualification pending.

## Context

Nexus needs a first integration that other applications can also use. Its Flux workflow
and Task deadlines do not by themselves describe occupied time or work duration. The
first planning problem is a prospective single Event inside an explicit bounded window.
The existing general CPIR facade also accepts legacy snapshots without explicit temporal
coverage; new consumers need a narrower admission contract before relying on free time.

## Decision under review

Add `cerebri-core::integration` with a separately versioned `0.1` contract and one profile,
`single_event_suggestion`. A manifest describes this profile, not all Cerebri capabilities
or release qualification. Requests carry `integration_version` and an unchanged CPIR
`request`. Admission requires CPIR 0.2, FIND_SLOT, exactly one prospective Event target,
an explicit temporal context, Test/Shadow/Suggestion mode, no uncertain-duration policy,
zero scope/policy mutation limits, no allowed mutation actions and no mutation grants.
Reject incompatible input rather than silently converting or relaxing it.

The admitted request goes unchanged to the existing Rust planner. Read/plan permission,
scope, provenance, temporal horizon equality, knowledge states, constraints, ranking and
resource admission remain existing core authorities. Incomplete temporal coverage reaches
the planner and returns InsufficientInformation with its original evidence. A caller's
Complete assertion is neither authenticated provider completeness nor permission.

The response distinguishes profile/decoding rejection from a completed planner call.
`planned` carries request/trace/context identity and the entire unchanged PlanningResult;
it does not imply Solution. All proposals remain analysis-only: even after validation,
conversion to ActionPlan must fail with AnalysisOnly. No executor is exposed.

REST adds `/v1/integration/manifest` and `/v1/integration/suggestions`; the provisional
Node bridge adds `--describe-integration` and `--suggest`. Both delegate to core.
Existing routes and the default Node `plan` protocol keep their compatibility behavior.
The new host-side Node client bounds process lifetime and output, supports cancellation,
checks envelope compatibility/correlation, and exposes fixed transport error codes without
raw stderr/parser content. Its TypeScript contract types the envelope, not a fully decoded
CPIR result; each maintained consumer must validate fields it presents.

## Authority and compatibility

Extends the transport options of ADR-0006 and Master section 19 without superseding any
accepted planning, lifecycle or authorization rule. Integration version is independent of
software 0.2.0, REST v1, CPIR 0.2, ranking features 0.1 and synthetic evaluation 0.1.
Exact 0.1 matching is required; no minor-version compatibility is inferred. The manifest
is descriptive metadata, not binary provenance, authentication or publication evidence.
Packaging must separately pin/review the source commit and built artifact.

Nexus retains product identities, permissions, provider collection, user interaction and
persistence. Its adapter constructs CPIR facts only from established evidence. Cerebri
owns generic deterministic planning. No Nexus types enter core. No production dependency
on Lab, research or evaluation collection is introduced.

## Bounded milestone and exclusions

This increment implements a local integration foundation with synthetic conformance
fixtures. It is not a rollout, packaged SDK release, authenticated service, browser/WASM
binding, execution integration, general Task scheduler, local-time interpretation API,
telemetry collector or production evaluation pipeline. Public REST deployment still needs
authentication, admission/concurrency isolation and operational design; this API remains
the existing loopback development service.

Future slices are separately gated: Nexus offline adapter/host boundary; developer-only
opt-in suggestions with honest coverage; execution with full preflight; privacy-qualified
evaluation; then any learned component. Completing this slice does not authorize the next.

## Verification and review

Core/profile rejection tests, incomplete-coverage and permission tests, unchanged-result
parity across transports, fixed synthetic starts, and an ActionPlan conversion negative
test establish implementation evidence. Host tests cover cancellation, timeout, malformed
output, incompatible envelopes and size limits using disposable test processes.
Independent Architecture/Security review remains required before claiming qualification;
existing Math qualification is not extended to this change by the implementation agent.

## Alternatives

A Flux-shaped endpoint would couple core to product semantics. Reusing the unrestricted
legacy route would leave completeness/authority expectations implicit. A second solver,
N-API packaging, schema generator or public remote service is unnecessary for this slice.
Rollback disables the consumer feature and removes the additive integration entry points;
there is no persistent state migration or automatic fallback that invents suggestions.

[Master](../specifications/master-v0.4.md) · [Integration reference](../../en/application-integration.md)
· [Nexus agent prompt](../../development/prompts/nexus-ecosystem-integration-foundation.md)
