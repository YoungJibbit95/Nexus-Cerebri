# ADR-0012: Bounded planner snapshot compilation and dependency evidence

Date: 2026-09-20
Status: Accepted

## Context

ADR-0009 deliberately separates diagnostic recurrence from planning. The planner needs
an explicit immutable view of temporal occupancy, without inventing provider records.
The dependency constraint exists but lacks graph validation and cycle evidence.

## Decision

Extend CPIR with schema `0.2`; continue accepting `0.1` without temporal input. The optional
`ContextSnapshot.temporal` contains an explicit horizon (equal to scope), coverage,
shared expansion limits and identified series. Each series has a rule, provenance,
evidence and source state (`Existing { revision }` or `Prospective`). Only existing,
fact-sourced series can compile into occupancy. Prospective series reject explicitly;
neither diagnostic `/v1/temporal` nor the unsupported `RecurrenceRule` hard constraint changes.

`compile_snapshot` independently produces a bounded `CompiledContextSnapshot` view from
the source snapshot. It retains series definitions and materializes distinct occurrences,
not PlanningObjects or provider records. Identity is SHA-256 of the series identity and
nominal local date/time/timezone; it is independent of horizon, clipping and source revision.
Full occurrence ranges and separately clipped visible ranges are retained. DST resolution
and skipped nominal dates come from ADR-0009. Duplicate series IDs and collisions with
ordinary object IDs reject the whole compilation. A caller must use one series identity
per external series, not resupply its instances as independent objects.

Verification clarification (2026-09-21): Earlier/Later fold selection intentionally retains
the same occurrence ID for the same nominal local occurrence. The UTC range and explicit
resolution evidence differ. This follows the identity tuple above; no hash change is made.

Limits: 32 series, at most 1,024 compiled occurrences, 36,600 examined dates shared across
series; existing planner workload limits also include occurrences. Exhaustion rejects
atomically. All supplied Event/Task objects and existing occurrences conservatively occupy
time regardless of mutation scope. The compiled complement is free only with explicit
Complete coverage; Incomplete yields unknown and blocks planning. CPIR 0.1 without temporal
input retains the legacy closed snapshot contract; it cannot make a new Free/Busy claim.
Required buffers check full occurrence ranges. A candidate's required buffer must also
fit inside the certified compilation horizon; outside coverage cannot prove separation
from an occurrence beyond the horizon. Series-level buffers/exceptions are deferred.

DependencyOrder edges mean predecessor.end <= dependent.start. Build a deduplicated, sorted
graph over at most 256 snapshot object IDs and 1,024 edges. Missing IDs reject with explicit
edge evidence. Iterative bounded traversal reports each cyclic strongly connected component
and its internal edges, including self-cycles. Stable Kahn ordering chooses the smallest
ready ID. Cyclic/missing graphs block planning; dependencies grant no scope or permission.
Search still places one event against fixed predecessors; joint search/repair is deferred.

Ranking remains the lexicographic tuple (preferred-start distance seconds, mutations,
shift seconds, start, object ID), now exposed in a typed ordering key. Hard constraints
are never penalties. Proof remains relative to the fully enumerated bounded grid (ADR-0006).

## Alternatives and consequences

Inventing existing PlanningObjects from occurrences confuses external identity and mutation
authority. A workflow engine or recurrence-aware search would exceed this milestone.
The compiler is a pure planner-layer boundary consuming the temporal crate; it changes no
lower-layer dependency. Lifecycle validation recompiles the same source snapshot and checks
all constraints again before granting ValidatedPlan. Source changes invalidate freshness.

## Migration / Compatibility

Amends ADR-0004's fixed CPIR 0.1 rule and extends Master sections 4, 6, 8 and 10.
REST `/v1` is unchanged. Compilation and graph reports plus ordering keys are additive
internal response fields with Rust/TypeScript parity tests. Software stays unreleased
0.2.0 while this bounded integration increment is verified; full 0.3 release scope is
evaluated separately. No release is implied. Generate schemas only when multiple maintained
clients or recurrent drift make explicit cross-language fixtures insufficient.

## References

[Master](../specifications/master-v0.4.md) · [ADR-0008](ADR-0008-scope-and-prospective-identity.md)
· [ADR-0009](ADR-0009-bounded-temporal-diagnostics.md) · [ADR-0010](ADR-0010-lab-svelte.md)
