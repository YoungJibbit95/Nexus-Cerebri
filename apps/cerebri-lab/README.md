# Cerebri Lab

An internal Svelte 5 + TypeScript workspace for inspecting Nexus Cerebri. The UI
uses visual planning instruments, a shared semantic grammar, light/dark themes,
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
  explains the returned search field, constraint rejections and lexicographic order,
  synchronizes candidate selection across instruments, and exports the complete result.
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

The persistent global control offers **01 Understand** (internal Simple mode),
**02 Technical** and **03 Research**. Understand shows the request sentence, real
time geometry, run counts, preference-distance comparison and proposal boundary;
raw JSON is absent and session activity starts collapsed. Technical adds exact
intervals, returned ordering-key comparison and expandable evidence. Research opens
complete request/result JSON, feature observations, source inspectors and the console.
Depth changes presentation only; they do not send a new planning request. Only the
depth choice is persisted, under `cerebri-lab-depth`; disabled storage is supported.
All charts expose their input data. Timeline timestamps are UTC; recurrence
tables also preserve the core's original local dates, times and timezone.

Imported results are not treated as proof of execution authorization. Their source
request is not assumed to match the current input. The planner timeline displays
at most eight candidates plus a selected candidate outside that prefix. Request facts,
occurrences and supplied preferences are bounded to eight each, with up to eight extra
blockers of each interval type; rejection markers are bounded to 48. Every candidate
and rejection remains selectable. Technical tables bound rejections to 64 and
occurrences/skips to 100; Research tables and JSON preserve all supplied rows.
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
Component tests additionally render every depth, verify source-data preservation,
shared selection, returned counts, provenance, complete Research disclosure and
cancellation/reduced-motion semantics. The optional browser check uses the existing
site Playwright and axe tooling (install site dependencies and Chromium first):

```sh
cargo build -p cerebri-api --locked
node apps/cerebri-lab/tests/browser-check.mjs
```

It starts a private loopback API serving the production Lab build, checks real API
counts and data parity, selection, keyboard focus, dark/light WCAG checks, 1600/
1366/820px layouts, DST and both coverage settings, depth persistence, export,
scenario loading, reduced motion and source-less result import. Screenshots default
to the system temporary directory; set `LAB_EVIDENCE_DIR` to retain them elsewhere.

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

## L1/L2 presentation checkpoint

The old summary-card grid and isolated cost chart are replaced by a scene-based
workbench: input → completed-run summary → authority rail → semantic time field →
deterministic comparison → selected-placement inspector. The Rust result and its
exact request snapshot remain the only planning data. Imported results do not
borrow facts or preferences from the current editable request.

One cancellable playback controller explains a completed response in six stages
at 600ms intervals (3 seconds total), with replay, skip and direct step selection.
Result data is available immediately. Changing depth, candidate, result or view
cancels obsolete playback. Reduced motion snaps to the same final semantic state.
The sequence is explicitly a post-run explanation, never an internal runtime trace.
CSS uses 180ms micro and 450ms state tokens; colors that establish text contrast
change together. There are no continuous loops or external animation dependencies.

The shared grammar distinguishes facts, recurrence, preferences, rejections,
candidates, first proposal and unknown coverage using text, glyphs, borders and
fills. The first-ranked proposal remains separate from the selected alternative.
Keyboard controls, pressed states, visible focus and a concise screen-reader
selection announcement accompany DOM diagrams. No execution route is added.

The global depth/disclosure layer also applies to existing Temporal, Trace,
Semantics, Preferences, Compilation and Dependency inspectors. Their dedicated
L3/L4 visual stories and the broader L5 polish campaign remain future work.
See the [dated checkpoint](../../docs/development/progress/2026-09-30-lab-l1-l2-checkpoint.md).
