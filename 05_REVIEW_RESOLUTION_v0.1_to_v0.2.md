# Nexus Cerebri — Review Resolution: v0.1 -> v0.2

**Date:** 2026-09-19  
**Status:** Accepted planning corrections

This document records how the independent v0.1 review was resolved.

## Critical findings

### CRITICAL-01 — DE/EN structure mismatch
**Accepted.** v0.2 defines parity for canonical public docs and explicitly allows internal logs/research notes/ADR records to remain canonical-language-only. The actual implementation repository must create the DE/EN structure during foundation bootstrap.

### CRITICAL-02 — Required governance files absent
**Accepted as bootstrap gap.** The planning ZIP is not itself the implementation repository. The foundation milestone must create README, CHANGELOG, ROADMAP, CONTRIBUTING, SECURITY, ADR and progress structures before claiming v0.1.0 software readiness.

### CRITICAL-03 — Version source of truth
**Accepted.** v0.2 defines separate canonical authorities for software, API, CPIR, model, dataset and specification revision.

### CRITICAL-04 — Plan lifecycle ambiguity
**Accepted.** Formal lifecycle is now:
`ProposedPlan -> ValidatedPlan -> ActionPlan -> AuthorizedActionPlan -> ExecutionResult`.

### CRITICAL-05 — Permission scope ambiguity
**Accepted.** Permissions are removed from ordinary hard constraints. Planning capability and execution authorization are separate security layers. Simulation may return explicitly non-executable hypothetical results when policy permits.

## Major findings

- README completeness: accepted as foundation bootstrap requirement.
- GitHub-state wording: clarified to default branch/releases.
- Calendar-driven releases: corrected; months provide candidate milestones only.
- CPIR v1 ambiguity: corrected to initial internal schema `0.1`.
- Planner status mixing: split into `PlanningOutcome` and `SearchAssessment`.
- Unknown/missing/uncertain/ambiguous: formal knowledge-state semantics added.
- Bounded scope: concrete dimensions defined.
- Derived facts/inference: provenance and derivation distinction added.
- ADR triggers: explicit trigger list added.
- Research/production boundary: production may never depend on research.
- ActionLedger ownership: executor/application layer owns durable state/recovery.
- Zero violation metric: validator correctness must be independently tested.

## Minor findings

Resolved or clarified:
- specification revision is separate from software/package version;
- ISO 8601 date rule added;
- DE-first fixtures are dataset strategy, not documentation/product-language priority;
- conceptual enum names may change in Rust unless serialized/public contract requires stability;
- external lock defined;
- ML-v0 etc. are learning architecture milestones, not model artifact versions;
- GitHub Pages does not imply a fixed generator/tool yet;
- visualization is required when applicable/useful;
- deterministic and wall-clock search budgets are distinguished.

## Questions resolved

- Default branch should be coherent; releaseability depends on release gates.
- Software version authority: Cargo workspace/package version.
- Spec `0.2` is a review/document revision, not compatibility guarantee.
- CPIR begins as internal versioned IR.
- Permissions affect executable planning capability and are rechecked at execution.
- Hypothetical non-executable suggestions are allowed only in analysis/simulation policy.
- OPTIMAL means proven optimum over the explicitly defined complete search space/objective.
- Tie-breaking is deterministic.
- v0.x ConflictSet is sound/explainable, not promised minimal.
- Preferences become policies only explicitly, never by learning alone.
- Policy is a typed deterministic layer.
- Mutation scope is explicit; visibility never implies mutability.
- Unknown external participant availability remains UNKNOWN; blocking depends on operation/policy.
- personal_pattern should minimize raw personal data.
- PERSONAL_LOCAL is logical personal scope, not necessarily on-device storage.
- progress logs are required for meaningful work/decisions, not passive reading.
- DE/EN CI verifies structural parity; semantic parity requires review.
- three-level explanation is mandatory for central public/didactic concepts, not every internal note.
- Cerebri Lab is initially internal research/developer tooling.
- specification revision and software v0.1.0 are separate namespaces.
