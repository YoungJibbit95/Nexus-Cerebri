//! Import syntax only: retain duplicate-key rejection before typed component decoding.
use serde::Deserialize;

pub(super) struct CanonicalDigestJson<const ALLOW_BOOL: bool>(pub serde_json::Value);
impl<'de, const ALLOW_BOOL: bool> Deserialize<'de> for CanonicalDigestJson<ALLOW_BOOL> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor<const ALLOW_BOOL: bool>;
        impl<'de, const ALLOW_BOOL: bool> serde::de::Visitor<'de> for Visitor<ALLOW_BOOL> {
            type Value = CanonicalDigestJson<ALLOW_BOOL>;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a canonical digest value without numbers, null or duplicate keys")
            }
            fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Self::Value, E> {
                if ALLOW_BOOL {
                    Ok(CanonicalDigestJson(serde_json::Value::Bool(v)))
                } else {
                    Err(E::custom("boolean outside DecisionInput grammar"))
                }
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(CanonicalDigestJson(serde_json::Value::String(v.to_owned())))
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> Result<Self::Value, A::Error> {
                let mut values = Vec::new();
                while let Some(v) = seq.next_element::<CanonicalDigestJson<ALLOW_BOOL>>()? {
                    values.push(v.0);
                }
                Ok(CanonicalDigestJson(serde_json::Value::Array(values)))
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                let mut values = serde_json::Map::new();
                while let Some(key) = map.next_key::<String>()? {
                    if values.contains_key(&key) {
                        return Err(serde::de::Error::custom(
                            "duplicate canonical digest member",
                        ));
                    }
                    values.insert(key, map.next_value::<CanonicalDigestJson<ALLOW_BOOL>>()?.0);
                }
                Ok(CanonicalDigestJson(serde_json::Value::Object(values)))
            }
        }
        deserializer.deserialize_any(Visitor::<ALLOW_BOOL>)
    }
}
