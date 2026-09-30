# Slice 2 Phase B.4 — DecisionObservation / O1 / O2

Date: 2026-09-30. Final planned Phase-B implementation checkpoint.
Independent B.4 Data/Hard-Math and complete Phase-B qualification remain pending.
Phase C is unstarted.

## Authority and base reconciliation

Authority: maintainer-supplied Intelligence Architecture v1.2.6 consolidated contract,
SHA-256 `007fdbab8630ecd8119215f329d7e9e36d73076de1bef885f1055c859da2edab`.
Scope follows its §§20–21, 29–32, 50–52, 55–58 and the bounded B.4 handoff.

PR #46 was merged before work began, on head
`ab76125618cf25ac556f7177bcf8a57b6cfedc6c` with all 14 then-required checks successful.
Fetched starting `origin/main`: `7947340f21a5b70975d9c1244e29766023aaab7c`.
Fresh branch: `codex/slice-2-phase-b-decision-observation`.
The existing Intelligence worktree was reused; unrelated work and preserved stashes
were untouched. B.1–B.3 qualification is maintainer-supplied evidence, not a claim
derived from this implementation agent's tests.

Reconciliation: merged B.3 plus mechanically reconciled application-integration drift.
Main's integration API/Core/Node changes and existing dependency edges were inspected;
they changed no planner/evaluation authority. Current Node integration tests are part
of the completion gate. No dependency, lockfile or version changes belong to B.4.

## Implementation boundary

`DecisionObservationProjectionV1::from_observations` accepts the DecisionInput
fingerprint and existing native generation, candidates, eligibility, ranking and display
observations. It neither executes planning nor accepts exposure/lifecycle data.
The exact eight-field digest payload uses canonical decimal strings, canonical optionals,
existing instant/placement grammars and the existing DecisionObservation hash domain:

`SHA256("nexus-cerebri:decision-observation-digest:v1\0" || UTF8(JCS(payload)))`.

- Admission is copied from native generation, with canonical unique issue-code order.
  Explicit pre-Episode non-fingerprintable issue codes cannot produce a digest payload.
- SEARCHED projects materialized X/T and stored counts/hashes, deriving discovered count
  from candidates. It checks unique visits, membership in X, existing set/order hashes,
  actual set equality for exhaustion and the assessment/proof combinations from §12.
- Candidates use one placement, the supplied discovered rank and the qualified Slice-1
  ordering tuple. Ranks must be exactly 1..V and consistent with that complete tuple.
  Placement, feature/key parity, unique fingerprints/semantic placements and membership
  in the visited set are checked. Preference provenance binds the digest but never ranks.
- Native projection recomputes CandidateFingerprint through the existing placement
  authority, then builds one unique CandidateId-to-CandidateFingerprint map. All product
  references must resolve. CandidateId itself is never serialized into the digest.
- Eligibility covers V exactly once; eligible/exclusion RuleId nullability must agree.
  Digest records are fingerprint-sorted. Ranked E must exactly match the eligible subset
  in supplied discovered-rank order. Display L is a unique eligible subset whose given
  deterministic order is preserved.
- NOT_ADMITTED normalizes proof to NONE, requires insufficient-information admission,
  empty candidates and all downstream states NOT_APPLICABLE/PLANNING_NOT_ADMITTED.
  It cannot carry search materialization or fake empty APPLIED policy results.

Trusted native projection sorts candidates by their supplied rank and eligibility by
fingerprint. Strict payload import rejects malformed collection order and cross-field
contradictions instead of repairing them. The existing DI duplicate-preserving syntax
visitor was extracted as a private shared helper; DI still rejects booleans, while DO
permits the grammar's booleans. JSON numbers, null and duplicate members remain rejected
before typed decoding. Closed nested objects reject extra/missing fields and positional arrays.

## External oracle and compatibility evidence

The [fixture record](../../../crates/cerebri-planner/tests/fixtures/evaluation/README.md)
documents exact external bytes. Typed semantic inputs are constructed independently.

| Vector | Bytes | Expected and obtained domain digest |
| --- | --- | --- |
| O1 | 2295 | `5fc321286e7e64467abb2692052f9d47f1d5a6cd66ba093d09ef07084b21076b` |
| O2 | 584 | `410cfb50881dd7bcabb341c44eec875bed1594917861c326447456903b0dd5bf` |

O1 is a DIGEST_SERIALIZATION_FIXTURE, not R3 replay. Its supplied all-a fingerprint
and DecisionInput fingerprint are prevalidated conformance inputs. Lower-level digest
wire/import validation does not authenticate supplied candidate or DecisionInput identities.
Normal native projection additionally rejects a candidate hash inconsistent with placement.

O2 is fingerprintable NOT_ADMITTED. A separate real typed request with missing duration
produces fresh BSF and DI and the O2 branch structure, without search. Its fresh source
identities are not asserted equal to the external fixture's prevalidated identities.

The nine focused tests cover exact O1/O2 bytes/digests, CandidateId renaming and native
collection-order invariance, exclusion of actual exposure D/O, display-L sensitivity,
eligibility/RuleId/ranking/features/key/placement/search/outcome sensitivity, invalid
mapping/coverage/ranks and strict nested wire negatives. Exposure tests use a local native
envelope only and make no lifecycle-validity claim.

Previous G1/G2/G3, B1, D1, H1, set/order, CandidateFingerprint and domain-sanity fixtures
and expected digests remain unchanged. Rust planning, ranking, preference resolution,
admission, generation, execution and transport authorities are unchanged.

## Verification

Focused B.4 tests: 9 passed. Focused prior regressions: DI 14, fingerprints 4,
Episode wire 16, scenario 20 and admission work 3 passed.
An expanded positional-array negative test first exposed Serde's acceptance of `["NONE"]`;
existing object-only guards at B.4 optional/branch/instant fields close that gap without
changing the frozen native primitives. Focused and full tests were rerun after the fix.

All local completion gates passed:

- `cargo fmt --check`
- `cargo check -p cerebri-planner --locked`
- `cargo build --workspace --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `cargo test --workspace --locked`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`
- `cargo build -p cerebri-node --locked`
- `node --test bindings/node/test.mjs` (8 passed, including current integration coverage)
- `npm run check`, `npm run docs:check`, `npm run docs:build`
- Unstaged and staged `git diff --check`

The live main ruleset was queried rather than inheriting an old check count. It currently
requires Lab build/check/test, three npm audits (root/site/lab), Repository policy,
Rust dependency advisories, Rust workspace, Rustdoc, Documentation check/build, Site
build/typecheck and Browser/E2E (14 checks). Visual verification is not required.
Actual CI results for the pushed commit belong to the PR/handoff evidence; local success
does not imply remote success, and known homepage visual debt is not waived or changed.

## Remaining boundary

Separate independent B.4 Data/Hard-Math review and full Phase-B qualification remain.
There is no runtime capture, telemetry, collection/lifecycle reducer, interaction semantics,
artifact resolver, replay, learned policy or Phase-C work. No release, tag, version change,
automatic merge or Codex PR Code Review is part of this checkpoint. Unrelated repository
baseline debt, including homepage visual baselines, is not changed.
