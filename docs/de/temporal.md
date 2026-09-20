<!-- doc: temporal; lang: de; counterpart: ../en/temporal.md -->
# Temporal Core
## Einfach
Der Core wandelt explizite lokale Kalenderregeln in UTC-Intervalle um. Eine Zeitlücke im
Frühjahr wird nur auf Wunsch übersprungen; eine doppelte Herbstzeit braucht eine klare
Auswahl. Belegte Zeit enthält Vorbereitungs- und Reisepuffer. Nur eine vollständige
Datensammlung belegt, dass die übrige Zeit frei ist; sonst bleibt sie unbekannt.

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
Harte CPIR-RecurrenceRule wird weiterhin sicher abgelehnt; der Planner expandiert keine Serien.
Externe Verfügbarkeit wird weder geladen noch geraten. Coverage ist eine explizite Angabe
für Diagnosen, keine Provider-Garantie oder Berechtigung. Core bleibt ohne Uhr, Netzwerk und Dateizugriff.
[Entwicklung](development.md) · [Lab](../../apps/cerebri-lab/README.md)
