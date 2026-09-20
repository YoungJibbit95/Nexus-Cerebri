# Nexus Cerebri agent instructions

Communicate with the maintainer in German. Code, identifiers and technical artifacts use English.
Read [the current Master](docs/architecture/specifications/master-v0.4.md), applicable
[ADRs](docs/architecture/decisions/ADR-0001-workspace.md), and [roadmap](docs/development/roadmap/README.md) before architectural work.

## Implementation
- Finish one bounded roadmap milestone at a time; preserve the strong lifecycle, execution preflight and deterministic core.
- Production must never depend on research. Lab consumes typed core data and never defines planning truth.
- Keep repository structure deliberate: specifications, standards, roadmaps, ADRs and logs belong under docs/.
- Public reference pages require DE/EN counterparts. Preserve historical records as non-normative.
- AI-assisted work must meet the same tests, review, safety and documentation gates as all other contributions.
- Use a separate UI subagent when the maintainer requests parallel Lab work; do not delegate unrelated work automatically.

## Session completion (standing maintainer authorization)
After successful verification, update the detailed changelog and dated progress log, inspect the
diff, create logically separated Conventional Commits, and push the session's work to the intended
remote branch. Do not include unrelated user changes or rewrite published history.
Check the resulting remote CI; never infer success from configuration alone.
Create a version/tag/GitHub release with detailed features, compatibility notes, tests, limitations,
ADRs and documentation references only when a genuine milestone meets the release criteria.
No artificial release/version bump for small sessions. Report a concrete blocker if publishing fails.
