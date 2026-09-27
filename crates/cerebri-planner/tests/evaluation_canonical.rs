use cerebri_planner::evaluation::*;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

fn decode<T: DeserializeOwned>(v: &str) -> T {
    serde_json::from_str(v).unwrap()
}
fn bad<T: DeserializeOwned>(v: &str) {
    assert!(serde_json::from_str::<T>(v).is_err(), "accepted {v}");
}
fn u(v: u64) -> CanonicalValueV1 {
    CanonicalValueV1::new(CanonicalValueKindV1::U64 {
        v: CanonicalU64Decimal::new(v).unwrap(),
    })
    .unwrap()
}

#[test]
fn canonical_value_total_order_is_typed_and_recursive() {
    let cases = [
        r#"{"t":"NONE"}"#,
        r#"{"t":"BOOL","v":false}"#,
        r#"{"t":"BOOL","v":true}"#,
        r#"{"t":"I64","v":"-10"}"#,
        r#"{"t":"I64","v":"-2"}"#,
        r#"{"t":"U64","v":"2"}"#,
        r#"{"t":"U64","v":"10"}"#,
        r#"{"t":"TEXT","v":"a"}"#,
        r#"{"t":"TOKEN","v":"EVENT"}"#,
        r#"{"t":"INSTANT","s":"-10","ns":"999999999"}"#,
        r#"{"t":"INSTANT","s":"-2","ns":"000000000"}"#,
        r#"{"t":"DATE","v":"2026-09-28"}"#,
        r#"{"t":"LOCAL_TIME","v":"09:30:00.000000000"}"#,
        r#"{"t":"SHA256","v":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#,
        r#"{"t":"REF","v":"v000001"}"#,
        r#"{"t":"LIST","v":[]}"#,
        r#"{"t":"LIST","v":[{"t":"U64","v":"2"}]}"#,
        r#"{"t":"LIST","v":[{"t":"U64","v":"10"}]}"#,
        r#"{"t":"SET","v":[]}"#,
        r#"{"t":"MULTISET","v":[]}"#,
        r#"{"t":"OBJECT","v":[]}"#,
    ];
    let values: Vec<CanonicalValueV1> = cases.iter().map(|s| decode(s)).collect();
    assert!(values.windows(2).all(|w| w[0] < w[1]));
    for value in &values {
        assert_eq!(
            &decode::<CanonicalValueV1>(
                &String::from_utf8(canonical_json_bytes(value).unwrap()).unwrap()
            ),
            value
        );
    }
}

#[test]
fn normalization_and_wire_rejection_are_distinct() {
    let set = CanonicalValueV1::new(CanonicalValueKindV1::Set {
        v: vec![u(10), u(2), u(2)],
    })
    .unwrap();
    assert_eq!(
        canonical_json_bytes(&set).unwrap(),
        br#"{"t":"SET","v":[{"t":"U64","v":"2"},{"t":"U64","v":"10"}]}"#
    );
    for s in [
        r#"{"t":"SET","v":[{"t":"U64","v":"2"},{"t":"U64","v":"2"}]}"#,
        r#"{"t":"SET","v":[{"t":"U64","v":"10"},{"t":"U64","v":"2"}]}"#,
        r#"{"t":"MULTISET","v":[{"value":{"t":"NONE"},"multiplicity":"0"}]}"#,
        r#"{"t":"MULTISET","v":[{"value":{"t":"NONE"},"multiplicity":"1"},{"value":{"t":"NONE"},"multiplicity":"2"}]}"#,
        r#"{"t":"OBJECT","v":[{"key":"x","value":{"t":"NONE"}},{"key":"x","value":{"t":"NONE"}}]}"#,
        r#"{"t":"OBJECT","v":[{"key":"z","value":{"t":"NONE"}},{"key":"a","value":{"t":"NONE"}}]}"#,
        r#"{"t":"OBJECT","v":[["x",{"t":"NONE"}]]}"#,
    ] {
        bad::<CanonicalValueV1>(s);
    }
    let entries = [1, 2].map(|n| CanonicalMultiplicityV1 {
        value: u(2),
        multiplicity: CanonicalU64Decimal::new(n).unwrap(),
    });
    let multi = CanonicalValueV1::new(CanonicalValueKindV1::Multiset {
        v: entries.to_vec(),
    })
    .unwrap();
    assert_eq!(
        canonical_json_bytes(&multi).unwrap(),
        br#"{"t":"MULTISET","v":[{"multiplicity":"3","value":{"t":"U64","v":"2"}}]}"#
    );
    assert!(
        CanonicalValueV1::new(CanonicalValueKindV1::Multiset {
            v: vec![
                CanonicalMultiplicityV1 {
                    value: u(1),
                    multiplicity: CanonicalU64Decimal::new(u64::MAX).unwrap()
                },
                CanonicalMultiplicityV1 {
                    value: u(1),
                    multiplicity: CanonicalU64Decimal::new(1).unwrap()
                },
            ]
        })
        .is_err()
    );
}

#[test]
fn malformed_canonical_values_reject() {
    for s in [
        r#"{"t":"NONE","v":null}"#,
        r#"{"t":"BOOL","v":"true"}"#,
        r#"{"t":"U64","v":"01"}"#,
        r#"{"t":"I64","v":"-0"}"#,
        r#"{"t":"U64","v":1}"#,
        r#"{"t":"I64","v":"9223372036854775808"}"#,
        r#"{"t":"TOKEN","v":"FUTURE_TOKEN"}"#,
        r#"{"t":"REF","v":"v1"}"#,
        r#"{"t":"INSTANT","s":"0","ns":"1"}"#,
        r#"{"t":"DATE","v":"2026-02-30"}"#,
        r#"{"t":"TEXT","v":null}"#,
        r#"{"t":"TEXT","v":"a","v":"b"}"#,
        r#"{"t":"TEXT","t":"TEXT","v":"a"}"#,
        r#"{"t":"TEXT","v":"a","x":0}"#,
        r#"{"t":"UNRECOGNIZED"}"#,
        r#"["TEXT","a"]"#,
    ] {
        bad::<CanonicalValueV1>(s);
    }
    for s in ["-9223372036854775808", "9223372036854775807"] {
        let _: CanonicalValueV1 = decode(&format!(r#"{{"t":"I64","v":"{s}"}}"#));
    }
    let _: CanonicalValueV1 = decode(r#"{"t":"U64","v":"18446744073709551615"}"#);
}

#[test]
fn jcs_is_only_the_final_writer_and_retains_string_bytes() {
    let value = json!({"z":"é\n\u{0001}/\\\"","a":true,"x":[]});
    assert_eq!(
        String::from_utf8(canonical_json_bytes(&value).unwrap()).unwrap(),
        "{\"a\":true,\"x\":[],\"z\":\"é\\n\\u0001/\\\\\\\"\"}"
    );
    // RFC 8785 UTF-16 member order differs from semantic UTF-8 string order.
    assert_eq!(
        String::from_utf8(
            canonical_json_bytes(&json!({"\u{e000}":true,"\u{10000}":false})).unwrap()
        )
        .unwrap(),
        "{\"𐀀\":false,\"\u{e000}\":true}"
    );
    assert!(canonical_json_bytes(&json!({"v":null})).is_err());
    assert!(canonical_json_bytes(&json!({"v":1})).is_err());
    assert!(canonical_json_bytes(&json!({"v":1.25})).is_err());
    assert_eq!(
        canonical_json_bytes(&json!(["z", "a", "a"])).unwrap(),
        br#"["z","a","a"]"#
    );
}

#[test]
fn every_constraint_variant_is_closed_and_uses_direct_fields() {
    let at = json!({"unix_seconds":"0","nanoseconds":"000000000"});
    let end = json!({"unix_seconds":"3600","nanoseconds":"000000000"});
    let mut forms = vec![
        json!({"kind":"NO_OVERLAP"}),
        json!({"kind":"RECURRENCE_RULE"}),
        json!({"kind":"EXPLICIT_DATE","date":"2026-09-28"}),
        json!({"kind":"DEPENDENCY_ORDER","object":"v000003"}),
        json!({"kind":"TIMEZONE_INTEGRITY","timezone":"Europe/Berlin"}),
        json!({"kind":"EXTERNAL_LOCK","provenance":"SYSTEM_FACT"}),
    ];
    for kind in ["EXPLICIT_TIME", "AVAILABILITY_WINDOW"] {
        forms.push(json!({"kind":kind,"range":{"start":at,"end":end}}));
    }
    for kind in ["EARLIEST_START", "LATEST_END", "DEADLINE"] {
        forms.push(json!({"kind":kind,"at":at}));
    }
    for kind in ["MIN_DURATION", "FIXED_DURATION", "REQUIRED_BUFFER"] {
        forms.push(json!({"kind":kind,"seconds":"1800"}));
    }
    assert_eq!(forms.len(), 14);
    for form in forms {
        let rule: CanonicalHardConstraintRuleV1 = serde_json::from_value(form.clone()).unwrap();
        assert_eq!(serde_json::to_value(rule).unwrap(), form);
        for key in form.as_object().unwrap().keys() {
            let mut missing = form.clone();
            missing.as_object_mut().unwrap().remove(key);
            assert!(serde_json::from_value::<CanonicalHardConstraintRuleV1>(missing).is_err());
            let mut null = form.clone();
            null[key] = Value::Null;
            assert!(serde_json::from_value::<CanonicalHardConstraintRuleV1>(null).is_err());
        }
        for key in ["value", "unexpected"] {
            let mut extra = form.clone();
            extra[key] = json!({});
            assert!(serde_json::from_value::<CanonicalHardConstraintRuleV1>(extra).is_err());
        }
    }
    let rule: CanonicalHardConstraintRuleV1 = decode(r#"{"seconds":"1800","kind":"MIN_DURATION"}"#);
    assert_eq!(
        canonical_json_bytes(&rule).unwrap(),
        br#"{"kind":"MIN_DURATION","seconds":"1800"}"#
    );
    let rule: CanonicalHardConstraintRuleV1 = decode(r#"{"kind":"NO_OVERLAP"}"#);
    assert_eq!(
        canonical_json_bytes(&rule).unwrap(),
        br#"{"kind":"NO_OVERLAP"}"#
    );
    for s in [
        r#"{"kind":"NO_OVERLAP","seconds":"1"}"#,
        r#"{"kind":"MIN_DURATION","seconds":"0"}"#,
        r#"{"kind":"MIN_DURATION","seconds":"01"}"#,
        r#"{"kind":"MIN_DURATION","seconds":1}"#,
        r#"{"kind":"NEW_RULE"}"#,
        r#"{"kind":"TIMEZONE_INTEGRITY","timezone":"Unknown/Zone"}"#,
        r#"{"kind":"NO_OVERLAP","kind":"NO_OVERLAP"}"#,
    ] {
        bad::<CanonicalHardConstraintRuleV1>(s);
    }
}

// External byte oracle copied verbatim from architecture (2), §17.5.5.
const SERIES: &str = r#"{"evidence_fact_refs":["v000002","v000003"],"provenance":"USER_EXPLICIT","rule":{"count":{"state":"SOME","value":"10"},"duration_seconds":"1800","fold_policy":"EARLIER","gap_policy":"SKIP","local_time":"09:30:00.000000000","pattern":{"every":"2","frequency":"WEEKLY","weekdays":["MONDAY","WEDNESDAY"]},"start_date":"2026-09-28","timezone":"Europe/Berlin","until":{"state":"SOME","value":"2026-12-31"}},"series":"v000010","state":"EXISTING"}"#;
#[test]
fn recurrence_exact_external_byte_oracles() {
    for source in [
        r#"{"every":"1","frequency":"DAILY"}"#,
        r#"{"every":"2","frequency":"WEEKLY","weekdays":["MONDAY","WEDNESDAY"]}"#,
    ] {
        let p: CanonicalRecurrencePatternV1 = decode(source);
        assert_eq!(canonical_json_bytes(&p).unwrap(), source.as_bytes());
    }
    let series: CanonicalTemporalSeriesV1 = decode(SERIES);
    assert_eq!(canonical_json_bytes(&series).unwrap(), SERIES.as_bytes());
}
#[test]
fn recurrence_and_series_are_closed_and_bounded() {
    for source in [
        r#"{"frequency":"DAILY","every":"0"}"#,
        r#"{"frequency":"DAILY","every":"65536"}"#,
        r#"{"frequency":"DAILY","every":"1","weekdays":[]}"#,
        r#"{"frequency":"MONTHLY","every":"1"}"#,
        r#"{"frequency":"WEEKLY","every":"1"}"#,
        r#"{"frequency":"WEEKLY","every":"1","weekdays":[]}"#,
        r#"{"frequency":"WEEKLY","every":"1","weekdays":["MONDAY","MONDAY"]}"#,
        r#"{"frequency":"WEEKLY","every":"1","weekdays":["WEDNESDAY","MONDAY"]}"#,
        r#"{"frequency":"WEEKLY","every":"1","weekdays":["FUNDAY"]}"#,
        r#"{"frequency":"DAILY","every":"1","value":{}}"#,
        r#"{"frequency":"DAILY","every":1}"#,
    ] {
        bad::<CanonicalRecurrencePatternV1>(source);
    }
    let _: CanonicalRecurrencePatternV1 = decode(r#"{"frequency":"DAILY","every":"65535"}"#);
    let template: Value = decode(SERIES);
    for path in [vec![], vec!["rule"], vec!["rule", "pattern"]] {
        let mut object = &template;
        for key in &path {
            object = &object[*key];
        }
        for key in object.as_object().unwrap().keys() {
            for null in [true, false] {
                let mut value = template.clone();
                let mut obj = &mut value;
                for part in &path {
                    obj = &mut obj[*part];
                }
                if null {
                    obj[key] = Value::Null;
                } else {
                    obj.as_object_mut().unwrap().remove(key);
                }
                assert!(serde_json::from_value::<CanonicalTemporalSeriesV1>(value).is_err());
            }
        }
        let mut value = template.clone();
        let mut obj = &mut value;
        for part in &path {
            obj = &mut obj[*part];
        }
        obj["extra"] = json!("x");
        assert!(serde_json::from_value::<CanonicalTemporalSeriesV1>(value).is_err());
    }
    for count in ["0", "4294967296"] {
        let mut value = template.clone();
        value["rule"]["count"]["value"] = json!(count);
        assert!(serde_json::from_value::<CanonicalTemporalSeriesV1>(value).is_err());
    }
    for refs in [json!(["v000002", "v000002"]), json!(["v000003", "v000002"])] {
        let mut value = template.clone();
        value["evidence_fact_refs"] = refs;
        assert!(serde_json::from_value::<CanonicalTemporalSeriesV1>(value).is_err());
    }
}

#[test]
fn range_and_alias_grammars_reject_invalid_imports() {
    for source in [
        r#"{"start":{"unix_seconds":"1","nanoseconds":"000000000"},"end":{"unix_seconds":"1","nanoseconds":"000000000"}}"#,
        r#"[{"unix_seconds":"0","nanoseconds":"000000000"},{"unix_seconds":"1","nanoseconds":"000000000"}]"#,
    ] {
        bad::<CanonicalTimeRangeV1>(source);
    }
    for source in [r#""v-00001""#, r#""V000001""#, r#""v1000000""#] {
        bad::<CanonicalAlias>(source);
    }
    assert!(CanonicalAlias::new(1_000_000).is_err());
    assert_eq!(CanonicalAlias::new(999_999).unwrap().as_str(), "v999999");
}
