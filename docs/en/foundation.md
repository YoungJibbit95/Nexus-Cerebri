<!-- doc: foundation; lang: en; counterpart: ../de/foundation.md -->
# Foundation architecture

## Which part does what?

In the [appointment example](introduction.md), 10:00 comes back as the first proposal.
Several parts contribute: one describes the input, another compares time intervals,
and others check rules and order the valid options. The website then displays the
result. No calendar entry has been created.

The Rust code is divided into packages called **crates**. These boundaries help define
a rule in one place and apply it consistently across applications. The following
sections trace the path through the existing code.

[Deutsch](../de/foundation.md) · [From an appointment request to CPIR](cpir.md)

## From input to proposal

1. **Check the inputs.** An application supplies a structured `PlanningRequest`, which
   reaches the planner through `cerebri-core`. Input validation checks the duration,
   target object, search boundaries and planning grants, among other things. Missing
   required information stops the search.
2. **Try possible times.** The planner generates starts on the specified grid. Time
   operations from `cerebri-temporal` and rules from `cerebri-constraints` help check
   whether a candidate fits. In the example, four starts fail because they overlap
   the existing appointment.
3. **Compare the remaining options.** `cerebri-preferences` calculates comparison values,
   such as distance from a supplied preferred time. The planner uses these values in
   its fixed ordering. The example has no preferred time, so the earlier start breaks
   the tie between 10:00 and 10:15.
4. **Return the result.** A `PlanningResult` contains candidates, rejection reasons and
   information about the searched space. `cerebri-core` returns it to the calling
   application. The proposed placements are not permission to execute them.

`cerebri-core` is the shared entry point for applications. The actual search lives in
`cerebri-planner`; Core forwards the call there. REST and the Node bridge therefore use
the same planner.

## Where the rules live in the code

| Responsibility | Code that owns it |
| --- | --- |
| Represent identifiers, provenance and states such as known or missing | [cerebri-types](../../crates/cerebri-types/src/lib.rs) |
| Handle instants, duration, intervals and time zones | [cerebri-temporal](../../crates/cerebri-temporal/src/lib.rs) |
| Represent and check facts and required rules | [cerebri-constraints](../../crates/cerebri-constraints/src/lib.rs) |
| Resolve supplied preferences and calculate comparison values | [cerebri-preferences](../../crates/cerebri-preferences/src/lib.rs) |
| Represent semantic information such as urgency with its provenance; unused in the appointment example | [cerebri-semantics](../../crates/cerebri-semantics/src/lib.rs) |
| Define CPIR inputs, search candidates and check transitions through authorization | [cerebri-planner](../../crates/cerebri-planner/src/lib.rs) |
| Give applications shared functions for input validation, planning and temporal diagnostics | [cerebri-core](../../crates/cerebri-core/src/lib.rs) |
| Recheck authorized actions before execution and pass them to an adapter | [cerebri-integrations](../../crates/cerebri-integrations/src/executor.rs) |

Planning requests and plan stages belong to `cerebri-planner`. The shared foundations
in `cerebri-types` do not contain all planning logic. The executor and the execution
record interface `ActionLedger` belong to `cerebri-integrations`.

## How applications reach the planner

The [REST application](../../apps/cerebri-api/src/main.rs) receives requests and calls
Core. The [Node binding](../../bindings/node/README.md) currently starts a Rust process
bridge. Both paths use the Rust computation. Neither an HTTP call nor a JavaScript
call adds a separate planning rule. There is no execution endpoint for calendar changes.

The [Lab](../../apps/cerebri-lab/README.md) sends requests to the API and displays its
responses. The public website works differently: when the pages are built,
[the build script](../../scripts/build-site-data.mjs) runs Rust examples and saves their
results for display. Visiting the website does not start a new planner search. Both
interfaces present results rather than calculating a second ranking.

## Dependencies, data flow and validation steps

These three relationships answer different questions:

- **Code dependency:** Which package may use another package's types or functions?
  For example, `cerebri-integrations` imports types from `cerebri-planner`. This does
  not mean that the planner calls the executor.
- **Data flow:** Where do requests and results go? A Lab request travels through the
  API and Core to the planner; the result returns through the same interfaces.
- **Validation step:** What must pass before a plan reaches the next stage? A proposal
  becomes a `ValidatedPlan` only after validation succeeds.

Selected code dependencies are:

- `cerebri-api` uses `cerebri-core`.
- `cerebri-core` uses `cerebri-planner` and `cerebri-temporal`.
- `cerebri-planner` uses `cerebri-constraints` and `cerebri-preferences`, among others.
- `cerebri-constraints` uses `cerebri-temporal`.
- `cerebri-preferences` uses `cerebri-types`.
- `cerebri-integrations` uses `cerebri-planner`.

This list describes imports, not runtime calls. The core never imports integrations or ML.
The [repository checker](../../scripts/check-repository.mjs) enforces an explicit crate
allowlist; Cargo rejects cycles. The foundation crates `cerebri-types` and
`cerebri-temporal` do not depend on higher Cerebri packages.

## Why a proposal cannot be executed yet

The planner has checked and compared possible placements. Before making a change, an
application must still validate the complete plan against the current context, translate
it into actions and have its permissions checked. These steps live in the
[lifecycle code](../../crates/cerebri-planner/src/lifecycle.rs). A trusted application
supplies the current permissions and any required confirmations.

Only an `AuthorizedActionPlan` can be passed to the
[executor](../../crates/cerebri-integrations/src/executor.rs). Even then, it rechecks
the data and permissions immediately before a change. A previously suitable time might
now be occupied, or a permission might have been withdrawn.

The existing adapters demonstrate this process with in-memory data. Production calendar
connections and production authentication are not implemented yet. The website example
stops at the proposal.

### Technical details of the plan stages

Lifecycle: PlanningRequest -> ProposedPlan -> ValidatedPlan -> ActionPlan ->
AuthorizedActionPlan -> ExecutionResult. Proposal creation is public and untrusted.
Later stages have private fields, controlled conversions and no Deserialize implementation.
The executor's type signature rejects all three earlier plan stages.

Facts, constraints, semantic evidence, preferences and permissions use different types.
The request captures a policy snapshot; live authorization comes from a trusted application,
never from the REST request. READ, PLAN and per-object EXECUTE grants are separate.
Grants bind action kind, object, calendar and integration.

Execution first atomically claims the plan and all action keys in ActionLedger, then
rechecks the complete context, source revision, current policy, planning capability,
execution grants, confirmation and adapter capabilities before the first provider call.
Provider apply must enforce object revisions atomically. Processing stops on the first
failure; remaining actions are Skipped. Successful earlier writes are not rolled back.
A lost ledger completion or uncertain provider response is RecoveryRequired. Claimed keys
remain blocked. In-memory implementations demonstrate this contract but do not survive restart;
durability, reconciliation and retry policy require a later application adapter.

Plan IDs are SHA-256 digests of canonicalized request/placements; action IDs and idempotency
keys add a stable action index. Confirmation binds the exact plan, including policy/snapshot.
A changed context requires a new validation and authorization.

## Where future learning methods belong

[cerebri-ml](../../crates/cerebri-ml/src/lib.rs) contains model and dataset metadata,
plus interfaces for registration and inference. Those definitions do not provide a
trained model or a running learning method. The current planner does not depend on
this crate.

The learning work under [research/](../../research/ml-from-scratch/README.md) sits outside
the production workspace. Production code must not import it. Later, learned methods
could help search for options or compare valid ones. Facts, required rules and execution
permissions would still need to be protected by explicit checks.

[Learning path and research questions](research.md): What later methods could learn and what already exists today.

## Search method and current limits

The search space is a single Event placed at horizon.start + k * granularity,
for nonnegative integer k, with a fixed requested duration and end <= horizon.end.
Default granularity in the example is 900 seconds. Every admitted position is checked
against scope, immutable facts and hard constraints before ranking.

Cost is absolute distance in seconds to the preferred start from the strongest preference
source; no preference means zero. Precedence is explicit current request, session context,
personal learned, global learned, default. Equal-source preferences select the earlier time.
Tie-breaks: cost, mutation count, total shift seconds, start, object ID.
Only a fully exhausted grid with a solution receives ProvenOptimal. This proves optimality
only within that grid and objective, never over continuous time or future repair spaces.
Exhausted empty search is Complete/NoSolution. Interrupted search is BestFound; if still
empty, NeedsRelaxation means the budget/declared search must be reconsidered.

All Event/Task times in the supplied context conservatively block overlap regardless of
calendar/resource. Multi-resource semantics and cross-calendar concurrency are deferred.
The baseline searches one target. Lifecycle validation can validate explicit multi-event
placements, but there is no batch search or safe provider ordering for dependent moves.
Move preserves duration. Update/Delete action vocabulary is reserved; CANCEL intent is
not automatically translated into deletion. Recurrence constraints fail closed.

ConflictSet contains actual rejected constraints/facts for the declared grid; it is not
minimal or irreducible. Fixed deterministic budgets and exhaustive small-domain interval
tests avoid uncontrolled timing and randomness.

[CPIR reference](cpir.md) · [Testing](development.md) · [Deutsch](../de/foundation.md)

Core also imports temporal directly for typed diagnostics; see [Temporal Core](temporal.md).
