//! Synthetic evaluation contracts and partial Phase-B canonical foundations.
//!
//! Canonical values, closed rule grammars, candidate/hypothesis identities and shared
//! admission-work measurement are implemented. Whole-scenario graph/BSF/DI/DO projection,
//! lifecycle/collection validation, artifact authentication and replay remain pending.
//! Planning, ranking, admission thresholds and execution retain their authorities.

// A closed wire struct must deserialize from a map, never a positional JSON array.
// The helper remains typed and streaming; no untyped JSON intermediate is used.
macro_rules! closed_wire {
    ($(#[$attr:meta])* pub struct $name:ident {
        $($(#[$field_attr:meta])* pub $field:ident: $ty:ty),* $(,)?
    }) => {
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub struct $name { $($(#[$field_attr])* pub $field: $ty),* }
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                #[derive(serde::Deserialize)]
                #[serde(deny_unknown_fields)]
                struct Fields { $($(#[$field_attr])* $field: $ty),* }
                let fields = super::object_only::<D, Fields>(deserializer)?;
                Ok(Self { $($field: fields.$field),* })
            }
        }
    };
}

macro_rules! deserialize_object_via {
    ($name:ty, $wire:ty) => {
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                super::object_only::<D, $wire>(deserializer)?
                    .try_into()
                    .map_err(serde::de::Error::custom)
            }
        }
    };
}

mod admission_work;
mod artifact;
mod canonical;
mod canonical_rules;
mod episode;
mod fingerprint;
mod graph;
mod graph_recurrence;
mod graph_registry;
mod observation;
mod primitives;
mod tokens;
mod wire;

pub use admission_work::*;
pub use artifact::*;
pub use canonical::*;
pub use canonical_rules::*;
pub use episode::*;
pub use fingerprint::*;
pub use graph::*;
pub use graph_recurrence::*;
pub use observation::*;
pub use primitives::*;
pub use tokens::*;
pub use wire::*;

/// A closed-contract validation failure. Input content is not included in the error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("{0}")]
pub struct EvaluationContractError(&'static str);

// Serde's default struct visitor also accepts positional JSON arrays. Closed JSON
// object contracts require a map visitor, while retaining duplicate-field detection.
fn object_only<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    struct ObjectVisitor<T>(std::marker::PhantomData<T>);
    impl<'de, T: serde::Deserialize<'de>> serde::de::Visitor<'de> for ObjectVisitor<T> {
        type Value = T;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a JSON object")
        }
        fn visit_map<M: serde::de::MapAccess<'de>>(self, map: M) -> Result<T, M::Error> {
            T::deserialize(serde::de::value::MapAccessDeserializer::new(map))
        }
    }
    deserializer.deserialize_map(ObjectVisitor(std::marker::PhantomData))
}

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
