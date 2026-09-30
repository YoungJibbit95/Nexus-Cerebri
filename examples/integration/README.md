# Application integration conformance fixture

`suggestion.json` is synthetic Integration 0.1 / CPIR 0.2 data. It contains no real
Task, account or calendar information. Read it as an envelope, not directly as CPIR.

Its prospective Event needs 1,800 seconds inside 09:00–12:00 UTC on 2026-10-01, with
09:00–10:00 occupied. The fixture explicitly certifies complete synthetic coverage.
The 900-second grid has seven feasible starts, 10:00 through 11:30, in that order.
They are non-executable proposals. Changing coverage to Incomplete produces
InsufficientInformation and no free-time candidates.

Rust core, REST and Node tests share this fixture. The Node test suite additionally
checks a fully occupied horizon, bounded search and failures of disposable processes.
Existing planner fixtures cover recurrence, dependencies and DST separately; this fixture
does not certify a Nexus domain mapping or production data completeness.

See [the application reference](../../docs/en/application-integration.md) and
[the Nexus implementation prompt](../../docs/development/prompts/nexus-ecosystem-integration-foundation.md).
