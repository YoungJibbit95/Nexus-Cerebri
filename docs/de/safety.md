<!-- doc: safety; lang: de; counterpart: ../en/safety.md -->
# Vom Vorschlag zur erlaubten Änderung

Eine passende Zeit zu finden beantwortet eine Planungsfrage. Einen Kalender zu ändern
beantwortet eine andere Frage: Darf genau diese Änderung jetzt ausgeführt werden?
Cerebri hält diese Schritte auseinander. Das Beispiel auf der Website endet mit einem
Vorschlag; es bucht keinen Termin.

## Warum ein guter Vorschlag nicht genügt

Angenommen, der Planer schlägt 10:00 Uhr vor. Bevor etwas geändert wird, könnte dort ein
anderer Termin hinzukommen, eine Berechtigung entzogen werden oder eine erforderliche
Bestätigung fehlen. Der frühere Vorschlag beantwortet keine dieser Fragen von allein.

Die aktuelle Rust-Implementierung bildet diese Schritte durch unterschiedliche Typen ab.
Ein Vorschlag lässt sich nicht direkt an den Executor übergeben:

1. **Vorschlagen — `ProposedPlan`.** Eine mögliche Platzierung beschreiben. Die Prüfung
   einzelner Kandidaten während der Suche macht daraus noch keine Erlaubnis zum Handeln.
2. **Prüfen — `ValidatedPlan`.** Das gesamte geplante Ergebnis mit der übergebenen aktuellen
   Ausgangslage, dem erlaubten Umfang und den verbindlichen Regeln abgleichen. Eine
   geänderte Quellrevision verlangt eine erneute Prüfung.
3. **Änderungen beschreiben — `ActionPlan`.** Den geprüften Plan in konkrete Änderungen
   übersetzen. Eine reine Analyseanfrage darf diesen Schritt nicht nehmen.
4. **Freigeben — `AuthorizedActionPlan`.** Die geltenden Richtlinien, Befugnisse,
   erforderliche Bestätigung und Ausführungsrechte für genau diesen Plan prüfen.
5. **Ausführen und festhalten — `ExecutionResult`.** Der Executor nimmt nur einen
   freigegebenen Aktionsplan an. Unmittelbar vor externen Änderungen prüft er Aktualität,
   geltende Rechte, Bestätigung und Schutz vor wiederholter Ausführung erneut. Das
   Ergebnis hält fest, was erfolgreich war, scheiterte oder übersprungen wurde.

Ein Ergebnis aus JSON einzulesen überspringt diese Schritte nicht. Ein gespeicherter
Vorschlag lässt sich nicht als bereits geprüfter oder freigegebener Plan importieren.

## Eine Prüfung kann veralten

Ändern sich die Ausgangsdaten nach der Prüfung, ist das frühere Ergebnis veraltet.
Vor der Ausführung muss Cerebri erneut prüfen und freigeben. Der Executor prüft außerdem,
ob die Objektversion beim Anbieter noch übereinstimmt, bevor er dieses Objekt verändert.

Der Schutz vor wiederholter Ausführung verhindert, dass eine erneut gesendete Anfrage
dieselbe Änderung einfach ein zweites Mal anwendet. Geht die Antwort eines Anbieters
verloren, kann der Ausgang unbekannt sein. Dann muss zuerst geklärt werden, was tatsächlich
passiert ist. Ein automatischer Wiederholungsversuch könnte die Änderung doppelt ausführen.

## Was heute vorhanden ist

Die Ablauf-Typen, Prüfungen und Ausführungsschnittstellen sind in Rust vorhanden.
Der enthaltene Anbieteradapter und das Ausführungsprotokoll sind **Testimplementierungen
im Arbeitsspeicher**. Sie verbinden sich mit keinem echten Kalender und bewahren keine
Ausführungsdaten über einen Programmneustart hinweg auf.

Authentifizierte Kalenderanbieter, dauerhafte Ausführungsprotokolle und Funktionen zur
Wiederherstellung bleiben spätere Arbeit. REST und die Node-Brücke bieten keine Operation
zur Ausführung an. Im Lab lassen sich Planung und Diagnosen untersuchen; es bietet keine
Bedienelemente zum Ändern eines Kalenders beim Anbieter.

Das sind konkrete Schutzmaßnahmen und Grenzen der Umsetzung, keine Aussage, dass alle
Sicherheitsrisiken gelöst wären. Ein späterer erlernter Vorschlag müsste dieselben
Prüfungen durchlaufen.

## Die technischen Grundlagen

- [Ablauf-Typen und Validierung](../architecture/decisions/ADR-0002-lifecycle.md)
- [Prüfungen vor der Ausführung und Fehlerbehandlung](../architecture/decisions/ADR-0005-execution.md)
- [Aktuelle Master-Spezifikation](../architecture/specifications/master-v0.4.md)
- [Ablauf-Implementierung in Rust](../../crates/cerebri-planner/src/lifecycle.rs)
- [Ausführungs-Implementierung in Rust](../../crates/cerebri-integrations/src/executor.rs)

[Planung verstehen](planner-integration.md) · [Architektur](foundation.md) ·
[Das Lab lokal ausprobieren](development.md) · [English](../en/safety.md)
