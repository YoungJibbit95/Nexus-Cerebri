# Research explanation and public-copy integration

Date: 2026-09-27

## Scope

Complete the next editorial step: connect the founder's learning motivation with concrete
research questions and the current deterministic planner. The maintainer also requested
a pull request for the accumulated, previously local public-copy work.

Added paired DE/EN research pages covering request interpretation, preference research,
search guidance, personal patterns and possible later applications. They explain the
reserved from-scratch learning path without implementing it. Current ranking observations,
Phase-A evaluation wire types and ML metadata/interfaces remain distinct from training,
collection, replay and learned planning. Linked the research explanation from the existing
homepage, roadmap prose, introductions, architecture references and language indexes.

The first-person origin uses the maintainer-supplied story. The paper's year and three
authors were checked against the publisher's page for
[Learning representations by back-propagating errors](https://www.nature.com/articles/323533a0).
Private handoff files were not copied into the repository.

## Integration

Kept the canonical workspace and preserved the unrelated `.gitignore` change and root
test output. Created `codex/public-explanation` from `origin/main` at `2a98cf8`, after
the earlier UI and evaluation PRs had merged. Applied the saved copy changes there,
retaining main's layouts, animation and newer components. Moved the comparator explanation
to its current component and updated existing button-text assertions. No CSS, planner
behavior, schema, versions or release publication changed.

The PR combines the already completed introduction, CPIR and architecture explanations
with this research conclusion as one public explanation change. The earlier dated logs
describe the checks on their original branch; this record qualifies the integrated result.
Full interactive-site localization, a backpropagation simulator and further UI work are
not part of this PR.

## Verification

- Repository checks passed, including 22 version-authority tests, links, DE/EN metadata
  and dependency boundaries.
- The regular site check passed with zero errors and warnings, including Rust data generation.
- The production build passed with the GitHub Pages base path `/Nexus-Cerebri`.
- Additional Chromium checks covered the roadmap and eight DE/EN explanation routes at
  1440, 390 and 320 pixels: HTTP 200, correct article language and no document overflow.
  Axe found no serious or critical violations at 390 pixels. Research language links and
  200% text checks passed. A source link in the introduction's input table restored
  keyboard access to its horizontal content on narrow screens.
- All 68 non-visual Playwright tests passed on the integrated site. The first run exposed
  two old button labels in an upstream continuity test; only those text selectors changed.
- Reviewed all 28 Windows visual captures, with detailed mobile crops for the changed
  explanation, comparison, measurement and founder passages. Updated the Windows hashes
  and reran the unmodified visual gates: all 28 passed.
- Checked semantic DE/EN parity, equal inline technical terms and unchanged component style
  blocks. `git diff --check` and the staged whitespace check passed.
- The initial PR CI run [36310686211](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/36310686211)
  on `c4bd764` passed 14 of 15 gates, including the Linux browser suite. Only the visual
  hash gate differed from the pre-copy references. Reviewed its 28 Linux captures and
  detailed mobile crops; every initial/retry pair had identical hashes. Updated only the
  Linux reference hashes from that artifact. The follow-up revision must pass the actual
  remote gates before this PR is marked ready for review.
- No release, tag, merge or deployment is authorized by this publication of code for review.

This is implementation evidence, not independent Architecture, Math or Security qualification.
