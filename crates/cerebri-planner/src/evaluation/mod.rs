//! Synthetic evaluation contract foundations (Slice 2, Phase A checkpoint).
//!
//! These types validate individual wire values only. They do not construct a validated
//! EvaluationEpisode, canonicalize a scenario, compute fingerprints, or replay planning.
//! Planning, ranking, admission and execution retain their existing authorities.

mod artifact;
mod primitives;
mod tokens;
mod wire;

pub use artifact::*;
pub use primitives::*;
pub use tokens::*;
pub use wire::*;

/// A closed-contract validation failure. Input content is not included in the error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("{0}")]
pub struct EvaluationContractError(&'static str);

/// Require the JSON member's presence while retaining explicit null as `None`.
///
/// Use with `#[serde(deserialize_with = "required_nullable")]`, without a default.
/// A transparent Option wrapper alone does not enforce member presence.
pub fn required_nullable<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    <Option<T> as serde::Deserialize>::deserialize(deserializer)
}
