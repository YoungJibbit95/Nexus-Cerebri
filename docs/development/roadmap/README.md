# Nexus Cerebri Roadmap

[Detailed development and learning roadmap](development-learning-roadmap.md).

The roadmap is target-based, not calendar-driven. Candidate software versions ship only when their acceptance criteria and Definition of Done are satisfied.

## Current status — 2026-09-26

Workspace/software version: **0.2.0**. **Unreleased; undergoing release qualification.**
Specification remains **0.4**. Current CPIR remains **0.2** with **0.1** retained as the
legacy compatibility schema. **Published release: none.** No version bump, tag or release is
created by this governance/documentation closure.

Ranking Feature Contract v0.1 / deterministic Intelligence Slice 1 is implemented and has
completed independent Math and Security qualification. Repository hardening is complete:
the retained Traffic Stats action is immutable-SHA pinned, workflow permissions are
least-privilege, `AGENTS.md` governance is hardened, and qualification runs as independent CI
gates. The hardening merge's CI gates and Documentation Pages were observed green.

Branch protection is still not configured as of 2026-09-26: GitHub reports `main` as
`protected: false` with no active repository rulesets. The connected GitHub integration cannot
read or write branch-protection configuration (`403 Resource not accessible by integration`),
so protection remains a manual maintainer task.

The next Intelligence step is bounded architecture/contract work for Slice 2. Slice 2, ML,
learned ranking/search and EvaluationEpisode implementation are not part of the current state
and must not be inferred from the completed deterministic Slice 1.
[The current Changelog header](../../../CHANGELOG.md) remains the release-status authority;
the dated entries below preserve earlier milestone context.

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
The [2026-09-21 verification campaign](../progress/2026-09-21-planner-verification.md) adds
512 deterministic schedule seeds, exhaustive three-node graph checks, generated recurrence,
malformed-input corpus/byte mutations and API/Lab parity. It fixes local-date overflow,
metadata amplification and incomplete presentation guards. ADR-0013 tightens resource admission.
This is bounded testing, not formal verification or a continuous fuzzing service.
Next bounded milestone: independent review and release qualification of the verified planner,
including the tightened admission contract and release artifacts. No automatic 0.3.0 release.
Do not begin ML, provider integration, advanced repair search or autonomous mutation.
The detailed roadmap's CPIR v1 remains a future target.
