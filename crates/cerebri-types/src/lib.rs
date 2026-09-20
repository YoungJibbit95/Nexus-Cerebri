//! Shared identity and evidence types. No I/O or planning dependencies.
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ValueError {
    #[error("identifier must contain 1..=128 ASCII letters, digits, '.', '_' or '-'")]
    InvalidIdentifier,
    #[error("confidence must be finite and within [0, 1]")]
    InvalidConfidence,
}

macro_rules! identifier {
    ($($name:ident),+ $(,)?) => {$(
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);
        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, ValueError> {
                let value = value.into();
                if value.is_empty() || value.len() > 128 ||
                    !value.bytes().all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c)) {
                    return Err(ValueError::InvalidIdentifier);
                }
                Ok(Self(value))
            }
            pub fn as_str(&self) -> &str { &self.0 }
        }
        impl TryFrom<String> for $name {
            type Error = ValueError;
            fn try_from(value: String) -> Result<Self, Self::Error> { Self::new(value) }
        }
        impl From<$name> for String {
            fn from(value: $name) -> String { value.0 }
        }
    )+};
}
identifier!(
    RequestId,
    PlanningObjectId,
    SeriesId,
    OccurrenceId,
    PlanId,
    ActionId,
    TraceId,
    CalendarId,
    IntegrationId,
    ResourceId,
    FactId,
    PolicySetId,
    PrincipalId,
    ModelId,
    TrainingRunId,
    IdempotencyKey
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Revision(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SchemaVersion {
    pub major: u16,
    pub minor: u16,
}
impl SchemaVersion {
    pub const CPIR_0_1: Self = Self { major: 0, minor: 1 };
    pub const CPIR_0_2: Self = Self { major: 0, minor: 2 };
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "f64", into = "f64")]
pub struct Confidence(f64);
impl Confidence {
    pub fn new(value: f64) -> Result<Self, ValueError> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(ValueError::InvalidConfidence);
        }
        Ok(Self(value))
    }
    pub fn value(self) -> f64 {
        self.0
    }
}
impl TryFrom<f64> for Confidence {
    type Error = ValueError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl From<Confidence> for f64 {
    fn from(value: Confidence) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Provenance {
    UserExplicit,
    IntegrationFact,
    SystemFact,
    PersonalLearned,
    GlobalLearned,
    ModelInference,
    Default,
}
impl Provenance {
    pub fn is_fact_source(self) -> bool {
        matches!(
            self,
            Self::UserExplicit | Self::IntegrationFact | Self::SystemFact
        )
    }
}

/// Epistemic state; parsing progress lives in FieldState, not here.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "state", content = "data", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Knowledge<T> {
    Known(T),
    Missing,
    Unknown,
    Uncertain { value: T, confidence: Confidence },
    Ambiguous(Vec<T>),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "processing",
    content = "knowledge",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum FieldState<T> {
    Resolved(Knowledge<T>),
    Unresolved,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequiredFieldError {
    Missing,
    Unknown,
    Uncertain,
    Ambiguous,
    Unresolved,
}
impl<T> FieldState<T> {
    pub fn known(value: T) -> Self {
        Self::Resolved(Knowledge::Known(value))
    }
    pub fn required(&self, allow_uncertain: bool) -> Result<(&T, bool), RequiredFieldError> {
        match self {
            Self::Resolved(Knowledge::Known(value)) => Ok((value, false)),
            Self::Resolved(Knowledge::Uncertain { value, .. }) if allow_uncertain => {
                Ok((value, true))
            }
            Self::Resolved(Knowledge::Missing) => Err(RequiredFieldError::Missing),
            Self::Resolved(Knowledge::Unknown) => Err(RequiredFieldError::Unknown),
            Self::Resolved(Knowledge::Uncertain { .. }) => Err(RequiredFieldError::Uncertain),
            Self::Resolved(Knowledge::Ambiguous(_)) => Err(RequiredFieldError::Ambiguous),
            Self::Unresolved => Err(RequiredFieldError::Unresolved),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceField<T> {
    pub value: FieldState<T>,
    pub provenance: Provenance,
    pub confidence: Option<Confidence>,
    pub evidence: Vec<FactId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StoreError {
    #[error("store unavailable")]
    Unavailable,
    #[error("immutable record already exists")]
    AlreadyExists,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_values_cannot_enter_through_json() {
        assert!(serde_json::from_str::<RequestId>("\"\"").is_err());
        assert!(serde_json::from_str::<Confidence>("1.1").is_err());
        assert!(Confidence::new(f64::NAN).is_err());
        assert!(Confidence::new(-0.1).is_err());
    }
    #[test]
    fn knowledge_states_remain_distinct() {
        let states = [
            FieldState::<u8>::Resolved(Knowledge::Missing),
            FieldState::Resolved(Knowledge::Unknown),
            FieldState::Resolved(Knowledge::Ambiguous(vec![1, 2])),
            FieldState::Unresolved,
        ];
        let errors = [
            RequiredFieldError::Missing,
            RequiredFieldError::Unknown,
            RequiredFieldError::Ambiguous,
            RequiredFieldError::Unresolved,
        ];
        for (state, error) in states.into_iter().zip(errors) {
            assert_eq!(state.required(false), Err(error));
            let json = serde_json::to_string(&state).unwrap();
            assert_eq!(
                serde_json::from_str::<FieldState<u8>>(&json).unwrap(),
                state
            );
        }
    }
}
