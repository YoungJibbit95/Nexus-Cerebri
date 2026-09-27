# Concept-to-CPIR explanation

Date: 2026-09-27

## Goal and scope

Continue the public-copy plan after the founder story with a worked explanation of how
the existing appointment example is represented in CPIR. Keep the maintainer's text-only
scope: paired German/English documentation and copy in the existing CPIR page. No new
visualization, interaction, planning behavior, schema or release change.

## Repository state

The canonical workspace remains at `aa86b82`, now on the existing
`codex/slice-2-phase-a-completion` branch. The previous copy changes, unrelated `.gitignore`
edit and root test output were preserved. Fetched `origin/main` was `890d954`.

Open PR #28 (`codex/technical-descent`, observed at `890fadc`) already implements CPIR
and architecture presentation work. This slice leaves those changed files untouched and
supplies the explanatory prose in the existing content sources. Open PR #27 concerns
evaluation foundations and is outside this task. Neither PR was changed or messaged.

## Changes and evidence

The DE/EN CPIR references now begin with the same concrete appointment as the introduction,
then map it to exact fields and two literal JSON excerpts. The original schema and
transport reference remains below the worked example. The introductions link into it.
The CPIR route and its existing primer now use the same example and explain the field
names. A stale claim that the website uses legacy CPIR 0.1 was corrected to 0.2.

The explanation specifically preserves these distinctions:

- Missing time on the prospective target is expected; missing required duration stops
  search. Source: [request validation](../../../crates/cerebri-planner/src/validation.rs).
- Processing state and knowledge state are separate. Source:
  [shared types](../../../crates/cerebri-types/src/lib.rs).
- Empty constraints/facts lists do not disable mandatory overlap checks against object
  times. Source: [candidate validation](../../../crates/cerebri-planner/src/lifecycle.rs).
- The 900-second grid yields 11 starts; the 256-candidate budget is an upper limit on
  evaluated starts. Source: [search](../../../crates/cerebri-planner/src/search.rs).
- Scope filters, policy and object-specific planning grants have different roles and do
  not authorize execution. Sources: [model](../../../crates/cerebri-planner/src/model.rs)
  and [policy](../../../crates/cerebri-planner/src/policy.rs).

The German reference also now distinguishes a required `ExplicitTime` placement from a
preferred start. All example values come from the unchanged
[CPIR fixture](../../../examples/request.json). Synthetic integration provenance is
identified as test data. No natural-language parser or live calendar connection is implied.

## Verification

- `npm run check`: passed, including 22 version-authority tests, links, DE/EN counterparts
  and repository boundaries.
- Both JSON excerpts in each language were parsed and compared with the exact corresponding
  values in the unchanged fixture: passed. Markdown rendering succeeded for both documents;
  their 44 distinct inline field names, values and technical terms match across languages.
- `npx svelte-kit sync`, followed by `npx svelte-check --tsconfig ./tsconfig.json
  --fail-on-warnings` in the site directory: passed with zero errors and warnings. This
  direct Svelte check used the existing generated data and is only component/type evidence.
- The regular `npm --prefix apps/cerebri-site run check` did not pass. Its first attempt
  encountered a temporarily missing evaluation module during concurrent Rust work. Once
  the module file appeared, the next attempt compiled far enough to run the examples, but
  Windows Application Control blocked `target/debug/examples/temporal.exe` with OS error
  4551. No policy or gate was changed to work around that restriction.
- A fresh production build and browser/visual checks remain unverified because their Rust
  data-generation prerequisite is blocked. The founder-story revision's earlier browser
  results are not claimed for this follow-up.
- `git diff --check`: passed.

Implementation evidence is not independent Architecture, Math or Security qualification.
No commit, push, remote CI or publication was performed for this slice.

## Remaining work

Resume the regular site check and build when the Windows execution restriction is resolved,
then check the new CPIR copy and both documentation routes in the browser. The previously
recorded visual baseline mismatch remains open. The parallel technical
presentation PR needs its own review; no acceptance of its implementation is implied here.
The next editorial topic is explaining implementation ownership after readers understand
the planning data. Full site localization and visual design remain outside this text slice.

## Verification follow-up during the architecture copy slice

The subsequent [architecture copy work](2026-09-27-architecture-copy.md) completed the
previously blocked regular site check and production build. Rust data generation succeeded
without changing Windows policy. All 30 existing non-visual Playwright tests passed.
The final CPIR route and both CPIR documents were then checked at 1440, 390 and 320 pixels:
HTTP 200, correct article languages and no horizontal document overflow. Axe reported
no serious or critical violations at 390 pixels after source links were added to the
field table. Its horizontal content was also reached with the keyboard in both languages.
The new links to the paired architecture explanation were checked by navigation.

This closes the build and functional browser gaps recorded above. The earlier visual
reference mismatch remains unresolved, and the local copy has not been published.
