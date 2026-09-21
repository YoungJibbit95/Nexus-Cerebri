//! Named, bounded JSON regressions shared by public parsing and real-router tests.
use serde_json::{Value, json};
pub fn malformed_cases() -> Vec<(String, String)> {
    let base: Value =
        serde_json::from_str(include_str!("../../examples/planner/recurrence-busy.json")).unwrap();
    let mut cases = Vec::new();
    for (name, path, value) in [
        ("empty-id", "/request_id", json!("")),
        ("invalid-id", "/request_id", json!("bad id")),
        ("excessive-id", "/request_id", json!("x".repeat(129))),
        ("negative-confidence", "/duration/confidence", json!(-0.1)),
        ("excessive-confidence", "/duration/confidence", json!(1.1)),
        ("zero-duration", "/duration/value/knowledge/data", json!(0)),
        (
            "negative-duration",
            "/duration/value/knowledge/data",
            json!(-1),
        ),
        (
            "overflow-duration",
            "/duration/value/knowledge/data",
            json!(i64::MAX),
        ),
        ("zero-granularity", "/granularity", json!(0)),
        (
            "invalid-range",
            "/scope/time_range/end",
            json!("2026-10-01T09:00:00Z"),
        ),
        ("future-schema", "/schema_version/minor", json!(3)),
        ("invalid-schema", "/schema_version/major", json!(-1)),
        ("old-schema-temporal", "/schema_version/minor", json!(1)),
        ("unknown-target", "/target_ids", json!(["missing"])),
        (
            "duplicate-target",
            "/target_ids",
            json!(["new-event", "new-event"]),
        ),
        ("no-target", "/target_ids", json!([])),
        ("unknown-operation", "/operation", json!("EXECUTE")),
        (
            "unknown-state",
            "/duration/value/knowledge/state",
            json!("ASSUMED"),
        ),
        (
            "unknown-duration",
            "/duration/value/knowledge",
            json!({"state":"UNKNOWN"}),
        ),
        (
            "unsafe-provenance",
            "/context/temporal/series/0/provenance",
            json!("MODEL_INFERENCE"),
        ),
        (
            "prospective-series",
            "/context/temporal/series/0/state",
            json!({"kind":"Prospective"}),
        ),
        (
            "series-object-collision",
            "/context/temporal/series/0/id",
            json!("busy"),
        ),
        (
            "horizon-mismatch",
            "/context/temporal/horizon/start",
            json!("2026-10-01T08:00:00Z"),
        ),
        (
            "incomplete-coverage",
            "/context/temporal/coverage",
            json!("Incomplete"),
        ),
        (
            "zero-recurrence-interval",
            "/context/temporal/series/0/rule/pattern/every",
            json!(0),
        ),
        (
            "empty-weekly-days",
            "/context/temporal/series/0/rule/pattern",
            json!({"frequency":"WEEKLY","every":1,"weekdays":[]}),
        ),
        (
            "duplicate-weekly-days",
            "/context/temporal/series/0/rule/pattern",
            json!({"frequency":"WEEKLY","every":1,"weekdays":["Mon","Mon"]}),
        ),
        (
            "unknown-recurrence",
            "/context/temporal/series/0/rule/pattern/frequency",
            json!("MONTHLY"),
        ),
        (
            "invalid-timezone",
            "/context/temporal/series/0/rule/timezone",
            json!("Mars/Olympus"),
        ),
        (
            "zero-count",
            "/context/temporal/series/0/rule/count",
            json!(0),
        ),
        (
            "until-before-start",
            "/context/temporal/series/0/rule/until",
            json!("2026-09-01"),
        ),
        ("excessive-budget", "/budget/max_candidates", json!(4097)),
        (
            "excessive-date-budget",
            "/context/temporal/limits/max_dates",
            json!(36601),
        ),
        (
            "excessive-occurrence-budget",
            "/context/temporal/limits/max_occurrences",
            json!(1025),
        ),
        (
            "empty-date-budget",
            "/context/temporal/limits/max_dates",
            json!(0),
        ),
        (
            "missing-dependency",
            "/constraints",
            json!([{"object_id":"new-event","rule":{"kind":"DEPENDENCY_ORDER","value":"missing"},"evidence":[]}]),
        ),
        (
            "self-cycle",
            "/constraints",
            json!([{"object_id":"new-event","rule":{"kind":"DEPENDENCY_ORDER","value":"new-event"},"evidence":[]}]),
        ),
    ] {
        let mut changed = base.clone();
        *changed.pointer_mut(path).unwrap() = value;
        cases.push((name.into(), changed.to_string()));
    }
    for (name, path, count) in [
        ("duplicate-object", "/context/objects", 3),
        ("too-many-objects", "/context/objects", 257),
        ("duplicate-series", "/context/temporal/series", 2),
        ("too-many-series", "/context/temporal/series", 33),
    ] {
        let mut changed = base.clone();
        let items = changed.pointer_mut(path).unwrap().as_array_mut().unwrap();
        items.resize(count, items[0].clone());
        cases.push((name.into(), changed.to_string()));
    }
    for name in ["duplicate-fact", "unsafe-fact", "too-many-facts"] {
        let mut changed = base.clone();
        let fact = json!({"id":"fact","object_id":"busy","value":{"kind":"EXTERNAL_LOCK"},"provenance":if name=="unsafe-fact" {"MODEL_INFERENCE"}else{"SYSTEM_FACT"}});
        changed["context"]["facts"] = json!(vec![
            fact;
            if name == "too-many-facts" {
                1025
            } else if name == "duplicate-fact" {
                2
            } else {
                1
            }
        ]);
        cases.push((name.into(), changed.to_string()));
    }
    for path in [
        "",
        "/scope",
        "/context",
        "/context/temporal",
        "/budget",
        "/duration",
    ] {
        let mut changed = base.clone();
        changed
            .pointer_mut(path)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("unexpected".into(), json!(true));
        cases.push((format!("unknown-field-{path}"), changed.to_string()));
    }
    for token in ["NaN", "Infinity", "-Infinity", "1e9999"] {
        cases.push((
            format!("nonfinite-{token}"),
            base.to_string()
                .replacen("\"confidence\":null", &format!("\"confidence\":{token}"), 1),
        ));
    }
    cases.push((
        "deep-json".into(),
        format!("{}0{}", "[".repeat(140), "]".repeat(140)),
    ));
    cases.push(("truncated-json".into(), "{\"schema_version\":".into()));
    cases
}
