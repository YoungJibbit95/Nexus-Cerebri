# Changelog

Notable implementation and specification changes are documented separately below.

## [Unreleased] - software target 0.1.0

No software release or tag has been created yet.

### 2026-09-20 — Temporal Core, Cerebri Lab and Repository Structure

#### Added

* Added the next Temporal Core foundation with bounded recurrence and horizon-aware temporal processing.
* Added deterministic recurrence and temporal boundary handling designed around half-open `[start, end)` intervals.
* Added Free/Busy foundations for future scheduling and candidate generation.
* Added additional temporal tests covering recurrence boundaries, timezone behavior, DST transitions, touching intervals and invalid temporal states.
* Added dedicated planning-scope review coverage.
* Added the next Cerebri Lab frontend foundation for developer, research and debugging workflows.
* Added a modern Nexus-inspired Lab design system with rounded surfaces, glass/blur effects, glow accents, responsive layouts and developer-focused information hierarchy.
* Added foundations for future planner visualization, structured trace inspection, semantic inspection, model output and graphical data visualization.
* Added ADRs covering documentation layout, scope and prospective-object identity, bounded temporal diagnostics and the Cerebri Lab frontend architecture.
* Added documentation describing the project's AI-assisted development workflow and the review/testing requirements applied to AI-assisted contributions.
* Added temporal example data for development and documentation purposes.

#### Changed

* Reorganized project documentation into a clearer `docs/` hierarchy while keeping repository entry-point documents at the root.
* Moved architecture decisions into `docs/architecture/decisions/`.
* Moved the Master Specification into `docs/architecture/specifications/`.
* Moved historical review-resolution documents into `docs/archive/reviews/`.
* Moved roadmap, progress and agent-boundary documentation into the appropriate `docs/development/` hierarchy.
* Updated documentation navigation and internal links to reflect the new repository structure.
* Updated repository and documentation checks where required by the new file organization.
* Updated CI and workspace configuration for the expanded Temporal Core and Cerebri Lab foundations.
* Advanced the project according to the current development roadmap while preserving the Foundation architecture boundaries.
* Clarified and documented Foundation review findings concerning planning scope and prospective-object identity.
* Updated API/Core integration required by the new temporal functionality.
* Updated progress and development documentation to reflect the completed work.

#### Architecture

* Preserved the strong planning lifecycle:
  `ProposedPlan -> ValidatedPlan -> ActionPlan -> AuthorizedActionPlan -> ExecutionResult`.
* Preserved the rule that only `AuthorizedActionPlan` may enter execution.
* Preserved the separation between planning scope, capabilities, authorization, hard constraints and learned preferences.
* Preserved deterministic lower layers and the separation between production and research code.
* Preserved the distinction between epistemic knowledge states and unresolved processing state.
* Kept Cerebri Lab as a consumer and visualization surface for Core data rather than a source of planning/domain truth.
* Documented architectural treatment of planning-scope semantics and prospective internal objects.
* Kept temporal recurrence and diagnostics bounded and deterministic at the current architecture stage.

#### Testing

* Verified the Rust workspace test suite successfully.
* Verified unit, integration and temporal test suites included in the workspace test run.
* Verified compile-fail doctests enforcing the execution lifecycle boundary.
* Confirmed that `ProposedPlan`, `ValidatedPlan` and `ActionPlan` remain rejected by the executor at compile time.
* Preserved temporal tests covering recurrence, availability, diagnostics and interval behavior.

#### Known Limitations

* Advanced recurrence behavior beyond the current bounded Temporal Core remains outside the completed milestone where not explicitly implemented.
* Real calendar/provider integrations remain intentionally unimplemented.
* Advanced repair search, learned ranking, preference learning and neural planning remain future milestones.
* Cerebri Lab remains a developer/research interface and is not a production end-user calendar application.
* The current Node integration remains provisional unless explicitly changed by a future milestone.
* Remote GitHub Actions verification is tracked separately from the successful local Rust workspace test run.

#### Development

* Continued development using an AI-assisted workflow with human-defined architecture, documented decisions, automated tests and repository-level verification requirements.
* AI-assisted changes remain subject to the same architecture, safety, testing, documentation and review requirements as other contributions.

### 2026-09-19 — Foundation Bootstrap

#### Added

* Added the Rust workspace with nine domain crates, development REST application and Node process bridge.
* Added validated IDs and time ranges, CPIR schema 0.1, knowledge states and bounded planning scope.
* Added deterministic grid planning, structured conflicts and restricted optimality claims.
* Added distinct lifecycle types, authorization, freshness, replay protection and partial-result mock contracts.
* Added storage and inference ports, synthetic fixtures, internal Lab foundation, CI and documentation build.
* Added paired DE/EN Foundation references, six initial ADRs and dated progress documentation.

#### Fixed

* Fixed the stale README 0.3 review reference and Master execution invariant.
* Synchronized the Master Specification section 9 preference summary with section 2 and the bootstrap requirements.

#### Deferred

* Deferred production providers, production authentication, durable execution, recurrence, advanced planning and ML beyond the initial Foundation Bootstrap.
* No software release, tag or deployment was created by the Foundation Bootstrap.

---

## [Specification 0.4] - 2026-09-19

### Changed

* Frozen Specification 0.4 as the Foundation Architecture Baseline.
* Synchronized the lifecycle diagram and `Execution => AuthorizedActionPlan` invariant.
* Defined CPIR `PolicyContext` and `PolicySnapshot` semantics.
* Defined `PlanningScope` omitted/empty/list semantics and `max_mutations = 0`.
* Defined `SearchAssessment::COMPLETE`.
* Simplified `PRESENT_KNOWN` to `KNOWN` and made `UNRESOLVED` a processing state.
* Defined per-action `ExecutionResult` semantics and final executor precondition checks.
* Added the Master Specification archive policy.

## [Specification 0.3] - 2026-09-19

### Changed

* Fully consolidated Specification 0.2 review corrections into the Master Specification.
* Removed `permission_scope` from hard constraints.
* Canonicalized lifecycle, planning result semantics, `PlanningScope` and knowledge states.
* Added normative document precedence and strong Foundation lifecycle typing.
* Corrected Master Specification numbering.
* Marked review-resolution documents as historical and non-normative.

## [Specification 0.2] - 2026-09-19

### Changed

* Resolved independent architecture review findings.
* Formalized plan lifecycle, permissions, version authorities, scope, knowledge states and planner result semantics.
* Clarified research/production, execution-state and documentation-governance boundaries.

## [Specification 0.1] - 2026-09-19

### Added

* Added the initial consolidated planning baseline.
