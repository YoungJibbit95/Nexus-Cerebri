//! Structural artifact identity/manifest contracts; no resolver or digest computation.
use super::{
    ArtifactClassToken, ArtifactManifestSchemaVersion, ArtifactRoleToken, CanonicalU64Decimal,
    EvaluationContractError, IdentifierString, SHA256Hex,
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

/// Semantic identity only. Structural validity does not authenticate referenced bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactSemanticIdentityV0_1 {
    pub artifact_class: ArtifactClassToken,
    pub artifact_id: IdentifierString,
    pub artifact_version: IdentifierString,
    pub manifest_schema_version: IdentifierString,
    pub manifest_sha256: SHA256Hex,
}

/// One immutable bundle entry, with nonempty, unique, token-sorted roles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ManifestEntryWire")]
pub struct ArtifactManifestEntryV0_1 {
    path: BundleRelativePath,
    roles: Vec<ArtifactRoleToken>,
    sha256: SHA256Hex,
    size_bytes: CanonicalU64Decimal,
}
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ManifestWire")]
pub struct ArtifactManifestV0_1 {
    schema_version: ArtifactManifestSchemaVersion,
    entries: Vec<ArtifactManifestEntryV0_1>,
}
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
