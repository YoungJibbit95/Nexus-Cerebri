<!-- doc: cpir; lang: en; counterpart: ../de/cpir.md -->
# CPIR 0.2 and interfaces

CPIR is the data format Cerebri uses to receive a planning task. Duration, occupied
times, rules and permissions each have their own fields. This lets the planner check
what information is available and what is still missing. The full name is
*Cerebri Planning Intermediate Representation*.

[Deutsch](../de/cpir.md) · [Project story and planning example](introduction.md)

## From an appointment request to data

The [repository example](../../examples/request.json) describes this task:

> Find a 30-minute appointment between 09:00 and 12:00 on 1 October 2026.
> An existing appointment occupies 09:00–10:00. No preferred time has been supplied.

All times are UTC. The sentence explains the example; the current planner does not read
free-form text. A calling application must supply the information as CPIR. The example
file contains synthetic data, not appointments retrieved from a connected calendar.

| Question | Request field | Value in this example |
| --- | --- | --- |
| What should be created? | [`operation`](../../crates/cerebri-planner/src/model.rs) | `CREATE`: propose a new appointment |
| For which object? | [`target_ids`](../../crates/cerebri-planner/src/model.rs) | `new-event`, described in `context.objects` |
| How long does it take? | [`duration`](../../crates/cerebri-planner/src/model.rs) | 1800 seconds, or 30 minutes |
| Where should the search take place? | [`scope.time_range`](../../crates/cerebri-planner/src/model.rs) | 09:00–12:00 |
| What already occupies time? | [`context.objects`](../../crates/cerebri-planner/src/model.rs) | The existing object `busy`, 09:00–10:00 |
| Are there additional required rules? | [`constraints`](../../crates/cerebri-planner/src/model.rs) | `[]`: no additional rules supplied |
| Are there preferences? | [`preferences.preferences`](../../crates/cerebri-preferences/src/lib.rs) | `[]`: no preferences supplied |

`new-event` has `revision: null`: it represents a prospective object. `busy` already has
`revision: 1` and a known time. The separate `context.revision` identifies the version
of the complete supplied context. The search produces a proposal for `new-event`;
it does not create a calendar entry.

## Why the new time may be missing but the duration may not

The time for `new-event` still needs to be found. Its `time.value` in the example file
therefore contains exactly this value:

```json
{
  "processing": "RESOLVED",
  "knowledge": {
    "state": "MISSING"
  }
}
```

This is an excerpt, not a complete planning request. `RESOLVED` means processing is
complete. `MISSING` means no time was supplied. This combination is expected for the new
appointment. An existing occupied time, however, must be known so that the planner can
check for overlaps.

The duration must also be established before searching. Here is the complete `duration`
field from the same file:

```json
{
  "value": {
    "processing": "RESOLVED",
    "knowledge": {
      "state": "KNOWN",
      "data": 1800
    }
  },
  "provenance": "USER_EXPLICIT",
  "confidence": null,
  "evidence": []
}
```

`KNOWN` identifies the established value of 1800 seconds. `USER_EXPLICIT` records that
the duration was explicitly supplied. The empty `evidence` array contains no additional
evidence identifiers. `confidence: null` means no separate confidence value was supplied;
it does not turn the known duration into zero or an unknown value.

Other states remain distinct: `UNKNOWN` means a needed value cannot currently be
determined. `AMBIGUOUS` retains multiple possible interpretations. `UNRESOLVED` means
processing is still pending. A missing duration or one in any of these states prevents
the search from starting. An estimated duration marked `UNCERTAIN` may be used only for
analysis with explicit policy permission.

## An empty rule list does not disable checks

Even with `constraints: []`, the planner checks duration, the planning window and
overlaps with known occupied times. The time of `busy` is recorded in the object itself,
so it is still considered when `context.facts` is empty.

The example marks that time's source as `INTEGRATION_FACT`. Here this is a field in
synthetic test data, not evidence of a production calendar connection.

Preferences live separately in `preferences`. This example's list is empty; no preferred
time is filled in or learned. Even with a preference, a candidate must first pass the
required checks. A preferred time cannot make a conflicting appointment acceptable.

## Search steps and search limits are different

`granularity: 900` places possible starts 900 seconds, or 15 minutes, apart. Combined
with the 30-minute duration and the 09:00–12:00 window, this gives 11 possible starts:
09:00, 09:15 and so on through 11:30.

`budget.max_candidates: 256` instead limits how many starts may be checked. It does not
request 256 candidates. The budget is enough for all 11 starts here, including those
later rejected. `max_repairs` and `max_depth` are zero; no broader repair search takes
place. The budget counts work, not elapsed milliseconds.

## A search boundary does not grant permission

`scope` limits where planning may take place. Its filters are `null` here, adding no
restriction by identifier. An empty list `[]` would select nothing. Neither grants
permission.

`policy` describes the rules governing changes; `planning_capability` carries planning
grants for specific objects. The example policy allows `CREATE_EVENT` and `MOVE_EVENT`
in principle. The capability, however, contains only a matching `CREATE_EVENT` grant
for `new-event`. That does not grant permission to move `busy`. The mutation limit of
three in the scope and policy is a ceiling, not a request for three changes. The search
still handles exactly one target appointment here.

These inputs are checked during planning. Execution requires further validation and
permission checks. In particular, a `principal_id` in a request does not authenticate
anyone. The [lifecycle reference](foundation.md) describes the steps to execution.

## From the inputs to the result

For this unchanged file, the Rust planner rejects four overlapping starts and retains
seven. With no preferred time, the fixed comparison rules put 10:00 first. The
[introduction](introduction.md) walks through that result; the
[planner reference](planner-integration.md) describes the exact comparison.

Next, the [architecture walkthrough](foundation.md) shows which part checks these
inputs, computes proposals and returns results to applications.

These sources connect the explanation to the implementation:

- [Complete CPIR request](../../examples/request.json): the runnable input.
- [Rust data model](../../crates/cerebri-planner/src/model.rs): fields and scope filters.
- [Knowledge states](../../crates/cerebri-types/src/lib.rs): `FieldState`, `Knowledge` and provenance.
- [Input validation](../../crates/cerebri-planner/src/validation.rs): required values and planning grants.
- [Candidate validation](../../crates/cerebri-planner/src/lifecycle.rs): includes the mandatory overlap check.
- [Search](../../crates/cerebri-planner/src/search.rs): grid, budget and ordering.

## Schema and further input rules

**Current CPIR:** `0.2`
**Legacy CPIR:** `0.1`

Software version 0.2.0; specification 0.4, CPIR 0.2 and REST v1 are independent.
CPIR is an internal evolving schema, not a stable public v1 protocol.

[The current CPIR 0.2 fixture](../../examples/request.json) is the normal Quick Start.
[The CPIR 0.1 legacy fixture](../../examples/legacy-cpir-0.1.json) exercises compatibility without temporal input.
These current/legacy declarations implement [ADR-0012](../architecture/decisions/ADR-0012-planner-snapshot-compilation.md)
and are checked against Rust schema constants, both fixtures and the README.
Rust structs in cerebri-planner/model.rs are the representation authority.
Top-level objects reject unknown fields, including misspelled scope controls.
Schemas {major: 0, minor: 1} and {major: 0, minor: 2} are accepted. Temporal input requires 0.2; other versions produce UnsupportedSchema.
See [planner integration](planner-integration.md) for the explicit compiler, graph and ordering contracts.

A request includes identity/trace/principal, operation, scope, immutable context snapshot
(objects, facts and optional temporal source state), targets, duration evidence, constraints, preferences, typed policy,
planning capability, granularity and deterministic search budget.
Event, Task, Deadline, Availability and Resource objects are represented. Only Events are
current search/mutation targets. Existing blocking times must be known; prospective Event
time must be Missing. A required placement belongs in ExplicitTime constraints;
a merely preferred start belongs in preferences.

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

