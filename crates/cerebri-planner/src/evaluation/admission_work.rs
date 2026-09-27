//! Shared bounded metadata-work observation (§28, ADR-0013). This is deliberately
//! compact serde_json 1.0.151 serialization of the typed request, never JCS.
use super::*;
use crate::PlanningRequest;
use serde::{Deserialize, Serialize};
use std::io::Write;

pub const SERIALIZED_REQUEST_LIMIT_BYTES: u64 = 262_144;
pub const CANDIDATE_WEIGHTED_LIMIT_BYTES: u64 = 16_777_216;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlannerAdmissionWorkMetricV0_1 {
    #[serde(rename = "RUST_SERDE_JSON_1_0_151_PLANNING_REQUEST_V0_1")]
    RustSerdeJson1_0_151PlanningRequestV0_1,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", deny_unknown_fields)]
pub enum SerializedRequestMeasurementV0_1 {
    #[serde(rename = "MEASURED")]
    Measured { value: CanonicalU64Decimal },
    #[serde(rename = "EXCEEDS_SERIALIZATION_LIMIT")]
    ExceedsSerializationLimit {},
    #[serde(rename = "SERIALIZATION_ERROR")]
    SerializationError {},
}
closed_wire! {
pub struct PlannerAdmissionWorkWireV0_1 {
    pub metric_version: PlannerAdmissionWorkMetricV0_1,
    pub serialized_request_bytes: SerializedRequestMeasurementV0_1,
    pub serialized_request_limit_bytes: CanonicalU64Decimal,
    pub candidate_weighted_bytes: CanonicalOptionalV1<CanonicalU64Decimal>,
    pub candidate_weighted_limit_bytes: CanonicalU64Decimal,
}
}
/// Validated observation; imported measurements still require source remeasurement
/// and `validate_budget` at the DecisionInput/replay boundary. No source authentication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct PlannerAdmissionWorkObservationV0_1(PlannerAdmissionWorkWireV0_1);
impl TryFrom<PlannerAdmissionWorkWireV0_1> for PlannerAdmissionWorkObservationV0_1 {
    type Error = EvaluationContractError;
    fn try_from(v: PlannerAdmissionWorkWireV0_1) -> Result<Self, Self::Error> {
        if v.serialized_request_limit_bytes.value() != SERIALIZED_REQUEST_LIMIT_BYTES
            || v.candidate_weighted_limit_bytes.value() != CANDIDATE_WEIGHTED_LIMIT_BYTES
        {
            return Err(EvaluationContractError(
                "incompatible admission work limits",
            ));
        }
        match (&v.serialized_request_bytes, &v.candidate_weighted_bytes) {
            (
                SerializedRequestMeasurementV0_1::Measured { value: b },
                CanonicalOptionalV1::Some { value: w },
            ) if (1..=SERIALIZED_REQUEST_LIMIT_BYTES).contains(&b.value())
                && w.value().is_multiple_of(b.value())
                && w.value() / b.value() <= u64::from(u32::MAX) => {}
            (
                SerializedRequestMeasurementV0_1::ExceedsSerializationLimit {}
                | SerializedRequestMeasurementV0_1::SerializationError {},
                CanonicalOptionalV1::None {},
            ) => {}
            _ => {
                return Err(EvaluationContractError(
                    "inconsistent admission work measurement",
                ));
            }
        }
        Ok(Self(v))
    }
}
deserialize_object_via!(
    PlannerAdmissionWorkObservationV0_1,
    PlannerAdmissionWorkWireV0_1
);
impl PlannerAdmissionWorkObservationV0_1 {
    pub fn measure(request: &PlanningRequest) -> Self {
        measure(request, request.budget.max_candidates)
    }
    pub fn wire(&self) -> &PlannerAdmissionWorkWireV0_1 {
        &self.0
    }
    pub fn admitted(&self) -> bool {
        matches!(self.0.candidate_weighted_bytes, CanonicalOptionalV1::Some {value} if value.value() <= CANDIDATE_WEIGHTED_LIMIT_BYTES)
    }
    pub fn validate_budget(&self, max_candidates: u32) -> Result<(), EvaluationContractError> {
        if let SerializedRequestMeasurementV0_1::Measured { value: b } =
            self.0.serialized_request_bytes
            && self.0.candidate_weighted_bytes
                != (CanonicalOptionalV1::Some {
                    value: decimal(b.value() * u64::from(max_candidates)),
                })
        {
            return Err(EvaluationContractError(
                "admission work disagrees with search budget",
            ));
        }
        Ok(())
    }
}
fn decimal(value: u64) -> CanonicalU64Decimal {
    CanonicalU64Decimal::new(value).expect("every u64 has a canonical decimal representation")
}
#[derive(Default)]
struct Counter {
    bytes: u64,
    exceeded: bool,
}
impl Write for Counter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() as u64 > SERIALIZED_REQUEST_LIMIT_BYTES - self.bytes {
            self.exceeded = true;
            return Err(std::io::Error::other("planner input limit"));
        }
        self.bytes += bytes.len() as u64;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
fn measure<T: Serialize>(request: &T, max_candidates: u32) -> PlannerAdmissionWorkObservationV0_1 {
    let mut writer = Counter::default();
    let result = serde_json::to_writer(&mut writer, request);
    let (serialized_request_bytes, candidate_weighted_bytes) = if writer.exceeded {
        (
            SerializedRequestMeasurementV0_1::ExceedsSerializationLimit {},
            CanonicalOptionalV1::None {},
        )
    } else if result.is_err() || writer.bytes == 0 {
        (
            SerializedRequestMeasurementV0_1::SerializationError {},
            CanonicalOptionalV1::None {},
        )
    } else {
        (
            SerializedRequestMeasurementV0_1::Measured {
                value: decimal(writer.bytes),
            },
            CanonicalOptionalV1::Some {
                value: decimal(writer.bytes * u64::from(max_candidates)),
            },
        )
    };
    PlannerAdmissionWorkObservationV0_1(PlannerAdmissionWorkWireV0_1 {
        metric_version: PlannerAdmissionWorkMetricV0_1::RustSerdeJson1_0_151PlanningRequestV0_1,
        serialized_request_bytes,
        serialized_request_limit_bytes: decimal(SERIALIZED_REQUEST_LIMIT_BYTES),
        candidate_weighted_bytes,
        candidate_weighted_limit_bytes: decimal(CANDIDATE_WEIGHTED_LIMIT_BYTES),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capped_writer_and_weight_boundaries() {
        for (bytes, accepted) in [(262143, true), (262144, true), (262145, false)] {
            let v = measure(&"x".repeat(bytes - 2), 64);
            assert_eq!(v.admitted(), accepted);
            if accepted {
                assert_eq!(
                    v.0.serialized_request_bytes,
                    SerializedRequestMeasurementV0_1::Measured {
                        value: decimal(bytes as u64)
                    }
                );
            } else {
                assert_eq!(
                    v.0.serialized_request_bytes,
                    SerializedRequestMeasurementV0_1::ExceedsSerializationLimit {}
                );
                assert_eq!(v.0.candidate_weighted_bytes, CanonicalOptionalV1::None {});
            }
        }
        // Single-byte JSON scalar isolates exact weighted boundary arithmetic.
        assert!(measure(&0, 16_777_216).admitted());
        assert!(!measure(&0, 16_777_217).admitted());
        assert!(measure(&0, 0).admitted());
    }
    #[test]
    fn serializer_failure_is_not_reported_as_a_complete_length() {
        struct Broken;
        impl Serialize for Broken {
            fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
                Err(serde::ser::Error::custom("synthetic serialization failure"))
            }
        }
        let v = measure(&Broken, 256);
        assert_eq!(
            v.0.serialized_request_bytes,
            SerializedRequestMeasurementV0_1::SerializationError {}
        );
        assert_eq!(v.0.candidate_weighted_bytes, CanonicalOptionalV1::None {});
        assert!(!v.admitted());
    }
}
