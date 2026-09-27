<!-- doc: research; lang: en; counterpart: ../de/research.md -->
# From planning to learning

**Research direction: the learning methods described here are not implemented yet.**
The current planner processes structured information and compares appointments using
fixed rules. It does not train a model or learn from your choices.

[Deutsch](../de/research.md) · [Project story](introduction.md) · [Architecture](foundation.md)

## The question behind the project

I wanted to understand how neural networks learn. That included the work of
David E. Rumelhart, Geoffrey E. Hinton and Ronald J. Williams on
[backpropagation in 1986](https://www.nature.com/articles/323533a0). The equations looked
difficult at first. Following the calculations one operation at a time helped me see
what they actually described.

Cerebri is a way to keep learning like that: implement ideas, test assumptions and
understand why a result occurs. Planning gives me concrete questions to work with.
Does an appointment fit? What time is already occupied? Which of several suitable
times is useful? And how would I tell whether a learned method helps?

## Why a planner with fixed rules comes first

The [example](cpir.md) has seven valid start times. Without a preferred time, 10:00 comes
first. The existing rules explain that result, and tests can check whether a change
affects the behavior.

This provides a baseline for later experiments. A model would need to show which task
it improves. It could not treat occupied time as free or make an invalid appointment
acceptable by giving it a high score. Checking the rules remains a separate responsibility.

## Questions that could be investigated later

**Understanding a request.** A method could suggest CPIR fields from a phrase such as
“half an hour in the afternoon.” It would need to identify ambiguity, such as which day
the person means. Today, an application must supply the structured information itself;
the current planner does not interpret that kind of text input.

**Comparing suitable times.** When several appointments are valid, an experiment could
test whether earlier explicit choices reveal useful preferences. A current instruction
would still take priority. The system does not learn these preferences today. Fields
that can name a learned source do not mean a learning method already exists.

**Searching more selectively.** For larger tasks, a model could suggest which options
to check first. Whether this produces better results within the same search budget
would need to be measured. An interrupted search would still have to report only what
it actually checked. Today's search handles one target appointment on a specified time grid.

**Accounting for personal patterns.** Later work could investigate how long-term
preferences differ from a wish in the current situation. Personal feedback should not
automatically become shared training data. The current planner does not collect that feedback.

**Supporting more applications.** Tasks, rooms or shifts are possible later uses. Each
would need appropriate data, rules and tested integrations. Production calendar and
provider integrations do not exist yet; a model alone would not replace them.

## What learning from scratch would mean here

The reserved [learning path](../../research/ml-from-scratch/README.md) calls for building
a small neural network step by step. These lessons do not exist yet. The plan is to
make its calculations traceable:

1. Pass inputs through the network and inspect the intermediate values.
2. Use a loss function to measure how far the output is from the desired result.
3. Use backpropagation to calculate how changes to the parameters affect that loss.
4. Adjust the parameters and observe how the next output changes.
5. Check the calculated derivatives against small numerical changes and an independent
   framework reference.

This would return to the original question: how does a formula become a procedure whose
individual steps I can explain and check? These exercises would stay separate from the
production planner. A successful learning example would not yet show that a model
improves real planning tasks.

## What is already prepared

The [ranking observations](../architecture/decisions/ADR-0014-ranking-feature-contract.md)
describe calculated comparison values and their sources. They do not change the fixed
ordering and are not a trained scoring model.

The [evaluation foundations, Phase A](../architecture/decisions/ADR-0015-evaluation-contract-foundations.md)
define data types and validate the wire format of synthetic records. They do not yet
collect planner observations, compute fingerprints or replay earlier runs. Complete
semantic validation also remains future work.

[cerebri-ml](../../crates/cerebri-ml/src/lib.rs) contains metadata and interfaces for models
and datasets. Those definitions do not provide a training run, a usable model or a
connection to the current planner search.

## How an experiment should be assessed

An experiment needs a specific question, documented data and settings, and a reproducible
comparison with the existing method. Examples used for learning must stay separate from
those used to assess performance. Failures, ambiguous requests and difficult cases belong
in the evaluation too. Added complexity would need to demonstrate a benefit.

Even better suggestions would not grant permission to execute. Facts, required rules,
permissions and the [checks before a change](foundation.md) remain outside a learned
model's authority.

The [current roadmap](../development/roadmap/README.md) records implementation progress.
The [development and learning plan](../development/roadmap/development-learning-roadmap.md)
describes later stages; its date and version targets are not publication dates.
The [research standards](../testing/testing-visualization-research-standard.md) describe
how experiments and comparisons should be documented.
