# LAB L3 + ELECTRON E1 checkpoint

Date: 2026-09-30. Bounded implementation evidence; no independent Architecture,
Math or Security qualification is asserted. No release publication is authorized.

## Base and reconciliation

PR [#45](https://github.com/YoungJibbit95/Nexus-Cerebri/pull/45) was verified merged
before modifications: head `e9ca9e6422f5bc13b1f77bd88c669869df80c63e`, merge/base
`f3a1726e5154bc5252e2a084fb3cff5e2e96f9ba`, merged 2026-09-30 11:39:24 UTC.
The new branch is `codex/lab-l3-electron-runtime`.

During local verification, separately owned Application Integration PR #47 and
Evaluation B.3 PR #46 merged. The branch was fast-forwarded to current main
`7947340f21a5b70975d9c1244e29766023aaab7c`, retaining bounded local changes.
Classification: **IMPLEMENTATION-MECHANICAL RECONCILIATION**. Evaluation files do
not overlap this implementation. The additive API static-directory wrapper
preserves the integration routes and body limit from PR #47. Those separate
implementations are not reviewed or requalified here.

During evidence collection the maintainer merged Evaluation B.4 PR #48, merged
main into PR #49 at `4a244d179a7c8a2b2a602c410017278526bbd8fd`, then merged PR #49
on 2026-09-30 13:08:20 UTC as `7fbab3daa97a0c577234b6d5845dc7144ebccd8f` and
deleted its remote branch. This additional drift is also
**IMPLEMENTATION-MECHANICAL RECONCILIATION**: no Lab, API, desktop workflow or
gitignore file differs from tested runtime commit `b07afd5`. The final evidence
update therefore uses `codex/lab-l3-electron-evidence` from that merged main and
changes only this checkpoint document. It does not recreate the deleted branch
or modify the separately owned Evaluation implementation.

## L3 behavior

Temporal has six manual steps: nominal local point, timezone interpretation, DST
discontinuity, policy, returned UTC occurrence and availability. A selected real
nonexistent local point enters a hatched gap glyph and branches to SKIPPED using
the core reason. A real LaterFold example displays the returned mapping. The API
does not supply the transition's complete local boundaries; the UI explicitly
does not infer a one-hour transition interval. Complete and Incomplete coverage
remain distinct; unknown is never free.

Compilation is first-class navigation. One selected source series fans out to its
actual materialized occurrences and a separate returned skip branch. Full and
visible intervals share one comparison axis including the horizon. Clipping does
not change full occurrence identity. Technical exposes provenance, revision,
sequence, resolution and evidence. Research retains every occurrence/skip table
row and complete snapshot, including beyond the Technical 100-row limit.

Trace connects only attached CPIR and returned validation, compilation, search,
constraint checks, feasible candidates, ordering keys and result. It is labelled
as a structural explanation of completed evidence, never an internal execution
trace. A selected rejection shows its actual reason, time and available blocker
intervals. App-owned rejection selection persists between Planner and Trace for
the unchanged result and resets on successful new request/result import or run.
Source-less results do not acquire an invented CPIR or source facts.

Understand uses relationships, interval geometry, counts and meaning without raw
IDs/JSON. Technical adds exact mechanics and expandable evidence. Research opens
complete evidence. Depth changes do not send requests or change planning data.
The same Svelte application serves browser and desktop mode. Rust remains the
sole planning, recurrence, validation and occupancy authority.

Temporal/Compilation semantic steps never auto-advance. CSS micro transitions
communicate selection and nominal-to-resolved movement; reduced motion preserves
the same states without travel. Replacement results reset local presentation
selection; no stale timer can advance an old L3 story. Rejection selection cancels
obsolete Planner playback. Diagrams are bounded: compilation first eight
occurrences plus selection/eight skips; availability first 48 intervals per type.
Source JSON and exports remain complete. Manual controls support keyboard focus
and pressed states. Labels, glyphs, hatching and borders supplement color.

## Desktop runtime

Packaged mode owns one native Rust API at application lifetime. Paths resolve
from `process.resourcesPath`:

```text
resources/cerebri-runtime/
  api/cerebri-api[.exe]
  lab/index.html
  lab/assets/...
  smoke-inputs.json
```

The binary is outside ASAR and directly executable. The stage script validates
explicit prior Lab/release builds, clears only its fixed generated directory,
copies resources and sets Unix executable permission. It never secretly builds.
The existing desktop-dist ignore entry was preserved and adopted for the required
generated output; staging receives its own bounded ignore entry.
The staged canonical `smoke-inputs.json` is consumed only in explicit smoke mode.

Startup uses `spawn`, no shell, a hidden Windows console, `127.0.0.1:0` and the
packaged `CEREBRI_LAB_DIST`. Split stdout is parsed for the strict printed IPv4
loopback address; malformed/external addresses fail closed. `/health` confirms
actual API metadata before `/lab/` loads. The native title separately displays
actual desktop and Core versions. The total startup deadline is 15 seconds;
health is separately bounded within it. The API's typed `router_with_lab_dir`
preserves the development `router()` default and all endpoint semantics.

Single-instance protection rejects a second launch and restores/focuses the
existing window without another API. macOS final-window close retains the API;
activate recreates a window using the same child. Quit cancels retry timers,
prevents new windows and waits for owned child termination, with bounded forced
termination. Crash clears the origin, shows a controlled local error page with
bounded stderr diagnostics and offers explicit Restart local core. There is no
automatic crash restart loop. Development mode keeps the external `127.0.0.1:3000`
API workflow and bounded individual health requests.

Node integration remains disabled, context isolation/sandbox enabled, permission
checks/requests denied, popups denied and navigation restricted to controlled
pages or the selected Lab origin. External requests are denied. No execution
endpoint, credentials, provider mutation or telemetry is added. Proposal ≠ execution.
Versions, release state, CPIR and other contract authorities are unchanged.

Builder 26.15.3 configuration was checked against its installed schema/source.
Executable `cerebri-lab`, metadata `dev.nexus.cerebri.lab.desktop` and
`linux.syncDesktopName: true` synchronize the desktop entry and Electron identity.
`linux.executableArgs: []` excludes the legacy builder default `--no-sandbox`.
Linux CI permits the Chromium user-namespace sandbox on its disposable runner;
it does not disable the application sandbox. Smoke reads actual X11 WM_CLASS,
and artifact verification checks the extracted desktop filename, StartupWMClass,
Exec entry and executable. Wayland compositor behavior is not separately tested.

## Local verification

Executed successfully:

- Lab check: zero errors/warnings; 46 tests; production build.
- API tests: 13 passing tests, including exact default/runtime route parity,
  explicit assets in paths containing spaces, absent `/v1/execute` and non-loopback
  startup rejection. API release build succeeds.
- `cargo test --workspace --locked` succeeds.
- Repository `npm run check`, docs check/build and `cargo fmt --check` succeed.
- Lab/root npm audit: zero vulnerabilities. Site audit gate succeeds at Moderate;
  it reports three existing Low findings in the cookie/SvelteKit chain, outside
  this slice. No threshold or dependency is changed.
- Real production-API browser check: exact planner JSON/export parity, 11 evaluated,
  four rejected, seven feasible, canonical first proposal/order, L3 manual stages,
  actual gap/skip/LaterFold, fan-out/clipping, source-less result, shared rejection
  identity/reset, reduced motion and no browser errors.
- Axe WCAG 2 A/AA + 2.1 AA: zero violations for Planner, Temporal, Compilation and
  Trace in dark and light themes. Layout checks cover 1600×900, 1366×768, minimum
  960×640 and 820×900 with no document overflow. This is bounded accessibility
  evidence, not exhaustive assistive-technology qualification.
- Actual packaged Windows x64 smoke succeeds. It checks bundled API/renderer,
  complete Planner/Temporal JSON parity, actual health, all three depths,
  second-instance rejection, real API termination/error/restart and absence of
  both owned API PIDs after exit. Resource and evidence paths contain spaces.
- Windows NSIS artifact builds and artifact verification succeeds. Installer
  installation is not executed: an existing maintainer Cerebri Lab installation
  uses the same application identity; it is preserved.

The explicit smoke flag is the only trigger for fault injection, hidden windows,
automatic planning/exit and evidence capture. Smoke does not alter core semantics.

## Visual evidence

Local evidence root:
`C:\Users\Administrator\.codex\visualizations\2026\09\30\01a0f1cf-f7aa-7f01-be80-fedf25cbe018\lab-l3-electron`.

Manually inspected screenshots include Temporal nominal/gap/policy, actual
LaterFold, Complete and Incomplete availability; Compilation fan-out with skip
and clipped intervals; Trace pipeline/selected rejection/Technical and open
Research evidence; Electron startup, connected Planner Understand, actual API
failure and recovered Lab. Screenshots are evidence, not approved baselines.
The smoke writes full proof.json and stdout/stderr, and checks all owned PIDs.
The successful native artifacts were downloaded and their full proofs inspected.
Linux and macOS connected Planner screenshots were also manually inspected.

## Remote gates

The [desktop workflow](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/36717945030)
completed successfully on runtime commit `b07afd5f0ae05071e65661e30c485401efb13810`.
All three native jobs passed unpacked renderer/API smoke, all three depth checks,
sandbox invariants, single-instance protection, crash/restart and orphan checks,
then built and verified their installer artifacts. macOS also passed window
close/activate with unchanged API PID. **L3 and functional Electron E1 are complete**;
this does not establish release readiness or independent qualification.

| Platform | Packaged smoke | Native installer | Additional evidence |
| --- | --- | --- | --- |
| Windows x64 | Passed | NSIS passed | Hidden child console; both owned API PIDs terminated; local paths with spaces also tested |
| Linux x64 | Passed under Xvfb | AppImage passed | `cerebri-lab`; desktop file `dev.nexus.cerebri.lab.desktop`; actual X11 WM_CLASS and StartupWMClass both `dev.nexus.cerebri.lab`; `Exec=AppRun %U`, no `--no-sandbox` |
| macOS arm64 | Passed | DMG passed | Native `.app` resources and executable permission; close/activate reused the same API |

Workflow artifacts are build evidence:
[Windows](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/36717945030/artifacts/11097635227),
[Linux](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/36717945030/artifacts/11097370661),
[macOS](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/36717945030/artifacts/11096319554).

The complete returned PlanningResult and TemporalResult in all three native
proofs were deep-compared with the local Windows packaged proof and matched.
Canonical counts remain 11 evaluated positions, four rejections and seven
candidates, with the same first proposal and ordering. The actual reported
desktop and Core versions are independently `0.2.0` in these builds.

| Platform | Final dynamically selected origin | Initial/restarted owned API PIDs |
| --- | --- | --- |
| Windows | `http://127.0.0.1:56602` | `1324`, `2604` |
| Linux | `http://127.0.0.1:37383` | `4696`, `4812` |
| macOS | `http://127.0.0.1:49195` | `3568`, `3728` |

These are recorded observations, not configured ports. Each native runner
verified both of its owned API PIDs were absent after Electron exited. The proof
also records sandbox invariants, all three depth assertions, second-instance
rejection and explicit crash recovery; macOS records same-PID activation.

The first desktop run failed its Linux identity inspection because `xprop` was
absent from the runner. Windows/macOS passed. The corrected workflow explicitly
installs `xvfb` and `x11-utils`; the second run passed without removing an assertion
or disabling sandboxing. Failure logs/evidence remain available in
[the first run](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/36717443912).

[Normal Cerebri CI](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/36717949077)
is not green overall: Website Visual verification fails the four existing homepage
hash checks at 1440/1024/768/390 pixels. The other 28 visual checks pass; all 14
other jobs pass, including Rust tests/Clippy, Rustdoc, RustSec, Lab tests/check/build,
repository policy, npm audits, browser/E2E and docs/site checks/builds. No Site code
or baseline was modified. The earlier run has the same four homepage failures.

The mechanically reconciled PR head was also checked by
[Cerebri CI](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/36718280092):
14 jobs passed, with only the same Website Visual verification job failing.
The merged main's
[CI run](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/36719496978)
likewise completed with 14 successful jobs and that one failed visual job.

Final handoff changes after the tested runtime commit are documentation only.
Desktop implementation file content is unchanged from the successful native run.

## Remaining work

L4 remains a separate PR: Preferences precedence/distance workspace, Semantics
knowledge/provenance grammar and interactive Dependencies paths, cycles and
topological order.

L5 remains: larger-input visual performance campaign; exhaustive empty/error
polish outside L3; installer branding/icons; signing/notarization planning;
release packaging policy; extended accessibility matrix; final motion timing.
No signed Windows installer, notarized macOS app, production updater, provider
execution or stable public release is claimed. This checkpoint ends at L3/E1.
macOS x64 and a separate Wayland compositor matrix were not executed. Current
depth persistence uses browser localStorage per origin; a fresh dynamic desktop
port does not guarantee persistence across desktop launches. Stable desktop
preference persistence remains broader product polish, outside this checkpoint.

[Lab README](../../../apps/cerebri-lab/README.md)

## Exact changed files

- `.github/workflows/desktop-installers.yml`
- `.gitignore`
- `apps/cerebri-api/src/lib.rs`
- `apps/cerebri-api/src/main.rs`
- `apps/cerebri-api/tests/runtime_assets.rs`
- `apps/cerebri-lab/README.md`
- `apps/cerebri-lab/desktop/main.cjs`
- `apps/cerebri-lab/desktop/runtime.cjs`
- `apps/cerebri-lab/desktop/smoke.cjs`
- `apps/cerebri-lab/desktop/stage.cjs`
- `apps/cerebri-lab/desktop/verify-artifact.cjs`
- `apps/cerebri-lab/package.json`
- `apps/cerebri-lab/src/App.svelte`
- `apps/cerebri-lab/src/components/AvailabilityTimeline.svelte`
- `apps/cerebri-lab/src/components/CompilationPanel.svelte`
- `apps/cerebri-lab/src/components/CompilationStory.svelte`
- `apps/cerebri-lab/src/components/InspectorViews.svelte`
- `apps/cerebri-lab/src/components/ManualSteps.svelte`
- `apps/cerebri-lab/src/components/PlannerScene.svelte`
- `apps/cerebri-lab/src/components/Sidebar.svelte`
- `apps/cerebri-lab/src/components/TemporalStory.svelte`
- `apps/cerebri-lab/src/components/TemporalView.svelte`
- `apps/cerebri-lab/src/components/Timeline.svelte`
- `apps/cerebri-lab/src/components/TraceView.svelte`
- `apps/cerebri-lab/src/lib/contracts.ts`
- `apps/cerebri-lab/src/lib/l3-presentation.ts`
- `apps/cerebri-lab/src/styles.css`
- `apps/cerebri-lab/tests/browser-check.mjs`
- `apps/cerebri-lab/tests/desktop.test.ts`
- `apps/cerebri-lab/tests/l3.test.ts`
- `docs/development/progress/2026-09-30-lab-l3-electron-checkpoint.md`
