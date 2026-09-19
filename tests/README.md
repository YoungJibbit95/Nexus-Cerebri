# Foundation verification

Crate unit/integration tests live next to their owners; support/mod.rs shares synthetic fixtures.
cerebri-integrations doctests verify that ProposedPlan, ValidatedPlan and ActionPlan cannot enter execute.
scripts/check-repository.mjs enforces the architectural dependency allowlist.
No test needs an external provider, database, credentials or current system time.
[Verification commands](../docs/en/development.md)

