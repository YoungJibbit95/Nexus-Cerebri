# Public planning explanation: first copy revision

Date: 2026-09-27

## Scope

Rewrite the English homepage passages represented in the supplied screenshots and add a
paired German/English introduction. Explain the appointment problem before introducing
CPIR, candidates, comparison fields, half-open intervals and whole-second measurements.
Keep the proposal/execution distinction and the limits of ProvenOptimal explicit.

The initial revision introduced the learning-project context and linked the 1986
Rumelhart, Hinton and Williams paper. The maintainer then supplied the missing personal
account. The follow-up below incorporates it without inventing biographical details.

The English homepage links to the German introduction. Both introductions link to each
other and are listed in the existing documentation indexes. This is bilingual explanatory
content, not a site-wide localization or a German interactive homepage.

## Workspace and boundaries

The canonical workspace and active `codex/slice-2-evaluation-replay` branch were retained
at `aa86b82`. Fetched `origin/main` was `af62b82`; its later visual continuity changes were
not merged into this unrelated evaluation checkpoint. Existing `.gitignore` changes and
untracked root test results were preserved.

Only public prose, labels, links, corresponding existing test assertions and documentation
were changed. CSS, animation, domain calculations, generated runtime inputs, Rust source,
versions and release state were not changed. The generated site still uses Rust results.
This record supplies implementation evidence, not independent Architecture, Math or
Security qualification.

## Verification

- `npm run check`: passed, including all 22 version-authority tests, Markdown links,
  DE/EN counterparts and repository boundaries.
- `npm --prefix apps/cerebri-site run check`: passed with zero errors and warnings.
- `npm --prefix apps/cerebri-site run build`: passed; Rust-generated examples were rebuilt.
- All 30 existing non-visual Playwright tests passed after updating changed text assertions.
  Existing checks for candidate counts, ordering, lifecycle states, keyboard controls,
  reduced motion, accessibility, enlarged text and responsive widths remain in place.
- Both new introduction routes were also checked in Chromium at 1440 and 390 pixels:
  correct article language, reciprocal language links, expected concepts and no horizontal
  document overflow. Desktop and mobile screenshots were inspected.
- All 10 visual tests ran and reported differences against the prior Windows hashes.
  The rendered reference captures were inspected, but no new baseline hashes were accepted
  for this provisional text revision. Linux references were not regenerated or changed.
- `git diff --check`: passed.

The initial browser command encountered an occupied port 4173. The successful browser
run imported the same Playwright configuration through a temporary ignored wrapper and
used port 4175 with a separate output directory. No assertions or thresholds were disabled.
A separate local preview runs on port 4177.

No commit, push, pull request, deployment or remote CI run was performed for this revision.
Visual baseline acceptance remains open before publication.

## Founder-story follow-up

The maintainer supplied a fuller account of the project's origin on the same day.
The paired introductions and the existing homepage finale now connect the neural-network
mathematics that sparked curiosity, understanding equations one operation at a time,
learning through implementation and tests, and planning as a concrete problem to study.
The opening also identifies Cerebri as a learning project rather than reducing its
purpose to appointment scheduling.

The story uses the maintainer's first-person perspective. The 1986 paper is credited to
David E. Rumelhart, Geoffrey E. Hinton and Ronald J. Williams and linked to the publisher.
The broader aim of planning across applications is explicitly future-facing, consistent
with Master Specification 0.4, section 1. Current single-target deterministic planning
stays distinct from future learned assistance. No unsupported personal anecdotes or
current neural-runtime capabilities were added.

The supplied handoff files remain private source material outside the repository.
Their broader design, implementation and publication instructions do not expand this
text-only task.

After the follow-up, repository checks, the Svelte check and the production build passed
again. All 30 existing non-visual Playwright tests passed. Chromium checks confirmed
both language links navigate to their counterpart, the article language is correct, and
neither introduction nor the homepage overflows horizontally at 1440 or 390 pixels.
The updated story passages were captured and inspected at both widths. Visual baseline
hashes were not rerun or updated for this follow-up; their earlier mismatch remains
unresolved. The changes remain local and unpublished.
