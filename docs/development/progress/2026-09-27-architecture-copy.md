# Architecture ownership explanation

Date: 2026-09-27

## Scope and repository state

Continue the maintainer's text-only plan after the CPIR walkthrough. Explain which
existing code owns input validation, time operations, required rules, comparison,
planning, transport, presentation and execution. Maintain paired DE/EN public references.
No architecture, dependency, UI layout, animation, planning semantics or release change.

Work remains in the canonical workspace on `codex/slice-2-phase-a-completion`, observed
at `b3ace68`. Existing copy changes, the unrelated `.gitignore` edit and root test output
were preserved. Fetched `origin/main` was `059b253`. The separate technical presentation
PR #28 had merged at `8f193d2`, and layout PR #30 was also merged on main. Neither was
merged into this active evaluation branch. PR #29 concerns evaluation contracts and was
not changed. This log qualifies only the local copy, not those separate changes.

## Changes

- Expanded the DE/EN Foundation references from the appointment example into a concrete
  account of input checking, candidate checks, comparison and returning the result.
- Added a source-linked ownership table for the Rust crates, including the distinction
  between the shared Core entry point and the planner implementation.
- Explained the REST/Node routes, Lab API calls and the public site's build-time examples.
- Distinguished code dependencies, request/result flow and lifecycle checks. The selected
  import relationships are explicitly an incomplete list, not a runtime trace.
- Explained why full-plan validation, action translation, authorization and final execution
  checks remain separate. Existing technical lifecycle and search contracts were retained.
- Located the metadata-only ML interfaces and isolated research work without implying
  neural inference in the current planner.
- Replaced the Architecture route's abstract prose and linked the CPIR walkthrough forward
  to the paired architecture references.
- Used a wrapping list for dependencies and source links for the CPIR field table after
  mobile Axe checks found horizontally scrolling content without keyboard access. No CSS
  or renderer changes were needed.

## Authority

Read the current Master, ADR index, roadmap and relevant accepted workspace, lifecycle,
execution, transport, Lab, Site and ranking decisions. Checked the explanation against
Cargo dependencies, the repository dependency allowlist, the Core facade, planner lifecycle,
executor, ML interfaces and semantic types. Key sources are linked from the public pages.
This is documentation of existing behavior, not a new architecture decision or independent
Architecture, Math or Security qualification.

## Verification

- `npm run check`: passed after the final content fixes, including all 22 version-authority
  tests, Markdown links, DE/EN counterparts and repository boundaries.
- `npm --prefix apps/cerebri-site run check`: passed after the architecture edits with zero
  errors and warnings. The normal Rust data-generation step succeeded; the Windows
  execution restriction recorded in the CPIR slice did not recur.
- `npm --prefix apps/cerebri-site run build`: passed after the final Markdown fixes,
  rebuilding the Rust examples and 67 canonical documents.
- All 30 existing non-visual Playwright tests passed. The run used the unchanged project
  configuration through the temporary ignored port-4175 wrapper documented in the earlier
  copy log. No assertions or thresholds were weakened.
- Additional Chromium checks after the final build covered `/architecture/`, `/cpir/`
  and both languages of the Foundation and CPIR documents at 1440, 390 and 320 pixels.
  All returned HTTP 200 without horizontal document overflow. Article languages matched
  their routes. Axe reported no serious or critical violations at 390 pixels.
- Both CPIR-to-Foundation links and reciprocal Foundation language links navigated
  correctly. Both Foundation documents passed the 200% text check at 390 pixels.
  Focusing a CPIR field link and pressing ArrowRight scrolled each language's table by
  40 pixels, confirming keyboard access to its horizontally overflowing content.
- Representative desktop and mobile captures were inspected. The dependency list's eight
  documented import edges were checked against Cargo metadata; DE/EN technical terms and
  dependencies matched.
- `git diff --check`: passed.

The earlier visual baseline mismatch remains open; this slice neither reran nor accepted
new visual reference hashes. No commit, push, remote CI or publication was performed for
the copy changes on the separate evaluation branch.

## Remaining work

The next text topic is the research direction and its connection to the founder's learning
path. Full localization of interactive pages and visual design remain outside this slice.
The earlier visual reference mismatch and integration of these local copy changes into
the current main branch remain separate outstanding work.
