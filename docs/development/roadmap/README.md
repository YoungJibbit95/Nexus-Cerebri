# Nexus Cerebri Roadmap

[Detailed development and learning roadmap](development-learning-roadmap.md).

The roadmap is target-based, not calendar-driven. Candidate software versions ship only when their acceptance criteria and Definition of Done are satisfied.

## 2026-09-20 status

Foundation (internal 0.1.0) and the bounded Temporal Core milestone are implemented.
The first public release target is 0.2.0: daily/weekly recurrence, explicit gap/fold policies,
interval algebra, free/busy completeness, buffers/travel, typed REST diagnostics and Svelte Lab.
The Foundation had no prior tag; 0.2.0 reflects completed scope, not elapsed calendar time.
Release publication requires the exact pushed commit's CI success; see the dated progress log.

Scope boundaries: no general RRULE parser, monthly/yearly/exception rules, provider data,
prospective recurrence planning or continuous fuzzing campaign. The Lab renders real
core results; ML/training/dataset panes have no implemented inference or training backend.

## 2026-09-21 integration status

The bounded deterministic planner integration is implemented: explicit recurrence-to-snapshot
compilation, dependency graph/cycle diagnostics, composed hard constraints, explicit ordering
keys, independent golden/oracle tests and real API-to-Lab scenarios. CPIR 0.2 adds temporal
source state; legacy 0.1 without temporal input remains supported. See
[ADR-0012](../../architecture/decisions/ADR-0012-planner-snapshot-compilation.md).

Software remains unreleased 0.2.0; this session does not claim the full 0.3.0 release gate.
The roadmap's broader randomized-schedule and malformed-input fuzzing evidence remains open.
Next bounded milestone: deterministic planner release validation with randomized schedules
and malformed-input fuzzing. Do not begin ML, provider integration, advanced repair search
or autonomous mutation. The detailed roadmap's CPIR v1 remains a future target.
