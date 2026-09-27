# 2026-09-27 — Evaluation foundations: partial Phase A checkpoint

Historical checkpoint at `aa86b82accac28144b39fd46b10da74e9e6da931`.
The corrected consolidated contract closes the four wire gaps below; current status and
evidence are in the [Phase-A completion record](2026-09-27-evaluation-phase-a-completion.md).

## Repository and authority revalidation

Fetched origin; authoring-time and actual base are both
`f1934417445099388e8123676879d51b3cd029ab`. Classification:
**IMPLEMENTATION-MECHANICAL RECONCILIATION**; no base drift or architecture conflict found.
Created `codex/slice-2-evaluation-replay` from that exact base in the canonical workspace.
The unrelated `codex/creative-fidelity` branch and its published commits are retained.
Pre-existing `.gitignore` changes and untracked `test-results/` are preserved and excluded.

The final Slice-2 implementation prompt authorizes Phase A only. The supplied consolidated
Intelligence v1.2.6 source and its SHA-256 are recorded in
[ADR-0015](../../architecture/decisions/ADR-0015-evaluation-contract-foundations.md).
Historical architecture-stage prohibitions are not treated as current implementation bans.
No Codex PR Code Review, delegated review or self-review loop was requested or run.
This is implementation evidence, not independent Architecture/Data/Math/Security qualification.
Usage checkpoints observed 16% then 18% of the weekly window consumed; this stop is caused
by missing exact wire definitions, not by treating elapsed time as clarification/approval.

## Implemented contract sections

- Sections 3–5: required-nullable decoding helper; bounded IdentifierString/RuleId,
  lowercase SHA-256, canonical i64/u64/positive-seconds strings, nanoseconds,
  seconds/nanoseconds instant, date/local time, typed positive time range and tagged optional.
- Sections 7–8 and 37: explicit closed token registries, including synthetic-only DataScope,
  the two RevisionEntityType values and canonical admission issue-code vocabulary.
- Section 24.1–24.3: structural artifact semantic identity and manifest schema 0.1;
  bundle-relative paths; unique/sorted paths and nonempty unique/token-sorted roles.
- Sections 27, 39–40: build/version-only SoftwareProvenance, PrincipalGroupRef shape and
  privacy metadata with content_fields_present fixed to false.
- Sections 43–44: strong EpisodeId, validated NEW_RUN/REUSED_RUN source nullability,
  ordered planner-run timestamps, and explicitly raw lifecycle/lineage wire components.
- Pin workspace serde_json to `=1.0.151`; Cargo.lock and resolved serializer are unchanged.

The new module is not called by existing planner or transport paths. No Episode domain
constructor exists, so partial structural validation cannot grant validated Episode status.
Raw lifecycle/lineage decoding does not perform parent lookup, reason derivation, event
reduction, time ordering across records or reusable-run equivalence validation.
Manifest decoding is not JCS canonicalization, digest verification or artifact resolution.

## Focused evidence and local gates

Sixteen contract tests cover real JSON decoding/roundtrips, absence/null/value, duplicate
members, unknown/wrong variant fields, full i64/u64 bounds, canonical spellings, invalid
dates/times, subsecond and negative-epoch values, typed range shape/order, non-synthetic
DataScope, path traversal/machine paths, manifest path/role duplicates, run source nullability,
planner-run time order, forbidden provenance authority and content-bearing privacy metadata.

| Command | Result |
| --- | --- |
| `cargo test -p cerebri-planner --test evaluation_wire --locked` | PASS, 16 tests |
| `cargo fmt --check` | PASS after applying the pinned formatter |
| `cargo check -p cerebri-planner --locked` | PASS |
| `cargo test -p cerebri-planner --locked` | BLOCKED at existing boundedness executable by Windows application control, OS error 4551; test never executed |
| `cargo test -p cerebri-planner --test boundedness --locked` | Same execution-policy block in the smallest affected target |
| `cargo clippy -p cerebri-planner --all-targets --all-features --locked -- -D warnings` | PASS |
| `npm run check` | PASS, 22 version-authority tests plus repository/link/DE-EN/dependency policy |
| `git diff --check` | PASS |

No application-control setting, binary identity, test threshold or CI gate was changed to
evade this local block. Remote Linux CI is inspected after pushing the checkpoint; its
actual result is reported at session completion, without inferring success for pending/skipped gates.

## Outstanding authoritative wire definitions

The consolidated text supplies most previously missing clauses, but four exact wire forms
remain unspecified. A clarification request was raised before implementing dependent types:

1. Section 24.4 references ImmutableArtifactRefV0_1 without its JSON shape or accepted
   immutable/content-addressable reference grammar. This blocks ArtifactBinding and replay provenance.
2. Sections 41–42 reference OutcomeObservationV0_1 and allow null in E2, but do not define
   the non-null structure. "May be null" does not authorize rejecting every non-null value.
3. Native observations use CanonicalU64 without specifying number versus decimal string;
   CanonicalU64Decimal is separately defined and must not silently be substituted.
4. Episode schema_version is described as 0.1, without choosing the exact JSON shape
   (string versus major/minor object). Other schema authorities use both forms explicitly.

No schema was invented or broadened to serde_json::Value to hide these gaps.
ExperimentReplayBinding is also not expanded in the source, but current E2 explicitly
requires experiment_assignment=null; this is not presently classified as a separate blocker.

## Resumable handoff

Branch: `codex/slice-2-evaluation-replay`.
Current commit SHA: the checkpoint commit containing this record; obtain with `git log -1`.
Base SHA: `f1934417445099388e8123676879d51b3cd029ab`.
Completed phase: partial Phase-A foundations only; Phase A is not complete.
Changed files: `evaluation/` module, planner lib export, evaluation_wire tests, workspace
serde_json pin, ADR/index, Master, DE/EN planner references, Changelog, roadmap and this progress record/index.
Contract sections implemented: bounded component subset listed above.
Tests executed/passing/failing: local table above; pushed-head CI reported separately.
Known baseline failures: local Windows 4551; remote baseline is not presumed failing.
Remaining phases: finish Phase A; B–E remain unstarted and outside this session.
Unresolved issues: four wire definitions above; complete Episode/nested wire types and
validated-domain conversion are not implemented.
Architecture conflicts: none identified; incomplete wire definitions require authoritative clarification.
Exact recommended next action: obtain those definitions, complete the remaining Phase-A
wire/domain surface, rerun its gates, then stop at the Phase-A boundary.

No Slice-1 ranking/preference/ordering, admission/search, policy, authorization, execution,
CPIR or transport changes. No Phase B, fingerprint writer, canonical graph, runtime capture,
real-user data, learning, provider integration, release, tag, version bump or PR creation.
