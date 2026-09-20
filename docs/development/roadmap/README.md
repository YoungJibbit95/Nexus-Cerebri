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
automatic CPIR recurrence expansion or continuous fuzzing campaign. The Lab renders real
core results; ML/training/dataset panes have no implemented inference or training backend.

Next concrete step (Deterministic Planner target 0.3.0): define explicit recurrence-to-snapshot
compilation and dependency graph/cycle handling, then verify hard-constraint composition,
deterministic candidate ordering and score decomposition on independent golden schedules.
Preserve lifecycle, policy/capability/confirmation and final execution checks.
Advanced ML, repair search, provider integration and autonomous mutation remain later milestones.
The detailed roadmap's January “CPIR v1” is a future target; the implemented schema is 0.1.
