# Canonical Terminology and Planning Example

**Status:** Internal communication foundation  
**Snapshot:** `main@a2bd97ef25a414f8504b6738a51e5679ad638293`  
**Purpose:** Keep German/English terminology and the primary worked example semantically aligned with current implementation.

This document defines communication vocabulary and a canonical explanatory scenario. It does **not** change CPIR, planner behavior, ranking semantics or execution authority.

---

# 1. Terminology policy

Use a concept before its project-specific term whenever the audience has not already chosen a technical context.

German and English do not need literal sentence-by-sentence translation. They must preserve the same distinctions, status and technical referents.

Identifiers, enum variants, schema fields and source-code names stay in their canonical English form when quoted exactly.

## Core terminology map

| Concept | English public term | German public term | Exact technical term / note |
| --- | --- | --- | --- |
| structured planner input | planning request | Planungsanfrage | `PlanningRequest`; current planner consumes structured input |
| intermediate planning representation | CPIR | CPIR | Cerebri Planning Intermediate Representation; introduce expansion before relying on acronym |
| known information | fact | Fakt / bekannte Tatsache | `Fact`; distinct from inference and preference |
| mandatory planning rule | hard constraint | harte Bedingung / Hard Constraint | `HardConstraint`; invalid candidates are rejected |
| soft desired outcome | preference | Präferenz | `PreferenceProfile`; influences ordering, not hard validity |
| area the planner may consider | planning scope | Planungsbereich / Scope | `PlanningScope`; filters consideration, never grants authority |
| one considered placement | candidate | Kandidat | ranked only after validity checks |
| rule failure | violation | Regelverletzung / Constraint-Verletzung | structured rejection evidence |
| candidate comparison | ordering | Sortierung / Reihenfolge | current total lexicographic `CandidateOrderingKey` |
| preferred-start distance | preference distance | Präferenzabstand | whole-second absolute distance to resolved preferred start |
| origin of preferred-start evidence | preference source | Präferenzquelle | provenance; not an ordering term itself |
| request-context mutation count | mutation count | Änderungszahl | current analysis-only path = 0; current mutating single-target path = 1 |
| displacement from an existing placement | shift | Verschiebung | whole-second projection; prospective/no original placement = 0 |
| candidate ordering observations | ranking features | Ranking-Beobachtungen / Ranking Features | `RankingFeatureSet` v0.1; deterministic observation contract, not ML feature space |
| completely traversed declared grid with a solution | proven optimal | `ProvenOptimal` / nachgewiesen optimal im Raster | never imply global/continuous optimality |
| fully traversed declared grid without solution | complete | `Complete` / vollständig durchsucht | describes search completion, not success |
| search stopped before exhaustion | best found | `BestFound` / bestes bisher gefundenes Ergebnis | budget-bounded result |
| suggested plan before stronger lifecycle proofs | proposed plan | vorgeschlagener Plan | `ProposedPlan`; not authorization |
| deterministic validity proof | validated plan | validierter Plan | `ValidatedPlan`; validation does not grant permission |
| application-level permitted action plan | authorized action plan | autorisierter Aktionsplan | `AuthorizedActionPlan`; downstream from planning |
| actual execution result | execution result | Ausführungsergebnis | `ExecutionResult`; current production provider integration is not implemented |
| exact moment | instant | Zeitpunkt / Instant | use exact technical term where temporal precision matters |
| time interval | time range / interval | Zeitraum / Intervall | half-open `[start, end)` |
| unknown availability | unknown | unbekannt | never render or describe as free |
| current repository behavior | built / implemented | implementiert | reserve for evidence-backed behavior |
| active non-stable lab/research surface | experimental | experimentell | distinguish from stable capability |
| not currently implemented | research direction / future | Forschungsrichtung / zukünftig | do not phrase in present tense |

## Terms that require context

### “Intelligence”

Use only when the sentence makes clear whether it refers to:

- project/research ambition;
- deterministic reasoning/planning behavior;
- a future learned/neural component.

Do not use “intelligence” as evidence that neural inference currently exists.

### “Learning”

Do not use as a current runtime verb unless an implemented training/update path is being described.

Current provenance enum variants such as `PersonalLearned` and `GlobalLearned` do not themselves prove that Cerebri currently learns preferences.

### “Understanding” / “interpretation”

For current planner behavior, prefer concrete verbs such as “receives structured planning context,” “validates,” “searches,” “orders” and “returns.”

Natural-language understanding is not established as a current runtime boundary. If human wording is shown before CPIR, identify it as an explanatory framing rather than a parsed input.

### “Proof”

Use narrowly.

A search assessment may prove a property about the declared bounded grid/objective. Lifecycle types may encode successful validation/authorization transitions. Neither justifies an unqualified claim that Cerebri has “proven the best schedule in general.”

### “Travel”

`RequiredBuffer` is a generic temporal buffer rule. Do not call it travel time unless a concrete current representation actually establishes travel semantics for that example.

---

# 2. Status language

Public status language should distinguish at least:

- **Implemented / Built** — observable now.
- **Experimental** — implemented but explicitly non-stable/lab/research.
- **Research direction / Future** — not current behavior.

Do not rely on color alone. Do not expect a small status badge to undo a broader sentence or visualization that implies the wrong capability.

For the current repository snapshot:

- deterministic temporal planning baseline — **Implemented**;
- CPIR 0.2 with documented legacy 0.1 path — **Implemented**;
- Ranking Feature Contract v0.1 — **Implemented and independently Math/Security-qualified**;
- Lab — **Experimental developer/research surface**;
- production provider integration — **Future**;
- free-form NLP interpretation — **Future / not established as current runtime**;
- learned ranking/search and neural inference — **Future / Research**;
- autonomous production calendar mutation — **Future / not implemented**;
- software 0.2.0 publication — **not released**.

---

# 3. Canonical worked example

## 3.1 Why this example replaces the earlier informal baseline for implementation work

An earlier communication concept used:

> Find 30 minutes for a meeting with Anna tomorrow. Prefer the morning. Allow 15 minutes of travel beforehand.

That remains a useful human scenario, but the current repository does not establish all of its implied boundaries as one supported runtime path:

- “with Anna” may imply attendee/resource/provider semantics beyond the baseline;
- a natural-language sentence may imply an NLP parser that is not currently implemented;
- “travel beforehand” is more specific than the current generic `RequiredBuffer` rule;
- “tomorrow” requires a natural-language temporal-resolution step not demonstrated by the current planner request boundary.

The canonical implementation-backed communication scenario therefore uses only semantics established in current planner code.

## 3.2 Surface version

> Find a 30-minute slot inside a three-hour planning window.  
> The first hour is already occupied.  
> Prefer a start at 10:30.

This teaches the core problem without claiming language parsing, provider access or learning.

## 3.3 Exact scenario

Use these exact values when a deterministic example is needed:

- target: one prospective Event, ID `new-event`;
- operation: `CREATE`;
- planning horizon: `[2026-10-01T09:00:00Z, 2026-10-01T12:00:00Z)`;
- requested duration: 1,800 seconds;
- search granularity: 900 seconds;
- blocking existing Event: `busy` at `[2026-10-01T09:00:00Z, 2026-10-01T10:00:00Z)`;
- hard validity: no overlap with supplied blocking context; when represented explicitly, use `NoOverlap`;
- preferred start evidence:
  - source: `ExplicitCurrentRequest`;
  - preferred start: `2026-10-01T10:30:00Z`;
- target is prospective, so there is no original placement;
- use a candidate budget large enough to traverse all 11 grid positions;
- use the current mutating single-target path, so candidate `mutation_count = 1`.

This PR defines the communication scenario only. It does not add a new JSON fixture or alter the existing `examples/request.json`.

## 3.4 Search grid

With a 30-minute duration and 15-minute granularity, the declared horizon contains these 11 candidate starts:

| Start | Candidate interval | Valid? | Reason / preference distance |
| --- | --- | --- | --- |
| 09:00 | [09:00, 09:30) | no | overlaps `busy` |
| 09:15 | [09:15, 09:45) | no | overlaps `busy` |
| 09:30 | [09:30, 10:00) | no | overlaps `busy` |
| 09:45 | [09:45, 10:15) | no | overlaps `busy` |
| 10:00 | [10:00, 10:30) | yes | 1,800 s from preferred start |
| 10:15 | [10:15, 10:45) | yes | 900 s |
| 10:30 | [10:30, 11:00) | yes | 0 s |
| 10:45 | [10:45, 11:15) | yes | 900 s |
| 11:00 | [11:00, 11:30) | yes | 1,800 s |
| 11:15 | [11:15, 11:45) | yes | 2,700 s |
| 11:30 | [11:30, 12:00) | yes | 3,600 s |

The 10:00 candidate is intentionally valid: `busy` ends at 10:00 and intervals are half-open, so touching at the boundary does not overlap.

## 3.5 Deterministic ordering

For this prospective `CREATE` example:

- every valid candidate has `mutation_count = 1`;
- every valid candidate has `shift_seconds = 0` because no original placement exists;
- all candidates share the same target object ID;
- the preferred-start distance is therefore the first distinguishing ordering component;
- equal distances are broken by the earlier start time.

Expected order:

1. 10:30 — distance 0
2. 10:15 — distance 900
3. 10:45 — distance 900
4. 10:00 — distance 1,800
5. 11:00 — distance 1,800
6. 11:15 — distance 2,700
7. 11:30 — distance 3,600

The exact current total key remains:

`(distance.unwrap_or(0), mutation_count, shift_seconds, start, object_id)`

The simple explanation “choose the valid slot closest to 10:30” is correct for this scenario because the later tie-break terms are equal until `start`.

The technical explanation must still expose the full key.

## 3.6 Expected search assessment

If the candidate budget allows all 11 positions to be evaluated:

- there are valid candidates;
- the declared grid is exhausted;
- outcome is `Solution`;
- assessment is `ProvenOptimal`.

The phrase must be understood as:

> Proven optimal for this declared discrete grid and this deterministic ordering objective.

It does not mean:

- optimal over continuous time;
- optimal for arbitrary scheduling objectives;
- optimal for multi-object repair;
- optimal for a future learned objective;
- globally optimal across calendars/providers not supplied in the request.

## 3.7 Explanation layers for this example

### Level 0 — orientation

Cerebri can search a bounded set of possible times, reject times that violate rules and order the valid ones deterministically.

### Level 1 — intuition

The first hour is occupied, so those overlapping options are impossible. Among the remaining options, the request says 10:30 is preferred. The valid option at 10:30 therefore comes first.

### Level 2 — worked example

Show the 11 positions on one timeline.

- rejected positions visibly intersect the occupied interval;
- valid positions remain;
- preference distance is revealed only after validity is understood;
- 10:30 becomes the first ranked candidate;
- 10:15 and 10:45 demonstrate deterministic tie-breaking.

### Level 3 — mechanism

Introduce:

- planning horizon;
- duration;
- granularity;
- hard validity;
- preference evidence;
- candidate;
- ranking feature;
- ordering key;
- search assessment.

### Level 4 — exact implementation

Link to:

- `crates/cerebri-planner/src/model.rs`;
- `crates/cerebri-planner/src/search.rs`;
- `crates/cerebri-constraints/src/lib.rs`;
- `crates/cerebri-preferences/src/lib.rs`;
- `crates/cerebri-preferences/tests/ranking.rs`;
- `docs/en/planner-integration.md`;
- `docs/de/planner-integration.md`;
- ADR-0014.

---

# 4. Visual specification for the canonical example

The canonical timeline should encode semantics rather than decoration.

## Required visual objects

- **Planning horizon:** one bounded horizontal or vertical interval.
- **Existing busy interval:** a clearly distinct blocking interval.
- **Candidate start positions:** repeated discrete positions at 15-minute spacing.
- **Rejected candidate:** status plus visible reason path to the blocking interval.
- **Valid candidate:** distinct from rejection without implying it is preferred.
- **Preferred start:** an explicit reference marker at 10:30.
- **Ordering distance:** optional measurement revealed after validity.
- **Selected/first candidate:** 10:30, marked as first in deterministic order.
- **Assessment:** `ProvenOptimal` only after the final grid position has been traversed.

## Motion semantics

If animated:

1. establish the horizon;
2. reveal the occupied interval;
3. generate grid positions;
4. reject overlapping positions;
5. reveal the preferred-start marker;
6. compare valid distances;
7. settle candidates into deterministic order;
8. show the bounded assessment after exhaustion.

Reduced-motion mode must show the same final states and reasons without requiring sequence timing.

## Do not show

- a neural network choosing the slot;
- a provider/calendar sync completing;
- a natural-language parser transforming the sentence;
- travel routing;
- multiple attendees/resources;
- continuous optimization;
- an autonomous mutation of an external calendar.

Those visuals would introduce unsupported semantics into this example.

---

# 5. German rendering of the canonical example

A natural German surface version:

> Finde innerhalb eines dreistündigen Planungsfensters 30 Minuten.  
> Die erste Stunde ist bereits belegt.  
> Bevorzuge einen Start um 10:30 Uhr.

Technical details remain the same exact UTC instants in the canonical fixture. A later user-facing local-time presentation may render them in an explicit timezone, but the underlying example must not silently change.

Suggested German explanation:

> Zuerst werden Zeiten verworfen, die sich mit dem belegten Zeitraum überschneiden. Danach werden nur die gültigen Kandidaten verglichen. Für dieses Beispiel liegt 10:30 Uhr genau auf der bevorzugten Startzeit und steht deshalb an erster Stelle. 10:15 Uhr und 10:45 Uhr sind gleich weit entfernt; der frühere Zeitpunkt entscheidet den Gleichstand.

Technical qualifier:

> `ProvenOptimal` gilt hier nur für das vollständig durchsuchte 15-Minuten-Raster und den aktuellen deterministischen Sortierschlüssel.

---

# 6. English rendering of the canonical example

Suggested surface explanation:

> First, discard times that overlap the occupied interval. Then compare only the valid candidates. In this example, 10:30 exactly matches the preferred start, so it ranks first. 10:15 and 10:45 are equally distant; the earlier start breaks the tie.

Technical qualifier:

> `ProvenOptimal` applies only to the fully exhausted 15-minute grid and the current deterministic ordering key.

The German and English versions may differ stylistically. They must preserve the same facts, numbers, boundaries and status.

---

# 7. Canonical-example truth checklist

Before reusing or extending this scenario, verify:

- [ ] still exactly one target in the baseline search;
- [ ] duration and granularity semantics are unchanged;
- [ ] half-open interval behavior is unchanged;
- [ ] supplied context still blocks overlap as documented;
- [ ] preferred-source resolution is unchanged;
- [ ] ranking feature schema/status is current;
- [ ] total ordering key is unchanged;
- [ ] mutation-count semantics are unchanged;
- [ ] prospective-object shift projection is unchanged;
- [ ] search-assessment semantics are unchanged;
- [ ] no added wording implies NLP, learning, provider access or execution;
- [ ] DE/EN numbers and claims remain equivalent.

If a checked invariant changes, update the technical truth first and then rederive every simpler explanation and visual from it.
