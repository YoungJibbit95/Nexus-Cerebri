//! Dedicated request-duration grammar (§19), distinct from generic knowledge values.
use super::scenario::{ProjectionResult, seconds};
use super::*;
use cerebri_temporal::Duration;
use cerebri_types::{FieldState, Knowledge};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct CanonicalDurationKnowledgeV1(DurationKnowledge);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "state")]
enum DurationKnowledge {
    #[serde(rename = "KNOWN")]
    Known { seconds: CanonicalPositiveSeconds },
    #[serde(rename = "UNCERTAIN")]
    Uncertain { seconds: CanonicalPositiveSeconds },
    #[serde(rename = "AMBIGUOUS")]
    Ambiguous {
        seconds: Vec<CanonicalPositiveSeconds>,
    },
    #[serde(rename = "MISSING")]
    Missing,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "UNRESOLVED")]
    Unresolved,
}

impl TryFrom<&FieldState<Duration>> for CanonicalDurationKnowledgeV1 {
    type Error = EvaluationContractError;

    fn try_from(value: &FieldState<Duration>) -> ProjectionResult<Self> {
        let value = match value {
            FieldState::Unresolved => DurationKnowledge::Unresolved,
            FieldState::Resolved(Knowledge::Missing) => DurationKnowledge::Missing,
            FieldState::Resolved(Knowledge::Unknown) => DurationKnowledge::Unknown,
            FieldState::Resolved(Knowledge::Known(value)) => DurationKnowledge::Known {
                seconds: seconds(*value)?,
            },
            FieldState::Resolved(Knowledge::Uncertain {
                value,
                confidence: _,
            }) => DurationKnowledge::Uncertain {
                seconds: seconds(*value)?,
            },
            FieldState::Resolved(Knowledge::Ambiguous(values)) => {
                let mut values = values
                    .iter()
                    .map(|value| seconds(*value))
                    .collect::<ProjectionResult<Vec<_>>>()?;
                values.sort_by_key(|value| value.value());
                values.dedup();
                if values.len() < 2 {
                    return Err(EvaluationContractError(
                        "ambiguous duration requires two distinct values",
                    ));
                }
                DurationKnowledge::Ambiguous { seconds: values }
            }
        };
        Ok(Self(value))
    }
}
