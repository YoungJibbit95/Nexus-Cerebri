# Understand and try Nexus Cerebri

Nexus Cerebri is a learning project about making planning decisions understandable and
checkable. Its current implementation finds possible times from structured data and
explicit rules. Neural learning is a research direction, not part of today's planner.

## Start with the idea, then follow an example

1. [Why this project exists](introduction.md): the learning motivation, the influence of
   Rumelhart, Hinton and Williams, and a first appointment example.
2. [What the planner needs](cpir.md): duration, occupied times, rules and permissions,
   followed by their fields in the CPIR input format.
3. [How possible times are compared](planner-integration.md): which options are rejected,
   how preferences affect the order and what a search result actually establishes.
4. [When a time is free](temporal.md): adjacent appointments, incomplete information,
   timezones and repeating appointments.
5. [What must happen before a change](safety.md): why a proposal needs further validation
   and permission before execution.

## Go further

- [Connect an application](application-integration.md) describes the bounded suggestion
  contract, Node/REST entry points and the next Nexus consumer milestone.

- [Architecture](foundation.md) explains which Rust package owns each calculation and
  how the API, Lab and website use it.
- [Try the Lab and develop locally](development.md) explains how to inspect results and
  where to start with the API and verification tools.
- [Learning and research](research.md) connects the project's origin with possible later
  experiments and separates them from existing implementation.
- [AI-assisted development](ai-assisted-development.md) describes human responsibility
  and the checks required for contributions.

## Read the status correctly

Software 0.2.0 is **unreleased**. Specification 0.4 and CPIR 0.2 (with legacy 0.1 support)
identify separate contracts; their numbers do not mean a product release is available.
Use the [current roadmap](../development/roadmap/README.md) for implementation status
and the [Changelog](../../CHANGELOG.md) for release-publication status.

These public guides have German counterparts. Technical specifications, architecture
decisions and dated progress logs are separate sources and are not all bilingual. Dated
logs explain work at that point in time; they do not replace the current contracts.

[Deutsch](../de/README.md) · [Project overview](../../README.md)
