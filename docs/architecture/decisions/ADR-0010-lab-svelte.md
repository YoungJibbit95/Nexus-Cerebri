# ADR-0010: Svelte Lab as a typed development client
Date: 2026-09-20
Status: Accepted; implements the Lab boundary from ADR-0006.

## Decision
Use Svelte 5, TypeScript and Vite in apps/cerebri-lab. Keep its package/lock/configuration
inside the app. Build static assets under dist with /lab/ base. Axum serves only that directory;
Vite proxies development REST requests to the loopback Rust app. No CDN runtime dependencies.
The API still builds without Node; build the frontend separately before opening /lab/.
Missing build assets return HTTP 404, never a misleading embedded source page.

Show planner results, costs, traces, temporal diagnostics and raw output from the core.
ML/training/dataset views are explicit future interfaces. The console may inspect actual
responses but cannot simulate successful model inference or execute arbitrary commands.
Maintain keyboard labels, visible focus, legibility and responsive layouts alongside glass,
rounded surfaces and restrained glow. A theme changes presentation only.

## Alternatives and consequences
Static HTML constrained inspection interactions; a second JavaScript planner would violate
the shared-core requirement. Svelte keeps UI components independent of Rust domain logic.
The Node process bridge remains provisional and unchanged; Vite is a separate developer tool.
Production hosting/authentication and desktop packaging are outside this milestone.

## Verification
Typecheck, presentation/transport tests, production build, dependency audit and browser checks.
[Lab README](../../../apps/cerebri-lab/README.md) · [Temporal reference](../../en/temporal.md)
