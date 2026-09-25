# Architecture decision index

Read the [current Master](../specifications/master-v0.4.md) and all Accepted decisions relevant
to each subsystem being changed. Read later amendments together with the earlier decisions they
extend. This index aids discovery; the decision files contain the precise scope and consequences.
Historical release statements retain their original date and do not set current publication status.

| ADR | Title | Status | Subsystem / topic |
| --- | --- | --- | --- |
| [ADR-0001](ADR-0001-workspace.md) | Workspace dependencies and application ownership | Accepted | Workspace layers, dependencies and application ownership |
| [ADR-0002](ADR-0002-lifecycle.md) | Controlled lifecycle proofs and batch validation | Accepted | Lifecycle proofs and batch validation |
| [ADR-0003](ADR-0003-temporal.md) | Chrono, IANA zones and validated half-open intervals | Accepted | Instants, IANA zones and intervals; extended by ADR-0009 |
| [ADR-0004](ADR-0004-cpir.md) | CPIR evidence, scope and serialization | Accepted | CPIR evidence, scope and serialization; schema 0.2 amendment in ADR-0012 |
| [ADR-0005](ADR-0005-execution.md) | Execution preflight, ledger claims and content-bound identity | Accepted | Execution preflight, idempotency and content-bound identity |
| [ADR-0006](ADR-0006-search-transports.md) | Declared grid proof semantics and provisional transports | Accepted | Search assessment and transport boundaries; Lab implementation in ADR-0010 |
| [ADR-0007](ADR-0007-documentation-layout.md) | Canonical documentation under docs/ | Accepted | Documentation locations and historical archives |
| [ADR-0008](ADR-0008-scope-and-prospective-identity.md) | ALL resource scope and prospective object identity | Accepted | Scope semantics and prospective identity |
| [ADR-0009](ADR-0009-bounded-temporal-diagnostics.md) | Bounded recurrence and completeness-aware diagnostics | Accepted | Bounded recurrence, coverage and temporal diagnostics |
| [ADR-0010](ADR-0010-lab-svelte.md) | Svelte Lab as a typed development client | Accepted | Typed Svelte Lab development client |
| [ADR-0011](ADR-0011-official-site.md) | Repository-backed SvelteKit official site | Accepted | Official static SvelteKit site and build-time repository authority |
| [ADR-0012](ADR-0012-planner-snapshot-compilation.md) | Bounded planner snapshot compilation and dependency evidence | Accepted | CPIR 0.2, snapshot compilation, occurrences and dependency graphs |
| [ADR-0013](ADR-0013-planner-resource-admission.md) | Bound planner metadata work and materialized evidence | Accepted | Planner admission limits and materialized evidence budgets |
| [ADR-0014](ADR-0014-ranking-feature-contract.md) | Versioned deterministic ranking observations | Accepted | Ranking feature schema 0.1, required-nullable output and provenance; extends ADR-0012 |

[Decision template](ADR-TEMPLATE.md) · [Documentation authority](../../README.md)
