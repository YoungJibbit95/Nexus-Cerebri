//! CanonicalValueV1 semantic normalization and the final, integer-free JCS writer.
use super::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Numeric graph alias, encoded with exactly six decimal digits (§13).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CanonicalAlias(String);
impl CanonicalAlias {
    pub fn new(index: u32) -> Result<Self, EvaluationContractError> {
        if index > 999_999 {
            return Err(EvaluationContractError("canonical alias out of range"));
        }
        Ok(Self(format!("v{index:06}")))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl TryFrom<String> for CanonicalAlias {
    type Error = EvaluationContractError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 7
            || !value.starts_with('v')
            || !value.as_bytes()[1..].iter().all(u8::is_ascii_digit)
        {
            return Err(EvaluationContractError("invalid canonical alias"));
        }
        Ok(Self(value))
    }
}
impl From<CanonicalAlias> for String {
    fn from(value: CanonicalAlias) -> Self {
        value.0
    }
}

/// A member of the architecture's explicit token registry, never a Rust enum spelling.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CanonicalTokenV1(String);
impl CanonicalTokenV1 {
    pub fn new(value: impl Into<String>) -> Result<Self, EvaluationContractError> {
        let value = value.into();
        if !super::tokens::is_canonical_token(&value) {
            return Err(EvaluationContractError("unknown canonical token"));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl TryFrom<String> for CanonicalTokenV1 {
    type Error = EvaluationContractError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl From<CanonicalTokenV1> for String {
    fn from(value: CanonicalTokenV1) -> Self {
        value.0
    }
}

closed_wire! {
#[derive(PartialOrd, Ord)]
pub struct CanonicalAttributeV1 {
    pub key: String,
    pub value: CanonicalValueV1,
}
}
closed_wire! {
#[derive(PartialOrd, Ord)]
pub struct CanonicalMultiplicityV1 {
    pub value: CanonicalValueV1,
    pub multiplicity: CanonicalU64Decimal,
}
}

/// Construction input. Use `CanonicalValueV1::new` to normalize collections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "t", deny_unknown_fields)]
pub enum CanonicalValueKindV1 {
    #[serde(rename = "NONE")]
    None {},
    #[serde(rename = "BOOL")]
    Bool { v: bool },
    #[serde(rename = "I64")]
    I64 { v: CanonicalI64Decimal },
    #[serde(rename = "U64")]
    U64 { v: CanonicalU64Decimal },
    #[serde(rename = "TEXT")]
    Text { v: String },
    #[serde(rename = "TOKEN")]
    Token { v: CanonicalTokenV1 },
    #[serde(rename = "INSTANT")]
    Instant {
        s: CanonicalI64Decimal,
        ns: NineDigitNanoseconds,
    },
    #[serde(rename = "DATE")]
    Date { v: CanonicalDate },
    #[serde(rename = "LOCAL_TIME")]
    LocalTime { v: CanonicalLocalTime },
    #[serde(rename = "SHA256")]
    Sha256 { v: SHA256Hex },
    #[serde(rename = "REF")]
    Ref { v: CanonicalAlias },
    #[serde(rename = "LIST")]
    List { v: Vec<CanonicalValueV1> },
    #[serde(rename = "SET")]
    Set { v: Vec<CanonicalValueV1> },
    #[serde(rename = "MULTISET")]
    Multiset { v: Vec<CanonicalMultiplicityV1> },
    #[serde(rename = "OBJECT")]
    Object { v: Vec<CanonicalAttributeV1> },
}
impl CanonicalValueKindV1 {
    fn rank(&self) -> u8 {
        match self {
            Self::None {} => 0,
            Self::Bool { .. } => 1,
            Self::I64 { .. } => 2,
            Self::U64 { .. } => 3,
            Self::Text { .. } => 4,
            Self::Token { .. } => 5,
            Self::Instant { .. } => 6,
            Self::Date { .. } => 7,
            Self::LocalTime { .. } => 8,
            Self::Sha256 { .. } => 9,
            Self::Ref { .. } => 10,
            Self::List { .. } => 11,
            Self::Set { .. } => 12,
            Self::Multiset { .. } => 13,
            Self::Object { .. } => 14,
        }
    }
}

/// Immutable canonical value. Source constructors normalize; imported wire must
/// already have canonical collection order, uniqueness and positive multiplicities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct CanonicalValueV1(CanonicalValueKindV1);
impl CanonicalValueV1 {
    pub fn new(mut kind: CanonicalValueKindV1) -> Result<Self, EvaluationContractError> {
        match &mut kind {
            CanonicalValueKindV1::Set { v } => {
                v.sort();
                v.dedup();
            }
            CanonicalValueKindV1::Object { v } => {
                v.sort_by(|a, b| a.key.cmp(&b.key));
                if v.windows(2).any(|w| w[0].key == w[1].key) {
                    return Err(EvaluationContractError("duplicate canonical object key"));
                }
            }
            CanonicalValueKindV1::Multiset { v } => {
                v.sort_by(|a, b| a.value.cmp(&b.value));
                let mut entries: Vec<CanonicalMultiplicityV1> = Vec::new();
                for entry in std::mem::take(v) {
                    if entry.multiplicity.value() == 0 {
                        return Err(EvaluationContractError("zero canonical multiplicity"));
                    }
                    if let Some(last) = entries.last_mut().filter(|last| last.value == entry.value)
                    {
                        last.multiplicity = CanonicalU64Decimal::new(
                            last.multiplicity
                                .value()
                                .checked_add(entry.multiplicity.value())
                                .ok_or(EvaluationContractError(
                                    "canonical multiplicity overflow",
                                ))?,
                        )?;
                    } else {
                        entries.push(entry);
                    }
                }
                *v = entries;
            }
            _ => {}
        }
        Ok(Self(kind))
    }
    pub fn kind(&self) -> &CanonicalValueKindV1 {
        &self.0
    }
}
impl<'de> Deserialize<'de> for CanonicalValueV1 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let wire = super::object_only::<D, CanonicalValueKindV1>(deserializer)?;
        let canonical = Self::new(wire.clone()).map_err(serde::de::Error::custom)?;
        if canonical.0 != wire {
            return Err(serde::de::Error::custom(
                "noncanonical collection order or duplicates",
            ));
        }
        Ok(canonical)
    }
}
impl PartialOrd for CanonicalValueV1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CanonicalValueV1 {
    fn cmp(&self, other: &Self) -> Ordering {
        use CanonicalValueKindV1::*;
        let rank = self.0.rank().cmp(&other.0.rank());
        if rank != Ordering::Equal {
            return rank;
        }
        match (&self.0, &other.0) {
            (None {}, None {}) => Ordering::Equal,
            (Bool { v: a }, Bool { v: b }) => a.cmp(b),
            (I64 { v: a }, I64 { v: b }) => a.cmp(b),
            (U64 { v: a }, U64 { v: b }) => a.cmp(b),
            (Text { v: a }, Text { v: b }) => a.cmp(b),
            (Token { v: a }, Token { v: b }) => a.cmp(b),
            (Instant { s: a, ns: an }, Instant { s: b, ns: bn }) => (a, an).cmp(&(b, bn)),
            (Date { v: a }, Date { v: b }) => a.cmp(b),
            (LocalTime { v: a }, LocalTime { v: b }) => a.cmp(b),
            (Sha256 { v: a }, Sha256 { v: b }) => a.cmp(b),
            (Ref { v: a }, Ref { v: b }) => a.cmp(b),
            (List { v: a }, List { v: b }) | (Set { v: a }, Set { v: b }) => a.cmp(b),
            (Multiset { v: a }, Multiset { v: b }) => a.cmp(b),
            (Object { v: a }, Object { v: b }) => a.cmp(b),
            _ => unreachable!("equal explicit canonical ranks imply equal variants"),
        }
    }
}

/// Final RFC-8785 string/array/object/boolean writer for the canonical payload
/// subset. All contract integers are decimal *strings*. Numbers and null reject;
/// this function does not normalize semantic sets, graphs or source identities.
pub fn canonical_json_bytes<T: Serialize + ?Sized>(
    value: &T,
) -> Result<Vec<u8>, EvaluationContractError> {
    let value = serde_json::to_value(value)
        .map_err(|_| EvaluationContractError("canonical JSON serialization failed"))?;
    let mut bytes = Vec::new();
    write_json(&value, &mut bytes)?;
    Ok(bytes)
}
fn write_json(value: &serde_json::Value, out: &mut Vec<u8>) -> Result<(), EvaluationContractError> {
    use serde_json::Value;
    match value {
        Value::Null | Value::Number(_) => {
            return Err(EvaluationContractError(
                "canonical payload forbids null and JSON numbers",
            ));
        }
        Value::Bool(value) => out.extend_from_slice(if *value { b"true" } else { b"false" }),
        Value::String(value) => serde_json::to_writer(out, value)
            .map_err(|_| EvaluationContractError("canonical string serialization failed"))?,
        Value::Array(values) => {
            out.push(b'[');
            for (index, value) in values.iter().enumerate() {
                if index != 0 {
                    out.push(b',');
                }
                write_json(value, out)?;
            }
            out.push(b']');
        }
        Value::Object(values) => {
            // JCS sorts JSON object names by UTF-16 code units. Semantic OBJECT
            // entry keys are string *values* in an array, ordered by UTF-8 (§5.6).
            // All fixed contract member names are ASCII: both orders coincide.
            let mut fields: Vec<_> = values.iter().collect();
            fields.sort_by(|(a, _), (b, _)| a.encode_utf16().cmp(b.encode_utf16()));
            out.push(b'{');
            for (index, (key, value)) in fields.into_iter().enumerate() {
                if index != 0 {
                    out.push(b',');
                }
                serde_json::to_writer(&mut *out, key)
                    .map_err(|_| EvaluationContractError("canonical key serialization failed"))?;
                out.push(b':');
                write_json(value, out)?;
            }
            out.push(b'}');
        }
    }
    Ok(())
}
