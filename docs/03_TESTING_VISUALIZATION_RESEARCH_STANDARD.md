# Nexus Cerebri --- Testing, Visualization & Research Standard

## Testing layers

1.  Unit tests
2.  Property-based tests
3.  Fuzz tests
4.  Scenario/golden tests
5.  Integration/adapter contract tests
6.  ML evaluation/regression tests

## Safety invariants

-   `ValidatedPlan => HardConstraintViolations == 0`
-   `ExecutableAction => PermissionGranted`
-   `Execution => ValidatedActionPlan`
-   `Inference cannot replace Fact`
-   `LearnedPreference cannot override ExplicitConstraint`
-   `StalePlan => Revalidation`
-   `Unknown != Assumed`
-   `Planner != Executor`

Invariant violations are critical defects.

## Golden scenarios

Fixed medical appointment vs flexible activity; deadline pressure;
external lock; participants/resources; timezone/DST; recurrence;
dependency chain/cycle; ambiguous time; no solution; repair search;
stale revision; unsupported adapter capability; permission denied.

## From-scratch ML verification

Compare manual forward pass to reference, manual gradient to numerical
gradient, and manual gradient to autograd. Keep deterministic seeded
fixtures.

`dL/dw ~= [L(w+eps) - L(w-eps)] / (2*eps)`

## Dataset quality

Measure class balance, duplicates/template leakage, vocabulary coverage,
linguistic-family split, DE/EN distribution, hard negatives, ambiguity
coverage, missing labels, dataset version/checksum.

Do not claim generalization from near-duplicate train/test templates.

## Visualization requirements

Graphs must be readable, titled, labeled with units, interactive where
useful, backed by inspectable raw data, accessible, usable in light/dark
mode and not dependent on color alone.

Planner: timeline, free/busy, candidates, rejected candidates,
constraints, score decomposition, repair path, trace.

ML: train/validation loss, F1/accuracy, confusion matrix, calibration,
gradients, learning rate, latency, parameter/model comparison.

Dataset: labels, language, splits, lengths, hard negatives, versions.

Preferences: evidence/weights over time, ranking before/after,
explicit/session/personal/global contribution.

## Explanation modes

SIMPLE, TECHNICAL, RESEARCH.

## Experiment record

Store run ID, date, hypothesis, Git commit, software/dataset/model
versions, checksum, architecture, hyperparameters, seed, environment,
metrics, plots, result, interpretation, limitations and next step.

## Preserve baselines

Rules -\> Mean Pooling -\> MLP -\> Sequence Encoder -\> Attention -\>
Transformer should remain comparable. Complexity must earn its place
through measured benefit.
