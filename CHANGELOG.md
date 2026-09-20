<div align="center">

# 📜 Changelog

**Nexus Cerebri · Software & Specification History**

<br>

![Software](https://img.shields.io/badge/software-0.2.0-0969da?style=flat-square)
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

**Current software:** `0.2.0`
**Next roadmap target:** `0.3.0`
**Release status:** `Unreleased`

> [!WARNING]
> No software release or tag has been created yet.

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
* Remote GitHub Actions verification is recorded after the coherent feature branch is pushed.

---

## 📅 2026-09-20 — Temporal Core, Cerebri Lab and Repository Structure

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
