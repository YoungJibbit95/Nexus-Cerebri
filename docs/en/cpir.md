<!-- doc: cpir; lang: en; counterpart: ../de/cpir.md -->
# CPIR 0.2 and interfaces

Software version 0.2.0; specification 0.4, CPIR 0.2 and REST v1 are independent.
CPIR is an internal evolving schema, not a stable public v1 protocol.

[The executable synthetic fixture](../../examples/request.json) is the complete request example.
Rust structs in cerebri-planner/model.rs are the representation authority.
Top-level objects reject unknown fields, including misspelled scope controls.
Schemas {major: 0, minor: 1} and {major: 0, minor: 2} are accepted. Temporal input requires 0.2; other versions produce UnsupportedSchema.
See [planner integration](planner-integration.md) for the explicit compiler, graph and ordering contracts.

A request includes identity/trace/principal, operation, scope, immutable context snapshot
(objects, facts and optional temporal source state), targets, duration evidence, constraints, preferences, typed policy,
planning capability, granularity and deterministic search budget.
Event, Task, Deadline, Availability and Resource objects are represented. Only Events are
current search/mutation targets. Existing blocking times must be known; prospective Event
time must be Missing. Explicit requested placement belongs in ExplicitTime constraints.

FieldState separates Unresolved processing from resolved Knowledge: Known, Missing, Unknown,
Uncertain and Ambiguous. Confidence is finite in [0,1] and does not replace knowledge state.
Missing/unknown/ambiguous/unresolved required fields block search. Uncertain duration is allowed
only for analysis with explicit policy permission and is recorded in the candidate explanation.
Existing event times must use a fact provenance, not learned/model inference.

TimeRange uses RFC 3339 UTC instants, validated on deserialization, with start < end.
All intervals are half-open [start,end); touching events do not overlap. Original IANA timezone
is retained on planning objects/ZonedDateTime. Naive local time conversion rejects DST folds/gaps.
Durations are positive integral seconds. Bounded recurrence diagnostics are available separately; CPIR recurrence constraints still fail closed. See [Temporal Core](temporal.md).

Optional scope lists: omitted/null adds no filter and no permission; [] selects nothing;
[A,B] restricts to those IDs. A resource filter requires a nonempty object resource set entirely
inside the filter. Movable IDs apply to existing objects. max_mutations=0 forbids ActionPlan
translation. FindSlot and Analyze are also always non-executable.

Create, Move, FindSlot, Plan and Analyze support the baseline. Update, Cancel, Reschedule and
Optimize are vocabulary only and return UnsupportedOperation. No silent fallback exists.
The planner searches one target; explicit batch proposals go through the same validator.

## Transports

- GET /health: software/schema/status metadata.
- POST /v1/validate: typed ValidationReport.
- POST /v1/plan: typed PlanningResult, including candidates, scores, conflicts and search coverage.
- POST /v1/temporal: typed recurrence/free-busy diagnostic report.
- GET /lab: redirects to the built Svelte Lab at /lab/.
- No execution endpoint.

Malformed transport input produces HTTP 400/422; oversized bodies produce 413.
Domain rejection is a normal typed HTTP 200 result. Body limit: 256 KiB.
Core admission limits: 256 objects, 1024 facts/constraints, 4096 candidate positions,
and a conservative combined work estimate capped at 1,000,000 units.
Defaults permit zero repair/depth expansion. No wall-clock timeout or implicit clock read exists.

The API is a loopback development process without authentication. Before exposing it externally,
a later application must authenticate principals, resolve trusted visibility/context, derive
capabilities server-side and own authorization/confirmation. Client policy/capability fields
in this demonstrator are planning inputs, never execution credentials.

Node: build cerebri-node, then import plan from bindings/node/index.mjs and await plan(request).
The provisional transport spawns the Rust bridge, with no JavaScript planning.
A native N-API/Electron distribution is deferred; a binary override supports application packaging.
Input is capped at 256 KiB, output at 16 MiB. No binary is committed.

[Architecture](foundation.md) · [Deutsch](../de/cpir.md)

