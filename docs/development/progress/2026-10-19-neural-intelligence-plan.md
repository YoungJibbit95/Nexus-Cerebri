# 2026-10-19 — Neural Intelligence development plan v1 (proposal)

## What was added

A documentation-only milestone that starts the Cerebri Neural Intelligence assignment with
discovery and governance rather than implementation:

- [Neural Intelligence development plan v1](../roadmap/neural-intelligence-development-plan.md) —
  the PR roadmap A–H, dependency graph, Lab mapping, dataset/evaluation strategy, privacy and
  security invariants, verification strategy, risk register and open maintainer decisions.
- A pointer from the [roadmap](../roadmap/README.md) to that plan.
- This progress record and its index entry.

## Decisions recorded (maintainer, 2026-10-19)

1. First slice: Milestone A, documentation-only plan (no ML implementation).
2. LittleDevTeam (`architect-dev-team`) installed locally and git-ignored; not versioned.
3. Language scope: DE-first with EN challenge fixtures.
4. Technology stack: PyTorch research + optional ONNX Rust inference, Burn for Rust teaching,
   Candle for comparison only.

## Governance state

- The roadmap restriction *"Do not begin ML …"* remains in force; this milestone does not
  lift it.
- ADR-0017 remains `Proposed`; no implementation is authorized.
- Outstanding independent qualifications (ADR-0015 B.4/full Phase-B, ADR-0016
  Architecture/Security) remain pending and affect sequencing.

## Verification

`repository_policy` (`npm run check`) and `documentation` (`npm run docs:check`,
`npm run docs:build`) are the required checks for this milestone; results are recorded in the
LittleDevTeam state and in the pull request. No Rust, Python, model or dataset code is
touched, so the Rust workspace is unchanged.

## Limitations

This milestone implements no interpretation, model, dataset or Lab feature. It only proposes
and sequences future work and records the prerequisite gates. Nothing here is independently
qualified; the plan is a proposal for maintainer review.

## Next action

Request approval to lift the roadmap restriction and start Milestone B (deterministic
interpretation baseline), or direct a different sequence.
