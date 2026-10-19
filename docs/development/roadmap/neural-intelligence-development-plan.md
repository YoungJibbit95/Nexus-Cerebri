# Cerebri Neural Intelligence — development plan v1

Status: Proposed plan — documentation only; no ML implementation is authorized.

This plan sequences the future Neural Intelligence work. It does **not** implement it,
does not lift any existing restriction, and does not change any accepted decision. It
refines the staged gates of [ADR-0017](../../architecture/decisions/ADR-0017-ml-nlp-architecture-proposal.md)
into small, reviewable, dependency-ordered pull requests and maps them to the
[Cerebri Lab](../../../apps/cerebri-lab/README.md) learning experience and to a
verification strategy.

## 1. Purpose and governing principle

Cerebri stays a deterministic temporal-planning engine. Neural intelligence is added only
as a bounded interpretation aid under the governing principle **neural intuition +
symbolic verification** ([Master v0.4](../../architecture/specifications/master-v0.4.md) §1–2, §13–16):
a model may propose interpretation evidence, but it never becomes authority for facts,
permissions, hard constraints, CPIR admission, feasibility, ordering or execution.

## 2. Verified current state

Present seams, not a runtime:

- `cerebri-ml` defines `ModelRegistry`, the `Inference` port and versioned
  `ModelMetadata`/`TrainingRunMetadata`/`DatasetMetadata` with checksum-addressed
  `ArtifactReference` ([source](../../../crates/cerebri-ml/src/lib.rs)).
- `cerebri-types` already carries `Provenance::ModelInference`, and `is_fact_source()`
  correctly excludes it, plus a `Knowledge<T>` state with `Missing`/`Unknown`/`Uncertain`/`Ambiguous`.
- `cerebri-semantics` derives deterministic temporal evidence; it does not parse arbitrary text.
- The Lab reserves `ML` and `Dataset` panes and labels them `LATER / NOT CONNECTED / PLANNED`.

Absent today: text normalization/tokenization, intent or slot extraction, training data,
training code, model artifacts and any local inference runtime. `research/ml-from-scratch/`,
`models/` and `datasets/` contain only READMEs.

## 3. Authority boundaries (unchanged)

Model output is **inference evidence**, never authoritative reality. The following remain
outside the model's authority: verified facts and provider availability; permissions and
authorization; hard planning constraints; CPIR admission and validation; deterministic
feasibility; the current ranking contract; execution authorization and lifecycle preflight;
claims of completeness or optimality; and the creation of application side effects.
Unknown availability stays unknown; a confidence score is never a permission.

## 4. Governance status and prerequisites

- The [roadmap](README.md) currently states: *"Do not begin ML, provider integration,
  advanced repair search or autonomous mutation."* **This plan does not lift that
  restriction.** Lifting it requires an explicit maintainer amendment (see §14).
- [ADR-0017](../../architecture/decisions/ADR-0017-ml-nlp-architecture-proposal.md) is
  `Proposed`; a merged proposal PR is not implementation authorization.
- Open independent qualifications affect sequencing: ADR-0015 B.4/full Phase-B and
  ADR-0016 Architecture/Security qualification are pending.

Milestone A (this document) is the only currently unblocked step, because it is documentation.

## 5. Technology decisions

Confirmed by the maintainer on 2026-10-19:

| Responsibility | Decision |
| --- | --- |
| Core / planner / contracts | Existing Rust workspace; no heavyweight ML runtime required |
| Research and training | Python + PyTorch, offline and separate from production |
| Rust teaching implementation | Burn, isolated from the production workspace |
| Local production-candidate inference | ONNX exported model via Rust `ort`, optional feature/backend |
| Runtime comparison only | Hugging Face Candle |
| Tokenization | Hugging Face `tokenizers` (Rust), exact `tokenizer.json` pinned in the artifact manifest |
| Visualization | Existing Svelte Cerebri Lab |
| Deployment | Local, offline-capable, CPU-first; no mandatory Python, no automatic downloads |

Exact versions/licenses/build configurations are chosen in the milestone that introduces
them, with a dependency allowlist change approved as a separate slice.

## 6. Target interpretation pipeline

```text
Untrusted natural-language text
        |
        v
Bounded limits + normalization + tokenization
        |
        +--> deterministic parser/rules (baseline, always available)
        |
        +--> optional versioned neural inference (local only)
        |
        v
IntentDraft: candidate intent, typed slot proposals, source spans,
             per-field confidence/abstention, missing/ambiguous fields
        |
        v
Deterministic resolver and validation (reference time, IANA zone,
             contradictions, negation, DST gap/fold, clarification)
        |
        v
Trusted host adapter builds a CPIR 0.2 planning request
        |
        v
Existing deterministic planner -> PlanningResult -> host preview
        |
        v
Explicit user/application decision; execution stays a separate boundary
```

No generic `impl From<IntentDraft> for PlanningRequest` may exist: promotion of a
prediction to a fact always passes the deterministic resolver and host validation.

## 7. PR roadmap

Each row is one bounded, reviewable, potentially mergeable pull request. Names are labels,
not calendar promises.

| PR (milestone) | Scope | Dependencies | Lab deliverable | Documentation | Verification |
| --- | --- | --- | --- | --- | --- |
| A — Plan v1 | This plan, roadmap pointer, progress record (docs-only) | none | none (references reserved panes) | this document | `repository_policy`, `documentation` |
| B — Deterministic baseline | Bounded single-event intent ontology (`FIND_SLOT`, `CREATE_EVENT_DRAFT`, `UNKNOWN`, `UNSUPPORTED`, `CLARIFICATION_NEEDED`), rule parser, exact date/duration/negation, adversarial DE/EN fixtures, measured baseline | A + explicit lift of the §4 restriction + an implementation ADR | deterministic-interpretation explanation (SIMPLE/TECHNICAL) | interpretation contract spec | Rust tests, DST gap/fold, negatives, `rust-workspace` |
| C — Dataset & evaluation | Versioned synthetic DE-first + EN challenge data, splits with no template leakage, manifests/hashes/provenance | B (baseline as denominator) | Dataset pane activated (versions, splits, distributions) | dataset schema + reproduction guide | dataset validation, leakage and distribution checks |
| D — Neural net from scratch | Educational tensor/linear/activation/embedding/softmax/loss/SGD/backprop in `research/ml-from-scratch/`, fixed seeds, numerical gradient checks | C | Neural Network Playground, Forward-pass and Backpropagation explorers on real values | mathematical lessons DE/EN | manual vs. numerical vs. independent autograd gradients |
| E — PyTorch intent model | Embedding → mean pooling → small MLP → intent logits; seeds, checkpoints, independent evaluation | C, D | Training dashboard on genuine experiments | experiment report | intent macro-F1, calibration, abstention, DE/EN, baseline comparison |
| F — Local Rust inference | ONNX export and `ort` backend; numerical and tokenizer parity; offline, CPU, Win/Linux | E | (backend spike; no UI) | runtime, size and latency measurements | parity, integrity, cold start, corrupt/incompatible artifacts |
| G — Lab neural interpretation | Opt-in experimental NLP flow with rule comparison | F + ADR-0016 qualification | Decision Inspector + Model Comparison | user/developer guide | E2E, accessibility, rules-only fallback |
| H — Learning to rank | Pairwise preference research on feasible candidates | separately gated: completed evaluation/architecture/math/security qualification | ranking comparison | dedicated ADR | controlled experiments; hard validity unchanged |

## 8. Dependency graph

```text
A --> B --> C --> D --> E --> F --> G --> (H)
                    \--> E
```

Unblocked now: **A** only. B–H each require the preceding milestone plus the explicit
governance decisions in §4/§14.

## 9. Lab learning experience

The Lab is a first-class deliverable, not an afterthought. Planned modules: Neural Network
Playground (A/D), Forward-pass Explorer (D), Backpropagation Explorer (D), Training
Dashboard (E), Model Decision Inspector (G) and Model Comparison (E/G). Every displayed
value must come from real calculations or clearly labelled, versioned replayed fixtures.
Manual stepping is preferred for multi-step mathematics; SIMPLE/TECHNICAL/RESEARCH depths,
keyboard access, reduced-motion support and honest loading/error/unavailable states apply.
No fabricated charts or placeholder values presented as results.

## 10. Dataset and evaluation strategy

Start synthetic, DE-first with EN challenge fixtures. Keep `training / validation / test /
held-out challenge` separate, avoid template leakage, and include difficult negatives,
negation, contradiction, ambiguity and hostile input. Record dataset identity/version,
content hashes, provenance/licence and scope, generation method and seed, class and language
distributions, metrics and experiment configuration. Selected metrics: intent macro-F1,
per-slot precision/recall/F1, exact-span/exact-value match, temporal normalization accuracy,
calibration/abstention and — as a hard invariant — **zero** validated hard-constraint
violations on adversarial scenarios. Personal planning data is never used for training by
default.

## 11. Privacy and security invariants

Local-first; no mandatory network after trusted installation; no hidden inference telemetry;
no automatic elevation of `PERSONAL_LOCAL` data into shared datasets; model and tokenizer
artifacts require source, licence, checksum and resolver verification; untrusted text is data,
never instructions; bounded input sizes, typed contracts, explicit errors and safe resource
limits; stale interpretation against a changed context is rejected; no execution authority is
derived from any NLP result.

## 12. Verification strategy

For each milestone, define mandatory checks before approval and run only the checks justified
by the touched components: Rust `fmt`/`build`/`clippy -D warnings`/`test`; Rustdoc; dependency
audit; repository policy; model-contract and dataset validation; numerical gradient checks;
independent mathematical reference tests; ML evaluation scripts; Rust/Python numerical parity;
Svelte type check, Lab unit tests and build; documentation check/build; browser/E2E,
accessibility and visual checks; offline inference, negative and malformed-input tests. A local
smoke test never replaces the required independent qualification, and AI-assisted work is not
self-certified.

## 13. Risk register

| Risk | Impact | Mitigation |
| --- | --- | --- |
| Model output mistaken for authority | High | typed draft + deterministic resolver; no prediction→fact conversion |
| Data leakage between splits | High | template-level split separation and dedup checks |
| Overconfident / miscalibrated model | Medium | calibration and abstention evaluation; explicit `Unknown` |
| Dependency / licence / packaging risk (native ONNX) | Medium | reviewed pinned versions, offline packaging and size checks |
| Desktop/OS coverage | Medium | measured Win/Linux (macOS if approved) before adoption |
| Governance bypass | High | explicit approvals; restriction stays until amended |
| Educational UI presenting fake data | High | only real outputs or labelled versioned fixtures |

## 14. Open maintainer decisions

1. When (and whether) to lift the §4 roadmap restriction, and under which amendment.
2. Confirm Milestone B as the first code slice and its intent ontology boundary.
3. Own the future `TextInterpretation` schema versioning separately from Integration 0.1.
4. Confirm target platforms (Win/Linux; macOS) and the maximum optional footprint.
5. Sequence Milestone B relative to the pending B.4/Phase-B and ADR-0016 qualifications.

## 15. Definition of done and release readiness

A milestone PR is releasable — not automatically released — when: its scope matches this
plan through an approved proposal; required local checks pass and remote CI is observed green
on the exact commit; DE/EN documentation conventions are honoured; limitations are stated
honestly; and no version, tag or release is changed. Merge, publication and any model or
dataset version remain separate maintainer approvals.

## References

[ADR-0017](../../architecture/decisions/ADR-0017-ml-nlp-architecture-proposal.md) ·
[ADR-0014](../../architecture/decisions/ADR-0014-ranking-feature-contract.md) ·
[ADR-0015](../../architecture/decisions/ADR-0015-evaluation-contract-foundations.md) ·
[ADR-0016](../../architecture/decisions/ADR-0016-application-suggestion-boundary.md) ·
[Master v0.4](../../architecture/specifications/master-v0.4.md) ·
[roadmap](README.md) ·
[learning roadmap](development-learning-roadmap.md) ·
[ML learning path](../../../research/ml-from-scratch/README.md) ·
[research explanation (EN)](../../en/research.md) ·
[Forschungserklärung (DE)](../../de/research.md) ·
[progress record](../progress/2026-10-19-neural-intelligence-plan.md)
