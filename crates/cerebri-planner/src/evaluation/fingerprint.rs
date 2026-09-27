//! Domain-separated byte hashing and pure candidate/hypothesis identities.
//! No planner execution, capture, policy application or replay takes place here.
use super::*;
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FingerprintDomainV1 {
    CanonicalGraph,
    BaseScenario,
    DecisionInput,
    DecisionObservation,
    Candidate,
    SearchHypothesis,
    HypothesisSet,
    VisitOrder,
}
impl FingerprintDomainV1 {
    pub const fn prefix(self) -> &'static [u8] {
        match self {
            Self::CanonicalGraph => b"nexus-cerebri:canonical-graph-serialization:v1\0",
            Self::BaseScenario => b"nexus-cerebri:base-scenario-fingerprint:v1\0",
            Self::DecisionInput => b"nexus-cerebri:decision-input-fingerprint:v1\0",
            Self::DecisionObservation => b"nexus-cerebri:decision-observation-digest:v1\0",
            Self::Candidate => b"nexus-cerebri:candidate-fingerprint:v1\0",
            Self::SearchHypothesis => b"nexus-cerebri:search-hypothesis:v0.1\0",
            Self::HypothesisSet => b"nexus-cerebri:hypothesis-set-identity:v0.1\0",
            Self::VisitOrder => b"nexus-cerebri:search-visit-order:v0.1\0",
        }
    }
    /// Hash bytes only. This does not validate or authenticate a semantic payload.
    pub fn digest_bytes(self, canonical_bytes: &[u8]) -> SHA256Hex {
        let mut hash = Sha256::new();
        hash.update(self.prefix());
        hash.update(canonical_bytes);
        SHA256Hex::new(format!("{:x}", hash.finalize()))
            .expect("SHA-256 produces 64 lowercase hex digits")
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CandidateFingerprintPayloadV1 {
    schema_version: &'static str,
    placements: [DigestPlacementV1; 1],
}
impl CandidateFingerprintPayloadV1 {
    pub fn new(placement: DigestPlacementV1) -> Result<Self, EvaluationContractError> {
        validate_placement(&placement)?;
        Ok(Self {
            schema_version: "1",
            placements: [placement],
        })
    }
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, EvaluationContractError> {
        canonical_json_bytes(self)
    }
    pub fn fingerprint(&self) -> Result<SHA256Hex, EvaluationContractError> {
        Ok(FingerprintDomainV1::Candidate.digest_bytes(&self.canonical_bytes()?))
    }
}
fn validate_placement(placement: &DigestPlacementV1) -> Result<(), EvaluationContractError> {
    if placement.start >= placement.end {
        return Err(EvaluationContractError("placement start must precede end"));
    }
    Ok(())
}
impl CanonicalSearchHypothesisV0_1 {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, EvaluationContractError> {
        validate_placement(&self.placements[0])?;
        canonical_json_bytes(self)
    }
    pub fn fingerprint(&self) -> Result<SHA256Hex, EvaluationContractError> {
        Ok(FingerprintDomainV1::SearchHypothesis.digest_bytes(&self.canonical_bytes()?))
    }
}
fn hypothesis_members(
    hypotheses: &[CanonicalSearchHypothesisV0_1],
) -> Result<Vec<Vec<u8>>, EvaluationContractError> {
    let bytes: Vec<_> = hypotheses
        .iter()
        .map(CanonicalSearchHypothesisV0_1::canonical_bytes)
        .collect::<Result<_, _>>()?;
    let mut unique = std::collections::BTreeSet::new();
    if bytes.iter().any(|value| !unique.insert(value)) {
        return Err(EvaluationContractError("duplicate hypothesis"));
    }
    Ok(bytes)
}
fn array_bytes(members: Vec<Vec<u8>>) -> Vec<u8> {
    let mut bytes = vec![b'['];
    for (index, member) in members.into_iter().enumerate() {
        if index != 0 {
            bytes.push(b',');
        }
        bytes.extend(member);
    }
    bytes.push(b']');
    bytes
}
/// Canonical semantic set (§11); duplicates reject before identity is accepted.
pub fn canonical_hypothesis_set_bytes(
    hypotheses: &[CanonicalSearchHypothesisV0_1],
) -> Result<Vec<u8>, EvaluationContractError> {
    let mut members = hypothesis_members(hypotheses)?;
    members.sort();
    Ok(array_bytes(members))
}
pub fn hypothesis_set_identity(
    hypotheses: &[CanonicalSearchHypothesisV0_1],
) -> Result<SHA256Hex, EvaluationContractError> {
    Ok(FingerprintDomainV1::HypothesisSet
        .digest_bytes(&canonical_hypothesis_set_bytes(hypotheses)?))
}
pub fn visit_order_identity(
    hypotheses: &[CanonicalSearchHypothesisV0_1],
) -> Result<SHA256Hex, EvaluationContractError> {
    Ok(FingerprintDomainV1::VisitOrder.digest_bytes(&array_bytes(hypothesis_members(hypotheses)?)))
}
