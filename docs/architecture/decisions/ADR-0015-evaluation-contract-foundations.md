# ADR-0015: Deterministic evaluation contract foundations

Date: 2026-09-27
Status: Accepted architecture authority; Phase A complete, Phase B partially implemented

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

The subsequent Phase-B source is
`NEXUS_CEREBRI_INTELLIGENCE_ARCHITECTURE_v1.2.6_CONSOLIDATED (2).md`, SHA-256
`007fdbab8630ecd8119215f329d7e9e36d73076de1bef885f1055c859da2edab`.
Sections 17.3/17.5 close previously unspecified constraint and recurrence variant bytes.
Section 64 explicitly requires targeted independent canonicalization re-qualification of
those new bytes. The maintainer has supplied both independent results:
`SLICE 2 PHASE-B BYTE GRAMMAR DATA QUALIFIED` and
`SLICE 2 PHASE-B BYTE GRAMMAR HARD-MATH QUALIFIED`; the targeted section 64 gate is closed.
This records external qualification, not implementation self-certification.
Phase-A contracts and existing architecture vector bytes are unchanged.

## Recorded contract

- Place the module in `cerebri-planner::evaluation`; introduce no workspace crate or
  dependency-layer change. Only the existing planner metadata admission gate now calls
  the shared work measurement; REST/Node/Lab have no evaluation capture integration.
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
  qualified admission-work metric. Phase B extracts the existing admission calculation
  into a shared pure observation without changing thresholds or the serialization metric.

## Partial Phase-B checkpoint

- CanonicalValueV1 has explicit tag ranks, numeric/recursive payload comparison, source
  collection normalization and strict canonical collection import validation. JSON strings
  retain their exact Unicode content; semantic object entries sort by UTF-8 key bytes.
- The final JCS writer handles the contract's string/boolean/array/object subset; numbers
  and null reject. Semantic integers use decimal strings. JCS member order uses UTF-16
  code units; fixed canonical member names are ASCII, so their UTF-8 order is identical.
  It does not normalize graphs, semantic sets or identities on behalf of callers.
- All 14 constraint rules use explicit `kind` tokens and direct payload fields. Temporal
  series/rules use the separate top-level recurrence grammar with `frequency`, bounded
  decimal `every`/`count`, and ordered unique weekdays. This is not graph SERIES projection.
- Candidate fingerprints accept exactly one positive placement and exclude run-local IDs
  and policy/lifecycle outcomes by construction. Pure hypothesis/set/visit helpers implement
  HBytes, duplicate rejection and the three separate digest domains, without running search.
- PlannerAdmissionWork uses the same pinned compact typed-request serialization as the
  planner gate. B is measured only on complete serialization, 1..=262144; W=B*M must be
  <=16777216 for admission. Failure observations contain no invented byte count or W.
  Imported observations still require budget validation and authenticated source remeasurement
  at the later DecisionInput/replay boundary.

## Phase B.1 graph checkpoint

The closed graph registry and SERIES intrinsic recurrence projection now exist separately
from the top-level temporal grammar. Identity-based relations create missing reference-only
vertices. Explicit edge/direction ranks, stable partition refinement and exhaustive
individualization use the existing canonical values and final JCS writer. An explicit DFS
stack enforces the 100000-state bound without recursive process-stack growth; failure
returns no partial labeling and never changes planner admission or feasibility.

The caller-supplied complete-payload renderer determines the minimum labeling. A synthetic
external target REF tests symmetry breaking; source-ID binding bytes break ties only after
equal complete payloads. The graph-only helper exists for G1/G2/G3 conformance, not as the
normative BSF criterion. B.2 supplies the complete PlanningRequest/BSF projection below.
RFC 8785/JCS property ordering is UTF-16; semantic CanonicalValue object-key ordering
remains UTF-8. The existing writer already implements that distinction correctly.

## Phase B.2 BaseScenario checkpoint

The typed PlanningRequest projection now renders the complete BaseScenarioPayloadV1 for
every candidate graph labeling through the B.1 interface. It uses the existing closed
constraint/top-level recurrence grammars, graph recurrence projection and fingerprint domain.
Request duration uses the dedicated six-state grammar; ambiguous durations sort numerically,
deduplicate and require at least two distinct values. All external identity references use
the selected canonical aliases; optional scope filters preserve UNBOUNDED versus BOUNDED [].

Identity bindings cover every participating graph identity. Separate revision bindings cover
only declared PlanningObjects and TemporalSeries, with NONE/SOME decimal revisions. Neither
binding enters BSF. Minimal source-projection checks reject duplicate declarations/targets,
object/series collisions, unsupported schemas and learned preference sources before returning
a valid BSF; the existing graph-state bound remains a fingerprint-only failure. This is not
planner admission, Episode eligibility, permission or execution authorization.

B1 matches its externally authored 2003 bytes and digest. Source-order/opaque-ID and excluded
metadata invariance, semantic changes, full-payload target symmetry and failure isolation are
covered by the [B.2 checkpoint](../../development/progress/2026-09-30-evaluation-phase-b2-base-scenario.md).

## Phase B.3 DecisionInput checkpoint

The [B.3 checkpoint](../../development/progress/2026-09-30-evaluation-phase-b3-decision-input.md)
adds the complete DecisionInputPayloadV1/D1. Production projects a fresh BSF and its
selected bindings, context revision, budget, temporal limits, policy identity and the
shared B.2 preference profile from one typed request. Admission work is freshly measured
with the existing helper and validated against that request's candidate budget.
Artifact semantic identities are explicit trusted inputs; resolver authentication remains
later work. E2 model/experiment are NONE. Imported payloads reject noncanonical collection
order and inconsistent work/budget values. D1 is an external byte/digest oracle whose
prevalidated work observation is accepted only through the lower-level conformance boundary.
This is implementation evidence, not independent B.3 Data/Hard-Math qualification.

## Compatibility and remaining work

Existing CPIR, RankingFeatureSet, preference resolution, candidate ordering, search,
admission, permissions, lifecycle/execution and transport behavior remain unchanged.
Software 0.2.0 remains unreleased; CPIR 0.2/legacy 0.1, REST /v1 and Master 0.4 retain
their authorities. There is no release, tag, Product Capture, telemetry, ML or provider work.

The complete Episode wire shape and Phase-A domain conversion now exist. Wire validity
is not final record-local semantic/R0 validity, proof of authenticated hashes or a behavioral
evaluation eligibility claim. Complete PlanningRequest/BSF projection and identity/revision
bindings are implemented in B.2; DecisionInput/D1 is implemented in B.3.
DecisionObservation/O1/O2 is the next Phase-B checkpoint.
There is no production observation/capture, interaction reducer, collection validation or
replay runner. Phase B is partial; Phases C–E remain unstarted.
Later phases must preserve the existing trusted planning boundary rather than treating
structural component decoding as source verification or planning truth.

## Evidence

See the [completion record](../../development/progress/2026-09-27-evaluation-phase-a-completion.md)
for executed tests, base reconciliation and the exact boundary of this checkpoint.
The new contract tests cover malformed JSON, missing/null/value, numeric boundaries,
variant shapes, paths/roles, independent schema tokens and structural roundtrips.

The [Phase-B checkpoint](../../development/progress/2026-09-27-evaluation-phase-b-checkpoint.md)
records canonical grammar, byte-oracle and admission-work tests and the exact remaining scope.
The [B.1 graph checkpoint](../../development/progress/2026-09-30-evaluation-phase-b1-graph.md)
records the bounded continuation and its executed gates. Phase D is replay; Phase E is
integration, documentation and qualification preparation.

[Master](../specifications/master-v0.4.md) · [ADR-0013](ADR-0013-planner-resource-admission.md)
· [ADR-0014](ADR-0014-ranking-feature-contract.md)
