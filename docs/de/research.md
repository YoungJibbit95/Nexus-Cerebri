<!-- doc: research; lang: de; counterpart: ../en/research.md -->
# Von der Planung zum Lernen

**Forschungsrichtung: Die hier beschriebenen Lernverfahren sind noch nicht implementiert.**
Der aktuelle Planer verarbeitet strukturierte Angaben und vergleicht Termine nach festen
Regeln. Er trainiert kein Modell und lernt nicht aus deinen Entscheidungen.

[English](../en/research.md) · [Projektgeschichte](introduction.md) · [Architektur](foundation.md)

## Die Frage hinter dem Projekt

Ich wollte verstehen, wie neuronale Netze lernen. Dazu gehörte die Arbeit von
David E. Rumelhart, Geoffrey E. Hinton und Ronald J. Williams über
[Backpropagation von 1986](https://www.nature.com/articles/323533a0). Die Formeln wirkten
zunächst schwer zugänglich. Als ich die einzelnen Rechenschritte nachvollzog, konnte
ich erkennen, was sie tatsächlich beschreiben.

Mit Cerebri möchte ich auf diese Weise weiterlernen: Ideen selbst umsetzen, Annahmen
testen und verstehen, warum ein Ergebnis entsteht. Zeitplanung bietet dafür konkrete
Fragen. Passt ein Termin? Was ist bereits belegt? Welche von mehreren passenden Zeiten
ist nützlich? Und woran würde ich erkennen, dass ein lernendes Verfahren dabei hilft?

## Warum zuerst ein Planer mit festen Regeln entsteht

Im [Beispiel](cpir.md) bleiben sieben passende Startzeiten übrig. Ohne Wunschzeit kommt
10:00 Uhr zuerst. Die vorhandenen Regeln erklären dieses Ergebnis, und Tests können
prüfen, ob eine Änderung das Verhalten beeinflusst.

Das schafft eine Vergleichsgrundlage für spätere Experimente. Ein Modell müsste zeigen,
welche Aufgabe es besser löst. Es dürfte dabei weder einen belegten Zeitraum als frei
behandeln noch einen unzulässigen Termin durch eine gute Bewertung zulässig machen.
Die Prüfung der Regeln bleibt eine eigene Aufgabe.

## Welche Fragen später untersucht werden könnten

**Eine Anfrage verstehen.** Aus „am Nachmittag eine halbe Stunde“ könnte ein Verfahren
Angaben für CPIR vorschlagen. Es müsste Unklarheiten erkennen, etwa welchen Tag die
Person meint. Heute muss eine Anwendung die strukturierten Angaben selbst liefern;
eine solche Texteingabe wird vom aktuellen Planer nicht interpretiert.

**Passende Zeiten vergleichen.** Bei mehreren zulässigen Terminen könnte ein Experiment
untersuchen, ob frühere ausdrückliche Entscheidungen hilfreiche Vorlieben erkennen
lassen. Eine aktuelle Vorgabe müsste weiterhin Vorrang haben. Das System lernt solche
Vorlieben derzeit nicht. Dass Datenfelder eine gelernte Herkunft benennen können,
bedeutet noch nicht, dass ein Lernverfahren vorhanden ist.

**Gezielter suchen.** Bei größeren Aufgaben könnte ein Modell Hinweise geben, welche
Möglichkeiten zuerst geprüft werden sollten. Ob das innerhalb desselben Suchbudgets
bessere Ergebnisse liefert, wäre zu messen. Eine abgebrochene Suche dürfte weiterhin
nur berichten, was sie tatsächlich geprüft hat. Die heutige Suche behandelt einen
Zieltermin in einem festgelegten Zeitraster.

**Persönliche Muster berücksichtigen.** Später könnte untersucht werden, wie sich
langfristige Vorlieben von einem Wunsch in der aktuellen Situation unterscheiden.
Persönliche Rückmeldungen sollen nicht automatisch zu gemeinsamen Trainingsdaten
werden. Der aktuelle Planer sammelt keine solchen Rückmeldungen.

**Weitere Anwendungen unterstützen.** Aufgaben, Räume oder Schichten sind mögliche
spätere Einsatzfelder. Dafür wären jeweils passende Daten, Regeln und geprüfte
Anbindungen nötig. Produktive Kalender- oder Anbieteranbindungen sind noch nicht
vorhanden; ein Modell allein würde sie nicht ersetzen.

## Was Lernen von Grund auf hier heißen soll

Der reservierte [Lernpfad](../../research/ml-from-scratch/README.md) sieht vor, ein kleines
neuronales Netz schrittweise selbst zu implementieren. Die Lektionen existieren noch
nicht. Geplant ist, die Berechnung nachvollziehbar zu machen:

1. Eingaben durch das Netz rechnen und die Zwischenwerte ansehen.
2. Mit einer Fehlerfunktion messen, wie weit die Ausgabe vom gewünschten Ergebnis entfernt ist.
3. Mit Backpropagation berechnen, wie Änderungen der Parameter diesen Fehler beeinflussen.
4. Die Parameter anpassen und beobachten, wie sich die nächste Ausgabe verändert.
5. Die berechneten Ableitungen durch kleine numerische Änderungen und eine unabhängige
   Framework-Referenz überprüfen.

Damit würde der Lernpfad zur Ausgangsfrage zurückkehren: Wie wird aus einer Formel ein
Verfahren, dessen einzelne Schritte ich erklären und prüfen kann? Diese Übungen wären
vom produktiven Planer getrennt. Ein gelungenes Lernbeispiel wäre noch kein Nachweis,
dass ein Modell echte Planungsaufgaben besser löst.

## Was dafür heute schon vorbereitet ist

Die [Ranking-Beobachtungen](../architecture/decisions/ADR-0014-ranking-feature-contract.md)
beschreiben bereits berechnete Vergleichswerte und ihre Herkunft. Sie verändern die
feste Reihenfolge nicht und sind kein trainiertes Bewertungsmodell.

Die [Evaluationsgrundlagen, Phase A](../architecture/decisions/ADR-0015-evaluation-contract-foundations.md)
definieren Datentypen und prüfen das Übertragungsformat synthetischer Einträge. Sie
sammeln noch keine Planerbeobachtungen, berechnen keine Fingerabdrücke und führen keine
Wiederholung früherer Läufe aus. Die semantische Gesamtprüfung bleibt ebenfalls spätere Arbeit.

[cerebri-ml](../../crates/cerebri-ml/src/lib.rs) enthält Metadaten und Schnittstellen für
Modelle und Datensätze. Ein Trainingslauf, ein einsatzfähiges Modell oder ein Anschluss
an die aktuelle Planersuche entsteht dadurch noch nicht.

## Woran ein Experiment gemessen werden soll

Ein Versuch braucht eine konkrete Frage, dokumentierte Daten und Einstellungen sowie
einen nachvollziehbaren Vergleich mit dem bisherigen Verfahren. Beispiele, mit denen
ein Modell lernt, müssen von denen getrennt bleiben, an denen seine Leistung geprüft
wird. Auch Fehler, unklare Anfragen und ungünstige Fälle gehören in die Auswertung.
Mehr Komplexität wäre nur sinnvoll, wenn sich ein Nutzen zeigen lässt.

Selbst bessere Vorschläge würden keine Ausführungsrechte erteilen. Fakten, verbindliche
Regeln, Berechtigungen und die [Prüfungen vor einer Änderung](foundation.md) bleiben
außerhalb der Zuständigkeit eines lernenden Modells.

Der [aktuelle Fahrplan](../development/roadmap/README.md) hält den Implementierungsstand
fest. Der [Entwicklungs- und Lernplan](../development/roadmap/development-learning-roadmap.md)
beschreibt spätere Etappen; seine Zeit- und Versionsziele sind keine Veröffentlichungstermine.
Die [Forschungsstandards](../testing/testing-visualization-research-standard.md) beschreiben,
wie Experimente und Vergleiche dokumentiert werden sollen.
