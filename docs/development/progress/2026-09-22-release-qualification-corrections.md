# Release qualification corrections — 2026-09-22

## Goal and starting evidence

Bounded corrections to the verified deterministic planner baseline; no new planner or website
features, version bump, tag or release. Software stays **0.2.0, unreleased, undergoing release
qualification; published release: none**. Independent review and a later explicit publication
decision remain required even after all correction gates pass.

Fetched `main` at `affec7949d11d78402426f1474118c10a5b85615` (merge of PR #6), whose head was
`99c1fad3d84e8bebb458396ee450f0279e358a6c`. Work starts from that main commit on
`codex/release-qualification-corrections`. Git tags and GitHub releases were empty.
The starting main CI [35635500578](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/35635500578)
and Pages [35635500777](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/35635500777) succeeded.
The earlier local worktree registration pointed to a missing Git directory; a clean clone was used
without altering the unrelated local directories or rewriting published history.

## Corrections and decisions

| Finding | Disposition | Evidence / decision |
| --- | --- | --- |
| Ambiguous 0.2.0 release wording | FIXED | README, current Changelog header, roadmap and generated site distinguish workspace version, qualification and publication. No formal RC claim. |
| Weak substring version checks | FIXED | Exact README declarations/badges/footer checked against Cargo, Master, schema reference and current Changelog header; 20 targeted mutations, positive repository case and historical-record case. Site uses the same checked authorities without fallback. |
| CPIR 0.1 primary example | FIXED | `examples/request.json` is minimal current 0.2; the old request is retained as `examples/legacy-cpir-0.1.json`. Both have Core/API roundtrip and Node coverage; Lab legacy parity uses the named legacy input. Unknown major/minor pairs fail closed. |
| Missing Rust advisory gate | FIXED | Dedicated CI job installs cargo-audit 0.22.2 with locked tool dependencies; committed Cargo.lock is scanned with `--deny warnings`. No ignores or target exclusions. |
| Mutable workflow actions | FIXED | All direct CI/Pages action references resolve to full upstream commit SHAs and exact tag comments; repository check enforces this form. Weekly GitHub Actions Dependabot updates require normal review. |
| Workflow permissions | CLARIFIED | CI/advisories read contents only; checkout credentials are not persisted. Pages/id-token write permissions are scoped to its main-only deployment job. Triggers unchanged. |
| Implicit workspace build | FIXED | Explicit `cargo build --workspace --locked` complements all existing gates. |
| Incomplete ADR discovery | FIXED | Index lists all 13 Accepted decisions, titles and topics/amendments; AGENTS requires every relevant Accepted ADR. |
| Ambiguous Master date | CLARIFIED | Original baseline date remains 2026-09-19; Git records later revisions. No invented last-updated date. |
| Missing archived revisions | FIXED after QUESTION investigation | Exact final Git blobs for 0.1/0.2/0.3 restored unchanged, with original commit/path/blob provenance. Current 0.4 is not duplicated. Generated archive pages are labeled historical/non-normative. |
| External view counter | FIXED / policy clarified | Removed Komarev counter; useful informational badges remain with documented dependency/authority limits. |
| Historical release statements | ACCEPTED AS-IS | Accurate dated ADR/progress statements remain intact and do not govern present release status. |
| Existing planner verification/invariants | ACCEPTED AS-IS | No production Rust or planner code changed; independent oracles, graph/recurrence campaigns, admission limits, lifecycle and execution boundaries retained. |

## Supply-chain evidence and remaining low-severity finding

`cargo audit --file Cargo.lock --deny warnings --json` passed with cargo-audit 0.22.2:
101 lockfile dependencies, 1,258 advisories, database commit
`57ad4063bb49c1deb04b6fcee30cfbac6b508474` (2026-09-21), zero vulnerabilities and zero warnings.
The scan checks known lockfile advisories, not general Rust security or function reachability.
Pins were resolved through upstream GitHub release and Git-ref APIs on 2026-09-22.
Scanner installation uses the published [0.22.2 release](https://github.com/rustsec/rustsec/releases/tag/cargo-audit/v0.22.2);
there is no downloaded installer script or new application dependency.

**MINOR / retained under the existing npm high-severity gate:**
[GHSA-pxg6-pf52-xh8x](https://github.com/advisories/GHSA-pxg6-pf52-xh8x), cookie 0.6.0,
via site -> @sveltejs/kit 2.70.3 -> cookie (adapter-static also reports the transitive finding).
The three low npm entries represent one advisory. Its affected operation serializes untrusted
cookie names/paths/domains. Source inspection found no application cookie setters or server routes;
the shipped site is fully prerendered static output. SvelteKit's development/build server includes
the dependency, so this is not described as absent from the repository or generally harmless.
Patched cookie starts at 0.7.0, outside the installed Kit `^0.6.0` range. The registry's current Kit
2.70.3 still requires that range; npm's offered automatic fix is a breaking downgrade.
No ignore, audit suppression or forced override was added. Maintainer review is due at independent
release review, before any server/cookie feature, and no later than 2026-10-22. Prefer an upstream
compatible Kit upgrade; assess a scoped override separately if none becomes available.

## Verification

Local environment is Windows with Rust 1.97.0 and Node 26.3.1; CI uses Linux and Node 22.
Browser checks use `CEREBRI_BASE_PATH=/Nexus-Cerebri`; rustdoc uses `RUSTDOCFLAGS=-D warnings`.

| Command | Local result |
| --- | --- |
| `cargo fmt --check` | PASS |
| `cargo build --workspace --locked` | PASS |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo test --workspace --locked` | PASS, 112 tests including doctests; generated campaigns retained |
| `cargo doc --workspace --no-deps --locked` | PASS, warnings denied |
| `cargo build -p cerebri-node --locked` | PASS |
| `node --test bindings/node/test.mjs` | PASS, 3 tests |
| `npm run check` | PASS, 22 authority tests plus repository consistency |
| `node scripts/check-repository.mjs` | PASS, run explicitly |
| `npm run docs:check` | PASS, no errors/warnings |
| `npm run docs:build` | PASS after correcting fragment links |
| `npm --prefix apps/cerebri-lab run check` | PASS, no errors/warnings |
| `npm --prefix apps/cerebri-lab test` | PASS, 22 tests after correcting the current-fixture assertion |
| `npm --prefix apps/cerebri-lab run build` | PASS |
| `npm --prefix apps/cerebri-site run check` | PASS, no errors/warnings |
| `npm --prefix apps/cerebri-site run build` | PASS after correcting fragment links |
| `npm --prefix apps/cerebri-site run test:e2e` | PASS, 7 tests including visible current CPIR/publication status |
| `npm --prefix apps/cerebri-site run test:visual` | PASS, 4 reviewed Windows references |
| `npm audit --audit-level=high` | PASS, zero findings |
| `npm --prefix apps/cerebri-lab audit --audit-level=high` | PASS, zero findings |
| `npm --prefix apps/cerebri-site audit --audit-level=high` | PASS threshold; 3 low entries / 1 advisory, assessed above |
| `cargo audit --file Cargo.lock --deny warnings --json` | PASS, zero advisories/warnings |

Dependency setup: root/site `npm ci --ignore-scripts`, Lab `npm ci`, pinned cargo-audit install and
`npm --prefix apps/cerebri-site exec -- playwright install chromium` succeeded.
The first branch CI run passed the Rust, Node, repository, site build, browser and accessibility
gates before the visual hash step exposed stale Linux references. Its uploaded failure artifact was
reviewed at 1440/1024/768/390 px; all four Linux renders matched the intended publication-status text
change and their exact hashes were registered. Exact-head CI remains authoritative for final status.
Pages only deploys main, so this PR's site build is the applicable pre-merge check, not a claimed
deployment of the correction branch.

## Visualizations, lessons and documentation

No new visualization or product feature. The homepage's existing metadata now shows current CPIR
and explicit unreleased/publication status. Its screenshot references require review of that intended
text change. All four Windows full-page renders and all four first-CI Linux renders
(1440/1024/768/390 px) were visually inspected. Exact hashes are maintained separately for Windows
and Linux; unregistered platforms fail rather
than skip comparison. No mismatch threshold or visual gate was relaxed.
The first local build caught links to Markdown heading fragments that this renderer
does not create; links were corrected to canonical documents without weakening prerender checks.
The first Lab run found one old primary-fixture minor-version assertion, now corrected to 0.2.
Historical evidence stays separate from current declarations, and fresh advisory results are dated
evidence rather than timeless security claims.

Updated README/Changelog/roadmap, DE/EN CPIR references, AGENTS/ADR index, Master metadata,
archive provenance, documentation/release and security policies, checks/workflows and this log.

## Next step

Independent release-qualification review of the correction PR, including the documented low-severity
dependency finding, before any subsequent explicit release decision. No release is authorized here.
