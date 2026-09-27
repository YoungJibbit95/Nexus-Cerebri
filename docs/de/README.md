# Nexus Cerebri verstehen und ausprobieren

Nexus Cerebri ist ein Lernprojekt darüber, wie sich Planungsentscheidungen erklären und
überprüfen lassen. Die aktuelle Umsetzung findet mögliche Zeiten anhand strukturierter
Daten und ausdrücklicher Regeln. Neuronales Lernen ist eine Forschungsrichtung und noch
kein Bestandteil des heutigen Planers.

## Mit der Idee beginnen und einem Beispiel folgen

1. [Warum es dieses Projekt gibt](introduction.md): die Lernmotivation, der Einfluss von
   Rumelhart, Hinton und Williams und ein erstes Terminbeispiel.
2. [Was der Planer braucht](cpir.md): Dauer, belegte Zeiten, Regeln und Berechtigungen,
   anschließend die zugehörigen Felder im CPIR-Eingabeformat.
3. [Wie mögliche Zeiten verglichen werden](planner-integration.md): welche Möglichkeiten
   ausscheiden, wie Wünsche die Reihenfolge beeinflussen und was ein Suchergebnis belegt.
4. [Wann eine Zeit frei ist](temporal.md): direkt anschließende Termine, unvollständige
   Informationen, Zeitzonen und wiederkehrende Termine.
5. [Was vor einer Änderung passieren muss](safety.md): warum ein Vorschlag vor der
   Ausführung noch geprüft und freigegeben werden muss.

## Weiter erkunden

- [Architektur](foundation.md) erklärt, welches Rust-Paket für welche Berechnung zuständig
  ist und wie API, Lab und Website darauf zugreifen.
- [Das Lab ausprobieren und lokal entwickeln](development.md) erklärt, wie du Ergebnisse
  untersuchst und mit API und Prüfwerkzeugen einsteigst.
- [Lernen und Forschung](research.md) verbindet den Ursprung des Projekts mit möglichen
  späteren Versuchen und grenzt sie von der vorhandenen Umsetzung ab.
- [KI-gestützte Entwicklung](ai-assisted-development.md) beschreibt menschliche
  Verantwortung und die erforderlichen Prüfungen für Beiträge.

## Den Entwicklungsstand richtig lesen

Software 0.2.0 ist **unveröffentlicht**. Spezifikation 0.4 und CPIR 0.2 (mit Unterstützung
für die ältere Version 0.1) bezeichnen getrennte Verträge. Aus ihren Nummern folgt nicht,
dass eine Produktversion verfügbar ist. Den Implementierungsstand zeigt die
[aktuelle Roadmap](../development/roadmap/README.md), den Veröffentlichungsstand das
[Changelog](../../CHANGELOG.md).

Diese öffentlichen Erklärseiten besitzen englische Gegenstücke. Technische Spezifikationen,
Architekturentscheidungen und datierte Fortschrittsprotokolle sind eigene Quellen und
nicht durchgehend zweisprachig. Datierte Protokolle erklären den damaligen Arbeitsstand;
sie ersetzen nicht die aktuellen Verträge.

[English](../en/README.md) · [Projektübersicht](../../README.md)
