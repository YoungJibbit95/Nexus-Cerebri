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
impl PreferenceProfile {
    pub fn preferred_start(&self) -> Option<&PreferenceEvidence> {
        self.preferences
            .iter()
            .min_by_key(|p| (p.source, p.preferred_start))
    }
    /// Equal-source ties resolve by time, independently of input order.
    pub fn features(&self, start: Instant) -> RankingFeatures {
        let preferred = self.preferred_start();
        RankingFeatures {
            distance_from_preferred_start_seconds: preferred.map_or(0, |p| {
                (start - p.preferred_start).num_seconds().unsigned_abs()
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
