<!-- doc: development; lang: de; counterpart: ../en/development.md -->
# Entwicklung und Prüfung

Rust 1.97.0 ist festgelegt; für Werkzeuge wird Node 22+ benötigt. Im Repository-Wurzelverzeichnis:

```sh
cargo fmt --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo build -p cerebri-node --locked
node --test bindings/node/test.mjs
npm ci --ignore-scripts
npm run check
npm run docs:build
cargo doc --workspace --no-deps --locked
```

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

API starten und /lab mit examples/request.json verwenden. Planner zeigt UTC-Zeitbalken und
Bewertungen, Trace typisierte Validierungs-/Konfliktdaten, Preferences die Bewertungskomponenten.
Semantics, ML und Dataset sind reservierte Ansichten. JSON-Export erhält die zugrunde liegenden Daten.
Die Oberfläche ist internes Werkzeug ohne stabile Produktgarantie. Fixtures enthalten nur
synthetische IDs und Daten, keine Kalendertitel, Beschreibungen, personenbezogenen Daten oder Tokens.

Der Pages-Workflow wird manuell auf main gestartet. Er generiert HTML aus Repository-Markdown,
stellt DE/EN-Navigation bereit und benötigt GitHub-Pages-Konfiguration. Lokale Builds veröffentlichen
nichts. Diese Workflows erzeugen keine Software-Releases.

[Architektur](foundation.md) · [English](../en/development.md)

