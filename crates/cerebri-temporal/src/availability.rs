//! Free/busy normalization does not infer completeness from an empty collection.
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Buffers {
    pub before_seconds: u32,
    pub after_seconds: u32,
    /// Travel occupies additional time before this event.
    pub travel_seconds: u32,
}
impl Buffers {
    pub fn apply(self, range: TimeRange) -> Result<TimeRange, TemporalError> {
        let before = i64::from(self.before_seconds) + i64::from(self.travel_seconds);
        let start = range
            .start()
            .checked_sub_signed(TimeDelta::seconds(before))
            .ok_or(TemporalError::Overflow)?;
        let end = range
            .end()
            .checked_add_signed(TimeDelta::seconds(i64::from(self.after_seconds)))
            .ok_or(TemporalError::Overflow)?;
        TimeRange::new(start, end)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BusyInterval {
    pub range: TimeRange,
    pub buffers: Buffers,
}
/// Completeness of the supplied collection, not an alternative field-knowledge model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Coverage {
    Complete,
    Incomplete,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntervalTrace {
    pub input_index: usize,
    pub original: TimeRange,
    pub buffered: TimeRange,
    pub clipped: Option<TimeRange>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreeBusyReport {
    pub horizon: PlanningHorizon,
    pub coverage: Coverage,
    pub busy: Vec<TimeRange>,
    pub free: Vec<TimeRange>,
    pub unknown: Vec<TimeRange>,
    pub trace: Vec<IntervalTrace>,
}
/// Clip and union busy coverage. Touching ranges remain non-overlapping, but their union
/// is contiguous and therefore has no positive-length free interval between them.
pub fn normalize_busy(horizon: PlanningHorizon, ranges: &[TimeRange]) -> Vec<TimeRange> {
    let mut ranges: Vec<_> = ranges
        .iter()
        .filter_map(|r| r.intersection(horizon.0))
        .collect();
    ranges.sort_by_key(|r| (r.start(), r.end()));
    let mut result: Vec<TimeRange> = Vec::new();
    for range in ranges {
        if let Some(last) = result.last_mut()
            && range.start() <= last.end()
        {
            *last = TimeRange::new(last.start(), last.end().max(range.end()))
                .expect("union of nonempty sorted ranges");
        } else {
            result.push(range);
        }
    }
    result
}
pub fn free_busy(
    horizon: PlanningHorizon,
    inputs: &[BusyInterval],
    coverage: Coverage,
) -> Result<FreeBusyReport, TemporalError> {
    if inputs.len() > 10_000 {
        return Err(TemporalError::InputLimit);
    }
    let mut trace = Vec::with_capacity(inputs.len());
    let mut expanded = Vec::with_capacity(inputs.len());
    for (index, input) in inputs.iter().enumerate() {
        let buffered = input.buffers.apply(input.range)?;
        expanded.push(buffered);
        trace.push(IntervalTrace {
            input_index: index,
            original: input.range,
            buffered,
            clipped: buffered.intersection(horizon.0),
        });
    }
    let busy = normalize_busy(horizon, &expanded);
    let mut complement = Vec::new();
    let mut cursor = horizon.0.start();
    for range in &busy {
        if cursor < range.start() {
            complement.push(TimeRange::new(cursor, range.start())?);
        }
        cursor = range.end();
    }
    if cursor < horizon.0.end() {
        complement.push(TimeRange::new(cursor, horizon.0.end())?);
    }
    let (free, unknown) = match coverage {
        Coverage::Complete => (complement, vec![]),
        Coverage::Incomplete => (vec![], complement),
    };
    Ok(FreeBusyReport {
        horizon,
        coverage,
        busy,
        free,
        unknown,
        trace,
    })
}
impl TimeRange {
    /// Subtract one half-open range; at most two nonempty fragments remain.
    pub fn subtract(self, other: Self) -> Vec<Self> {
        let Some(overlap) = self.intersection(other) else {
            return vec![self];
        };
        let mut result = Vec::with_capacity(2);
        if self.start() < overlap.start() {
            result.push(Self::new(self.start(), overlap.start()).expect("ordered bounds"));
        }
        if overlap.end() < self.end() {
            result.push(Self::new(overlap.end(), self.end()).expect("ordered bounds"));
        }
        result
    }
}
