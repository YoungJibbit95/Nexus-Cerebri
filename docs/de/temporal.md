<!-- doc: temporal; lang: de; counterpart: ../en/temporal.md -->
# Wann ist eine Zeit wirklich frei?

Eine Lücke im Kalender bedeutet nicht immer, dass dort nichts geplant ist. Vielleicht
fehlen in den übergebenen Daten einfach Termine. Cerebri unterscheidet deshalb belegte,
freie und unbekannte Zeit. So wird deutlich, worauf eine Aussage zur Verfügbarkeit beruht.

## Ein Termin darf beginnen, wenn der andere endet

Endet ein Termin um 10:00 Uhr und beginnt der nächste um 10:00 Uhr, überschneiden sie sich
nicht. Bei Cerebri gehört der Anfang zu einem Zeitraum, sein Endzeitpunkt aber nicht mehr.
Die kurze Schreibweise lautet `[start, end)`. Beginnt der zweite Termin bereits um
09:59:59 Uhr, entsteht dagegen eine Überschneidung von einer Sekunde.

Vorbereitung und Anreise können zusätzliche Zeit brauchen. Sind vor einem Termin um
10:00 Uhr zehn Minuten Vorbereitung angegeben, ist diese Zeit ebenfalls belegt.
Die Berechnung nutzt die übergebenen Puffer; sie schätzt keine Reisezeit selbst.

## Fehlende Termine lassen die Verfügbarkeit offen

Angenommen, die Daten enthalten im Suchfenster 09:00–12:00 Uhr einen Termin von 09:00 bis
10:00 Uhr, ohne zusätzliche Puffer. Erklärt die aufrufende Anwendung die Sammlung für
dieses Fenster als vollständig, kann 10:00–12:00 Uhr als frei gelten. Ist die Sammlung
unvollständig, bleibt derselbe Zeitraum unbekannt.

Die Angabe zur Vollständigkeit kommt von der aufrufenden Anwendung. Cerebri verbindet sich
nicht mit einem Kalenderanbieter, um sie zu überprüfen. Bei der Planung mit zeitlichen
Quelldaten verhindert unvollständige Abdeckung die Aussage, ein Kandidat passe in freie Zeit.

## Eine lokale Uhrzeit braucht eine Zeitzone

UTC bezeichnet den Zeitpunkt, mit dem gerechnet wird. Eine Zeitzone wie `Europe/Berlin`
erklärt, welche lokale Uhrzeit dazu gehört. Bei einem wiederkehrenden Termin sind lokale
Uhrzeit und Zeitzone wichtig: Ein Termin um 09:00 Uhr kann nach der Zeitumstellung zu
einer anderen UTC-Zeit stattfinden.

Bei einer Zeitumstellung gibt es manche lokalen Uhrzeiten gar nicht oder zweimal.
Für unterstützte tägliche und wöchentliche Serien muss die aufrufende Anwendung festlegen,
was dann gilt: eine fehlende Uhrzeit ablehnen oder überspringen; eine doppelte Uhrzeit
ablehnen oder ihr früheres beziehungsweise späteres Vorkommen wählen. Das Ergebnis hält
diese Entscheidungen fest. Die gewöhnliche Umrechnung einer lokalen Uhrzeit lehnt beide
unklaren Fälle ab.

## Was die aktuelle Umsetzung unterstützt

Die Zeitdiagnose berechnet tägliche und wöchentliche Wiederholungen für einen begrenzten
Zeitraum, berücksichtigt angegebene Puffer und meldet belegte, freie und unbekannte Zeit.
Der Planer kann außerdem aus bestehenden, als Fakten gelieferten Serien in CPIR 0.2
belegte Zeiten ableiten.

Monats- und Jahresregeln, Ausnahmen und Feiertagskalender sind nicht implementiert.
Auch das Vorschlagen oder Ändern einer Serie gehört noch nicht zum Planer. Eine harte
`RecurrenceRule`-Bedingung bleibt ununterstützt; sie ist etwas anderes als eine bestehende
Serie in den Ausgangsdaten.

[Planung verstehen](planner-integration.md) · [Eingabeformat](cpir.md) ·
[Ergebnisse im Lab untersuchen](development.md)

## Technisch
POST /v1/temporal akzeptiert [das synthetische Beispiel](../../examples/temporal-request.json).
Pflichtfelder sind horizon, busy, coverage, recurrences und limits.
Busy-Einträge enthalten range und before_seconds/after_seconds/travel_seconds.
Wiederholungen enthalten start_date, local_time, timezone (IANA), duration (verstrichene
Sekunden), pattern, optionale until/count sowie erforderliche gap_policy/fold_policy.
DAILY benötigt ein positives every. WEEKLY zusätzlich eindeutige Wochentage (Mon..Sun).
until ist inklusiv; count zählt nominelle Termine einschließlich übersprungener DST-Lücken.
Wochen beginnen montags; vor start_date liegende Wochentage der ersten Woche entfallen.
Alle Intervalle gelten als [start,end). Berührung mit der Horizontgrenze ist keine Überlappung.
Der Bericht bewahrt das volle range und das beschnittene visible_range getrennt auf.

Antwort: {status: Complete, data: {availability, expansions}} oder
{status: Rejected, data: typisierter Fehler}. Ablehnung enthält keine Teil-Verfügbarkeit.
availability liefert busy/free/unknown und Puffer-/Clipping-Traces. expansions liefert
nominelle Sequenz/Datum, Zeitauflösung, ausgelassene Tage und examined_dates.
Termin- und Datumsbudgets gelten gemeinsam für alle Regeln.
Grenzen: 32 Regeln, 10.000 Termine, 36.600 geprüfte Daten, 10.000 kombinierte Busy-Einträge.
Auch der Zweitagesrand für Zeitzonenwechsel zählt zum Datumsbudget.
Fehlerhafte JSON-Daten/Typen ergeben HTTP 400/422, über 256 KiB HTTP 413.
Fachliche Ablehnung ist ein typisierter HTTP-200-Bericht, kein Transportfehler.

## Forschungstiefe und Grenzen
[ADR-0009](../architecture/decisions/ADR-0009-bounded-temporal-diagnostics.md) beschreibt
Grammatik und Vollständigkeit. Tests vergleichen Wochenordinalzahlen mit einem unabhängigen
tageweisen Referenzverfahren, prüfen Intervallpartitionen exhaustiv und Berlin, Lord Howe, Samoa.
Ein deterministischer Korpus fehlerhafter Bytes ist ein reproduzierbarer Smoke-Test, kein
dauerhaftes Coverage-Fuzzing. IANA-Daten stammen aus den durch Cargo.lock fixierten
Abhängigkeiten. Updates können Zonenresultate verändern und müssen die Golden-Tests bestehen.

Kein vollständiger RFC-5545-Parser: Monats-/Jahresregeln, Ausnahmen und Feiertage fehlen.
Die harte CPIR-RecurrenceRule wird weiterhin sicher abgelehnt. Bestehende, als Fakten
gelieferte Serien in `context.temporal` können inzwischen über den ausdrücklichen
Snapshot-Compiler belegte Zeiten beitragen;
siehe [ADR-0012](../architecture/decisions/ADR-0012-planner-snapshot-compilation.md).
Externe Verfügbarkeit wird weder geladen noch geraten. Coverage ist eine explizite Angabe
für Diagnosen, keine Provider-Garantie oder Berechtigung. Core bleibt ohne Uhr, Netzwerk und Dateizugriff.
[Entwicklung](development.md) · [Lab](../../apps/cerebri-lab/README.md) · [English](../en/temporal.md)
