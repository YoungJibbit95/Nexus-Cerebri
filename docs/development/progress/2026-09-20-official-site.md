# 2026-09-20 — Official website implementation

Status: in progress on `feat/cerebri-official-site`

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
