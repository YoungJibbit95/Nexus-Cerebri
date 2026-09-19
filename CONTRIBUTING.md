# Contributing to Nexus Cerebri

Before implementation, read the Master Specification, Development & Learning Roadmap, Documentation/Git/Release Standard, Testing/Visualization/Research Standard and Coding Agent Boundary.

Use Conventional Commits. Keep changes narrow and testable. Update affected documentation before merge to the default branch. Architectural changes covered by ADR triggers require an ADR.

The Foundation now uses Rust 1.97.0 (rust-toolchain.toml) and shared workspace version 0.1.0, unreleased.
Run the [verification commands](docs/en/development.md) before proposing a change.
Keep Cargo.lock and package-lock.json current. Do not add production dependencies on research.
Public Foundation docs require matching DE/EN pages; ADRs and progress logs may remain in one language.
