# Contributing to Nexus Cerebri

Before implementation, read the Master Specification, Development & Learning Roadmap, Documentation/Git/Release Standard, Testing/Visualization/Research Standard and Coding Agent Boundary.

Use Conventional Commits. Keep changes narrow and testable. Update affected documentation before merge to the default branch. Architectural changes covered by ADR triggers require an ADR.

The workspace uses Rust 1.97.0 (rust-toolchain.toml) and shared software version 0.2.0.
Run the [verification commands](docs/en/development.md) before proposing a change.
Keep Cargo.lock and package-lock.json current. Do not add production dependencies on research.
Public Foundation docs require matching DE/EN pages; ADRs and progress logs may remain in one language.

Nexus Cerebri is MIT-licensed and developed with AI assistance. Disclose material assistance,
review generated code and preserve the research learning path. All contributions meet identical
test, safety, review, documentation and CI gates. Read the
[AI development policy](docs/en/ai-assisted-development.md) and [documentation map](docs/README.md).
