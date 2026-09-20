> Historical / non-normative review record. Current authority: [Master specification](../../architecture/specifications/master-v0.4.md).

# Historical Review Resolution: Specification 0.2 -> 0.3

**Date:** 2026-09-19  
**Status:** Historical audit record — non-normative after consolidation

## Resolved

- Removed `permission_scope` from HardConstraint vocabulary; permissions remain a separate capability/authorization layer.
- Consolidated canonical lifecycle into Master Specification: `ProposedPlan -> ValidatedPlan -> ActionPlan -> AuthorizedActionPlan -> ExecutionResult`.
- Replaced stale mixed planner status semantics with `PlanningOutcome` + `SearchAssessment`.
- Moved concrete `PlanningScope` dimensions into the central Operations section.
- Moved explicit `KNOWN/MISSING/UNKNOWN/UNCERTAIN/AMBIGUOUS/UNRESOLVED` semantics into the central CPIR section.
- Rebuilt Master Specification chapter numbering in sequential order.
- Declared review-resolution files historical/non-normative after consolidation.
- Added explicit normative-document precedence.
- Required strong Rust lifecycle type boundaries during the foundation implementation.
- Renamed the initial boundary to “Initial software v0.1.0 implementation boundary” to distinguish it from specification revisioning.

## Normative authority

Current architecture is defined by the Master Specification and applicable accepted ADRs/domain standards, not this historical review record.
