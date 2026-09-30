# Application integration foundation — 2026-09-30

## Scope and source evidence

Maintainer request: begin implementing Cerebri's application-independent integration
foundation and supply a concrete Nexus Ecosystem agent prompt. This supersedes the earlier
research-only request, but does not authorize rollout, execution or publication.

Implementation branch: `codex/application-integration-foundation`, canonical Cerebri
workspace, based on `f3a1726` (main, merged Lab PR #45). Nexus model paths were re-inspected
at `10f8684c2da530ddc5549f2298f2794123a093d7`; no Nexus file was changed. Its concurrent
merge work is outside this slice. Concurrent local Cerebri Lab/desktop asset/startup changes
and the maintainer's `.gitignore` edit are preserved and excluded from this work.

## Implemented boundary

Core owns Integration 0.1 admission and the manifest. REST and native Node commands delegate.
Only a prospective single Event with FIND_SLOT, explicit coverage and zero mutation authority
is admitted. Core still owns permissions, scope, evidence, temporal compilation, constraints,
ranking, budgets and lifecycle. Planned responses carry correlation/context identity and the
complete unchanged PlanningResult. No executor, data store, provider or research dependency.

The new Node host calls have cancellation, a bounded process deadline, bounded output and
sanitized transport errors. TypeScript covers the envelope; consumers must decode the CPIR
result fields they use. Existing general `plan`/REST behavior remains compatible.

The shared synthetic fixture, DE/EN reference and Nexus N1 implementation prompt make the
boundary reviewable. N1 is an offline mapper/host seam with real Rust conformance, not a
user-facing feature rollout. Missing real free/busy evidence remains an explicit prerequisite.

## Verification

Targeted implementation checks executed successfully:

- Core tests: exact seven expected starts, unchanged result parity, all profile rejections,
  read/plan denial, incomplete coverage, unknown duration, empty scope, horizon mismatch,
  admission limits, malformed/oversized input and ActionPlan conversion denied as AnalysisOnly.
- REST tests: exact core manifest/result parity including incomplete coverage; typed sanitized
  rejection, malformed/oversized body handling and no execution endpoint.
- Node tests: actual Rust result parity, legacy CPIR 0.1 retained, version/profile rejection,
  incomplete coverage, NoSolution, BestFound, bounded input/output, sanitized process/parser
  failure, response correlation, and cancellation/deadline killing real disposable children.

The first exact-start assertion accidentally compared Rust Display formatting to wire RFC3339;
it was corrected to inspect serialized instants. No planner behavior or threshold was changed.
Broader local checks passed: `cargo test --workspace --locked`, `cargo fmt --check`,
`cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`,
the eight Node tests, strict TypeScript declaration checking, `npm run check`,
`npm run docs:check`, `npm run docs:build`, and `cargo audit --file Cargo.lock --deny warnings`.
All three npm audits passed the repository's `--audit-level=high` gate; the site audit
reported three low-severity transitive cookie/SvelteKit findings. No dependency or gate
was changed to suppress those findings.

Rustdoc initially failed with E0460 due to inconsistent compiled crate versions in the
shared build cache. The same source passed `cargo doc --workspace --no-deps --locked
--target-dir target/integration-doc -j 2` with `RUSTDOCFLAGS=-D warnings` in a fresh build
directory inside the canonical checkout. No replacement workspace or cache deletion.

Remote CI for the pushed commit is checked through the associated pull request before
handoff; the PR records its run/status. Local checks include concurrent uncommitted work,
so the remote clean checkout is the decisive evidence for the isolated committed change.

## Qualification and exclusions

This is implementation evidence, not independent Architecture, Math or Security qualification.
ADR-0016 remains Proposed with Architecture/Security review pending. Existing qualifications
are not extended by assertion. No software version, tag, release or model/data publication.
No Nexus production integration, general timezone input resolver, provider adapter, public
service authentication/isolation, full TypeScript CPIR SDK, persistent execution or production
evaluation capture. Rollback is consumer disablement/removal of additive routes, with no
state migration and no fallback that fabricates slots.
