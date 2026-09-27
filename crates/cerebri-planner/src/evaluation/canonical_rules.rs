//! Closed top-level BSF grammars (§§17.3, 17.5); distinct from graph typed values.
use super::*;
use cerebri_temporal::{RecurrencePattern, RecurrenceRule, TimeRange, TimeZoneId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CanonicalTimeRangeV1 {
    start: CanonicalInstantV1,
    end: CanonicalInstantV1,
}
impl CanonicalTimeRangeV1 {
    pub fn new(
        start: CanonicalInstantV1,
        end: CanonicalInstantV1,
    ) -> Result<Self, EvaluationContractError> {
        if start >= end {
            return Err(EvaluationContractError(
                "canonical range start must precede end",
            ));
        }
        Ok(Self { start, end })
    }
    pub fn start(self) -> CanonicalInstantV1 {
        self.start
    }
    pub fn end(self) -> CanonicalInstantV1 {
        self.end
    }
}
closed_wire! {
#[derive(Copy)]
pub struct CanonicalTimeRangeWireV1 {
    pub start: CanonicalInstantV1,
    pub end: CanonicalInstantV1,
}
}
impl TryFrom<CanonicalTimeRangeWireV1> for CanonicalTimeRangeV1 {
    type Error = EvaluationContractError;
    fn try_from(v: CanonicalTimeRangeWireV1) -> Result<Self, Self::Error> {
        Self::new(v.start, v.end)
    }
}
deserialize_object_via!(CanonicalTimeRangeV1, CanonicalTimeRangeWireV1);
impl TryFrom<TimeRange> for CanonicalTimeRangeV1 {
    type Error = EvaluationContractError;
    fn try_from(v: TimeRange) -> Result<Self, Self::Error> {
        Self::new(v.start().try_into()?, v.end().try_into()?)
    }
}

/// Validated exact timezone name. No normalization or alias rewriting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CanonicalTimezoneV1(String);
impl From<TimeZoneId> for CanonicalTimezoneV1 {
    fn from(v: TimeZoneId) -> Self {
        Self(v.name().to_owned())
    }
}
impl TryFrom<String> for CanonicalTimezoneV1 {
    type Error = EvaluationContractError;
    fn try_from(v: String) -> Result<Self, Self::Error> {
        let zone: TimeZoneId = v
            .parse()
            .map_err(|_| EvaluationContractError("invalid canonical timezone"))?;
        if zone.name() != v {
            return Err(EvaluationContractError("noncanonical timezone"));
        }
        Ok(Self(v))
    }
}
impl From<CanonicalTimezoneV1> for String {
    fn from(v: CanonicalTimezoneV1) -> Self {
        v.0
    }
}

/// Canonical rule variants have direct payload members, without a `value` envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum CanonicalHardConstraintRuleV1 {
    #[serde(rename = "NO_OVERLAP")]
    NoOverlap {},
    #[serde(rename = "EXPLICIT_TIME")]
    ExplicitTime { range: CanonicalTimeRangeV1 },
    #[serde(rename = "EXPLICIT_DATE")]
    ExplicitDate { date: CanonicalDate },
    #[serde(rename = "EARLIEST_START")]
    EarliestStart { at: CanonicalInstantV1 },
    #[serde(rename = "LATEST_END")]
    LatestEnd { at: CanonicalInstantV1 },
    #[serde(rename = "DEADLINE")]
    Deadline { at: CanonicalInstantV1 },
    #[serde(rename = "MIN_DURATION")]
    MinDuration { seconds: CanonicalPositiveSeconds },
    #[serde(rename = "FIXED_DURATION")]
    FixedDuration { seconds: CanonicalPositiveSeconds },
    #[serde(rename = "AVAILABILITY_WINDOW")]
    AvailabilityWindow { range: CanonicalTimeRangeV1 },
    #[serde(rename = "DEPENDENCY_ORDER")]
    DependencyOrder { object: CanonicalAlias },
    #[serde(rename = "REQUIRED_BUFFER")]
    RequiredBuffer { seconds: CanonicalPositiveSeconds },
    #[serde(rename = "RECURRENCE_RULE")]
    RecurrenceRule {},
    #[serde(rename = "TIMEZONE_INTEGRITY")]
    TimezoneIntegrity { timezone: CanonicalTimezoneV1 },
    #[serde(rename = "EXTERNAL_LOCK")]
    ExternalLock { provenance: ProvenanceToken },
}

/// Input/wire form; the domain wrapper enforces interval bounds and weekday order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "frequency", deny_unknown_fields)]
pub enum CanonicalRecurrencePatternWireV1 {
    #[serde(rename = "DAILY")]
    Daily { every: CanonicalU64Decimal },
    #[serde(rename = "WEEKLY")]
    Weekly {
        every: CanonicalU64Decimal,
        weekdays: Vec<WeekdayToken>,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct CanonicalRecurrencePatternV1(CanonicalRecurrencePatternWireV1);
pub(crate) fn weekday_rank(day: WeekdayToken) -> u8 {
    match day {
        WeekdayToken::Monday => 0,
        WeekdayToken::Tuesday => 1,
        WeekdayToken::Wednesday => 2,
        WeekdayToken::Thursday => 3,
        WeekdayToken::Friday => 4,
        WeekdayToken::Saturday => 5,
        WeekdayToken::Sunday => 6,
    }
}
impl TryFrom<CanonicalRecurrencePatternWireV1> for CanonicalRecurrencePatternV1 {
    type Error = EvaluationContractError;
    fn try_from(v: CanonicalRecurrencePatternWireV1) -> Result<Self, Self::Error> {
        let every = match &v {
            CanonicalRecurrencePatternWireV1::Daily { every } => every,
            CanonicalRecurrencePatternWireV1::Weekly { every, weekdays } => {
                if weekdays.is_empty()
                    || weekdays
                        .windows(2)
                        .any(|w| weekday_rank(w[0]) >= weekday_rank(w[1]))
                {
                    return Err(EvaluationContractError(
                        "weekdays must be nonempty, unique and Monday-Sunday ordered",
                    ));
                }
                every
            }
        };
        if !(1..=65535).contains(&every.value()) {
            return Err(EvaluationContractError(
                "recurrence interval outside source domain",
            ));
        }
        Ok(Self(v))
    }
}
deserialize_object_via!(
    CanonicalRecurrencePatternV1,
    CanonicalRecurrencePatternWireV1
);
impl TryFrom<&RecurrencePattern> for CanonicalRecurrencePatternV1 {
    type Error = EvaluationContractError;
    fn try_from(v: &RecurrencePattern) -> Result<Self, Self::Error> {
        match v {
            RecurrencePattern::Daily { every } => CanonicalRecurrencePatternWireV1::Daily {
                every: CanonicalU64Decimal::new(u64::from(every.get()))?,
            },
            RecurrencePattern::Weekly { every, weekdays } => {
                let mut weekdays: Vec<_> = weekdays
                    .iter()
                    .map(|day| match day {
                        cerebri_temporal::Weekday::Mon => WeekdayToken::Monday,
                        cerebri_temporal::Weekday::Tue => WeekdayToken::Tuesday,
                        cerebri_temporal::Weekday::Wed => WeekdayToken::Wednesday,
                        cerebri_temporal::Weekday::Thu => WeekdayToken::Thursday,
                        cerebri_temporal::Weekday::Fri => WeekdayToken::Friday,
                        cerebri_temporal::Weekday::Sat => WeekdayToken::Saturday,
                        cerebri_temporal::Weekday::Sun => WeekdayToken::Sunday,
                    })
                    .collect();
                weekdays.sort_by_key(|day| weekday_rank(*day));
                CanonicalRecurrencePatternWireV1::Weekly {
                    every: CanonicalU64Decimal::new(u64::from(every.get()))?,
                    weekdays,
                }
            }
        }
        .try_into()
    }
}
closed_wire! {
pub struct CanonicalRecurrenceRuleWireV1 {
    pub start_date: CanonicalDate,
    pub local_time: CanonicalLocalTime,
    pub timezone: CanonicalTimezoneV1,
    pub duration_seconds: CanonicalPositiveSeconds,
    pub pattern: CanonicalRecurrencePatternV1,
    pub until: CanonicalOptionalV1<CanonicalDate>,
    pub count: CanonicalOptionalV1<CanonicalU64Decimal>,
    pub gap_policy: GapPolicyToken,
    pub fold_policy: FoldPolicyToken,
}
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct CanonicalRecurrenceRuleV1(CanonicalRecurrenceRuleWireV1);
impl TryFrom<CanonicalRecurrenceRuleWireV1> for CanonicalRecurrenceRuleV1 {
    type Error = EvaluationContractError;
    fn try_from(v: CanonicalRecurrenceRuleWireV1) -> Result<Self, Self::Error> {
        if let CanonicalOptionalV1::Some { value } = v.count
            && !(1..=u64::from(u32::MAX)).contains(&value.value())
        {
            return Err(EvaluationContractError(
                "recurrence count outside source domain",
            ));
        }
        if let CanonicalOptionalV1::Some { value } = &v.until
            && value < &v.start_date
        {
            return Err(EvaluationContractError("recurrence until precedes start"));
        }
        Ok(Self(v))
    }
}
deserialize_object_via!(CanonicalRecurrenceRuleV1, CanonicalRecurrenceRuleWireV1);
impl TryFrom<&RecurrenceRule> for CanonicalRecurrenceRuleV1 {
    type Error = EvaluationContractError;
    fn try_from(v: &RecurrenceRule) -> Result<Self, Self::Error> {
        let v = v.definition();
        CanonicalRecurrenceRuleWireV1 {
            start_date: v.start_date.try_into()?,
            local_time: v.local_time.try_into()?,
            timezone: v.timezone.into(),
            duration_seconds: CanonicalPositiveSeconds::new(v.duration.as_seconds() as u64)?,
            pattern: (&v.pattern).try_into()?,
            until: match v.until {
                None => CanonicalOptionalV1::None {},
                Some(date) => CanonicalOptionalV1::Some {
                    value: date.try_into()?,
                },
            },
            count: match v.count {
                None => CanonicalOptionalV1::None {},
                Some(count) => CanonicalOptionalV1::Some {
                    value: CanonicalU64Decimal::new(u64::from(count.get()))?,
                },
            },
            gap_policy: match v.gap_policy {
                cerebri_temporal::GapPolicy::Reject => GapPolicyToken::Reject,
                cerebri_temporal::GapPolicy::Skip => GapPolicyToken::Skip,
            },
            fold_policy: match v.fold_policy {
                cerebri_temporal::FoldPolicy::Reject => FoldPolicyToken::Reject,
                cerebri_temporal::FoldPolicy::Earlier => FoldPolicyToken::Earlier,
                cerebri_temporal::FoldPolicy::Later => FoldPolicyToken::Later,
            },
        }
        .try_into()
    }
}
closed_wire! {
pub struct CanonicalTemporalSeriesWireV1 {
    pub series: CanonicalAlias,
    pub state: SeriesStateToken,
    pub provenance: ProvenanceToken,
    pub evidence_fact_refs: Vec<CanonicalAlias>,
    pub rule: CanonicalRecurrenceRuleV1,
}
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct CanonicalTemporalSeriesV1(CanonicalTemporalSeriesWireV1);
impl TryFrom<CanonicalTemporalSeriesWireV1> for CanonicalTemporalSeriesV1 {
    type Error = EvaluationContractError;
    fn try_from(v: CanonicalTemporalSeriesWireV1) -> Result<Self, Self::Error> {
        if v.evidence_fact_refs.windows(2).any(|w| w[0] >= w[1]) {
            return Err(EvaluationContractError(
                "evidence refs must be unique and sorted",
            ));
        }
        Ok(Self(v))
    }
}
deserialize_object_via!(CanonicalTemporalSeriesV1, CanonicalTemporalSeriesWireV1);
