# Cerebri Lab

An internal Svelte 5 + TypeScript workspace for inspecting Nexus Cerebri. The UI
uses rounded glass panels, restrained cyan/violet accents, light/dark themes,
responsive navigation and reduced-motion support. It is research/developer
tooling; its UI contracts are not a stable consumer API.

## Setup

From the repository root, with Node.js **22.12 or newer**:

```sh
npm --prefix apps/cerebri-lab ci
npm --prefix apps/cerebri-lab test
```

## Run

Before starting the vite server run this to setup the api:

```sh
cargo run -p cerebri-api
```

From the repository root, with Node.js **22.12 or newer**:

```sh
npm --prefix apps/cerebri-lab run dev 
```

Open [the built Lab](http://127.0.0.1:3000/lab/). The API serves
`apps/cerebri-lab/dist` at `/lab/`. The Vite base is `/lab/`; the build produces
`dist/index.html` and content-hashed JavaScript/CSS under `dist/assets/`.
Build output and dependencies are ignored and must not be committed.

For UI development, keep the API running and use:

```sh
npm --prefix apps/cerebri-lab run dev
```

Vite serves [the development Lab](http://127.0.0.1:5173/lab/) and proxies `/v1`
and `/health` to `http://127.0.0.1:3000`. There is no frontend planning engine.
The optional `preview` command only previews assets; use the API server for a
functional production-build review.

## What works

- **Planner:** loads the canonical synthetic CPIR fixture with an explicit
  10:45 UTC preferred start, imports CPIR, calls `/v1/validate` or `/v1/plan`,
  inspects candidate timelines and costs, and exports the complete result.
- **Temporal:** loads the canonical DST fixture or an imported temporal request,
  calls `/v1/temporal`, and displays the returned busy/free/unknown intervals,
  recurrence occurrences, skips, timezone resolution and buffer/clipping traces.
  A coverage selector explicitly changes the request's collection completeness.
- **Trace:** shows request validation, declared search-space metadata and typed
  rejection reasons. It does not parse human-readable logs.
- **Semantics / Preferences:** inspect supplied CPIR evidence and actual score
  components. Semantic inference and preference learning remain deferred.
- **Console:** timestamped Lab events and actual core responses, retained only in
  session memory (latest 40 entries). Structured payloads remain inspectable.
- **ML / Dataset:** clearly labelled reserved workspaces. The model console is
  inactive; there is no fabricated inference, training curve or evaluation score.

Simple mode emphasizes readable views; Technical mode exposes structured
contracts; Research mode opens raw details and explains the bounded search proof.
All charts expose their input data. Timeline timestamps are UTC; recurrence
tables also preserve the core's original local dates, times and timezone.

Imported results are not treated as proof of execution authorization. Their source
request is not assumed to match the current input. The planner timeline displays
at most eight candidates; the cost chart displays twelve. Trace tables display
64 rejections; recurrence tables display 100 occurrences and 100 skips per rule.
Exports always retain the full response.

## Structure and boundaries

```text
src/
  App.svelte                 workspace state and application events
  components/                focused navigation, input, charts and inspectors
  lib/contracts.ts           internal CPIR and planner transport shapes
  lib/transport.ts           import guards, HTTP and JSON export
  lib/temporal.ts            temporal diagnostic transport shapes and guards
  lib/presentation.ts        labels and viewport-only chart geometry
  styles.css                 shared theme and responsive layout
tests/transport.test.ts      malformed imports, knowledge and transport checks
```

Transport guards verify the shape and representability needed for display, not
domain validity. The Rust core remains the sole authority for CPIR validation,
planning, recurrence expansion and free/busy computation. No executor route is
exposed. All synthetic inputs are imported from `examples/`, avoiding duplicated
fixture authorities. Test-only transport fixtures are never shown as planner runs.

Requests are capped at the API's 256 KiB body limit. Result imports are capped at
8 MiB. Requests, console entries and results are not persisted in local storage;
exports are explicit. The UI sends data only to the same-origin local API and
loads no fonts, telemetry, images or assets from external services.

## Verification

```sh
npm --prefix apps/cerebri-lab run check
npm --prefix apps/cerebri-lab test
npm --prefix apps/cerebri-lab run build
```

`check` fails on Svelte/TypeScript errors and warnings. The Node test runner checks
transport failure behavior, malformed inputs, preservation of epistemic states,
unknown availability, explicit core rejections, and viewport-only clipping.
Browser review should also run a demo, select candidate bars, inspect Trace and
Temporal (both coverage settings), test light mode and a narrow viewport, and
confirm that the model console remains inactive.

Nexus Cerebri is an open-source project built with human and AI collaboration.
Human review, inspectable evidence and reproducible checks remain part of the
development process. See the [development guide](../../docs/en/development.md).

## Planner integration (CPIR 0.2)

The source-scenario selector loads checked-in CPIR inputs. Compiler panels show actual Core
occurrence identities, provenance, full/clipped ranges, DST skips and busy/free/unknown
coverage. The dependency graph displays Core order and cycle/missing-reference evidence.
Candidate details expose every lexicographic ordering component. No UI planner is implemented.

The Lab tests build and start the Rust API on a private loopback port and exercise the
shared scenario manifest plus negative contract mutations. Rust is therefore required for
`npm test`. See [planner integration](../../docs/en/planner-integration.md).
