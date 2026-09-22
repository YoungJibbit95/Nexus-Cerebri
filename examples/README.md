# Nexus Cerebri — CPIR Complexity Scenario Suite

Diese Suite wurde aus `legacy-cpir-0.1.json` abgeleitet.

- `00-legacy-baseline-0.1.json`: Legacy-Kompatibilitätskontrolle.
- `01` bis `10`: normale neue Szenarien in CPIR 0.2, aber ohne Temporal-Series-Felder.
- Die `expected_*`-Werte in `manifest.json` sind **Oracles vor Ausführung**, keine behaupteten Lab-Ergebnisse.

## Reihenfolge

1. `01-basic-feasible.json` — Atomic
2. `02-ranking-preferred-start.json` — Preference/Ranking
3. `03-halfopen-touching-boundary.json` — Half-open Boundary PASS
4. `04-halfopen-one-second-overlap.json` — Paired +1s Counterexample
5. `05-dense-three-exact-gaps.json` — Dense Planning
6. `06-constraint-composition.json` — Constraint Composition
7. `07-dependency-chain.json` — Dependency DAG
8. `08-resource-scope-subset-rejection.json` — Scope ALL-containment
9. `09-budget-best-found.json` — SearchAssessment / Budget truncation
10. `10-compound-unique-slot.json` — Compound unique-solution challenge

## Wichtig

Importiere jeden Request einzeln ins Cerebri Lab und vergleiche danach:
Validation State, Outcome, SearchAssessment, Candidate Count, evaluated positions,
erste Platzierung, ordering_key, rejection reasons und Dependency/Scope Evidence
mit `manifest.json`.
