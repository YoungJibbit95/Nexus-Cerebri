//! Exact scalar wire grammars from the consolidated contract, sections 3–5.
use super::EvaluationContractError;
use cerebri_temporal::{Instant, LocalDate, LocalTime};
use serde::{Deserialize, Serialize};

macro_rules! validated_string {
    ($name:ident, $description:literal, $validate:expr) => {
        #[doc = $description]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);
        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, EvaluationContractError> {
                let value = value.into();
                if !($validate)(&value) {
                    return Err(EvaluationContractError(concat!(
                        "invalid ",
                        stringify!($name)
                    )));
                }
                Ok(Self(value))
            }
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
        impl TryFrom<String> for $name {
            type Error = EvaluationContractError;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

fn identifier(value: &str) -> bool {
    (1..=128).contains(&value.len())
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
}

validated_string!(
    RepositoryCommitHex,
    "Full lowercase Git SHA-1 or SHA-256 commit identity; no abbreviated revision.",
    |value: &str| {
        matches!(value.len(), 40 | 64)
            && value
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    }
);

validated_string!(
    IdentifierString,
    "Bounded ASCII identity; case-sensitive and never normalized.",
    identifier
);
validated_string!(
    RuleIdV1,
    "Rule identity, distinct from other identifiers (contract section 32.1).",
    identifier
);
validated_string!(
    SHA256Hex,
    "Exactly 64 lowercase ASCII hex digits; this type does not verify a digest.",
    |value: &str| {
        value.len() == 64
            && value
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    }
);

macro_rules! decimal {
    ($name:ident, $inner:ty, $description:literal, $allowed:expr) => {
        #[doc = $description]
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        #[serde(try_from = "String", into = "String")]
        pub struct $name($inner);
        impl $name {
            pub fn new(value: $inner) -> Result<Self, EvaluationContractError> {
                if !($allowed)(value) {
                    return Err(EvaluationContractError(concat!(
                        "invalid ",
                        stringify!($name)
                    )));
                }
                Ok(Self(value))
            }
            pub fn value(self) -> $inner {
                self.0
            }
        }
        impl TryFrom<String> for $name {
            type Error = EvaluationContractError;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                let number: $inner = value
                    .parse()
                    .map_err(|_| EvaluationContractError(concat!("invalid ", stringify!($name))))?;
                // Numeric parse alone admits alternate spellings such as +1 or 01.
                if number.to_string() != value {
                    return Err(EvaluationContractError(concat!(
                        "noncanonical ",
                        stringify!($name)
                    )));
                }
                Self::new(number)
            }
        }
        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0.to_string()
            }
        }
    };
}

decimal!(
    CanonicalI64Decimal,
    i64,
    "Canonical signed i64 decimal JSON string; numeric comparison.",
    |_: i64| true
);
decimal!(
    CanonicalU64Decimal,
    u64,
    "Canonical unsigned u64 decimal JSON string; numeric comparison.",
    |_: u64| true
);
decimal!(
    CanonicalPositiveSeconds,
    u64,
    "Strictly positive canonical u64 seconds; no floating point.",
    |value: u64| value > 0
);

/// Exactly nine ASCII digits, including leading zeroes, with range 0..=999999999.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct NineDigitNanoseconds(u32);
impl NineDigitNanoseconds {
    pub fn new(value: u32) -> Result<Self, EvaluationContractError> {
        if value >= 1_000_000_000 {
            return Err(EvaluationContractError(
                "nanoseconds exceed nine-digit range",
            ));
        }
        Ok(Self(value))
    }
    pub fn value(self) -> u32 {
        self.0
    }
}
impl TryFrom<String> for NineDigitNanoseconds {
    type Error = EvaluationContractError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 9 || !value.bytes().all(|b| b.is_ascii_digit()) {
            return Err(EvaluationContractError(
                "expected exactly nine ASCII digits",
            ));
        }
        Self::new(
            value
                .parse()
                .map_err(|_| EvaluationContractError("invalid nanoseconds"))?,
        )
    }
}
impl From<NineDigitNanoseconds> for String {
    fn from(value: NineDigitNanoseconds) -> Self {
        format!("{:09}", value.0)
    }
}

/// Canonical seconds/nanoseconds pair. Its i64 domain is not limited by Chrono.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalInstantV1 {
    pub unix_seconds: CanonicalI64Decimal,
    pub nanoseconds: NineDigitNanoseconds,
}
impl TryFrom<Instant> for CanonicalInstantV1 {
    type Error = EvaluationContractError;
    fn try_from(value: Instant) -> Result<Self, Self::Error> {
        Ok(Self {
            unix_seconds: CanonicalI64Decimal::new(value.timestamp())?,
            // Fail closed on a source leap-second representation outside the grammar.
            nanoseconds: NineDigitNanoseconds::new(value.timestamp_subsec_nanos())?,
        })
    }
}

/// Typed OBJECT time range, with exactly end/start INSTANT entries in that order.
/// This validates the canonical value shape, not a JCS byte stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "TimeRangeValueWire", into = "TimeRangeValueWire")]
pub struct CanonicalTimeRangeValueV1 {
    start: CanonicalInstantV1,
    end: CanonicalInstantV1,
}
impl CanonicalTimeRangeValueV1 {
    pub fn new(
        start: CanonicalInstantV1,
        end: CanonicalInstantV1,
    ) -> Result<Self, EvaluationContractError> {
        if start >= end {
            return Err(EvaluationContractError(
                "canonical range start must precede end",
            ));
        }
        Ok(Self { start, end })
    }
    pub fn start(self) -> CanonicalInstantV1 {
        self.start
    }
    pub fn end(self) -> CanonicalInstantV1 {
        self.end
    }
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "t", deny_unknown_fields)]
enum TimeRangeValueWire {
    #[serde(rename = "OBJECT")]
    Object { v: [TimeRangeEntryWire; 2] },
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TimeRangeEntryWire {
    key: TimeRangeKey,
    value: InstantValueWire,
}
#[derive(PartialEq, Eq, Serialize, Deserialize)]
enum TimeRangeKey {
    #[serde(rename = "end")]
    End,
    #[serde(rename = "start")]
    Start,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "t", deny_unknown_fields)]
enum InstantValueWire {
    #[serde(rename = "INSTANT")]
    Instant {
        s: CanonicalI64Decimal,
        ns: NineDigitNanoseconds,
    },
}
impl From<CanonicalInstantV1> for InstantValueWire {
    fn from(value: CanonicalInstantV1) -> Self {
        Self::Instant {
            s: value.unix_seconds,
            ns: value.nanoseconds,
        }
    }
}
impl From<InstantValueWire> for CanonicalInstantV1 {
    fn from(value: InstantValueWire) -> Self {
        let InstantValueWire::Instant { s, ns } = value;
        Self {
            unix_seconds: s,
            nanoseconds: ns,
        }
    }
}
impl TryFrom<TimeRangeValueWire> for CanonicalTimeRangeValueV1 {
    type Error = EvaluationContractError;
    fn try_from(value: TimeRangeValueWire) -> Result<Self, Self::Error> {
        let TimeRangeValueWire::Object { v: [end, start] } = value;
        if end.key != TimeRangeKey::End || start.key != TimeRangeKey::Start {
            return Err(EvaluationContractError(
                "expected unique end/start entries in canonical order",
            ));
        }
        Self::new(start.value.into(), end.value.into())
    }
}
impl From<CanonicalTimeRangeValueV1> for TimeRangeValueWire {
    fn from(value: CanonicalTimeRangeValueV1) -> Self {
        Self::Object {
            v: [
                TimeRangeEntryWire {
                    key: TimeRangeKey::End,
                    value: value.end.into(),
                },
                TimeRangeEntryWire {
                    key: TimeRangeKey::Start,
                    value: value.start.into(),
                },
            ],
        }
    }
}

fn date(value: &str) -> bool {
    value.len() == 10
        && value.bytes().enumerate().all(|(i, b)| match i {
            4 | 7 => b == b'-',
            _ => b.is_ascii_digit(),
        })
        && LocalDate::parse_from_str(value, "%Y-%m-%d").is_ok()
}
fn local_time(value: &str) -> bool {
    value.len() == 18
        && value.bytes().enumerate().all(|(i, b)| match i {
            2 | 5 => b == b':',
            8 => b == b'.',
            _ => b.is_ascii_digit(),
        })
        && LocalTime::parse_from_str(value, "%H:%M:%S%.9f").is_ok()
}
validated_string!(
    CanonicalDate,
    "Exact YYYY-MM-DD, validated using the existing temporal calendar.",
    date
);
validated_string!(
    CanonicalLocalTime,
    "Exact HH:MM:SS.NNNNNNNNN local time; no timezone conversion.",
    local_time
);

impl TryFrom<LocalDate> for CanonicalDate {
    type Error = EvaluationContractError;
    fn try_from(value: LocalDate) -> Result<Self, Self::Error> {
        Self::new(value.format("%Y-%m-%d").to_string())
    }
}
impl TryFrom<LocalTime> for CanonicalLocalTime {
    type Error = EvaluationContractError;
    fn try_from(value: LocalTime) -> Result<Self, Self::Error> {
        Self::new(value.format("%H:%M:%S%.9f").to_string())
    }
}

/// Canonical semantic optionality; distinct from the required-nullable wire convention.
/// The concrete payload type must itself implement its closed canonical contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", deny_unknown_fields)]
pub enum CanonicalOptionalV1<T> {
    #[serde(rename = "NONE")]
    None {},
    #[serde(rename = "SOME")]
    Some { value: T },
}
