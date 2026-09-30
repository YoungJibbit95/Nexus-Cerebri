# Lab L1/L2 visual presentation checkpoint

Date: 2026-09-30. Internal implementation/handoff record; non-normative.

## Starting state and bounded delivery

- Canonical workspace: `F:/Coding/Nexus Workspace/Nexus Cerebri Project`.
- Fetched starting `origin/main`: `510e17314af6ab76d3da5690af6f274ef46da2e2`, exactly the expected prompt baseline. No main drift at start.
- Branch: `codex/lab-three-depth-visual-experience`.
- Starting remote CI was inspected: [Cerebri CI run 36695787816](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/36695787816) failed its four website homepage visual hashes. Lab checks/tests/build and Browser / E2E passed. This baseline must not be described as wholly green.
- Work is confined to Lab presentation, its tests/README and this record. The pre-existing `.gitignore` change is preserved and excluded from the commit.
- Rust planner, CPIR/evaluation/ranking/temporal contracts, Slice 2, website code/baselines, dependencies, software version and publication state are unchanged.

The implementation follows the current Master and applicable Accepted ADRs, including
ADR-0010, ADR-0012 and ADR-0014. This record supplies implementation evidence; it does
not self-certify independent Architecture, Math or Security qualification.

## Presentation before and after

| Concern | Before | L1/L2 checkpoint |
| --- | --- | --- |
| Depth | Small local display toggle | Persistent global Understand / Technical / Research control, shared disclosure context |
| Composition | Four summary cards, request/timeline/card grid, prominent console | Input beside an instrument scene; coherent result flow, early authority rail, time field, comparison, placement inspector |
| Request | Metadata card plus JSON disclosure | Sentence from actual request, horizon, duration ruler and supplied preference markers; metadata at greater depth |
| Ranking | Scalar cost bars | Whole-second preference-distance geometry in Understand; returned lexicographic keys and first-difference emphasis in Technical/Research |
| Selection | Existing App-owned candidate index | Same authority preserved and used consistently by field, comparison and inspector; selected candidates beyond the visual prefix remain visible |
| Diagnostics | Raw disclosures/tables across views | No JSON in Understand; expandable Technical evidence; complete Research JSON, tables and console |

One PlanningRequest/PlanningResult remains the truth. Candidate order, ranking features,
counts, rejections, compilation, coverage and graph data come from Rust. The presentation
never generates candidates, resolves effective preference precedence, computes planning
truth or invents a runtime trace. A supplied preference outside the horizon is labelled
outside rather than clamped into a false time position. Unattached imported results
never borrow source facts/preferences/semantics from the current editable input.

Understand keeps plain explanations, spatial time intervals and a collapsed activity
drawer. Technical adds exact boundaries, provenance, ordering components and expandable
inspectors. Research opens complete JSON, identifiers, feature/version observations,
source structures and the full console; its tables expose all supplied rows. Exports
preserve the complete response at every depth. Only depth is persisted in localStorage;
requests, results and logs remain in session memory. Storage denial is handled.

## Planner flow, motion and accessibility

The displayed flow is request/context → returned search field → constraint filter →
feasible candidates → deterministic comparison → proposal/authority boundary. The actual
default response remains **11 evaluated → 4 rejected → 7 feasible → #1 proposal**.
A selected alternative never replaces the first-ranked proposal's identity. Rejection
selection highlights returned blocking object/occurrence identities where intervals are
attached. Missing source information remains explicit. Empty completed runs say there
is no proposal rather than implying that a proposal is still pending.

A single cancellable scheduler explains a newly completed API response in six stages,
600ms apart (3 seconds). All result data is available immediately. Replay, skip and
direct selection are provided. Selection, depth changes, replacement results and view
teardown cancel obsolete work; returning to a view does not restart old playback.
No background loops or animation dependency is added. CSS uses 180ms micro and 450ms
state tokens. Dense ordering/constraint evidence is inspected manually. Reduced motion
snaps to the same final semantic state and disables CSS animation/transitions.

Facts, recurrence, preferences, rejected positions, candidates, first proposal and
unknown coverage share text/glyph/fill/border conventions. DOM controls expose pressed
states, explicit names, focus and a concise screen-reader selection announcement.
The authority rail says **Proposal ≠ execution. No calendar has been changed.**
Validation/authorization and execution preflight remain required beyond the Lab.
Contrast colors switch together. Solid panels avoid extra backdrop compositor layers;
this also corrected missing offscreen content in narrow full-page browser captures.

Visuals are bounded: 8 facts, 8 occurrences, up to 8 additional blockers of each type,
8 supplied preferences, 8 candidates plus selection, 48 rejection markers and 32
coverage intervals of each type. Full source data remains inspectable/exportable.
The existing dependency graph remains bounded to 24 nodes/64 visible edges.

## Executed verification

- `npm --prefix apps/cerebri-lab run check`: zero errors and warnings.
- `npm --prefix apps/cerebri-lab test`: 34 passing tests, including real Rust API/Node parity, all scenario-manifest expectations, malformed imports and 9 new presentation/component tests.
- `npm --prefix apps/cerebri-lab run build`: production assets built successfully with the existing `/lab/` base.
- `cargo build -p cerebri-api --locked`: passed.
- `npm --prefix apps/cerebri-lab audit --audit-level=high`: zero vulnerabilities.
- `npm run check`: repository/version authority checks passed.
- `node apps/cerebri-lab/tests/browser-check.mjs`: production Lab served by a private real Rust API; returned counts/data parity, synchronized selection, blocker highlighting, keyboard focus, dark/light axe WCAG checks, 1600/1366/820px widths, DST/Complete/Incomplete coverage, persistence, no-solution scenario, full JSON export, reduced motion and source-less import passed.
- `git diff --check`: passed before commit.

Browser checks use the already installed site Playwright/axe development tooling; the
Lab gains no dependency. The production build and loopback routes used by Electron
are preserved. A packaged Electron installer launch was not tested in this checkpoint.
No independent Architecture/Math/Security certification is asserted here. Actual final
remote CI status is reported with the PR delivery after pushing the commit.

## Screenshot evidence

Local evidence directory:
`C:/Users/Administrator/.codex/visualizations/2026/09/30/01a0f1cf-f7aa-7f01-be80-fedf25cbe018/lab-l1-l2`.
Captures are inspectable local artifacts, not new website visual baselines. They can be
regenerated with `LAB_EVIDENCE_DIR` and the browser check. Finite visual transitions are
finished for stable screenshots; live semantic interaction is checked separately.

Inspected states:

- `planner-before-run.png`, `planner-understand.png`, `planner-technical.png`.
- `planner-research.png`, `planner-research-data.png` (complete structured data).
- `planner-selected-alternative.png`, `planner-selected-rejection.png` (blocking fact highlighted).
- `planner-1366.png`, `planner-820.png`, `planner-820-comparison.png`, `planner-820-proposal.png`, `planner-light.png`.
- `planner-no-solution.png`.
- `temporal-dst.png`, `temporal-incomplete.png`.
- `preferences.png`, `trace.png`.

The selected-rejection interaction belongs to the new Planner field. The dedicated
Trace pipeline/rejection traversal is L3 work, not claimed complete by its legacy
Trace screenshot. The preferred-start marker/distance instrument belongs to the Planner;
the Preferences screenshot verifies retained source precedence and disclosure.

## Remaining bounded Lab work

- L3: dedicated DST local-gap/fold story, recurrence fan-out/materialization scene,
  full trace pipeline and cross-view rejection traversal.
- L4: richer preference precedence/distance workspace, knowledge/provenance visual
  grammar and dependency path/cycle/topological interactions.
- L5: broader empty/error/large-input and desktop packaging campaign, motion timing
  polish and extended visual/accessibility coverage. Necessary responsive/reduced-motion
  support for L1/L2 is implemented and checked now.

ML/Dataset remain NOT CONNECTED / PLANNED / DEFERRED. No learning, training or execution
is introduced. Stop at this checkpoint; no auto-merge, Codex review request or release.

## Exact changed files

- `apps/cerebri-lab/README.md`
- `apps/cerebri-lab/src/App.svelte`
- `apps/cerebri-lab/src/components/AuthorityRail.svelte`
- `apps/cerebri-lab/src/components/CandidateDetail.svelte`
- `apps/cerebri-lab/src/components/CompilationPanel.svelte`
- `apps/cerebri-lab/src/components/DependencyPanel.svelte`
- `apps/cerebri-lab/src/components/DepthControl.svelte`
- `apps/cerebri-lab/src/components/InspectorDrawer.svelte`
- `apps/cerebri-lab/src/components/InspectorViews.svelte`
- `apps/cerebri-lab/src/components/JsonPanel.svelte`
- `apps/cerebri-lab/src/components/OutputConsole.svelte`
- `apps/cerebri-lab/src/components/PlannerScene.svelte`
- `apps/cerebri-lab/src/components/RequestPanel.svelte`
- `apps/cerebri-lab/src/components/ScoreChart.svelte`
- `apps/cerebri-lab/src/components/SemanticLegend.svelte`
- `apps/cerebri-lab/src/components/TemporalView.svelte`
- `apps/cerebri-lab/src/components/Timeline.svelte`
- `apps/cerebri-lab/src/lib/depth.ts`
- `apps/cerebri-lab/src/lib/motion.ts`
- `apps/cerebri-lab/src/lib/planner-presentation.ts`
- `apps/cerebri-lab/src/styles.css`
- `apps/cerebri-lab/tests/browser-check.mjs`
- `apps/cerebri-lab/tests/presentation.test.ts`
- `docs/development/progress/2026-09-30-lab-l1-l2-checkpoint.md`
