# Nexus Cerebri — Master Specification v0.4

**Status:** Planning baseline\
**Specification version:** 0.4\
**Baseline date:** 2026-09-19\
**Repository:** `github.com/YoungJibbit95/Nexus-Cerebri`\
**Primary implementation language:** Rust\
**Documentation:** German + English\
**Core principle:** **Neural intuition + symbolic verification.**

## 1. Mission and scope

Nexus Cerebri is an application-independent temporal planning and
reasoning engine. Its first concrete product form is a scheduling
engine; its architecture should later support general temporal planning
for tasks, meetings, resources, shifts, learning plans, rooms, projects,
travel, production and comparable constrained processes.

Cerebri is not the calendar UI. It receives planning context,
understands temporal and semantic meaning, constructs a validated
planning representation, generates feasible alternatives, ranks them,
explains the result and returns proposed actions.

### Non-goals of the initial core

-   general-purpose LLM/chatbot
-   speech recognition
-   calendar UI
-   email assistant
-   authentication provider
-   calendar provider
-   unrestricted autonomous calendar mutation
-   ML as final authority for hard validity

## 2. Governing principles

1.  Facts outrank inference.
2.  Explicit constraints outrank learned preferences.
3.  Permissions outrank planning decisions.
4.  Unknown is explicit; `UNKNOWN != 0`.
5.  Planner and executor are separate.
6.  Probabilistic components interpret/rank; deterministic components
    validate reality.
7.  Implement simple versions first, while preserving interfaces/data
    models needed for justified future complexity.
8.  Code + tests + visualization + documentation = feature.
9.  GitHub represents the current documented state of Cerebri.
10. Important reasoning must be explainable at SIMPLE, TECHNICAL and
    RESEARCH depth.

Precedence:

`Permissions > Policy > Hard Constraints > Explicit Current Preference > Session Context > Personal Learned Preference > Global Learned Prior > Default`

### Policy is a formal layer

`Policy` is a typed, deterministic rule layer distinct from constraints and preferences. Policies govern what the system is allowed to propose or execute (for example mutation limits, confirmation requirements, or deployment-mode restrictions). A soft preference becomes a policy only through explicit configuration or user/application action; learning cannot silently promote it.

### Documentation governance

The statement “GitHub represents the current documented state” applies to the **default branch and releases**, not every intermediate feature-branch commit. Pull requests may temporarily contain partial work, but merge gates require affected code, tests and documentation to be coherent.


### Normative authority

When documents conflict, use this precedence:

1. **Accepted ADRs that explicitly amend/supersede a named Master Specification rule**
2. **Master Specification**
3. **Domain standards** (testing/research, documentation/release, security and similar standards)
4. **Coding Agent Implementation Boundary**
5. **Roadmap**
6. **Review-resolution documents and progress logs**

An ADR may override the Master Specification only when it explicitly identifies the superseded section/decision and the Master Specification is updated in the same change or immediately before merge. Therefore, on the default branch, accepted ADRs and the Master Specification should not remain knowingly contradictory.

Review-resolution documents are historical/audit records after consolidation and are never the sole normative source of an accepted architecture decision.

## 3. End-to-end architecture

```text
Input (Text / Voice Transcript / App / API)
        ↓
Interpretation / Semantic + Temporal Parsing
        ↓
CPIR Build + Validation
        ↓
PlanningRequest
        ↓
Planner
        ↓
ProposedPlan
        ↓
Deterministic Plan Validation
        ↓
ValidatedPlan
        ↓
Action Translation
        ↓
ActionPlan
        ↓
Policy + Capability + Confirmation + Authorization
        ↓
AuthorizedActionPlan
        ↓
Executor Freshness / Revision / Idempotency Preconditions
        ↓
Integration Adapter
        ↓
External System
        ↓
ExecutionResult
```

The planner never mutates a calendar.

## 4. CPIR

Raw natural language never enters the scheduler directly. The Cerebri
Planning Intermediate Representation contains:

-   `schema_version`
-   `request_id`
-   `operation`
-   bounded `scope`
-   temporal/context snapshot
-   planning objects
-   facts
-   hard constraints
-   soft constraints
-   preferences
-   semantic inference
-   PolicyContext
-   provenance
-   per-field confidence

Validation states:

-   `VALID`
-   `VALID_WITH_UNCERTAINTY`
-   `INSUFFICIENT_INFORMATION`

Unresolved time values may exist during interpretation but may not enter
actual optimization.

Provenance values include `USER_EXPLICIT`, `INTEGRATION_FACT`,
`SYSTEM_FACT`, `PERSONAL_LEARNED`, `GLOBAL_LEARNED`, `MODEL_INFERENCE`,
and `DEFAULT`.


### Field knowledge states

Every semantically relevant CPIR field has an explicit state when absence/uncertainty is meaningful. `KNOWN`, `MISSING`, `UNKNOWN`, `UNCERTAIN`, and `AMBIGUOUS` are epistemic states; `UNRESOLVED` is a temporary processing state:

- `KNOWN` — a sufficiently established value exists.
- `MISSING` — no value/evidence was supplied.
- `UNKNOWN` — the field is relevant, but its value cannot currently be determined.
- `UNCERTAIN` — a candidate value exists, but confidence is insufficient for a certainty-requiring use.
- `AMBIGUOUS` — multiple materially different interpretations remain plausible.
- `UNRESOLVED` — temporary **processing state**, not an epistemic state; parsing/resolution has not completed yet. After processing it must become `KNOWN`, `MISSING`, `UNKNOWN`, `UNCERTAIN`, or `AMBIGUOUS`.

Confidence may accompany a candidate value but does not replace knowledge state.

A planning operation declares which fields are required. If a required field is `MISSING`, `UNKNOWN`, `AMBIGUOUS`, or `UNRESOLVED`, optimization is blocked and the result becomes `INSUFFICIENT_INFORMATION` or a clarification flow. `UNCERTAIN` may proceed only where the operation/policy explicitly permits uncertainty and the resulting plan records it.


### Policy context in CPIR

CPIR does not embed an open-ended “PolicyContext” blob. It carries a bounded `PolicyContext` containing:

- `policy_set_id` and/or `policy_version`;
- an immutable request-time `PolicySnapshot` of policy inputs relevant to planning;
- `ConfirmationRequirements`;
- `MutationPolicy`;
- deployment/execution mode where applicable.

Policy evaluation remains deterministic. The snapshot supports explanation/reproducibility; current authorization and freshness are still rechecked before execution.

## 5. Planning object model

Use composition and Rust enums/structs.

Core variants:

-   Event
-   Task
-   Deadline
-   Availability
-   Resource

Common metadata includes ID, type, version, facts, semantic profile,
constraints, dependencies, required resources, provenance, confidence
and timestamps.

Tasks may define `splittable`, `interruptible`, minimum/preferred chunk
duration and maximum chunk count.

Resources later generalize people, rooms, vehicles, machines, devices
and teams.

## 6. Temporal engine

The temporal engine is deterministic and trusted.

Strong concepts:

-   Instant
-   LocalDate
-   LocalTime
-   ZonedDateTime
-   Duration
-   TimeRange
-   Deadline
-   Recurrence
-   PlanningHorizon

Use real timezone rules. Preserve original timezone while deriving
unambiguous instants.

Document intervals as half-open `[start, end)` unless changed through an
ADR.

Overlap:

`A.start < B.end && B.start < A.end`

Relations: BEFORE, AFTER, OVERLAPS, CONTAINS, INSIDE, TOUCHES, EQUAL.

Recurrence is expanded only inside a bounded PlanningHorizon.

Candidate granularity is configurable; proposed initial default: 15
minutes.

Buffers: pre, post and travel.

Natural-language temporal parsing remains outside this engine.

## 7. Semantic representation

Priority is multidimensional. Event Semantic Vector v0.1:

`E = [I, U, R, X, H, C, D, F]`

-   I --- importance
-   U --- urgency
-   R --- temporal rigidity
-   X --- external dependency
-   H --- human dependency
-   C --- rescheduling cost
-   D --- consequence of delay/missing
-   F --- duration flexibility

Deadline pressure is derived dynamically from deadline facts and
remaining time.

Semantic signals are evidence, not hard rules. Sources: GENERAL,
DOMAIN/INTEGRATION, PERSONAL.

Explicit facts override inferred semantics.

## 8. Hard constraints

Initial vocabulary:

-   no_overlap
-   explicit_time
-   explicit_date
-   earliest_start
-   latest_end
-   deadline
-   min_duration
-   fixed_duration
-   availability_window
-   dependency_order
-   required_buffer
-   recurrence_rule
-   timezone_integrity
-   external_lock
**Inference can rank possibilities, but cannot rewrite facts.**

Permissions are deliberately **not** ordinary hard constraints. They form a higher security/capability layer:

- **Planning Capability:** constrains which executable mutations the planner may include in an executable proposal.
- **Execution Authorization:** is checked again immediately before execution.
- ANALYZE/simulation may optionally show hypothetical `NON_EXECUTABLE` solutions outside current write capabilities when policy permits, but these can never become executable ActionPlans without authorization.

FACT, CONSTRAINT, INFERENCE and PREFERENCE must remain separate
concepts/types.

## 9. Preferences and scoring

Initial soft factors:

-   preferred_time
-   schedule_fragmentation
-   buffer_preference
-   task_grouping
-   context_switch_cost
-   early_completion
-   schedule_stability
-   workload_balance
-   personal_pattern

An initial weighted cost function is acceptable. Lower cost is better.
Keep multiple ranked candidates.

Preference precedence: explicit user \> personal learned \>
integration/domain \> global learned \> default.

## 10. Candidate generation and repair

Tiers:

0.  direct free slots
1.  relax soft preferences / flexible duration
2.  move low-rigidity movable events
3.  jointly replan multiple objects
4.  no safe solution: explain conflict / ask user

Hard constraints define feasible space before soft ranking.

No-solution cases create a `ConflictSet`; movable blockers may create
`RepairCandidate`s.

Repair cost considers number of moved objects, total shift, rigidity,
rescheduling cost, preference violations, external dependency and
schedule disruption.

Principle: **find a valid solution with minimal disruption.**

SearchBudget includes max candidates, repairs, moved objects, search
depth and timeout.

Planner result semantics are two-dimensional:

`PlanningOutcome`: `SOLUTION`, `NO_SOLUTION`, `NEEDS_RELAXATION`, `INSUFFICIENT_INFORMATION`.

`SearchAssessment`: `PROVEN_OPTIMAL`, `COMPLETE`, `BEST_FOUND`.

`PROVEN_OPTIMAL` requires proof/exhaustive evaluation over the explicitly declared search space, objective, constraints, and deterministic tie-break rules. Budget- or timeout-limited search can only report `BEST_FOUND`.



`SearchAssessment::COMPLETE` means the explicitly declared search space was fully traversed/exhausted, but no optimization proof is asserted. It is suitable for exhaustive no-solution/conflict analysis or algorithms where traversal completion alone does not prove optimality.

`SearchAssessment::PROVEN_OPTIMAL` is stronger: the declared search space is complete and the returned solution is proven optimal for the declared objective and deterministic tie-break rules.

`SearchAssessment::BEST_FOUND` means search stopped before completion/proof.

## 11. Operations

CREATE, MOVE, UPDATE, CANCEL, FIND_SLOT, RESCHEDULE, OPTIMIZE, PLAN,
ANALYZE.

Intent is separate from executable action. Every operation has bounded
scope.

### PlanningScope

Every operation carries a typed, bounded `PlanningScope`. Initial fields are:

- `time_range`
- `calendar_ids`
- `object_ids`
- `resource_ids`
- `integration_ids`
- `movable_object_ids` and/or explicit `movement_policy`
- `max_mutations`

Visibility/read access does not imply mutability. Missing scope dimensions never silently expand to “all visible objects” unless an explicit policy defines that expansion. `MOVE`, `RESCHEDULE`, and `OPTIMIZE` may mutate only objects included by scope, planning capability, and policy.


#### Scope field presence semantics

For optional collection dimensions such as `object_ids`, `movable_object_ids`, `calendar_ids`, `resource_ids`, and `integration_ids`:

- omitted / `None`: this dimension adds no additional scope filter; it never grants permission or mutability;
- present as `[]`: explicitly selects no objects for that dimension;
- present as `[A, B]`: restricts that dimension to exactly those identifiers.

The effective mutable set is the intersection of scope, policy, planning capability, hard constraints, and current authorization. Read visibility never implies mutability.

`max_mutations = 0` means **analysis-only / non-mutating planning**. No executable mutating ActionPlan may be produced.

## 12. NLP/semantic pipeline

Normalization -\> Intent -\> Entity Extraction -\> Temporal Parsing -\>
Semantic Signals -\> Context Resolution -\> Confidence/Ambiguity -\>
CPIR Builder -\> Validation.

Initial entities: DATE, TIME, DURATION, TIME_WINDOW, LOCATION, PERSON,
ACTIVITY, RESOURCE, DEADLINE, RECURRENCE.

Confidence is per field.

Ambiguity produces `ClarificationRequest` rather than silent guessing.

Use deterministic/hybrid parsing for exact
time/date/duration/timezone/basic recurrence/hard constraints.

## 13. ML strategy

Cerebri is not a small general LLM. It borrows useful mechanisms while
maintaining an explicit temporal world model.

Potential ML responsibilities:

-   intent classification
-   semantic signals
-   category classification
-   preference learning
-   candidate ranking
-   search guidance

Hard validity remains deterministic.

Roadmap:

-   ML-v0: Tokens -\> Embedding -\> Mean Pooling -\> MLP -\> Intent
-   ML-v1: sequence-aware encoder
-   ML-v2: self-attention
-   ML-v3: small Transformer + multi-task heads
-   later: personal representations and ML-guided search

## 14. Educational ML Lab

Separate learning implementation for matrix/tensor basics, embedding
lookup, Linear, ReLU, Softmax, Cross-Entropy, SGD and manual
Backpropagation.

Compare against numerical gradients and a framework/autograd reference.

Educational clarity may take precedence over performance. Keep it
separate from production inference.

## 15. Data and training

Four levels:

1.  structured synthetic
2.  linguistically varied synthetic
3.  curated human examples
4.  real feedback

Avoid template leakage. Maintain a stable challenge set containing
negation, ambiguity, corrections, references, typos, contradictions,
missing information, short/long requests and hard negatives.

Feedback: explicit correction, selection, modification, rejection,
passive behavior. No change is not automatically approval.

Data scopes: SYNTHETIC, GLOBAL_CURATED, PERSONAL_LOCAL, OPT_IN_SHARED.

Personal feedback does not become global training data by default.

Evaluation: intent metrics, entity F1, temporal accuracy, semantic
metrics, calibration, ambiguity detection, ranking quality, Top-K
acceptance and hard-constraint violations.

Validated hard-constraint violation target: **0**.

This is a safety invariant, not sufficient evidence that the validator is correct. Validator correctness is measured separately through mutation/negative tests, property tests, adversarial fixtures and independently constructed scenario expectations. Invalid inputs/plans must be tested to ensure the validator actually rejects violations.

## 16. Personal preference learning

Start interpretable: feature vectors and weights/pairwise ranking.
Repeated evidence changes preferences gradually.

Separate global priors, personal long-term patterns, session context and
explicit current requests.

Prepare data for future user embeddings without implementing them
prematurely.

## 17. Rust workspace

``` text
crates/
  cerebri-types/
  cerebri-temporal/
  cerebri-constraints/
  cerebri-semantics/
  cerebri-planner/
  cerebri-preferences/
  cerebri-ml/
  cerebri-core/
  cerebri-integrations/

apps/
  cerebri-api/
  cerebri-lab/

bindings/
  node/
  wasm/  # later

research/
  ml-from-scratch/
```

Deterministic lower layers may not depend on probabilistic upper layers.

Core remains independent of HTTP, database and provider schemas.

## 18. Persistence

The core owns no database.

Separate external state, configuration, learned state and
operational/audit state.

Ports: PreferenceStore, FeedbackStore, ModelRegistry, ActionLedger.

`ActionLedger` belongs to the execution/application layer, not the pure planner. The executor/service layer owns atomic action-state transitions, idempotency checks, retry/recovery policy and reconciliation after process failure. The planner only emits immutable planning results.

Concrete stores may be PostgreSQL, SQLite or in-memory.

Model artifacts are immutable/versioned.

`Planner.plan()` must not silently write storage, call providers, train
models or mutate calendars.

## 19. API and integrations

One core, multiple interfaces:

-   Rust library
-   Node binding
-   REST
-   WASM later

Transport contains no planning logic.

Adapters declare capabilities: read
events/availability/tasks/resources/participants and
create/update/move/delete where supported.

Provider-specific fields are mapped at the adapter boundary.

Use optimistic concurrency; stale revisions require revalidation.

## 20. Execution safety

The canonical lifecycle is:

`PlanningRequest -> ProposedPlan -> ValidatedPlan -> ActionPlan -> AuthorizedActionPlan -> ExecutionResult`

These are distinct strong domain types in the foundation architecture:

- `ProposedPlan` — planner output; never executable.
- `ValidatedPlan` — ProposedPlan validated against facts, scope, temporal rules, hard constraints and a specific context/revision.
- `ActionPlan` — explicit mutation intent derived from a ValidatedPlan; not yet executable.
- `AuthorizedActionPlan` — ActionPlan that passed current policy, planning capability, confirmation requirements and execution authorization. Only this type may be submitted to an executor.
- `ExecutionResult` — immutable result record for the attempted execution.

A stale source revision invalidates validation/authorization and requires revalidation before execution.

Actions: CREATE_EVENT, MOVE_EVENT, UPDATE_EVENT, DELETE_EVENT.

Use idempotency keys.

Operational ledger states may include PROPOSED, APPROVED, EXECUTING, EXECUTED, FAILED, STALE, REJECTED and CANCELLED; these are persistence/execution states, not substitutes for the strong lifecycle types above.

Early releases treat deletion, broad optimization and modification of external events conservatively.


### ExecutionResult and executor preconditions

One `ExecutionResult` represents one execution attempt for one `AuthorizedActionPlan`. It contains ordered per-action `ActionExecutionResult` records so multi-action plans can represent partial success/failure explicitly (`SUCCEEDED`, `FAILED`, `SKIPPED`, plus provider/error metadata). Partial success must not collapse into one boolean.

`AuthorizedActionPlan` records successful authorization for a specific context/revision, but the executor must immediately before the first external side effect recheck:

- source revision/freshness;
- authorization/capabilities where re-checkable;
- confirmation requirements;
- idempotency/replay protection.

If a final precondition fails, no new external mutation may begin; the attempt becomes stale/rejected and requires revalidation/re-authorization.

## 21. Security/privacy

**Collect less, infer carefully, retain deliberately.**

Separate READ, PLAN and EXECUTE scopes.

Use data classification/redaction. Never commit credentials or
production personal data.

Threat model includes malicious input, compromised adapters, permission
escalation, replay, stale plans, log leakage, data/model poisoning,
denial of service, oversized requests, corrupted artifacts and
supply-chain risk.

ML can never bypass permissions/hard constraints.

## 22. Testing

Layers: unit, property-based, fuzzing, golden/scenario,
integration/contract and ML regression/evaluation.

Core invariants:

-   ValidatedPlan =\> zero hard-constraint violations
-   Execution =\> validated ActionPlan
-   LearnedPreference cannot override explicit constraint
-   Inference cannot override fact
-   StalePlan =\> revalidation
-   Unknown != assumed
-   Planner != executor


Safety invariant: `Execution => AuthorizedActionPlan`.

## 23. Visualization

Visualization is first-class.

Cerebri Lab eventually visualizes timelines, candidates, rejection
reasons, score decomposition, repair search, semantic vectors/evidence,
training curves, F1/calibration/confusion matrices, gradients, dataset
distributions, preference evolution and traces.

Every important graph should expose underlying data.

Explanation modes: SIMPLE, TECHNICAL, RESEARCH.

## 24. Documentation

Documentation is Markdown and versioned in Git.

Root: README, CHANGELOG, CONTRIBUTING, SECURITY, ROADMAP, version
metadata.

Detailed DE/EN docs live below `docs/`. Important architecture changes
use ADRs.

GitHub Pages is generated from the same source.

Code/identifiers/commits remain English.

Important concepts offer intuitive, technical and scientific
explanations. Central formulas define variables and intuition. Research
docs distinguish established methods from Cerebri-specific changes.

## 25. Version authorities and terminology

The documentation baseline/specification revision is **not** the software release version.

Canonical authorities:

- **Software version:** root Cargo workspace package metadata / designated workspace package version; release tags use `vMAJOR.MINOR.PATCH`.
- **API version:** route/protocol declaration (initially `v1` only when the public API is intentionally declared).
- **CPIR schema version:** serialized CPIR `schema_version` plus schema documentation; initial implementation starts at `0.1`, not “v1”.
- **Model version:** immutable ModelRegistry metadata/artifact ID.
- **Dataset version:** dataset manifest metadata/checksum.
- **Specification revision:** document front matter/header, e.g. `0.2`; no software compatibility guarantee is implied.

README surfaces the software version/status and links the current specification revision.

The labels `ML-v0`, `ML-v1`, etc. are **architecture-learning milestones**, not model artifact versions.

All machine-readable dates use ISO 8601 (`YYYY-MM-DD`; timestamps RFC 3339 where appropriate).

## 26. Version/Git

Semantic Versioning; research phase uses 0.x.y.

Track software, API, CPIR schema, model and dataset versions separately.

Use Conventional Commits. Changelog entries are release-relevant and
dated.

## 27. Observability/reproducibility

Separate logs, metrics and traces. Planning requests support trace IDs.

TrainingRun records run ID/date/Git commit/software+dataset
versions/checksum/architecture/hyperparameters/seed/environment/metrics/model
artifact.

Preserve baselines.

Goal:
`Code + Data + Config + Seed + Environment -> reproducible experiment`.

## 28. Deployment progression

LOCAL -\> TEST -\> SHADOW -\> SUGGESTION -\> CONFIRMATION -\>
LIMITED_AUTOMATION.

Support both embedded/local and server deployment.

## 29. Development method

Understand -\> simple implementation -\> visualize -\> test limitations
-\> understand failure -\> advanced concept -\> compare -\> retain
justified improvement.

## 30. Definition of Done

When applicable:

-   implementation complete
-   unit/property/scenario tests complete
-   CI green
-   visualization/debug output considered
-   DE docs updated
-   EN docs updated
-   README accurate
-   ADR current
-   changelog/version updated if release-relevant
-   dated progress log updated
-   no stale documentation introduced

## 31. Initial software v0.1.0 implementation boundary

### Build initially

Workspace, strong types, temporal primitives, CPIR/validation, basic
hard constraints, deterministic candidates, simple scoring, plan/action
separation, storage ports/in-memory stores, API/Node skeletons, Cerebri
Lab foundation, testing, documentation, CI.

### Prepare but do not fully implement

Repair-search extension points, multi-resource planning,
preference-learning interfaces, model registry, semantic encoder
interfaces, adapter capability contracts, experiment registry, advanced
visualization hooks.

### Deliberately reserved learning milestones

Neural net from scratch, manual backprop, framework comparison, sequence
encoder, attention, Transformer, multi-task model, learned ranking,
personal embeddings, ML-guided search, advanced optimization.

A scaffolding agent must not silently implement these learning
milestones.


## 32. Additional domain definitions

### External lock

`external_lock` means a deterministic immutability restriction originating outside the planner, such as provider immutability, organizer/ownership rules, explicit user lock, or integration policy. Its provenance and reason must be recorded.

### External participant availability

Unknown external participant availability is represented as UNKNOWN rather than silently free or busy. Whether it blocks planning depends on the operation/policy: a meeting requiring all participants' verified availability must block or request clarification; exploratory suggestions may be marked uncertain/non-executable.

### Personal data terminology

`PERSONAL_LOCAL` means logically scoped to one user/profile and excluded from global/shared training by default. It does **not** necessarily mean physically stored only on the local device. Physical storage location is a deployment/privacy policy decision.

`personal_pattern` should preferentially store derived/aggregated evidence needed for ranking rather than unnecessary raw event text. Retention, export and deletion policies apply to both raw feedback and derived profiles.

### Cerebri Lab

Cerebri Lab is initially an internal developer/research/learning interface, not a stable end-user product API. Accessibility and usability remain quality goals, but its UI/API compatibility is not guaranteed during 0.x.

### Documentation parity

DE/EN parity is required for canonical public architecture/reference/learning pages. CI can verify counterpart existence and structural metadata; semantic equivalence remains a human review responsibility. Internal progress logs, raw experiment notes and ADR source records may have one canonical language unless a later policy requires translation.

### ADR triggers

An ADR is mandatory for changes to public API contracts, CPIR/schema semantics, dependency direction/module boundaries, persistence ownership, planner/executor boundary, security/permission model, versioning authority, interval/time semantics, solver/search semantics, or replacement of an accepted architectural strategy.

### Progress logs

Meaningful implementation or architecture-design sessions should be logged. Pure reading/research sessions need a progress entry only when they produce a decision, experiment, finding, or materially change the plan.

## Foundation typing requirement

The foundation implementation must create real Rust type boundaries/skeletons for `ProposedPlan`, `ValidatedPlan`, `ActionPlan`, `AuthorizedActionPlan`, and `ExecutionResult`. They do not need full future behavior, but they must not be aliases of one generic plan type. Constructors/conversions should enforce the intended lifecycle so later execution code cannot accidentally accept a raw `ProposedPlan`.


## Specification archival policy

The repository root contains only the **current normative Master Specification**. Superseded Master Specification revisions belong under `docs/archive/specifications/` and are historical/non-normative. Coding agents must not treat archived revisions as current architecture.

Specification v0.4 is the **Foundation Architecture Baseline**. Future architectural changes use ADRs and synchronized updates to the current Master Specification before merge.
