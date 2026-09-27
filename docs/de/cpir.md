<!-- doc: cpir; lang: de; counterpart: ../en/cpir.md -->
# CPIR 0.2 und Schnittstellen

CPIR ist das Datenformat, in dem Cerebri eine Planungsaufgabe erhält. Dauer, belegte
Zeiten, Regeln und Berechtigungen stehen in eigenen Feldern. So kann der Planer prüfen,
welche Angaben vorliegen und welche noch fehlen. Der vollständige Name lautet
*Cerebri Planning Intermediate Representation*.

[English](../en/cpir.md) · [Projektgeschichte und Planungsbeispiel](introduction.md)

## Vom Terminwunsch zur Anfrage

Das [Beispiel im Repository](../../examples/request.json) beschreibt diese Aufgabe:

> Finde am 1. Oktober 2026 einen 30-minütigen Termin zwischen 09:00 und 12:00 Uhr.
> Von 09:00 bis 10:00 Uhr liegt bereits ein Termin. Eine Wunschzeit ist nicht angegeben.

Alle Uhrzeiten sind UTC. Der Satz erklärt das Beispiel; der aktuelle Planer liest keine
freie Texteingabe. Eine aufrufende Anwendung muss die Angaben als CPIR bereitstellen.
Die Beispieldatei enthält synthetische Daten, keine Termine aus einem angebundenen Kalender.

| Frage | Feld in der Anfrage | Wert im Beispiel |
| --- | --- | --- |
| Was soll entstehen? | [`operation`](../../crates/cerebri-planner/src/model.rs) | `CREATE`: einen neuen Termin vorschlagen |
| Für welches Objekt? | [`target_ids`](../../crates/cerebri-planner/src/model.rs) | `new-event`, beschrieben in `context.objects` |
| Wie lange dauert es? | [`duration`](../../crates/cerebri-planner/src/model.rs) | 1800 Sekunden, also 30 Minuten |
| In welchem Zeitraum wird gesucht? | [`scope.time_range`](../../crates/cerebri-planner/src/model.rs) | 09:00–12:00 Uhr |
| Was belegt schon Zeit? | [`context.objects`](../../crates/cerebri-planner/src/model.rs) | Das bestehende Objekt `busy`, 09:00–10:00 Uhr |
| Gibt es zusätzliche verbindliche Regeln? | [`constraints`](../../crates/cerebri-planner/src/model.rs) | `[]`: keine zusätzlichen Regeln angegeben |
| Gibt es Wünsche? | [`preferences.preferences`](../../crates/cerebri-preferences/src/lib.rs) | `[]`: keine Präferenzen angegeben |

`new-event` hat `revision: null`: Es ist ein geplantes neues Objekt. `busy` hat bereits
`revision: 1` und eine bekannte Zeit. Die separate `context.revision` kennzeichnet den
Stand des gesamten übergebenen Kontexts. Die Suche erzeugt einen Vorschlag für
`new-event`; sie legt dadurch noch keinen Kalendereintrag an.

## Warum die neue Zeit fehlen darf, die Dauer aber nicht

Die Zeit für `new-event` muss erst gefunden werden. Deshalb enthält dessen `time.value`
in der Beispieldatei genau diesen Wert:

```json
{
  "processing": "RESOLVED",
  "knowledge": {
    "state": "MISSING"
  }
}
```

Das ist ein Ausschnitt, keine vollständige Planungsanfrage. `RESOLVED` bedeutet, dass
der Verarbeitungszustand geklärt ist. `MISSING` sagt, dass keine Zeit angegeben wurde.
Diese Kombination ist für den neuen Termin vorgesehen. Eine bereits belegte Zeit
dürfte hingegen nicht fehlen: Der Planer braucht sie, um Überschneidungen zu prüfen.

Auch die Dauer muss vor der Suche feststehen. So sieht das vollständige Feld `duration`
in derselben Datei aus:

```json
{
  "value": {
    "processing": "RESOLVED",
    "knowledge": {
      "state": "KNOWN",
      "data": 1800
    }
  },
  "provenance": "USER_EXPLICIT",
  "confidence": null,
  "evidence": []
}
```

`KNOWN` kennzeichnet den bekannten Wert von 1800 Sekunden. `USER_EXPLICIT` hält fest,
dass die Dauer ausdrücklich vorgegeben wurde. Das leere `evidence`-Array enthält keine
zusätzlichen Belegkennungen. `confidence: null` bedeutet, dass kein separater
Konfidenzwert angegeben ist; es macht die bekannte Dauer nicht zu null oder unbekannt.

Andere Zustände bleiben unterscheidbar: `UNKNOWN` heißt, dass sich ein benötigter Wert
derzeit nicht bestimmen lässt. `AMBIGUOUS` hält mehrere mögliche Deutungen fest.
`UNRESOLVED` bedeutet, dass die Verarbeitung noch offen ist. Wenn die Dauer fehlt oder
in einem dieser Zustände steht, beginnt die Suche nicht. Ein geschätzter Wert mit
`UNCERTAIN` darf nur für eine Analyse mit ausdrücklicher Policy-Erlaubnis verwendet werden.

## Eine leere Regelliste schaltet Prüfungen nicht ab

Auch bei `constraints: []` prüft der Planer die Dauer, das Planungsfenster und
Überschneidungen mit bekannten Belegungen. Die Zeit von `busy` steht direkt im Objekt
und wird deshalb auch berücksichtigt, wenn `context.facts` leer ist.

Die Herkunft dieser Zeit ist im Beispiel als `INTEGRATION_FACT` angegeben. Hier ist das
eine Angabe in synthetischen Testdaten, kein Nachweis einer produktiven Kalenderanbindung.

Wünsche stehen separat in `preferences`. In diesem Beispiel ist die Liste leer; es wird
keine Wunschzeit ergänzt oder gelernt. Auch mit einem Wunsch müsste ein Kandidat zuerst
die verbindlichen Prüfungen bestehen. Eine bevorzugte Uhrzeit kann einen Konflikt nicht
aufheben.

## Suchschritte und Suchgrenze sind verschiedene Angaben

`granularity: 900` legt mögliche Startzeiten im Abstand von 900 Sekunden, also 15 Minuten,
fest. Zusammen mit der Dauer von 30 Minuten und dem Fenster 09:00–12:00 Uhr ergeben sich
11 mögliche Starts: 09:00, 09:15 und so weiter bis 11:30 Uhr.

`budget.max_candidates: 256` begrenzt dagegen, wie viele Startzeiten geprüft werden
dürfen. Es verlangt keine 256 Kandidaten. Das Budget reicht hier für alle 11 Starts,
einschließlich der später verworfenen. `max_repairs` und `max_depth` stehen auf null;
eine weitergehende Reparatursuche findet nicht statt. Das Budget zählt Arbeitsschritte,
keine verstrichenen Millisekunden.

## Ein Suchraum ist noch keine Berechtigung

`scope` begrenzt, wo geplant werden darf. Seine Filter stehen hier auf `null`, fügen also
keine Einschränkung nach Kennungen hinzu. Eine leere Liste `[]` würde nichts auswählen.
Keine dieser Angaben erteilt eine Berechtigung.

`policy` beschreibt die geltenden Regeln für Änderungen; `planning_capability` enthält
die Planungsrechte für konkrete Objekte. Die Beispiel-Policy erlaubt grundsätzlich
`CREATE_EVENT` und `MOVE_EVENT`. Die Capability enthält aber nur einen passenden Eintrag
für `CREATE_EVENT` an `new-event`. Daraus entsteht kein Recht, `busy` zu verschieben.
Die Änderungsgrenze von drei im Scope und in der Policy ist eine Obergrenze, kein Auftrag
für drei Änderungen. Die Suche behandelt hier weiterhin genau einen Zieltermin.

Diese Angaben werden schon bei der Planung geprüft. Für eine Ausführung sind danach
weitere Validierungs- und Berechtigungsprüfungen erforderlich. Insbesondere ersetzt ein
`principal_id` in einer Anfrage keine Anmeldung. Die [Lifecycle-Referenz](foundation.md)
beschreibt die Schritte bis zur Ausführung.

## Von den Eingaben zum Ergebnis

Für diese unveränderte Datei verwirft der Rust-Planer vier überlappende Starts; sieben
bleiben übrig. Ohne Wunschzeit steht nach den festen Vergleichsregeln 10:00 Uhr an erster
Stelle. Die [Einführung](introduction.md) führt durch dieses Ergebnis, die
[Planungsreferenz](planner-integration.md) beschreibt den genauen Vergleich.

Als Nächstes zeigt die [Architektur-Erklärung](foundation.md), welcher Teil diese
Eingaben prüft, Vorschläge berechnet und Ergebnisse an Anwendungen zurückgibt.

Die folgenden Quellen verbinden die Erklärung mit der Implementierung:

- [Vollständige CPIR-Anfrage](../../examples/request.json): die ausführbare Eingabe.
- [Rust-Datenmodell](../../crates/cerebri-planner/src/model.rs): Felder und Scope-Filter.
- [Wissenszustände](../../crates/cerebri-types/src/lib.rs): `FieldState`, `Knowledge` und Herkunft.
- [Eingabeprüfung](../../crates/cerebri-planner/src/validation.rs): benötigte Werte und Planungsrechte.
- [Kandidatenprüfung](../../crates/cerebri-planner/src/lifecycle.rs): unter anderem die feste Überschneidungsprüfung.
- [Suche](../../crates/cerebri-planner/src/search.rs): Raster, Budget und Reihenfolge.

## Schema und weitere Eingaberegeln

**Current CPIR:** `0.2`
**Legacy CPIR:** `0.1`

Softwareversion 0.2.0. Spezifikation 0.4, CPIR 0.2 und REST v1 sind unabhängige
Versionsbereiche. CPIR ist ein internes, veränderliches Schema ohne öffentliches v1-Stabilitätsversprechen.

[Die aktuelle CPIR-0.2-Anfrage](../../examples/request.json) ist der normale Quick Start.
[Die CPIR-0.1-Legacy-Anfrage](../../examples/legacy-cpir-0.1.json) prüft Kompatibilität ohne temporale Eingaben.
Die Current-/Legacy-Angaben setzen [ADR-0012](../architecture/decisions/ADR-0012-planner-snapshot-compilation.md) um;
die Repository-Prüfung gleicht sie mit Rust-Schemakonstanten, beiden Beispielen und der README ab.
Die Rust-Strukturen in cerebri-planner/model.rs definieren die Repräsentation.
Zentrale Objekte lehnen unbekannte Felder ab, einschließlich vertippter Scope-Felder.
Akzeptiert werden {major: 0, minor: 1} und {major: 0, minor: 2}. Temporale Eingaben verlangen 0.2; andere Versionen ergeben UnsupportedSchema.
Die [Planner-Integration](planner-integration.md) beschreibt Compiler, Graph und Sortierschlüssel.

Eine Anfrage enthält Identität/Trace/Principal, Operation, Scope, unveränderlichen Kontext mit
Objekten, Fakten und optionalen temporalen Quelldaten, Ziele, Dauerevidenz, Constraints, Präferenzen, typisierte Policy,
Planungsrechte, Raster und deterministisches Budget. Event, Task, Deadline, Availability und
Resource sind modelliert. Such- und Mutationsziele sind derzeit Events. Bestehende blockierende
Zeiten müssen bekannt sein; neue Events haben den Zeitstatus Missing. Eine verbindlich
vorgegebene Platzierung wird als ExplicitTime-Constraint angegeben, eine bloß bevorzugte
Startzeit gehört zu den Präferenzen.

FieldState trennt den Verarbeitungszustand Unresolved von Knowledge: Known, Missing, Unknown,
Uncertain und Ambiguous. Confidence ist endlich und liegt in [0,1]; sie ersetzt keinen Wissensstatus.
Fehlende, unbekannte, mehrdeutige oder unaufgelöste Pflichtfelder blockieren die Suche.
Unsichere Dauer darf nur bei Analyse und ausdrücklicher Policy-Erlaubnis verwendet werden;
die Erklärung hält sie fest. Bestehende Eventzeiten benötigen Faktenprovenienz.

TimeRange verwendet RFC-3339-UTC-Zeitpunkte und prüft beim Deserialisieren start < end.
Intervalle sind halboffen [start,end); angrenzende Events überlappen nicht.
Die IANA-Ursprungszeitzone bleibt am Objekt/ZonedDateTime erhalten. Lokale Umrechnung weist
DST-Lücken und doppelte Zeiten zurück. Dauer ist eine positive ganze Sekundenzahl.
Begrenzte Wiederholungsdiagnosen sind separat verfügbar; CPIR-Wiederholungsconstraints werden
weiterhin sicher abgelehnt. Siehe [Temporal Core](temporal.md).

Optionale Scope-Listen: weggelassen/null bedeutet keinen Zusatzfilter und keine Berechtigung;
[] wählt nichts; [A,B] begrenzt auf diese IDs. Ressourcenfilter verlangen eine nichtleere
Objektressourcenmenge vollständig innerhalb des Filters. Movable-IDs betreffen bestehende
Objekte. max_mutations=0 verhindert ActionPlan-Übersetzung. FindSlot und Analyze sind ebenfalls
immer nicht ausführbar.

Create, Move, FindSlot, Plan und Analyze unterstützen den Basisplaner.
Update, Cancel, Reschedule und Optimize sind vorbereitet und liefern UnsupportedOperation.
Es gibt keinen stillen Ersatz. Die Suche umfasst ein Ziel; explizite Batch-Vorschläge
durchlaufen denselben Validator.

## Transporte

- GET /health: Software-, Schema- und Statusmetadaten.
- POST /v1/validate: typisierter ValidationReport.
- POST /v1/plan: PlanningResult mit Kandidaten, Bewertung, Konflikten und Suchabdeckung.
- POST /v1/temporal: typisierter Wiederholungs-/Free-Busy-Bericht.
- GET /lab: Weiterleitung zum gebauten Svelte Lab unter /lab/.
- Kein Ausführungsendpunkt.

Fehlerhafte Transporteingaben ergeben HTTP 400/422, übergroße Anfragen 413.
Fachliche Ablehnung ist ein typisierter HTTP-200-Befund. Body-Limit: 256 KiB.
Core-Grenzen: 256 Objekte, jeweils 1024 Fakten/Constraints, 4096 Rasterpositionen und
eine konservative kombinierte Arbeitsabschätzung von höchstens 1.000.000 Einheiten.
Reparatur- und Tiefensuche sind auf null begrenzt. Es gibt weder Wall-Clock-Timeouts noch versteckte Uhrzugriffe.

Die API ist ein lokaler Entwicklungsprozess ohne Authentifizierung. Vor externer Bereitstellung
muss eine spätere Anwendung Principals authentifizieren, vertrauenswürdigen Kontext/Sichtbarkeit
auflösen, Rechte serverseitig bestimmen und Autorisierung/Bestätigung verwalten.
Client-Policy und -Capabilities sind hier Planungseingaben, keine Ausführungsberechtigungen.

Node: cerebri-node bauen, plan aus bindings/node/index.mjs importieren und await plan(request)
aufrufen. Die vorläufige Prozessbrücke startet Rust und enthält keine JavaScript-Planung.
Native N-API-/Electron-Verteilung folgt später; ein Binary-Override ermöglicht Packaging.
Limits: 256 KiB Eingabe, 16 MiB Ausgabe. Binärdateien werden nicht eingecheckt.

[Architektur](foundation.md) · [English](../en/cpir.md)
