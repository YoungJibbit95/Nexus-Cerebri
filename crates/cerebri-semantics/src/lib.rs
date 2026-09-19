//! Inference is evidence and cannot change source facts.
use cerebri_temporal::{Deadline, Instant};
use cerebri_types::{EvidenceField, FactId, Provenance};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticVector {
    pub importance: EvidenceField<f64>,
    pub urgency: EvidenceField<f64>,
    pub temporal_rigidity: EvidenceField<f64>,
    pub external_dependency: EvidenceField<f64>,
    pub human_dependency: EvidenceField<f64>,
    pub rescheduling_cost: EvidenceField<f64>,
    pub consequence_of_delay: EvidenceField<f64>,
    pub duration_flexibility: EvidenceField<f64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeadlinePressure {
    pub remaining_seconds: i64,
    pub source: FactId,
    pub source_provenance: Provenance,
    pub reference_time: Instant,
    pub derivation_rule: String,
}
pub fn deadline_pressure(
    deadline: Deadline,
    source: FactId,
    provenance: Provenance,
    now: Instant,
) -> DeadlinePressure {
    DeadlinePressure {
        remaining_seconds: (deadline.0 - now).num_seconds(),
        source,
        source_provenance: provenance,
        reference_time: now,
        derivation_rule: "remaining-seconds/0.1".into(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn derivation_preserves_source_and_uses_injected_time() {
        let instant = Instant::from_timestamp(100, 0).unwrap();
        let deadline = Deadline(instant);
        let result = deadline_pressure(
            deadline,
            FactId::new("deadline").unwrap(),
            Provenance::IntegrationFact,
            Instant::from_timestamp(40, 0).unwrap(),
        );
        assert_eq!(result.remaining_seconds, 60);
        assert_eq!(deadline.0, instant);
        assert_eq!(result.source_provenance, Provenance::IntegrationFact);
    }
}
