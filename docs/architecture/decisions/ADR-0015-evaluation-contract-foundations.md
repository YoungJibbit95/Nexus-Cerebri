# ADR-0015: Deterministic evaluation contract foundations

Date: 2026-09-27
Status: Accepted architecture authority; Phase A implementation checkpoint only

## Authority and scope

This record implements the maintainer-supplied, independently Data/Hard-Math-qualified
Intelligence Architecture v1.2.6 consolidated contract. It introduces no architecture
revision and does not qualify the implementation independently. The source is
`NEXUS_CEREBRI_INTELLIGENCE_ARCHITECTURE_v1.2.6_CONSOLIDATED.md`, SHA-256
`66918dc0ae8b246bd4fe9aad8a70586aa893ef183815f39efd85c47f42c6abdb`.

Only the fully specified Phase-A components are implemented. This extends Master
section 27 with observation-contract foundations, without changing planning authority.
The complete Episode wire/domain boundary remains blocked on the exact wire forms for
ImmutableArtifactRefV0_1, non-null OutcomeObservationV0_1, native CanonicalU64 and the
Episode schema-version field. No substitute shape or validated Episode is supplied.

## Recorded contract

- Place the module in `cerebri-planner::evaluation`; introduce no workspace crate or
  dependency-layer change. No current planner/REST/Node/Lab path calls these components.
- Required-nullable fields reject absence, accept explicit null where permitted and
  validate typed values. Closed component structures reject unknown fields.
- IdentifierString and RuleId use 1–128 ASCII bytes from `A-Z a-z 0-9 . _ -`;
  SHA-256 is exactly 64 lowercase hex digits. No normalization is performed.
- Canonical i64/u64 values are unique decimal JSON strings, checked across the full
  integer range. Positive seconds exclude zero; nanoseconds are exactly nine digits.
  Canonical instants use seconds/nanoseconds pairs; typed ranges have ordered end/start
  entries with start strictly before end. Dates and local times use exact source forms.
- Canonical optionality uses closed NONE/SOME tagged objects, separate from wire null.
  Token spellings are explicit architecture constants, independent of Rust enum naming.
- Manifest schema 0.1 is a separate authority. Paths are normalized bundle-relative
  UTF-8 paths, unique and sorted; roles are nonempty, unique and token-sorted. One file
  with multiple roles has one entry. Structural identity/manifest validation does not
  authenticate bytes, compute a manifest digest or resolve an artifact.
- SoftwareProvenance contains build/version metadata only. Privacy components reject
  content_fields_present=true; the E2 DataScope registry accepts SYNTHETIC only.
- PlannerRunBinding enforces NEW_RUN/null versus REUSED_RUN/non-null. Planner-run
  timestamps require context capture not after generation. Lifecycle/lineage wire
  components remain explicitly raw; decoding does not grant Episode/collection validity.
- Pin workspace serde_json to `=1.0.151`, preserving Cargo.lock, as required by the
  qualified admission-work metric. Existing admission code remains unchanged.

## Compatibility and remaining work

Existing CPIR, RankingFeatureSet, preference resolution, candidate ordering, search,
admission, permissions, lifecycle/execution and transport behavior remain unchanged.
Software 0.2.0 remains unreleased; CPIR 0.2/legacy 0.1, REST /v1 and Master 0.4 retain
their authorities. There is no release, tag, Product Capture, telemetry, ML or provider work.

This checkpoint has no complete EvaluationEpisode, JCS writer, canonical graph,
fingerprint computation, interaction reducer, collection validation or replay runner.
The remaining Phase-A contracts must be completed before advancing to Phase B.
Later phases must preserve the existing trusted planning boundary rather than treating
structural component decoding as source verification or planning truth.

## Evidence

See the [checkpoint record](../../development/progress/2026-09-27-evaluation-phase-a-checkpoint.md)
for executed tests, local execution-policy limitations and unresolved wire definitions.
The new contract tests cover malformed JSON, missing/null/value, numeric boundaries,
variant shapes, paths/roles, independent schema tokens and structural roundtrips.

[Master](../specifications/master-v0.4.md) · [ADR-0013](ADR-0013-planner-resource-admission.md)
· [ADR-0014](ADR-0014-ranking-feature-contract.md)
