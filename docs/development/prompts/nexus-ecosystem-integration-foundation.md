# NEXUS ECOSYSTEM — CEREBRI INTEGRATION FOUNDATION PROMPT

You are implementing the first bounded Nexus consumer of Cerebri's application integration
contract. Communicate with the maintainer in German. Code, identifiers, commits and technical
artifacts are English. Implement the milestone below; do not stop at another architecture plan.

## Mission and current boundary

Prepare Nexus to request deterministic time suggestions for one prospective focus Event
linked to an existing Task. Flux is the first intended entry point; the application planning
boundary must also be usable by Tasks or Calendar without copying a planner into those views.

Implement **N1: offline adapter, host boundary and synthetic end-to-end conformance**.
Do not activate a production feature, add a default user-facing workflow or mutate product
state. A developer test/harness must call the real Rust bridge and decode its proposals.
Finish this milestone, report evidence, and stop before rollout or execution.

## Repositories and starting evidence

Canonical workspaces observed on 2026-09-30:

- Nexus Ecosystem: `F:\Coding\Nexus Workspace\Nexus-Ecosystem`, `dev`, inspected HEAD
  `10f8684c2da530ddc5549f2298f2794123a093d7`.
- Cerebri: `F:\Coding\Nexus Workspace\Nexus Cerebri Project`, integration implemented on
  `codex/application-integration-foundation`, based on
  `f3a1726` (merge of Lab PR #45). Use the reviewed commit containing this prompt and its
  contract, not that base commit alone. Resolve and record its full SHA before work.
- NexusAPI is outside N1; no backend/provider or persistence change is required.

These are discovery records, not instructions to reset either checkout. Recheck HEAD,
branch, worktree/index status and applicable AGENTS instructions. At the latest inspection,
Nexus had an in-progress merge with conflicts in `package.json` and architecture status
documents; preserve that work. If still unresolved, report the canonical workspace conflict
and wait for its owner before implementation. Do not reset, resolve unrelated conflicts,
silently create a substitute clone/worktree, or adopt another agent's edits.

Read Cerebri's current Master, ADR index and relevant Accepted ADRs with amendments,
especially 0002, 0004–0006, 0008–0009 and 0012–0015. Also read the proposed ADR-0016,
`docs/en/application-integration.md`, `crates/cerebri-core/src/integration.rs`,
`bindings/node/integration.mjs`, its declarations/tests, and
`examples/integration/suggestion.json`. ADR-0016 has implementation evidence but independent
Architecture/Security qualification is pending. Do not claim that this prompt supplies it.

Inspect Nexus's current architecture/status documents and actual active entry points.
The inspected product model was:

- `Nexus Main/src/store/appStore.ts`: Task has title/description, status, priority, optional
  deadline, workflow dependencies and timestamps; no authoritative occupied interval or
  focus duration. Reminder has a notification datetime and repeat setting, not busy duration.
- `Nexus Main/src/views/FluxView.tsx`: activity and operational task/reminder queues,
  urgency/focus filters, triage and navigation. It is not an existing temporal solver.
- `Nexus Main/src/views/CalendarView.tsx`: projects Task deadlines and Reminder times.
  Its rendered entries are not proof of complete free/busy coverage.
- `packages/nexus-core/src`: shared platform-neutral domain utilities/contracts.
- `Nexus Main/package.json`: active desktop entry is `electron-main.cjs`. Trace its actual
  IPC registration and preload; do not wire only the similarly named legacy TypeScript entry.
  Inspect `Nexus Main/preload.cjs` and the existing IPC tests/guards.
- `Nexus Mobile` shares core concepts but cannot spawn the Node bridge. Browser/mobile
  must report capability unavailable until a separately designed transport exists.

## Ownership and architecture

Keep Flux triage, priorities, completion actions, product state, permissions, navigation,
notifications and storage in Nexus. Do not replace severity sorting with Cerebri scheduling.
Keep generic interval arithmetic, recurrence expansion, constraint satisfaction, search,
candidate ordering and evidence in Rust. The adapter maps; it does not solve.

Use a small shared planning module, initially under `packages/nexus-core/src/planning/`
if current repository conventions still support it. Suggested responsibilities:

1. Product intent/snapshot/result contracts, independent of React, Electron, Node and stores.
2. Pure Task-context-to-CPIR adapter with explicit missing-information results.
3. Decoder from the subset of Rust result fields Nexus presents into a product view model,
   retaining original candidate order and evidence references.
4. A minimal async planning port with cancellation and unavailable/error outcomes.

Inject the port into Nexus Main's integration service. Keep process spawning and trusted
configuration in the desktop host. A renderer requests a narrow intent; it cannot choose
the executable, shell arguments, principal, write grants or arbitrary CPIR authority.
Read/plan permission and context visibility must come from the trusted application boundary.
Use existing sender/frame and payload guards if exposing a desktop IPC channel. A fake
transport is valid for unit tests only; conformance must exercise the actual Rust process.

Do not build an all-purpose provider registry, cross-app scheduler, data pipeline or a
second state store. Extract a small Flux selector only if it is needed for the adapter;
preserve existing behavior and test it. No wholesale Flux rewrite is justified by N1.

## Exact Cerebri wire contract

First query `--describe-integration` (or `describeIntegration`). Require:

```json
{
  "integration_version": { "major": 0, "minor": 1 },
  "profile": "single_event_suggestion",
  "cpir_schema_version": { "major": 0, "minor": 2 },
  "operations": ["FIND_SLOT"],
  "deployment_modes": ["Test", "Shadow", "Suggestion"],
  "temporal_coverage_required": true,
  "execution_supported": false,
  "max_request_bytes": 262144
}
```

The real manifest also has `software_version`. Do not infer compatibility from it. Integration
0.1, REST v1, CPIR 0.2, ranking features 0.1, evaluation schema and release state are independent.
Exact version matching is required. Manifest compatibility is not binary authentication;
record the reviewed source SHA and built artifact checksum in development evidence.

Call `cerebri-node-bridge --suggest`, pass one JSON envelope on stdin, close stdin, and read
one JSON response. The envelope is `{ "integration_version": {"major":0,"minor":1}, "request": CPIR }`.
The reviewed Node client is `bindings/node/index.mjs`; prefer using it through a trusted,
pinned development source dependency. Do not commit machine-specific absolute import or
package paths. For the N1 harness, a host-owned configuration may identify the reviewed
Cerebri checkout/module and built binary. Reproducible release packaging is a later slice.
If existing Nexus packaging requires a thin local transport wrapper, mirror only this
protocol and its bounds; never copy planning logic or import Cerebri Lab/research.

Node options: trusted `binary`, `signal`, and `timeoutMs` in 1..60000 (default 10000).
The new client caps input at 256 KiB and output at 16 MiB, kills on cancellation/timeout,
and returns sanitized `CerebriBridgeError` codes. This deadline is not a performance SLO.
Bound concurrent calls at the host; do not start an unbounded child for each renderer event.
Disable the feature on incompatible/missing binary; do not silently synthesize fallback slots.

REST has equivalent `/v1/integration/manifest` and `/v1/integration/suggestions` routes,
but is still a loopback development API without production service isolation/authentication.
Do not expose it publicly or silently use it as the browser/mobile deployment strategy.

## First use case and mapping

Intent: “Find a possible focus block for this Task inside my explicitly selected window.”
The Task remains a Nexus Task. Create only a prospective CPIR Event identity for the proposed
block and maintain a transient mapping to the source Task ID. No provider object is invented.

| Nexus input/evidence | CPIR mapping and rule |
| --- | --- |
| Explicit request and trace identities | `request_id`, `trace_id`; correlate responses to the active generation |
| Trusted user/account | `principal_id`; never trust a renderer-supplied principal or grant |
| Explicit window endpoints | `scope.time_range` as unambiguous UTC instants, half-open `[start,end)` |
| Explicit IANA timezone | Target `timezone`; preserve it while resolving instants |
| User-confirmed work duration | `duration` as `RESOLVED/KNOWN`, seconds, `USER_EXPLICIT`; no default from title/priority/deadline |
| Prospective block ID | One `target_ids` entry and Event in `context.objects`, `revision: null`, time `RESOLVED/MISSING` |
| Existing busy records | Separate established Event intervals with source revisions and fact provenance; no reminder duration guesses |
| Visibility and scope | Host-selected IDs; omitted/null != empty array; scope never grants authority |
| Context identity | Adapter-owned snapshot revision with explicit mapping to source revisions/content; capture time is not a revision |
| Provider/collection coverage | Explicit `context.temporal` with matching horizon, bounded limits, coverage and `series`; an empty list is not completeness |
| No execution | `FIND_SLOT`, scope and policy mutation maxima `0`, allowed actions `[]`, mutation grants `[]`, Test or Suggestion mode |
| Read/plan permission | Trusted `planning_capability.read` and `.plan`; deny if not established |
| Duration policy | `allow_uncertain_duration: false`; preserve confirmation requirements conservatively |
| Optional explicit preference | Supported preferred-start value and provenance; do not turn Flux severity into a hidden score |
| Task workflow dependencies | Keep as Nexus workflow readiness unless actual temporal predecessor intervals establish `DependencyOrder` semantics |

N1 accepts already resolved instants. If product input is date-only, timezone is absent, or
local time is in a DST gap/fold without an explicit resolution, return a typed clarification
state before calling Cerebri. Do not implement a new timezone solver or guess midnight.
The existing Task deadline can constrain a latest end only when its actual semantics and
instant are explicit; it never establishes occupied time. Reminder recurrence is not an
occupied recurring event. Do not convert monthly reminders into unsupported Cerebri series.

Preserve Missing, Unknown, Uncertain and Ambiguous knowledge states. Marking coverage Complete
requires an actual collection contract; current Nexus Task/Reminder views alone cannot supply
one. N1 uses synthetic certified busy windows for conformance. Real product context lacking
coverage must produce clarification/InsufficientInformation, not a fabricated successful demo.
No automatic persistence or fake provider is permitted to conceal this gap.

Use safe integer checks for Rust integer fields crossing JavaScript. Reject unsafe revisions,
budgets and duration values; do not round identifiers or timestamps. Keep source strings/titles,
descriptions, reminder bodies and note content out of CPIR unless necessary for an explicitly
supported constraint. N1 does not need them.

## Result and stale-state handling

The outer `status: "rejected"` has a stable profile/decoding code. `status: "planned"` only
means Rust returned; it contains `request_id`, `trace_id`, `context_revision`, and the unchanged
PlanningResult. Decode fields, do not cast JSON blindly. The Node declarations type the
envelope and intentionally leave PlanningResult as JSON; Nexus must validate consumed fields.

Map these distinctions explicitly:

- Solution: non-executable candidate times, kept in Rust order.
- NoSolution: no feasible candidate in the declared search space; show actual assessment.
- InsufficientInformation: preserve structured reasons, especially incomplete coverage.
- NeedsRelaxation: keep distinct even if uncommon in this bounded baseline; do not auto-relax.
- BestFound: bounded search, never label optimal. ProvenOptimal is relative to the stated grid.
- Profile rejection: incompatible/invalid request, separate from no solution.
- Timeout, cancellation, unavailable binary, malformed/mismatched response: transport failure,
  no partial result and no product mutation.

Carry candidate identity, placements, source revision and explanation/evidence necessary for
presentation. Preserve ordering/feature evidence; do not recompute costs or perform client-side
constraint repair. Source Task/context changes, permission changes, sign-out, new intent or
window change invalidate pending/results. Abort old work and discard late replies even if
transport cancellation races with completion. A request counter is correlation, not a provider
revision or authorization proof.

Selecting a candidate does not create an Event, complete a Task, snooze a Reminder or update
a deadline. The Rust analysis-only proposal cannot become ActionPlan. Future execution requires
a separate fresh request and the complete validation/authorization/preflight lifecycle.

## Required tests and evidence

Add meaningful tests to the existing Nexus runners; do not introduce unused test infrastructure.

1. Pure mapping: explicit known window/duration/zone and zero mutation contract; immutable
   source Task; distinct prospective identity; correct scope null/empty behavior.
2. Missing duration, missing zone, date-only deadline, ambiguous local time, and unsupported
   recurrence never become guessed facts. Incomplete real context never becomes Complete.
3. Workflow dependency without established scheduled predecessor remains workflow readiness;
   no invented temporal edge. Reminder timestamps never become busy intervals by default.
4. Real Rust conformance using the shared synthetic fixture: exactly seven starts at
   10:00, 10:15, 10:30, 10:45, 11:00, 11:15, 11:30 UTC on 2026-10-01; same Rust ordering.
   Add complete busy horizon -> NoSolution; Incomplete -> no candidates; bounded budget ->
   BestFound; scope exclusion and denied read/plan -> InsufficientInformation.
5. Retain recurrence/dependency/DST evidence using Cerebri's existing synthetic planner
   fixtures where the N1 mapper supports them. Do not claim unsupported adapter coverage.
6. Wrong manifest/profile/version, malformed output, unsafe numeric fields, oversized input/
   output, unavailable process, cancellation and timeout fail without raw content in errors.
7. Stale/out-of-order response is discarded. Off/unavailable mode starts no process. Switching
   off or disposing the owner cancels in-flight work. Browser/mobile report unavailable.
8. If IPC is added, verify sender checks, payload bounds, host-selected context and executable,
   concurrency limits, and renderer inability to supply write grants or arbitrary commands.
9. Regression: existing Flux triage, Task completion, Reminder snooze and Calendar projections
   are unchanged; no product store write occurs merely from requesting or selecting suggestions.

Run current relevant core/type/desktop/IPC tests and builds. The inspected shared package
provides `npm --prefix packages/nexus-core run typecheck`, `test` and `build`; inspect current
scripts before running. `Nexus Main` has `test:release-ui` and a build with release-UI checks.
Run affected browser checks if changing any UI. Record actual commands, results, inspected
source SHAs, binary checksum and measured conformance timings; do not invent latency targets.

## Documentation, privacy, workflow and Definition of Done

Add a Nexus architecture decision for the shared planning boundary, mapping reference,
developer setup, compatibility pin and dated evidence. Document the absence of authoritative
product free/busy data and the next required provider/explicit-availability decision.
Record ADR-0016's review status honestly. No release/version/tag change is authorized.

Do not collect personal planning episodes, interaction events or model data. No raw CPIR,
titles, descriptions or child stderr in telemetry. Synthetic fixture artifacts stay synthetic.
Cerebri's evaluation Phase B is partial and does not authorize production observations,
replay, retention or learning. No ML or research imports.

Use one canonical workspace and a bounded branch following repository conventions. Preserve
unrelated work. Inspect the complete diff, use coherent Conventional Commits, push and inspect
the actual remote CI for the commit. Attach the resulting PR. Do not force-push, auto-publish,
or claim independent Architecture/Math/Security review yourself.

Done means the pure adapter and host seam are implemented, a real Rust end-to-end synthetic
test passes, unavailable/stale/error paths are honest, no product state is changed, relevant
checks pass, and the reviewed contract/setup is documented. Rollback switches the integration
off without database migration. Full UX, production shadow/suggestions, provider integrations,
authorized execution, evaluation collection and learning require later bounded milestones.

Final response in German: exact implemented scope and entry points, commit/PR, checks and
remote CI result, demonstrated conformance, remaining qualification and data gaps, and exactly
one recommended next slice. Do not equate “adapter implemented” with “production integration complete”.
