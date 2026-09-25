use cerebri_preferences::*;
use cerebri_temporal::{Instant, TimeDelta};
use cerebri_types::FactId;
use serde_json::{Value, json};

fn at(seconds: i64) -> Instant {
    Instant::from_timestamp(seconds, 0).unwrap()
}
fn evidence(source: PreferenceSource, seconds: i64) -> PreferenceEvidence {
    PreferenceEvidence {
        source,
        preferred_start: at(seconds),
        evidence: vec![],
    }
}
fn decode(value: Value) -> Result<RankingFeatureSet, serde_json::Error> {
    // Exercise the actual JSON wire, not only Value deserialization or constructors.
    serde_json::from_str(&value.to_string())
}
fn absent_preference() -> Value {
    json!({"schema_version":{"major":0,"minor":1},
        "preferred_start_distance_seconds":null,"preferred_start_source":null,
        "mutation_count":0,"shift_seconds":0})
}

#[test]
fn source_precedence_time_ties_duplicates_and_evidence_permutations() {
    use PreferenceSource::*;
    let sources = [
        ExplicitCurrentRequest,
        SessionContext,
        PersonalLearned,
        GlobalLearned,
        Default,
    ];
    // Every pair, including the requested explicit/personal, session/global and personal/global.
    for (rank, higher) in sources.iter().enumerate() {
        for lower in &sources[rank + 1..] {
            let mut entries = [
                evidence(*lower, 0),
                evidence(*higher, 3600),
                evidence(*higher, 7200),
                evidence(*higher, 3600),
            ];
            entries[1].evidence = vec![FactId::new("b").unwrap(), FactId::new("a").unwrap()];
            // All 24 permutations, including exact source/time duplicates with distinct evidence.
            for a in 0..4 {
                for b in 0..4 {
                    for c in 0..4 {
                        for d in 0..4 {
                            let indices = [a, b, c, d];
                            if (0..4).any(|i| indices[i + 1..].contains(&indices[i])) {
                                continue;
                            }
                            let profile = PreferenceProfile {
                                preferences: indices.map(|i| entries[i].clone()).to_vec(),
                            };
                            let features = profile.ranking_features(at(0), 1, None);
                            assert_eq!(
                                profile.preferred_start().unwrap().preferred_start,
                                at(3600)
                            );
                            assert_eq!(features.preferred_start_source(), Some(*higher));
                            assert_eq!(features.preferred_start_distance_seconds(), Some(3600));
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn whole_second_distance_and_shift_preserve_missingness_and_quantization() {
    let empty = PreferenceProfile::default();
    let no_signal = empty.ranking_features(at(0), 0, None);
    assert_eq!(no_signal.preferred_start_distance_seconds(), None);
    assert_eq!(no_signal.preferred_start_source(), None);
    assert_eq!(
        serde_json::to_value(&no_signal).unwrap(),
        absent_preference()
    );
    let profile = PreferenceProfile {
        preferences: vec![evidence(PreferenceSource::ExplicitCurrentRequest, 0)],
    };
    for (millis, seconds) in [
        (0, 0),
        (800, 0),
        (-800, 0),
        (1000, 1),
        (-1000, 1),
        (12345, 12),
        (-12345, 12),
    ] {
        let start = at(0) + TimeDelta::milliseconds(millis);
        let feature = profile.ranking_features(start, 1, Some(at(0)));
        assert_eq!(feature.preferred_start_distance_seconds(), Some(seconds));
        assert_eq!(
            feature.preferred_start_source(),
            Some(PreferenceSource::ExplicitCurrentRequest)
        );
        assert_eq!(feature.shift_seconds(), seconds);
        assert_eq!(feature.mutation_count(), 1);
        assert_eq!(
            profile
                .features(start)
                .distance_from_preferred_start_seconds,
            seconds
        );
        assert_ne!(feature, no_signal);
    }
    // Current chrono domain extremes remain representable, without clamping or new errors.
    let extreme = PreferenceProfile {
        preferences: vec![PreferenceEvidence {
            source: PreferenceSource::Default,
            preferred_start: Instant::MIN_UTC,
            evidence: vec![],
        }],
    }
    .ranking_features(Instant::MAX_UTC, 1, Some(Instant::MIN_UTC));
    let magnitude = (Instant::MAX_UTC - Instant::MIN_UTC)
        .num_seconds()
        .unsigned_abs();
    assert_eq!(extreme.preferred_start_distance_seconds(), Some(magnitude));
    assert_eq!(extreme.shift_seconds(), magnitude);
}

#[test]
fn required_nullable_json_presence_truth_table_and_option_collapse_regression() {
    #[derive(serde::Deserialize)]
    struct OrdinaryOption {
        preferred_start_distance_seconds: Option<u64>,
    }
    assert!(
        serde_json::from_str::<OrdinaryOption>("{}")
            .unwrap()
            .preferred_start_distance_seconds
            .is_none()
    );
    // Absent, null and value on each axis: only null/null and value/value are valid.
    for distance in [None, Some(Value::Null), Some(json!(0))] {
        for source in [
            None,
            Some(Value::Null),
            Some(json!("ExplicitCurrentRequest")),
        ] {
            let mut wire = absent_preference();
            let map = wire.as_object_mut().unwrap();
            map.remove("preferred_start_distance_seconds");
            map.remove("preferred_start_source");
            if let Some(value) = &distance {
                map.insert("preferred_start_distance_seconds".into(), value.clone());
            }
            if let Some(value) = &source {
                map.insert("preferred_start_source".into(), value.clone());
            }
            let valid =
                matches!((&distance, &source), (Some(d), Some(s)) if d.is_null() == s.is_null());
            let decoded = decode(wire.clone());
            assert_eq!(decoded.is_ok(), valid, "{wire}");
            if let Ok(domain) = decoded {
                assert_eq!(
                    domain.preferred_start_distance_seconds(),
                    distance.as_ref().and_then(Value::as_u64)
                );
                assert_eq!(serde_json::to_value(&domain).unwrap(), wire);
            }
        }
    }
}

#[test]
fn wire_rejects_missing_unknown_duplicate_malformed_and_incompatible_fields() {
    for key in [
        "schema_version",
        "preferred_start_distance_seconds",
        "preferred_start_source",
        "mutation_count",
        "shift_seconds",
    ] {
        let mut wire = absent_preference();
        wire.as_object_mut().unwrap().remove(key);
        assert!(
            decode(wire)
                .unwrap_err()
                .to_string()
                .contains("missing field"),
            "{key}"
        );
    }
    for version in [
        json!({"major":0,"minor":0}),
        json!({"major":0,"minor":2}),
        json!({"major":1,"minor":1}),
        json!({"major":0}),
        json!({"major":0,"minor":1,"extra":0}),
        Value::Null,
    ] {
        let mut wire = absent_preference();
        wire["schema_version"] = version;
        assert!(decode(wire).is_err());
    }
    for (key, value) in [
        ("preferred_start_distance_seconds", json!(-1)),
        ("preferred_start_distance_seconds", json!(0.5)),
        ("preferred_start_distance_seconds", json!("0")),
        ("preferred_start_distance_seconds", json!(true)),
        ("preferred_start_source", json!("Unknown")),
        ("preferred_start_source", json!(0)),
        ("mutation_count", json!(4294967296u64)),
        ("mutation_count", Value::Null),
        ("shift_seconds", json!(-1)),
        ("extra", json!(0)),
    ] {
        let mut wire = absent_preference();
        wire[key] = value;
        assert!(decode(wire).is_err(), "{key}");
    }
    let wire = absent_preference().to_string();
    let duplicate = wire.replacen('{', "{\"preferred_start_source\":null,", 1);
    assert!(serde_json::from_str::<RankingFeatureSet>(&duplicate).is_err());
    let overflow = wire.replace(
        "\"shift_seconds\":0",
        "\"shift_seconds\":18446744073709551616",
    );
    assert!(serde_json::from_str::<RankingFeatureSet>(&overflow).is_err());
}

#[test]
fn all_sources_and_numeric_boundaries_roundtrip_deterministically() {
    use PreferenceSource::*;
    for source in [
        ExplicitCurrentRequest,
        SessionContext,
        PersonalLearned,
        GlobalLearned,
        Default,
    ] {
        for distance in [0, 3600, u64::MAX] {
            let wire = json!({"schema_version":{"major":0,"minor":1},
                "preferred_start_distance_seconds":distance,"preferred_start_source":source,
                "mutation_count":u32::MAX,"shift_seconds":u64::MAX});
            let domain = decode(wire.clone()).unwrap();
            assert_eq!(domain.schema_version(), RANKING_FEATURE_SCHEMA_V0_1);
            assert_eq!(domain.preferred_start_source(), Some(source));
            assert_eq!(domain.preferred_start_distance_seconds(), Some(distance));
            assert_eq!(serde_json::to_value(&domain).unwrap(), wire);
            let encoded = serde_json::to_string(&domain).unwrap();
            assert_eq!(
                serde_json::to_string(
                    &serde_json::from_str::<RankingFeatureSet>(&encoded).unwrap()
                )
                .unwrap(),
                encoded
            );
        }
    }
}
