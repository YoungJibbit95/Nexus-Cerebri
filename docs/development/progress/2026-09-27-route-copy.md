# Public guide consistency and design handoff

Date: 2026-09-27. Editorial slice based on main at `d2d8a42`, after PR 31 and PR 33.
Fast-forwarded to `c7d1721` before committing to include the merged PR 34 test-renderer
correction; its production code is unchanged.

## Scope and ownership

The maintainer requested the next text step in parallel with the Website Design Agent.
This change supplies paired DE/EN explanations in canonical public Markdown. The design
agent owns their integration into route introductions, diagrams and other website elements.
No Svelte, CSS, route metadata, visual baselines or runtime code changes are included here.
The public Markdown pages are rendered by the existing documentation build.

This is the editorial portion of the cross-route consistency pass. It does not complete
the separate visual integration or full localization of the interactive site. Private
handoff files are not included in the repository.

## Content and source checks

- Planning begins with the checked-in 30-minute, 11-start example. It explains candidates,
  required rules, preferences and search assessment without implying global optimality.
  `BestFound` alone does not promise a solution; the initial insufficient-information
  result uses that assessment too.
- Time explains touching endpoints, buffers, complete versus incomplete coverage, local
  time and DST policies. Corrected the old claim that planning cannot expand existing
  series: ADR-0012 connects fact-sourced CPIR temporal context to occupancy, while the
  hard RecurrenceRule constraint and prospective series remain unsupported.
- New safety guides separate proposal checks, full-plan validation, action translation,
  authorization and executor preflight. They identify the in-memory provider/ledger
  boundary and the absence of REST/Node execution.
- Development guides distinguish the static website, local Lab and Rust-backed transports,
  explain the two different preferred-start fixtures, and give the build-before-serve
  sequence. Request validation is explicitly distinct from lifecycle validation.
- The language indexes now provide a reading sequence from the founder's learning
  motivation through planning, time and safety. Technical references remain available.
- Research status now reflects the partial Phase-B checkpoint: candidate/hypothesis
  fingerprints exist; whole-scenario/decision projections, collection and replay remain
  unfinished. Phase B is not represented as complete.
- The Pages description now matches the workflow's push-to-main and manual triggers.

Checked against Master 0.4, ADRs 0002/0003/0005/0006/0009/0010/0012 and the current
ADR-0015 status; Rust search/lifecycle/executor/API code; the two canonical appointment
fixtures; the Lab README; and the actual Pages workflow.

## Route-to-copy handoff

Use the following DE/EN pairs as the content source. Adapt presentation in the design
slice; preserve the stated boundaries and source links.

| Website route | English source | German source | Passage to integrate |
| --- | --- | --- | --- |
| Explore and Docs | [English index](../../en/README.md) | [German index](../../de/README.md) | Opening, reading sequence and current-status explanation |
| Planning | [Planning guide](../../en/planner-integration.md) | [Planungsleitfaden](../../de/planner-integration.md) | All sections before the technical reference |
| Time | [Time guide](../../en/temporal.md) | [Zeit erklären](../../de/temporal.md) | All sections before the technical reference |
| Safety | [Safety guide](../../en/safety.md) | [Prüfung und Freigabe](../../de/safety.md) | Proposal example, five stages and current implementation limits |
| Developers | [Development guide](../../en/development.md) | [Entwicklungsleitfaden](../../de/development.md) | Entry choices and API validation explanation |
| Lab | [Lab introduction](../../en/development.md) | [Lab-Einstieg](../../de/development.md) | Opening, local setup and what to inspect |
| Roadmap | [Research guide](../../en/research.md) | [Forschungsleitfaden](../../de/research.md) | Updated evaluation foundations in the current-status section |

Homepage, CPIR and Architecture retain the copy delivered in PR 31. The origin remains
linked from the reading sequence; this pass invents no new biographical claims. A neural
learning simulator and new planning behavior remain outside this editorial slice.

## Verification

- Repository checks passed, including all 22 version-authority tests, local links,
  DE/EN counterpart metadata and dependency boundaries.
- Site typecheck passed with zero errors and warnings. The production build passed with
  the `/Nexus-Cerebri` base path and current Rust-generated fixture output.
- All 75 existing non-visual Playwright tests passed locally.
- Additional Chromium checks passed for all 12 affected public guide routes at 1440,
  390 and 320 pixels: HTTP 200, correct language, reciprocal language links and no page
  overflow. All 35 distinct linked documentation routes returned HTTP 200.
- The 12 routes also passed 200% text checks at 390 pixels. Axe reported no serious or
  critical violations. The development command list replaces an unfocusable scrolling
  code block while preserving every command. Planning and safety use the repository's
  supported comment metadata so language markers do not leak into the visible article.
- Reviewed the changed DE/EN prose and mobile safety-page captures. Checked the complete
  diff and whitespace; no source, configuration, schema or release-version changes are
  included. Existing unrelated maintainer files remain outside the commit.

The PR carries the actual remote CI result for its final commit. This record is
implementation evidence, not independent Architecture, Math or Security qualification.
