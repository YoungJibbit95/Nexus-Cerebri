# 2026-09-26 — Ranking import parity correction

## Scope and implementation

Correct only independent Slice-1 review findings MAJOR-01 (Lab import cross-field
parity) and MAJOR-02 (negative imports), starting from main
`736e3ec989e19f58890090d03b56bc97edfe437c` on
`codex/ranking-import-parity-correction`. Authority remains Master v0.4 and
Accepted ADR-0014; no architecture or version authority changes.

`parseResult()` uses the already validated `RankingFeatureSet` and numeric candidate /
ordering fields to enforce, for every candidate:

- `ranking_features.mutation_count == mutation_count == ordering_key.mutation_count`.
- `ranking_features.shift_seconds == shifted_seconds == ordering_key.shifted_seconds`.
- A present preferred-start distance equals both `cost` and
  `ordering_key.preference_distance_seconds`.
- A null preferred-start distance requires both legacy projections to equal zero.

The null-to-zero projection is used only for comparison; parsing returns the original
object unchanged. Missing fields remain invalid, and null remains distinct from a
present zero distance. Source remains provenance, without source resolution, temporal
calculation or ranking logic in TypeScript. File and HTTP imports share this guard.

## Regression and compatibility evidence

The new focused test failed against the unchanged guard because a contradictory
feature mutation count was accepted. It passes after the correction, with 52 rejected
JSON imports and six accepted controls spanning null, zero and nonzero distance,
mutation counts zero and one, and nonzero shift. Each case mutates one repeated field;
the second candidate is targeted to check validation beyond the first entry. Accepted
controls retain object identity and contents. All eight requested mismatch categories
are covered, together with direct feature-field tampering.

Existing missing/null/value, paired nullability, schema 0.1, unknown-field/source,
numeric validation and transport parity tests remain intact. Real Rust REST / Node /
Lab comparisons still pass, including all five preference sources and subsecond
quantization. The API scenario test also preserves exact core/planner parity.
This is bounded regression evidence, not independent qualification of the milestone.

## Executed local gates

| Command | Result |
| --- | --- |
| `node --experimental-strip-types --test apps/cerebri-lab/tests/transport.test.ts` | PASS, 9 tests |
| `npm --prefix apps/cerebri-lab run check` | PASS, 0 errors / 0 warnings |
| `npm --prefix apps/cerebri-lab run test` | PASS, 25 tests including real transport parity |
| `npm --prefix apps/cerebri-lab run build` | PASS |
| `cargo build -p cerebri-node --locked` | PASS |
| `node --test bindings/node/test.mjs` | PASS, 4 tests |
| `cargo test -p cerebri-api --test planner_scenarios --locked` | PASS, exact API/core/planner scenario parity |
| `npm run check` | FAIL at the existing `gtapps/gh-traffic-stats@v1` pinning check; all 22 version-authority tests pass |
| `git diff --check` | PASS |

The complete correction diff and both import call sites were reviewed locally from
the boundary-validation perspective. Delegated investigation was unavailable because
of the agent usage limit; no independent review completion is claimed. Remote CI
results are reported with the pushed branch; this record does not claim remote gates
passed or that skipped gates ran.

## Boundaries and remaining limitations

Rust production, ranking semantics, preferred-start resolution, candidate generation,
ordering, SearchAssessment, permissions, execution, CPIR and dependencies are unchanged.
There is no Slice 2, EvaluationEpisode, Nexus/Flux, ML or learned-search work.
No release, tag or version bump. Baseline traffic-action pinning, branch protection,
AGENTS.md, CI structure, site checks and historical research wording remain outside
this correction. Pre-existing local AGENTS.md edits are preserved and excluded.
