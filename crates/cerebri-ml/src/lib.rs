//! Versioned production metadata and inference ports; no framework or research dependency.
use cerebri_temporal::Instant;
use cerebri_types::{ModelId, StoreError, TrainingRunId};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelVersion(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReference {
    pub uri: String,
    pub sha256: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatasetMetadata {
    pub version: String,
    pub artifact: ArtifactReference,
    pub scope: DataScope,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataScope {
    Synthetic,
    GlobalCurated,
    PersonalLocal,
    OptInShared,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainingRunMetadata {
    pub id: TrainingRunId,
    pub at: Instant,
    pub git_commit: String,
    pub software_version: String,
    pub dataset: DatasetMetadata,
    pub architecture: String,
    pub hyperparameters: std::collections::BTreeMap<String, f64>,
    pub seed: u64,
    pub environment: String,
    pub metrics: std::collections::BTreeMap<String, f64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub id: ModelId,
    pub version: ModelVersion,
    pub artifact: ArtifactReference,
    pub training_run: TrainingRunMetadata,
}
/// Registration must reject replacement of an existing (ID, version).
pub trait ModelRegistry {
    fn get(
        &self,
        id: &ModelId,
        version: &ModelVersion,
    ) -> Result<Option<ModelMetadata>, StoreError>;
    fn register(&mut self, metadata: ModelMetadata) -> Result<(), StoreError>;
}
pub trait Inference {
    type Input;
    type Output;
    type Error;
    fn infer(&self, input: &Self::Input) -> Result<Self::Output, Self::Error>;
}
