<!-- doc: introduction; lang: en; counterpart: ../de/introduction.md -->
# How Nexus Cerebri plans a time

[Deutsch](../de/introduction.md)

## What Cerebri is about

Nexus Cerebri is an open-source learning project exploring how software can build plans
and make its results understandable. The current example is an appointment: you supply
the duration, the time window and the rules to follow. Cerebri checks possible times,
rules out conflicts and returns a proposal. You can inspect why a time was rejected
and why another came first.

The current planner follows fixed rules: the same input produces the same result. This
is what **deterministic** means here. It currently plans one target appointment at a time.
Finding a time does not book it in a calendar.

## Why I started Cerebri

I wanted to understand how neural networks learn. The mathematics behind work such as
the 1986 paper
[Learning representations by back-propagating errors](https://www.nature.com/articles/323533a0)
by David E. Rumelhart, Geoffrey E. Hinton and Ronald J. Williams helped drawing me in.
At first, the equations looked like a language I could barely read.

That changed when I followed the calculations one operation at a time. A formula that
seemed impenetrable as a whole could be broken into manageable steps. The symbols
described a process I could trace. I wanted to keep developing that understanding.

Cerebri grew out of that curiosity. I wanted to implement ideas, test assumptions and
see how the parts worked together in a real system. Building it also lets me check
whether an explanation matches what the code actually does. The examples and
explanations on this website are part of that learning process.

## Why planning?

Even a single appointment raises connected questions: Which times are occupied? What
information is missing? Which rules must hold? Which time would simply be preferable?
Why does one option work while another fails? Planning gives those questions a
concrete setting where the answers can be checked.

The longer-term aim is planning software that different applications can use, for
example to plan tasks, rooms or shifts. That is the project's direction. The current
planner handles one target appointment at a time; scheduling is its first practical
setting for learning.

## Why start with fixed rules?

Before a learning method can influence a proposal, the system needs explicit information,
rules and limits on what it may change. Today's planner makes those foundations
inspectable. The same input produces the same result, and a proposal alone gives no
permission to carry it out.

Later, I want to explore how neural networks could help search for options or compare
those that satisfy the rules. Fixed checks would still have to enforce facts, required
rules and permissions. A learned model would have no authority to override them.
Neural networks and learned preferences belong to that research direction; the planner
shown here does not use them. The
[development and learning roadmap](../development/roadmap/development-learning-roadmap.md)
describes the planned steps.

## What exists today

The repository contains a Rust planner, temporal checks and examples you can run and
inspect. The software is version **0.2.0**, still unreleased and undergoing release
qualification. There is no published release yet.

These numbers describe different things:

| Item | Version | Meaning |
| --- | --- | --- |
| [Software](../../Cargo.toml) | 0.2.0 | The current development version of the code |
| Specification | 0.4 | The document defining the system's architecture |
| CPIR | 0.2 | The structured format used for the current planning example |
| Rust | 1.97 | The minimum Rust version declared by the workspace |

For the current publication status, see the [Changelog](../../CHANGELOG.md).

## Start with the appointment you need

Suppose you need a 30-minute appointment between 09:00 and 12:00. An existing appointment
occupies 09:00–10:00. The new one must fit inside the requested window and must not overlap
the existing appointment. All times in this example are UTC.

This sentence explains the task to a reader. The current planner receives the same task
as structured data, not as a sentence. Its input format is called **CPIR**. Each piece of
information has its own field, so the planner can check what was supplied.

| Information | In this example | Why it matters |
| --- | --- | --- |
| [Duration](../../examples/request.json) | 30 minutes | The whole appointment must fit |
| Planning window | 09:00–12:00 | The search stays inside this window |
| Known appointment | 09:00–10:00 | These times are occupied |
| Required rule | No overlap | A conflicting option is rejected |
| Preferred start | None supplied | No time is favoured by a preference |
| Search step | 15 minutes | Possible starts are checked at quarter-hour intervals |

The format also records where information came from and which planning permissions apply.
Missing information stays missing. In particular, incomplete availability information is
not treated as proof of free time. A planning window limits the search; it does not grant
permission to change a calendar.

The exact input is in [examples/request.json](../../examples/request.json).
The [CPIR walkthrough](cpir.md) shows where each piece of information goes and why a
start time that still needs to be found differs from a missing duration.

## Check the possible start times

The planner checks 11 starts, from 09:00 through 11:30 in 15-minute steps. The last start
is 11:30 because a 30-minute appointment must finish by 12:00. Each possible placement is
called a **candidate**.

Four candidates are rejected: 09:00, 09:15, 09:30 and 09:45 all overlap the existing
appointment. Seven remain: 10:00, 10:15, 10:30, 10:45, 11:00, 11:15 and 11:30.

This is the order of work:

1. Generate possible starts within the requested window.
2. Reject candidates that conflict with known appointments or required rules.
3. Compare the remaining candidates using preferences and fixed tie-break rules.
4. Return the first candidate as a proposal, together with the search result.

The website displays the result produced by the Rust planner for this example. It does
not calculate a separate ranking in the browser.

## Why 10:00 comes first

The comparison uses these fields in order: distance from the preferred start, number of
changes, amount of time shifted, start time and object identifier. The first differing
field decides the order; the fields are not added into a total score.

There is no preferred start in this example. Both the 10:00 and 10:15 options would create
one new appointment and move no existing appointment. Their first three comparison values
are equal. The next field is the start time, so 10:00 comes before 10:15.

The result is labelled **ProvenOptimal** because all starts in this 15-minute grid were
checked and the proposal comes first under the stated comparison rules. That says
nothing about start times between the grid points or different planning rules.

The [planner reference](planner-integration.md) describes the exact comparison fields.

## A proposal still needs checks and permission

The example ends with a proposal for 10:00–10:30. Nothing has been booked.

Before an application could carry out a change, it would have to validate the full plan,
translate it into concrete actions and obtain authorization. Immediately before execution,
the system checks that the information and permissions are still current. The execution
result then records what actually happened.

| Stage | What it means |
| --- | --- |
| Proposed | The planner has suggested a plan |
| Validated | The plan has passed validation for a specific state of the input data |
| Action plan | The intended changes have been written out explicitly |
| Authorized | Those actions have passed the required permission checks |
| Executed | An execution attempt has produced a result, including any failures |

The repository demonstrates these execution checks with mock and in-memory components.
Production calendar connections and production authentication are not implemented.
See the [lifecycle reference](foundation.md).

## When one appointment ends and another starts

An appointment ending at 10:00 can be followed by one starting at 10:00. The first
appointment no longer occupies time at that instant; the second begins there. They touch
but do not overlap.

Starting the second appointment at 09:59:59 instead would create one second of overlap.
That candidate would fail the no-overlap check.

Cerebri writes an interval as **[start, end)**: the start belongs to the interval, the end
does not. This is called a half-open interval. The filled start marker and open end marker
in the diagram show the same rule. See the [time reference](temporal.md).

## How a preferred time changes the order

The next example adds a preferred start of 10:45. It is a separate example from the one
above, which has no preference. A candidate at 10:30 is 15 minutes, or 900 seconds, away
from that preferred time.

The preference helps compare candidates that have already passed the required checks.
A start further away can still be valid. A preference never makes an overlapping
appointment acceptable.

## Why a measured distance of zero needs context

The current ranking records distance in whole seconds. A difference of 0.8 seconds is
therefore recorded as zero whole seconds. The exact timestamp is preserved; the
appointment has not been moved to the preferred start.

The small measurement examples exercise this calculation directly. They are not new
planner searches and do not change calendar events.

There is another important distinction: no preferred start was supplied versus a
preferred start was supplied and its measured distance is zero. The observation format
keeps these states separate as `None` and `Some(0)`. The numeric comparison uses zero in
both cases, but the underlying information remains different. See
[Ranking Feature Contract v0.1](../architecture/decisions/ADR-0014-ranking-feature-contract.md).

## Continue exploring

- [CPIR](cpir.md): the information a planning request contains.
- [Planning](planner-integration.md): how the current search and comparison work.
- [Time](temporal.md): intervals, time zones and incomplete availability.
- [Foundation](foundation.md): the checks between a proposal and execution.
- [Roadmap](../development/roadmap/README.md): current progress and later research.

- [Learning path and research questions](research.md): What later methods could learn and what already exists today.
