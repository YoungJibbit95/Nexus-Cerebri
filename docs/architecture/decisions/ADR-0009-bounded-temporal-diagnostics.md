# ADR-0009: Bounded recurrence and completeness-aware diagnostics
Date: 2026-09-20
Status: Accepted; extends ADR-0003 and ADR-0006 without changing CPIR 0.1.

## Decision
Use a validated typed daily/weekly calendar rule, not an RFC 5545 string parser.
Daily every=N counts local calendar days; weekly every=N uses Monday weeks anchored to the
start date's week, with unique selected weekdays and no date before start_date.
until is an inclusive local date. count counts nominal matching dates, including skipped
gaps. Duration is positive elapsed seconds, not wall-clock end time.

Gap policy must explicitly reject or skip. Fold policy must explicitly reject, choose the
earlier instant or choose the later instant. Trace all relevant skipped dates and resolutions.
Keep full occurrence identity/range and separately expose its horizon-clipped visible_range.
A series may start before the horizon and overlap it. Touching endpoints do not overlap.
General ZonedDateTime::from_local continues to reject ambiguity without choosing a policy.

Seek near the horizon using elapsed-duration lookback and a two-day zone transition margin;
do not scan from a distant recurrence anchor. Check all arithmetic. Deterministic limits
bound examined dates and emitted occurrences, shared across the request. Limits are at most
36,600 examined dates, 10,000 occurrences, 32 rules and 10,000 combined busy inputs.
Exhaustion or overflow rejects the whole diagnostic: no truncated result claims completeness.

Apply before/after/travel-before buffers, clip busy intervals, sort, union, then subtract.
Touching busy intervals have a contiguous union but are still non-overlapping intervals.
Coverage::Complete is an explicit caller assertion about the collection. With Incomplete
coverage, the complement is unknown, never free. This does not replace per-field knowledge
states, infer provider coverage or provide authorization.

## Boundaries
Core directly reexports temporal diagnostics; API /v1/temporal only delegates.
Lab renders typed reports and makes no domain decisions. CPIR RecurrenceRule constraints
remain unsupported/fail-closed: diagnostic expansion is not automatic planner integration.
The next planner milestone must define the snapshot/compiler connection before using it.

## Alternatives and consequences
Unbounded RRULE support, implicit DST selection and guessed free time would hide assumptions.
The small supported grammar is auditable but omits monthly/yearly rules, exceptions,
holiday calendars and natural-language recurrence. Bundled timezone data changes only through
reviewed dependency updates. Large public workloads need application worker isolation later.

## Verification
Exhaustive interval partition/subtraction checks; weekly ordinal comparison with an independent
day-by-day oracle; Berlin DST, Lord Howe half-hour fold and Samoa date-line goldens;
overflow, malformed input, shared budgets and REST/core parity.
[Temporal reference](../../en/temporal.md) · [ADR-0003](ADR-0003-temporal.md)
