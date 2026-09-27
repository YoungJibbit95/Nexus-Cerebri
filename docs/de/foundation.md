<!-- doc: foundation; lang: de; counterpart: ../en/foundation.md -->
# Foundation-Architektur

## Welche Teile erledigen welche Aufgabe?

Im [Terminbeispiel](introduction.md) kommt 10:00 Uhr als erster Vorschlag zurück.
Dafür arbeiten mehrere Teile zusammen: Einer beschreibt die Eingaben, ein anderer
vergleicht Zeiträume, weitere prüfen Regeln und sortieren die zulässigen Möglichkeiten.
Die Website zeigt anschließend das Ergebnis. Ein Kalendereintrag entsteht dabei noch nicht.

Der Rust-Code ist dafür in Pakete aufgeteilt, sogenannte **Crates**. Die Grenzen helfen,
eine Regel an einer Stelle festzulegen und in verschiedenen Anwendungen gleich anzuwenden.
Die folgenden Abschnitte zeigen den Weg durch den vorhandenen Code.

[English](../en/foundation.md) · [Vom Terminwunsch zu CPIR](cpir.md)

## Der Weg vom Eingang zum Vorschlag

1. **Die Eingaben prüfen.** Eine Anwendung übergibt eine strukturierte `PlanningRequest`.
   Über `cerebri-core` gelangt sie zum Planer. Dessen Eingabeprüfung kontrolliert unter
   anderem Dauer, Zielobjekt, Suchgrenzen und Planungsrechte. Fehlende Pflichtangaben
   stoppen die Suche.
2. **Mögliche Zeiten ausprobieren.** Der Planer erzeugt Starts im angegebenen Raster.
   Die Zeitfunktionen aus `cerebri-temporal` und die Regeln aus `cerebri-constraints`
   helfen zu prüfen, ob ein Kandidat passt. Im Beispiel scheiden vier Starts wegen
   Überschneidungen aus.
3. **Die übrigen Möglichkeiten vergleichen.** `cerebri-preferences` berechnet Werte für
   den Vergleich, etwa den Abstand zu einer angegebenen Wunschzeit. Der Planer verwendet
   diese Werte in seiner festen Sortierreihenfolge. Im Beispiel gibt es keine Wunschzeit;
   der frühere Beginn entscheidet den Gleichstand zwischen 10:00 und 10:15 Uhr.
4. **Das Ergebnis zurückgeben.** Ein `PlanningResult` enthält Kandidaten, Ablehnungsgründe
   und Angaben zur durchsuchten Menge. `cerebri-core` gibt dieses Ergebnis an die aufrufende
   Anwendung zurück. Die vorgeschlagenen Platzierungen sind noch keine Ausführungsfreigabe.

`cerebri-core` ist der gemeinsame Einstieg für Anwendungen. Die eigentliche Suche liegt
in `cerebri-planner`; Core leitet den Aufruf dorthin weiter. So verwenden REST und die
Node-Anbindung denselben Planer.

## Wo die Regeln im Code liegen

| Aufgabe | Zuständiger Code |
| --- | --- |
| Kennungen, Herkunft und Zustände wie bekannt oder fehlend darstellen | [cerebri-types](../../crates/cerebri-types/src/lib.rs) |
| Zeitpunkte, Dauer, Intervalle und Zeitzonen behandeln | [cerebri-temporal](../../crates/cerebri-temporal/src/lib.rs) |
| Fakten und verbindliche Regeln darstellen und prüfen | [cerebri-constraints](../../crates/cerebri-constraints/src/lib.rs) |
| Angegebene Präferenzen auswerten und Vergleichswerte berechnen | [cerebri-preferences](../../crates/cerebri-preferences/src/lib.rs) |
| Semantische Angaben wie Dringlichkeit mit ihrer Herkunft darstellen; im Terminbeispiel ungenutzt | [cerebri-semantics](../../crates/cerebri-semantics/src/lib.rs) |
| CPIR-Eingaben beschreiben, Kandidaten suchen und Übergänge bis zur Autorisierung prüfen | [cerebri-planner](../../crates/cerebri-planner/src/lib.rs) |
| Anwendungen gemeinsame Funktionen für Eingabeprüfung, Planung und Zeitdiagnosen anbieten | [cerebri-core](../../crates/cerebri-core/src/lib.rs) |
| Autorisierte Aktionen vor der Ausführung erneut prüfen und an einen Adapter übergeben | [cerebri-integrations](../../crates/cerebri-integrations/src/executor.rs) |

Insbesondere leben Planungsanfragen und die Planstufen in `cerebri-planner`. Gemeinsame
Grundtypen in `cerebri-types` enthalten nicht die gesamte Planungslogik. Der Executor und
das Ausführungsprotokoll `ActionLedger` gehören zu `cerebri-integrations`.

## Wie Anwendungen auf den Planer zugreifen

Die [REST-Anwendung](../../apps/cerebri-api/src/main.rs) nimmt Anfragen entgegen und ruft
Core auf. Die [Node-Anbindung](../../bindings/node/README.md) startet dafür derzeit eine
Rust-Prozessbrücke. Beide Wege verwenden die Rust-Berechnung. Weder ein HTTP-Aufruf noch
ein Aufruf aus JavaScript fügt eine eigene Planungsregel hinzu. Es gibt keinen
Ausführungsendpunkt für Kalenderänderungen.

Das [Lab](../../apps/cerebri-lab/README.md) sendet Anfragen an die API und zeigt deren
Antworten. Die öffentliche Website funktioniert anders: Beim Erstellen der Seiten führt
[das Build-Skript](../../scripts/build-site-data.mjs) Rust-Beispiele aus und speichert
deren Ergebnisse für die Anzeige. Ein Besuch der Website startet keine neue Planersuche.
Beide Oberflächen stellen Ergebnisse dar, statt eine zweite Rangfolge zu berechnen.

## Abhängigkeit, Datenfluss und Prüfschritt auseinanderhalten

Diese drei Beziehungen beantworten unterschiedliche Fragen:

- **Code-Abhängigkeit:** Welches Paket darf Typen oder Funktionen eines anderen Pakets
  verwenden? Zum Beispiel importiert `cerebri-integrations` Typen aus `cerebri-planner`.
  Daraus folgt kein Aufruf des Executors durch den Planer.
- **Datenfluss:** Wohin gehen Anfrage und Ergebnis? Eine Lab-Anfrage geht über die API
  und Core zum Planer; das Ergebnis kommt über dieselben Schnittstellen zurück.
- **Prüfschritt:** Welche Bedingung muss erfüllt sein, bevor ein Plan die nächste Stufe
  erreicht? Aus einem Vorschlag wird erst nach erfolgreicher Validierung ein `ValidatedPlan`.

Ausgewählte Code-Abhängigkeiten sind:

- `cerebri-api` verwendet `cerebri-core`.
- `cerebri-core` verwendet `cerebri-planner` und `cerebri-temporal`.
- `cerebri-planner` verwendet unter anderem `cerebri-constraints` und `cerebri-preferences`.
- `cerebri-constraints` verwendet `cerebri-temporal`.
- `cerebri-preferences` verwendet `cerebri-types`.
- `cerebri-integrations` verwendet `cerebri-planner`.

Diese Liste beschreibt Imports, keine Laufzeitaufrufe. Core importiert weder Integrations noch ML.
Der [Repository-Check](../../scripts/check-repository.mjs) prüft eine ausdrückliche Liste
erlaubter Crate-Abhängigkeiten; Cargo verhindert Zyklen. Die Grundpakete `cerebri-types`
und `cerebri-temporal` hängen von keinem höheren Cerebri-Paket ab.

## Warum der Vorschlag noch nicht ausgeführt werden darf

Der Planer hat mögliche Platzierungen geprüft und verglichen. Eine Anwendung muss vor
einer Änderung trotzdem den vollständigen Plan gegen den aktuellen Kontext validieren,
die Aktionen festlegen und ihre Berechtigung prüfen lassen. Diese Schritte liegen im
[Lifecycle-Code](../../crates/cerebri-planner/src/lifecycle.rs). Eine vertrauenswürdige
Anwendung liefert dafür die aktuellen Rechte und erforderlichen Bestätigungen.

Nur ein `AuthorizedActionPlan` kann an den
[Executor](../../crates/cerebri-integrations/src/executor.rs) übergeben werden. Auch dann
prüft dieser unmittelbar vor einer Änderung erneut, ob Daten und Berechtigungen noch
passen. Ein einmal passender Termin kann inzwischen belegt oder eine Berechtigung
entzogen worden sein.

Die vorhandenen Adapter demonstrieren diesen Ablauf mit Daten im Arbeitsspeicher.
Produktive Kalenderanbindungen und eine produktive Authentifizierung sind noch nicht
implementiert. Das Beispiel auf der Website endet bereits beim Vorschlag.

### Technische Details der Planstufen

Lifecycle: PlanningRequest -> ProposedPlan -> ValidatedPlan -> ActionPlan ->
AuthorizedActionPlan -> ExecutionResult. Vorschläge dürfen öffentlich erstellt werden und
sind ungeprüft. Die Folgestufen haben private Felder, kontrollierte Übergänge und keine
Deserialize-Implementierung. Der Executor lehnt die drei früheren Planstufen bereits beim Kompilieren ab.

Fakten, Constraints, semantische Evidenz, Präferenzen und Rechte sind getrennte Typen.
Die Anfrage hält einen Policy-Snapshot fest. Aktuelle Autorisierung liefert die vertrauenswürdige
Anwendung, niemals der REST-Aufruf. READ, PLAN und objektbezogene EXECUTE-Rechte sind getrennt.
Rechte binden Aktionsart, Objekt, Kalender und Integration.

Vor der ersten Provider-Mutation reserviert ActionLedger atomar den Plan und alle Aktionsschlüssel.
Danach werden vollständiger Kontext, Revision, aktuelle Policy, Planungsrechte, Ausführungsrechte,
Bestätigung und Adapterfähigkeiten erneut geprüft. Der Adapter muss Objektversionen atomar prüfen.
Beim ersten Fehler endet die Ausführung; verbleibende Aktionen sind Skipped. Frühere Erfolge werden
nicht zurückgerollt. Ein verlorener Ledger-Abschluss oder ungewisser Provider-Ausgang führt zu
RecoveryRequired. Reservierte Schlüssel bleiben gesperrt. Die In-Memory-Implementierungen
überleben keinen Neustart; dauerhafte Speicherung, Wiederherstellung und Retry-Policy folgen später.

Plan-IDs entstehen aus SHA-256 über kanonisierte Anfrage und Platzierungen.
Aktions-ID und Idempotenzschlüssel ergänzen einen stabilen Index. Bestätigung gilt für genau
diesen Plan einschließlich Policy und Snapshot. Kontextänderungen erfordern erneute Validierung
und Autorisierung.

## Wo spätere Lernverfahren hingehören

[cerebri-ml](../../crates/cerebri-ml/src/lib.rs) enthält Metadaten für Modelle und
Datensätze sowie Schnittstellen für Registrierung und Inferenz. Daraus folgt noch kein
trainiertes Modell oder laufendes Lernverfahren. Der aktuelle Planer hängt nicht von
diesem Paket ab.

Die Lernarbeiten unter [research/](../../research/ml-from-scratch/README.md) liegen
außerhalb des produktiven Workspace. Produktiver Code darf sie nicht importieren.
Später könnten lernende Verfahren bei der Suche oder beim Vergleich zulässiger
Möglichkeiten helfen. Fakten, verbindliche Regeln und Ausführungsrechte müssten
weiterhin durch ausdrückliche Prüfungen geschützt bleiben.

[Lernweg und Forschungsfragen](research.md): Was später gelernt werden könnte und was heute bereits existiert.

## Suchverfahren und aktuelle Grenzen

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
