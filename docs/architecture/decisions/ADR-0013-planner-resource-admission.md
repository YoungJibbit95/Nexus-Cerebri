# ADR-0013: Bound planner metadata work and materialized evidence

Date: 2026-09-21
Status: Accepted

## Context

The verification campaign demonstrated three gaps in ADR-0006/ADR-0012 workload
admission: direct Rust requests could contain arbitrarily large metadata; a request
below the HTTP body cap could repeat large evidence vectors during candidate checking
and hashing; and compact recurrence definitions could amplify evidence across occurrences.
Object/constraint counts alone do not account for those costs.

## Decision

Add admission bounds to the existing cardinality/work limits, without raising them:

- A PlanningRequest's compact serde JSON representation must fit in 256 KiB. Count
  bytes through a capped writer without allocating another serialized request. This
  also applies to direct typed Rust requests. HTTP retains its separate 256 KiB raw-body cap.
- Serialized request bytes multiplied by declared `max_candidates` must not exceed
  16 MiB. This conservative metadata-work proxy bounds repeated hashing and cloning;
  it is neither a wall-clock deadline nor an exact heap/output-size guarantee.
- The compiler admits at most 1 MiB of materialized occurrence evidence, counted as
  the sum of canonical evidence ID lengths plus three bytes per ID, multiplied by
  emitted occurrences. Reject before cloning evidence. Exhaustion is atomic.

Existing `InputLimit` errors represent these rejections. They block search and never
produce an optimization proof. Request input memory must still be bounded by the
caller before deserialization; HTTP already does so. Public concurrent server load,
worker isolation and absolute output-memory budgets remain deployment concerns.

## Alternatives

Merely capping each evidence vector leaves multiplicative amplification. Raising
limits or silently truncating evidence would weaken the bounded/fail-closed contract.
A cost-calibrated scheduler or general streaming response protocol is unnecessary here.

## Consequences

Some previously admitted metadata-heavy requests now fail closed. The 4,096 candidate,
256 object, 1,024 constraint, 32 series and 1,024 occurrence maxima remain individually
reachable under compatible combined budgets. No new schema field, dependency, lifecycle
proof or execution permission is introduced.

## Migration / Compatibility

Tightens the admission policy of ADR-0006 and ADR-0012 under Master sections 10 and 21.
CPIR 0.1/0.2 meanings and REST /v1 remain unchanged. Small supported legacy inputs retain
their results. Lower the declared search budget or provide less irrelevant evidence
when a request exceeds the new work bound; partial search remains `BestFound`.

## References

[Verification report](../../development/progress/2026-09-21-planner-verification.md) ·
[ADR-0012](ADR-0012-planner-snapshot-compilation.md) ·
[Master](../specifications/master-v0.4.md)
