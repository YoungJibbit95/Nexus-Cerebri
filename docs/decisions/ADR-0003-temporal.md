# ADR-0003: Chrono, IANA zones and validated half-open intervals

Date: 2026-09-19
Status: Accepted (Foundation implementation within specification 0.4)

## Context

Instant identity and DST behavior must not rely on manually maintained timezone arithmetic.

## Decision

Use chrono without clock features and chrono-tz with the bundled IANA database. Persist UTC instants plus an explicit original timezone. TimeRange has private bounds and validating serde conversion. Duration is positive integral seconds. Local resolution rejects ambiguous/nonexistent times; callers must resolve them before planning. Recurrence remains a bounded expansion trait.

## Alternatives

A custom zone engine is unsafe; silently selecting either DST fold loses meaning; allowing naive datetimes inside TimeRange weakens identity.

## Consequences

Timezone database updates arrive via dependency updates. No hidden system clock. ExplicitDate currently constrains the local start date. Subsecond instants are representable; candidate steps/durations use integral seconds.

## Migration / Compatibility

Temporal serde validation is part of CPIR 0.1. No change to the Master's half-open convention.

## References

Master section 6; Testing standard.

[Master](../../00_MASTER_SPECIFICATION_v0.4.md) · [Foundation reference](../en/foundation.md)

