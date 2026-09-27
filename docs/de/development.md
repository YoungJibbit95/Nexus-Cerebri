<!-- doc: development; lang: de; counterpart: ../en/development.md -->
# Cerebri ausprobieren und Ergebnisse untersuchen

Die öffentliche Website erklärt feste Beispiele. Das **Lab** ist ein separates lokales
Werkzeug, mit dem du strukturierte Anfragen an die Rust-Implementierung senden und die
Antworten untersuchen kannst. Es hilft, wenn du eine Eingabe verändern und die Auswirkung
auf das Ergebnis sehen möchtest.

## Den passenden Einstieg wählen

- Für das Projekt und seine Lernmotivation beginne mit der [Einführung](introduction.md).
- Für eine eigene strukturierte Anfrage nutze die Lab-Anleitung unten. Der aktuelle Planer
  verarbeitet [CPIR-Daten](cpir.md), keinen Satz mit einem Terminwunsch.
- Für die Anbindung einer Anwendung stehen die Rust-Fassade, die lokale REST-API und die
  vorläufige [Node-Prozessbrücke](../../bindings/node/README.md) bereit. Alle drei Wege
  führen zum selben Rust-Code.

## Das Lab lokal starten

Du brauchst Rust 1.97.0 und Node 22.12 oder neuer. Im Repository-Wurzelverzeichnis:

```sh
npm --prefix apps/cerebri-lab ci
npm --prefix apps/cerebri-lab run build
cargo run -p cerebri-api
```

Lass den letzten Befehl weiterlaufen und öffne <http://127.0.0.1:3000/lab/>. Die API stellt
das gebaute Lab bereit und ist nur auf dem lokalen Rechner erreichbar. Wurden die Dateien
noch nicht gebaut, liefert die Lab-Seite einen 404-Fehler. Für die Oberflächenentwicklung
mit Vite gibt es die [Lab-Anleitung](../../apps/cerebri-lab/README.md).

## Was du untersuchen kannst

Beginne mit einem vorhandenen synthetischen Beispiel. **Planner** zeigt mögliche Zeiten
und ihre Vergleichswerte. **Trace** zeigt Eingabeprüfungen und Ablehnungsgründe.
**Temporal** zeigt belegte, freie und unbekannte Zeit sowie Entscheidungen bei Zeitumstellungen.

Das Lab-Beispiel mit bevorzugtem Start enthält den Wunsch 10:45 Uhr UTC. Das einführende
Beispiel auf der Website enthält keine Wunschzeit. Das sind zwei unterschiedliche Eingaben;
ihre erste Wahl kann sich unterscheiden, ohne dass eines der Ergebnisse falsch ist.
Wähle dieselbe Eingabe, wenn du Ergebnisse vergleichen möchtest.

**Semantics** und **Preferences** zeigen bereits übergebene oder berechnete Informationen.
**ML** und **Dataset** sind inaktive Platzhalter für spätere Arbeit. Sie trainieren kein
Modell und erschließen keine Vorlieben. Über den JSON-Export kannst du die untersuchten
Daten aufbewahren; das Lab speichert Anfragen und Ergebnisse nicht im lokalen Browserspeicher.

## Was die API prüft

Die [lokalen REST-Routen](../../apps/cerebri-api/src/lib.rs) nehmen strukturierte Anfragen an:

- `POST /v1/validate` prüft, ob eine Planungsanfrage die Anforderungen an die Eingabe erfüllt.
- `POST /v1/plan` prüft die Anfrage, sucht mögliche Platzierungen und liefert das Ergebnis.
- `POST /v1/temporal` berechnet Zeitdiagnosen, etwa Wiederholungen und Verfügbarkeit.

Die Eingabeprüfung ist nicht die spätere Prüfung eines vollständigen Planvorschlags und
erteilt keine Ausführungsrechte. REST und Node bieten keine Operation zum Ausführen von
Kalenderänderungen. Siehe dazu die [Schritte vor der Ausführung](safety.md).

## Entwicklung und Prüfung

Rust 1.97.0 ist festgelegt; für Werkzeuge wird Node 22.12+ benötigt. Im Repository-Wurzelverzeichnis:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `cargo test --workspace --locked`
- `cargo build -p cerebri-node --locked`
- `node --test bindings/node/test.mjs`
- `npm ci --ignore-scripts`
- `npm run check`
- `npm run docs:build`
- `npm --prefix apps/cerebri-lab ci`
- `npm --prefix apps/cerebri-lab run check`
- `npm --prefix apps/cerebri-lab test`
- `npm --prefix apps/cerebri-lab run build`
- `npm audit --audit-level=high`
- `npm --prefix apps/cerebri-lab audit --audit-level=high`
- `cargo doc --workspace --no-deps --locked`

CI führt diese Prüfungen aus und behandelt Rustdoc-Warnungen als Fehler. Tests prüfen
Wissenszustände/Schema-Roundtrips, Scope-Präsenz, Constraint-Ablehnung, veraltete Snapshots,
planbezogene Bestätigung, entzogene Rechte, Replay, Teilerfolge und verlorenen Ledger-Abschluss.
Compile-fail-Doctests belegen, dass frühere Lifecycle-Typen execute nicht aufrufen können.
Intervalle werden auf einem kleinen Definitionsbereich erschöpfend geprüft, ohne Zufallsseed.
Berlin-DST-Fälle sind fest vorgegeben. Alle Zeitwerte sind Eingaben.

Der Repository-Check prüft Crate-Abhängigkeiten, lokale Markdown-Links, DE/EN-Dateien mit
Sprachmetadaten, Versionsmarker und ausgewählte Credential-Muster. Er ersetzt weder einen
vollständigen Secret-Scanner noch Übersetzungsprüfung und externe Linkprüfung.
Cargo.lock und package-lock.json sind versioniert; target/, site/ und Binärdateien werden ignoriert.

Die Oberfläche ist internes Werkzeug ohne stabile Produktgarantie. Fixtures enthalten nur
synthetische IDs und Daten, keine Kalendertitel, Beschreibungen, personenbezogenen Daten oder Tokens.

Der [Pages-Workflow](../../.github/workflows/pages.yml) läuft nach Pushes auf main und lässt
sich auf main auch manuell starten. Er baut die Website einschließlich der Markdown-Seiten
aus dem Repository und der DE/EN-Navigation. Danach veröffentlicht er sie auf dem
konfigurierten GitHub Pages. Ein lokaler Build veröffentlicht nichts. Die Veröffentlichung
der Dokumentation erzeugt kein Software-Release.

[Architektur](foundation.md) · [English](../en/development.md)

