<!-- doc: development; lang: en; counterpart: ../de/development.md -->
# Try Cerebri and inspect the results

The public website explains a fixed set of examples. The **Lab** is a separate local tool
for sending structured requests to the Rust implementation and examining its responses.
It is useful when you want to change an input and see how the result changes.

## Choose where to start

- To understand the project and its learning motivation, start with the
  [introduction](introduction.md).
- To try your own structured request, use the Lab instructions below. The current planner
  accepts [CPIR data](cpir.md), not a sentence describing an appointment.
- To integrate with an application, use the Rust facade, local REST API or provisional
  [Node process bridge](../../bindings/node/README.md). All three reach the same Rust code.

## Run the Lab locally

Use Rust 1.97.0 and Node 22.12 or newer. From the repository root:

```sh
npm --prefix apps/cerebri-lab ci
npm --prefix apps/cerebri-lab run build
cargo run -p cerebri-api
```

Keep the last command running and open <http://127.0.0.1:3000/lab/>. The API serves the
built Lab and listens only on the local machine. If its assets have not been built,
the Lab page returns 404. For frontend development with Vite, follow the
[Lab README](../../apps/cerebri-lab/README.md).

## What to look at

Start with a supplied synthetic example. **Planner** shows the possible times and their
comparison values. **Trace** shows request checks and reasons for rejection. **Temporal**
shows occupied, free and unknown time, including decisions made for clock changes.

The Lab's preferred-start example includes a wish for 10:45 UTC. The website's introductory
example has no preferred start. Those are two different inputs, so their first choices can
differ without either result being wrong. Select the same input when comparing results.

**Semantics** and **Preferences** inspect information already supplied or calculated.
**ML** and **Dataset** are inactive placeholders for future work. They do not train a model
or infer preferences. JSON export lets you keep the inspected data; the Lab does not store
requests and results in browser local storage.

## What the API validates

The [local REST routes](../../apps/cerebri-api/src/lib.rs) accept structured requests:

- `POST /v1/validate` checks whether a planning request meets the input requirements.
- `POST /v1/plan` checks the request, searches possible placements and returns its result.
- `POST /v1/temporal` calculates temporal diagnostics, such as recurrence and availability.

Request validation is not the later validation of an entire proposed plan, and it grants
no execution permission. There is no REST or Node operation for executing calendar
changes. See [the steps before execution](safety.md).

## Development and verification

Use pinned Rust 1.97.0; use Node 22.12+ for tooling. From the repository root:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `cargo test --workspace --locked`
- `cargo build -p cerebri-node --locked`
- `node --test bindings/node/test.mjs`
- `npm ci --ignore-scripts`
- `npm run check`
- `npm run docs:build`
- `npm --prefix apps/cerebri-lab ci`
- `npm --prefix apps/cerebri-lab run check`
- `npm --prefix apps/cerebri-lab test`
- `npm --prefix apps/cerebri-lab run build`
- `npm audit --audit-level=high`
- `npm --prefix apps/cerebri-lab audit --audit-level=high`
- `cargo doc --workspace --no-deps --locked`

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

The UI is internal tooling, not a stable product contract. Test fixtures contain synthetic IDs
and dates, no calendar titles, descriptions, personal data or tokens.

The [Pages workflow](../../.github/workflows/pages.yml) runs on pushes to main and can also
be started manually on main. It builds the website, including the repository Markdown
pages and DE/EN navigation, then deploys to configured GitHub Pages. A local build publishes
nothing. A documentation deployment does not create a software release.

[Architecture](foundation.md) · [Deutsch](../de/development.md)

