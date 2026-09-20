//! Bounded local-calendar recurrence, deliberately not a general RFC 5545 parser.
use crate::*;
pub use chrono::Weekday;
use chrono::{Datelike, Days, LocalResult, Offset, TimeZone, Timelike};
use serde::{Deserialize, Serialize};
use std::num::{NonZeroU16, NonZeroU32};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "frequency",
    rename_all = "SCREAMING_SNAKE_CASE",
    deny_unknown_fields
)]
pub enum RecurrencePattern {
    Daily {
        every: NonZeroU16,
    },
    Weekly {
        every: NonZeroU16,
        weekdays: Vec<Weekday>,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GapPolicy {
    Reject,
    Skip,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FoldPolicy {
    Reject,
    Earlier,
    Later,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecurrenceDefinition {
    pub start_date: LocalDate,
    pub local_time: LocalTime,
    pub timezone: TimeZoneId,
    pub duration: Duration,
    pub pattern: RecurrencePattern,
    /// Inclusive local date boundary.
    pub until: Option<LocalDate>,
    /// Counts nominal scheduled dates, including explicitly skipped DST gaps.
    pub count: Option<NonZeroU32>,
    pub gap_policy: GapPolicy,
    pub fold_policy: FoldPolicy,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RecurrenceDefinition", into = "RecurrenceDefinition")]
pub struct RecurrenceRule(RecurrenceDefinition);
impl RecurrenceRule {
    pub fn new(mut definition: RecurrenceDefinition) -> Result<Self, TemporalError> {
        if definition
            .until
            .is_some_and(|until| until < definition.start_date)
            || definition.local_time.nanosecond() >= 1_000_000_000
        {
            return Err(TemporalError::InvalidRecurrence);
        }
        if let RecurrencePattern::Weekly { weekdays, .. } = &mut definition.pattern {
            weekdays.sort_by_key(|d| d.num_days_from_monday());
            if weekdays.is_empty() || weekdays.windows(2).any(|w| w[0] == w[1]) {
                return Err(TemporalError::InvalidRecurrence);
            }
        }
        Ok(Self(definition))
    }
    pub fn definition(&self) -> &RecurrenceDefinition {
        &self.0
    }
    fn sequence(&self, date: LocalDate) -> Option<u64> {
        let days = date.signed_duration_since(self.0.start_date).num_days();
        if days < 0 {
            return None;
        }
        let days = days as u64;
        match &self.0.pattern {
            RecurrencePattern::Daily { every } => days
                .is_multiple_of(u64::from(every.get()))
                .then_some(days / u64::from(every.get()) + 1),
            RecurrencePattern::Weekly { every, weekdays } => {
                let anchor = u64::from(self.0.start_date.weekday().num_days_from_monday());
                let week = (days + anchor) / 7;
                if !week.is_multiple_of(u64::from(every.get()))
                    || !weekdays.contains(&date.weekday())
                {
                    return None;
                }
                let excluded = weekdays
                    .iter()
                    .filter(|d| u64::from(d.num_days_from_monday()) < anchor)
                    .count() as u64;
                let current = weekdays
                    .iter()
                    .filter(|d| d.num_days_from_monday() <= date.weekday().num_days_from_monday())
                    .count() as u64;
                Some((week / u64::from(every.get())) * weekdays.len() as u64 + current - excluded)
            }
        }
    }
    pub fn expand_report(
        &self,
        horizon: PlanningHorizon,
        limits: ExpansionLimits,
    ) -> Result<ExpansionReport, TemporalError> {
        limits.validate()?;
        let definition = &self.0;
        let lookback = horizon
            .0
            .start()
            .checked_sub_signed(TimeDelta::seconds(definition.duration.as_seconds()))
            .ok_or(TemporalError::Overflow)?;
        // Dates are sought near the horizon instead of scanning from a distant anchor.
        let local_checked = |instant: Instant| {
            let offset = definition
                .timezone
                .offset_from_utc_datetime(&instant.naive_utc());
            instant
                .naive_utc()
                .checked_add_signed(TimeDelta::seconds(i64::from(
                    offset.fix().local_minus_utc(),
                )))
                .ok_or(TemporalError::Overflow)
        };
        let lower_local = local_checked(lookback)?;
        let upper_local = local_checked(horizon.0.end())?;
        let mut date = lower_local
            .date()
            .checked_sub_days(Days::new(2))
            .unwrap_or(LocalDate::MIN)
            .max(definition.start_date);
        let last = upper_local
            .date()
            .checked_add_days(Days::new(2))
            .unwrap_or(LocalDate::MAX);
        let mut report = ExpansionReport {
            horizon,
            timezone: definition.timezone,
            occurrences: vec![],
            skipped: vec![],
            examined_dates: 0,
        };
        while date <= last && definition.until.is_none_or(|until| date <= until) {
            if report.examined_dates >= limits.max_dates {
                return Err(TemporalError::DateLimitExceeded);
            }
            report.examined_dates += 1;
            if let Some(sequence) = self.sequence(date) {
                if definition
                    .count
                    .is_some_and(|count| sequence > u64::from(count.get()))
                {
                    break;
                }
                let local = date.and_time(definition.local_time);
                let selected = match definition.timezone.from_local_datetime(&local) {
                    LocalResult::Single(value) => {
                        Some((value.with_timezone(&Utc), OccurrenceResolution::Unique))
                    }
                    LocalResult::Ambiguous(a, b) => {
                        let earlier = a.with_timezone(&Utc).min(b.with_timezone(&Utc));
                        let later = a.with_timezone(&Utc).max(b.with_timezone(&Utc));
                        let mut relevant = false;
                        for start in [earlier, later] {
                            if start < horizon.0.end() {
                                relevant |=
                                    TimeRange::new(start, definition.duration.add_to(start)?)?
                                        .overlaps(horizon.0);
                            }
                        }
                        if !relevant {
                            None
                        } else {
                            match definition.fold_policy {
                                FoldPolicy::Reject => {
                                    return Err(TemporalError::AmbiguousLocalTime);
                                }
                                FoldPolicy::Earlier => {
                                    Some((earlier, OccurrenceResolution::EarlierFold))
                                }
                                FoldPolicy::Later => Some((later, OccurrenceResolution::LaterFold)),
                            }
                        }
                    }
                    LocalResult::None => {
                        // A nonexistent local time outside the queried local window is irrelevant.
                        if local >= lower_local && local < upper_local {
                            match definition.gap_policy {
                                GapPolicy::Reject => {
                                    return Err(TemporalError::NonexistentLocalTime);
                                }
                                GapPolicy::Skip => report.skipped.push(SkippedOccurrence {
                                    sequence,
                                    date,
                                    reason: SkipReason::NonexistentLocalTime,
                                }),
                            }
                        }
                        None
                    }
                };
                if let Some((start, resolution)) = selected {
                    // Ignore occurrences starting after the horizon before end arithmetic.
                    if start < horizon.0.end() {
                        let range = TimeRange::new(start, definition.duration.add_to(start)?)?;
                        if let Some(visible_range) = range.intersection(horizon.0) {
                            if report.occurrences.len() >= limits.max_occurrences {
                                return Err(TemporalError::OccurrenceLimitExceeded);
                            }
                            report.occurrences.push(Occurrence {
                                sequence,
                                date,
                                local_time: definition.local_time,
                                range,
                                visible_range,
                                resolution,
                            });
                        }
                    }
                }
            }
            if date == last || definition.until == Some(date) {
                break;
            }
            date = date.succ_opt().ok_or(TemporalError::Overflow)?;
        }
        report
            .occurrences
            .sort_by_key(|o| (o.range.start(), o.sequence));
        Ok(report)
    }
}
impl TryFrom<RecurrenceDefinition> for RecurrenceRule {
    type Error = TemporalError;
    fn try_from(value: RecurrenceDefinition) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl From<RecurrenceRule> for RecurrenceDefinition {
    fn from(value: RecurrenceRule) -> Self {
        value.0
    }
}
impl RecurrenceExpansion for RecurrenceRule {
    type Error = TemporalError;
    fn expand(
        &self,
        horizon: PlanningHorizon,
        limit: usize,
    ) -> Result<Vec<TimeRange>, Self::Error> {
        Ok(self
            .expand_report(
                horizon,
                ExpansionLimits {
                    max_occurrences: limit,
                    ..Default::default()
                },
            )?
            .occurrences
            .into_iter()
            .map(|o| o.visible_range)
            .collect())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpansionLimits {
    pub max_occurrences: usize,
    pub max_dates: u32,
}
impl Default for ExpansionLimits {
    fn default() -> Self {
        Self {
            max_occurrences: 1_000,
            max_dates: 36_600,
        }
    }
}
impl ExpansionLimits {
    pub fn validate(self) -> Result<(), TemporalError> {
        if self.max_occurrences > 10_000 || self.max_dates > 36_600 {
            return Err(TemporalError::InputLimit);
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OccurrenceResolution {
    Unique,
    EarlierFold,
    LaterFold,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Occurrence {
    pub sequence: u64,
    pub date: LocalDate,
    pub local_time: LocalTime,
    pub range: TimeRange,
    pub visible_range: TimeRange,
    pub resolution: OccurrenceResolution,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkipReason {
    NonexistentLocalTime,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkippedOccurrence {
    pub sequence: u64,
    pub date: LocalDate,
    pub reason: SkipReason,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpansionReport {
    pub horizon: PlanningHorizon,
    pub timezone: TimeZoneId,
    pub occurrences: Vec<Occurrence>,
    pub skipped: Vec<SkippedOccurrence>,
    pub examined_dates: u32,
}
