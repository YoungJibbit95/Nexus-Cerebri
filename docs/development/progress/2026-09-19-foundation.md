# Foundation bootstrap — 2026-09-19

## Date
2026-09-19

## Goal
Implement the first tested software foundation against specification 0.4.

## Completed
Nine domain crates, REST facade, Node process bridge, Lab result inspection, temporal/CPIR
validation, deterministic grid planning, strong lifecycle and mock execution contracts.
Added storage/model interfaces, synthetic fixtures, CI and paired public Foundation references.

## Tests
Final local verification passed:
- cargo fmt --check
- cargo clippy --workspace --all-targets --all-features -- -D warnings
- cargo test --workspace: 41 unit/integration tests and 3 compile-fail doctests, all passed
- cargo build --workspace --locked
- node --test bindings/node/test.mjs: 2 tests passed
- npm ci --ignore-scripts; npm run check; npm run docs:build
- npm audit --audit-level=moderate: zero reported vulnerabilities
- cargo doc --workspace --no-deps --locked with RUSTDOCFLAGS=-D warnings
- git diff --check

GitHub Actions has been configured but has not been run remotely in this session.

## Visualization
Lab consumes typed PlanningResult JSON, renders UTC candidate timelines and exposes scores,
validation and conflicts with raw JSON export. Semantics/ML/Dataset views are reserved.
Browser smoke test loaded the synthetic request, called the local API, displayed seven
candidates from eleven grid positions and four overlap rejections, and opened the typed Trace
view. No browser console errors were captured. The temporary server and browser tab were closed.

## Learned
Private lifecycle fields are insufficient if deserialization can fabricate proof types; later
stages deliberately have no Deserialize. Replay claims must survive failed completion recording.
Search optimality must name the grid/objective and never follow from a truncated candidate list.

## Problems / Open Questions
No unresolved Foundation architecture decision. Master section 9 contained a preference order
inconsistent with section 2 and the bootstrap instructions; synchronized the stale summary.
The initial documentation renderer dependency had npm advisories; updated to 14.3.2 and audited
again with zero reported npm vulnerabilities. No production authentication, durable ledger,
native Node addon, recurrence expansion or provider integration is claimed.

## Documentation Updated
README, Master consistency cleanup, CONTRIBUTING, SECURITY, ROADMAP, CHANGELOG;
DE/EN Foundation/CPIR/development references; six implementation ADRs; research/data/model notes.

## Next
Temporal Core: bounded recurrence, free/busy and independently specified timezone scenarios.

## Version / Commit
Workspace 0.1.0, unreleased; specification 0.4; CPIR 0.1; REST v1 development routes.
Local working-tree implementation based on bcf7540. No commit, tag, release or deployment created.
