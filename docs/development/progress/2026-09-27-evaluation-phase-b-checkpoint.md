# Slice 2: partial Phase-B canonicalization checkpoint

Date: 2026-09-27. Software 0.2.0 remains unreleased. Phase A remains complete;
**Phase B is not complete. Phases C, D and E are unstarted.**

## Source and workspace

- Original verified Phase-B base: `2a98cf85026176590f357f2349126112baaef5a6`.
- Resumed verified base: `35eb360f132be086c8d3df9ded7433c0f8ae5fd8`.
  The intervening 23 changed files concern the Site only; planner/dependencies/contracts
  are unchanged. The clean Phase-B branch was fast-forwarded, without rewriting history.
- Branch: `codex/slice-2-phase-b-canonicalization`. A managed isolated checkout preserves
  the original checkout's unrelated maintainer Site/documentation changes.
- Current supplied authority: `NEXUS_CEREBRI_INTELLIGENCE_ARCHITECTURE_v1.2.6_CONSOLIDATED (2).md`,
  SHA-256 `007fdbab8630ecd8119215f329d7e9e36d73076de1bef885f1055c859da2edab`.
  It closes the two previously missing byte grammars. Section 64 requires targeted
  independent re-qualification of §§17.3/17.5; this checkpoint does not claim that gate.
- Execution authority remains the final Slice-2 implementation prompt plus the Phase-B kickoff.
  Its multi-session rule permits the nearest coherent compiling/testable checkpoint when
  the remaining phase cannot be completed cleanly within the session context/budget.
  Weekly usage was 71% at resume and 75% at the implementation checkpoint. These are
  observed usage values, not invented stop thresholds or a claim of exhausted quota.

## Implemented boundary

- `canonical.rs`: closed CanonicalValueV1, explicit tag rank, recursive numeric/UTF-8
  ordering, strict imports, source set/multiset normalization, unique object keys,
  canonical aliases/tokens, and final JCS writer for the contract's nonnumeric JSON subset.
- `canonical_rules.rs`: all 14 exact constraint variant forms; raw canonical time ranges;
  IANA timezone scalars; DAILY/WEEKLY patterns; typed recurrence-source conversion;
  closed top-level recurrence and temporal-series forms. Source-domain interval/count
  limits and sorted unique weekday/evidence collections reject invalid imports.
- `fingerprint.rs`: explicit eight domain prefixes, exact one-placement CandidateFingerprint,
  HBytes/hypothesis digest, semantic hypothesis-set bytes/identity and ordered visit identity.
  No planner traversal, materialization or Episode capture is introduced.
- `admission_work.rs`: one pure bounded typed-request serde_json 1.0.151 calculation,
  consumed by the existing planner gate and available for later DI/replay. No separate
  copied planner boolean algorithm remains. Complete B is in 1..=262144; W=B*M;
  admission includes W=16777216 and rejects larger W. Serialization failures expose neither
  an unobserved full length nor weighted bytes. Imported observations validate fixed limits
  and reachable measurements; `validate_budget` checks the supplied M separately.

Existing Phase-A wire types, RankingFeatureSet, preferred-start resolution, ordering,
candidate generation, search/proof semantics, permissions, execution, CPIR, transports,
dependencies and all version authorities retain their behavior. The only production change
is extraction of the existing metadata admission measurement into its shared implementation.

## Test evidence

Dedicated tests cover canonical ordering, malformed scalars/variants, duplicate and unordered
collections, multiplicity overflow, Unicode/JCS escaping and ordering, strict rule/recurrence
members, source bounds, and the externally authored §§17.3.4/17.5.5 byte examples.
H1, the single-member hypothesis set, visit order and four domain-separation sanity hashes
use literal architecture expectations. Candidate identity binds only source placement.

Admission tests compare 24 typed requests against independent complete serialization over
six budgets and four metadata lengths. Existing planner boundedness tests independently
exercise exact serialization/work limits. New tests include B=1/0, 262144/262145,
W=16777216/16777217, failure states, missing/null/value, altered limits and budget mismatch.

Initial local `cargo check -p cerebri-planner --locked` was blocked executing the unchanged
serde_core build script by Windows application control (OS 4551). The first focused test run
passed three admission tests but OS 4551 blocked `evaluation_canonical` execution. No binary
renaming, policy change, target relocation or other security workaround was used. Subsequent
ordinary required gates were able to compile and execute those same paths.

Executed local gates:

- `cargo fmt --check`: passed.
- `cargo check -p cerebri-planner --locked`: passed on the subsequent ordinary gate run.
- `cargo clippy -p cerebri-planner --all-targets --all-features --locked -- -D warnings`:
  passed after fixing three collapsible-if diagnostics, without suppressions.
- `cargo test -p cerebri-planner --locked`: passed, 110 tests (17 new), plus doc tests.
  This includes the previously blocked canonical target and the unchanged boundedness,
  composition/ranking, graph-oracle, randomized, lifecycle and Phase-A wire tests.
- `npm run check`: passed, 22 version-authority tests and repository-policy checks.
- `git diff --check`: passed.

Remote CI status is recorded in the delivery handoff after pushing; no pending or blocked
remote gate is represented here as passed.

## Remaining Phase B and exact next action

Resume this branch and checkpoint. Do not restart Phase A or replace the current authority.
Implement the closed graph/edge registry and bounded payload-aware canonical labeling first,
with source-to-bytes G1/G2/G3 and rename/permutation/symmetry/state-limit tests. Then finish
full PlanningRequest/BaseScenario projection and BSF (B1), identity/revision bindings,
DecisionInput (D1), DecisionObservation projections (O1/O2), and their required negative/
invariance tests. Domain-prefix helpers alone do not implement these payloads or fingerprints.
Canonical duration projection and graph-internal recurrence projection also remain pending.

Re-run the prescribed Phase-B gates before claiming Phase B complete. Obtain the targeted
independent byte-grammar qualification separately. No architectural conflict is currently
known; that pending qualification is not supplied by implementation tests.

No Codex PR Code Review or delegated review was requested or run. No Phase C implementation,
capture, telemetry, learning, provider work, release, tag, version bump or baseline-debt cleanup
was performed. Do not merge this partial checkpoint as completed Phase B.
