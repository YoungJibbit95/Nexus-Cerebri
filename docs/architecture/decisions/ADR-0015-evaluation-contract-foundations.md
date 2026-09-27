# ADR-0015: Deterministic evaluation contract foundations

Date: 2026-09-27
Status: Accepted architecture authority; Phase A contract foundations implemented

## Authority and scope

This record implements the maintainer-supplied, independently Data/Hard-Math-qualified
Intelligence Architecture v1.2.6 consolidated contract. It introduces no architecture
revision and does not qualify the implementation independently. The source is
`NEXUS_CEREBRI_INTELLIGENCE_ARCHITECTURE_v1.2.6_CONSOLIDATED.md`, SHA-256
`f6e10d0a20dcada223d02328b50bdd0a4dee4e5c10442ed76d1500ba7e6baadf`.
This corrected consolidated source supersedes the earlier source digest recorded in the
[partial checkpoint](../../development/progress/2026-09-27-evaluation-phase-a-checkpoint.md).

Phase A implements foundational types and wire validation only. This extends Master
section 27 without changing planning authority. The corrected source closes the four
previous wire gaps; no missing definition was reconstructed or new architecture introduced.

## Recorded contract

- Place the module in `cerebri-planner::evaluation`; introduce no workspace crate or
  dependency-layer change. No current planner/REST/Node/Lab path calls these components.
- Required-nullable fields reject absence, accept explicit null where permitted and
  validate typed values. Closed component structures reject unknown fields.
- EvaluationEpisode has a distinct editable wire type and immutable, fallibly constructed
  wire-validated domain container. Closed Episode objects reject positional JSON arrays;
  decoding retains duplicate-field rejection without an untyped JSON intermediate.
- Episode schema is exactly the JSON object `{"major":0,"minor":1}` with u16 members.
  E2 outcome is required and null-only, represented by `Option` of an uninhabited type.
  Native CanonicalU64 is exactly CanonicalU64Decimal, while the existing RankingFeatureSet
  and native ordering-key numeric fields retain their qualified representations.
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
- ArtifactBinding has the closed GIT_COMMIT, CONTENT_ADDRESSED and SYNTHETIC_FIXTURE
  reference variants. Git commits are full immutable object IDs; paths remain relative.
  Synthetic fixture and semantic-identity manifest digests must match. A content-addressed
  reference requires a separate nonempty pin and excludes bare URLs/local paths; its
  resolver-specific content-address semantics and byte authentication remain replay work.
- ReplayProvenance requires explicit nullable members, a non-null privacy-bound synthetic
  source snapshot, no model and null-only experiment assignment. SoftwareProvenance remains
  separate; no replay resolver metadata becomes deterministic semantic identity.
- SoftwareProvenance contains build/version metadata only. Privacy components reject
  content_fields_present=true; the E2 DataScope registry accepts SYNTHETIC only.
- PlannerRunBinding enforces NEW_RUN/null versus REUSED_RUN/non-null. Planner-run
  timestamps require context capture not after generation. The Episode boundary checks
  root/child shape, local reused-parent identity and foundational time ordering without
  requiring a reused planner context to postdate the child start. Lineage cause/reason
  derivation, interaction reduction and parent-collection validation remain Phase C.
- Pin workspace serde_json to `=1.0.151`, preserving Cargo.lock, as required by the
  qualified admission-work metric. Existing admission code remains unchanged.

## Compatibility and remaining work

Existing CPIR, RankingFeatureSet, preference resolution, candidate ordering, search,
admission, permissions, lifecycle/execution and transport behavior remain unchanged.
Software 0.2.0 remains unreleased; CPIR 0.2/legacy 0.1, REST /v1 and Master 0.4 retain
their authorities. There is no release, tag, Product Capture, telemetry, ML or provider work.

The complete Episode wire shape and Phase-A domain conversion now exist. Wire validity
is not final record-local semantic/R0 validity, proof of authenticated hashes or a behavioral
evaluation eligibility claim. There is no JCS writer, canonical graph, fingerprint computation,
production observation/capture, interaction reducer, collection validation or replay runner.
Phases B–E remain unstarted; Phase A completion does not authorize starting them in this session.
Later phases must preserve the existing trusted planning boundary rather than treating
structural component decoding as source verification or planning truth.

## Evidence

See the [completion record](../../development/progress/2026-09-27-evaluation-phase-a-completion.md)
for executed tests, base reconciliation and the exact boundary of this checkpoint.
The new contract tests cover malformed JSON, missing/null/value, numeric boundaries,
variant shapes, paths/roles, independent schema tokens and structural roundtrips.

[Master](../specifications/master-v0.4.md) · [ADR-0013](ADR-0013-planner-resource-admission.md)
· [ADR-0014](ADR-0014-ranking-feature-contract.md)
