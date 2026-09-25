<div align="center">

# 📜 Changelog

**Nexus Cerebri · Software & Specification History**

<br>

![Software Target](https://img.shields.io/badge/software_target-0.2.0-0969da?style=flat-square)
![Specification](https://img.shields.io/badge/specification-0.4-8250df?style=flat-square)
![Status](https://img.shields.io/badge/status-unreleased-d29922?style=flat-square)

</div>

---

> [!NOTE]
> Notable implementation and specification changes are documented separately below.

## 🧭 Changelog Structure

This changelog maintains two independent histories:

| Track             | Purpose                                                                      |
| :---------------- | :--------------------------------------------------------------------------- |
| **Software**      | Implementation, runtime, testing, tooling, UI and repository changes         |
| **Specification** | Architecture and specification revisions defining normative project behavior |

> [!IMPORTANT]
> Software versions and specification revisions are separate authorities and must not be interpreted as the same version sequence.

---

# 🚧 [Unreleased]

**Software target:** `0.2.0`
**Next roadmap target:** `0.3.0`
**Release status:** `Unreleased`
**Release qualification:** `Undergoing release qualification`
**Published release:** `None`

### 2026-09-25 — Ranking observations (Slice 1)

- Add `RankingFeatureSet` schema 0.1 to candidate output with required-nullable
  preferred-start distance/source, contextual mutation count and legacy shift seconds.
- Validate missing vs null, paired nullability and schema compatibility at the Rust wire
  boundary. Preserve whole-second quantization and the complete deterministic ordering key.
- Carry core-produced features through REST, Node and Lab; add presence, subsecond,
  generated-mode/permutation and full scenario-order regression coverage.
- Document [ADR-0014](docs/architecture/decisions/ADR-0014-ranking-feature-contract.md)
  and synchronized Master/DE/EN references. No CPIR/software/API version bump or release.
- Independent implementation review remains required; see the
  [verification record](docs/development/progress/2026-09-25-ranking-feature-contract.md)
  for actual gates and any baseline/environment blockers.

> [!WARNING]
> No software release or tag has been created yet.

---

## 2026-09-22 — Release qualification corrections

### Fixed

* Separated workspace 0.2.0 from unreleased qualification and publication status in current documentation and generated website metadata. No formal RC, tag, release or version bump.
* Replaced loose README version presence checks with exact authority fields, badge/footer checks and targeted stale/contradictory-metadata mutations; site generation shares these checks.
* Made CPIR 0.2 the normal Quick Start, preserved the exact former CPIR 0.1 example as a named legacy fixture, and strengthened Core/API/Node/Lab compatibility and unknown-version rejection coverage.

### Security and CI

* Added pinned cargo-audit 0.22.2 lockfile scanning; known vulnerabilities, warnings and scanner failures block CI without ignored advisories. Added the explicit locked workspace build.
* Pinned all direct CI/Pages actions to upstream commit SHAs, retained exact tag comments, added weekly Dependabot updates, removed persisted checkout credentials and scoped Pages write permissions to deployment.
* Preserved short-lived synthetic browser failure evidence for review. Kept all existing deterministic planner, frontend, documentation and npm audit gates.

### Documentation and governance

* Added the Accepted ADR index and subsystem-reading rule; clarified the Master's original baseline date without inventing revision dates.
* Restored exact historical specification 0.1/0.2/0.3 blobs with provenance and non-normative labeling; preserved accurate historical ADR/progress wording.
* Removed the external view counter and documented the retained informational badge policy. Documented advisory policy and the existing low-severity site dependency finding without suppressing it.
* [Qualification evidence and limitations](docs/development/progress/2026-09-22-release-qualification-corrections.md). Independent review and a later explicit release decision remain required.

## 2026-09-21 — Deterministic planner verification and hardening

### Added

* Deterministic SplitMix64 test generator without new dependencies: 512 independent schedule seeds with integer feasibility/ranking oracles and three full-response permutations each; 256 recurrence seeds with day-by-day expectations; all 512 directed three-node graphs and 256 additional small graph seeds checked with transitive-closure and exhaustive-order oracles.
* Named malformed CPIR corpus, 128 reproducible byte-mutation cases, explicit CPIR 0.1/0.2/future-version round trips and API/Core parity. Added arithmetic, maximum-workload, DST identity/clipping and shared-budget regressions.

### Fixed / Security

* Reuse the validated immutable compilation during planning instead of compiling twice and discarding the second error. Lifecycle revalidation remains independent.
* Reject unrepresentable local dates through checked timezone-offset arithmetic; extreme valid UTC JSON input no longer panics in ExplicitDate constraints.
* Bound serialized typed input, repeated request metadata work and materialized occurrence evidence (ADR-0013). Existing count/work limits remain intact; rejection is atomic.
* Lab now rejects malformed validation issue payloads and domain identifiers before rendering. Real Rust response mutations demonstrate the fixes.

### Documentation / compatibility

* Confirmed ADR-0012 nominal occurrence identity: Earlier/Later share an ID while UTC ranges and resolution evidence differ. CPIR 0.1 remains distinct from temporal CPIR 0.2; unknown coverage still blocks planning.
* Updated Master, roadmap, DE/EN references and the [dated verification report](docs/development/progress/2026-09-21-planner-verification.md), including limits and remaining blind spots.
* No new dependency, schema generation, UI redesign, feature expansion, software bump, tag or release. Independent review and release qualification remain the next bounded milestone.

## 2026-09-21 — Deterministic planner integration

### Added

* Added CPIR 0.2 temporal source input and an independently testable bounded snapshot compiler; CPIR 0.1 remains accepted without temporal input.
* Materialized existing recurrence occupancy with stable occurrence identity, full and clipped ranges, source revision, provenance, evidence and explicit DST outcomes. Duplicate series, unsafe provenance, prospective series, horizon mismatch and exhausted budgets fail closed.
* Added deterministic dependency graphs with stable topological order, missing-reference diagnostics and iterative cycle-component evidence. Graphs grant no scope, policy, capability or authorization.
* Exposed the complete lexicographic candidate ordering key; preserved single-event grid search and truthful exhaustive/budget assessments.
* Added real Lab compiler, availability, dependency and score inspection plus 11 source scenarios exercised through Core, REST and TypeScript contract guards.
* Added combined hard-constraint, independent integer-oracle, golden schedule and negative contract tests.

### Architecture and compatibility

* Accepted ADR-0012 and synchronized the Master and DE/EN CPIR/planner references. ADR-0011 remains the official-site decision.
* Kept occurrences separate from mutable planning/provider objects. Lifecycle validation rechecks the same source snapshot. Recurrence constraints and prospective series remain unsupported.
* Incomplete temporal coverage blocks planning; required buffers must fit within the compiled horizon. Series reconciliation, exceptions, joint search and repair remain deferred.
* Software remains unreleased 0.2.0, specification 0.4 and REST /v1; no release or tag is created by this session.

### Verification

* Exact commands, results and the interruption recovery are recorded in [the session log](docs/development/progress/2026-09-20-planner-integration.md).

---

## 📅 2026-09-20 — Official Nexus Cerebri Website

### ✨ Added

* Added `apps/cerebri-site`, a static SvelteKit/Svelte 5/TypeScript official website.
* Added public knowledge surfaces for Explore, CPIR, deterministic planning, time, safety, architecture, Lab, roadmap, developers and canonical repository documentation.
* Added global Understand / Technical / Research explanation depth.
* Added explicit REAL / EDUCATIONAL / FUTURE CONCEPT labels and text alternatives for significant visualizations.
* Added build-time website authority data derived from Cargo, the current Master, CPIR fixtures and repository Markdown.
* Added Rust-generated planner and temporal website fixtures through `cerebri-core`.
* Added Playwright navigation, accessibility, responsive-overflow and visual-regression coverage.

### 🔄 Changed

* Replaced the bespoke static documentation portal with the repository-backed SvelteKit site while keeping `site/` as the Pages artifact and `npm run docs:build` as the canonical build command.
* Updated GitHub Pages and CI to build the official site with the `/Nexus-Cerebri` base path.
* Extended repository checks for site version authority, route inventory, static-host configuration, retired portal files and runtime font imports.
* Kept canonical Markdown as documentation truth and Cerebri Lab as a separate developer/research client.

### 🏛️ Architecture

* Added ADR-0011 for the official-site frontend strategy.
* Preserved Rust as planner/temporal authority; TypeScript contains no duplicated planning or recurrence implementation.
* Preserved Planner ≠ Executor and does not expose or simulate provider execution, production authentication, durable ledger behavior, preference learning or neural inference.

### 🧪 Testing

* Added Axe serious/critical accessibility checks for representative public routes.
* Added route/prerender checks plus horizontal-overflow coverage at 1440, 1024, 768 and 390 px.
* Added visual-regression capture at the same four widths.

### 🛠️ Development

* Retired `scripts/build-docs.mjs`, `scripts/docs-theme.css` and `scripts/docs-ui.js`.
* Added an official-site README and dated implementation/progress record.
* Verified the final site stack on GitHub Actions push run `35520398395` and pull-request run `35520401015`; both completed successfully.
* Committed the official-site lockfile, switched CI/Pages to reproducible `npm ci`, updated GitHub Actions dependencies, and removed the temporary lock bootstrap workflow.
* Locked deterministic visual-regression hashes at 1440, 1024, 768 and 390 px after successful browser/Axe verification.

---

## 📅 2026-09-20 — Temporal Core, Cerebri Lab and Repository Structure

### Version authority reconciliation

* Synchronized the current software target with Cargo workspace and Lab version `0.2.0`.
* `0.1.0` remains the historical unreleased Foundation target; specification `0.4`,
  CPIR `0.1` and REST `/v1` are independent authorities. No release was created by this correction.

### ✨ Added

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

### 🔄 Changed

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

### 🏛️ Architecture

* Preserved the strong planning lifecycle:
  `ProposedPlan -> ValidatedPlan -> ActionPlan -> AuthorizedActionPlan -> ExecutionResult`.
* Preserved the rule that only `AuthorizedActionPlan` may enter execution.
* Preserved the separation between planning scope, capabilities, authorization, hard constraints and learned preferences.
* Preserved deterministic lower layers and the separation between production and research code.
* Preserved the distinction between epistemic knowledge states and unresolved processing state.
* Kept Cerebri Lab as a consumer and visualization surface for Core data rather than a source of planning/domain truth.
* Documented architectural treatment of planning-scope semantics and prospective internal objects.
* Kept temporal recurrence and diagnostics bounded and deterministic at the current architecture stage.

### 🧪 Testing

* Verified the Rust workspace test suite successfully.
* Verified unit, integration and temporal test suites included in the workspace test run.
* Verified compile-fail doctests enforcing the execution lifecycle boundary.
* Confirmed that `ProposedPlan`, `ValidatedPlan` and `ActionPlan` remain rejected by the executor at compile time.
* Preserved temporal tests covering recurrence, availability, diagnostics and interval behavior.

### ⚠️ Known Limitations

* Advanced recurrence behavior beyond the current bounded Temporal Core remains outside the completed milestone where not explicitly implemented.
* Real calendar/provider integrations remain intentionally unimplemented.
* Advanced repair search, learned ranking, preference learning and neural planning remain future milestones.
* Cerebri Lab remains a developer/research interface and is not a production end-user calendar application.
* The current Node integration remains provisional unless explicitly changed by a future milestone.
* Remote GitHub Actions verification is tracked separately from the successful local Rust workspace test run.

### 🛠️ Development

* Continued development using an AI-assisted workflow with human-defined architecture, documented decisions, automated tests and repository-level verification requirements.
* AI-assisted changes remain subject to the same architecture, safety, testing, documentation and review requirements as other contributions.

---

<details>
<summary><strong>📦 2026-09-19 — Foundation Bootstrap</strong></summary>

<br>

### ✨ Added

* Added the Rust workspace with nine domain crates, development REST application and Node process bridge.
* Added validated IDs and time ranges, CPIR schema 0.1, knowledge states and bounded planning scope.
* Added deterministic grid planning, structured conflicts and restricted optimality claims.
* Added distinct lifecycle types, authorization, freshness, replay protection and partial-result mock contracts.
* Added storage and inference ports, synthetic fixtures, internal Lab foundation, CI and documentation build.
* Added paired DE/EN Foundation references, six initial ADRs and dated progress documentation.

### 🐛 Fixed

* Fixed the stale README 0.3 review reference and Master execution invariant.
* Synchronized the Master Specification section 9 preference summary with section 2 and the bootstrap requirements.

### ⏳ Deferred

* Deferred production providers, production authentication, durable execution, recurrence, advanced planning and ML beyond the initial Foundation Bootstrap.
* No software release, tag or deployment was created by the Foundation Bootstrap.

</details>

---

# 📐 Specification History

> Architecture and specification revisions are tracked independently from software releases.

## 🟣 [Specification 0.4] — 2026-09-19

![Specification](https://img.shields.io/badge/specification-0.4-8250df?style=flat-square)
![Baseline](https://img.shields.io/badge/Foundation-Architecture_Baseline-1f883d?style=flat-square)

### 🔄 Changed

* Frozen Specification 0.4 as the Foundation Architecture Baseline.
* Synchronized the lifecycle diagram and `Execution => AuthorizedActionPlan` invariant.
* Defined CPIR `PolicyContext` and `PolicySnapshot` semantics.
* Defined `PlanningScope` omitted/empty/list semantics and `max_mutations = 0`.
* Defined `SearchAssessment::COMPLETE`.
* Simplified `PRESENT_KNOWN` to `KNOWN` and made `UNRESOLVED` a processing state.
* Defined per-action `ExecutionResult` semantics and final executor precondition checks.
* Added the Master Specification archive policy.

---

<details>
<summary><strong>📐 Specification 0.3 — 2026-09-19</strong></summary>

<br>

### 🔄 Changed

* Fully consolidated Specification 0.2 review corrections into the Master Specification.
* Removed `permission_scope` from hard constraints.
* Canonicalized lifecycle, planning result semantics, `PlanningScope` and knowledge states.
* Added normative document precedence and strong Foundation lifecycle typing.
* Corrected Master Specification numbering.
* Marked review-resolution documents as historical and non-normative.

</details>

<details>
<summary><strong>📐 Specification 0.2 — 2026-09-19</strong></summary>

<br>

### 🔄 Changed

* Resolved independent architecture review findings.
* Formalized plan lifecycle, permissions, version authorities, scope, knowledge states and planner result semantics.
* Clarified research/production, execution-state and documentation-governance boundaries.

</details>

<details>
<summary><strong>📐 Specification 0.1 — 2026-09-19</strong></summary>

<br>

### ✨ Added

* Added the initial consolidated planning baseline.

</details>

---

## 🗂️ Change Categories

To keep future entries consistent and easy to scan, changes may be grouped using the following categories:

| Category                 | Purpose                                              |
| :----------------------- | :--------------------------------------------------- |
| ✨ **Added**              | New functionality, components, APIs or documentation |
| 🔄 **Changed**           | Changes to existing behavior or structure            |
| 🐛 **Fixed**             | Bug fixes and corrections                            |
| 🗑️ **Removed**          | Removed functionality or obsolete components         |
| 🔐 **Security**          | Security-related changes                             |
| 🏛️ **Architecture**     | Architecture decisions, invariants and boundaries    |
| 🧪 **Testing**           | Test coverage and verification changes               |
| ⚠️ **Known Limitations** | Explicit limitations of the current implementation   |
| ⏳ **Deferred**           | Work intentionally postponed to a future milestone   |
| 🛠️ **Development**      | Tooling, CI and development-process changes          |

---

## 📝 Entry Template

<!--
Copy this block for a new dated Unreleased entry.

## 📅 YYYY-MM-DD — Short Milestone Name

### ✨ Added

* ...

### 🔄 Changed

* ...

### 🐛 Fixed

* ...

### 🏛️ Architecture

* ...

### 🧪 Testing

* ...

### ⚠️ Known Limitations

* ...

### ⏳ Deferred

* ...

### 🛠️ Development

* ...

---
-->

<!--
When creating an actual software release, move the relevant Unreleased
entries beneath a version heading such as:

# 📦 [0.1.0] — YYYY-MM-DD

![Release](https://img.shields.io/badge/release-0.1.0-0969da?style=flat-square)

Then reset the Unreleased section for the next development cycle.

Software releases and Specification revisions remain separate histories.
-->

<div align="center">

---

**Nexus Cerebri · Change History**

<sub>Software releases and specification revisions are maintained as separate version authorities.</sub>

</div>
