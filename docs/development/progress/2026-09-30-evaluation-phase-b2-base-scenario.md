# Slice 2 Phase B.2: BaseScenario checkpoint

Date: 2026-09-30. Phase B remains partial; Phase C is unstarted.

## Base and authority

Verified starting `origin/main`: `510e17314af6ab76d3da5690af6f274ef46da2e2`,
the merge of PR #43 (B.1). Live main exactly matched the expected merge; its Rust/Cargo
tree matched B.1 commit `66dc9342ab05f45460e6c6e253c28d1e1af52f7e`.
Classification: **IMPLEMENTATION-MECHANICAL RECONCILIATION**; no intervening drift.

Fresh branch: `codex/slice-2-phase-b-base-scenario`, in the existing canonical managed
workspace. The original maintainer checkout and its changes remain untouched. Selected
scenario/source-graph/test files were recovered from the separately preserved paused
Phase-B snapshot, then adapted to the merged B.1 interface. No DI/DO files were recovered.
The backup remains pinned by `refs/codex-backups/slice-2-phase-b-20260927`.

Normative authority: Intelligence Architecture v1.2.6 consolidated implementation contract,
SHA-256 `007fdbab8630ecd8119215f329d7e9e36d73076de1bef885f1055c859da2edab`.
This checkpoint implements §§17–19 and 22, consumes §§16/20, and covers the required
pre-BSF subset of §50, external B1 §56.4 and BSF/graph tests from §57. It does not
redesign architecture or claim independent implementation qualification.

## Implemented boundary

`BaseScenarioProjectionV1::from_request` accepts the trusted typed PlanningRequest.
It constructs declared and required reference-only graph vertices, reuses B.1 intrinsic
recurrence and registry validation, and renders every complete BaseScenario payload through
`canonicalize_with_payload`. All external references participate before the winning
labeling is selected. Graph-only minimality is not used for BSF.

The exact 13-field BaseScenario shape uses schema `"1"`. Request duration has its dedicated
six-state grammar; ambiguous durations sort by mathematical seconds, deduplicate and require
at least two values. Scope preserves UNBOUNDED versus BOUNDED []; constraints reuse the
qualified rule grammar; preferences admit only explicit request/session/default sources.
Temporal context preserves ABSENT/PRESENT and the distinct outer recurrence grammar.
Policy and capability contain only the specified semantic snapshot.

Identity bindings cover every participating source identity in numeric alias order.
Revision bindings cover only declared PLANNING_OBJECT and TEMPORAL_SERIES entities:
prospective is `{"state":"NONE"}`; existing is `{"state":"SOME","value":"<u64>"}`.
Bindings are returned separately. Source IDs and revisions never enter BSF bytes.

Request/trace/principal identities, context capture/revision, object and temporal revisions,
policy identity/version, SearchBudget, temporal expansion limits, ExternalLock.reason,
unused object semantics and raw confidence magnitude remain excluded. No artifact identity
or admission-work measurement is added. The existing domain-separated SHA-256 helper is reused.

Duplicate objects/facts/targets/series, object/series collisions, unsupported CPIR versions,
unsupported E2 preferences or invalid canonical duration projection return no scenario.
The existing state-limit error returns no partial BSF. This limited pre-fingerprintability
boundary does not implement Episode eligibility and does not call or change planner admission.
A BSF is evaluation identity, never permission, policy, execution or proof authority.

## Evidence

B1's external literal is exactly **2003 UTF-8 bytes**, no BOM or trailing newline,
verified directly against the supplied architecture rather than generated from production.
Both exact bytes and the expected BSF pass:
`9989e8381324dea03f300aec98ba28cfb4851a941eb8cd0f2e85682adb504b0c`.
The prefix is `nexus-cerebri:base-scenario-fingerprint:v1\0`, ending in one NUL byte.
See [fixture provenance](../../../crates/cerebri-planner/tests/fixtures/evaluation/README.md).

The initial 16 scenario tests cover exact duration shapes and invalid ambiguity, all 14 constraint
variants, all optional scope dimensions, policy/capability semantics, metadata exclusions,
temporal states and revisions, closed revision-binding wire shapes, duplicates/collisions,
learned-source rejection, full external-reference renaming/permutation invariance,
target-driven symmetry breaking and real bounded-search failure without planner changes.
G1/G2/G3 and all 11 existing graph integration tests passed unchanged.

Focused command: `cargo test -p cerebri-planner --locked --test evaluation_scenario`:
16 passed, including the real canonicalization-limit case.
Initial checkpoint completion gates (all passed):

- `cargo fmt --check`.
- `cargo check -p cerebri-planner --locked`.
- `cargo build --workspace --locked`.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`.
- `cargo test --workspace --locked`: 200 tests passed; doc-test runners also succeeded.
  Existing Phase-A, canonical, hypothesis, admission-work, graph, ranking and planner tests
  passed unchanged. No OS execution block occurred.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`.
- `cargo build -p cerebri-node --locked`; `node --test bindings/node/test.mjs`:
  4 passed, including frozen ranking/nullability and legacy CPIR 0.1 parity.
- `npm run check`: 22 tests plus repository link/version/dependency/security checks passed.
- `npm run docs:check`: zero errors/warnings; `npm run docs:build`: passed.
- `git diff --check` and `git diff --cached --check`: passed.

The complete staged diff was inspected before commit; no separate Codex review was run.
All production changes are confined to evaluation projection/exports and documentation.
No planner call site is connected to the new projection. The existing graph engine differs
only in a status comment; frozen graph fixture bytes, constraints/recurrence grammars,
planner model, execution boundaries, Cargo manifests/lockfile and dependencies are unchanged.

Starting-main CI run `36695787816` has 14 successful required jobs and the known failing,
non-required `Visual verification` job. The live main ruleset was checked: its 14 required
checks exclude Visual verification. Actual PR CI is inspected after push and reported in the
final delivery; no pending/skipped check is claimed as successful here.

## Exact changed files

- `crates/cerebri-planner/src/evaluation/graph.rs`
- `crates/cerebri-planner/src/evaluation/mod.rs`
- `crates/cerebri-planner/src/evaluation/scenario.rs`
- `crates/cerebri-planner/src/evaluation/scenario_duration.rs`
- `crates/cerebri-planner/src/evaluation/source_graph.rs`
- `crates/cerebri-planner/tests/evaluation_scenario.rs`
- `crates/cerebri-planner/tests/fixtures/evaluation/README.md`
- `crates/cerebri-planner/tests/fixtures/evaluation/b1.json`
- `docs/architecture/decisions/ADR-0015-evaluation-contract-foundations.md`
- `docs/architecture/decisions/README.md`
- `docs/architecture/specifications/master-v0.4.md`
- `docs/de/planner-integration.md`
- `docs/development/progress/2026-09-30-evaluation-phase-b2-base-scenario.md`
- `docs/development/progress/README.md`
- `docs/development/roadmap/README.md`
- `docs/en/planner-integration.md`

## PR #44 correction: preference precedence and compilation identity

Continued the same open branch from `2b31dbacff42e8ee0ad889dc50d546e1a8edf13e`;
base/main remained `510e17314af6ab76d3da5690af6f274ef46da2e2`. The authority digest
was revalidated unchanged. This correction addresses only the two independently reported
B.2 blockers under frozen PreferenceSource precedence and §§17.4/50/57.

Preference entries now compare explicit architecture ranks 0/1/2/3/4, then canonical
instant and evidence-ref set. Accepted E2 order is EXPLICIT_CURRENT_REQUEST,
SESSION_CONTEXT, DEFAULT. Rank is internal only; the serialized source remains its token.
Exact duplicates collapse and both learned sources still reject.

The pre-BSF check reuses the existing pure, bounded `compile_snapshot` once and recognizes
only `CompilationError::IdentityCollision` as `COMPILATION_IDENTITY_COLLISION`.
This deliberately shares the complete native occurrence-identity authority instead of
copying the hash algorithm or changing planner compilation. Other compilation errors
are not propagated into fingerprintability failure. The existing direct declaration
collision/duplicate checks remain in place. The extra bounded compilation is evaluation
work only; it does not alter native planner behavior or emit observations.

Four correction tests were added. Before the fix, the two preference-order assertions
and reachable occurrence-collision assertion failed; the ordinary-error control passed.
After the fix all four pass. The occurrence test first compiles a valid existing series,
takes its actual generated OccurrenceId, then renames a PlanningObject to that exact ID:
native compilation rejects with IdentityCollision and BaseScenario returns no payload/BSF.
No occurrence hash is hardcoded.

The positive preference oracle asserts the exact three-record token order and all six
input permutations with duplicate entries. A second oracle checks mathematical instant
ordering (2 before 10), evidence-set tie breaking, sorted/deduplicated references and
absence of a serialized numeric rank. Six controls assert native HorizonMismatch,
InputLimit, ProspectiveSeries, InvalidProvenance, temporal OccurrenceLimitExceeded and
UnknownObjectTime while retaining a valid BaseScenario projection.

Correction verification:

- `cargo test -p cerebri-planner --locked --test evaluation_scenario correction_`:
  all 4 new tests pass after demonstrating the failures on the old implementation.
- `cargo test -p cerebri-planner --locked --test evaluation_scenario --test evaluation_graph`:
  20 scenario and 11 graph tests pass, including both real state-limit cases.
- `cargo test -p cerebri-planner --locked --test compilation --test boundedness`:
  10 native compilation and 5 boundedness tests pass.
- All local completion commands listed above were rerun successfully for the correction:
  format, planner check, workspace build, strict all-feature Clippy, workspace tests,
  Rustdoc, Node build/parity, repository check, documentation check/build and Git whitespace.
  The workspace now passes 204 tests; Node passes 4 and repository checks pass 22.
- B1 remains exactly 2003 bytes and
  `9989e8381324dea03f300aec98ba28cfb4851a941eb8cd0f2e85682adb504b0c`.
  G1/G2/G3, duration/binding tests and both canonicalization-limit tests pass unchanged.
- Native compilation, occurrence-ID derivation, graph code, fixtures, dependencies and
  versions have no diff. Actual correction-head remote CI is checked after push and
  reported in the delivery; the known non-required homepage Visual baseline is untouched.

Correction files only:

- `crates/cerebri-planner/src/evaluation/scenario.rs`
- `crates/cerebri-planner/tests/evaluation_scenario.rs`
- `docs/development/progress/2026-09-30-evaluation-phase-b2-base-scenario.md`

## Remaining work

**Next: DecisionInput/D1**, consuming BSF and the separate identity/revision bindings.
DecisionObservation/O1/O2 remains later Phase B. No candidate-generation instrumentation,
Episode collection validation, lifecycle reducer, replay R0–R4, Product Capture, telemetry,
ML or learned search was implemented. Phase C remains unstarted.

Canonical graph behavior, G1/G2/G3, canonical value/rule grammars, Slice-1 ranking,
planner/search/admission/authorization/execution, transports, dependencies and version
authorities are unchanged. The graph module edit is documentation only.
No Site screenshot baselines, installer or unrelated repository hygiene debt was changed.
No Codex review, delegated self-review, merge, release, tag or version bump was performed.
