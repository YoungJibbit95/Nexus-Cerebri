<!-- doc: safety; lang: en; counterpart: ../de/safety.md -->
# From a proposal to a permitted change

Finding a suitable time answers a planning question. Changing a calendar answers a
different question: may this exact change be made now? Cerebri keeps those steps separate.
The website example ends with a proposal; it does not book an appointment.

## Why a good suggestion is not enough

Suppose the planner suggests 10:00. Before a change is made, another appointment could
occupy that time, a permission could be withdrawn, or a required confirmation could be
missing. The earlier suggestion cannot settle any of those questions by itself.

The current Rust implementation represents the steps with different types. Each later
type can only be obtained through the corresponding checks:

1. **Propose — `ProposedPlan`.** Describe a possible placement. Candidate checks during
   search do not turn this proposal into permission to act.
2. **Validate — `ValidatedPlan`.** Check the whole proposed result against the supplied
   current context, allowed scope and required rules. A changed source revision requires
   renewed validation.
3. **Describe the actions — `ActionPlan`.** Translate the validated plan into explicit
   changes. An analysis-only request cannot take this step.
4. **Authorize — `AuthorizedActionPlan`.** Check the applicable policy, capabilities,
   required confirmation and execution permission for this particular plan.
5. **Execute and record — `ExecutionResult`.** The executor accepts only an authorized
   action plan. Immediately before external changes, it checks freshness, applicable
   permissions, confirmation and protection against repeated execution again. The result
   records what succeeded, failed or was skipped.

Reading a result from JSON does not skip these steps. A serialized proposal cannot be
imported as a validated or authorized plan.

## A check can become outdated

If the source data changes after validation, the earlier result is stale. Cerebri must
validate and authorize again before execution. An executor also checks that the provider's
object revision still matches before changing that object.

Protection against repeated execution prevents a repeated request from simply applying
the same change again. If a provider's response is lost, the outcome may be unknown.
That requires checking what actually happened before deciding how to recover. Automatically
retrying would risk applying a change twice.

## What exists today

The lifecycle types, checks and execution interfaces exist in Rust. The included provider
adapter and execution ledger are **in-memory test implementations**. They do not connect
to a real calendar or preserve execution records across process restarts.

Authenticated calendar providers, durable execution records and recovery adapters remain
future work. REST and the Node bridge expose no execution operation. The Lab lets you
inspect planning and diagnostics; it has no controls for changing a provider's calendar.

These are specific safeguards and implementation limits, not a claim that all security
risks have been solved. A future learned suggestion would have to pass the same boundaries.

## Read the contracts

- [Lifecycle types and validation](../architecture/decisions/ADR-0002-lifecycle.md)
- [Execution checks and failure handling](../architecture/decisions/ADR-0005-execution.md)
- [Current Master specification](../architecture/specifications/master-v0.4.md)
- [Rust lifecycle implementation](../../crates/cerebri-planner/src/lifecycle.rs)
- [Rust execution implementation](../../crates/cerebri-integrations/src/executor.rs)

[Planning explained](planner-integration.md) · [Architecture](foundation.md) ·
[Try the Lab locally](development.md) · [Deutsch](../de/safety.md)
