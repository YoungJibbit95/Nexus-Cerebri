//! Structural artifact identity/manifest contracts; no resolver or digest computation.
use super::{
    ArtifactClassToken, ArtifactManifestSchemaVersion, ArtifactRoleToken, CanonicalU64Decimal,
    EvaluationContractError, IdentifierString, RepositoryCommitHex, SHA256Hex,
};
use serde::{Deserialize, Serialize};

/// A normalized bundle-relative path, independent of the host filesystem's syntax.
/// No filesystem access, Unicode normalization or path repair occurs here.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct BundleRelativePath(String);
impl BundleRelativePath {
    pub fn new(value: impl Into<String>) -> Result<Self, EvaluationContractError> {
        let value = value.into();
        let bytes = value.as_bytes();
        if value.contains(['\\', '\0'])
            || value
                .split('/')
                .any(|part| part.is_empty() || part == "." || part == "..")
            || (bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':')
        {
            return Err(EvaluationContractError(
                "expected a normalized bundle-relative path",
            ));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl TryFrom<String> for BundleRelativePath {
    type Error = EvaluationContractError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl From<BundleRelativePath> for String {
    fn from(value: BundleRelativePath) -> Self {
        value.0
    }
}

closed_wire! {
/// Semantic identity only. Structural validity does not authenticate referenced bytes.
pub struct ArtifactSemanticIdentityV0_1 {
    pub artifact_class: ArtifactClassToken,
    pub artifact_id: IdentifierString,
    pub artifact_version: IdentifierString,
    pub manifest_schema_version: IdentifierString,
    pub manifest_sha256: SHA256Hex,
}
}

/// One immutable bundle entry, with nonempty, unique, token-sorted roles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ArtifactManifestEntryV0_1 {
    path: BundleRelativePath,
    roles: Vec<ArtifactRoleToken>,
    sha256: SHA256Hex,
    size_bytes: CanonicalU64Decimal,
}
deserialize_object_via!(ArtifactManifestEntryV0_1, ManifestEntryWire);
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestEntryWire {
    path: BundleRelativePath,
    roles: Vec<ArtifactRoleToken>,
    sha256: SHA256Hex,
    size_bytes: CanonicalU64Decimal,
}
impl ArtifactManifestEntryV0_1 {
    pub fn new(
        path: BundleRelativePath,
        roles: Vec<ArtifactRoleToken>,
        sha256: SHA256Hex,
        size_bytes: CanonicalU64Decimal,
    ) -> Result<Self, EvaluationContractError> {
        if roles.is_empty()
            || roles
                .windows(2)
                .any(|pair| pair[0].as_str() >= pair[1].as_str())
        {
            return Err(EvaluationContractError(
                "artifact roles must be nonempty, unique and token-sorted",
            ));
        }
        Ok(Self {
            path,
            roles,
            sha256,
            size_bytes,
        })
    }
    pub fn path(&self) -> &BundleRelativePath {
        &self.path
    }
    pub fn roles(&self) -> &[ArtifactRoleToken] {
        &self.roles
    }
    pub fn sha256(&self) -> &SHA256Hex {
        &self.sha256
    }
    pub fn size_bytes(&self) -> CanonicalU64Decimal {
        self.size_bytes
    }
}
impl TryFrom<ManifestEntryWire> for ArtifactManifestEntryV0_1 {
    type Error = EvaluationContractError;
    fn try_from(value: ManifestEntryWire) -> Result<Self, Self::Error> {
        Self::new(value.path, value.roles, value.sha256, value.size_bytes)
    }
}

/// Validated manifest structure, with unique paths in unsigned UTF-8 order.
/// JCS serialization, manifest hashing and artifact resolution are not implemented here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ArtifactManifestV0_1 {
    schema_version: ArtifactManifestSchemaVersion,
    entries: Vec<ArtifactManifestEntryV0_1>,
}
deserialize_object_via!(ArtifactManifestV0_1, ManifestWire);
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestWire {
    schema_version: ArtifactManifestSchemaVersion,
    entries: Vec<ArtifactManifestEntryV0_1>,
}
impl ArtifactManifestV0_1 {
    pub fn new(entries: Vec<ArtifactManifestEntryV0_1>) -> Result<Self, EvaluationContractError> {
        if entries
            .windows(2)
            .any(|pair| pair[0].path() >= pair[1].path())
        {
            return Err(EvaluationContractError(
                "manifest paths must be unique and UTF-8 sorted",
            ));
        }
        Ok(Self {
            schema_version: ArtifactManifestSchemaVersion::V0_1,
            entries,
        })
    }
    pub fn schema_version(&self) -> ArtifactManifestSchemaVersion {
        self.schema_version
    }
    pub fn entries(&self) -> &[ArtifactManifestEntryV0_1] {
        &self.entries
    }
}
impl TryFrom<ManifestWire> for ArtifactManifestV0_1 {
    type Error = EvaluationContractError;
    fn try_from(value: ManifestWire) -> Result<Self, Self::Error> {
        let ArtifactManifestSchemaVersion::V0_1 = value.schema_version;
        Self::new(value.entries)
    }
}

/// Closed replay locator union (§24.4.1). Decoding does not resolve/authenticate bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum ImmutableArtifactRefV0_1 {
    GitCommit {
        repository_id: IdentifierString,
        commit_sha: RepositoryCommitHex,
        manifest_path: BundleRelativePath,
    },
    ContentAddressed {
        locator: String,
        content_address: String,
    },
    SyntheticFixture {
        fixture_id: IdentifierString,
        manifest_sha256: SHA256Hex,
    },
}

/// A structurally checked binding. Content-address syntax and resolver support are
/// resolver-owned: this boundary requires a separate nonempty pin, never a bare URL.
/// No successful replay/authentication status follows from constructing this value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ArtifactBindingWireV0_1")]
pub struct ArtifactBindingV0_1 {
    artifact_semantic_identity: ArtifactSemanticIdentityV0_1,
    immutable_ref: ImmutableArtifactRefV0_1,
}
closed_wire! {
pub struct ArtifactBindingWireV0_1 {
    pub artifact_semantic_identity: ArtifactSemanticIdentityV0_1,
    pub immutable_ref: ImmutableArtifactRefV0_1,
}
}
impl ArtifactBindingV0_1 {
    pub fn new(
        artifact_semantic_identity: ArtifactSemanticIdentityV0_1,
        immutable_ref: ImmutableArtifactRefV0_1,
    ) -> Result<Self, EvaluationContractError> {
        match &immutable_ref {
            ImmutableArtifactRefV0_1::SyntheticFixture {
                manifest_sha256, ..
            } if manifest_sha256 != &artifact_semantic_identity.manifest_sha256 => {
                return Err(EvaluationContractError("fixture manifest digest mismatch"));
            }
            ImmutableArtifactRefV0_1::ContentAddressed {
                locator,
                content_address,
            } => {
                // A Phase-A reference is structural evidence only. Do not invent a
                // universal content-address grammar or treat a locator as authentication.
                for value in [locator, content_address] {
                    if value.is_empty()
                        || value.chars().any(char::is_whitespace)
                        || value.chars().any(char::is_control)
                        || value.starts_with(['/', '\\', '.'])
                        || value.contains('\\')
                        || value.to_ascii_lowercase().starts_with("file:")
                        || (value.as_bytes().get(1) == Some(&b':')
                            && value.as_bytes()[0].is_ascii_alphabetic())
                    {
                        return Err(EvaluationContractError(
                            "invalid content-addressed reference",
                        ));
                    }
                }
                if content_address.contains("://") {
                    return Err(EvaluationContractError(
                        "content address must be a separate immutable pin",
                    ));
                }
            }
            _ => {}
        }
        Ok(Self {
            artifact_semantic_identity,
            immutable_ref,
        })
    }
    pub fn artifact_semantic_identity(&self) -> &ArtifactSemanticIdentityV0_1 {
        &self.artifact_semantic_identity
    }
    pub fn immutable_ref(&self) -> &ImmutableArtifactRefV0_1 {
        &self.immutable_ref
    }
}
impl TryFrom<ArtifactBindingWireV0_1> for ArtifactBindingV0_1 {
    type Error = EvaluationContractError;
    fn try_from(value: ArtifactBindingWireV0_1) -> Result<Self, Self::Error> {
        Self::new(value.artifact_semantic_identity, value.immutable_ref)
    }
}

closed_wire! {
/// Privacy/retention metadata belongs to replay provenance, never semantic identity.
pub struct PrivacyBoundArtifactRefV0_1 {
    pub artifact_binding: ArtifactBindingV0_1,
    pub snapshot_schema_id: IdentifierString,
    pub snapshot_schema_version: IdentifierString,
    pub privacy_classification: IdentifierString,
    pub retention_policy_id: IdentifierString,
    pub retention_policy_version: IdentifierString,
    pub content_kind: IdentifierString,
    pub resolution_state: super::ReplayResolutionState,
}
}

/// No non-null experiment assignment exists in E2.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExperimentReplayBindingV1 {}

closed_wire! {
pub struct ReplayProvenanceWireV0_1 {
    #[serde(deserialize_with = "super::required_nullable")]
    pub source_snapshot_ref: Option<PrivacyBoundArtifactRefV0_1>,
    pub planner_artifact: ArtifactBindingV0_1,
    pub generator_artifact: ArtifactBindingV0_1,
    #[serde(deserialize_with = "super::required_nullable")]
    pub generator_configuration_artifact: Option<ArtifactBindingV0_1>,
    pub product_eligibility_policy: ArtifactBindingV0_1,
    pub ranking_policy: ArtifactBindingV0_1,
    pub display_policy: ArtifactBindingV0_1,
    #[serde(deserialize_with = "super::required_nullable")]
    pub model_artifact: Option<ArtifactBindingV0_1>,
    #[serde(deserialize_with = "super::required_nullable")]
    pub experiment_assignment: Option<ExperimentReplayBindingV1>,
}
}

/// E2 structural replay contract, not a resolver or proof of DecisionInput equality.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    try_from = "ReplayProvenanceWireV0_1",
    into = "ReplayProvenanceWireV0_1"
)]
pub struct ReplayProvenanceV0_1(ReplayProvenanceWireV0_1);
impl ReplayProvenanceV0_1 {
    pub fn as_wire(&self) -> &ReplayProvenanceWireV0_1 {
        &self.0
    }
}
impl TryFrom<ReplayProvenanceWireV0_1> for ReplayProvenanceV0_1 {
    type Error = EvaluationContractError;
    fn try_from(value: ReplayProvenanceWireV0_1) -> Result<Self, Self::Error> {
        if value.source_snapshot_ref.is_none() || value.model_artifact.is_some() {
            return Err(EvaluationContractError(
                "E2 requires a source snapshot and no model",
            ));
        }
        Ok(Self(value))
    }
}
impl From<ReplayProvenanceV0_1> for ReplayProvenanceWireV0_1 {
    fn from(value: ReplayProvenanceV0_1) -> Self {
        value.0
    }
}
