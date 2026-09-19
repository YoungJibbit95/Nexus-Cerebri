# ADR-0005: Execution preflight, ledger claims and content-bound identity

Date: 2026-09-19
Status: Accepted (Foundation implementation within specification 0.4)

## Context

The baseline requires replay/freshness checks, without selecting a durable database or provider API.

## Decision

Execution lives in integrations. ActionLedger atomically claims the plan and every action key before fresh adapter preflight; final checks precede the first external write. Providers must enforce per-object expected revisions atomically and deduplicate. Canonical request/placements are SHA-256 hashed into plan identity; action index extends that identity for idempotency. Stop on first failure, preserve ordered Succeeded/Failed/Skipped outcomes, and retain claims on every failure. Missing durable completion or unknown provider outcome requires reconciliation.

## Alternatives

Random IDs hinder reproducibility; request ID alone conflates distinct candidates; a boolean result loses partial success; silently retrying after unknown side effects risks duplication.

## Consequences

The included adapter/ledger are in-memory test implementations only. They provide no cross-process durability or transaction spanning multiple providers. Retry, recovery and reconciliation belong to future application adapters, never the planner.

## Migration / Compatibility

Native authenticated execution is not exposed through REST or Node. A durable adapter must honor the port contract before production use. No Master rule is superseded.

## References

Master sections 18–21; Foundation prompt sections 21–27.

[Master](../../00_MASTER_SPECIFICATION_v0.4.md) · [Foundation reference](../en/foundation.md)

