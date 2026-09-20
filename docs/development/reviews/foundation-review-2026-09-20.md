# Foundation review resolution — 2026-09-20

The review was checked against main at 260a590 (Foundation 0.1.0, no prior release).
The existing architecture is retained; this is a targeted clarification and guard update.

| Finding | Status | Evidence / resolution |
| --- | --- | --- |
| 1. Resource ALL versus ANY ambiguity | Confirmed ambiguity; clarified and verified | Existing ALL behavior retained. ADR-0008 and Master specify full containment. Nine scope tests cover omitted, empty, one/partial/full/disjoint/resource-free, capability separation and wire preservation. |
| 2. Prospective object versus existing provider state | Boundary clarified; mock strengthened | revision=None is prospective, Some is existing. CREATE does not require a preexisting remote row. Mock rejects incompatible action/revision combinations; create promotion and unchanged-state rejection tests pass. No external-object redesign. |
| 3. Configured CI versus actual remote result | Baseline verified; final release gated | Foundation run 35443973099 succeeded on 260a590. Publication requires a successful remote run for the final session commit; record the observed result in progress/release notes. |
| 4. None versus [] | Verified; preserved | Serde omitted/null yields None, explicit [] remains empty for all five optional scope lists. New scope tests plus existing lifecycle tests pass. |
| 5. Half-open interval boundaries | Verified; preserved | Touching is not overlap. Union may join touching busy coverage without inventing a free interval. Exhaustive partition/subtraction and endpoint tests pass. |
| 6. Node binding status | Verified; explicitly deferred | The asynchronous process bridge remains provisional, with the same Rust core and no JS planner. Native N-API/Electron packaging is not claimed. |
| 7. Rust pin | Verified; unchanged | rust-toolchain.toml remains 1.97.0; local checks and CI use the same pin. No downgrade. |

The private lifecycle stages, lack of Deserialize on authorization proofs, final preflight,
idempotency, ordered partial results, typed knowledge states, CPIR 0.1 and deterministic
planner semantics remain in place. Three compile-fail doctests protect execution boundaries.

[ADR-0008](../../architecture/decisions/ADR-0008-scope-and-prospective-identity.md) ·
[Scope tests](../../../crates/cerebri-planner/tests/scope_review.rs) ·
[Execution tests](../../../crates/cerebri-integrations/tests/execution.rs) ·
[Baseline CI](https://github.com/YoungJibbit95/Nexus-Cerebri/actions/runs/35443973099)
