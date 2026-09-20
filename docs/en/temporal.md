<!-- doc: temporal; lang: en; counterpart: ../de/temporal.md -->
# Temporal Core
## Simple
The core turns explicit local-calendar rules into actual UTC intervals. A spring clock gap
may be skipped only when requested; an autumn repeated time requires a stated choice.
Busy intervals include preparation/travel buffers. Only a complete collection proves that
its remaining horizon is free; otherwise that time remains unknown.

## Technical
POST /v1/temporal accepts [the synthetic fixture](../../examples/temporal-request.json).
Required fields are horizon, busy, coverage, recurrences and limits.
Every busy input has a range and before_seconds/after_seconds/travel_seconds buffers.
A recurrence has start_date, local_time, timezone (IANA), duration (elapsed seconds),
pattern, optional until/count, and required gap_policy/fold_policy.
DAILY requires positive every. WEEKLY also requires nonempty unique weekdays (Mon..Sun).
until is inclusive; count counts nominal dates, including skipped DST gaps. Weekly periods
start Monday; the start_date excludes earlier weekdays in the first week.
All intervals use [start,end). A start at horizon.end or end at horizon.start is invisible.
The report preserves full range and horizon-clipped visible_range separately.

The response is {status: Complete, data: {availability, expansions}} or
{status: Rejected, data: typed error}. A rejected result contains no partial availability.
availability includes busy/free/unknown plus input buffer/clipping traces. expansions contain
nominal sequence/date, chosen resolution, skipped dates and examined_dates.
The occurrence and examined-date budgets apply across all rules, not independently.
Maximums: 32 rules, 10,000 occurrences, 36,600 examined dates, 10,000 combined busy inputs.
The two-day timezone transition margin also counts against the date budget.
Malformed JSON/types are HTTP 400/422; body sizes above 256 KiB are HTTP 413.
Domain rejection is a typed HTTP 200 report, distinct from transport failure.

## Research depth and limits
[ADR-0009](../architecture/decisions/ADR-0009-bounded-temporal-diagnostics.md) records the
grammar and completeness decision. Tests compare weekly ordinals with an independent
day-by-day oracle, exhaust interval partitions and include Berlin, Lord Howe and Samoa.
A seeded malformed-byte corpus is a repeatable smoke test, not continuous coverage-guided fuzzing.
The IANA database comes from the pinned Cargo.lock dependency graph; dependency updates can
change historical/future zone results and must rerun these goldens.

This is not a full RFC 5545 engine: monthly/yearly rules, exceptions and holidays are absent.
CPIR hard RecurrenceRule still fails closed; the planner does not automatically expand series.
No external availability is fetched or inferred. Coverage is caller-supplied diagnostic evidence,
not a provider guarantee or permission. Core has no clock, network or filesystem dependency.
[Development](development.md) · [Lab](../../apps/cerebri-lab/README.md)
