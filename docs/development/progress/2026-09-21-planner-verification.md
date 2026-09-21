# Deterministic planner verification and hardening — 2026-09-21

## Starting state and scope

Fetched origin/main at `3efd5237e3ef5415724ca6803a6195fe989225b6` (merged PR #5).
Observed successful remote Cerebri CI run 35542866311 and Pages run 35542866332.
The maintainer's original checkout remained at f986bae with staged/untracked previous
work. Created isolated branch `codex/planner-verification` from fetched main; preserved
the original checkout. Read Master 0.4, accepted ADRs 0001–0012, roadmap, integration
progress and implementation/tests before changing code.

Goal: stronger evidence for the existing deterministic baseline, not expanded planning.
No ML, providers, autonomous execution, RRULE parser, repair search, joint optimization,
website redesign or fabricated verification dashboard. Software remains unreleased 0.2.0.

## Methodology and reproducibility

Test-only SplitMix64 uses wrapping u64 arithmetic, independent per-case integer seeds,
and Fisher-Yates permutations. No entropy, clock, new crate or production dependency.
A property crate would add dependency/lockfile cost without a needed generator/shrinker
for these small domains. Failures identify the seed and/or complete request/graph; named
regressions preserve discovered defects. Automatic shrinking is not implemented.

| Campaign | Domain and independent oracle |
| --- | --- |
| Planner | Seeds 0..512 (exclusive upper bound); horizon 4–12 minutes, step 1–3, duration 1–5, budget 0–14, 1–3 busy objects, optional two one-day series, earliest/latest/availability/deadline, fixed/minimum duration, date/time, buffers, dependency, external lock and explicit/learned preference. Integer enumeration determines full grid, evaluated prefix, feasible set, rank and first candidate. No production constraint/overlap helper defines expected results. |
| Determinism | Three shuffles per planner seed: objects, facts, constraints/dependency declarations, source series, preferences and their evidence. Compare complete serialized results: candidate/occurrence IDs, graph, report, conflicts, ordering and assessment. |
| Graph | Seeds 0..768: first 512 encode every directed graph on three nodes including loops; remaining 256 generate 0–6 nodes, missing references and mixed components. Floyd-Warshall closure finds cyclic SCCs; enumeration of all node permutations selects the lexicographically first valid topological order. Production uses bounded traversals/Kahn instead. Duplicate edge declarations and shuffled inputs preserve graph results. |
| Recurrence | Seeds 0..256: daily/weekly intervals 1–3, count 1–12, inclusive until, anchors before horizon, clipped boundaries and no-occurrence domains. Enumerate days from a Monday anchor rather than production's seek/ordinal helpers. Existing Temporal Core goldens supplement varied weekly anchors. |
| Malformed input | Named JSON mutations in tests/support/corpus.rs cover IDs, confidence/nonfinite attempts, duration/ranges, schema, provenance, recurrence, duplicates, targets, graph defects, budgets, unknown fields/tags, deep/truncated JSON. Seeds 0..128 perform bounded deletion/truncation/replacement of fixture bytes. Successful parses must agree exactly across Core and both HTTP routes; rejected parses produce HTTP 4xx. Oversized bodies produce 413. |
| Limits | Individually exercise 256 objects, 1,024 constraints, 256-node/1,024-declaration graph, 32 series/1,024 occurrences, shared date/occurrence budgets and 4,096 candidate evaluations. Limits are combined, not a promise that all maxima fit simultaneously. No brittle timing assertions. |

Supplementary lifecycle validation checks every returned candidate. Existing golden
composition tests mutate constraints one at a time and verify hard contradictions remove
solutions instead of adding a soft penalty. Occurrences remain occupancy, never actions.

Reproduce with `cargo test -p cerebri-planner --test randomized --test graph_oracle
--test compilation --test boundedness --locked` and
`cargo test -p cerebri-api --test malformed --locked` (single-line commands).
The normal workspace suite includes all campaigns; no optional fuzz runner is required.

## Previous independent-review findings

- A — RESOLVED. With a pure compiler and immutable request the hypothesized differing
  second result was not reproducible. Nevertheless `.ok().flatten()` discarded the typed
  error boundary. Internal validation now returns its report and compiled view together;
  planning reuses that result. Failed compilation always has a blocking issue, zero evaluated
  candidates and no solution. Lifecycle validation independently recompiles current state.
- B — DOCUMENTED and regression-tested. The identity tuple includes series ID, nominal
  local date/time and timezone; excludes horizon, clipping and revision. Earlier/Later
  intentionally share nominal identity while UTC ranges/resolution differ. No hash change.
- C — RESOLVED. Round trips and HTTP tests distinguish 0.1 without temporal state, 0.2
  with/without it, 0.1 carrying temporal state, future/invalid versions. No silent upgrade.
- D — RESOLVED. Incomplete coverage blocks generated scenarios before search and retains
  unknown ranges. No partial-coverage planning semantics were introduced.

## Discovered defects and minimal fixes

1. Externally reachable local-date overflow: valid extreme UTC JSON plus a positive/negative
   zone offset panicked in Chrono's infallible local-date conversion. Preserved upper/lower
   boundary regressions and real API parity. A checked Temporal Core conversion uses the
   same IANA offset and reports Overflow; ExplicitDate rejects OutsideBounds. Internal
   proven-invariant expects elsewhere were not mechanically replaced.
2. Metadata amplification: requests below the HTTP cap could pass count-based admission
   while large evidence vectors were repeatedly cloned/hashed. Typed callers also had no
   whole-request bound. Capped serialization and declared metadata-work admission reject
   before candidate enumeration. The three new resource regressions failed before the fix.
3. Compiler evidence amplification: canonical source evidence could be copied into 1,024
   occurrences. A cumulative emitted-evidence limit rejects atomically before those clones.
4. Lab guards accepted arbitrary validation-issue entries and invalid ID strings. Mutations
   of real Rust responses failed the new tests before correction. Added variant/payload
   validation and bounded identifier checks; no UI redesign or planning logic in TypeScript.

[ADR-0013](../../architecture/decisions/ADR-0013-planner-resource-admission.md) records the
resource admission change. Request compact bytes <= 256 KiB; bytes × declared candidates
<= 16 MiB; emitted canonical occurrence evidence <= 1 MiB. No existing maximum was raised.
Compiler reuse and the checked local date restore existing contracts rather than replace them.

One test-development correction: an ended recurrence can examine zero dates. Its duplicate
also consumes zero, so the shared-date exhaustion assertion applies only when the first
series actually examined dates. This was an oracle setup mistake, not a production defect.

## Proof, contracts and ordering

`ProvenOptimal` occurs only after complete enumeration with a feasible first candidate
minimal under the declared objective. Full no-solution yields `Complete`; a truncated
empty prefix yields `NeedsRelaxation`/`BestFound`, never `NoSolution`. Budgets zero/one and
exact exhaustion are exercised. Overflow beyond the representable grid closes only a
space containing no further representable placements; it does not turn a partial prefix
into a proof. Unknown/invalid requests do not claim completion.

Set-like input order is permuted, while output rank, search trace, recurrence sequence and
topological order remain intentionally ordered and are compared exactly. Duplicate graph
declarations are graph-idempotent; adding declarations/evidence is not promised to preserve
the identity of an entire request. Single-target search is the campaign's valid domain;
request validation also serves explicitly supplied lifecycle batches, so `/validate` can
accept a multi-target batch that baseline `/plan` does not support.

DST goldens cover Berlin gaps/folds, Lord Howe's half-hour fold and Apia's skipped day:
Reject/Skip, Earlier/Later, full versus clipped ranges, identity and source evidence.
Existing Temporal Core tests continue checking timezone primitives. No manual zone table.

Rust stays the domain authority. Schema generation remains deferred: this campaign found
localized missing presentation checks, not recurring schema evolution/drift among multiple
maintained clients. Real HTTP fixtures and negative mutation tests now cover those gaps.

## Resource/security review and remaining blind spots

Reviewed validation multiplication, iterative graph reachability, recurrence seek/expansion,
candidate cloning/hashing, conflict/evidence accumulation, serialization and HTTP body limits.
The graph remains O(V(V+E)) with V<=256/E<=1,024; recurrence has shared deterministic date
and occurrence caps; search retains its 1,000,000 count-based combined work cap and now
adds metadata admission. Compiler output is atomic. No scope/capability/authorization grant
is derived from graph order or recurrence identity.

This is property-tested and independently cross-checked over bounded generated domains,
not formally verified. It does not exhaust all schedules, every timezone transition, the
full JSON grammar, repeated requests/concurrent load, allocator behavior, or lifecycle
batch optimization. Direct callers own memory before deserialization. Metadata admission
is a conservative work proxy, not an exact memory/latency guarantee. Deployment worker
isolation remains outside this local development API milestone. No wall-clock performance
claim or continuous coverage-guided fuzzing is made.

## Verification results

| Command | Observed local result |
| --- | --- |
| `cargo fmt --check` | Passed |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | Passed |
| `cargo test --workspace --locked` | Passed, including all new campaigns and lifecycle/execution tests |
| `cargo test -p cerebri-planner --test boundedness --locked` | 5 passed, including exact byte-bound acceptance/one-byte-over rejection |
| `cargo doc --workspace --no-deps --locked` | Passed with `RUSTDOCFLAGS=-D warnings` |
| `cargo build -p cerebri-node --locked` | Passed |
| `node --test bindings/node/test.mjs` | 2 passed |
| `npm run check` | Passed |
| `npm run docs:check` | Passed, zero errors/warnings; delegates to site check |
| `npm run docs:build` | Passed with `CEREBRI_BASE_PATH=/Nexus-Cerebri`; delegates to site build |
| `npm --prefix apps/cerebri-site run check` | Passed, also run directly after factual roadmap-status update |
| `npm --prefix apps/cerebri-site run build` | Passed, also run directly with /Nexus-Cerebri base |
| `npm --prefix apps/cerebri-lab run check` | Passed, zero errors/warnings |
| `npm --prefix apps/cerebri-lab test` | 22 passed, including expanded real-response negative mutations |
| `npm --prefix apps/cerebri-lab run build` | Passed |
| `npm --prefix apps/cerebri-site run test:e2e` | 7 passed (Chromium, including Axe and responsive overflow) |
| `npm --prefix apps/cerebri-site run test:visual` | 4 local Windows hash mismatches; baselines unchanged; remote Linux gate checked after push |
| `npm audit --audit-level=high` | Passed, zero findings |
| `npm --prefix apps/cerebri-lab audit --audit-level=high` | Passed, zero findings |
| `npm --prefix apps/cerebri-site audit --audit-level=high` | Passed threshold, 3 existing Low findings in cookie/SvelteKit dependency chain |
| `cargo audit --version` | Unavailable: cargo-audit is not installed; no Rust advisory verification claimed |

The local website visual result matches the limitation recorded by the preceding integration
session; no site CSS/layout or screenshot baseline is changed. Exact-head remote Linux CI
must establish the repository visual gate. Final diff review includes every new file and
confirms no lockfile changes, generated artifacts or original-checkout changes in this PR.

Remaining findings: MINOR — local Windows visual-baseline portability and existing website
Low advisories. Missing cargo-audit is a verification limitation, not a discovered vulnerability.
No unresolved CRITICAL/MAJOR planner
defect was found by this campaign. This is evidence from the listed checks, not a claim
that all defects are absent. Independent release review remains pending.

Verified implementation commits: `861fc5e` (planner hardening and schedule oracle),
`acd32e8` (recurrence/graph invariants), `eb6f078` (API corpus/parity), `bdf8d01`
(Lab guard fixes). The final documentation/status commit and exact remote CI outcome
are identified in the PR and maintainer report. Only factual roadmap status changed
on the official site; its separate architecture and visual baselines remain intact.

## Release decision and next step

No version bump, tag or release in this feature PR. The deterministic randomized and
malformed-input evidence gap is now addressed for the declared baseline domain. Publication
still needs independent PR review, accepted tightening of admission, and release artifact/
compatibility qualification under the repository release standard. Exact pushed-head CI
is checked separately and reported with the PR; configuration alone is not evidence.

Exactly one next milestone: independent review and release qualification of this verified
deterministic planner. That milestone is not started here.
