# Slice 2 Phase B.1: canonical graph checkpoint

Date: 2026-09-30. Phase B remains partial; Phase C is unstarted.

## Authority and reconciliation

Verified starting `origin/main`: `4a80c7a9ae1537a72ed0211bab8d82ccda975a7a`.
Compared with merged partial checkpoint `b57266c5494f2126d6301de0cd7b9475075f802a`,
the drift changes only unrelated Site/Lab/setup/documentation/CI files. No Rust crate,
PlanningRequest, constraint/temporal/ranking/search/admission contract, Cargo dependency
or serializer pin changed: **IMPLEMENTATION-MECHANICAL RECONCILIATION**.

Branch: `codex/slice-2-phase-b-graph-canonicalization`, freshly created from that main.
The existing managed workspace was reused. Earlier uncommitted full-Phase-B work is
preserved locally in Git stash object `ba92f6673f263ca26e942ce32779429b0fd216a7`, pinned
by `refs/codex-backups/slice-2-phase-b-20260927`; it is not part of this B.1 branch.
Only graph foundations and G1/G2/G3 fixtures were recovered. The original maintainer
checkout and its unrelated `.gitignore` change were left untouched.

Normative authority: `NEXUS_CEREBRI_INTELLIGENCE_ARCHITECTURE_v1.2.6_CONSOLIDATED (2).md`,
revalidated SHA-256 `007fdbab8630ecd8119215f329d7e9e36d73076de1bef885f1055c859da2edab`.
The maintainer supplied `SLICE 2 PHASE-B BYTE GRAMMAR DATA QUALIFIED` and
`SLICE 2 PHASE-B BYTE GRAMMAR HARD-MATH QUALIFIED`: the targeted section 64 gate is
closed. This records external results, not implementation self-qualification.

## Implemented boundary

Sections 6/7 are reused unchanged. Sections 13–16 implement the closed six-vertex,
six-edge graph registry, declared/reference-only semantics, directed relations,
explicit edge/direction ranks and canonical multiplicities. Identity-based relations
create required missing reference-only endpoints and external reference vertices.
SERIES intrinsic recurrence is the section 14.6 typed OBJECT, with DAILY empty and
WEEKLY nonempty weekday SETs; it is separate from the section 17.5 outer grammar.

Stable incident-multiset refinement and exhaustive individualization compare every
complete architecture-controlled payload. Source IDs enter only the secondary binding
comparison after payload equality. An explicit DFS stack avoids recursive process-stack
growth. More than 100000 states returns `FINGERPRINT_CANONICALIZATION_LIMIT`; no partial
result escapes, and the planner is neither called nor modified by this engine.
The callback must be pure and render the complete canonical semantic payload. The
graph-only helper is limited to conformance use; it is not the normative BSF selector.

No complete PlanningRequest-to-BSF, DI or DO projection is included. Identity bindings
currently serve the labeling callback/tie-break only; source/revision binding integration
belongs to B.2. There is no runtime capture, lifecycle validation or replay runner.
The evaluation code comment now assigns replay authentication to Phase D, not Phase E.
JCS property sorting remains UTF-16, distinct from semantic UTF-8 object-key ordering.

## Evidence

G1/G2/G3 source graphs match the externally authored exact bytes and domain-separated
SHA-256 digests; expectations are literal fixtures, never implementation-generated.
See the [fixture provenance](../../../crates/cerebri-planner/tests/fixtures/evaluation/README.md).

New tests cover permutation/rename invariance, true graph symmetry and stable binding
ties, synthetic external target REF symmetry breaking, declared versus reference-only,
Fact subjects, evidence roles/direction, exact edge ranks, DAILY/WEEKLY recurrence,
unknown vocabulary/fields, invalid endpoints/payloads, duplicate identities/attributes,
invalid Fact subjects, multiplicity overflow and malformed direction/numeric wire values.
Unit tests cover the exact 100000/100001 boundary and compressed incident comparison
against independently expanded multisets. A real nine-way symmetry limit failure leaves
normal planner output unchanged. Existing canonical/hypothesis/admission/Phase-A tests pass.

Executed gates:

- `cargo test -p cerebri-planner --locked --test evaluation_graph`: passed; all
  11 graph integration tests also passed in the full workspace run.
- `cargo test -p cerebri-planner --locked --lib evaluation::graph`: 2 passed,
  including after moving the test module to satisfy Clippy.
- Focused `evaluation_canonical`, `evaluation_fingerprints`, `evaluation_admission_work`,
  `evaluation_episode_wire` and `evaluation_wire`: 47 passed.
- `cargo fmt --check`: passed.
- `cargo check -p cerebri-planner --locked`: passed.
- `cargo build --workspace --locked`: passed.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`:
  passed after correcting test-module placement; no lint suppression.
- `cargo test --workspace --locked`: all 184 tests passed, plus doc tests. No OS 4551
  execution block occurred in this continuation.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`: passed.
- `cargo build -p cerebri-node --locked` and `node --test bindings/node/test.mjs`:
  passed, 4 Node tests, including frozen ranking semantics and legacy CPIR compatibility.
- `npm run check`: 22 tests and repository checks passed.
- `npm run docs:check` and `npm run docs:build`: passed, zero Svelte errors/warnings.
  The first check lacked local `svelte-kit`; `npm --prefix apps/cerebri-site ci
  --ignore-scripts` restored the lockfile-defined tools without dependency changes.
- `git diff --check` and staged whitespace check: passed.

Remote checks are inspected after commit/push and recorded in the delivery handoff;
no queued, pending or skipped gate is claimed as passed here. Starting-main CI run
`36412431068` has 14 successful jobs and the known `Visual verification` failure.
The live main rules require those 14 checks and exclude Visual verification. No
visual baseline, installer or root test-results cleanup is part of this checkpoint.

## Exact changed files

- `crates/cerebri-planner/src/evaluation/episode.rs`
- `crates/cerebri-planner/src/evaluation/mod.rs`
- `crates/cerebri-planner/src/evaluation/graph.rs`
- `crates/cerebri-planner/src/evaluation/graph_registry.rs`
- `crates/cerebri-planner/src/evaluation/graph_recurrence.rs`
- `crates/cerebri-planner/tests/evaluation_graph.rs`
- `crates/cerebri-planner/tests/fixtures/evaluation/README.md`
- `crates/cerebri-planner/tests/fixtures/evaluation/g1.json`
- `crates/cerebri-planner/tests/fixtures/evaluation/g2.json`
- `crates/cerebri-planner/tests/fixtures/evaluation/g3.json`
- `docs/architecture/decisions/ADR-0015-evaluation-contract-foundations.md`
- `docs/architecture/decisions/README.md`
- `docs/architecture/specifications/master-v0.4.md`
- `docs/de/planner-integration.md`
- `docs/en/planner-integration.md`
- `docs/development/roadmap/README.md`
- `docs/development/progress/README.md`
- `docs/development/progress/2026-09-27-evaluation-phase-b-checkpoint.md`
- `docs/development/progress/2026-09-30-evaluation-phase-b1-graph.md`

## Remaining scope

Next is B.2: complete PlanningRequest-to-BaseScenario projection, real complete-payload
labeling integration, BaseScenarioFingerprint/B1 and identity/revision bindings. DI/D1
and DO/O1/O2 remain later Phase-B continuations. Phase C remains unstarted; Phase D
is replay and Phase E is integration/docs/qualification preparation.

RankingFeatureSet v0.1, deterministic planner/search/authorization/execution, CPIR,
transports, dependencies and version authorities are unchanged. No unrelated visual
baselines, desktop packaging, startup scripts or repository debt were modified.
No Codex review, auto-merge, release, tag or version bump was requested or performed.
