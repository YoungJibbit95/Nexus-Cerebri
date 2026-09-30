# 2026-09-30 — Reviewed Linux homepage visual baseline

## Goal and checkpoint

Resolve the four stale Linux homepage expectations without changing product source,
visual coverage, renderer flags, platform ownership or release status.

The work starts at main `7bcf4abe289ada74453a0bf558b0b9e4f3b14c07` in the isolated
`codex/v7-cerebri-visual-contract` branch. Software remains unreleased 0.2.0;
specification, CPIR, integration and evaluation versions do not change.

## Observed cause and source intent

The last fully green baseline is
`314fb34593b62297c56daa372c3e1ea167556628`,
[CI run 36322982686](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/36322982686)
(32 visual tests passed). Since that checkpoint, official-site source changes are
limited to the origin finale:

- `8eb535aef1f2ec2e764ee1526b2af46c3c5c6a0a`, `fix(site): center origin finale content`,
  centers the finale paragraphs and desktop actions.
- `b112eb8404d1dd3b682d303535f1976880d0dcf7` changes the first origin paragraph's wording.

The current site source is identical between integration head
`1e1ecd6a2b38fde70c7d6ba8f6573404089334ac` and the pinned main checkpoint.
The screenshot expectations were left at the preceding composition.

Actual PNGs were downloaded from the `site-visual-evidence` artifacts for the
green baseline run, [integration run 36713042894](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/36713042894),
and [main run 36723594256](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/36723594256).
Integration first attempts, integration retries, main first attempts and main retries
are byte-identical at each of the four viewport widths. The four current and previous
images have identical dimensions. Every changed pixel lies within the finale bounds
below; every pixel outside those rectangles is unchanged. Crops at all four widths
were visually inspected alongside the source change.

| Width | Full image size | Changed rectangle `[left, top, right, bottom)` | Changed pixels |
| --- | --- | --- | --- |
| 1440 | 1440 × 13084 | `[346, 12220, 1058, 12712)` | 109695 |
| 1024 | 1024 × 13457 | `[138, 12593, 850, 13085)` | 109701 |
| 768 | 768 × 14536 | `[16, 13702, 722, 14194)` | 109393 |
| 390 | 390 × 19062 | `[16, 17865, 371, 18524)` | 57157 |

This evidence supports stale expectations for an intended source change. It does not
support a flaky rendering diagnosis or a frontend/Core regression elsewhere.

## Exact renderer and artifact pins

The baseline and current CI logs identify the same Ubuntu 24.04 runner image
`20260920.314`, runner `2.337.0`, Playwright `1.63.0`, Chromium / Chrome Headless Shell
`153.0.8010.12` (Playwright revision `1243`), `--disable-skia-runtime-opts`,
the `/Nexus-Cerebri` base path and reduced-motion capture. Viewports use 1000px
height and device scale factor 1; full-page dimensions are recorded above.
The locked site dependency file is unchanged, SHA256
`18fb90e40ec6b5d62ccab924b3856543f4cafb542ef536b461f8fee3735f9964`.

Downloaded artifact identities:

| Run | Artifact ID | Archive digest |
| --- | --- | --- |
| Green baseline 36322982686 | 10932684903 | `sha256:9e0e288ca408a4010a6bfaea536c5e3a52a90703d2072e4c20eb7e5069ca3390` |
| Integration 36713042894 | 11095496381 | `sha256:ff806826979250db0a58c2a14fa3abbf1bb90baac58c920a54afd6af5620e676` |
| Main 36723594256 | 11101997583 | `sha256:81cafd25f64d0f4084b23d027637366b1ddef638de37406d4ae82a8df2c71771` |

The eight integration failure traces were also inspected for renderer/context pins;
they are artifact `11095177033` (`visual-failure-evidence`, digest
`sha256:de8f7f5893b5b5f5985e329c5bf1987a00f7b36b64c54a3278e90c1bdca328d3`).
GitHub's seven-day artifact retention is not a permanent image archive. The V7
investigation retains the downloaded PNGs/traces, before/after diagnostic crops,
source diff, exact comparison data and a reproducible read-only comparison script.

## Change

Only these four keys in `visualBaselines.linux` change:

| Key | Reviewed current PNG SHA256 |
| --- | --- |
| 1024 | `833d5f6672f10f784169ffa9df22322376c77213c511d579bbff196bc37351b6` |
| 1440 | `165d8dda73b3b4c203c64cf7be83afd94cf3614ddb4d69d391230900c3d1b409` |
| 390 | `40ff1ca2fa8841651c8a151fee6e412332028cd559a9c8b9b39bd51e26abc0b4` |
| 768 | `4f33a9c0933b670e47dbba3840535059b6951db16803287bc11a3d284ca88e4a` |

All 32 Linux baseline keys match all 36 current uploaded captures (the additional
four captures are homepage retries). The other 28 Linux values, every Windows
value, key coverage and exact equality assertion remain unchanged. No product
implementation, solver, transport, authorization, Lab source or UI copy changes.

## Verification and limits

Completed locally with Node v26.3.1 and pinned Rust 1.97.0:

- Fresh locked root and site installs with `npm ci --ignore-scripts`.
- `npm run check`: all 22 version-authority tests and repository checks passed.
- Site `npm run check`: zero errors and zero warnings.
- Site build with `/Nexus-Cerebri`: passed with actual Rust-generated fixtures.
- Independent artifact binding: 32 Linux keys / 36 uploaded captures pass; only the
  four specified keys change and Windows values remain equal to the base commit.
- Existing visual suite executed on Windows: 6 passed, 26 failed exact Windows
  expectations. It does not exercise the changed Linux values and is not reported
  as a green gate. Existing Windows expectations remain separately unqualified.
- Existing local Browser/E2E suite: all 80 tests passed on Windows using the
  pinned Chromium build and `/Nexus-Cerebri` base path. This includes responsive
  bounds, enlarged text, reduced motion, keyboard paths, serious axe checks and
  source/result correspondence; it is not native-device or release acceptance.

The local host has no Linux runtime. Current Linux artifact equality is strong
evidence for this bounded expectation change; a fresh draft-PR Linux visual gate
is still required for the actual pushed commit. No human-only screenshot-approval
requirement was found in the applicable agent, official-site or testing rules.
This is reviewed baseline maintenance and implementation evidence, not independent
Architecture, Math or Security qualification, native acceptance or a release.

## Next step

Review the two-file source diff and inspect the exact pushed commit's required
CI. Preserve the independent Windows gap.
Do not merge, publish a release, deploy, relax a gate or refresh any other baseline
as part of this packet.
