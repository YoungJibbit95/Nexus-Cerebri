# Nexus Cerebri --- Development & Learning Roadmap

**Baseline:** 2026-09-19\
**Initial planning horizon:** 2026-10-01 through 2027-09-30

Dates are planning targets. Re-estimate the next 4--8 weeks at each
monthly review. Reserve roughly 20--30% of development capacity for
learning, debugging, refactoring, documentation and unexpected work.

## October 2026 --- Foundation --- target v0.1.0

Learn: Rust workspace architecture, errors, serialization, CI/test
basics.

Build: workspace/crates, IDs/version types, error model, CPIR skeleton,
config/tracing, in-memory ports, API/Node placeholders, Cerebri Lab
shell.

Docs: README, CONTRIBUTING, SECURITY, ROADMAP, CHANGELOG, ADR template,
DE/EN structure, progress log, GitHub Pages skeleton.

Verify: CI build/test/lint/format and first property-test harness.

## November 2026 --- Temporal Core --- target v0.2.0

Learn: interval algebra, timezone/DST behavior, recurrence boundaries.

Build: TimeRange, Duration, ZonedDateTime, relations,
overlap/intersection, free/busy, PlanningHorizon, granularity, buffers,
bounded recurrence.

Visualize: interactive timeline, free/busy ranges, DST/timezone debug.

Verify: interval properties, timezone/DST golden cases, malformed input
fuzzing.

## December 2026 --- Deterministic Planner --- target v0.3.0

Learn: constraint satisfaction, dependency graphs, objective functions.

Build: hard constraints, cycle detection, candidate generation, weighted
scoring, ranking, ConflictSet, explanations, SearchBudget v0.

Visualize: candidates, rejection reasons, constraint status, score
decomposition.

Verify: hard-constraint invariants, randomized schedules, golden
scenarios.

## January 2027 --- CPIR/API/Integration --- target v0.4.0

Learn: schema evolution, API contracts, idempotency, optimistic
concurrency.

Build: CPIR v1, operations/scope, ProposedPlan/ActionPlan, action
states, idempotency, stale checks, Rust API, REST skeleton, usable Node
binding sample, adapter capabilities.

Verify: serialization round trips, schema compatibility, adapter
contract tests, stale-plan scenarios.

## February 2027 --- Semantic Baseline --- target v0.5.0

Learn: features, vectors, uncertainty, classification/calibration
intuition.

Build: semantic vector, provenance/confidence, rule/signal extractor,
source layers, ambiguity model, explanation modes, DE-first fixtures
with EN equivalents.

Visualize: semantic bars, evidence tree, confidence, signal
contributions.

Verify: hard negatives, negation/context, bilingual fixtures, ambiguity.

## March 2027 --- Data & ML Foundations --- target v0.6.0

Learn: vectors/matrices, logits/probability, splits/leakage,
precision/recall/F1.

Build: dataset schema/versioning, synthetic generator, linguistic
variation, challenge set, experiment metadata, baseline rule classifier.

Visualize: class/split/language distributions, confusion matrix, example
browser.

## April 2027 --- Neural Network From Scratch --- target v0.7.0

Learn deeply: Linear, activations, Softmax, Cross-Entropy, derivatives,
Gradient Descent, Backpropagation.

Build educationally: required matrix ops, embeddings, mean pooling,
Linear, ReLU, Softmax, loss, SGD, backward pass, numerical gradient
checks.

Visualize: activations, logits, probabilities, loss, gradients, weight
updates.

Verify: manual vs numerical gradients and reference framework.

## May 2027 --- Production Intent Intelligence --- target v0.8.0

Learn: framework/autograd, hyperparameters, overfitting, calibration.

Build: framework model, model registry, immutable artifacts,
TrainingRun, production inference interface, UNKNOWN/clarification
policy, regression suite.

Compare rules vs from-scratch vs framework.

## June 2027 --- Preference Learning --- target v0.9.0

Learn: ranking, pairwise preferences, regularization, recency/decay.

Build: FeedbackEvent, PreferenceEvidence, interpretable
PreferenceProfile, pairwise records, simple learning-to-rank experiment,
global/personal/session separation.

Visualize: evidence and weights over time, before/after rankings, source
contribution.

## July 2027 --- Repair Planner --- target v0.10.0

Learn: heuristic search and complexity.

Build: Tier-2 repair candidates, minimal-disruption repair cost, bounded
multi-event replanning, dependency propagation, BEST_FOUND/OPTIMAL
semantics, benchmarks.

Visualize: repair paths, budget, moved-event cost, best-so-far
progression.

## August 2027 --- Attention Research --- target v0.11.0

Learn: sequence representations, Q/K/V, scaled dot-product and
multi-head attention.

Experiment: sequence-aware baseline and attention; compare against mean
pooling, especially word order/negation.

Visualize: token representations, attention matrices, failure
comparisons.

## September 2027 --- Small Transformer Research --- target v0.12.0

Learn: Transformer encoder, residuals, normalization, multi-task heads.

Experiment: small Transformer, intent head, additional semantic/category
heads only where data supports them. Measure complexity vs benefit and
document whether adoption is justified.

## Monthly review

At month end: 1. compare completed vs planned work; 2. record learning
outcomes; 3. review tests/metrics; 4. verify DE/EN documentation parity;
5. review ADRs and technical debt; 6. re-estimate next two months; 7.
update README/ROADMAP; 8. release only if Definition of Done is
satisfied.
