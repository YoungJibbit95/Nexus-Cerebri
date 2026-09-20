<!-- doc: development; lang: en; counterpart: ../de/development.md -->
# Development and verification

Use pinned Rust 1.97.0; use Node 22.12+ for tooling. From the repository root:

```sh
cargo fmt --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo build -p cerebri-node --locked
node --test bindings/node/test.mjs
npm ci --ignore-scripts
npm run check
npm run docs:build
npm --prefix apps/cerebri-lab ci
npm --prefix apps/cerebri-lab run check
npm --prefix apps/cerebri-lab test
npm --prefix apps/cerebri-lab run build
npm audit --audit-level=high
npm --prefix apps/cerebri-lab audit --audit-level=high
cargo doc --workspace --no-deps --locked
```

CI runs these checks, denying Rustdoc warnings. Tests cover knowledge/schema round trips,
scope presence, hard-constraint rejection, stale snapshots, exact-plan confirmation,
authorization revocation, replay, partial execution and lost ledger completion.
Compile-fail doctests prove earlier lifecycle types cannot enter execute.
Interval properties are exhaustively tested over a small domain with no random seed.
Berlin DST transition fixtures are explicit. All time inputs are injected.

The repository checker enforces crate dependency directions, local Markdown links, counterpart
existence/language metadata, version markers and selected credential patterns.
It is not a complete secret scanner and does not evaluate translation meaning or remote links.
Cargo.lock and package-lock.json are committed. Generated target/, site/ and binaries are ignored.

Build the Lab before running the API and open /lab/ with examples/request.json. Planner shows UTC candidate timelines
and scores; Trace shows typed validation/conflict data; Preferences exposes score components.
Temporal displays core free/busy/unknown and DST traces. Semantics inspects supplied evidence; ML and Dataset remain inactive interfaces. JSON export preserves the inspected data.
The UI is internal tooling, not a stable product contract. Test fixtures contain synthetic IDs
and dates, no calendar titles, descriptions, personal data or tokens.

The Pages workflow is manual and restricted to main. It generates HTML from repository Markdown,
provides DE/EN navigation, and requires GitHub Pages setup. Nothing is published by a local build.
No software release is created by these workflows.

[Architecture](foundation.md) · [Deutsch](../de/development.md)

