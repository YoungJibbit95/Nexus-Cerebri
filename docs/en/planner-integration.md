---
lang: en
---
# Deterministic planner integration

## Simple

Existing daily/weekly series can now block planning slots. A series definition, its
individual occurrences and an editable planning object are different concepts.
The Lab displays actual Rust results, including skipped DST dates, unknown availability,
dependency evidence and the complete candidate ordering key.

## Technical

CPIR `0.2` adds optional `context.temporal` with `horizon`, `coverage`, shared `limits`
and `series`. Each series carries `id`, `state`, `provenance`, `evidence`, and `rule`.
`Existing { revision }` with factual provenance can compile; `Prospective` rejects.
`compile_snapshot` derives a `CompiledContextSnapshot` view of the source ContextSnapshot.
It never inserts provider objects or grants mutation rights. Scope and horizon must agree.
CPIR `0.1` remains supported without temporal input; REST `/v1` and software `0.2.0`
(unreleased) are independent versions. The legacy snapshot is closed supplied context,
not a provider completeness assertion.

The compiler retains the full range plus its half-open horizon intersection. An occurrence
ID hashes series ID and nominal local date/time/timezone, independently of horizon,
clipping and source revision. Duplicate series and object/occurrence identity collisions
reject atomically. Callers must not supply one external occurrence both as an ordinary
object and under a different series identity. Provider reconciliation remains future work.

Earlier/Later DST fold policies intentionally keep the same nominal occurrence identity,
while UTC ranges and resolution evidence differ. Planning reuses the validated immutable
compilation view; compiler errors cannot fall back to legacy planning. Lifecycle validation
still checks and recompiles the current snapshot independently.

All supplied Event/Task objects and compiled occurrences conservatively block overlap,
even outside mutation scope. Incomplete coverage returns unknown complement and
`InsufficientInformation`; it never creates free slots. RequiredBuffer checks full ranges
and requires the buffered candidate to fit within the compiled horizon. Recurrence constraints,
prospective series, series mutation, exceptions and series-level buffers remain unsupported.

DependencyOrder means predecessor.end <= dependent.start. The graph deduplicates and sorts
edges, reports missing references, and uses iterative traversal to find cyclic strongly
connected components with member/edge evidence. Invalid graphs have no claimed order.
The single-event grid search uses fixed predecessors; batch proposals still require full
lifecycle validation. Graph ordering is not an execution order or authorization.

Ranking is lexicographic: preference distance seconds, mutation count, shifted seconds,
start instant, object ID. `ordering_key` exposes every component. `explanation` identifies
the selected preference source. Existing source precedence is preserved; no learning is
implemented. Exhaustive solution => ProvenOptimal **for this grid and objective**;
exhaustive rejection => Complete; budget exhaustion => BestFound.

## Research and limits

At most 32 series, 1,024 occurrences, 36,600 examined dates shared across series, 256 graph
nodes and 1,024 constraints. Combined planner work must stay within the existing 1,000,000
bound with occurrences included. Limits reject whole compilation, never truncate coverage.
The graph uses at most V bounded traversals of V nodes and E edges, O(V(V+E)); no recursion.

[Fixtures and independent expected outcomes](../../examples/planner/manifest.json) cover
feasibility, hard rejection, missing information, scope, dependency order/cycles, recurrence,
duplicate materialization, DST and incomplete coverage. Rust API and live HTTP-to-TypeScript
tests consume these inputs. Negative contract tests reject drift in newly added fields.
Schema generation becomes justified with multiple maintained clients or recurrent drift;
small explicit parity tests suffice for this internal Lab contract today.

The [2026-09-21 verification campaign](../development/progress/2026-09-21-planner-verification.md)
cross-checks seeded small schedules, every three-node directed graph, generated recurrence,
malformed JSON and real API/Lab contracts. This is bounded property testing, not formal
verification. Manual guards now check validation variants and bounded IDs; schema generation
remains deferred because the observed gaps are localized missing guards, not recurring wire
schema changes across multiple clients.

[ADR-0013](../architecture/decisions/ADR-0013-planner-resource-admission.md) adds compact
request size <= 256 KiB, request bytes times candidate budget <= 16 MiB and materialized
occurrence evidence <= 1 MiB. These complement the existing workload limits. Local-date
overflow rejects the candidate rather than panicking. No feature expansion or release is implied.

[ADR-0012](../architecture/decisions/ADR-0012-planner-snapshot-compilation.md) ·
[Temporal reference](temporal.md) · [CPIR reference](cpir.md)
