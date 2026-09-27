<!-- doc: temporal; lang: en; counterpart: ../de/temporal.md -->
# When is a time actually free?

An empty space in a calendar does not always mean that nothing is scheduled there. Some
appointments may simply be missing from the supplied data. Cerebri distinguishes occupied,
free and unknown time so that a plan shows what its availability claim depends on.

## One appointment can start when another ends

If an appointment ends at 10:00 and the next starts at 10:00, they do not overlap.
Cerebri includes the start of an interval and excludes its end. The short notation is
`[start, end)`. Starting the second appointment at 09:59:59 would create a one-second overlap.

Preparation and travel can require additional time. If ten minutes of preparation are
supplied before a 10:00 appointment, that time is occupied too. The calculation uses the
provided buffers; it does not estimate travel by itself.

## Missing appointments leave availability unknown

Suppose the supplied data contains an appointment from 09:00 to 10:00 within a search
window of 09:00–12:00, with no extra buffers. If the caller declares the collection complete
for that window, the remaining 10:00–12:00 can be reported as free. If the collection is
incomplete, that same period stays unknown.

This completeness declaration comes from the calling application. Cerebri does not connect
to a calendar provider to verify it. In planning with temporal source data, incomplete
coverage prevents a claim that a candidate fits into free time.

## Local clock times need a timezone

UTC identifies the instant used for comparisons. A timezone such as `Europe/Berlin`
explains how that instant appears on a local clock. For a repeating appointment, keeping
the local time and timezone matters: a 09:00 appointment can have a different UTC time
after the clocks change.

During a clock change, some local times do not occur or occur twice. For supported daily
and weekly series, the caller must specify how to handle these cases: reject or skip a
missing time; reject or choose the earlier or later occurrence of a repeated time. The
result records those decisions. Ordinary local-time conversion rejects either ambiguity.

## What the current implementation supports

Temporal diagnostics expand daily and weekly rules for a limited period, apply supplied
buffers and report occupied, free and unknown intervals. The planner can also derive
occupied times from existing, fact-sourced series in CPIR 0.2.

Monthly and yearly rules, exceptions and holiday calendars are not implemented. Proposing
or editing a series is also outside the current planner. A hard `RecurrenceRule` constraint
is still unsupported; it is different from supplying an existing series as context.

[Planning explained](planner-integration.md) · [Input format](cpir.md) ·
[Inspect results in the Lab](development.md)

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
The CPIR hard RecurrenceRule still fails closed. Existing, fact-sourced series in
`context.temporal` can now contribute occupancy through the explicit snapshot compiler;
see [ADR-0012](../architecture/decisions/ADR-0012-planner-snapshot-compilation.md).
No external availability is fetched or inferred. Coverage is caller-supplied diagnostic evidence,
not a provider guarantee or permission. Core has no clock, network or filesystem dependency.
[Development](development.md) · [Lab](../../apps/cerebri-lab/README.md) · [Deutsch](../de/temporal.md)
