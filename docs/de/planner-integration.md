<!-- doc: planner-integration; lang: de; counterpart: ../en/planner-integration.md -->
# Wie Cerebri mögliche Zeiten vergleicht

Cerebri sucht eine Zeit, die zu den übergebenen Informationen und verbindlichen Regeln
passt. Anschließend vergleicht es die übrigen Möglichkeiten in einer festen Reihenfolge.
Bei gleichen Eingaben liefert der aktuelle Planer dieselbe Reihenfolge. Das bedeutet hier
**deterministische Planung**.

## Ein Termin als Beispiel

Das [Beispiel der Website](../../examples/request.json) sucht 30 Minuten zwischen 09:00
und 12:00 Uhr UTC. Ein vorhandener Termin belegt 09:00–10:00 Uhr. Mögliche Startzeiten
liegen jeweils 15 Minuten auseinander. So prüft der Planer 11 Starts von 09:00 bis 11:30 Uhr.

Vier Möglichkeiten überschneiden sich mit dem vorhandenen Termin und scheiden aus.
Sieben bleiben übrig, alle ab 10:00 Uhr. Jede mögliche Platzierung heißt **Kandidat**.
Im Beispiel gibt es keine Wunschzeit, und die übrigen Vergleichswerte sind gleich.
Deshalb steht der frühere Start vorn: 10:00 Uhr UTC. Gebucht wird dadurch noch nichts.

## Wünsche helfen bei der Auswahl gültiger Möglichkeiten

Eine bevorzugte Startzeit ist ein Wunsch, etwa 10:45 Uhr. Eine verbindliche Regel muss
jede zulässige Möglichkeit erfüllen, etwa einen belegten Zeitraum freizuhalten.
Ein Wunsch kann keinen Regelverstoß ausgleichen.

Das separate [Beispiel mit Wunschzeit](../../examples/02-ranking-preferred-start.json)
zeigt diesen Vergleich. Der Planer vergleicht zuerst den Abstand zur Wunschzeit, danach
die Anzahl der Änderungen, die zeitliche Verschiebung, den Start selbst und die Objekt-ID.
Der erste Unterschied entscheidet die Reihenfolge. Das sind berechnete Werte, keine
erlernten Bewertungen.

## Wie viel wurde tatsächlich geprüft?

Das Ergebnis nennt sowohl den Ausgang als auch den Umfang der Suche. Beides gehört zusammen:

- `ProvenOptimal`: Alle Starts im vorgegebenen Zeitraster wurden geprüft, und mindestens
  eine zulässige Möglichkeit wurde gefunden. Nach den aktuellen Vergleichsregeln steht
  die beste Möglichkeit **innerhalb dieses Rasters** vorn. Über Zeiten außerhalb des
  Suchfensters oder zwischen den Rasterschritten sagt das nichts aus.
- `Complete`: Das Raster wurde vollständig geprüft, ohne eine zulässige Möglichkeit zu finden.
- `BestFound`: Die Suche belegt keine dieser beiden vollständigen Aussagen. Sie kann ihre
  Kandidatengrenze erreicht haben oder gar nicht erst gestartet sein. Deshalb gehören der
  Ausgang, der Prüfbericht und die Zahl geprüfter Starts dazu. Der Name allein bedeutet
  nicht, dass eine Lösung vorliegt.

Fehlende notwendige Informationen werden als `InsufficientInformation` gemeldet. Aus unbekannter
Verfügbarkeit macht Cerebri keine freie Zeit. Ein anderes Suchfenster, andere Regeln oder
andere Eingaben ergeben eine neue Planungsfrage.

## Was ist mit wiederkehrenden Terminen?

Bestehende tägliche oder wöchentliche Serien können Zeiten belegen. Vor der Suche leitet
der Planer ihre einzelnen Termine für den angefragten Zeitraum ab. Eine Wochenregel,
ihr konkreter Termin am Dienstag und ein neu zu platzierender Termin sind unterschiedliche Dinge.

Dadurch wird keine neue Terminserie erstellt, und es werden nicht mehrere Termine
gemeinsam umgeplant. Die automatische Suche platziert derzeit ein Ziel anhand der
übergebenen Ausgangslage.

[Eingaben verstehen](cpir.md) · [Zeit und Verfügbarkeit](temporal.md) ·
[Vom Vorschlag zur erlaubten Änderung](safety.md)

## Technisch

CPIR `0.2` ergänzt optional `context.temporal` mit `horizon`, `coverage`, gemeinsamen
`limits` und `series`. Jede Serie hat `id`, `state`, `provenance`, `evidence` und `rule`.
`Existing { revision }` mit faktischer Herkunft lässt sich kompilieren; `Prospective` wird
abgelehnt. `compile_snapshot` erzeugt eine `CompiledContextSnapshot`-Sicht auf den
Quell-ContextSnapshot, ohne Provider-Objekte oder Änderungsrechte zu erzeugen. Scope und
Horizont müssen übereinstimmen. CPIR `0.1` bleibt ohne temporale Eingaben unterstützt;
REST `/v1` und Software `0.2.0` (unveröffentlicht) sind unabhängige Versionen. Der alte
Snapshot beschreibt einen geschlossenen Eingabekontext, keine zugesicherte Provider-Abdeckung.

Der Compiler bewahrt den vollständigen Zeitraum und dessen halboffenen Horizont-Ausschnitt.
Occurrence-IDs hashen Serien-ID und nominales lokales Datum/Zeit/Zeitzone unabhängig von
Horizont, Ausschnitt und Quellrevision. Doppelte Serien und Kollisionen mit Objekt- oder
Occurrence-IDs brechen die Kompilierung atomar ab. Aufrufer dürfen dasselbe externe
Vorkommen nicht zusätzlich als Objekt unter einer anderen Identität einspeisen.
Provider-Abgleich bleibt spätere Arbeit.

Bei einer doppelten lokalen Uhrzeit behalten Earlier/Later-Auswahlen dieselbe nominelle
Occurrence-ID; UTC-Zeiträume und Auflösungsbelege unterscheiden sich. Die Planung nutzt
die validierte unveränderliche Kompilierungssicht. Compilerfehler dürfen nicht auf die
ältere Planung ohne zeitliche Quelldaten zurückfallen. Die Lifecycle-Validierung prüft
den aktuellen Snapshot unabhängig und kompiliert ihn erneut.

Alle gelieferten Event/Task-Objekte und kompilierten Vorkommen blockieren konservativ
Überlappungen, auch außerhalb des Änderungsscopes. Unvollständige Abdeckung liefert einen
unbekannten Rest und `InsufficientInformation`, niemals freie Slots. RequiredBuffer prüft
vollständige Zeiträume; auch der gepufferte Kandidat muss im kompilierten Horizont liegen.
Recurrence-Constraints, prospektive Serien, Serienänderungen, Ausnahmen und Serienpuffer
bleiben unimplementiert.

DependencyOrder bedeutet predecessor.end <= dependent.start. Der Graph sortiert und
dedupliziert Kanten, meldet fehlende Referenzen und findet zyklische stark zusammenhängende
Komponenten iterativ samt Mitgliedern und Kanten. Ungültige Graphen behaupten keine Ordnung.
Die Einzelevent-Rastersuche arbeitet gegen feste Vorgänger; Batch-Vorschläge benötigen
weiter die vollständige Lifecycle-Validierung. Graphordnung ist keine Ausführungsreihenfolge
und verleiht keine Autorisierung.

Die Sortierung ist lexikografisch: Präferenzabstand in Sekunden, Änderungszahl, Verschiebung
in Sekunden, Startzeitpunkt, Objekt-ID. `ordering_key` zeigt jede Komponente; `explanation`
benennt die ausgewählte Präferenzquelle. Die Quellenrangfolge bleibt erhalten; Lernen wird
nicht implementiert. Vollständig durchsuchtes Raster mit Lösung => ProvenOptimal **für dieses
Raster und Ziel**; vollständige Ablehnung => Complete; Budgetabbruch => BestFound.

## Forschung und Grenzen

### Ranking Feature Contract v0.1

Jeder Kandidat enthält `ranking_features`: einen versionierten deterministischen
Beobachtungsvertrag für Inspektion und Parität, keinen endgültigen Merkmalsraum für
Präferenzlernen. Er enthält genau `schema_version: {major: 0, minor: 1}`,
`preferred_start_distance_seconds`, `preferred_start_source`, `mutation_count` und
`shift_seconds`. Die Version ist unabhängig von CPIR, Software, API, Modell, Datensatz
und Spezifikation; keine dieser anderen Versionen wird angehoben.

Die Präferenzauflösung wählt das Minimum von `(source_rank, preferred_start)` mit
ExplicitCurrentRequest=0, SessionContext=1, PersonalLearned=2, GlobalLearned=3, Default=4.
Innerhalb einer Quelle gewinnt der frühere Zeitpunkt; exakte Duplikate sind ranggleich.
Eingabereihenfolge und Inhalt des Evidenzvektors beeinflussen die Auflösung nicht.
Ein leeres Profil liefert keine Evidenz. Die Quelle dokumentiert die Herkunft derselben
aufgelösten Präferenz und ist kein Bestandteil der Sortierung.

Alle Felder sind Pflichtfelder. Für **beide** nullable Felder gilt: fehlend => abgelehnt,
explizites `null` => None, typkorrekter Wert => Some(value). Distanz ist genau dann None,
wenn die Quelle None ist. Unbekannte Versionen/Felder und widersprüchliche Nullbarkeit
werden abgelehnt. Der private Rust-Wire-Typ erzwingt Feldpräsenz durch `deserialize_with`
ohne Defaults und prüft anschließend Version und Konsistenz vor Erzeugung des Domänentyps.
Gewöhnliche Serde-Option-Felder würden fehlend und null zusammenführen; die vollständige
Präsenztabelle ist mit echtem JSON getestet. Serialisierung gibt beide Felder auch als null
aus; Roundtrips erhalten None.

Die Distanz bleibt `(candidate_start - resolved_start).num_seconds().unsigned_abs()`.
None unterscheidet sich von Some(0): Some(0) bedeutet vorhandene Evidenz mit auf ganze
Sekunden quantisiertem Nullabstand, nicht zwingend Zeitpunktgleichheit. Gleichheit und
Abstände von +/-0,8 Sekunden ergeben null; +/-1 Sekunde ergibt eins. Die bestehende
chrono-Arithmetik benötigt weder Clamping noch Sättigung.

Änderungszahl ist `m(candidate, request_context)`: analysis-only (FindSlot, Analyze oder
max_mutations=0) => 0; aktueller mutierender Einzelzielpfad => 1. Verschiebung ist
`(original_start - candidate_start).num_seconds().unsigned_abs()` oder `0` ohne bestehende
Platzierung. Prospektive, unveränderte und von null verschiedene Subsekundenverschiebungen
erhalten absichtlich denselben numerischen Wert `0`.

Der vollständige Schlüssel bleibt `(distance.unwrap_or(0), mutation_count, shift_seconds,
start, object_id)` und verwendet dieselben Beobachtungswerte. Die bisherige None-zu-0-
Projektion der Sortierung löscht damit nicht die Domänenbedeutung. Kompatibilitätsziel ist
identische Reihenfolge für alle unterstützten gültigen Eingaben einschließlich Subsekunden.
Begrenzte Regressionstests liefern Evidenz, keinen universellen Beweis. Generierung,
Validierung, SearchAssessment und Ausführung bleiben unverändert.

REST und Node transportieren Core-Ausgaben; Lab prüft und zeigt sie ohne Neuberechnung.
Alte importierte Lab-Ergebnisse ohne `ranking_features` werden abgelehnt. Bisherige numerische
Kandidatenfelder und der Rust-Kompatibilitätshelfer `RankingFeatures` bleiben erhalten.
Strikte Konsumenten müssen diese additive vorläufige Ausgabeerweiterung berücksichtigen.
JavaScript-Guards behalten ihre Safe-Integer-Anzeigegrenze; tatsächliche Core-Zeitabstände
passen hinein, während Rust den vollständigen u64-Wertebereich akzeptiert. Dieser
Ranking-Vertrag ergänzt weder Lernen und Training noch Telemetrie, das Sammeln von
Evaluationsläufen, gelernte Suche oder Anbieteranbindungen. Die getrennten
Evaluationsgrundlagen sind unten beschrieben. Rohinhalte und Identifikatoren sind keine
Features. Siehe [ADR-0014](../architecture/decisions/ADR-0014-ranking-feature-contract.md).

Maximal 32 Serien, 1.024 Vorkommen, 36.600 gemeinsam geprüfte Tage, 256 Graphknoten und
1.024 Constraints. Die vorhandene kombinierte Arbeitsgrenze von 1.000.000 schließt Vorkommen
ein. Limits brechen vollständig ab und kürzen keine Abdeckung. Der Graph benötigt höchstens
V begrenzte Traversierungen von V Knoten und E Kanten, O(V(V+E)), ohne Rekursion.

[Fixtures mit unabhängigen Erwartungen](../../examples/planner/manifest.json) prüfen
Planbarkeit, harte Ablehnung, fehlende Information, Scope, Abhängigkeiten/Zyklen, Rekurrenz,
Duplikate, DST und unvollständige Abdeckung. Rust-API- und echte HTTP-zu-TypeScript-Tests
verwenden diese Eingaben. Negative Vertragstests erkennen Abweichungen neuer Felder.
Schemagenerierung wird bei mehreren gepflegten Clients oder wiederholter Vertragsdrift
sinnvoll; derzeit reichen kleine explizite Paritätstests für den internen Lab-Vertrag.

Earlier/Later bei einer DST-Faltung behalten absichtlich dieselbe nominale Occurrence-ID;
UTC-Intervall und Auflösungsevidenz unterscheiden sich. Die Planung übernimmt die bereits
validierte unveränderliche Compilation. Compilerfehler können nicht in Legacy-Planung
übergehen; die spätere Lifecycle-Validierung prüft und kompiliert den aktuellen Snapshot
weiterhin unabhängig.

Die [Verifikationskampagne vom 2026-09-21](../development/progress/2026-09-21-planner-verification.md)
prüft kleine generierte Zeitpläne, alle gerichteten Dreiknotengraphen, generierte Rekurrenz,
fehlerhaftes JSON und echte API/Lab-Verträge unabhängig gegen. Das sind begrenzte Property-Tests,
keine formale Verifikation. Manuelle Guards prüfen jetzt Validierungsvarianten und begrenzte
IDs. Schemagenerierung bleibt zurückgestellt: Die gefundenen Lücken waren örtlich fehlende
Guards, keine wiederkehrenden Schemaänderungen über mehrere Clients.

[ADR-0013](../architecture/decisions/ADR-0013-planner-resource-admission.md) ergänzt folgende
Grenzen: kompaktes Request-JSON <= 256 KiB, Request-Bytes mal Kandidatenbudget <= 16 MiB und
materialisierte Occurrence-Evidenz <= 1 MiB. Die bisherigen Arbeitslimits gelten zusätzlich.
Ein Überlauf des lokalen Datums lehnt den Kandidaten kontrolliert ab. Daraus folgt weder eine
Funktionserweiterung noch eine Veröffentlichung.

Die abgeschlossenen [Evaluationsgrundlagen](../architecture/decisions/ADR-0015-evaluation-contract-foundations.md)
aus Slice 2, Phase A ergänzen den geschlossenen Episode-Wire-Vertrag, einen separaten
wire-validierten Domain-Container und strikte Skalar-/Manifest-, Provenance- und Run-Binding-Typen
in Rust. Der partielle Phase-B-Stand ergänzt kanonische Wert-/Regel-Grammatiken, reine
Candidate-/Hypothesen-Fingerprints und eine gemeinsame Admission-Work-Messung. Phase B.1 ergänzt
begrenzte Graph-Kanonisierung mit einer Schnittstelle für die vollständige Payload. Phase B.2
ergänzt die vollständige BaseScenario-/BSF-Projektion und separate Identity-/Revision-Bindings.
Phase B.3 ergänzt DecisionInput/D1 aus einem frischen BSF, dessen Bindings, typisierten
Request-Eingaben und der gemeinsamen Admission-Work-Messung; semantische Artefaktidentitäten
werden explizit aus vertrauenswürdiger Quelle übergeben. DecisionObservation, semantische
Lifecycle-/Collection-Validierung und Replay sind noch nicht fertig.
Planner- und Transportverhalten bleiben unverändert;
Live-Erfassung, Telemetrie und Lernen sind weiterhin nicht vorhanden. Die gezielte unabhängige
Data-/Hard-Math-Qualifikation der Regel-Byte-Grammatik ist abgeschlossen; Phase B bleibt partiell.

[ADR-0012](../architecture/decisions/ADR-0012-planner-snapshot-compilation.md) ·
[Temporale Referenz](temporal.md) · [CPIR-Referenz](cpir.md) · [English](../en/planner-integration.md)
