# 2026-09-20 — Official website implementation

Status: implemented and verified on `feat/cerebri-official-site`

## Research snapshot

The implementation starts from main commit `f9b38fa109315c9cdf54cdf768b1679e2d1308ee`. Software authority is Cargo workspace version 0.2.0, Specification authority is Master 0.4 and CPIR is 0.1. The current planner is deterministic and bounded. There is no production provider integration, production authentication, durable execution ledger, learned preference system, neural inference path or REST execution endpoint.

All accepted ADRs through ADR-0010 were reviewed before architecture work. ADR-0011 records the public-site frontend strategy while preserving the canonical Markdown and Lab boundaries.

## Implementation plan

1. Preserve the Rust planning/domain dependency graph and keep Cerebri Lab separate.
2. Add a static SvelteKit/Svelte 5/TypeScript application at `apps/cerebri-site`.
3. Keep `site/` as deployment output and `npm run docs:build` as the canonical build entry point.
4. Generate website metadata from Cargo, Master, CPIR fixtures, ADRs, roadmap and repository Markdown.
5. Generate planner and temporal website fixtures by executing `cerebri-core`, never by duplicating algorithms in TypeScript.
6. Implement GitHub Pages base-path and trailing-slash behavior.
7. Build the global shell, accessible navigation, source anchors and responsive layout.
8. Implement the global Understand / Technical / Research explanation depth.
9. Implement `/` and the Intent → Plan educational entry experience.
10. Implement `/explore`, `/cpir`, `/planning`, `/time` and `/safety`.
11. Implement `/architecture`, `/roadmap`, `/developers` and the separate `/lab` handoff.
12. Render canonical repository docs through `/docs/[...slug]` without a CMS or runtime GitHub fetch.
13. Mark significant visualizations REAL / EDUCATIONAL / FUTURE CONCEPT and expose text alternatives.
14. Remove the obsolete static portal generator/presentation assets after SvelteKit becomes canonical.
15. Add type/component-level checks plus Playwright navigation, accessibility and responsive/visual smoke coverage.
16. Update repository checks, CI and Pages workflow for the new frontend and real fixture generation.
17. Update ADR/documentation map, changelog and this dated progress log with verification evidence.
18. Inspect the complete branch diff and keep commits conventional and logically separated.
19. Push the branch and inspect remote GitHub Actions results before considering the session complete.

## Architecture guardrails

- Facts outrank inference; constraints and permissions remain explicit.
- Planner is not Executor.
- The website never claims unavailable providers, production execution, neural inference or learned preferences.
- Recurrence diagnostics remain separate from CPIR planner constraints until the roadmap milestone integrates them.
- Research/future concepts are visually distinguishable from implemented behavior.

## Implemented website slice

- Added ADR-0011 and `apps/cerebri-site` as the static public SvelteKit application.
- Added a build-time authority extractor for Cargo version, Master revision, CPIR version and canonical Markdown.
- Added real Rust-generated planner and temporal outputs through `cerebri-core` examples.
- Added the public routes `/`, `/explore`, `/cpir`, `/planning`, `/time`, `/safety`, `/architecture`, `/lab`, `/roadmap`, `/developers` and repository-backed `/docs/[...slug]`.
- Added global Understand / Technical / Research depth state and REAL / EDUCATIONAL / FUTURE CONCEPT visualization labels.
- Added keyboard focus treatment, skip navigation, text alternatives for visualizations, reduced-motion handling and responsive transformations.
- Added Playwright route/navigation checks, Axe serious/critical accessibility floor, horizontal-overflow checks and visual baselines at 1440, 1024, 768 and 390 px.
- Retired the old `scripts/build-docs.mjs`, `scripts/docs-theme.css` and `scripts/docs-ui.js` portal pipeline.
- Extended repository truth checks to guard official-site versioning, static Pages configuration, route inventory, retired portal files and runtime font imports.

## Verification state

Verified on commit `ca3406a027b68e66a3e98a1dbc53138f7e4c8171`.

GitHub Actions push run `35520398395` and pull-request run `35520401015` both completed successfully.

The verified pipeline includes:

- `cargo fmt --check`;
- workspace `clippy` with warnings denied;
- complete Rust workspace tests;
- Node bridge tests;
- repository truth/link/version checks;
- reproducible `npm ci` for the official site;
- Svelte diagnostics with zero errors/warnings;
- static SvelteKit build under the real `/Nexus-Cerebri` Pages base path;
- Playwright route, explanation-depth and horizontal-overflow tests;
- Axe serious/critical accessibility checks;
- deterministic visual regression hashes at 1440, 1024, 768 and 390 px;
- Cerebri Lab check/test/build;
- root, site and Lab high-severity npm audits;
- Rust documentation with warnings denied.

The official-site `package-lock.json` is committed. The one-time lock bootstrap workflow was removed after generating it. CI and Pages now use `npm ci`. GitHub Actions dependencies were updated to their current majors during verification.
