//! Trusted half-open intervals and IANA timezone resolution. No clock reads.
pub use chrono::{DateTime, NaiveDate as LocalDate, NaiveTime as LocalTime, TimeDelta, Utc};
use chrono::{LocalResult, TimeZone};
pub use chrono_tz::Tz as TimeZoneId;
use serde::{Deserialize, Serialize};
use thiserror::Error;
pub type Instant = DateTime<Utc>;
mod availability;
mod diagnostics;
mod recurrence;
pub use availability::*;
pub use diagnostics::*;
pub use recurrence::*;

#[derive(Debug, Clone, PartialEq, Eq, Error, Serialize, Deserialize)]
pub enum TemporalError {
    #[error("range start must precede end")]
    InvalidRange,
    #[error("duration must be positive")]
    InvalidDuration,
    #[error("instant arithmetic overflow")]
    Overflow,
    #[error("local time is ambiguous during a timezone transition")]
    AmbiguousLocalTime,
    #[error("local time does not exist during a timezone transition")]
    NonexistentLocalTime,
    #[error("invalid recurrence definition")]
    InvalidRecurrence,
    #[error("temporal input exceeds admitted limits")]
    InputLimit,
    #[error("occurrence limit exceeded; no partial expansion may be treated as complete")]
    OccurrenceLimitExceeded,
    #[error("date evaluation limit exceeded; no partial expansion may be treated as complete")]
    DateLimitExceeded,
}

/// Positive integral seconds. Zero/negative durations cannot enter trusted planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "i64", into = "i64")]
pub struct Duration(i64);
impl Duration {
    pub fn seconds(value: i64) -> Result<Self, TemporalError> {
        if value <= 0 || chrono::TimeDelta::try_seconds(value).is_none() {
            return Err(TemporalError::InvalidDuration);
        }
        Ok(Self(value))
    }
    pub fn as_seconds(self) -> i64 {
        self.0
    }
    pub fn add_to(self, start: Instant) -> Result<Instant, TemporalError> {
        start
            .checked_add_signed(chrono::TimeDelta::seconds(self.0))
            .ok_or(TemporalError::Overflow)
    }
}
impl TryFrom<i64> for Duration {
    type Error = TemporalError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Self::seconds(value)
    }
}
impl From<Duration> for i64 {
    fn from(value: Duration) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RangeWire", into = "RangeWire")]
pub struct TimeRange {
    start: Instant,
    end: Instant,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RangeWire {
    start: Instant,
    end: Instant,
}
impl TryFrom<RangeWire> for TimeRange {
    type Error = TemporalError;
    fn try_from(value: RangeWire) -> Result<Self, Self::Error> {
        Self::new(value.start, value.end)
    }
}
impl From<TimeRange> for RangeWire {
    fn from(value: TimeRange) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}
impl TimeRange {
    pub fn new(start: Instant, end: Instant) -> Result<Self, TemporalError> {
        if start >= end {
            return Err(TemporalError::InvalidRange);
        }
        Ok(Self { start, end })
    }
    pub fn start(self) -> Instant {
        self.start
    }
    pub fn end(self) -> Instant {
        self.end
    }
    pub fn overlaps(self, other: Self) -> bool {
        self.start < other.end && other.start < self.end
    }
    pub fn contains(self, instant: Instant) -> bool {
        self.start <= instant && instant < self.end
    }
    pub fn contains_range(self, other: Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }
    pub fn duration(self) -> chrono::TimeDelta {
        self.end - self.start
    }
    pub fn relation(self, other: Self) -> Relation {
        if self == other {
            Relation::Equal
        } else if self.end == other.start || other.end == self.start {
            Relation::Touches
        } else if self.end < other.start {
            Relation::Before
        } else if self.start > other.end {
            Relation::After
        } else if self.contains_range(other) {
            Relation::Contains
        } else if other.contains_range(self) {
            Relation::Inside
        } else {
            Relation::Overlaps
        }
    }
    pub fn intersection(self, other: Self) -> Option<Self> {
        Self::new(self.start.max(other.start), self.end.min(other.end)).ok()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Relation {
    Before,
    After,
    Overlaps,
    Contains,
    Inside,
    Touches,
    Equal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZonedDateTime {
    pub instant: Instant,
    pub timezone: TimeZoneId,
}
impl ZonedDateTime {
    /// Rejects DST folds/gaps; callers must resolve ambiguity explicitly.
    pub fn from_local(
        date: LocalDate,
        time: LocalTime,
        timezone: TimeZoneId,
    ) -> Result<Self, TemporalError> {
        match timezone.from_local_datetime(&date.and_time(time)) {
            LocalResult::Single(value) => Ok(Self {
                instant: value.with_timezone(&Utc),
                timezone,
            }),
            LocalResult::Ambiguous(..) => Err(TemporalError::AmbiguousLocalTime),
            LocalResult::None => Err(TemporalError::NonexistentLocalTime),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deadline(pub Instant);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningHorizon(pub TimeRange);
/// Implementations must expand only within the supplied horizon and occurrence limit.
pub trait RecurrenceExpansion {
    type Error;
    fn expand(&self, horizon: PlanningHorizon, limit: usize)
    -> Result<Vec<TimeRange>, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    fn at(seconds: i64) -> Instant {
        Instant::from_timestamp(seconds, 0).unwrap()
    }
    fn range(a: i64, b: i64) -> TimeRange {
        TimeRange::new(at(a), at(b)).unwrap()
    }
    #[test]
    fn touching_half_open_ranges_do_not_overlap() {
        let a = range(0, 60);
        let b = range(60, 120);
        assert!(!a.overlaps(b));
        assert_eq!(a.relation(b), Relation::Touches);
        assert!(!a.contains(at(60)));
        assert_eq!(a.duration().num_seconds(), 60);
        assert!(a.intersection(b).is_none());
        assert!(a.overlaps(range(59, 70)));
    }
    #[test]
    fn invalid_ranges_are_rejected_including_deserialization() {
        assert!(TimeRange::new(at(1), at(1)).is_err());
        assert!(TimeRange::new(at(2), at(1)).is_err());
        assert!(
            serde_json::from_str::<TimeRange>(
                r#"{"start":"2026-01-01T02:00:00Z","end":"2026-01-01T01:00:00Z"}"#
            )
            .is_err()
        );
    }
    #[test]
    fn exhaustive_small_domain_interval_properties() {
        // Exhaustive property testing: deterministic, no random seed needed.
        for a in -5..5 {
            for b in a + 1..6 {
                for c in -5..5 {
                    for d in c + 1..6 {
                        let x = range(a, b);
                        let y = range(c, d);
                        assert_eq!(x.overlaps(y), y.overlaps(x));
                        assert_eq!(x.overlaps(y), x.intersection(y).is_some());
                        if let Some(i) = x.intersection(y) {
                            assert!(x.contains_range(i) && y.contains_range(i));
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn berlin_dst_gap_and_fold_are_not_silently_guessed() {
        let time = LocalTime::from_hms_opt(2, 30, 0).unwrap();
        assert_eq!(
            ZonedDateTime::from_local(
                LocalDate::from_ymd_opt(2026, 3, 29).unwrap(),
                time,
                chrono_tz::Europe::Berlin
            ),
            Err(TemporalError::NonexistentLocalTime)
        );
        assert_eq!(
            ZonedDateTime::from_local(
                LocalDate::from_ymd_opt(2026, 10, 25).unwrap(),
                time,
                chrono_tz::Europe::Berlin
            ),
            Err(TemporalError::AmbiguousLocalTime)
        );
        let zoned = ZonedDateTime::from_local(
            LocalDate::from_ymd_opt(2026, 1, 1).unwrap(),
            time,
            chrono_tz::Europe::Berlin,
        )
        .unwrap();
        assert_eq!(zoned.timezone, chrono_tz::Europe::Berlin);
        assert_eq!(zoned.instant.to_rfc3339(), "2026-01-01T01:30:00+00:00");
    }
}
