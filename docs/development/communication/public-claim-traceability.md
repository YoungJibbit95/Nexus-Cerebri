# Public Claim Traceability and Truth Inventory

**Status:** Internal communication foundation  
**Snapshot:** `main@a2bd97ef25a414f8504b6738a51e5679ad638293`  
**Audit date:** 2026-09-26  
**Purpose:** Map important public-facing claims to current repository authority before README/site rewrites.

This inventory is not a substitute for the source code or normative contracts. It is a communication control surface: public explanations should be reviewed whenever an authority listed here changes.

## Status vocabulary

- **BUILT** — current implementation or current observable repository behavior supports the claim.
- **EXPERIMENTAL** — implemented as a lab/research surface, but not a stable product capability.
- **FUTURE / RESEARCH** — roadmap, architecture direction or reserved contract; not current behavior.
- **UNCLEAR / DRIFT** — evidence conflicts, a status line is stale, or public wording is broader than what current implementation establishes.

## Authority order used by this audit

For a factual capability claim, prefer:

1. accepted specification/contract;
2. current implementation and public types;
3. tests/fixtures demonstrating behavior;
4. accepted ADRs and maintained technical references;
5. release/status/version authorities;
6. existing README/site copy.

Where an older accepted document is superseded by a later synchronized architecture decision, use the current Master plus later accepted ADRs and implementation.

---

## 1. Project identity and current execution model

### Claim

Cerebri currently provides a deterministic temporal planning foundation. Neural inference is not part of the current planner runtime.

**Status:** BUILT for deterministic planning; FUTURE / RESEARCH for neural inference.

**Primary evidence**

- `crates/cerebri-planner/src/search.rs` — deterministic `BaselinePlanner` grid traversal, validation and lexicographic sorting.
- `crates/cerebri-preferences/src/lib.rs` — deterministic preference observations and ordering inputs; crate explicitly states there is no learning or implicit write behavior.
- `docs/en/planner-integration.md` / `docs/de/planner-integration.md` — current ranking/search semantics; explicitly state that learning is not implemented.
- `README.md` — explicitly says the current implementation is deterministic and neural inference is a future milestone.

**Communication consequence**

“Neural intuition + symbolic verification” may remain as a project principle or research identity only when nearby copy clearly states that the present planner is deterministic. It must not be used as shorthand for a currently running neural planner.

---

## 2. Natural-language interpretation

### Claim under audit

Cerebri currently interprets or understands free-form human language before planning.

**Status:** FUTURE / RESEARCH as a runtime capability; educational wording exists today.

**Evidence**

- `apps/cerebri-site/src/lib/components/IntentPlan.svelte` explicitly states that the walkthrough does **not** parse free-form language and that the human sentence is an explanatory label.
- `crates/cerebri-planner/src/model.rs` accepts a structured `PlanningRequest`; it does not expose a free-form natural-language parser.
- Master v0.4 describes an end-to-end architecture containing interpretation/parsing, but that architecture is broader than the implemented deterministic planner boundary.

**Current public-risk locations**

- `apps/cerebri-site/src/routes/+page.svelte` says Cerebri “separates flexible interpretation from binding action.”
- `apps/cerebri-site/src/lib/components/CerebriExplainer.svelte` says Cerebri “can explore meaning and ambiguity” and uses “Interpret broadly” as a current principle.
- The explainer visual contains an “OPEN INTERPRETATION SPACE.”

These phrases are broader than the current runtime evidence. Later public-copy work should narrow them or explicitly frame them as architecture/research context.

**Do not change runtime code to make this copy true.**

---

## 3. CPIR input boundary

### Claim

Current planner inputs are structured through CPIR. CPIR 0.2 is current and legacy CPIR 0.1 remains supported in its documented compatibility boundary.

**Status:** BUILT.

**Evidence**

- `examples/request.json` — current CPIR 0.2 example.
- `crates/cerebri-planner/src/model.rs` — current structured planning request model.
- `docs/en/planner-integration.md` / `docs/de/planner-integration.md` — CPIR 0.2 temporal source state and CPIR 0.1 legacy compatibility.
- Master v0.4 and ADR-0012 — current temporal compilation boundary.

**Communication consequence**

An educational natural-language request may precede CPIR, but public text must make clear that the demonstrated planner consumes structured state rather than claiming that the current product parses that sentence.

---

## 4. Planning scope

### Claim

Planning scope limits what the planner considers. Scope does not grant mutation capability or execution authorization.

**Status:** BUILT.

**Evidence**

- `crates/cerebri-planner/src/model.rs`, `PlanningScope::selects` and `allows_placement`.
- Source comment: “Scope only filters; it never grants capability or authorization.”
- Master v0.4 separates planning capability, execution authorization, policy and constraints.

**Communication consequence**

Do not describe scope as permission. Diagrams must not visually merge “inside scope” with “authorized to mutate.”

---

## 5. Current baseline search

### Claim

The baseline planner searches one target event over a discrete time grid within a bounded horizon.

**Status:** BUILT.

**Evidence**

- `crates/cerebri-planner/src/search.rs`.
- The planner rejects unsupported target counts for baseline search unless input is already insufficient.
- Candidate start positions advance by `granularity`.
- A candidate is admitted only if its duration fits inside the scope horizon.
- `docs/en/foundation.md` and `docs/de/foundation.md` describe the same single-event grid.

**Important limit**

Lifecycle validation can validate explicit multi-placement states, but that is not the same as batch search or joint repair.

**Communication consequence**

Use “single-event grid search” for the present baseline. Do not imply general multi-object optimization, broad repair search or continuous-time optimization.

---

## 6. Candidate validity and hard constraints

### Claim

Candidates are checked against deterministic scope/fact/constraint logic before ranking.

**Status:** BUILT.

**Evidence**

- `crates/cerebri-planner/src/search.rs` calls compiled placement-violation checks before producing a ranked candidate.
- `crates/cerebri-constraints/src/lib.rs` implements current hard-constraint vocabulary.
- Constraints include NoOverlap, ExplicitTime, ExplicitDate, EarliestStart, LatestEnd, Deadline, MinDuration, FixedDuration, AvailabilityWindow, DependencyOrder, RequiredBuffer, TimezoneIntegrity and ExternalLock.
- RecurrenceRule is intentionally unsupported/fail-closed at this boundary.

**Communication consequence**

A rejected candidate should be shown as invalid because of a specific rule/evidence path, not because a learned model “disliked” it.

---

## 7. Half-open interval behavior

### Claim

Temporal intervals are half-open `[start, end)`; touching boundaries do not overlap.

**Status:** BUILT.

**Evidence**

- Master v0.4 temporal section.
- `crates/cerebri-constraints/src/lib.rs` overlap checks use current `TimeRange::overlaps` behavior and tests include a touching case.
- Planner verification and integration documentation preserve this boundary.

**Communication consequence**

Examples may intentionally place a candidate exactly when a blocking interval ends. This is valid and useful for teaching exact interval semantics.

---

## 8. Preferences

### Claim

A preferred start can influence deterministic candidate ordering. Preference evidence has an explicit source precedence.

**Status:** BUILT.

**Evidence**

- `crates/cerebri-preferences/src/lib.rs`.
- Source precedence:
  1. `ExplicitCurrentRequest`
  2. `SessionContext`
  3. `PersonalLearned`
  4. `GlobalLearned`
  5. `Default`
- Equal-source ties choose the earlier preferred instant.
- Empty preference profile has no preferred-start evidence.

**Important wording rule**

The enum contains `PersonalLearned` and `GlobalLearned` source labels, but this does not establish that current Cerebri learns those preferences. They are provenance vocabulary and storage/contract boundaries.

**Communication consequence**

Public material may explain explicit preferred-start ordering now. Do not infer an implemented learning pipeline from the source names.

---

## 9. Ranking Feature Contract v0.1

### Claim

Each ranked candidate exposes deterministic ranking observations through `RankingFeatureSet` v0.1. This is not an ML training feature space.

**Status:** BUILT and independently Math/Security-qualified according to current roadmap/governance status.

**Evidence**

- `crates/cerebri-preferences/src/lib.rs` — validated wire/domain type.
- `crates/cerebri-preferences/tests/ranking.rs` — presence/nullability, source precedence, quantization and boundary tests.
- `crates/cerebri-planner/src/search.rs` — core-produced candidate feature set and shared ordering values.
- `docs/development/progress/2026-09-25-ranking-feature-contract.md` — implementation evidence.
- `docs/development/progress/2026-09-26-ranking-import-parity-correction.md` — Lab import parity correction.
- `docs/development/roadmap/README.md` — current 2026-09-26 status states Slice 1 is implemented and has completed independent Math and Security qualification.

### Document drift

`docs/architecture/decisions/ADR-0014-ranking-feature-contract.md` still has the status line:

> Accepted (qualified Slice 1 contract; implementation pending independent review)

That line describes an earlier point in time and is stale relative to the current roadmap/governance status and implemented code.

**Status:** UNCLEAR / DRIFT in the ADR header only; implementation status itself is established by newer authority/evidence.

**Later action**

A documentation-consistency slice should update the stale ADR status without changing the contract semantics.

---

## 10. Exact deterministic candidate ordering

### Claim

The current total candidate ordering is lexicographic:

`(preference_distance_seconds, mutation_count, shift_seconds, start, object_id)`

with absent preferred-start evidence projected to numeric zero **only for the existing ordering key**.

**Status:** BUILT.

**Evidence**

- `crates/cerebri-planner/src/search.rs` — `CandidateOrderingKey`.
- `crates/cerebri-preferences/src/lib.rs` — `RankingFeatureSet`.
- ADR-0014 and DE/EN planner-integration reference.
- Ranking tests cover missing vs. present zero and subsecond quantization.

**Material distinction**

Domain/wire `None` is not the same as `Some(0)`, even though the legacy ordering projection uses zero for absent preference evidence.

**Communication consequence**

A surface explanation may say “closest to the preferred start wins first,” but the technical layer must expose the complete tie-break sequence and the missing-vs-zero distinction.

---

## 11. Search assessment

### Claim

`ProvenOptimal` means optimal only for the explicitly declared, exhausted discrete grid and current objective.

**Status:** BUILT.

**Evidence**

- `crates/cerebri-planner/src/search.rs`.
- `docs/en/foundation.md` / `docs/de/foundation.md`.
- `docs/en/planner-integration.md` / `docs/de/planner-integration.md`.

Current behavior:

- exhausted grid + at least one solution → `ProvenOptimal`;
- exhausted grid + no solution → `Complete`;
- search stopped by budget → `BestFound`.

**Communication consequence**

Never translate `ProvenOptimal` into “the globally best possible schedule.” Always retain the grid/objective boundary at technical depth, and preferably a short qualifier at surface depth when the term is shown.

---

## 12. Recurrence and temporal context

### Claim

Existing bounded daily/weekly series can compile into planner occupancy. Prospective series, recurrence constraints, series mutation and broader recurrence forms remain unsupported.

**Status:** BUILT for the bounded existing-series path; FUTURE / RESEARCH for unsupported forms.

**Evidence**

- Master v0.4 synchronized with ADR-0012.
- `docs/en/planner-integration.md` / `docs/de/planner-integration.md`.
- Current planner compilation and recurrence tests.

**Communication consequence**

Do not generalize “recurrence support” into arbitrary RRULEs, provider recurrence management or planning new recurring series.

---

## 13. Timezones, DST and availability

### Claim

The temporal foundation uses explicit IANA zones, half-open intervals and bounded DST-aware recurrence diagnostics; incomplete availability coverage does not become free time.

**Status:** BUILT.

**Evidence**

- current temporal crate and tests;
- Master v0.4;
- ADR-0009 and ADR-0012;
- DE/EN planner integration.

**Communication consequence**

“Unknown” must remain distinct from “free.” A visual gap cannot be rendered as available unless coverage establishes it.

---

## 14. Lifecycle and execution authority

### Claim

Planning, validation, action translation, authorization and execution are separate states. Planning itself does not mutate an external calendar.

**Status:** BUILT as a typed lifecycle and mock/in-memory execution contract.

**Evidence**

- `docs/en/foundation.md` / `docs/de/foundation.md`.
- Master v0.4: “The planner never mutates a calendar.”
- `crates/cerebri-planner/src/lifecycle.rs`.
- `crates/cerebri-integrations/src/executor.rs` and in-memory adapters/tests.

**Limit**

Current adapters demonstrate the contract in memory. They are not production provider integrations.

**Communication consequence**

Do not visually collapse “validated plan” into “executed calendar change.” Authorization and execution remain separate downstream boundaries.

---

## 15. Provider integration

### Claim under audit

Cerebri currently integrates with production calendar/provider systems.

**Status:** FUTURE / RESEARCH.

**Evidence**

- `README.md` explicitly says no provider integration, production authentication or durable ledger is implemented.
- Site safety/developer copy describes authenticated providers, durable ledgers and reconciliation as future.
- Current integrations are ports/mock/in-memory contract adapters.

**Communication consequence**

Do not use provider logos, sync animations or “works with your calendar” language as current capability unless a later implementation establishes it.

---

## 16. Autonomous calendar mutation

### Claim under audit

Cerebri currently autonomously changes calendars.

**Status:** FUTURE / RESEARCH / explicitly not current.

**Evidence**

- Master v0.4 lists unrestricted autonomous calendar mutation as a non-goal of the initial core.
- README explicitly says autonomous calendar mutation is not implemented.
- Planner and executor boundaries are separate; current provider integration is absent.

**Communication consequence**

Proposal and planning visuals must stop before implying an external calendar was changed, unless the visual is explicitly about the mock lifecycle contract.

---

## 17. Machine learning and neural inference

### Claim under audit

Current Cerebri performs learned or neural inference/ranking.

**Status:** FUTURE / RESEARCH.

**Evidence**

- `crates/cerebri-ml/src/lib.rs` contains model/dataset/training metadata types and inference ports, not an implemented neural runtime.
- `docs/en/planner-integration.md` explicitly says no ML, training, learned search or provider work is introduced.
- Current roadmap states Slice 2, ML, learned ranking/search and EvaluationEpisode implementation are not current.
- README says neural inference is a future milestone.

### Public wording risk

The README workspace description for `cerebri-ml` includes “production model/dataset/training metadata and inference ports.” This is technically about interfaces/metadata, but “production” can be misread as an active production model. Later README work should preserve the interface fact while making absence of an inference backend immediately obvious.

---

## 18. Website truth labels and explanatory visuals

### Claim

The current site intentionally distinguishes repository-backed output, educational models and future concepts.

**Status:** BUILT as presentation infrastructure.

**Evidence**

- `apps/cerebri-site/src/lib/site.ts`: `TruthKind = 'REAL' | 'EDUCATIONAL' | 'FUTURE CONCEPT'`.
- `IntentPlan.svelte` marks its human-language trajectory as educational and exposes real CPIR/planner data.
- Site sections generally link to repository sources.

### Communication risk

Some surrounding visual language still uses terms such as “ORBITAL INTELLIGENCE SYSTEM,” “open interpretation space,” “human / probabilistic” and present-tense interpretation claims. These may make an educational/research concept feel like current runtime capability despite the truth labels.

Later website work should make the semantic distinction understandable without relying on a small badge to repair a broader false impression.

---

## 19. Release and version status

### Claim

Current software version is 0.2.0, unpublished/unreleased, undergoing release qualification. Specification 0.4, CPIR 0.2 (legacy 0.1), REST /v1 and Ranking Feature Schema 0.1 are independent version authorities.

**Status:** BUILT / current repository status.

**Evidence**

- `README.md`.
- `CHANGELOG.md`.
- `docs/development/roadmap/README.md`.
- Master v0.4 and ADR-0014 for independent authorities.

**Communication consequence**

Never infer a release, CPIR version, model version or REST version from another authority. A completed qualification slice is not a software release.

---

## 20. Current qualification state

### Claim

Deterministic Ranking Feature Contract Slice 1 is implemented and has completed independent Math and Security qualification; repository hardening and active qualification rules are in place. The overall software remains unreleased and release qualification continues.

**Status:** BUILT / current governance status.

**Evidence**

- `docs/development/roadmap/README.md` current-status section.
- `docs/development/progress/2026-09-26-final-governance-cleanup.md`.
- current `main` includes the governance follow-up merge at the audit base SHA.

**Communication consequence**

Keep “Slice-1 qualification” separate from “software release qualification” and from future Slice 2.

---

# Public-surface findings to carry into later PRs

## Finding CTF-01 — Present-tense interpretation language

**Type:** capability inflation risk  
**Surface:** homepage hero and `CerebriExplainer.svelte`  
**Issue:** current wording can imply that a running interpretation/NLP layer already explores free-form meaning. Current structured planner evidence does not establish that capability.  
**Later correction:** frame interpretation as architecture/research context or remove present-tense runtime implication. Preserve the current honest disclaimer in `IntentPlan.svelte`.

## Finding CTF-02 — Neural identity can outrun current implementation

**Type:** current/future ambiguity  
**Surface:** README identity line and related visual language  
**Issue:** “neural intuition + symbolic verification” is an architectural/research principle, while the current planner is deterministic and neural inference is future.  
**Later correction:** keep the research identity only with immediate current-state qualification; do not describe the current planner as neural.

## Finding CTF-03 — ADR-0014 status line is stale

**Type:** documentation drift  
**Surface:** `docs/architecture/decisions/ADR-0014-ranking-feature-contract.md`  
**Issue:** header says implementation is pending independent review, while current roadmap/governance says Slice 1 is implemented and independently Math/Security-qualified.  
**Later correction:** update status text in the dedicated documentation-consistency slice or another narrowly authorized documentation change. Do not alter contract semantics.

## Finding CTF-04 — Visual intelligence/probabilistic wording is semantically stronger than evidence

**Type:** visual-language ambiguity  
**Surface:** homepage field readout and educational interpretation diagram  
**Issue:** phrases such as “ORBITAL INTELLIGENCE SYSTEM” and “HUMAN / PROBABILISTIC” can imply an active probabilistic subsystem.  
**Later correction:** visual grammar should distinguish current deterministic computation from research/educational layers by structure, not only badges.

## Finding CTF-05 — `cerebri-ml` workspace label needs stronger boundary context

**Type:** terminology ambiguity  
**Surface:** root README workspace table  
**Issue:** the crate currently exposes metadata and ports, not an implemented inference/training backend.  
**Later correction:** describe exactly that boundary and link to the research horizon.

---

# Claims that should remain prominent

The audit also found several current public statements worth preserving:

- current planner search is bounded and deterministic;
- Rust remains the planning/domain authority;
- the website/Lab do not implement a JavaScript planner;
- CPIR and software/API/spec versions are independent;
- planning does not grant permission;
- validation does not equal authorization;
- current provider integration is absent;
- current learning/neural inference is absent;
- `ProvenOptimal` is scoped to the declared grid/objective;
- recurrence support is bounded rather than general;
- unknown availability is not free time;
- current software is unreleased.

These statements form the factual backbone for later README and website restructuring.

# Review rule

When any source listed above changes semantics, version/status, authority, supported operations or failure behavior, review this inventory and every dependent public explanation. The inventory should follow repository truth; repository behavior must never be changed merely to preserve this document.
