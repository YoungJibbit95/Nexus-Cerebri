//! Deterministic facts and constraints; permissions deliberately live elsewhere.
use cerebri_temporal::{Deadline, Duration, Instant, LocalDate, TimeDelta, TimeRange, TimeZoneId};
use cerebri_types::{FactId, OccurrenceId, PlanningObjectId, Provenance};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FactValue {
    ScheduledTime(TimeRange),
    Deadline(Deadline),
    ExternalLock,
    Availability(TimeRange),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fact {
    pub id: FactId,
    pub object_id: PlanningObjectId,
    pub value: FactValue,
    pub provenance: Provenance,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HardConstraint {
    NoOverlap,
    ExplicitTime(TimeRange),
    ExplicitDate(LocalDate),
    EarliestStart(Instant),
    LatestEnd(Instant),
    Deadline(Deadline),
    MinDuration(Duration),
    FixedDuration(Duration),
    AvailabilityWindow(TimeRange),
    DependencyOrder(PlanningObjectId),
    RequiredBuffer(Duration),
    /// Reserved boundary: fails closed until bounded recurrence is implemented.
    RecurrenceRule,
    TimezoneIntegrity(TimeZoneId),
    ExternalLock {
        provenance: Provenance,
        reason: String,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConstraintSpec {
    pub object_id: PlanningObjectId,
    pub rule: HardConstraint,
    pub evidence: Vec<FactId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationReason {
    Overlap,
    OutsideBounds,
    WrongDuration,
    DependencyMissing,
    DependencyOrder,
    InsufficientBuffer,
    TimezoneMismatch,
    Locked,
    UnsupportedRecurrence,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstraintEvidence {
    pub facts: Vec<FactId>,
    pub blocking_objects: Vec<PlanningObjectId>,
    #[serde(default)]
    pub blocking_occurrences: Vec<OccurrenceId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstraintViolation {
    pub constraint: HardConstraint,
    pub object_id: PlanningObjectId,
    pub reason: ViolationReason,
    pub evidence: ConstraintEvidence,
}
#[derive(Debug, Clone)]
pub struct TimedObject {
    pub id: PlanningObjectId,
    pub range: TimeRange,
    pub timezone: TimeZoneId,
}
pub fn check(
    specification: &ConstraintSpec,
    candidate: &TimedObject,
    original: Option<TimeRange>,
    others: &[TimedObject],
) -> Option<ConstraintViolation> {
    let range = candidate.range;
    let mut blockers = Vec::new();
    let reason = match &specification.rule {
        HardConstraint::NoOverlap => {
            blockers = others
                .iter()
                .filter(|o| o.id != candidate.id && o.range.overlaps(range))
                .map(|o| o.id.clone())
                .collect();
            (!blockers.is_empty()).then_some(ViolationReason::Overlap)
        }
        HardConstraint::ExplicitTime(expected) => {
            (range != *expected).then_some(ViolationReason::OutsideBounds)
        }
        HardConstraint::ExplicitDate(date) => (cerebri_temporal::ZonedDateTime {
            instant: range.start(),
            timezone: candidate.timezone,
        }
        .local_date()
            != Ok(*date))
        .then_some(ViolationReason::OutsideBounds),
        HardConstraint::EarliestStart(start) => {
            (range.start() < *start).then_some(ViolationReason::OutsideBounds)
        }
        HardConstraint::LatestEnd(end) => {
            (range.end() > *end).then_some(ViolationReason::OutsideBounds)
        }
        HardConstraint::Deadline(deadline) => {
            (range.end() > deadline.0).then_some(ViolationReason::OutsideBounds)
        }
        HardConstraint::MinDuration(duration) => (range.duration()
            < TimeDelta::seconds(duration.as_seconds()))
        .then_some(ViolationReason::WrongDuration),
        HardConstraint::FixedDuration(duration) => (range.duration()
            != TimeDelta::seconds(duration.as_seconds()))
        .then_some(ViolationReason::WrongDuration),
        HardConstraint::AvailabilityWindow(window) => {
            (!window.contains_range(range)).then_some(ViolationReason::OutsideBounds)
        }
        HardConstraint::DependencyOrder(id) => match others.iter().find(|o| o.id == *id) {
            Some(dependency) if dependency.range.end() <= range.start() => None,
            Some(_) => {
                blockers.push(id.clone());
                Some(ViolationReason::DependencyOrder)
            }
            None => Some(ViolationReason::DependencyMissing),
        },
        HardConstraint::RequiredBuffer(buffer) => {
            blockers = others
                .iter()
                .filter(|o| o.id != candidate.id)
                .filter(|o| {
                    if o.range.overlaps(range) {
                        return true;
                    }
                    let gap = if o.range.end() <= range.start() {
                        range.start() - o.range.end()
                    } else {
                        o.range.start() - range.end()
                    };
                    gap < TimeDelta::seconds(buffer.as_seconds())
                })
                .map(|o| o.id.clone())
                .collect();
            (!blockers.is_empty()).then_some(ViolationReason::InsufficientBuffer)
        }
        HardConstraint::RecurrenceRule => Some(ViolationReason::UnsupportedRecurrence),
        HardConstraint::TimezoneIntegrity(zone) => {
            (*zone != candidate.timezone).then_some(ViolationReason::TimezoneMismatch)
        }
        HardConstraint::ExternalLock { .. } => {
            (original != Some(range)).then_some(ViolationReason::Locked)
        }
    };
    blockers.sort();
    reason.map(|reason| ConstraintViolation {
        constraint: specification.rule.clone(),
        object_id: candidate.id.clone(),
        reason,
        evidence: ConstraintEvidence {
            facts: specification.evidence.clone(),
            blocking_objects: blockers,
            blocking_occurrences: vec![],
        },
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn timed(id: &str, start: i64, end: i64) -> TimedObject {
        TimedObject {
            id: PlanningObjectId::new(id).unwrap(),
            range: TimeRange::new(
                Instant::from_timestamp(start, 0).unwrap(),
                Instant::from_timestamp(end, 0).unwrap(),
            )
            .unwrap(),
            timezone: TimeZoneId::UTC,
        }
    }
    #[test]
    fn independent_valid_and_intentional_overlap_examples() {
        let a = timed("a", 0, 60);
        let b = timed("b", 60, 120);
        let c = timed("c", 59, 120);
        let rule = ConstraintSpec {
            object_id: a.id.clone(),
            rule: HardConstraint::NoOverlap,
            evidence: vec![],
        };
        assert!(check(&rule, &a, None, &[b]).is_none());
        let violation = check(&rule, &a, None, &[c]).unwrap();
        assert_eq!(violation.reason, ViolationReason::Overlap);
        assert_eq!(
            violation.evidence.blocking_objects,
            vec![PlanningObjectId::new("c").unwrap()]
        );
    }
    #[test]
    fn lock_and_unimplemented_recurrence_fail_closed() {
        let a = timed("a", 0, 60);
        let mut rule = ConstraintSpec {
            object_id: a.id.clone(),
            rule: HardConstraint::ExternalLock {
                provenance: Provenance::UserExplicit,
                reason: "locked".into(),
            },
            evidence: vec![],
        };
        assert!(check(&rule, &a, Some(a.range), &[]).is_none());
        assert!(check(&rule, &a, None, &[]).is_some());
        rule.rule = HardConstraint::RecurrenceRule;
        assert_eq!(
            check(&rule, &a, None, &[]).unwrap().reason,
            ViolationReason::UnsupportedRecurrence
        );
    }
}
