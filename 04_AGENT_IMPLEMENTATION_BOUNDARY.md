# Nexus Cerebri --- Coding Agent Implementation Boundary

## The future scaffolding agent should implement thoroughly

-   Rust workspace/crate boundaries
-   strong shared types
-   errors/serialization/config
-   logging/tracing
-   CI, format/lint/test infrastructure
-   documentation structure and templates
-   README/CONTRIBUTING/SECURITY/ROADMAP/CHANGELOG foundations
-   ADR/progress-log system
-   GitHub Pages foundation
-   CPIR structures and validation skeleton
-   temporal primitives and established utilities
-   storage ports + in-memory implementations
-   REST skeleton
-   Node-binding skeleton
-   Cerebri Lab shell
-   visualization data contracts
-   model/dataset metadata schemas
-   integration capability interfaces
-   safe fixtures/examples/TODO documentation

## The agent must not silently finish deliberate learning milestones

Unless explicitly instructed later, do not fully implement:

-   neural network from scratch
-   manual backpropagation
-   attention
-   Transformer
-   advanced preference learning
-   personal embeddings
-   ML-guided search
-   advanced repair/optimization algorithms
-   opaque solver replacing the educational planner
-   automatic production calendar mutation

Interfaces, tests, diagrams, fixtures and TODOs for these areas are
welcome.

## Quality requirements

Foundation must compile, be tested, avoid speculative dead abstractions,
document public interfaces, maintain DE/EN documentation where required,
contain no secrets or production personal data, avoid provider lock-in,
keep core independent of DB/HTTP/provider, preserve Planner/Executor
separation and deterministic validation.

Prefer boring, explicit and testable infrastructure over clever
abstractions.


## Decisions the agent must not invent

The following are now fixed for the foundation:
- specification revision and software version are separate;
- initial CPIR schema is 0.1/internal, not a public v1 compatibility promise;
- permissions are not ordinary hard constraints;
- executable lifecycle is ProposedPlan -> ValidatedPlan -> ActionPlan -> AuthorizedActionPlan -> ExecutionResult;
- planner outcome and search assessment are separate;
- bounded PlanningScope is explicit;
- production code never depends on research code;
- ActionLedger/retry/recovery ownership belongs to execution/application layer;
- `PERSONAL_LOCAL` is logical user scope, not necessarily physical device storage;
- Cerebri Lab is initially internal/research tooling.

If implementation pressure reveals a contradiction with these rules, stop at the boundary, document the issue, and request an ADR rather than silently choosing a new architecture.
