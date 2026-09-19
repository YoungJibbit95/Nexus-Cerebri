# Cerebri Lab foundation

Run `cargo run -p cerebri-api` and open http://127.0.0.1:3000/lab.
Load examples/request.json from the repository to plan through the core,
or load an exported PlanningResult JSON. Export preserves the raw structured result.

Planner renders candidate timelines; Trace and Preferences inspect typed output.
Semantics, ML and Dataset are reserved. No log parsing, provider mutation or model inference exists.
This is internal developer/research tooling, not a stable consumer product.
[Development guide](../../docs/en/development.md)

