# Nexus Cerebri --- Documentation, Git & Release Standard

## Core rule

**GitHub must represent the current state of Nexus Cerebri.** A feature
with stale documentation is incomplete.

## Language

Code, identifiers, branches, commits and internal errors: English.\
Documentation and relevant UI: German + English.

Use parallel `docs/de/` and `docs/en/` trees. Translations should
preserve meaning, not awkward word-for-word phrasing.

## Three explanation depths

-   **Simple:** understandable to students/beginners.
-   **Technical:** precise developer/university-level treatment.
-   **Research:** equations, assumptions, algorithm details,
    limitations, experiments and references.

Central formulas must define variables and give intuition.

## Required files

README.md, CHANGELOG.md, docs/development/roadmap/README.md, CONTRIBUTING.md, SECURITY.md,
LICENSE when selected, `docs/`, ADRs, research docs and dated progress
logs.

## README must stay current

It states what Cerebri is, current version/status,
implemented/not-yet-implemented capabilities, architecture overview,
quick start, test commands, docs, roadmap, contributing and security
links.

## ADR format

`ADR-XXXX: Title`, Date, Status, Context, Decision, Alternatives,
Consequences, Migration/Compatibility, References.

Never silently reverse an accepted architecture decision.

## Dated development log

Every meaningful work session records:

-   date
-   goal
-   completed
-   tests
-   visualizations
-   learned
-   problems/open questions
-   docs updated
-   next step
-   version/commit

## Git

Use Conventional Commits: feat, fix, test, docs, refactor, perf, build,
ci, chore(release).

Commits should be coherent and reviewable.

## Versioning

Semantic Versioning. Research phase: `0.x.y`.

Version domains are separate: software, REST API, CPIR schema, models,
datasets.

Do not increment software version for every commit.

## Changelog

Each release has `## [version] - YYYY-MM-DD` and relevant
Added/Changed/Fixed/Deprecated/Removed/Security/Documentation sections.

## GitHub Pages

Generated from repository Markdown; never maintain a second
documentation truth.

Provide DE/EN navigation, architecture diagrams, learning/research,
API/reference, visible version/status, responsive layout and accessible
presentation.

## Documentation CI

Check docs build, links, DE/EN counterparts, version consistency,
examples where practical, secrets and release metadata.

## Release checklist

Implementation/tests/security green; DE/EN docs current;
README/ROADMAP/CHANGELOG/version current; ADRs/progress log current;
compatibility reviewed; artifacts/checksums prepared where relevant;
release notes/tag/Pages build complete.


## Canonical version authorities

- Software: Cargo workspace/package version; Git release tags mirror it.
- Specification: independent document revision (`0.x`), not a software release.
- CPIR: serialized schema version + schema docs.
- API: route/protocol version.
- Model: ModelRegistry artifact metadata.
- Dataset: dataset manifest + checksum.

Use ISO 8601 dates everywhere.

## ADR triggers

ADR required for public API/CPIR semantics, dependency/module boundaries, persistence ownership, planner/executor boundary, permission/security model, version authority, temporal interval semantics, solver/search semantics, or replacement of an accepted architecture strategy.

## DE/EN parity

Canonical public architecture/reference/learning docs require DE and EN counterparts. CI checks existence/metadata/links, not semantic translation quality. Progress logs, raw experiment notes and ADR records may remain in one canonical language unless explicitly promoted to public documentation.

## Visualization applicability

“Code + tests + visualization + documentation = feature” means visualization **when it materially improves understanding of behavior, trends, distributions or decisions**. Infrastructure changes do not require artificial graphs.

## Master Specification archive rule

The current normative Master Specification lives at `docs/architecture/specifications/master-v0.4.md` (ADR-0007). Superseded revisions move to `docs/archive/specifications/`. Root Markdown is reserved for repository entry points and agent instructions.
