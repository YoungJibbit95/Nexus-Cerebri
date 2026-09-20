<!-- doc: cpir; lang: de; counterpart: ../en/cpir.md -->
# CPIR 0.1 und Schnittstellen

Softwareversion 0.2.0. Spezifikation 0.4, CPIR 0.1 und REST v1 sind unabhängige
Versionsbereiche. CPIR ist ein internes, veränderliches Schema ohne öffentliches v1-Stabilitätsversprechen.

[Die ausführbare synthetische Anfrage](../../examples/request.json) zeigt das vollständige Format.
Die Rust-Strukturen in cerebri-planner/model.rs definieren die Repräsentation.
Zentrale Objekte lehnen unbekannte Felder ab, einschließlich vertippter Scope-Felder.
Andere Schemaversionen als {major: 0, minor: 1} ergeben UnsupportedSchema.

Eine Anfrage enthält Identität/Trace/Principal, Operation, Scope, unveränderlichen Kontext mit
Objekten und Fakten, Ziele, Dauerevidenz, Constraints, Präferenzen, typisierte Policy,
Planungsrechte, Raster und deterministisches Budget. Event, Task, Deadline, Availability und
Resource sind modelliert. Such- und Mutationsziele sind derzeit Events. Bestehende blockierende
Zeiten müssen bekannt sein; neue Events haben den Zeitstatus Missing. Explizite Wunschzeiten
werden als ExplicitTime-Constraint angegeben.

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
