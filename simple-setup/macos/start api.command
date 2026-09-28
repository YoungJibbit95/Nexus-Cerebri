#!/bin/bash

# Resolve the repository root from simple-setup/macos (two levels up).
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$PROJECT_ROOT" || exit 1

exec cargo run -p cerebri-api
