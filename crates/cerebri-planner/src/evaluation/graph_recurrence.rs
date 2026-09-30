//! SERIES intrinsic recurrence (§14.6), deliberately distinct from §17.5's outer grammar.
use super::graph_registry::cv as c;
use super::*;
use cerebri_temporal::{FoldPolicy, GapPolicy, RecurrencePattern, RecurrenceRule, Weekday};

/// Project a validated temporal rule into the closed graph OBJECT. This does not
/// expand occurrences or construct a temporal context, PlanningRequest or BSF.
pub fn canonical_graph_recurrence_v1(
    rule: &RecurrenceRule,
) -> Result<CanonicalValueV1, EvaluationContractError> {
    use CanonicalValueKindV1 as K;
    let r = rule.definition();
    let (frequency, interval, weekdays) = match &r.pattern {
        RecurrencePattern::Daily { every } => ("DAILY", every.get(), vec![]),
        RecurrencePattern::Weekly { every, weekdays } => (
            "WEEKLY",
            every.get(),
            weekdays
                .iter()
                .map(|v| {
                    c::token(match v {
                        Weekday::Mon => "MONDAY",
                        Weekday::Tue => "TUESDAY",
                        Weekday::Wed => "WEDNESDAY",
                        Weekday::Thu => "THURSDAY",
                        Weekday::Fri => "FRIDAY",
                        Weekday::Sat => "SATURDAY",
                        Weekday::Sun => "SUNDAY",
                    })
                })
                .collect(),
        ),
    };
    let until = match r.until {
        None => c::none(),
        Some(v) => c::value(K::Date { v: v.try_into()? }),
    };
    Ok(c::object(vec![
        (
            "count",
            r.count
                .map_or_else(c::none, |v| c::unsigned(u64::from(v.get()))),
        ),
        (
            "duration_seconds",
            c::unsigned(r.duration.as_seconds() as u64),
        ),
        (
            "fold_policy",
            c::token(match r.fold_policy {
                FoldPolicy::Reject => "REJECT",
                FoldPolicy::Earlier => "EARLIER",
                FoldPolicy::Later => "LATER",
            }),
        ),
        ("frequency", c::token(frequency)),
        (
            "gap_policy",
            c::token(match r.gap_policy {
                GapPolicy::Reject => "REJECT",
                GapPolicy::Skip => "SKIP",
            }),
        ),
        ("interval", c::unsigned(u64::from(interval))),
        (
            "local_time",
            c::value(K::LocalTime {
                v: r.local_time.try_into()?,
            }),
        ),
        (
            "start_date",
            c::value(K::Date {
                v: r.start_date.try_into()?,
            }),
        ),
        ("timezone", c::text(r.timezone.name())),
        ("until", until),
        ("weekdays", c::set(weekdays)),
    ]))
}
