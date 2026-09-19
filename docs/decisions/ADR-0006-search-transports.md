# ADR-0006: Declared grid proof semantics and provisional transports

Date: 2026-09-19
Status: Accepted (Foundation implementation within specification 0.4)

## Context

A useful bootstrap needs observable planning behavior without premature solver or native-binding complexity.

## Decision

Use exhaustive fixed-duration single-event grid traversal with deterministic candidate limits. Ranking minimizes preferred-start distance and the Master's tie-break tuple. Claim ProvenOptimal only after traversing the full grid with a solution; Complete for exhausted no-solution; BestFound otherwise. Keep synchronous domain functions. Axum only parses/delegates; bind the demonstrator to loopback. Use a Node process bridge to the same Rust JSON facade instead of introducing native addon toolchains. Lab consumes typed PlanningResult JSON.

## Alternatives

A heavy solver hides the baseline; wall-clock limits complicate proof/reproducibility; duplicated JavaScript logic would diverge; N-API setup adds packaging work before integration requirements are known.

## Consequences

Proof is relative to a discrete declared grid, not continuous/global scheduling. All context Event/Task times conservatively block overlap. Core input/work limits bound demonstration workloads. Native Electron packaging, worker isolation for public API load, recurrence and batch repair remain later work.

## Migration / Compatibility

REST v1 denotes transport routes, not CPIR stability or a released product. Node transport can later change while retaining the Rust core. No Master rule is superseded.

## References

Master sections 10, 19, 23, 25; Foundation prompt sections 17–20, 32–35.

[Master](../../00_MASTER_SPECIFICATION_v0.4.md) · [Foundation reference](../en/foundation.md)

