<!-- doc: introduction; lang: de; counterpart: ../en/introduction.md -->
# Wie Nexus Cerebri einen Termin plant

[English](../en/introduction.md)

## Worum es bei Cerebri geht

Nexus Cerebri ist ein Open-Source-Lernprojekt rund um die Frage, wie Software Pläne
erstellt und ihre Ergebnisse nachvollziehbar macht. Das aktuelle Beispiel ist ein
Termin: Du gibst die Dauer, das Zeitfenster und die zu beachtenden Regeln vor. Cerebri
prüft mögliche Zeiten, schließt Konflikte aus und gibt einen Vorschlag zurück.
Du kannst nachvollziehen, warum eine Zeit ausscheidet und eine andere an erster Stelle steht.

Der aktuelle Planer arbeitet nach festen Regeln: Gleiche Eingaben liefern das gleiche
Ergebnis. Das bedeutet hier **deterministisch**. Er plant derzeit jeweils einen Zieltermin.
Eine passende Zeit zu finden, trägt noch keinen Termin in einen Kalender ein.

## Warum ich Cerebri begonnen habe

Am Anfang wollte ich verstehen, wie neuronale Netze lernen. Ein wichtiger Anstoß war
die Mathematik hinter Arbeiten wie
[Learning representations by back-propagating errors](https://www.nature.com/articles/323533a0)
von David E. Rumelhart, Geoffrey E. Hinton und Ronald J. Williams aus dem Jahr 1986.
Die Formeln wirkten auf mich zunächst wie eine schwer zugängliche Sprache.

Das änderte sich, als ich die einzelnen Rechenschritte nachvollzog. Was als ganze Formel
kaum verständlich wirkte, ließ sich in überschaubare Operationen zerlegen. Die Zeichen
beschrieben einen Ablauf, den ich Schritt für Schritt verfolgen konnte. Genau dieses
Verständnis wollte ich weiterentwickeln.

So entstand Cerebri als Lernprojekt. Ich wollte Ideen selbst programmieren, Annahmen
testen und sehen, wie die Teile in einem wirklichen System zusammenspielen. Dabei
lässt sich prüfen, ob eine Erklärung auch zu dem passt, was der Code tatsächlich tut.
Die Beispiele und Erklärungen auf dieser Website gehören zu diesem Lernweg.

## Warum gerade Planung?

Schon bei einem einzelnen Termin hängen mehrere Fragen zusammen: Welche Zeiten sind
belegt? Was wissen wir noch nicht? Welche Regeln müssen eingehalten werden? Welche
Uhrzeit wäre nur ein Wunsch? Warum kommt eine Möglichkeit infrage und eine andere nicht?
Planung gibt diesen Fragen einen konkreten Zusammenhang und macht Antworten überprüfbar.

Langfristig soll daraus eine Planungssoftware entstehen, die sich in unterschiedlichen
Anwendungen einsetzen lässt, etwa für Aufgaben, Räume oder Schichten. Das ist die
Richtung des Projekts. Der aktuelle Planer bearbeitet jeweils einen Zieltermin; die
Terminplanung dient als erstes praktisches Lernfeld.

## Warum der Planer zuerst nach festen Regeln arbeitet

Bevor ein lernendes Verfahren Vorschläge beeinflussen kann, muss klar sein, welche
Informationen vorliegen, welche Regeln gelten und was überhaupt geändert werden darf.
Der heutige Planer macht diese Grundlagen ausdrücklich prüfbar. Gleiche Eingaben
liefern dasselbe Ergebnis, und ein Vorschlag allein erlaubt noch keine Ausführung.

Später möchte ich untersuchen, wie neuronale Netze bei der Suche oder beim Vergleich
zulässiger Möglichkeiten helfen können. Auch dann müssten feste Prüfungen sicherstellen,
dass Fakten, verbindliche Regeln und Berechtigungen eingehalten werden. Ein gelerntes
Modell dürfte sie nicht übergehen. Neuronale Netze und gelernte Vorlieben sind Teil
dieser Forschungsrichtung und werden im hier gezeigten Planer noch nicht eingesetzt.
Die [Entwicklungs- und Lernroadmap](../development/roadmap/development-learning-roadmap.md)
beschreibt die geplanten Schritte.

## Was heute vorhanden ist

Das Repository enthält einen Rust-Planer, Prüfungen für Zeitangaben und ausführbare
Beispiele, deren Ergebnisse sich untersuchen lassen. Die Software hat die Version
**0.2.0**. Sie ist noch unveröffentlicht und wird für eine Veröffentlichung geprüft.
Ein veröffentlichtes Release gibt es bisher nicht.

Die Versionsnummern beziehen sich auf unterschiedliche Dinge:

| Bestandteil | Version | Bedeutung |
| --- | --- | --- |
| [Software](../../Cargo.toml) | 0.2.0 | Der aktuelle Entwicklungsstand des Codes |
| Spezifikation | 0.4 | Das Dokument, das die Architektur des Systems festlegt |
| CPIR | 0.2 | Das strukturierte Datenformat des aktuellen Planungsbeispiels |
| Rust | 1.97 | Die im Workspace festgelegte Mindestversion von Rust |

Den aktuellen Veröffentlichungsstatus beschreibt das [Changelog](../../CHANGELOG.md).

## Mit dem gewünschten Termin beginnen

Angenommen, du brauchst einen 30-minütigen Termin zwischen 09:00 und 12:00 Uhr. Von
09:00 bis 10:00 Uhr liegt bereits ein Termin. Der neue Termin muss vollständig in das
gewünschte Zeitfenster passen und darf den bestehenden Termin nicht überschneiden.
Alle Uhrzeiten in diesem Beispiel sind UTC.

Dieser Satz erklärt die Aufgabe für Menschen. Der aktuelle Planer erhält dieselbe
Aufgabe als strukturierte Daten, nicht als Satz. Sein Eingabeformat heißt **CPIR**.
Jede Information steht in einem eigenen Feld, damit der Planer die Angaben prüfen kann.

| Angabe | In diesem Beispiel | Wozu sie gebraucht wird |
| --- | --- | --- |
| [Dauer](../../examples/request.json) | 30 Minuten | Der gesamte Termin muss in das Zeitfenster passen |
| Planungsfenster | 09:00–12:00 Uhr | Die Suche bleibt innerhalb dieses Zeitraums |
| Bekannter Termin | 09:00–10:00 Uhr | Diese Zeit ist belegt |
| Verbindliche Regel | Keine Überschneidung | Eine widersprüchliche Möglichkeit scheidet aus |
| Bevorzugter Beginn | Nicht angegeben | Keine Wunschzeit wird bevorzugt |
| Suchschritt | 15 Minuten | Mögliche Startzeiten werden im Viertelstundentakt geprüft |

Das Format hält außerdem fest, woher Angaben stammen und welche Planungsberechtigungen
gelten. Fehlende Informationen bleiben als fehlend erkennbar. Insbesondere gelten
unvollständige Angaben zur Verfügbarkeit nicht als Nachweis freier Zeit. Ein
Planungsfenster begrenzt die Suche; es erlaubt keine Änderungen an einem Kalender.

Die genaue Eingabe steht in [examples/request.json](../../examples/request.json).
Die [CPIR-Erklärung](cpir.md) zeigt Schritt für Schritt, in welchen Feldern diese Angaben
stehen und warum eine noch fehlende Startzeit etwas anderes ist als eine fehlende Dauer.

## Die möglichen Startzeiten prüfen

Der Planer prüft 11 Startzeiten von 09:00 bis 11:30 Uhr in Schritten von 15 Minuten.
Die letzte Startzeit ist 11:30 Uhr, weil der 30-minütige Termin bis 12:00 Uhr beendet
sein muss. Jede mögliche Platzierung nennt man einen **Kandidaten**.

Vier Kandidaten scheiden aus: 09:00, 09:15, 09:30 und 09:45 Uhr überschneiden sich mit
dem bestehenden Termin. Sieben bleiben übrig: 10:00, 10:15, 10:30, 10:45, 11:00,
11:15 und 11:30 Uhr.

Die Schritte sind:

1. Mögliche Startzeiten innerhalb des gewünschten Fensters erzeugen.
2. Kandidaten ausschließen, die bekannten Terminen oder verbindlichen Regeln widersprechen.
3. Die übrigen Kandidaten anhand von Wünschen und festen Vergleichsregeln sortieren.
4. Den ersten Kandidaten als Vorschlag zusammen mit dem Suchergebnis zurückgeben.

Die Website zeigt das Ergebnis, das der Rust-Planer für dieses Beispiel berechnet hat.
Sie berechnet im Browser keine eigene Rangfolge.

## Warum 10:00 Uhr an erster Stelle steht

Verglichen werden nacheinander: Abstand zur Wunschzeit, Anzahl der Änderungen,
zeitliche Verschiebung, Startzeit und Objektkennung. Das erste unterschiedliche
Merkmal entscheidet über die Reihenfolge. Die Werte werden nicht zu einer Gesamtpunktzahl
addiert.

In diesem Beispiel wurde keine Wunschzeit angegeben. Sowohl 10:00 als auch 10:15 Uhr
würden einen neuen Termin anlegen und keinen vorhandenen Termin verschieben. Die ersten
drei Vergleichswerte sind deshalb gleich. Als Nächstes zählt die Startzeit: 10:00 Uhr
steht vor 10:15 Uhr.

Das Ergebnis heißt **ProvenOptimal**, weil alle Startzeiten dieses 15-Minuten-Rasters
geprüft wurden und der Vorschlag nach den festgelegten Vergleichsregeln an erster Stelle
steht. Über Zeiten zwischen den Rasterpunkten oder andere Planungsregeln sagt das nichts aus.

Die [Planungsreferenz](planner-integration.md) beschreibt die genauen Vergleichsfelder.

## Ein Vorschlag braucht noch Prüfungen und Berechtigungen

Das Beispiel endet mit einem Vorschlag für 10:00–10:30 Uhr. Es wurde nichts gebucht.

Bevor eine Anwendung eine Änderung ausführen könnte, müsste sie den vollständigen Plan
validieren, in konkrete Aktionen übersetzen und autorisieren lassen. Unmittelbar vor
der Ausführung wird geprüft, ob die Angaben und Berechtigungen noch aktuell sind.
Das Ausführungsergebnis hält anschließend fest, was tatsächlich passiert ist.

| Schritt | Bedeutung |
| --- | --- |
| Vorgeschlagen | Der Planer hat einen Plan vorgeschlagen |
| Validiert | Der Plan wurde für einen bestimmten Stand der Eingabedaten erfolgreich geprüft |
| Aktionsplan | Die beabsichtigten Änderungen sind ausdrücklich beschrieben |
| Autorisiert | Die Aktionen haben die erforderlichen Berechtigungsprüfungen bestanden |
| Ausgeführt | Für einen Ausführungsversuch liegt ein Ergebnis vor, einschließlich möglicher Fehler |

Das Repository demonstriert diese Ausführungsprüfungen mit Testkomponenten und Daten
im Arbeitsspeicher. Produktive Kalenderanbindungen und eine produktive Authentifizierung
sind nicht implementiert. Siehe die [Lifecycle-Referenz](foundation.md).

## Wenn ein Termin endet und der nächste beginnt

Auf einen Termin, der um 10:00 Uhr endet, kann direkt einer folgen, der um 10:00 Uhr
beginnt. Der erste belegt zu diesem Zeitpunkt keine Zeit mehr; der zweite beginnt genau
dort. Die Termine grenzen aneinander, überschneiden sich aber nicht.

Würde der zweite Termin schon um 09:59:59 Uhr beginnen, entstünde eine Sekunde
Überschneidung. Dieser Kandidat würde die Prüfung auf Überschneidungen nicht bestehen.

Cerebri schreibt einen Zeitraum als **[start, end)**: Der Anfang gehört dazu, das Ende
nicht mehr. Das nennt man ein halboffenes Intervall. Der ausgefüllte Anfangspunkt und
der offene Endpunkt im Diagramm zeigen dieselbe Regel. Siehe die [Zeitreferenz](temporal.md).

## Wie eine Wunschzeit die Reihenfolge verändert

Das nächste Beispiel ergänzt 10:45 Uhr als bevorzugten Beginn. Es ist ein eigenes
Beispiel; das vorherige enthält keine Wunschzeit. Ein Kandidat um 10:30 Uhr liegt
15 Minuten, also 900 Sekunden, von dieser Wunschzeit entfernt.

Der Wunsch hilft beim Vergleich von Kandidaten, die die verbindlichen Prüfungen bereits
bestanden haben. Auch ein weiter entfernter Beginn kann gültig sein. Eine Wunschzeit
macht einen überschneidenden Termin niemals zulässig.

## Warum ein gemessener Abstand von null erklärt werden muss

Die aktuelle Rangfolge verwendet Abstände in ganzen Sekunden. Eine Differenz von
0,8 Sekunden wird deshalb als null ganze Sekunden erfasst. Der genaue Zeitstempel
bleibt erhalten; der Termin wurde nicht auf die Wunschzeit verschoben.

Die kleinen Messbeispiele prüfen diese Berechnung direkt. Sie führen keine neue
Planersuche aus und verändern keine Kalendereinträge.

Außerdem macht es einen Unterschied, ob keine Wunschzeit angegeben wurde oder eine
Wunschzeit vorliegt und der gemessene Abstand null beträgt. Das Datenformat hält
beide Fälle als `None` und `Some(0)` auseinander. Der numerische Vergleich verwendet
in beiden Fällen null, die zugrunde liegenden Informationen bleiben jedoch verschieden.
Siehe [Ranking Feature Contract v0.1](../architecture/decisions/ADR-0014-ranking-feature-contract.md).

## Weiterlesen

- [CPIR](cpir.md): Welche Angaben eine Planungsanfrage enthält.
- [Planung](planner-integration.md): Wie die aktuelle Suche und der Vergleich funktionieren.
- [Zeit](temporal.md): Intervalle, Zeitzonen und unvollständige Verfügbarkeit.
- [Foundation](foundation.md): Die Prüfungen zwischen Vorschlag und Ausführung.
- [Roadmap](../development/roadmap/README.md): Aktueller Stand und spätere Forschung.

- [Lernweg und Forschungsfragen](research.md): Was später gelernt werden könnte und was heute bereits existiert.
