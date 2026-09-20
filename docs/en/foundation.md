<!-- doc: foundation; lang: en; counterpart: ../de/foundation.md -->
# Foundation architecture

## Simple

The planner suggests a time. A validator checks the suggestion. A separate application
boundary checks current permission and confirmation before a mock adapter can change its
in-memory state. Planning itself has no external side effects.

## Technical

```text
API / Node -> core -> planner -> constraints / semantics / preferences
                        |              |           |           |
                        +--------------+-----------+-----------+-> temporal / types
integrations -> planner (lifecycle proofs), constraints (facts), temporal, types
ml -> temporal / types
research -> no production dependency
```

Arrows mean imports, not runtime calls. The core never imports integrations or ML.
The dependency checker enforces an explicit crate allowlist; Cargo rejects cycles.

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

## Research depth and limitations

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
