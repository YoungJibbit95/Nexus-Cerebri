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
