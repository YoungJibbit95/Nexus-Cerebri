# 2026-09-26 — Repository hardening

## Ausgangspunkt

Der Repository-Hardening-Slice startete auf
`8cc2d45cc5355aa8bfeb25f892bd2fe50f30b637` und wurde über PR #10,
`ci: harden repository governance and independent verification gates`, in
`968f8a427f02bef8ed9036c56baecd9bf217ca69` gemergt.

## Security / Supply Chain

- `gtapps/gh-traffic-stats@v1` wurde auf den immutable Commit
  `f05c995b41d817087f7f728dd1794af559d81471` gepinnt; dieser entspricht
  `v1.0.0`.
- Der Traffic-Workflow verwendet global `permissions: {}`; `contents: write`
  ist nur dem Traffic-Update-Job zugewiesen, der den Badge-Daten-Branch
  aktualisieren muss.
- Bereits direkt verwendete GitHub Actions bleiben auf immutable SHAs gepinnt.

## CI

Der frühere monolithische Foundation-Job wurde in unabhängige Gates aufgeteilt:

- Repository policy;
- Rust workspace;
- Rust dependency advisories;
- Rustdoc;
- Lab check;
- Lab test;
- Lab build;
- npm audit für root, Lab und Site;
- Documentation check / build;
- Site typecheck;
- Site build;
- Browser / E2E;
- Visual verification.

Damit verhindert ein einzelner Policy-Fehler nicht mehr automatisch die Ausführung
und Evidence-Erzeugung unabhängiger Gates.

## Governance

`AGENTS.md` dokumentiert jetzt die Arbeits- und Autoritätsgrenzen ausdrücklich:

- Arbeit erfolgt in bounded slices; Scope darf nicht still erweitert werden.
- Rust Core bleibt Domain- und Planning-Authority.
- Production-Code hängt nicht von `research/` ab.
- Fremde oder parallele Änderungen werden nicht überschrieben.
- Force Pushes und History Rewrites sind keine zulässigen Abkürzungen.
- Implementierung und unabhängige Architecture-/Math-/Security-Qualification
  bleiben getrennte Verantwortlichkeiten.
- Commit-/Push-Autorisierung ist keine Release-Autorisierung.
- Software-Version, Tag oder GitHub Release werden nur nach expliziter
  Maintainer-Anweisung für die jeweilige Publication-Aktion erstellt oder geändert.

## Site / Docs

- `figcaption` wurde semantisch korrigiert und ist jetzt direktes Kind des
  zugehörigen `figure`.
- Der Homepage-E2E-Test adressiert den Qualification-Status über die semantische
  `Release status` group statt über einen global mehrdeutigen Text-Locator.
- Die exakte Visual-Hash-Prüfung rendert den bereits unterstützten
  Reduced-Motion-Zustand, damit der statische Screenshot-Zustand deterministisch
  ist.
- Die daraus erzeugten Linux-Screenshots wurden überprüft und die vier Linux
  Visual Baselines anschließend auf die stabilen, in Retry identischen Hashes
  aktualisiert. Windows-Baselines blieben unverändert.

## Remote verification

Auf Merge-Commit `968f8a427f02bef8ed9036c56baecd9bf217ca69`
lief Cerebri CI #82 erfolgreich. Tatsächlich beobachtet wurden:

| Gate | Ergebnis |
| --- | --- |
| Repository policy | PASS |
| Rust workspace | PASS |
| Rust dependency advisories | PASS |
| Rustdoc | PASS |
| Lab check | PASS |
| Lab test | PASS |
| Lab build | PASS |
| npm audit (root) | PASS |
| npm audit (lab) | PASS |
| npm audit (site) | PASS |
| Documentation check / build | PASS |
| Site typecheck | PASS |
| Site build | PASS |
| Browser / E2E | PASS |
| Visual verification | PASS |

Documentation Pages #21 lief auf demselben Merge-Commit ebenfalls erfolgreich.

## Branch Protection

Nach dem Merge meldete GitHub für `main` weiterhin `protected: false`.
Der verwendete Browser-/GitHub-Agent konnte diesen Status lesen, verfügte aber
nicht über ausreichende Admin-Rechte, um Branch Protection selbst zu konfigurieren.
Branch Protection bleibt deshalb eine manuelle Maintainer-Aufgabe. Dieser Nachtrag
ändert keine Branch-Protection-Einstellung.

## Scope

Dieser Hardening-Slice änderte keine Ranking- oder Planner-Semantik. Es gab kein
Slice 2, kein ML und keine Änderung an CPIR-, Software- oder API-Versionen.
Es wurde kein Release und kein Tag erstellt.
