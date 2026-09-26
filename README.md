<div align="center">

# 🧠 Nexus Cerebri

**Deterministic temporal planning in Rust, with typed evidence, bounded search and explicit authority boundaries.**

<br>

[![Software](https://img.shields.io/badge/software-0.2.0-0969da?style=for-the-badge)](#project-status)
[![Specification](https://img.shields.io/badge/specification-0.4-8250df?style=for-the-badge)](docs/architecture/specifications/master-v0.4.md)
[![CPIR](https://img.shields.io/badge/CPIR-0.2-1f883d?style=for-the-badge)](docs/en/cpir.md)
[![Release status](https://img.shields.io/badge/status-unreleased-d29922?style=for-the-badge)](#project-status)
[![REST](https://img.shields.io/badge/REST-%2Fv1-d1242f?style=for-the-badge)](docs/en/api.md)

[![Rust](https://img.shields.io/badge/Rust-1.97.0-000000?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Node.js](https://img.shields.io/badge/Node.js-22.12%2B-339933?style=flat-square&logo=nodedotjs&logoColor=white)](https://nodejs.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](LICENSE)
[![CI](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/workflows/ci.yml/badge.svg)](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/workflows/ci.yml)

[![GitHub stars](https://img.shields.io/github/stars/YoungJibbit95/Nexus-Cerebri?style=flat-square&logo=github)](https://github.com/YoungJibbit95/Nexus-Cerebri/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/YoungJibbit95/Nexus-Cerebri?style=flat-square&logo=github)](https://github.com/YoungJibbit95/Nexus-Cerebri/forks)
[![GitHub issues](https://img.shields.io/github/issues/YoungJibbit95/Nexus-Cerebri?style=flat-square&logo=github)](https://github.com/YoungJibbit95/Nexus-Cerebri/issues)
[![GitHub last commit](https://img.shields.io/github/last-commit/YoungJibbit95/Nexus-Cerebri?style=flat-square&logo=github)](https://github.com/YoungJibbit95/Nexus-Cerebri/commits/main)
![Views](https://img.shields.io/endpoint?style=flat-square&url=https://raw.githubusercontent.com/YoungJibbit95/Nexus-Cerebri/_gh_traffic_stats/.github/badges/views.json)
![Clones](https://img.shields.io/endpoint?style=flat-square&url=https://raw.githubusercontent.com/YoungJibbit95/Nexus-Cerebri/_gh_traffic_stats/.github/badges/clones.json)

</div>

Nexus Cerebri is an open-source temporal planning foundation. It turns a **structured planning problem** into a bounded set of candidate plans, rejects candidates that conflict with known state or hard rules, orders the remaining candidates deterministically, and keeps planning separate from authorization and execution.

The project exists to make scheduling and temporal reasoning inspectable instead of implicit. Timezones, daylight-saving transitions, incomplete availability, dependencies, constraints, preferences, permissions and execution state are represented explicitly so that a result can be checked rather than merely trusted.

The current planner is deterministic. Learned or neural components are a later research direction, not part of the current planning runtime.

---

## What Cerebri can do today

Current repository behavior includes:

- typed identifiers, evidence, provenance, confidence and explicit knowledge/processing states;
- UTC instants, IANA timezones, half-open intervals and DST-aware temporal diagnostics;
- bounded daily/weekly recurrence expansion for existing series;
- completeness-aware availability: unknown coverage is not treated as free time;
- CPIR validation with explicit scope, facts, hard constraints, preferences, policy and planning capability;
- bounded single-target grid search with deterministic candidate ordering;
- dependency graph and cycle diagnostics;
- versioned deterministic ranking observations through Ranking Feature Contract v0.1;
- structured search outcomes such as `ProvenOptimal`, `Complete` and `BestFound`;
- a typed lifecycle from proposal through validation, action translation, authorization and execution;
- in-memory/mock execution contracts with freshness and replay protection;
- REST and provisional Node transports backed by the same Rust core;
- Cerebri Lab for inspecting real planner and temporal outputs;
- a static official site generated from repository documentation and Rust-produced fixtures.

The baseline planner currently searches **one target object**. It does not perform general multi-object repair or continuous-time optimization.

---

## One real planning example

The repository ships a current CPIR 0.2 fixture at [`examples/request.json`](examples/request.json).

It asks Cerebri to create one prospective 30-minute event inside this planning horizon:

```text
09:00Z ├──────────── planning scope ────────────┤ 12:00Z
        ├──── existing event "busy" ────┤
        09:00Z                         10:00Z
```

The request uses:

- operation: `CREATE`;
- one target: `new-event`;
- duration: 1,800 seconds;
- planning scope: `[09:00Z, 12:00Z)`;
- one existing event: `[09:00Z, 10:00Z)`;
- candidate granularity: 900 seconds (15 minutes);
- no preferred-start evidence in this fixture.

The current grid therefore evaluates 11 possible starts:

| Candidate start | Result |
| --- | --- |
| 09:00 | rejected — overlaps the existing event |
| 09:15 | rejected — overlaps the existing event |
| 09:30 | rejected — overlaps the existing event |
| 09:45 | rejected — overlaps the existing event |
| 10:00 | valid |
| 10:15 | valid |
| 10:30 | valid |
| 10:45 | valid |
| 11:00 | valid |
| 11:15 | valid |
| 11:30 | valid |

The 10:00 candidate is valid because Cerebri uses half-open intervals: `[09:00, 10:00)` ends exactly where `[10:00, 10:30)` begins.

There is no preferred-start signal in this fixture. The ranking feature therefore records no preferred-start evidence. For the existing numeric ordering key, that absence projects to zero only for the distance term. The remaining terms are also equal for these prospective candidates until the start instant, so **10:00Z ranks first**.

The grid is fully traversed and contains valid candidates, so the result is `Solution / ProvenOptimal`.

Here, **`ProvenOptimal` means optimal for this declared 15-minute grid and the current deterministic ordering objective**. It does not claim global optimality over continuous time, future repair spaces, unsupplied provider data or future learned objectives.

---

## How the current planner works

The current planner does not parse the English description above. Its actual input is structured CPIR.

```text
structured CPIR
    ↓
request validation
    ↓
bounded temporal/context compilation
    ↓
candidate grid generation
    ↓
scope + known-state + hard-rule checks
    ↓
deterministic ordering of valid candidates
    ↓
PlanningResult + search assessment
```

A few distinctions are central to the design:

| Concept | What it means |
| --- | --- |
| **Known state / facts** | supplied reality the planner must respect |
| **Hard constraints** | rules that make a candidate invalid when violated |
| **Preferences** | soft signals that can order valid candidates but do not rewrite facts |
| **Planning scope** | the bounded domain the planner may consider; it does not grant permission |
| **Policy / capability** | deterministic limits on what may be proposed or later executed |
| **Validation** | establishes plan validity; it does not itself authorize execution |

The planner checks validity before ranking. A preference can make one valid candidate preferable to another, but it cannot make an invalid candidate valid.

For the exact request and output contracts, see [CPIR](docs/en/cpir.md), [planner integration](docs/en/planner-integration.md) and [ADR-0014](docs/architecture/decisions/ADR-0014-ranking-feature-contract.md).

---

## Deterministic ordering

The current total candidate ordering key is:

```text
(
  preferred_start_distance_seconds,
  mutation_count,
  shift_seconds,
  start,
  object_id
)
```

The preferred-start source follows this precedence when evidence exists:

1. explicit current request;
2. session context;
3. personal learned provenance;
4. global learned provenance;
5. default.

These provenance categories are part of the current data contract. They do **not** imply that a preference-learning pipeline is currently implemented.

Ranking Feature Contract v0.1 exposes deterministic observations for inspection and parity. It is not an ML training schema. In particular, missing preferred-start evidence remains distinct from a present value that quantizes to zero: `None` and `Some(0)` are different domain states even though the legacy numeric ordering projection uses zero for absence.

---

## What a search assessment actually says

| Assessment | Meaning |
| --- | --- |
| **`ProvenOptimal`** | the declared bounded grid was exhausted and the first candidate is optimal for the current objective/tie-break rules |
| **`Complete`** | the declared grid was exhausted without a valid solution |
| **`BestFound`** | search stopped before exhausting the declared grid, for example because of a candidate budget |

These assessments describe the search that actually ran. They do not turn a bounded discrete search into a claim about every possible continuous schedule.

Rejected positions are retained as structured conflict evidence for the declared search space. The current `ConflictSet` is not claimed to be a minimal or irreducible explanation.

---

## Planning is not execution

Cerebri deliberately separates planning from authority:

```text
PlanningRequest
  → ProposedPlan
  → ValidatedPlan
  → ActionPlan
  → AuthorizedActionPlan
  → ExecutionResult
```

The planner never gains execution authority merely by finding a good candidate. Validation, current policy, planning capability, execution grants, confirmation, freshness and adapter capabilities remain separate checks.

The repository includes in-memory/mock execution contracts that exercise this lifecycle. It does **not** currently include production calendar-provider integration, production authentication or a durable production action ledger.

---

## Current boundary and research direction

Implemented today:

- deterministic temporal reasoning and bounded planning;
- CPIR 0.2 with the documented CPIR 0.1 legacy path;
- bounded recurrence occupancy for existing daily/weekly series;
- deterministic ranking observations;
- lifecycle and mock/in-memory execution contracts;
- Rust-backed REST, Node bridge, Lab and static documentation site.

Not implemented as current runtime behavior:

- free-form natural-language interpretation;
- learned ranking or learned search;
- neural inference inside the planner;
- production provider synchronization;
- production authentication;
- durable execution/reconciliation infrastructure;
- general multi-object repair search;
- prospective recurrence planning and recurrence constraints;
- unrestricted autonomous calendar mutation.

The `cerebri-ml` crate currently defines versioned model/dataset/training metadata types and inference **ports**. It does not provide an inference or training backend.

The longer-term research direction explores learned or neural intuition as an advisory layer around deterministic verification. Hard validity, permissions and execution authority remain deterministic boundaries.

---

## Project status

| Authority | Current state |
| --- | --- |
| Software | `0.2.0` — **unreleased**, undergoing release qualification |
| Published release | none |
| Master specification | `0.4` |
| Current CPIR | `0.2` |
| Legacy CPIR | `0.1` |
| REST | development routes under `/v1` |
| Ranking observations | `RankingFeatureSet` schema `0.1` |

These are independent version domains. A change to one does not imply a change to the others.

Deterministic Ranking Feature Contract Slice 1 is implemented and has completed independent Math and Security qualification. That qualification does not publish software `0.2.0`; release qualification and an explicit publication decision remain separate.

See the [roadmap](docs/development/roadmap/README.md), [changelog](CHANGELOG.md) and [version/release standard](docs/development/documentation-git-release-standard.md).

---

## Quick start

The repository pins Rust **1.97.0**. Node.js **22.12+** is required for the Lab, Node bridge, official site and documentation tooling.

Run commands from the repository root.

### Build and install dependencies

```sh
cargo build --workspace --locked

npm --prefix apps/cerebri-lab ci
npm --prefix apps/cerebri-site ci
npm ci --ignore-scripts
```

### Run the main checks

```sh
cargo test --workspace --locked
cargo fmt --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

npm --prefix apps/cerebri-lab run check
npm --prefix apps/cerebri-lab test
npm --prefix apps/cerebri-lab run build

npm --prefix apps/cerebri-site run check
npm run check
npm run docs:build
```

Run the deterministic planner example directly:

```sh
cargo run -p cerebri-core --example plan
```

### Start the API

```sh
cargo run -p cerebri-api
```

The API binds to [http://127.0.0.1:3000](http://127.0.0.1:3000).

### Start Cerebri Lab

In a second terminal:

```sh
npm --prefix apps/cerebri-lab run dev
```

Open [http://127.0.0.1:5173/lab/](http://127.0.0.1:5173/lab/).

Vite proxies `/v1` and `/health` to the Rust API, so the Lab remains an inspection frontend while the Rust core remains the planning and temporal authority.

Useful fixtures:

- [current CPIR 0.2 request](examples/request.json);
- [legacy CPIR 0.1 request](examples/legacy-cpir-0.1.json);
- [Berlin DST temporal request](examples/temporal-request.json);
- [planner scenario manifest](examples/planner/manifest.json).

For a production-build Lab review:

```sh
npm --prefix apps/cerebri-lab run build
cargo run -p cerebri-api
```

Then open [http://127.0.0.1:3000/lab/](http://127.0.0.1:3000/lab/).

---

## Workspace map

| Area | Responsibility |
| --- | --- |
| **`cerebri-types`** | IDs, revisions, confidence, evidence and knowledge states |
| **`cerebri-temporal`** | trusted instants, zones, bounded recurrence, availability and interval relationships |
| **`cerebri-constraints`** | facts, hard rules and structured violations |
| **`cerebri-semantics`** | semantic evidence vectors and derived deadline metadata |
| **`cerebri-preferences`** | interpretable preference precedence, deterministic ranking observations and storage ports |
| **`cerebri-ml`** | model/dataset/training metadata types and inference ports; no inference/training backend |
| **`cerebri-planner`** | CPIR, scope, validation, compilation, search and lifecycle proofs |
| **`cerebri-core`** | synchronous planning/temporal facade and JSON bridge |
| **`cerebri-integrations`** | execution/ledger ports and in-memory contract adapters |
| **`apps/cerebri-api`** | HTTP transport; no public execution endpoint |
| **`bindings/node`** | provisional asynchronous process bridge to the same Rust core |
| **`apps/cerebri-lab`** | Svelte developer/research workspace for real structured outputs |
| **`apps/cerebri-site`** | static official site built from repository docs and Rust-generated fixtures |
| **`research/ml-from-scratch`** | isolated research/learning material outside the production workspace |

Production code does not depend on `research/`. The Rust core remains the domain and planning authority; transports and presentation layers consume its typed outputs.

---

## Documentation and contracts

Start here:

- [Documentation map](docs/README.md)
- [Master Specification 0.4](docs/architecture/specifications/master-v0.4.md)
- [Architecture decisions](docs/architecture/decisions/README.md)
- [Planner integration](docs/en/planner-integration.md)
- [CPIR reference](docs/en/cpir.md)
- [Temporal reference](docs/en/temporal.md)
- [Foundation architecture](docs/en/foundation.md)
- [API reference](docs/en/api.md)
- [Roadmap](docs/development/roadmap/README.md)
- [Testing standard](docs/testing/testing-visualization-research-standard.md)
- [Documentation and release standard](docs/development/documentation-git-release-standard.md)

Language references:

- [English](docs/en/README.md)
- [Deutsch](docs/de/README.md)

Project governance:

- [Contributing](CONTRIBUTING.md)
- [Security](SECURITY.md)
- [Changelog](CHANGELOG.md)
- [AI-assisted development](docs/en/ai-assisted-development.md)

---

## Verification

Repository CI keeps major evidence streams independent. Current qualification includes Rust formatting/build/Clippy/tests, Rustdoc, repository policy checks, Lab checks/tests/build, site typecheck/build, browser/accessibility checks, visual verification and dependency audits.

The deterministic planner has additional bounded oracle/property-style coverage for schedules, dependency graphs, recurrence, malformed inputs and transport parity. This is extensive bounded verification, not formal verification.

See:

- [planner verification campaign](docs/development/progress/2026-09-21-planner-verification.md);
- [release qualification corrections](docs/development/progress/2026-09-22-release-qualification-corrections.md);
- [Ranking Feature Contract implementation record](docs/development/progress/2026-09-25-ranking-feature-contract.md);
- [ranking import parity correction](docs/development/progress/2026-09-26-ranking-import-parity-correction.md).

---

## Next work

Software `0.2.0` remains unpublished while release qualification continues.

The next Intelligence step is bounded architecture/contract work for Slice 2. Slice 2 implementation, learned ranking/search, EvaluationEpisode implementation, provider integration, advanced repair and autonomous mutation are not part of the current state.

---

<div align="center">

**Nexus Cerebri**

Temporal planning · deterministic foundations · future learned intuition behind explicit verification boundaries

<br>

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-000000?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Open Source](https://img.shields.io/badge/Open%20Source-MIT-1f883d?style=flat-square&logo=opensourceinitiative&logoColor=white)](LICENSE)

<sub>Software 0.2.0 · Specification 0.4 · CPIR 0.2 (legacy 0.1) · REST /v1 · Unreleased</sub>

</div>
