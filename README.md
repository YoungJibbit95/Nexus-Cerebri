<div align="center">

# 🧠 Nexus Cerebri

**A Rust temporal planning foundation built around neural intuition + symbolic verification.**

<br>

[![Software](https://img.shields.io/badge/software-0.2.0-0969da?style=for-the-badge)](#)
[![Specification](https://img.shields.io/badge/specification-0.4-8250df?style=for-the-badge)](#)
[![CPIR](https://img.shields.io/badge/CPIR-0.2-1f883d?style=for-the-badge)](#)
[![Release status](https://img.shields.io/badge/status-unreleased-d29922?style=for-the-badge)](#)
[![REST](https://img.shields.io/badge/REST-%2Fv1-d1242f?style=for-the-badge)](#)

[![Rust](https://img.shields.io/badge/Rust-1.97.0-000000?style=flat-square\&logo=rust\&logoColor=white)](https://www.rust-lang.org/)
[![Node.js](https://img.shields.io/badge/Node.js-22.12%2B-339933?style=flat-square\&logo=nodedotjs\&logoColor=white)](https://nodejs.org/)
[![Svelte](https://img.shields.io/badge/Svelte-Lab-FF3E00?style=flat-square\&logo=svelte\&logoColor=white)](https://svelte.dev/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](LICENSE)

[![CI](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/workflows/ci.yml/badge.svg)](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/workflows/ci.yml)
[![GitHub stars](https://img.shields.io/github/stars/YoungJibbit95/Nexus-Cerebri?style=flat-square\&logo=github)](https://github.com/YoungJibbit95/Nexus-Cerebri/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/YoungJibbit95/Nexus-Cerebri?style=flat-square\&logo=github)](https://github.com/YoungJibbit95/Nexus-Cerebri/forks)
[![GitHub issues](https://img.shields.io/github/issues/YoungJibbit95/Nexus-Cerebri?style=flat-square\&logo=github)](https://github.com/YoungJibbit95/Nexus-Cerebri/issues)
[![GitHub last commit](https://img.shields.io/github/last-commit/YoungJibbit95/Nexus-Cerebri?style=flat-square\&logo=github)](https://github.com/YoungJibbit95/Nexus-Cerebri/commits/main)
[![GitHub repo size](https://img.shields.io/github/repo-size/YoungJibbit95/Nexus-Cerebri?style=flat-square\&logo=github)](https://github.com/YoungJibbit95/Nexus-Cerebri)
[![Repo Views](https://hits.sh/github.com/YoungJibbit95/Nexus-Cerebri.svg?style=flat-square&label=views)](https://hits.sh/github.com/YoungJibbit95/Nexus-Cerebri/)

</div>

---

> **Software:** `0.2.0`
>
> **Release status:** `Unreleased`
>
> **Release qualification:** `Undergoing release qualification`
>
> **Published release:** `None`
>
> **Specification:** `0.4`
>
> **Current CPIR:** `0.2`
>
> **Legacy CPIR:** `0.1`
>
> **REST:** development routes under `/v1`.

These are separate version domains. No formal release candidate, version tag or software release
has been published. See [version authorities](docs/development/documentation-git-release-standard.md).

A Rust temporal planning foundation built around **neural intuition + symbolic verification**.
The current implementation is deterministic; neural inference is a future milestone.

---

## ✨ Working today

* Validated identifiers, confidence and explicit knowledge/processing states.
* IANA timezones, half-open interval algebra, bounded daily/weekly recurrence with explicit DST policies.
* Completeness-aware free/busy, buffer/travel calculations and typed temporal traces.
* CPIR validation, bounded scope, facts, hard constraints and interpretable preferences.
* Single-event grid search with deterministic ordering, structured explanations and honest search assessments.
* Identified recurrence occupancy, dependency graph/cycle evidence and explicit candidate ordering keys ([integration reference](docs/en/planner-integration.md)).
* Strong plan lifecycle, policy/capability/confirmation checks, and mock execution with freshness, replay protection and per-action results.
* REST facade, provisional Node process bridge and a Svelte developer Lab with timelines, scores and structured output.
* Static SvelteKit official site with repository-backed documentation, Rust-generated planning/temporal fixtures, explicit truth labels and Understand/Technical/Research explanation depth.

> [!IMPORTANT]
> No provider integration, production authentication, durable ledger, advanced repair, learning or autonomous calendar mutation is implemented. Existing recurrence occupancy compiles into bounded planner snapshots; recurrence constraints and prospective series remain unsupported.

This MIT-licensed open-source project also uses AI development tools. Assisted contributions
must pass the same review, tests, safety, documentation and CI gates as every other contribution.
See the [AI development policy](docs/en/ai-assisted-development.md) and [license](LICENSE).

---

## 🚀 Quick start

Install Rust via rustup. The repository pins Rust 1.97.0 and its formatter/linter.
Node 22.12+ is needed for the Lab, Node bridge and documentation tooling.

```sh
cargo build --workspace --locked
cargo test --workspace --locked
cargo fmt --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo run -p cerebri-core --example plan
npm --prefix apps/cerebri-lab ci
npm --prefix apps/cerebri-lab run build
npm --prefix apps/cerebri-site ci
npm --prefix apps/cerebri-site run check
npm run docs:build
cargo run -p cerebri-api
```

> [!TIP]
> The development server binds only to [localhost:3000](http://127.0.0.1:3000).

Open [Cerebri Lab](http://127.0.0.1:3000/lab/) and load [the current CPIR 0.2 request](examples/request.json).
[CPIR 0.1](examples/legacy-cpir-0.1.json) is an explicitly supported legacy fixture.
Temporal diagnostics use [a Berlin DST fixture](examples/temporal-request.json).
For UI development use `npm --prefix apps/cerebri-lab run dev` alongside the Rust API.

<details>
<summary><strong>🛠️ Additional build, bridge and documentation commands</strong></summary>

<br>

```sh
cargo build -p cerebri-node
node --test bindings/node/test.mjs
npm ci --ignore-scripts
npm run check
npm run docs:build
npm --prefix apps/cerebri-lab run check
npm --prefix apps/cerebri-lab test
```

</details>

The official SvelteKit site is generated into ignored `site/`. Canonical documentation still comes directly from repository Markdown, while planner and temporal demonstrations are generated by executing `cerebri-core` during the build.
GitHub Pages deploys that static artifact from `main`; the site build handles the `/Nexus-Cerebri` base path and contains no runtime CDN or GitHub dependency.

---

## 🧩 Workspace

| Area                           | Responsibility                                                                    |
| :----------------------------- | :-------------------------------------------------------------------------------- |
| **`cerebri-types`**            | IDs, revisions, confidence, evidence and knowledge states                         |
| **`cerebri-temporal`**         | trusted instants, zones, bounded recurrence, free/busy and interval relationships |
| **`cerebri-constraints`**      | facts, hard rules and structured violations                                       |
| **`cerebri-semantics`**        | eight-dimensional evidence and derived deadline metadata                          |
| **`cerebri-preferences`**      | interpretable precedence, scoring features and storage ports                      |
| **`cerebri-ml`**               | production model/dataset/training metadata and inference ports                    |
| **`cerebri-planner`**          | CPIR, scope, validation, search and lifecycle proofs                              |
| **`cerebri-core`**             | synchronous planning/temporal facade and JSON bridge                              |
| **`cerebri-integrations`**     | execution/ledger ports and in-memory contract adapters                            |
| **`apps/cerebri-api`**         | HTTP transport; no execution endpoint                                             |
| **`bindings/node`**            | provisional asynchronous process bridge to the same Rust core                     |
| **`apps/cerebri-lab`**         | Svelte timelines, typed-result inspection and structured console                  |
| **`apps/cerebri-site`**        | static official site, repository docs and Rust-generated explanatory fixtures      |
| **`research/ml-from-scratch`** | reserved human learning path, outside the workspace                               |

---

## 📚 Documentation

### 🌐 Language & reference

* [Deutsch](docs/de/README.md) · [English](docs/en/README.md)
* [Documentation map](docs/README.md) · [Temporal Core](docs/en/temporal.md)
* [Master specification 0.4](docs/architecture/specifications/master-v0.4.md)

### 🏗️ Development & architecture

* [Implementation boundary](docs/development/agent-boundaries/implementation-boundary.md)
* [Testing standard](docs/testing/testing-visualization-research-standard.md)
* [Documentation/release standard](docs/development/documentation-git-release-standard.md)
* [Roadmap](docs/development/roadmap/README.md) · [Learning roadmap](docs/development/roadmap/development-learning-roadmap.md)

### 🤝 Project

* [Contributing](CONTRIBUTING.md) · [Security](SECURITY.md) · [Changelog](CHANGELOG.md)
* [Foundation decisions](docs/architecture/decisions/README.md) · [Official site ADR](docs/architecture/decisions/ADR-0011-official-site.md) · [Progress](docs/development/progress/2026-09-20-official-site.md)

> [!NOTE]
> Accepted ADRs explicitly amending the Master take precedence only with synchronized Master updates;
> then the current Master, domain standards, agent boundary and roadmap apply.
> Review-resolution and archived documents remain historical/non-normative.
> The new Foundation reference pages have DE/EN counterparts; the entire historical repository does not claim bilingual parity.

---

## 🎯 Next milestone

**Deterministic Planner release qualification:** independently review the bounded verification
campaign and tightened resource admission, then qualify release artifacts. Recurrence/snapshot
integration, dependency diagnostics and deterministic oracle tests are implemented; see the
[verification report](docs/development/progress/2026-09-21-planner-verification.md).

> Software version, specification revision, CPIR schema, REST version and future model/dataset versions remain separate authorities.

---

<div align="center">

### 🧠 Nexus Cerebri

**Temporal planning · deterministic foundations · neural intuition + symbolic verification**

<br>

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-000000?style=flat-square\&logo=rust)](https://www.rust-lang.org/)
[![Open Source](https://img.shields.io/badge/Open%20Source-MIT-1f883d?style=flat-square\&logo=opensourceinitiative\&logoColor=white)](LICENSE)

<sub>Software 0.2.0 · Specification 0.4 · CPIR 0.2 (legacy 0.1) · REST /v1 · Unreleased</sub>

</div>
