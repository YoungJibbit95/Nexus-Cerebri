# Slice 2 Phase B.3 — DecisionInput / D1

## Authority and reconciliation

Normative source: maintainer-supplied Intelligence Architecture v1.2.6 Consolidated
Implementation Contract, SHA-256
`007fdbab8630ecd8119215f329d7e9e36d73076de1bef885f1055c859da2edab` (revalidated).
Implements §§20, 22–24, 28, 50, 55, 56.5 and the DecisionInput parts of §57.
Preference representation reuses qualified §17.4 behavior.

`IMPLEMENTATION-MECHANICAL RECONCILIATION`: fetched `origin/main` was exactly
`9dfe7531d38df5d37bd5360ec318531294e9bf35`, the merge of PR #44. No open PR existed
at reconciliation. Created `codex/slice-2-phase-b-decision-input` from that main in the
existing Intelligence worktree. Unrelated staged Lab UI work and `.gitignore` changes
in the maintainer checkout were preserved. No stash recovery or replacement checkout.

Phase A and B.1 are complete. B.2 is complete and independently qualified according
to the maintainer's B.3 handoff. B.3 implements DecisionInput/D1 only. This report is
implementation evidence, not independent B.3 Data/Hard-Math qualification.

## Construction and wire boundary

`DecisionInputProjectionV1::from_request(request, artifacts)` is the production entry.
It constructs a fresh `BaseScenarioProjectionV1`, uses its fingerprint and exact selected
identity/revision bindings, and projects context source revision, all four SearchBudget
fields, optional temporal expansion limits and policy identity from the same request.
No stored BSF, unrelated caller-provided bindings or work observation can enter this API.

The payload has exactly the 18 §23 fields: schema_version, base_scenario_fingerprint,
identity_bindings, revision_bindings, context_source_revision, planner_artifact,
generator_artifact, generator_configuration_artifact, search_budget,
temporal_expansion_limits, policy_identity, active_preferences, product_eligibility_policy,
ranking_policy, display_policy, model_artifact, experiment_assignment,
planner_admission_work. Schema is `"1"`. Semantic integers are canonical decimal strings.
Canonical NONE/SOME objects preserve absence; model and experiment must be NONE in E2.

The small B.2 integration exposes typed `CanonicalPreferenceEvidenceV1` and its existing
rank/instant/evidence-set comparator. Both BSF and DI use the same source projection.
Trusted source projection normalizes entries; DI import rejects unsorted/duplicate records
or evidence, unknown/wrong-type references and learned sources. No rank enters bytes.
Identity bindings retain complete numeric alias order, including reference-only identities;
revisions remain limited to matching PlanningObject/TemporalSeries identities.

`DecisionInputArtifactsV1` accepts existing `ArtifactSemanticIdentityV0_1` values for the
six semantic roles, including optional generator configuration. It has no default manifests,
resolver paths, Git locators, privacy or retention fields. The caller is responsible for
verified identities. This phase provides no resolver or artifact authentication.

`DecisionInputWireV1` is an editable lower-level conformance input; conversion to the private
payload validates canonical collections, binding references and work/budget parity. Validated
payload import additionally rejects duplicate object members at every depth and noncanonical
token-object/null/numeric forms before typed component decoding. This import-only syntax tree
does not sort semantic input or change existing Phase-A component decoding. Structural import
does not prove that a supplied BSF/binding/work triple matches an authenticated source request.

## Admission work and semantic boundaries

Production calls only `PlannerAdmissionWorkObservationV0_1::measure(request)` and
`validate_budget(max_candidates)`. The pinned compact serde_json 1.0.151 metric, 262144-byte
serialization cap and 16777216 weighted cap are unchanged. Measurement is not JCS.
DI hashing uses the existing JCS writer and `FingerprintDomainV1::DecisionInput`.

Changed context/object/series revision, each budget field, either expansion limit or either
policy identity field leaves BSF unchanged but changes DI. Semantic artifact identity changes
also change DI. Preferences reuse B.2's exact canonical order; permutations preserve DI when
their serialized sizes match. Duplicate semantic preferences collapse, but extra source bytes
can still change the fresh work observation and DI.

The metadata test holds BSF fixed while increasing excluded ExternalLock prose from a
65536-byte request to 65537 bytes with M=256: W moves from exactly 16777216 (admitted)
to 16777472 (not admitted). Native validation agrees. Equal-length alternate prose preserves
DI; a request exceeding the serialization cap remains representable with failure state and
NONE weighted bytes. No admission failure is changed into a new pre-BSF exclusion.

## External vectors and verification

`d1.json` was copied from the exact §56.5 literal, independently of production serialization:
**2389 UTF-8 bytes**, no BOM/trailing newline. A separately constructed typed semantic input
produces those exact bytes and digest
`5c121f1a9a588ea01567042aea696691b17acabf814ee4eb3e7d115971ba48aa` under
`nexus-cerebri:decision-input-fingerprint:v1\0`. Its MEASURED(1024), W=262144 observation
and repeated-digit artifact hashes are external conformance values only.

B1 remains 2003 bytes, digest
`9989e8381324dea03f300aec98ba28cfb4851a941eb8cd0f2e85682adb504b0c`.
G1/G2/G3 and B1 fixtures are unchanged. Existing six-state duration, binding, collision and
100000-state tests pass, as do H1, hypothesis-set/visit identities, CandidateFingerprint,
admission-work sanity, Slice-1 ranking and Phase-A wire/canonical grammar tests.

Focused verification passed:

- `cargo test -p cerebri-planner --locked --test evaluation_decision_input`: 14 tests.
- `cargo test -p cerebri-planner --locked --test evaluation_decision_input --test evaluation_scenario --test evaluation_admission_work --test evaluation_graph`:
  13 DI, 20 scenario, 3 admission-work and 11 graph tests before the final import-guard test.

The final DI import guard was retested separately and in the full workspace.
Negative tests cover all required top-level
members, missing/null/unknown fields, positional objects, duplicate members, noncanonical
scalars/SHA/aliases/bindings/preferences, wrong revision types/references, model/experiment SOME,
learned sources and work/budget mismatch. Invariance tests cover every requested DI source
authority, all six preference permutations and all five semantic artifact fields in six roles.

All local completion gates passed:

```text
cargo fmt --check
cargo check -p cerebri-planner --locked
cargo build --workspace --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo build -p cerebri-node --locked
node --test bindings/node/test.mjs
npm run check
npm run docs:check
npm run docs:build
git diff --check
```

Workspace tests: **218 passed**. Node parity: **4 passed**, including legacy CPIR and frozen
ranking semantics. Repository checks: **22 passed**, plus link/version/dependency/credential
checks. Documentation check: zero errors/warnings. The complete diff was inspected before
commit, with staged whitespace verification. Actual pushed-head remote CI is inspected and
reported in the delivery; no pending check is treated as passing. The main ruleset was read:
14 required contexts, excluding the known unrelated homepage Visual verification baseline.

## Exact changed files

- `crates/cerebri-planner/src/evaluation/decision_input.rs`
- `crates/cerebri-planner/src/evaluation/mod.rs`
- `crates/cerebri-planner/src/evaluation/scenario.rs`
- `crates/cerebri-planner/tests/evaluation_decision_input.rs`
- `crates/cerebri-planner/tests/fixtures/evaluation/d1.json`
- `crates/cerebri-planner/tests/fixtures/evaluation/README.md`
- `docs/architecture/decisions/ADR-0015-evaluation-contract-foundations.md`
- `docs/architecture/decisions/README.md`
- `docs/architecture/specifications/master-v0.4.md`
- `docs/de/planner-integration.md`
- `docs/en/planner-integration.md`
- `docs/development/progress/2026-09-30-evaluation-phase-b3-decision-input.md`
- `docs/development/progress/README.md`
- `docs/development/roadmap/README.md`

## Remaining scope

Overall Phase B remains partial. DecisionObservation/O1/O2 is next. CandidateGeneration
instrumentation, SearchVisit capture, product decision runtime, Episode lifecycle/collection
validation, interaction capture, replay R0–R4, Product Capture, telemetry and learned
ranking/search are not implemented here. Phase C remains unstarted.

Planner search/admission/ranking/feasibility/outcomes/authorization/execution, dependencies
and version authorities are unchanged. No Lab/Site UI work, visual baseline changes, review
request, Codex review, delegated review, merge, release, tag or version bump is included.
