<!-- doc: application-integration; lang: en; counterpart: ../de/application-integration.md -->
# Connect an application to Cerebri

The first application integration profile finds time for **one prospective Event** in an
explicit window. Nexus can use it for a proposed focus block linked to a Task; other apps
can supply the same generic CPIR data. The application keeps product state, permissions,
provider collection and UI. Rust remains the planning authority.

This is a local development foundation for review, not a released SDK or production
rollout. [ADR-0016](../architecture/decisions/ADR-0016-application-suggestion-boundary.md)
records the proposal and pending independent qualification.

## Contract and entry points

| Interface | Describe the profile | Request suggestions |
| --- | --- | --- |
| Rust | `cerebri_core::integration::describe()` | `suggest(SuggestionRequest)` or `suggest_from_slice(bytes)` |
| Node host | `describeIntegration(options)` | `suggest(envelope, options)` from `bindings/node/index.mjs` |
| Native process | `cerebri-node-bridge --describe-integration` | `cerebri-node-bridge --suggest`, one JSON envelope on stdin |
| Development REST | `GET /v1/integration/manifest` | `POST /v1/integration/suggestions`, `application/json` |

The manifest identifies integration **0.1**, profile `single_event_suggestion`, CPIR **0.2**,
operation `FIND_SLOT`, deployment modes `Test`, `Shadow`, `Suggestion`, explicit temporal
coverage and no execution support. Its `software_version` is build metadata. Check the
integration version/profile independently; software 0.2.0, REST v1, CPIR, ranking features,
evaluation schemas and publication status are different authorities. Exact version matching
is required; the manifest is not an artifact signature or proof of a published release.
Pin and review the source revision and built binary separately.

The [complete synthetic request](../../examples/integration/suggestion.json) is the shared
conformance fixture. Its envelope has exactly `integration_version: { major: 0, minor: 1 }`
and `request`, containing CPIR. The existing `/v1/plan` and Node `plan()` retain their
general/legacy behavior; this new profile does not fall back to them on rejection.

## Input requirements

- Use CPIR 0.2 and `FIND_SLOT`; identify one Event with `revision: null`. Its time is
  `RESOLVED/MISSING` because the requested placement does not yet exist.
- Supply an explicit duration, IANA zone, UTC instants and bounded scope. No date-only
  midnight, guessed duration, invented busy interval or implicit timezone conversion.
  The policy cannot enable uncertain duration in this profile.
- Set `scope.max_mutations` and `policy.snapshot.mutation.max_mutations` to zero;
  `allowed_actions` and `planning_capability.mutations` must be empty. Use Test, Shadow
  or Suggestion mode. The host must independently establish read/plan permission.
- Supply `context.temporal` with the same horizon as the scope, bounded expansion limits,
  explicit `coverage` and a `series` array (possibly empty). `Complete` is a caller
  assertion about the supplied collection, never inferred from an empty list. `Incomplete`
  is valid input, but core returns InsufficientInformation and no free-time proposals.
- Preserve provenance and knowledge states. Existing occupied intervals require established
  evidence. Scope omission/null, empty arrays and populated filters retain their distinct
  [CPIR semantics](cpir.md); scope never grants permission.

Core still validates temporal resolution, horizon equality, evidence, permissions, scopes,
constraints, dependencies and all [planner admission limits](planner-integration.md).
The facade does not repair malformed or incompatible input. `max_request_bytes: 262144`
describes the raw JSON envelope cap, not all domain or materialized-evidence budgets.
Direct typed Rust calls still undergo domain admission; the raw byte cap applies to decoding.

## Understand the response

`status: "rejected"` means decoding or profile admission failed. `code` is one of:

```text
invalid_request                 request_too_large
unsupported_integration_version unsupported_cpir_version
unsupported_operation           mutation_authority_forbidden
unsupported_deployment_mode     prospective_event_required
temporal_coverage_required       uncertain_duration_forbidden
```

`status: "planned"` includes `request_id`, `trace_id`, `context_revision` and `result`,
the full unchanged Rust PlanningResult. This status only means the planner returned.
Render `result.outcome` (Solution, NoSolution, NeedsRelaxation, InsufficientInformation)
and `result.assessment` (ProvenOptimal, Complete, BestFound) separately. Preserve candidate
order, placements, explanation, ranking features, conflicts, compilation and graph evidence.
ProvenOptimal is only relative to the declared bounded grid. BestFound is not an optimality
claim. Do not sort candidates with a second planner in the client.

The fixture yields seven starts at 15-minute increments from 10:00 through 11:30 UTC.
Changing coverage to Incomplete yields no candidates and unknown availability. These are
synthetic test facts, not a statement about a user's calendar.

REST uses 200 for a completed planner call (including insufficient information), 422 for
profile rejection, and the extractor's 400/415/422/413 status for malformed JSON, wrong media
type, invalid shape or oversized bodies. Rejections contain the same sanitized envelope.
The process protocol returns one JSON response and exits zero for handled rejections;
process failure is distinct. Neither transport exposes execution.

## Use the Node host bridge

Build with `cargo build -p cerebri-node --locked`, then from the Cerebri checkout:

```js
import { readFile } from 'node:fs/promises';
import { describeIntegration, suggest } from './bindings/node/index.mjs';

const manifest = await describeIntegration();
const envelope = JSON.parse(await readFile('./examples/integration/suggestion.json', 'utf8'));
const controller = new AbortController();
const response = await suggest(envelope, { signal: controller.signal, timeoutMs: 10_000 });
// Decode the result fields used by your view before presenting them.
```

`binary` is optional trusted host configuration, defaulting to this checkout's debug
binary. Never accept a binary path, command, raw CPIR authority or shell arguments from
an untrusted renderer. The host maps a narrow product intent and constructs its CPIR.
No Electron/native packaging or npm publication is supplied by this slice. A consumer must
establish a reproducible pinned source/binary dependency before deployment.

The new Node calls default to a 10-second process deadline, configurable from 1 to 60,000 ms.
This is a failure-isolation bound, not a measured latency target or a solver budget.
Aborting or timing out kills the child and returns no partial plan. Output is capped at
16 MiB; stderr is discarded. `CerebriBridgeError.code` distinguishes `aborted`, `timeout`,
`bridge_unavailable`, `bridge_failed`, `invalid_response`, `incompatible_bridge`,
`request_too_large`, `response_too_large`, `invalid_request` and `invalid_options`.
Error messages contain no raw parser/child output. The old `plan()` API is unchanged.

TypeScript declarations validate/type the manifest and response envelope, with the inner
PlanningResult represented as JSON. They do **not** claim a complete decoded CPIR DTO.
The app must decode every result field it uses, reject unsafe integer values and correlate
responses to the current request/context. Do not cast JSON into executable lifecycle proofs.
Never retain a result after its source Task, scope, permissions or availability changes.

## Boundaries and next step

No Task, reminder, calendar or database changes are performed. Even a validated suggestion
cannot convert into ActionPlan: core returns `AnalysisOnly`. A future execution slice must
start with a fresh authorized request and preserve all validation, freshness, confirmation,
capability and idempotency checks. Selection of a suggestion is not execution authorization.

The REST service remains the loopback development server. Authentication, concurrency
isolation, public deployment, browser/mobile transport, generic local-time interpretation,
provider adapters, production observations and learned ranking are outside this milestone.
No personal planning data is captured or logged by the new client. Existing synthetic
evaluation types are not a production collection/replay service.

The [Nexus implementation prompt](../development/prompts/nexus-ecosystem-integration-foundation.md)
specifies the next bounded consumer slice and tests. Rollback disables the app's feature
and stops calling the additive profile; no data migration is required.

[Deutsch](../de/application-integration.md) · [CPIR](cpir.md) · [Lifecycle](safety.md)
