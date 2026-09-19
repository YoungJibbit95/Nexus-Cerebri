# Nexus Cerebri

**Software:** 0.1.0 foundation — unreleased. No release/tag has been created.

**Specification:** 0.4, frozen Foundation Architecture Baseline.

**CPIR:** internal schema 0.1. **REST:** development routes under /v1.

A Rust temporal planning foundation built around **neural intuition + symbolic verification**.
The current implementation is deterministic; neural inference is a future milestone.

## Working today

- Validated identifiers, confidence and explicit knowledge/processing states.
- IANA timezones, DST ambiguity rejection and half-open intervals.
- CPIR validation, bounded scope, facts, hard constraints and interpretable preferences.
- Single-event grid search with deterministic ordering, structured explanations and honest search assessments.
- Strong plan lifecycle, policy/capability/confirmation checks, and mock execution with freshness, replay protection and per-action results.
- REST facade, Node process bridge and a small developer Lab.

No provider integration, production authentication, durable ledger, recurrence engine, advanced repair, learning or autonomous calendar mutation is implemented.

## Quick start

Install Rust via rustup. The repository pins Rust 1.97.0 and its formatter/linter.
Node 22+ is needed only for the Node bridge and documentation tooling.

```sh
cargo build --workspace --locked
cargo test --workspace --locked
cargo fmt --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo run -p cerebri-core --example plan
cargo run -p cerebri-api
```

The development server binds only to [localhost:3000](http://127.0.0.1:3000).
Open [Cerebri Lab](http://127.0.0.1:3000/lab) and load [the synthetic request](examples/request.json).

```sh
cargo build -p cerebri-node
node --test bindings/node/test.mjs
npm ci --ignore-scripts
npm run check
npm run docs:build
```

Documentation is generated from repository Markdown into ignored `site/`.
The optional Pages workflow requires manual dispatch and repository Pages configuration.

## Workspace

| Area | Responsibility |
| --- | --- |
| cerebri-types | IDs, revisions, confidence, evidence and knowledge states |
| cerebri-temporal | trusted instants, zones, duration and interval relationships |
| cerebri-constraints | facts, hard rules and structured violations |
| cerebri-semantics | eight-dimensional evidence and derived deadline metadata |
| cerebri-preferences | interpretable precedence, scoring features and storage ports |
| cerebri-ml | production model/dataset/training metadata and inference ports |
| cerebri-planner | CPIR, scope, validation, search and lifecycle proofs |
| cerebri-core | synchronous planning facade and JSON bridge |
| cerebri-integrations | execution/ledger ports and in-memory contract adapters |
| apps/cerebri-api | HTTP transport; no execution endpoint |
| bindings/node | provisional asynchronous process bridge to the same Rust core |
| apps/cerebri-lab | internal typed-result inspection shell |
| research/ml-from-scratch | reserved human learning path, outside the workspace |

## Documentation

- [Deutsch](docs/de/README.md) · [English](docs/en/README.md)
- [Master specification 0.4](00_MASTER_SPECIFICATION_v0.4.md)
- [Implementation boundary](04_AGENT_IMPLEMENTATION_BOUNDARY.md)
- [Testing standard](03_TESTING_VISUALIZATION_RESEARCH_STANDARD.md)
- [Documentation/release standard](02_DOCUMENTATION_GIT_RELEASE_STANDARD.md)
- [Roadmap](ROADMAP.md) · [Learning roadmap](01_DEVELOPMENT_LEARNING_ROADMAP.md)
- [Contributing](CONTRIBUTING.md) · [Security](SECURITY.md) · [Changelog](CHANGELOG.md)
- [Foundation decisions](docs/decisions/ADR-0001-workspace.md) · [Progress](docs/progress/2026-09-19-foundation.md)

Accepted ADRs explicitly amending the Master take precedence only with synchronized Master updates;
then the current Master, domain standards, agent boundary and roadmap apply.
Review-resolution and archived documents remain historical/non-normative.
The new Foundation reference pages have DE/EN counterparts; the entire historical repository does not claim bilingual parity.

## Next milestone

Temporal Core: bounded recurrence and independently verified free/busy behavior.
Software version, specification revision, CPIR schema, REST version and future model/dataset versions remain separate authorities.
