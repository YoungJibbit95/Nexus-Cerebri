# 2026-09-27 — Evaluation contract foundations: Phase A completion

## Authority and repository reconciliation

The maintainer explicitly resumed from `aa86b82accac28144b39fd46b10da74e9e6da931`
and replaced the earlier consolidated source with the corrected
`NEXUS_CEREBRI_INTELLIGENCE_ARCHITECTURE_v1.2.6_CONSOLIDATED.md`.
Its SHA-256 is `f6e10d0a20dcada223d02328b50bdd0a4dee4e5c10442ed76d1500ba7e6baadf`.
It closes all four wire-definition gaps recorded in the
[partial checkpoint](2026-09-27-evaluation-phase-a-checkpoint.md).
[ADR-0015](../../architecture/decisions/ADR-0015-evaluation-contract-foundations.md)
records this replacement without changing the qualified architecture.

Classification: **IMPLEMENTATION-MECHANICAL RECONCILIATION**. No architecture conflict.
The canonical checkout initially matched the requested resume SHA. Fetch showed the older
remote implementation branch advanced to `1d348e05a04a54e0624e39e1ceb3023799ad9a44`
and main to `890d954602f84d686a34b2e0eb6449de69b663bc`. Their diff from the resume
point affects Site/CI only, not Slice-2 inputs. To honor the explicit resume point without
rewriting that published history or interfering with concurrent Site/doc edits, this checkpoint
uses `codex/slice-2-phase-a-completion` from exactly `aa86b82` in the same checkout.
No replacement clone/worktree or history rewrite was used.

Pre-existing and concurrent `.gitignore`, Site, public-copy/CPIR documentation and
`test-results/` changes are excluded. The shared progress index is left untouched; the
existing indexed partial checkpoint links here. No unrelated cleanup or baseline fixes.

## Completed Phase-A contract

- Full closed EvaluationEpisodeWireV0_1, a separate immutable EvaluationEpisodeV0_1
  container and fallible wire/domain conversion. Required members never default.
- Episode schema is exactly `{"major":0,"minor":1}` with required u16 integers;
  strings, positional arrays, fractional values, incompatible versions and extra fields reject.
- E2 outcome is required and explicitly null. The non-null type is uninhabited; no fake
  empty outcome or future outcome semantics are introduced.
- CanonicalU64 is an exact alias of CanonicalU64Decimal. Native observation counters
  are decimal JSON strings. Qualified RankingFeatureSet and native ordering-key numeric
  representations remain unchanged; TypeScript and planner production paths are untouched.
- Closed immutable-reference variants, artifact bindings, fixture digest parity, required
  privacy/retention source metadata and structural ReplayProvenance. E2 requires a source
  snapshot, no model and an explicit null experiment assignment.
- Typed nested wire shapes for generation, candidates, product decisions, exposure,
  replacements, interactions and capture completeness. These define decoding contracts;
  they do not produce observations or execute policies/reducers.
- Foundational lineage/run shape and timestamp validation, retaining the valid case in
  which reused planner-run timestamps predate the child Episode.
- Closed object decoders reject Serde's otherwise accepted positional-array alternatives,
  preserving streaming typed decoding and duplicate/unknown-field rejection.

The earlier canonical scalar/token/manifest components and serde_json `=1.0.151` pin remain.
No dependency or lockfile changes. Architecture sections 3–8, 24–27 and 29–44 supply this
Phase-A wire surface; nested search/placement shapes come from 10–11 and 51.5 only.
No canonical writer, graph, projection or fingerprint computation from Phase B is implemented.

## Validation boundary and remaining phases

Wire-valid domain construction is **not** R0 lifecycle/collection qualification, artifact
authentication, a fingerprint proof or behavioral-evaluation eligibility. In particular,
opaque content-address pins still need resolver-specific validation and manifest verification;
imported resolution-state tokens do not prove that verification occurred.
Candidate/search-set proofs, policy-order conformance, lineage reason derivation, parent
collection checks, manual-replacement context validation and interaction reduction remain
later-phase work. There is no production consumer that treats these wire types as such proofs.

The four prior definition blockers are resolved. Remaining work is the explicitly deferred
Phases B–E, not an incomplete Phase-A wire contract. Phase B was not begun. No PR Code
Review, independent qualification claim, delegated review, release, tag or version bump.

## Verification evidence

New tests cover every required Episode field; nested missing/null/value; null-only E2
fields; exact schema forms; duplicate/unknown/wrong-variant fields; synthetic-only scope;
decimal-string counters; all reference variants; fixture digest equality; source privacy
metadata; immutable domain conversion; NOT_ADMITTED shape and reused-run timestamps.
The existing 16 foundational tests remain. There are 16 new tests, 32 focused tests total.

| Gate | Local result |
| --- | --- |
| `cargo fmt --check` | PASS |
| `cargo check -p cerebri-planner --locked` | PASS |
| `cargo test -p cerebri-planner --test evaluation_episode_wire --test evaluation_wire --locked` | 31 tests passed before the final standalone-object guard/test; final rerun blocked executing evaluation_episode_wire by Windows Application Control, OS error 4551 |
| `cargo test -p cerebri-planner --locked` | 92 tests passed before the final standalone-object guard/test; final rerun compiled successfully, passed 28 boundedness/compilation/composition/dependency tests, then Windows blocked evaluation_episode_wire with error 4551 |
| `cargo clippy -p cerebri-planner --all-targets --all-features --locked -- -D warnings` | PASS |
| `npm run check` | PASS, 22 authority tests plus repository/link/DE-EN/dependency checks |

The positional-array regression was first reproduced with a failing negative test, then
fixed using the same typed map-only boundary. No security policy, binary naming, test
threshold or CI gate was changed to evade the Windows block. The pushed-head Linux CI
result is inspected separately and reported at session completion; pending gates are not
treated as success. This record is implementation evidence, not independent qualification.
Account usage checkpoints observed 28% then 32% of the weekly window consumed.

## Resumable handoff

Branch: `codex/slice-2-phase-a-completion`.
Current commit SHA: the checkpoint containing this record; obtain with `git log -1`.
Resume/base SHA: `aa86b82accac28144b39fd46b10da74e9e6da931`.
Original Slice-2 base SHA: `f1934417445099388e8123676879d51b3cd029ab`.
Completed phase: Phase A, contract foundations and wire validation only.
Changed files: evaluation artifact/module/primitive/component sources, new episode and
observation modules, new evaluation_episode_wire tests, ADR/index, Master, DE/EN planner
references, roadmap, Changelog, original checkpoint annotation and this record.
Contract sections implemented: bounded Phase-A surface listed above.
Tests executed/passing/failing: exact local table above; pushed-head CI reported separately.
Known baseline failure: local Windows Application Control; no remote failure is presumed.
Remaining phases: B–E, deliberately unstarted in this session.
Unresolved Phase-A wire definitions: none.
Architecture conflicts: none identified.
Exact recommended next action: stop; retain this checkpoint for separately authorized
Phase B and independent qualification. Do not automatically begin the next phase.
