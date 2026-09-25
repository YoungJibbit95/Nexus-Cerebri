# ADR-0014: Versioned deterministic ranking observations

Date: 2026-09-25
Status: Accepted (qualified Slice 1 contract; implementation pending independent review)

## Context

ADR-0012 exposes a deterministic ordering key, but its numeric zero merges absent
preferred-start evidence with a quantized distance of zero. Inspection needs an explicit,
versioned observation contract without changing the objective or introducing learning.

## Decision

Add `ranking_features: RankingFeatureSet` to every ranked candidate, owned by
`cerebri-preferences` and computed by the Rust planner. Its independent feature-schema
version is `{major: 0, minor: 1}`. This extends Master sections 9, 19 and 25 and
ADR-0012's output contract; it does not change CPIR input, REST routes or search semantics.

Resolve the preferred evidence as the lexicographic minimum of `(source_rank, preferred_start)`:
ExplicitCurrentRequest=0, SessionContext=1, PersonalLearned=2, GlobalLearned=3, Default=4.
Empty preferences resolve to None; equal-source ties choose the earlier instant. Exact
source/time duplicates are ranking-equivalent; evidence-vector contents and vector order
do not affect this resolution.

Required fields are `schema_version`, `preferred_start_distance_seconds`,
`preferred_start_source`, `mutation_count`, and `shift_seconds`. For both nullable fields:
missing rejects, explicit null means None, and a correctly typed value means Some(value).
Distance is None iff source is None. A v0.1 consumer rejects incompatible versions,
unknown fields, malformed types and inconsistent nullability. Serialization emits null
explicitly and never omits the fields.

Ordinary Serde-derived Option fields collapse missing and null and are insufficient.
A private wire type requires presence using field-level `deserialize_with` without
defaults, then validates version and paired nullability before constructing the private-field
domain type. Actual JSON tests cover all nine presence combinations and the ordinary
Option regression. No deserialized invalid state acquires validated domain status.

Preferred distance is `(candidate_start - resolved_start).num_seconds().unsigned_abs()`.
Some(0) records present evidence with quantized whole-second zero, not necessarily exact
instant equality: equality and +/-0.8 seconds yield zero; +/-1 second yield one.
None remains distinct from Some(0) in the domain and wire. Source records provenance only.

Mutation count is contextual `m(candidate, request)`: analysis-only (including FindSlot,
Analyze or max_mutations=0) maps to zero; the current mutating single-target path maps to one.
Shift is `(original_start - candidate_start).num_seconds().unsigned_abs()` if original
placement exists, otherwise zero. Missing placement, unchanged placement and nonzero
subsecond shifts intentionally alias to zero. No saturation, clamping or new temporal
error is introduced for supported chrono instants.

The total ordering key remains `(distance.unwrap_or(0), mutation_count, shift_seconds,
start, object_id)`. Features and key reuse the same computed values. Source does not rank.
No learned component, telemetry, EvaluationEpisode, dataset capture, Nexus integration,
search guidance, provider call or execution authority is introduced. Only the five fields
above belong to the feature contract; no raw content or identifiers are included.

## Alternatives

Keeping only numeric zero hides missing evidence. Ordinary Option or unproven nested
Option deserialization hides missing wire fields. Nanosecond ranking or applicability-aware
shift features would change the qualified legacy contract and belong to a later decision.
This observation contract is not the final preference-learning feature space.

## Consequences

Rust, REST, Node and Lab expose the same core-produced observations. Lab guards and renders
the contract without computing features. Old imported Lab results lacking ranking_features
are incompatible with the updated inspector. The provisional response grows additively;
consumers rejecting unknown response fields must update. Existing Rust `RankingFeatures`
and `PreferenceProfile::features` retain their numeric compatibility projection.

## Migration / Compatibility

Semantic target: identical candidate ordering for every currently supported valid input,
including subsecond instants. Feasibility, generation, traversal, SearchAssessment, proof,
permissions, policy and execution remain unchanged. Golden fixtures, independent integer
oracles, generated permutations and baseline differential comparison provide bounded
evidence, not a universal proof. Manifest expectations remain independent test oracles.

Feature schema 0.1 is independent of software 0.2.0, CPIR 0.1/0.2, REST /v1,
specification 0.4, model and dataset versions. None of those authorities is bumped.
No tag or release is implied; independent implementation qualification is still required.

## References

[Master](../specifications/master-v0.4.md) · [ADR-0012](ADR-0012-planner-snapshot-compilation.md) ·
[EN reference](../../en/planner-integration.md) · [DE reference](../../de/planner-integration.md)
