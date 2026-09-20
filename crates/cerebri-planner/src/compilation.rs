//! Pure recurrence-to-snapshot boundary. Occurrences are occupancy, never mutation targets.
use crate::{ContextSnapshot, PlanningObjectKind};
use cerebri_temporal::*;
use cerebri_types::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum SeriesState {
    Existing { revision: Revision },
    Prospective,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemporalSeries {
    pub id: SeriesId,
    pub state: SeriesState,
    pub provenance: Provenance,
    pub evidence: Vec<FactId>,
    pub rule: RecurrenceRule,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemporalContext {
    pub horizon: PlanningHorizon,
    pub coverage: Coverage,
    pub limits: ExpansionLimits,
    pub series: Vec<TemporalSeries>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedOccurrence {
    pub id: OccurrenceId,
    pub series_id: SeriesId,
    pub source_revision: Revision,
    pub provenance: Provenance,
    pub evidence: Vec<FactId>,
    pub occurrence: Occurrence,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeriesExpansion {
    pub series: TemporalSeries,
    pub expansion: ExpansionReport,
}
/// Immutable derived view of a ContextSnapshot. Does not replace source/freshness identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompiledContextSnapshot {
    pub source_revision: Revision,
    pub horizon: PlanningHorizon,
    pub occurrences: Vec<MaterializedOccurrence>,
    pub series: Vec<SeriesExpansion>,
    pub availability: FreeBusyReport,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompilationError {
    HorizonMismatch,
    InputLimit,
    DuplicateSeries(SeriesId),
    IdentityCollision(String),
    ProspectiveSeries(SeriesId),
    InvalidProvenance(SeriesId),
    UnknownObjectTime(PlanningObjectId),
    Temporal(TemporalError),
}
impl From<TemporalError> for CompilationError {
    fn from(value: TemporalError) -> Self {
        Self::Temporal(value)
    }
}

pub fn compile_snapshot(
    context: &ContextSnapshot,
    horizon: PlanningHorizon,
) -> Result<Option<CompiledContextSnapshot>, CompilationError> {
    let Some(source) = &context.temporal else {
        return Ok(None);
    };
    if source.horizon != horizon {
        return Err(CompilationError::HorizonMismatch);
    }
    if source.series.len() > 32
        || context.objects.len() > 256
        || source.limits.max_occurrences > 1_024
        || source.series.iter().any(|s| s.evidence.len() > 1_024)
    {
        return Err(CompilationError::InputLimit);
    }
    source.limits.validate()?;
    let mut sorted: Vec<_> = source.series.iter().collect();
    sorted.sort_by(|a, b| a.id.cmp(&b.id));
    let mut seen = BTreeSet::new();
    for series in &sorted {
        if !seen.insert(&series.id) {
            return Err(CompilationError::DuplicateSeries(series.id.clone()));
        }
        if context
            .objects
            .iter()
            .any(|o| o.id.as_str() == series.id.as_str())
        {
            return Err(CompilationError::IdentityCollision(
                series.id.as_str().into(),
            ));
        }
        if matches!(series.state, SeriesState::Prospective) {
            return Err(CompilationError::ProspectiveSeries(series.id.clone()));
        }
        if !series.provenance.is_fact_source() {
            return Err(CompilationError::InvalidProvenance(series.id.clone()));
        }
    }
    let mut remaining = source.limits;
    let mut occurrences = Vec::new();
    let mut expansions = Vec::new();
    let mut identities = BTreeSet::new();
    for series in sorted {
        let expansion = series.rule.expand_report(horizon, remaining)?;
        remaining.max_dates -= expansion.examined_dates;
        remaining.max_occurrences -= expansion.occurrences.len();
        let SeriesState::Existing { revision } = series.state else {
            unreachable!()
        };
        let mut canonical_series = series.clone();
        canonical_series.evidence.sort();
        canonical_series.evidence.dedup();
        for occurrence in &expansion.occurrences {
            let key = serde_json::to_vec(&(
                &series.id,
                occurrence.date,
                occurrence.local_time,
                expansion.timezone,
            ))
            .expect("serializable identity");
            let id =
                OccurrenceId::new(format!("occ-{:x}", Sha256::digest(key))).expect("bounded ID");
            if !identities.insert(id.clone())
                || context.objects.iter().any(|o| o.id.as_str() == id.as_str())
            {
                return Err(CompilationError::IdentityCollision(id.as_str().into()));
            }
            occurrences.push(MaterializedOccurrence {
                id,
                series_id: series.id.clone(),
                source_revision: revision,
                provenance: series.provenance,
                evidence: canonical_series.evidence.clone(),
                occurrence: occurrence.clone(),
            });
        }
        expansions.push(SeriesExpansion {
            series: canonical_series,
            expansion,
        });
    }
    occurrences.sort_by(|a, b| a.id.cmp(&b.id));
    let mut busy = Vec::new();
    let mut objects: Vec<_> = context.objects.iter().collect();
    objects.sort_by(|a, b| a.id.cmp(&b.id));
    for object in objects {
        if matches!(
            object.kind,
            PlanningObjectKind::Event | PlanningObjectKind::Task(_)
        ) {
            match object.time.value.required(false) {
                Ok((range, _)) => busy.push(BusyInterval {
                    range: *range,
                    buffers: Buffers::default(),
                }),
                Err(_)
                    if object.revision.is_none()
                        && matches!(
                            object.time.value,
                            FieldState::Resolved(Knowledge::Missing)
                        ) => {}
                Err(_) => return Err(CompilationError::UnknownObjectTime(object.id.clone())),
            }
        }
    }
    busy.extend(occurrences.iter().map(|o| BusyInterval {
        range: o.occurrence.range,
        buffers: Buffers::default(),
    }));
    Ok(Some(CompiledContextSnapshot {
        source_revision: context.revision,
        horizon,
        occurrences,
        series: expansions,
        availability: free_busy(horizon, &busy, source.coverage)?,
    }))
}
