# ADR-0017: Proposed local NLP and ML architecture for Cerebri

Date: 2026-10-19
Status: **Proposed — design discussion only; not accepted or authorized for implementation**

## Context and current state

Cerebri is a Rust-first, application-independent temporal planning engine. The target user journey is natural-language text in a host application -> a bounded, validated planning suggestion -> an application-specific proposed action. Text understanding, local model execution and neural training are **not currently implemented**.

Verified existing seams on `main`:
- `cerebri-ml` defines `Inference`, `ModelRegistry`, `ModelMetadata`, `DatasetMetadata`, `TrainingRunMetadata`, `DataScope`, and checksum-addressed artifact references; these are interfaces/metadata, not a model runtime.
- `cerebri-semantics` defines a semantic evidence vector and pure temporal derivation; it does not extract arbitrary natural language.
- `cerebri-preferences` supplies deterministic preference precedence and ranking observations; no learned model changes candidate order.
- The deterministic planner consumes typed CPIR 0.2. ADR-0014 defines ranking observations, and ADR-0015 provides evaluation foundations; Phase B.4/full Phase B still await independent qualification and Phase C observation/replay is unimplemented.
- ADR-0016 introduces a proposed, implemented Integration 0.1 `single_event_suggestion` profile with unchanged planner responses and **no execution**; independent Architecture/Security qualification is pending.
- `research/ml-from-scratch` and `models/`/`datasets/` are reserved; no trained model, production corpus or ML research lesson is present.

This proposal refines implementation choices under Master v0.4 sections 2, 3, 12–16, 27–31 and is **not** an amendment to any accepted authority. Existing roadmap gates apply. In particular, neither this document nor a merged documentation PR is permission to implement, deploy, learn from personal data, or alter authority boundaries.

## Goals and non-goals

**Goals**
1. Offer a future opt-in, local and application-independent text-to-*draft intent* pipeline.
2. Preserve evidence, ambiguity, source provenance and user control from raw text to validated CPIR and a **suggestion-only** integration.
3. Allow swapping deterministic baselines, neural backends and models without changing planner feasibility, authorization, deterministic ranking, or execution.
4. Produce reproducible experiments with synthetic data first and independently assess whether ML provides measurable benefit.
5. Avoid a Python runtime, cloud-model dependency, silent model download or mutable third-party model dependency in the deployed default path.

**Not in scope now:** a model implementation; generalized LLM/chat generation; autonomous execution; a new public REST endpoint; a new CPIR version; a new production crate; model downloads; calendar-provider adapters; product rollout; new telemetry, data collection or personal-data training; learned ranking or search guidance; release/tag/version changes.

## Proposed system decomposition

```text
Host UI / chat input (untrusted text)
            |
            v
Host supplies locale, reference clock, trusted timezone, access/scope
            |
            v
Text limits + normalization + tokenization
            |
            +---- deterministic parsing / rule baseline
            |
            +---- optional versioned neural inference (local only)
            |
            v
IntentDraft: candidate intent, typed slot proposals, source spans,
per-field confidence/abstention, missing/ambiguous fields
            |
            v
Deterministic resolver and validation
   - exact temporal interpretation with reference time and IANA zone
   - contradictions, negation, missing fields, DST gap/fold
   - clarification instead of fabrication
            |
            v
Trusted host adapter constructs CPIR 0.2 using authorized context
   - verifies capabilities / fresh provider-derived facts
   - never upgrades model inference to independent fact or permission
            |
            v
Existing core Integration 0.1 (single_event_suggestion)
            |
            v
Unchanged Rust planner -> PlanningResult -> host preview
            |
            v
Explicit user/app decision; any future execution uses separate preflight
```

The model **never emits an executable API call**. It proposes interpreted structured values with uncertainty; the trusted app translates validated proposals into a **planning request**. Any subsequent action API is application-owned, subject to a separate authority and permission boundary. Incomplete free/busy coverage is unknown, never free.

## Draft typed boundary (illustrative, not an approved wire contract)

The initial domain is `find_time_for_single_event`, not unrestricted scheduling. Proposed representations:

- `TextInterpretationRequest`: bounded raw text; declared language; host-supplied reference instant and IANA timezone; supported intent profile; caller correlation ID; **no embedded provider credentials**. Untrusted text cannot select capabilities.
- `IntentDraft`: independent interpretation schema/version; intent candidate or `Unknown`; slot proposals for date/time/window/duration/person/activity/resource; exact source spans; per-slot provenance, uncertainty and explicit absent/contradictory/ambiguous states.
- `InterpretationOutcome`: `Draft`, `ClarificationNeeded`, `UnsupportedIntent`, or `Rejected` (including limits/incompatible model). A numerical confidence score is **not** authorization.
- `ValidatedIntent`: only constructed by deterministic resolution and host validation, with opaque tokens for referenced entities as needed; it is not automatically a valid CPIR PlanningRequest.
- `ModelIdentity`: model ID/version, artifact digest, tokenizer/config digests, feature/schema version, runtime/backend identity, training provenance and declared compatible profiles.

Actual names, field presence, enums, serialization schema, error taxonomy, sizes and budget ceilings need a separate implementation ADR and conformance fixtures. Do not add a generic `impl From<IntentDraft> for PlanningRequest` that could silently promote predictions to facts.

## Default technology shortlist (evaluation decisions, not new dependencies)

| Responsibility | Proposed baseline | Alternatives and gate |
| --- | --- | --- |
| Deterministic temporal interpretation | Existing Rust `cerebri-temporal` + bounded rules/parser | No neural authority for dates, DST resolution or hard constraints |
| Tokenizer | Hugging Face `tokenizers` (Rust) | Pin exact tokenizer.json, normalization and version in artifact manifest |
| Supervised research/training | Python + PyTorch in **offline, separate research tooling** | Avoid shipping Python with production Rust; deterministic seeds and independent evaluation |
| Initial trainable architecture | Token embeddings -> mean pooling -> small MLP intent head | Master v0.4 ML-v0; train from scratch only on bounded synthetic/curated corpus |
| First production-candidate local inference | ONNX Runtime via Rust `ort` behind an optional backend/feature | **Conditional** on model export parity, audited native packaging, offline startup, architecture/OS compatibility, size and CPU latency measurements |
| Rust-native training research | Burn, isolated from production workspace until qualified | Evaluate learning ergonomics, autograd, gradient checks and packaging; do not require for first shipping backend |
| Rust-native Transformer fallback | Hugging Face Candle | Evaluate when native models or ONNX export/runtime prove unsuitable; avoid two mandatory production inference stacks |
| Experiment tracking | Versioned files/manifests + hashes; existing ModelRegistry/TrainingRunMetadata ports | No cloud tracking dependency or raw personal content in default telemetry |
| Default serving model | In-process local Rust library/host-controlled process | Existing bounded Node bridge is transport reference; native/process isolation requires separate implementation review |

No versions are pinned by this *architecture shortlist*. An implementation slice must choose reviewed fixed versions/licenses/build configurations, lock dependencies, and test actual platform artifacts. A successful ONNX export by itself is not evidence of operator parity or acceptable inference performance.

## Responsibilities and dependency isolation

- `cerebri-ml`: model artifact/registry, backend-agnostic inference port and narrow prediction metadata; do not make it a domain/planning authority. Extend ports only when the first bounded integration slice justifies concrete types. Do not add Tensor/ONNX/PyTorch directly to existing core by default.
- `cerebri-semantics`: interpretation evidence and deterministic semantic signals, preserving separation between `Fact`, `Inference`, `Preference` and `Constraint`.
- `cerebri-temporal`: trusted time calculations, range and DST semantics.
- `cerebri-core`/`cerebri-planner`: continue receiving explicit typed CPIR, performing all admission/feasibility/order/proof checks; no model or Python dependency.
- `cerebri-integrations`: existing independent execution preflight/authority remains untouched.
- Host/application: UI, provider data, permissions, identity mapping, opt-in setting, trusted current time/zone, entity resolution, clarification and final preview. An embedded/local runtime is not a grant of authority.
- `research/ml-from-scratch`: separate pedagogical implementations of tensors, autograd checks and small nets; forbidden as production dependency.
- Update `scripts/check-repository.mjs` dependency allowlist only through a separately approved slice; do not evade it.

## Recommended staged gates

**Gate 0 — now, docs only**: review this proposal, decide text schema version ownership, privacy/threat model, target desktop platforms, supported DE/EN language set, and ONNX/Burn research split. No production code or new dependency.

**Gate 1 — deterministic text interpretation baseline (later, explicit authorization)**: bounded single-event intent ontology, hand-written text-to-draft rules (exact date/duration/negation, clarifications), fully synthetic DE/EN challenge fixtures, typed negative tests, host-to-CPIR adapter in test-only mode. Keep Integration 0.1 unchanged. This baseline establishes an honest denominator.

**Gate 2 — offline neural experiment**: synthetic/curated datasets with provenance, disjoint templates/splits; PyTorch ML-v0 intent classifier (embedding + pooling + MLP). Separate invalid/ambiguous examples. Compare with Gate 1 under identical splits and abstention policy. Keep all research out of production dependencies.

**Gate 3 — local runtime feasibility spike**: export an approved classifier to ONNX, test numerical parity and tokenizer parity in `ort` under CPU-only Win/Linux at minimum (macOS if a supported target). Measure cold start, resident memory, model package size, p50/p95 latency and failures; verify truly offline startup, no raw-input logging, checksum enforcement and bounded input. Reject adoption if benefits/gates fail.

**Gate 4 — shadow interpretation (new review required)**: optional adapter within declared app profile; return structured draft and clarification, *never* invented facts, execution or silent fallback. Compare exact field values, calibration/abstention and host-independent replay on synthetic fixtures. Version separately from CPIR and Integration 0.1.

**Gate 5 — learned candidate ranking/search (not included)**: only after independently qualified Phase B, the necessary Phase C lifecycle/replay/evaluation work and a distinct math/security ADR. Hard filters and proof semantics remain unchanged; current ADR-0014 deterministic ordering is never silently replaced.

Gate numbers describe dependencies, **not calendar deadlines or release versions**. The current development roadmap's learning milestones remain target-based.

## Evaluation design and failure requirements

- Establish rules-only accuracy before fitting a model; measure **intent macro-F1**, per-slot precision/recall/F1 and exact-span/exact-value match, temporal normalization accuracy, negation/contradiction errors, calibration/reliability and appropriately abstained ambiguous inputs.
- Use separate training/validation/test sets, held-out linguistic templates, explicit DE/EN and difficult negative fixtures; deduplicate near-identical renderings across splits.
- Compare ML to baseline on exactly the same fixtures and operating budgets; adoption threshold must be specified **before** final evaluation. No arbitrary latency/F1 target is asserted by this proposal.
- Deterministic invariant: validated hard-constraint violations = **0** on adversarial and independent scenario tests, with independent negative validator tests to detect a broken checker. Model improvement never waives this.
- Failure tests: no timezone/reference clock, invalid or conflicting dates, DST gaps/folds, unknown duration, incomplete availability, cancelled model call, corrupt/incompatible weights/tokenizer, oversized text/tokens, hostile instructions, malformed output, missing capabilities, unavailable CPU backend and model-version mismatch.
- Record dataset scope, digest, model and tokenizer digests, training seed/environment, code revision, metrics, failures, platform/runtime, and reproducible comparison artifacts without recording private request text.

## Privacy, security, packaging

Local-first means **no mandatory network after trusted installation**, no hidden inference telemetry, explicit opt-in for any personalized feedback and no automatic elevation of `PERSONAL_LOCAL` material into globally shared datasets. Separate trusted reference data from user prose. Training data requires license and provenance review; model weights require source, license, checksum and artifact resolver checks. Artifact digest metadata alone does not authenticate bytes until a resolver actually verifies them.

Reject stale interpretation against a changed context. In the absence of trusted input, request clarification. Redact errors; limit CPU/time/memory concurrency as deployment decisions; never load model paths or executables from untrusted text. All suggestion/execution boundaries from ADR-0005 and ADR-0016 remain unchanged.

## Open decisions for maintainer review

1. Confirm initial product profile: **single-event create/find-slot interpretation only**; defer move/cancel/recurrence.
2. Confirm language scope: DE-first plus EN challenge fixtures, or DE+EN fully first-class immediately.
3. Approve **PyTorch experiments + optional ONNX Rust runtime** as the first reference workflow, with Burn reserved for Rust teaching and Candle as comparison rather than mandatory deployment.
4. Decide initial runtime targets (Windows/Linux/macOS; CPU-only default), maximum acceptable optional binary/model footprint and whether an embedded library or isolated local process is preferable.
5. Choose future `TextInterpretation` schema ownership and versioning after testing the shape; avoid prematurely coupling it to Integration 0.1.
6. Define the implementation trigger once the outstanding independent B.4/Phase-B and ADR-0016 reviews are resolved to the extent required by the intended slice; honor the roadmap's `Do not begin ML` boundary until expressly revised by the maintainer.

## Alternatives considered

- **LLM directly generates app actions**: rejected as initial approach; unrestricted free-form outputs cannot become planning or execution authority.
- **Pure Burn in production immediately**: attractive for Rust-only learning, but unnecessarily couples research framework maturity and app distribution before a representative model and deployment benchmark exist.
- **Train a new Transformer from scratch first**: rejected initially due to data and complexity cost without a measured intent baseline.
- **Pure ONNX model with no deterministic parser**: rejected; a trained classifier cannot authoritatively interpret exact instants, provider coverage or permissions.
- **Add a new `cerebri-nlp` workspace crate now**: deferred until real code and dependency direction justify the boundary.

## Compatibility, rollback and acceptance

This proposal changes **documentation only**. It neither amends Master v0.4 nor modifies any accepted ADR, CPIR 0.2, Integration 0.1, ranking, planner or release authority. A rejected proposal is deleted without migration. Accepting a future implementation will require its own bounded request, explicit schema definition, dependency review, conformance/tests, security qualification, accurate DE/EN documentation and release gate.

References: [Master v0.4](../specifications/master-v0.4.md), [ADR-0001](ADR-0001-workspace.md), [ADR-0005](ADR-0005-execution.md), [ADR-0012](ADR-0012-planner-snapshot-compilation.md), [ADR-0014](ADR-0014-ranking-feature-contract.md), [ADR-0015](ADR-0015-evaluation-contract-foundations.md), [ADR-0016](ADR-0016-application-suggestion-boundary.md), [Roadmap](../../development/roadmap/README.md), [ML learning path](../../../research/ml-from-scratch/README.md), [Burn](https://burn.dev/), [PyTorch](https://pytorch.org/), [Candle](https://github.com/huggingface/candle), [Tokenizers](https://github.com/huggingface/tokenizers), [ONNX Runtime](https://onnxruntime.ai/).
