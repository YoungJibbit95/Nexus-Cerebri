//! Structured diagnostics for Lab; no interpretation, planning or I/O.
use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemporalRequest {
    pub horizon: PlanningHorizon,
    pub busy: Vec<BusyInterval>,
    pub coverage: Coverage,
    pub recurrences: Vec<RecurrenceRule>,
    pub limits: ExpansionLimits,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalReport {
    pub availability: FreeBusyReport,
    pub expansions: Vec<ExpansionReport>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", content = "data")]
pub enum TemporalResult {
    Complete(TemporalReport),
    Rejected(TemporalError),
}
pub fn temporal_diagnostics(request: TemporalRequest) -> TemporalResult {
    match diagnose(request) {
        Ok(report) => TemporalResult::Complete(report),
        Err(error) => TemporalResult::Rejected(error),
    }
}
fn diagnose(request: TemporalRequest) -> Result<TemporalReport, TemporalError> {
    request.limits.validate()?;
    if request.recurrences.len() > 32 || request.busy.len() > 10_000 {
        return Err(TemporalError::InputLimit);
    }
    let mut remaining = request.limits;
    let mut busy = request.busy;
    let mut expansions = Vec::new();
    for rule in request.recurrences {
        let report = rule.expand_report(request.horizon, remaining)?;
        remaining.max_occurrences -= report.occurrences.len();
        remaining.max_dates -= report.examined_dates;
        busy.extend(report.occurrences.iter().map(|o| BusyInterval {
            range: o.visible_range,
            buffers: Buffers::default(),
        }));
        expansions.push(report);
    }
    Ok(TemporalReport {
        availability: free_busy(request.horizon, &busy, request.coverage)?,
        expansions,
    })
}
