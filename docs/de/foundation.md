<!-- doc: foundation; lang: de; counterpart: ../en/foundation.md -->
# Foundation-Architektur

## Einfach

Der Planer schlägt eine Zeit vor. Der Validator prüft sie. Eine getrennte Anwendungsschicht
prüft aktuelle Rechte und Bestätigung, bevor ein Mock-Adapter seinen Speicherzustand ändern
darf. Die Planung selbst verursacht keine externen Änderungen.

## Technisch

```text
API / Node -> core -> planner -> constraints / semantics / preferences
                        |              |           |           |
                        +--------------+-----------+-----------+-> temporal / types
integrations -> planner (Lifecycle-Nachweise), constraints (Fakten), temporal, types
ml -> temporal / types
research -> keine Produktionsabhängigkeit
```

Pfeile bedeuten Imports, keine Laufzeitaufrufe. Core importiert weder Integrations noch ML.
Der Architekturcheck prüft eine ausdrückliche Liste erlaubter Crate-Abhängigkeiten; Cargo verhindert Zyklen.

Lifecycle: PlanningRequest -> ProposedPlan -> ValidatedPlan -> ActionPlan ->
AuthorizedActionPlan -> ExecutionResult. Vorschläge dürfen öffentlich erstellt werden und
sind ungeprüft. Die Folgestufen haben private Felder, kontrollierte Übergänge und keine
Deserialize-Implementierung. Der Executor lehnt die drei früheren Planstufen bereits beim Kompilieren ab.

Fakten, Constraints, semantische Evidenz, Präferenzen und Rechte sind getrennte Typen.
Die Anfrage hält einen Policy-Snapshot fest. Aktuelle Autorisierung liefert die vertrauenswürdige
Anwendung, niemals der REST-Aufruf. READ, PLAN und objektbezogene EXECUTE-Rechte sind getrennt.
Rechte binden Aktionsart, Objekt, Kalender und Integration.

Vor der ersten Provider-Mutation reserviert ActionLedger atomar den Plan und alle Aktionsschlüssel.
Danach werden vollständiger Kontext, Revision, aktuelle Policy, Planungskompetenz, Ausführungsrechte,
Bestätigung und Adapterfähigkeiten erneut geprüft. Der Adapter muss Objektversionen atomar prüfen.
Beim ersten Fehler endet die Ausführung; verbleibende Aktionen sind Skipped. Frühere Erfolge werden
nicht zurückgerollt. Ein verlorener Ledger-Abschluss oder ungewisser Provider-Ausgang führt zu
RecoveryRequired. Reservierte Schlüssel bleiben gesperrt. Die In-Memory-Implementierungen
überleben keinen Neustart; dauerhafte Speicherung, Wiederherstellung und Retry-Policy folgen später.

Plan-IDs entstehen aus SHA-256 über kanonisierte Anfrage und Platzierungen.
Aktions-ID und Idempotenzschlüssel ergänzen einen stabilen Index. Bestätigung gilt für genau
diesen Plan einschließlich Policy und Snapshot. Kontextänderungen erfordern erneute Validierung
und Autorisierung.

## Forschungstiefe und Grenzen

Der Suchraum umfasst ein einzelnes Event an horizon.start + k * granularity, für ganzzahliges
k >= 0, mit fester Dauer und Ende innerhalb des Horizonts. Das Beispiel verwendet 900 Sekunden.
Jede Position durchläuft Scope-, Fakten- und Constraint-Prüfung vor der Bewertung.

Kosten sind der absolute Abstand in Sekunden zur bevorzugten Startzeit der stärksten Quelle;
ohne Präferenz gelten null Kosten. Vorrang: aktuelle explizite Anfrage, Sitzungskontext,
persönlich gelernt, global gelernt, Standard. Bei gleicher Quelle gewinnt die frühere Präferenzzeit.
Sortierung: Kosten, Mutationszahl, gesamte Verschiebung in Sekunden, Startzeit, Objekt-ID.
Nur ein vollständig durchsuchtes Raster mit Lösung erhält ProvenOptimal. Das beweist Optimalität
ausschließlich in diesem Raster und für diese Zielfunktion, nicht für kontinuierliche Zeit oder
zukünftige Reparatursuchen. Vollständig erfolglose Suche ist Complete/NoSolution.
Abgebrochene Suche ist BestFound; ohne bisherigen Kandidaten bedeutet NeedsRelaxation,
dass Budget oder deklarierter Suchraum überdacht werden müssen.

Alle Event-/Task-Zeiten des Kontexts blockieren konservativ Überlappungen, unabhängig von
Kalender und Ressource. Mehrressourcenplanung und kalenderübergreifende Parallelität folgen später.
Der Basisplaner sucht ein Zielobjekt. Die Lifecycle-Prüfung akzeptiert explizite Mehrfachplatzierungen,
führt aber keine Batch-Suche oder sichere Provider-Reihenfolge für voneinander abhängige Moves aus.
Move erhält die Dauer. Update/Delete sind vorbereitet; CANCEL wird nicht automatisch zum Löschen.
Recurrence-Constraints werden bis zur Implementierung abgelehnt.

ConflictSet enthält tatsächlich beteiligte Constraints/Fakten abgelehnter Rasterpositionen,
ohne Minimalitätsgarantie. Feste Budgets und erschöpfende Intervalltests auf einem kleinen
Definitionsbereich vermeiden unkontrollierte Zeit- und Zufallsabhängigkeit.

[CPIR-Referenz](cpir.md) · [Tests](development.md) · [English](../en/foundation.md)

Core importiert temporal zusätzlich direkt für typisierte Diagnosen; siehe [Temporal Core](temporal.md).
