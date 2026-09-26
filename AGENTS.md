# Nexus Cerebri agent instructions

Communicate with the maintainer in German. Code, identifiers and technical artifacts use English.
Before architectural work, read [the current Master](docs/architecture/specifications/master-v0.4.md),
the [ADR index](docs/architecture/decisions/README.md), all Accepted ADRs relevant to every subsystem
you intend to modify, and the [roadmap](docs/development/roadmap/README.md). Follow amendments
identified in the index; an early ADR alone does not describe the current architecture.

## Scope and authority

- Work on one bounded milestone or slice at a time. Do not silently expand scope; stop and report when a requested change would cross the active boundary.
- Preserve deterministic authority boundaries. Rust Core remains the domain and planning authority.
- Production code must never depend on `research/`. Lab, transports and presentation layers consume typed core data and must not define planning truth.
- Learned components may advise or rank only within their explicit contract. They must never become authority for facts, permissions, hard constraints, authorization or execution.
- Preserve the strong lifecycle and execution preflight. Do not weaken validation, permission or execution boundaries to make a feature or test pass.
- Treat contract and version authorities independently. Software, REST API, CPIR schema, model, dataset and release-publication state must not be inferred from one another or changed as a bundle without an explicit requirement.
- Keep repository structure deliberate: specifications, standards, roadmaps, ADRs and logs belong under `docs/`. Public reference pages require DE/EN counterparts; historical records remain non-normative.

## Workspace and Git safety

- Use one canonical workspace for the task. Do not silently create or switch to a replacement clone or worktree; if the canonical workspace is unusable, report the reason before substituting it.
- Preserve unrelated maintainer or user changes. Never overwrite work that is outside the active slice.
- Never force-push, rewrite published history or use destructive Git recovery as a shortcut.
- Inspect the complete diff before committing. Use coherent Conventional Commits and the intended remote branch.
- After the relevant verification succeeds, commit and push the bounded work when the task requires it, then inspect the actual remote CI result.
- Commit/push authorization is not release authorization. Create or change a software version, tag or GitHub Release only after an explicit maintainer instruction for that publication action.

## Verification and qualification

- Run the relevant tests, linters, builds, audits and browser checks; do not claim a gate passed unless it was actually executed successfully.
- Do not weaken thresholds, remove gates or use `continue-on-error` to hide a real failure.
- AI-assisted work must meet the same review, safety, documentation and evidence standards as other contributions.
- The implementation agent may provide implementation evidence, but must not simultaneously self-certify independent Architecture, Math or Security qualification. Use a separate review/qualification step for those claims.
- Check resulting remote CI rather than inferring success from workflow configuration alone.

## Implementation

- Finish one bounded roadmap milestone at a time; preserve the deterministic core, strong lifecycle and execution preflight.
- Prefer explicit, testable changes over speculative abstractions.
- Use a separate UI subagent when the maintainer requests parallel Lab work; do not delegate unrelated work automatically.
