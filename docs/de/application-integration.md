<!-- doc: application-integration; lang: de; counterpart: ../en/application-integration.md -->
# Eine Anwendung mit Cerebri verbinden

Das erste Integrationsprofil sucht Zeit für **ein einzelnes zukünftiges Event** in einem
ausdrücklichen Zeitfenster. Nexus kann damit einen Fokusblock zu einer Task vorschlagen;
andere Apps liefern dieselben allgemeinen CPIR-Daten. Produktzustand, Berechtigungen,
Datensammlung bei Providern und Bedienoberfläche bleiben bei der Anwendung. Rust behält
die Planungsautorität.

Dies ist eine lokale Entwicklungsgrundlage zur Prüfung, kein veröffentlichtes SDK oder
Produktiv-Rollout. [ADR-0016](../architecture/decisions/ADR-0016-application-suggestion-boundary.md)
dokumentiert den Vorschlag und die noch offene unabhängige Qualifikation.

## Vertrag und Zugänge

| Schnittstelle | Profil beschreiben | Vorschläge anfordern |
| --- | --- | --- |
| Rust | `cerebri_core::integration::describe()` | `suggest(SuggestionRequest)` oder `suggest_from_slice(bytes)` |
| Node-Host | `describeIntegration(options)` | `suggest(envelope, options)` aus `bindings/node/index.mjs` |
| Nativer Prozess | `cerebri-node-bridge --describe-integration` | `cerebri-node-bridge --suggest`, ein JSON-Auftrag über stdin |
| Entwicklungs-REST | `GET /v1/integration/manifest` | `POST /v1/integration/suggestions`, `application/json` |

Das Manifest nennt Integration **0.1**, Profil `single_event_suggestion`, CPIR **0.2**,
Operation `FIND_SLOT`, die Betriebsarten `Test`, `Shadow`, `Suggestion`, die erforderliche
Verfügbarkeitsangabe und fehlende Ausführungsunterstützung. `software_version` ist eine
Build-Angabe. Integrationsversion und Profil müssen eigenständig geprüft werden: Software
0.2.0, REST v1, CPIR, Ranking-Features, Evaluationsschemas und Veröffentlichungsstatus sind
getrennte Autoritäten. Die Version muss genau passen. Das Manifest ist keine Signatur und
kein Veröffentlichungsnachweis; Quellrevision und gebautes Programm separat festlegen und prüfen.

Der [vollständige synthetische Auftrag](../../examples/integration/suggestion.json) ist die
gemeinsame Vertragsfixture. Seine Hülle enthält genau `integration_version: { major: 0, minor: 1 }`
und `request` mit CPIR. `/v1/plan` und Node `plan()` behalten ihr allgemeines beziehungsweise
älteres Verhalten. Das neue Profil weicht bei Ablehnung nicht auf diese Zugänge aus.

## Anforderungen an die Eingabe

- CPIR 0.2 und `FIND_SLOT` verwenden; genau ein Event mit `revision: null` als Ziel angeben.
  Dessen Zeit ist `RESOLVED/MISSING`, weil seine Platzierung erst gesucht wird.
- Dauer, IANA-Zeitzone, eindeutige UTC-Zeitpunkte und begrenzten Scope ausdrücklich angeben.
  Keine Umwandlung eines bloßen Datums in Mitternacht, keine geschätzte Dauer oder erfundene
  Belegung und keine implizite Zeitzonenumrechnung. Unsichere Dauer darf die Policy hier nicht erlauben.
- `scope.max_mutations` und `policy.snapshot.mutation.max_mutations` auf `0` setzen;
  `allowed_actions` und `planning_capability.mutations` müssen leere Arrays sein.
  Betriebsart Test, Shadow oder Suggestion verwenden. Der Host muss Lese- und
  Planungsberechtigung eigenständig feststellen.
- `context.temporal` mit demselben Horizont wie der Scope, begrenzten Expansionsbudgets,
  ausdrücklichem `coverage` und einem gegebenenfalls leeren `series`-Array liefern.
  `Complete` ist eine Aussage des Aufrufers über die Datensammlung; eine leere Liste
  beweist sie nicht. `Incomplete` ist eine gültige Eingabe, führt im Kern aber zu
  InsufficientInformation und keinen Vorschlägen für freie Zeit.
- Provenienz und Wissenszustände erhalten. Bestehende Belegung braucht belegte Fakten.
  Fehlende/null-Filter, leere Arrays und gefüllte Scope-Filter behalten ihre verschiedenen
  [CPIR-Bedeutungen](cpir.md); Scope erteilt keine Berechtigung.

Der Kern prüft weiterhin Zeitauflösung, gleiche Horizonte, Evidenz, Berechtigungen, Scope,
Bedingungen, Abhängigkeiten und alle [Planerbudgets](planner-integration.md).
Die Fassade repariert unpassende Eingaben nicht stillschweigend.
`max_request_bytes: 262144` bezeichnet die Grenze der rohen JSON-Hülle, nicht sämtliche
Domänen- oder Evidenzbudgets. Direkte typisierte Rust-Aufrufe behalten die Domänenprüfung;
die Byte-Grenze gilt beim Dekodieren.

## Antwort richtig verstehen

`status: "rejected"` bedeutet, dass Dekodierung oder Profilzulassung gescheitert ist.
`code` ist einer dieser festen Werte:

```text
invalid_request                 request_too_large
unsupported_integration_version unsupported_cpir_version
unsupported_operation           mutation_authority_forbidden
unsupported_deployment_mode     prospective_event_required
temporal_coverage_required       uncertain_duration_forbidden
```

`status: "planned"` enthält `request_id`, `trace_id`, `context_revision` und als `result`
das vollständige, unveränderte Rust-PlanningResult. Dieser Status bedeutet nur, dass der
Planer geantwortet hat. `result.outcome` (Solution, NoSolution, NeedsRelaxation,
InsufficientInformation) und `result.assessment` (ProvenOptimal, Complete, BestFound)
getrennt darstellen. Reihenfolge, Platzierungen, Erklärungen, Ranking-Features, Konflikte,
Kompilierungs- und Graphevidenz erhalten. ProvenOptimal gilt nur im deklarierten begrenzten
Suchraster. BestFound behauptet keine Optimalität. Die Anwendung soll Kandidaten nicht mit
einem zweiten Planer neu sortieren.

Die Fixture liefert sieben Startzeiten in 15-Minuten-Schritten von 10:00 bis 11:30 UTC.
Bei Incomplete-Coverage gibt es keine Kandidaten und die Verfügbarkeit bleibt unbekannt.
Das sind synthetische Testfakten, keine Aussage über einen echten Kalender.

REST liefert 200 für einen abgeschlossenen Planeraufruf, auch bei fehlenden Informationen,
und 422 für Profilablehnung. Dekodierfehler behalten 400/415/422/413 für kaputtes JSON,
falschen Medientyp, falsche Struktur beziehungsweise zu große Eingaben. Ablehnungen enthalten
dieselbe bereinigte Hülle. Der Prozess liefert eine JSON-Antwort und Exitcode null auch für
behandelte Ablehnungen; Prozessfehler bleiben unterscheidbar. Kein Zugang bietet Ausführung.

## Node im Host verwenden

Mit `cargo build -p cerebri-node --locked` bauen; anschließend im Cerebri-Checkout:

```js
import { readFile } from 'node:fs/promises';
import { describeIntegration, suggest } from './bindings/node/index.mjs';

const manifest = await describeIntegration();
const envelope = JSON.parse(await readFile('./examples/integration/suggestion.json', 'utf8'));
const controller = new AbortController();
const response = await suggest(envelope, { signal: controller.signal, timeoutMs: 10_000 });
// Decode the result fields used by your view before presenting them.
```

`binary` ist eine optionale, vertrauenswürdige Host-Konfiguration; standardmäßig wird die
Debug-Binary dieses Checkouts verwendet. Niemals Programmpfad, Befehl, rohe CPIR-Autorität
oder Shell-Argumente aus einem nicht vertrauenswürdigen Renderer übernehmen. Der Host
übersetzt eine begrenzte Produktabsicht und erstellt daraus CPIR. Dieser Schritt liefert
weder Electron-Paketierung noch eine npm-Veröffentlichung. Vor Bereitstellung muss der
Verbraucher eine reproduzierbare, festgelegte Quell-/Binary-Abhängigkeit herstellen.

Die neuen Node-Aufrufe begrenzen die Prozesslaufzeit standardmäßig auf zehn Sekunden;
konfigurierbar sind 1 bis 60.000 ms. Dies begrenzt Fehlerfolgen und ist weder gemessenes
Latenzziel noch Suchbudget. Abbruch oder Zeitüberschreitung beenden den Kindprozess und
geben keinen Teilplan zurück. Die Ausgabe ist auf 16 MiB begrenzt, stderr wird verworfen.
`CerebriBridgeError.code` unterscheidet `aborted`, `timeout`, `bridge_unavailable`,
`bridge_failed`, `invalid_response`, `incompatible_bridge`, `request_too_large`,
`response_too_large`, `invalid_request` und `invalid_options`. Fehlermeldungen enthalten
keine rohen Parser-/Prozessausgaben. Das bisherige `plan()` bleibt unverändert.

Die TypeScript-Deklarationen prüfen/typisieren Manifest und Antworthülle; das innere
PlanningResult bleibt JSON. Sie behaupten **kein vollständig dekodiertes CPIR-Datenmodell**.
Die Anwendung muss verwendete Felder prüfen, nicht sicher darstellbare Ganzzahlen ablehnen
und Antworten dem aktuellen Auftrag/Kontext zuordnen. JSON darf nicht als ausführbarer
Lifecycle-Beweis gecastet werden. Ergebnisse bei Änderungen an Task, Scope,
Berechtigungen oder Verfügbarkeit verwerfen.

## Grenzen und nächster Schritt

Tasks, Reminder, Kalender und Datenbanken werden nicht verändert. Selbst ein validierter
Vorschlag kann nicht zu einem ActionPlan werden: Der Kern liefert `AnalysisOnly`.
Ein späterer Ausführungsschritt benötigt einen frischen autorisierten Auftrag und alle
Validierungs-, Aktualitäts-, Bestätigungs-, Capability- und Idempotenzprüfungen.
Die Auswahl eines Vorschlags ist keine Ausführungsberechtigung.

REST bleibt der lokale Entwicklungsserver. Authentifizierung, Isolation paralleler Last,
öffentliche Bereitstellung, Browser-/Mobile-Transport, allgemeine lokale Zeitinterpretation,
Provider-Adapter, Produktivbeobachtungen und gelerntes Ranking liegen außerhalb dieses Schritts.
Der neue Client sammelt oder protokolliert keine persönlichen Planungsdaten. Die bestehenden
synthetischen Evaluationstypen sind kein produktiver Sammel- oder Replay-Dienst.

Der [Nexus-Implementierungsprompt](../development/prompts/nexus-ecosystem-integration-foundation.md)
beschreibt den nächsten begrenzten Verbraucherschritt samt Tests. Für einen Rollback wird
das App-Feature deaktiviert und das additive Profil nicht mehr aufgerufen; eine Datenmigration
ist nicht erforderlich.

[English](../en/application-integration.md) · [CPIR](cpir.md) · [Lifecycle](safety.md)
