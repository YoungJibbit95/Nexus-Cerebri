<!-- doc: planner-integration; lang: en; counterpart: ../de/planner-integration.md -->
# How Cerebri compares possible times

Cerebri looks for a time that fits the supplied information and required rules. It then
compares the remaining options in a fixed order. With the same input, the current planner
returns the same order. This is what **deterministic planning** means here.

## Follow one appointment

The [website example](../../examples/request.json) needs a 30-minute appointment between
09:00 and 12:00 UTC. An existing appointment occupies 09:00–10:00. Possible starts are
15 minutes apart, so the planner checks 11 starts, from 09:00 through 11:30.

Four options overlap the existing appointment and are rejected. Seven remain, starting
at 10:00 or later. Each possible placement is called a **candidate**. The example has no
preferred start, and its other comparison values are equal, so the earlier start comes
first: 10:00 UTC. Nothing is booked by this result.

## A preference helps choose among valid options

A preferred start is a wish, such as 10:45. A required rule is a condition every accepted
option must satisfy, such as avoiding an occupied time. A preference cannot excuse a
rule violation.

The separate [preferred-start example](../../examples/02-ranking-preferred-start.json)
shows this comparison. The planner first compares distance from the preferred start,
then the number of changes, the amount of time moved, the start itself and the object ID.
The first difference decides the order. These are calculated values, not learned scores.

## How much of the search was checked?

The result reports both what was found and how far the search went. Read them together:

- `ProvenOptimal`: all starts in the declared grid were checked and at least one valid
  option was found. The first option is best under the current comparison rules **within
  that grid**. This says nothing about times outside the search window or between its steps.
- `Complete`: the grid was fully checked and no valid option was found.
- `BestFound`: the search does not establish either of those complete conclusions. It may
  have reached its candidate limit or been unable to start. Check the outcome, validation
  report and evaluated count; the label alone does not mean a solution exists.

Missing required information is reported as `InsufficientInformation`. Cerebri does not turn
unknown availability into free time. Changing the search window, rules or supplied
information means asking a new planning question.

## What about repeating appointments?

Existing daily or weekly series can contribute occupied times. The planner derives their
individual occurrences for the requested period before searching. A weekly rule, its
Tuesday occurrence and a new appointment to place are different things.

This does not create a new repeating series or rearrange several appointments together.
The automatic search currently places one target against the supplied context.

[Input explained](cpir.md) · [Time and availability](temporal.md) ·
[From a proposal to a permitted change](safety.md)

## Technical

CPIR `0.2` adds optional `context.temporal` with `horizon`, `coverage`, shared `limits`
and `series`. Each series carries `id`, `state`, `provenance`, `evidence`, and `rule`.
`Existing { revision }` with factual provenance can compile; `Prospective` rejects.
`compile_snapshot` derives a `CompiledContextSnapshot` view of the source ContextSnapshot.
It never inserts provider objects or grants mutation rights. Scope and horizon must agree.
CPIR `0.1` remains supported without temporal input; REST `/v1` and software `0.2.0`
(unreleased) are independent versions. The legacy snapshot is closed supplied context,
not a provider completeness assertion.

The compiler retains the full range plus its half-open horizon intersection. An occurrence
ID hashes series ID and nominal local date/time/timezone, independently of horizon,
clipping and source revision. Duplicate series and object/occurrence identity collisions
reject atomically. Callers must not supply one external occurrence both as an ordinary
object and under a different series identity. Provider reconciliation remains future work.

Earlier/Later DST fold policies intentionally keep the same nominal occurrence identity,
while UTC ranges and resolution evidence differ. Planning reuses the validated immutable
compilation view; compiler errors cannot fall back to legacy planning. Lifecycle validation
still checks and recompiles the current snapshot independently.

All supplied Event/Task objects and compiled occurrences conservatively block overlap,
even outside mutation scope. Incomplete coverage returns unknown complement and
`InsufficientInformation`; it never creates free slots. RequiredBuffer checks full ranges
and requires the buffered candidate to fit within the compiled horizon. Recurrence constraints,
prospective series, series mutation, exceptions and series-level buffers remain unsupported.

DependencyOrder means predecessor.end <= dependent.start. The graph deduplicates and sorts
edges, reports missing references, and uses iterative traversal to find cyclic strongly
connected components with member/edge evidence. Invalid graphs have no claimed order.
The single-event grid search uses fixed predecessors; batch proposals still require full
lifecycle validation. Graph ordering is not an execution order or authorization.

Ranking is lexicographic: preference distance seconds, mutation count, shifted seconds,
start instant, object ID. `ordering_key` exposes every component. `explanation` identifies
the selected preference source. Existing source precedence is preserved; no learning is
implemented. Exhaustive solution => ProvenOptimal **for this grid and objective**;
exhaustive rejection => Complete; budget exhaustion => BestFound.

## Research and limits

### Ranking Feature Contract v0.1

Each candidate exposes `ranking_features`, a versioned deterministic observation contract
for inspection and parity, not a final preference-learning feature space. It contains exactly
`schema_version: {major: 0, minor: 1}`, `preferred_start_distance_seconds`,
`preferred_start_source`, `mutation_count` and `shift_seconds`. Its version is independent
of CPIR, software, API, model, dataset and specification versions; none is bumped here.

Resolve the preferred evidence by minimum `(source_rank, preferred_start)` with
ExplicitCurrentRequest=0, SessionContext=1, PersonalLearned=2, GlobalLearned=3, Default=4.
The earlier instant wins within one source; exact duplicates are ranking-equivalent.
Input order and evidence-vector contents do not affect resolution. An empty profile yields
no evidence. Source records provenance of this same resolution, never an ordering term.

Every field is required. For **both** nullable fields: missing => rejected, explicit
`null` => None, typed value => Some(value). Distance is None iff source is None.
Unsupported schema versions, mismatched nullability and unknown fields reject.
The private Rust wire type uses `deserialize_with` without defaults on both nullable fields,
then validates version and consistency before constructing the domain type. Ordinary
Serde-derived Option fields would merge missing and null; the presence truth table is tested
with actual JSON. Serialization emits both fields even when null; roundtrips preserve None.

Distance is `(candidate_start - resolved_start).num_seconds().unsigned_abs()`, preserving
the existing whole-second projection. None differs from Some(0): Some(0) means evidence
exists with quantized zero, not necessarily instant equality. Equality and +/-0.8 seconds
yield zero; +/-1 second yield one. Current chrono arithmetic needs no clamp or saturation.

Mutation count is `m(candidate, request_context)`: analysis-only (FindSlot, Analyze or
max_mutations=0) => 0; the current mutating single-target path => 1. Shift is
`(original_start - candidate_start).num_seconds().unsigned_abs()` or zero with no original
placement. Prospective, unchanged and nonzero subsecond shifts intentionally alias to zero.

The ordering key still is `(distance.unwrap_or(0), mutation_count, shift_seconds, start,
object_id)`, reusing the observation values. This preserves the legacy None-to-zero ordering
projection without erasing domain missingness. The semantic target is identical ordering for
every supported valid input, including subseconds; bounded regression tests are evidence,
not a universal proof. Generation, validation, SearchAssessment and execution stay unchanged.

REST and Node transport core output; Lab checks and displays it without recomputing features.
Old imported Lab outputs lacking `ranking_features` reject. Existing numeric candidate fields
and the Rust `RankingFeatures` compatibility helper remain. This is an additive provisional
output change; strict consumers must update. JavaScript guards retain their safe-integer
display limit; actual core time distances fit it, while Rust accepts the full u64 wire domain.
This ranking contract adds no learning, training, telemetry, episode collection, learned
search or provider integration. The separate evaluation foundations are described below.
No raw content/identifiers are features. See [ADR-0014](../architecture/decisions/ADR-0014-ranking-feature-contract.md).

At most 32 series, 1,024 occurrences, 36,600 examined dates shared across series, 256 graph
nodes and 1,024 constraints. Combined planner work must stay within the existing 1,000,000
bound with occurrences included. Limits reject whole compilation, never truncate coverage.
The graph uses at most V bounded traversals of V nodes and E edges, O(V(V+E)); no recursion.

[Fixtures and independent expected outcomes](../../examples/planner/manifest.json) cover
feasibility, hard rejection, missing information, scope, dependency order/cycles, recurrence,
duplicate materialization, DST and incomplete coverage. Rust API and live HTTP-to-TypeScript
tests consume these inputs. Negative contract tests reject drift in newly added fields.
Schema generation becomes justified with multiple maintained clients or recurrent drift;
small explicit parity tests suffice for this internal Lab contract today.

The [2026-09-21 verification campaign](../development/progress/2026-09-21-planner-verification.md)
cross-checks seeded small schedules, every three-node directed graph, generated recurrence,
malformed JSON and real API/Lab contracts. This is bounded property testing, not formal
verification. Manual guards now check validation variants and bounded IDs; schema generation
remains deferred because the observed gaps are localized missing guards, not recurring wire
schema changes across multiple clients.

[ADR-0013](../architecture/decisions/ADR-0013-planner-resource-admission.md) adds compact
request size <= 256 KiB, request bytes times candidate budget <= 16 MiB and materialized
occurrence evidence <= 1 MiB. These complement the existing workload limits. Local-date
overflow rejects the candidate rather than panicking. No feature expansion or release is implied.

The completed Slice-2 Phase-A [evaluation foundations](../architecture/decisions/ADR-0015-evaluation-contract-foundations.md)
add the closed Episode wire contract, a distinct wire-validated domain container and strict
scalar/manifest, provenance and run-binding components in Rust. Partial Phase B adds canonical
value/rule grammars, pure candidate/hypothesis fingerprints and shared admission-work measurement.
Phase B.1 adds bounded graph canonicalization with a payload-aware labeling interface.
Phase B.2 adds complete BaseScenario/BSF projection with separate identity/revision bindings.
DecisionInput/DecisionObservation projection, semantic lifecycle/collection validation and
replay remain unfinished. Planner and transport behavior is unchanged; live
capture, telemetry and learning remain absent. The targeted independent Data/Hard-Math
qualification of the rule byte grammar is complete; overall Phase B remains partial.

[ADR-0012](../architecture/decisions/ADR-0012-planner-snapshot-compilation.md) ·
[Temporal reference](temporal.md) · [CPIR reference](cpir.md) · [Deutsch](../de/planner-integration.md)
