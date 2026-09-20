# ADR-0008: ALL resource scope and prospective object identity
Date: 2026-09-20
Status: Accepted; clarifies Master PlanningScope collection semantics.

## Decision
For a nonempty resource filter an object is selected only if its resource list is nonempty
and **all** its resource IDs occur in the filter. Preserve existing behavior: this is
containment of the object's complete resource footprint, not any-overlap discovery.
Omitted/null adds no resource restriction; [] selects nothing. Other dimensions intersect.
Selection never grants read, plan or execute capability.

| Object resources | Filter | Selected |
| --- | --- | --- |
| A, B | omitted/null | yes |
| A, B | [] | no |
| A | A | yes |
| A, B | A | no |
| A, B | A, B, C | yes |
| A, B | C | no |
| none | A | no |

ANY would include objects partly outside the resource boundary; a future discovery query may
add explicit ANY semantics through a new decision, not silently alter PlanningScope.

## Prospective versus existing
PlanningObject.id is a planning identity. revision=None means a prospective target in
the planning context; Some(revision) represents already-existing external state.
CREATE requires a prospective target. MOVE/UPDATE/DELETE require existing state.
The mock stores both together for testability. A real provider CREATE must not require
that the prospective target already exists remotely; it returns external identity/revision.
No premature ExternalEvent refactor or provider ID schema is introduced.
Atomic revision checks, idempotency replay and executor authorization remain required.

## Verification and compatibility
Nine scope/wire tests cover the table, all optional filter dimensions and absent mutation grants.
Integration tests cover promotion after CREATE and rejection of action/revision mismatches.
The adapter adds a guard; valid CPIR 0.1 behavior is unchanged.
[Master](../specifications/master-v0.4.md) · [Review resolution](../../development/reviews/foundation-review-2026-09-20.md)
