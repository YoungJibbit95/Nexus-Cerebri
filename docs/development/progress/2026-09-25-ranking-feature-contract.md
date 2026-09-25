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

## Verification (in progress)

Local Windows / pinned Rust 1.97.0 / Node 26.3.1. Final command outcomes and remote CI are
recorded below after verification; unexecuted or blocked gates are never implied to pass.

Initial findings: Windows application control blocked one freshly compiled composition
test executable (4551); later rebuilt subsystem tests ran successfully. The pinned rustfmt
executable is also blocked. No application-control setting was changed. Two existing baseline
gate failures are unrelated to this slice: the unpinned gh-traffic-stats@v1 action and a
nested figcaption in the official site. Minimal corrections require the maintainer's scope decision.

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
