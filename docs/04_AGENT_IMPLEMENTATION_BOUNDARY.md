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
