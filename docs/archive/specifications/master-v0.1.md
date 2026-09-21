# Nexus Cerebri --- Master Specification v0.1

**Status:** Planning baseline\
**Specification version:** 0.1\
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

## 3. End-to-end architecture

``` text
Text / Voice transcript / Structured App Input
                    |
                    v
           Interpretation Layer
                    |
                    v
 Intent + Entities + Temporal Facts
 + Semantic Signals + Confidence
                    |
                    v
              CPIR Builder
                    |
                    v
               Validation
                    |
                    v
        Deterministic Planning Core
 Candidate Generation / Constraints / Search
       Repair / Scoring / Ranking
                    |
                    v
             ProposedPlan(s)
                    |
                    v
          Final Plan Validation
                    |
                    v
               ActionPlan
                    |
                    v
 Permission + Confirmation Policy
                    |
                    v
          Integration / Adapter
                    |
                    v
             External System
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
-   execution policy
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
-   permission_scope

**Inference can rank possibilities, but cannot rewrite facts.**

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

Statuses: OPTIMAL, BEST_FOUND, NO_SOLUTION, NEEDS_RELAXATION,
INSUFFICIENT_INFORMATION.

Never claim optimality for budget-limited best-so-far search.

## 11. Operations

CREATE, MOVE, UPDATE, CANCEL, FIND_SLOT, RESCHEDULE, OPTIMIZE, PLAN,
ANALYZE.

Intent is separate from executable action. Every operation has bounded
scope.

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

Planner -\> ProposedPlan -\> Validation -\> ActionPlan -\>
Permission/Confirmation -\> Adapter.

Actions: CREATE_EVENT, MOVE_EVENT, UPDATE_EVENT, DELETE_EVENT.

Use idempotency keys.

States: PROPOSED, APPROVED, EXECUTING, EXECUTED, FAILED, STALE,
REJECTED, CANCELLED.

Early releases treat deletion, broad optimization and modification of
external events conservatively.

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

## 25. Version/Git

Semantic Versioning; research phase uses 0.x.y.

Track software, API, CPIR schema, model and dataset versions separately.

Use Conventional Commits. Changelog entries are release-relevant and
dated.

## 26. Observability/reproducibility

Separate logs, metrics and traces. Planning requests support trace IDs.

TrainingRun records run ID/date/Git commit/software+dataset
versions/checksum/architecture/hyperparameters/seed/environment/metrics/model
artifact.

Preserve baselines.

Goal:
`Code + Data + Config + Seed + Environment -> reproducible experiment`.

## 27. Deployment progression

LOCAL -\> TEST -\> SHADOW -\> SUGGESTION -\> CONFIRMATION -\>
LIMITED_AUTOMATION.

Support both embedded/local and server deployment.

## 28. Development method

Understand -\> simple implementation -\> visualize -\> test limitations
-\> understand failure -\> advanced concept -\> compare -\> retain
justified improvement.

## 29. Definition of Done

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

## 30. v0.1 implementation boundary

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
