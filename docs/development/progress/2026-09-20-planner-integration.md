# Deterministic planner integration — 2026-09-20

## Repository truth and version authority

Started from clean `main` at `f9b38fa` after fetching and fast-forwarding the local branch.
The existing workspace and Lab declare software `0.2.0`; the roadmap agrees. The
Changelog's `0.1.0` software target was stale and is corrected with a repository guard.
Specification remains `0.4`; CPIR baseline is `0.1`; REST routes remain `/v1`.
Observed successful CI and Pages runs for `f9b38fa` before implementation.

Goal: bounded recurrence compilation, dependency diagnostics, deterministic constraint
composition and real typed Lab results. Preserve execution authorization and research isolation.
No release is created merely to synchronize documentation.

Verification of this documentation slice: `npm run check`, `npm run docs:build`.
Next: define the compiler and graph contracts in an accepted ADR before implementation.


## Completion after interruption — 2026-09-21

The maintainer requested completion of the interrupted final step and commits, without
starting another milestone. The original checkout contained an unfinished merge of the
already-published official-site work and a stash of this session's tracked modifications.
Recovered only the planner session into an isolated worktree, retaining that stash and the
original merge. Merged published main 339b1c7 there. Renumbered the planner ADR to 0012
because official-site ADR-0011 had since reached main. No unrelated uncommitted work is included.

Completed: independently testable recurrence snapshot compilation; distinct source/series/
occurrence state; explicit horizon, DST and coverage semantics; atomic limits and duplicate
rejection; deterministic dependency graph and cycle evidence; explicit ranking keys;
hard-constraint composition and independent integer-oracle/golden schedules. Preserved
single-target search, scope ALL semantics and execution authorization/freshness boundaries.

Visualization: Lab now renders real compiler occupancy/free/unknown ranges, occurrence
provenance and DST skips, graph edges/order/issues, candidate ordering and raw typed output.
Eleven checked-in source scenarios pass through REST/Core/Planner and the TypeScript guards.
The unchanged website example remains CPIR 0.1; its reference text now explains 0.2 support.

### Verification

| Command | Observed result |
| --- | --- |
| cargo fmt --check | Passed |
| cargo clippy --workspace --all-targets --all-features --locked -- -D warnings | Passed |
| cargo test --workspace --locked | Passed, including API scenario parity, planner oracles and lifecycle compile-fail doctests |
| cargo doc --workspace --no-deps --locked | Passed with RUSTDOCFLAGS=-D warnings |
| cargo build -p cerebri-node --locked | Passed |
| node --test bindings/node/test.mjs | 2 passed |
| npm run check | Passed |
| npm run docs:check | Passed, zero errors/warnings |
| npm run docs:build | Passed, GitHub Pages base /Nexus-Cerebri |
| npm --prefix apps/cerebri-lab run check | Passed, zero errors/warnings |
| npm --prefix apps/cerebri-lab test | 22 passed, including 11 live API scenarios and negative drift cases |
| npm --prefix apps/cerebri-lab run build | Passed |
| npm audit --audit-level=high | Passed, zero vulnerabilities |
| npm --prefix apps/cerebri-lab audit --audit-level=high | Passed, zero vulnerabilities |
| npm --prefix apps/cerebri-site audit --audit-level=high | Passed threshold; 3 existing low-severity cookie/SvelteKit dependency findings |
| npm --prefix apps/cerebri-site run test:e2e | 7 passed after installing Playwright Chromium |
| npm --prefix apps/cerebri-site run test:visual | 4 screenshot-hash mismatches on local Windows; existing baselines preserved, remote Linux CI must decide this gate |
| cargo audit --version | Unavailable locally: cargo-audit is not installed; no Rust advisory audit success claimed |

The first shared-target test attempt hit a Windows executable lock from an existing API
process. A fresh isolated target directory resolved it without stopping that process.
The first local website browser attempt was blocked by missing Playwright Chromium;
after installing it, browser/Axe/overflow checks passed. Local screenshot hashes differed from
the existing baselines at all four widths; no baseline was changed to hide these results.
Exact-HEAD remote Linux CI is checked after push.

### Findings and release decision

CRITICAL: no findings from the performed checks.
MAJOR: no known failing planner or Lab checks; broad randomized/fuzz release evidence remains
outside this bounded completion and is not claimed complete.
MINOR: existing website dependency audit findings; cargo-audit unavailable locally; local
Windows visual hashes differ from the existing website baselines (remote gate pending).
QUESTION: none requiring a decision for this bounded commit.

No release/tag or software bump: software remains unreleased 0.2.0, specification 0.4,
CPIR 0.2 with legacy 0.1, REST /v1. Full 0.3.0 release acceptance is not asserted.
Next milestone: deterministic planner release validation with randomized schedules and
malformed-input fuzzing. No ML, providers, advanced repair or autonomous mutation started.

Commit sequence: version reconciliation (f986bae), published-main integration (d9c6dc6),
planner compilation/graph slice (8827da6), Lab contract/visualization slice (db37b01),
and completion records.
Exact resulting hashes and remote CI outcome are reported to the maintainer after push.
