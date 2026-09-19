# ADR-0002: Controlled lifecycle proofs and batch validation

Date: 2026-09-19
Status: Accepted (Foundation implementation within specification 0.4)

## Context

A runtime status enum and freely deserializable proof types would permit accidental lifecycle jumps.

## Decision

ProposedPlan is public/untrusted and binds a private copy of its request. validate consumes it and checks the complete supplied current snapshot plus resulting-world constraints. ValidatedPlan, ActionPlan and AuthorizedActionPlan have private fields and no Deserialize. Translation consumes validation; authorization consumes actions. execute accepts AuthorizedActionPlan by value. Test earlier stages with compile-fail doctests. Explicit batch placements use this validator; automatic search remains single-event.

## Alternatives

One Plan with booleans is weaker; a generic type-state framework is unnecessarily complex for this learning foundation.

## Consequences

Serialized proposals cannot be deserialized as proofs. External applications must reconstruct and revalidate. Analysis-only requests cannot translate to actions. Dependent move ordering is deferred.

## Migration / Compatibility

No stable wire format exists for lifecycle proofs. No Master rule is superseded.

## References

Master sections 3, 20 and Foundation typing requirement.

[Master](../../00_MASTER_SPECIFICATION_v0.4.md) · [Foundation reference](../en/foundation.md)

