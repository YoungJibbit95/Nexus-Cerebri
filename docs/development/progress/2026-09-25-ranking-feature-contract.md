# Ranking Feature Contract v0.1 — 2026-09-25

## Goal, baseline and scope

Implement only the qualified first intelligence-foundation slice: deterministic ranking
observations. Fetched origin/main at `6aae6d97f011cd1acae440444385754272c34021`;
fast-forwarded the canonical local checkout from `32fa8c7` (only the research README differed).
Branch: `codex/ranking-feature-contract-v0-1`. The pre-existing local AGENTS.md edit is
preserved and excluded from session commits. No clone, worktree, history rewrite or release.
Software remains 0.2.0 unreleased; CPIR 0.1/0.2 and REST /v1 are unchanged.

## Implemented decisions

- Private-field RankingFeatureSet in cerebri-preferences, independent schema 0.1.
  A private wire struct uses `deserialize_with` without defaults for both nullable fields;
  TryFrom validates version and paired nullability. Missing rejects, null is None,
  typed value is Some. Ordinary derived Option missing/null collapse is explicitly tested.
- Reuse one feature computation for candidate output, costs, explanation source and
  the existing total ordering key. Retain the numeric RankingFeatures compatibility helper.
  No changes to candidate generation, hard validity, traversal, proof, authorization or execution.
- Preserve `.num_seconds().unsigned_abs()`, including +/-0.8 seconds mapping to zero.
  Contextual mutations and lossy shift-zero semantics are documented in ADR-0014 and DE/EN.
- Rust/REST/Node/Lab parity; the candidate inspector shows missing evidence and provenance.
  Lab validates imported observation shape and never computes ranking features.

## Compatibility evidence

- 240 preference permutations (all ten source-precedence pairs, 24 permutations each),
  including same-source time ties and duplicate source/time entries with different evidence.
- 384 generated planner cases (32 seeds x 3 fractional starts x 4 request modes), each
  checked against an independent integer-millisecond ordering/feasibility oracle and a
  reversed preference/object/constraint input. Includes missing/present preference evidence,
  prospective/existing placements, full/bounded search and contextual mutation counts.
- Eleven complexity scenarios check unchanged independent manifest expectations plus
  recorded baseline full order, candidate count, evaluated count and exhaustion. Includes
  half-open touching/+1-second overlap, dense gaps, composition, dependencies, scope rejection,
  BestFound and the compound unique slot. Manifest files were not rewritten.
- Direct comparison with the pre-edit baseline executable: all 23 planning example outputs
  are equal after removing only each candidate's new ranking_features field. This includes
  validation, outcome, assessment, complete candidate contents/order, conflicts, search-space
  counters, compilation, graph and content-bound plan IDs. The temporary local capture at
  target/ranking-before.json has SHA-256
  `3b5a844cfb8574ee32cdddad529cd7cef07eeac02651ca94cd5ae1fa0606d3ac`.
  It is bounded differential evidence, not an independent correctness oracle or universal proof.
- Actual JSON truth table, malformed/missing/duplicate/unknown fields, all five sources,
  u32/u64 wire limits, chrono extremes, deterministic roundtrips, and None vs Some(0).

## Local verification

Local Windows / pinned Rust 1.97.0 / Node 26.3.1. Site build/browser gates use
`CEREBRI_BASE_PATH=/Nexus-Cerebri`; rustdoc uses `RUSTDOCFLAGS=-D warnings`.

| Command | Result |
| --- | --- |
| `cargo test -p cerebri-preferences --locked` | PASS, focused wire/projection tests |
| `cargo test -p cerebri-planner --test randomized ranking_observations --locked` | PASS, 384 cases and reversed permutations |
| `cargo test -p cerebri-planner --test composition --locked` | PASS, 6 tests |
| `cargo test -p cerebri-preferences -p cerebri-planner --locked` | PASS before formatting; post-format local rerun blocked at composition by Windows 4551; corrected code passes full Linux workspace tests |
| `cargo fmt --check` | PASS in Linux CI after applying its formatter diff; BLOCKED locally by Windows application control (4551) |
| `cargo build --workspace --locked` | PASS |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS after replacing a test-only unnecessary vec with an array |
| `cargo test --workspace --locked` | PASS, 120 tests including doctests |
| `cargo doc --workspace --no-deps --locked` | PASS, warnings denied |
| `cargo build -p cerebri-node --locked` | PASS |
| `node --test bindings/node/test.mjs` | PASS, 4 tests |
| `npm run check` | FAIL: pre-existing unpinned traffic action; all 22 authority tests pass |
| `npm run docs:check` | FAIL: pre-existing figcaption-parent warning, 0 errors / 1 warning |
| `npm run docs:build` | PASS; existing figcaption warning remains |
| `npm --prefix apps/cerebri-lab run check` | PASS, 0 errors / 0 warnings |
| `npm --prefix apps/cerebri-lab run test` | PASS, 24 tests including real REST/Node/Lab comparison |
| `npm --prefix apps/cerebri-lab run build` | PASS |
| `npm --prefix apps/cerebri-site run test:e2e` | FAIL, 6/7 pass; existing qualification-status locator matches two elements |
| `npm --prefix apps/cerebri-site run test:visual` | FAIL, 4/4 homepage hashes differ from stored references |
| `npm audit --audit-level=high` | PASS, no findings |
| `npm --prefix apps/cerebri-lab audit --audit-level=high` | PASS, no findings |
| `npm --prefix apps/cerebri-site audit --audit-level=high` | PASS threshold, existing 3 low entries for cookie advisory |
| `cargo audit --file Cargo.lock --deny warnings --json` | PASS, 101 dependencies, zero vulnerabilities/warnings; advisory DB e2111519ba6d14a5da59a7b2e5c8083ae8a37c01 |

Initial findings: Windows application control blocked one freshly compiled composition
test executable (4551); later rebuilt subsystem tests ran successfully. The pinned rustfmt
executable is also blocked. No application-control setting was changed. The first e2e attempt
collided with the visual test's server port; the sequential rerun produced the 6/7 result above.
No test, threshold, lint or security gate was weakened.

## Remaining baseline gates and remote verification

The starting commit already includes `.github/workflows/gh-traffic-stats.yml` with mutable
`gtapps/gh-traffic-stats@v1` and the nested figcaption in VisualizationFrame.svelte. Upstream
v1 and v1.0.0 both resolved to `f05c995b41d817087f7f728dd1794af559d81471` on this date.
Minimal pin/markup corrections were proposed to the maintainer as a separate scope decision;
no such corrections are included without approval. The homepage duplicates qualification
text, invalidating the old strict Playwright locator. Its visual references predate the
baseline's `32fa8c7` site changes. This slice changes no official-site source or visual hashes.
The existing cookie finding and its retained high-severity gate are discussed in the
[prior qualification report](2026-09-22-release-qualification-corrections.md); no dependencies changed.

Implementation commit `ab8971b` and contract/docs commit `38360b6` were pushed for remote
verification. [First CI run](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/36192618668)
reported formatting differences; the pinned Linux formatter's exact output was applied in
`5b3e292` without changing semantic tokens. The
[subsequent run](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/36192804975)
passes formatting, workspace build, Clippy, workspace tests, Node build and Node tests, then
fails at the existing unpinned action in `npm run check`. Final pushed-head CI is inspected separately and reported at
session completion. This record does not claim all gates or independent qualification passed.

## Documentation, limitations and next step

ADR-0014, ADR index, Master, DE/EN planner reference, Changelog and progress index are updated.
The Lab uses the existing candidate inspection surface; no new visualization subsystem.
Lessons: field-level custom deserialization must be tested for presence, and zero remains
an intentionally lossy ordering projection. JavaScript displays safe integers; Rust's external
wire validator accepts full u64 while actual chrono-derived distances fit safe integers.
Old Lab output imports without ranking_features reject rather than silently inventing data.
The feature contract is not the final ML feature space. No EvaluationEpisode, telemetry,
Nexus/Flux, learned ranking/search, training, provider work or execution expansion was added.
Next step: independent implementation qualification, not Slice 2 or release publication.
