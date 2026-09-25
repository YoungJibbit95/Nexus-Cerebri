//! Interpretable preferences and storage ports; no learning or implicit writes.
use cerebri_temporal::Instant;
use cerebri_types::{FactId, PlanningObjectId, PrincipalId, StoreError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// Ordered from highest to lowest precedence (Master section 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PreferenceSource {
    ExplicitCurrentRequest,
    SessionContext,
    PersonalLearned,
    GlobalLearned,
    Default,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreferenceEvidence {
    pub source: PreferenceSource,
    pub preferred_start: Instant,
    pub evidence: Vec<FactId>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreferenceProfile {
    pub preferences: Vec<PreferenceEvidence>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankingFeatures {
    pub distance_from_preferred_start_seconds: u64,
}
/// Independent observation-schema authority, unrelated to CPIR or software versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RankingFeatureSchemaVersion {
    pub major: u16,
    pub minor: u16,
}
pub const RANKING_FEATURE_SCHEMA_V0_1: RankingFeatureSchemaVersion =
    RankingFeatureSchemaVersion { major: 0, minor: 1 };

/// Validated deterministic observations, not a learning schema or execution authority.
/// Private fields preserve the version and paired-nullability invariants after decoding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RankingFeatureSetWire")]
pub struct RankingFeatureSet {
    schema_version: RankingFeatureSchemaVersion,
    preferred_start_distance_seconds: Option<u64>,
    preferred_start_source: Option<PreferenceSource>,
    mutation_count: u32,
    shift_seconds: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RankingFeatureSetWire {
    schema_version: RankingFeatureSchemaVersion,
    // deserialize_with (without default) makes presence mandatory even for Option<T>.
    // Plain derived Option<T>, including a transparent wrapper, is not sufficient.
    #[serde(deserialize_with = "present_nullable")]
    preferred_start_distance_seconds: Option<u64>,
    #[serde(deserialize_with = "present_nullable")]
    preferred_start_source: Option<PreferenceSource>,
    mutation_count: u32,
    shift_seconds: u64,
}
fn present_nullable<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer)
}
impl TryFrom<RankingFeatureSetWire> for RankingFeatureSet {
    type Error = &'static str;
    fn try_from(wire: RankingFeatureSetWire) -> Result<Self, Self::Error> {
        if wire.schema_version != RANKING_FEATURE_SCHEMA_V0_1 {
            return Err("unsupported ranking feature schema version");
        }
        if wire.preferred_start_distance_seconds.is_none() != wire.preferred_start_source.is_none() {
            return Err("preferred-start distance and source must both be null or both have values");
        }
        Ok(Self {
            schema_version: wire.schema_version,
            preferred_start_distance_seconds: wire.preferred_start_distance_seconds,
            preferred_start_source: wire.preferred_start_source,
            mutation_count: wire.mutation_count,
            shift_seconds: wire.shift_seconds,
        })
    }
}
impl RankingFeatureSet {
    pub fn schema_version(&self) -> RankingFeatureSchemaVersion {
        self.schema_version
    }
    /// None means no evidence; Some(0) includes nonzero subsecond distances.
    pub fn preferred_start_distance_seconds(&self) -> Option<u64> {
        self.preferred_start_distance_seconds
    }
    /// Provenance only; never an ordering term.
    pub fn preferred_start_source(&self) -> Option<PreferenceSource> {
        self.preferred_start_source
    }
    /// Contextual m(candidate, request): analysis-only = 0, current mutation = 1.
    pub fn mutation_count(&self) -> u32 {
        self.mutation_count
    }
    /// Legacy projection: absent placement, zero and subsecond shifts all map to zero.
    pub fn shift_seconds(&self) -> u64 {
        self.shift_seconds
    }
}
impl PreferenceProfile {
    pub fn preferred_start(&self) -> Option<&PreferenceEvidence> {
        self.preferences
            .iter()
            .min_by_key(|p| (p.source, p.preferred_start))
    }
    /// Equal-source ties resolve by time, independently of input order.
    pub fn features(&self, start: Instant) -> RankingFeatures {
        RankingFeatures {
            distance_from_preferred_start_seconds: self
                .ranking_features(start, 0, None)
                .preferred_start_distance_seconds()
                .unwrap_or(0),
        }
    }
    /// Resolve source/time once, preserving the existing whole-second projection.
    /// The planner supplies request-dependent mutation count and original placement.
    pub fn ranking_features(
        &self,
        start: Instant,
        mutation_count: u32,
        original_start: Option<Instant>,
    ) -> RankingFeatureSet {
        let preferred = self.preferred_start();
        RankingFeatureSet {
            schema_version: RANKING_FEATURE_SCHEMA_V0_1,
            preferred_start_distance_seconds: preferred.map(|p| {
                (start - p.preferred_start).num_seconds().unsigned_abs()
            }),
            preferred_start_source: preferred.map(|p| p.source),
            mutation_count,
            shift_seconds: original_start.map_or(0, |original| {
                (original - start).num_seconds().unsigned_abs()
            }),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeedbackKind {
    Selected,
    Corrected,
    Rejected,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedbackEvent {
    pub principal_id: PrincipalId,
    pub object_id: PlanningObjectId,
    pub at: Instant,
    pub kind: FeedbackKind,
}
pub trait PreferenceStore {
    fn load(&self, principal: &PrincipalId) -> Result<Option<PreferenceProfile>, StoreError>;
    fn save(
        &mut self,
        principal: PrincipalId,
        profile: PreferenceProfile,
    ) -> Result<(), StoreError>;
}
pub trait FeedbackStore {
    fn append(&mut self, event: FeedbackEvent) -> Result<(), StoreError>;
    fn for_principal(&self, principal: &PrincipalId) -> Result<Vec<FeedbackEvent>, StoreError>;
}
#[derive(Default)]
pub struct InMemoryPreferenceStore(BTreeMap<PrincipalId, PreferenceProfile>);
impl PreferenceStore for InMemoryPreferenceStore {
    fn load(&self, principal: &PrincipalId) -> Result<Option<PreferenceProfile>, StoreError> {
        Ok(self.0.get(principal).cloned())
    }
    fn save(
        &mut self,
        principal: PrincipalId,
        profile: PreferenceProfile,
    ) -> Result<(), StoreError> {
        self.0.insert(principal, profile);
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nullable_decoder_accepts_explicit_json_null_and_value_only() {
        for (json, expected) in [("null", None), ("0", Some(0)), ("3600", Some(3600))] {
            let mut decoder = serde_json::Deserializer::from_str(json);
            assert_eq!(present_nullable::<_, u64>(&mut decoder).unwrap(), expected);
        }
        for json in ["", "-1", "false", "\"0\""] {
            let mut decoder = serde_json::Deserializer::from_str(json);
            assert!(present_nullable::<_, u64>(&mut decoder).is_err());
        }
    }
    #[test]
    fn explicit_preference_outranks_learned_and_store_is_scoped() {
        let at = |t| Instant::from_timestamp(t, 0).unwrap();
        let profile = PreferenceProfile {
            preferences: vec![
                PreferenceEvidence {
                    source: PreferenceSource::GlobalLearned,
                    preferred_start: at(60),
                    evidence: vec![],
                },
                PreferenceEvidence {
                    source: PreferenceSource::ExplicitCurrentRequest,
                    preferred_start: at(0),
                    evidence: vec![],
                },
            ],
        };
        assert_eq!(
            profile
                .features(at(0))
                .distance_from_preferred_start_seconds,
            0
        );
        let mut store = InMemoryPreferenceStore::default();
        let user = PrincipalId::new("synthetic-user").unwrap();
        store.save(user.clone(), profile.clone()).unwrap();
        assert_eq!(store.load(&user).unwrap(), Some(profile));
        assert_eq!(
            store.load(&PrincipalId::new("another").unwrap()).unwrap(),
            None
        );
    }
}
