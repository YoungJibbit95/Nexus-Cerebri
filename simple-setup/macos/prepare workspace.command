#!/bin/bash

# Resolve the repository root from simple-setup/macos (two levels up).
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$PROJECT_ROOT" || exit 1

cargo build --workspace --locked
cargo test --workspace --locked
cargo fmt --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

npm --prefix apps/cerebri-lab run check
npm --prefix apps/cerebri-lab test
