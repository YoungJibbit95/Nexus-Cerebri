# ADR-0001: Workspace dependencies and application ownership

Date: 2026-09-19
Status: Accepted (Foundation implementation within specification 0.4)

## Context

The baseline names crates but leaves concrete ownership and build choices open.

## Decision

Use Rust 2024 with a pinned 1.97.0 toolchain and workspace version 0.1.0 (unreleased). Put CPIR/lifecycle in planner, execution and ledger ports in integrations, and a thin synchronous facade in core. Types/temporal remain independent of higher layers. API and Node import core. Metadata-only ml does not participate in planning. Enforce a dependency allowlist in scripts/check-repository.mjs.

## Alternatives

One large crate would hide boundaries; putting all domain types in types would create a god module; placing execution inside core would widen its trusted surface.

## Consequences

More crates, but dependency direction is inspectable. Ports are synchronous until a real I/O adapter justifies async. Research is outside workspace membership.

## Migration / Compatibility

No stable bootstrap API compatibility is promised. No Master rule is superseded.

## References

Master sections 17–19, 31; Agent Implementation Boundary.

[Master](../specifications/master-v0.4.md) · [Foundation reference](../../en/foundation.md)


## 2026-09-20 follow-up

The initial 0.1.0 Foundation was unreleased. The completed Temporal milestone uses workspace 0.2.0; the pinned Rust toolchain is unchanged.
