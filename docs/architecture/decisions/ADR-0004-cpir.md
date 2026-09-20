# ADR-0004: CPIR evidence, scope and serialization

Date: 2026-09-19
Status: Accepted (Foundation implementation within specification 0.4)

## Context

Serde needs concrete representations for schema authority, uncertain values and optional scope filters.

## Decision

Use an explicit {major,minor} schema version fixed to 0.1. CPIR input structs use serde and reject unknown fields at bounded struct boundaries. Knowledge is a tagged epistemic enum, nested inside FieldState's separate Resolved/Unresolved processing state. Scope dimensions use Option<Vec<ID>> to preserve omission/null versus empty. Facts, constraints, inference, preferences and policy are distinct types. Existing times require fact provenance; uncertain duration is accepted only by explicitly enabled analysis.

## Alternatives

Option<T> loses knowledge distinctions; untyped policy JSON hides invariants; accepting arbitrary schema versions misrepresents compatibility.

## Consequences

No public stable v1 CPIR promise. Unsupported operations return structured issues. New required data fields are introduced deliberately through schema changes. The five preference sources follow Master section 2 and the explicit bootstrap instruction; section 9 is synchronized to that same order.

## Migration / Compatibility

No migration for historic JSON exists. Unknown schema versions are rejected. The section 9 edit removes a stale conflicting preference summary; it does not add a new source or precedence.

## References

Master sections 2, 4, 9, 11, 25; Foundation prompt sections 7, 10–13, 28.

[Master](../specifications/master-v0.4.md) · [Foundation reference](../../en/foundation.md)

