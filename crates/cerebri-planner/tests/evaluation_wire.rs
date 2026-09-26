//! Independent contract cases for the implemented Phase-A foundations only.
use cerebri_planner::evaluation::*;
use cerebri_temporal::{Instant, LocalDate, LocalTime};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};

fn rejects<T: DeserializeOwned>(wire: &str) {
    assert!(
        serde_json::from_str::<T>(wire).is_err(),
        "unexpected acceptance: {wire}"
    );
}

#[test]
fn required_nullable_distinguishes_missing_null_value_and_duplicate_members() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct PresenceProbe {
        #[serde(deserialize_with = "required_nullable")]
        value: Option<IdentifierString>,
    }
    for wire in [r#"{"value":null}"#, r#"{"value":"synthetic-1"}"#] {
        let parsed: PresenceProbe = serde_json::from_str(wire).unwrap();
        assert_eq!(serde_json::to_string(&parsed).unwrap(), wire);
    }
    for wire in [
        r#"{}"#,
        r#"{"value":false}"#,
        r#"{"value":null,"extra":0}"#,
        r#"{"value":null,"value":"id"}"#,
    ] {
        rejects::<PresenceProbe>(wire);
    }
}

#[test]
fn canonical_optional_rejects_null_unknown_and_wrong_variant_fields() {
    type Optional = CanonicalOptionalV1<CanonicalU64Decimal>;
    for wire in [
        r#"{"state":"NONE"}"#,
        r#"{"state":"SOME","value":"0"}"#,
        r#"{"state":"SOME","value":"18446744073709551615"}"#,
    ] {
        let parsed: Optional = serde_json::from_str(wire).unwrap();
        assert_eq!(serde_json::to_string(&parsed).unwrap(), wire);
    }
    for wire in [
        "null",
        "{}",
        r#"{"state":"UNKNOWN"}"#,
        r#"{"state":"SOME"}"#,
        r#"{"state":"SOME","value":null}"#,
        r#"{"state":"SOME","value":0}"#,
        r#"{"state":"NONE","value":"1"}"#,
        r#"{"state":"NONE","value":null}"#,
        r#"{"state":"NONE","extra":true}"#,
        r#"{"state":"SOME","value":"1","extra":true}"#,
        r#"{"state":"SOME","value":"1","value":"2"}"#,
    ] {
        rejects::<Optional>(wire);
    }
}

#[test]
fn decimal_wire_is_string_only_unique_and_covers_full_integer_ranges() {
    for raw in ["-9223372036854775808", "-1", "0", "9223372036854775807"] {
        let wire = format!("\"{raw}\"");
        let parsed: CanonicalI64Decimal = serde_json::from_str(&wire).unwrap();
        assert_eq!(serde_json::to_string(&parsed).unwrap(), wire);
    }
    for raw in ["0", "1", "18446744073709551615"] {
        let wire = format!("\"{raw}\"");
        let parsed: CanonicalU64Decimal = serde_json::from_str(&wire).unwrap();
        assert_eq!(serde_json::to_string(&parsed).unwrap(), wire);
    }
    for raw in ["", "+1", "01", "-0", " 1", "1 ", "1.0", "1e0", "١", "--1"] {
        let wire = serde_json::to_string(raw).unwrap();
        rejects::<CanonicalI64Decimal>(&wire);
        rejects::<CanonicalU64Decimal>(&wire);
    }
    for wire in ["0", "1.0", "null", "true", "[]", "{}"] {
        rejects::<CanonicalI64Decimal>(wire);
        rejects::<CanonicalU64Decimal>(wire);
    }
    for raw in ["-9223372036854775809", "9223372036854775808"] {
        rejects::<CanonicalI64Decimal>(&serde_json::to_string(raw).unwrap());
    }
    for raw in ["-1", "18446744073709551616"] {
        rejects::<CanonicalU64Decimal>(&serde_json::to_string(raw).unwrap());
    }
    rejects::<CanonicalPositiveSeconds>(r#""0""#);
    assert!(CanonicalPositiveSeconds::new(0).is_err());
    assert_eq!(
        serde_json::from_str::<CanonicalPositiveSeconds>(r#""18446744073709551615""#)
            .unwrap()
            .value(),
        u64::MAX
    );
    assert!(CanonicalU64Decimal::new(2).unwrap() < CanonicalU64Decimal::new(10).unwrap());
}

#[test]
fn identifiers_rule_ids_and_hashes_reject_normalization_and_bad_boundaries() {
    for value in ["a", "Case-sensitive_1.2", &"a".repeat(128)] {
        assert_eq!(IdentifierString::new(value).unwrap().as_str(), value);
        assert_eq!(RuleIdV1::new(value).unwrap().as_str(), value);
    }
    for value in [
        "",
        " white",
        "white ",
        "two words",
        "ä",
        "a/b",
        "a\0b",
        &"a".repeat(129),
    ] {
        rejects::<IdentifierString>(&serde_json::to_string(value).unwrap());
        assert!(RuleIdV1::new(value).is_err());
    }
    assert_ne!(
        RuleIdV1::new("Rule").unwrap(),
        RuleIdV1::new("rule").unwrap()
    );
    for value in ["0".repeat(64), "abcdef0123456789".repeat(4)] {
        assert_eq!(SHA256Hex::new(&value).unwrap().as_str(), value);
    }
    for value in [
        "0".repeat(63),
        "0".repeat(65),
        "A".repeat(64),
        "g".repeat(64),
    ] {
        rejects::<SHA256Hex>(&serde_json::to_string(&value).unwrap());
    }
}

#[test]
fn canonical_instants_preserve_subseconds_and_i64_boundaries() {
    for raw in ["000000000", "000000001", "999999999"] {
        let wire = serde_json::to_string(raw).unwrap();
        let parsed: NineDigitNanoseconds = serde_json::from_str(&wire).unwrap();
        assert_eq!(serde_json::to_string(&parsed).unwrap(), wire);
    }
    for raw in ["0", "00000000", "1000000000", "-00000001", "00000000a"] {
        rejects::<NineDigitNanoseconds>(&serde_json::to_string(raw).unwrap());
    }
    assert!(NineDigitNanoseconds::new(1_000_000_000).is_err());
    let instant: Instant = "1969-12-31T23:59:59.999999999Z".parse().unwrap();
    assert_eq!(
        serde_json::to_value(CanonicalInstantV1::try_from(instant).unwrap()).unwrap(),
        json!({"unix_seconds":"-1", "nanoseconds":"999999999"})
    );
    for seconds in ["-9223372036854775808", "9223372036854775807"] {
        let wire = json!({"unix_seconds":seconds, "nanoseconds":"000000000"});
        let parsed: CanonicalInstantV1 = serde_json::from_value(wire.clone()).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), wire);
    }
    for wire in [
        r#"{"unix_seconds":"0"}"#,
        r#"{"unix_seconds":"0","nanoseconds":0}"#,
        r#"{"unix_seconds":"0","nanoseconds":"000000000","extra":0}"#,
    ] {
        rejects::<CanonicalInstantV1>(wire);
    }
}

#[test]
fn calendar_and_local_time_use_exact_width_and_existing_calendar_validation() {
    let leap_day = LocalDate::from_ymd_opt(2024, 2, 29).unwrap();
    assert_eq!(
        CanonicalDate::try_from(leap_day).unwrap().as_str(),
        "2024-02-29"
    );
    for value in [
        "2023-02-29",
        "2024-2-29",
        "2024-02-30",
        "2024-13-01",
        "2024/02/29",
        "+2024-02-29",
        "2024-02-29Z",
    ] {
        assert!(CanonicalDate::new(value).is_err(), "{value}");
    }
    let time = LocalTime::from_hms_nano_opt(9, 8, 7, 42).unwrap();
    assert_eq!(
        CanonicalLocalTime::try_from(time).unwrap().as_str(),
        "09:08:07.000000042"
    );
    for value in [
        "9:08:07.000000000",
        "09:08:07",
        "09:08:07.0",
        "24:00:00.000000000",
        "09:60:00.000000000",
        "09:08:07.000000000Z",
    ] {
        assert!(CanonicalLocalTime::new(value).is_err(), "{value}");
    }
}

#[test]
fn architecture_tokens_are_explicit_and_e2_data_scope_is_closed() {
    assert_eq!(
        PreferenceSourceToken::ExplicitCurrentRequest.as_str(),
        "EXPLICIT_CURRENT_REQUEST"
    );
    assert_eq!(
        serde_json::to_string(&PreferenceSourceToken::ExplicitCurrentRequest).unwrap(),
        r#""EXPLICIT_CURRENT_REQUEST""#
    );
    rejects::<PreferenceSourceToken>(r#""ExplicitCurrentRequest""#);
    assert_eq!(
        serde_json::from_str::<DataScope>(r#""SYNTHETIC""#).unwrap(),
        DataScope::Synthetic
    );
    for value in [
        "GLOBAL_CURATED",
        "PERSONAL_LOCAL",
        "OPT_IN_SHARED",
        "UNKNOWN",
        "Synthetic",
    ] {
        rejects::<DataScope>(&serde_json::to_string(value).unwrap());
    }
    for value in ["PLANNING_OBJECT", "TEMPORAL_SERIES"] {
        let token: RevisionEntityTypeV1 = serde_json::from_value(json!(value)).unwrap();
        assert_eq!(token.as_str(), value);
    }
    rejects::<RevisionEntityTypeV1>(r#""FACT""#);
    rejects::<GraphEdgeTypeToken>(r#""OBJECT_SERIES""#);
}

fn manifest_entry(path: &str, roles: Value) -> Value {
    json!({"path":path, "roles":roles, "sha256":"a".repeat(64), "size_bytes":"18446744073709551615"})
}

#[test]
fn artifact_manifest_preserves_one_path_with_multiple_roles_and_rejects_ambiguity() {
    let entry = manifest_entry(
        "src/planner.rs",
        json!(["DEPENDENCY_SOURCE", "PRIMARY_SOURCE"]),
    );
    let wire = json!({"schema_version":"0.1", "entries":[entry.clone()]});
    let manifest: ArtifactManifestV0_1 = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(serde_json::to_value(&manifest).unwrap(), wire);
    assert_eq!(manifest.entries().len(), 1);
    assert_eq!(manifest.entries()[0].roles().len(), 2);
    assert_eq!(manifest.entries()[0].size_bytes().value(), u64::MAX);
    for entries in [
        json!([entry.clone(), entry.clone()]),
        json!([manifest_entry("z", json!(["CONFIG"])), entry.clone()]),
        json!([manifest_entry("a", json!([]))]),
        json!([manifest_entry("a", json!(["CONFIG", "CONFIG"]))]),
        json!([manifest_entry("a", json!(["SCHEMA", "CONFIG"]))]),
        json!([manifest_entry("a", json!(["UnknownRole"]))]),
    ] {
        assert!(
            serde_json::from_value::<ArtifactManifestV0_1>(
                json!({"schema_version":"0.1", "entries":entries})
            )
            .is_err()
        );
    }
    let mut wrong_version = wire.clone();
    wrong_version["schema_version"] = json!("0.2");
    assert!(serde_json::from_value::<ArtifactManifestV0_1>(wrong_version).is_err());
    let mut unknown = wire.clone();
    unknown["extra"] = json!(true);
    assert!(serde_json::from_value::<ArtifactManifestV0_1>(unknown).is_err());
    let mut unknown = wire.clone();
    unknown["entries"][0]["extra"] = json!(true);
    assert!(serde_json::from_value::<ArtifactManifestV0_1>(unknown).is_err());
    for field in ["schema_version", "entries"] {
        let mut missing = wire.clone();
        missing.as_object_mut().unwrap().remove(field);
        assert!(serde_json::from_value::<ArtifactManifestV0_1>(missing).is_err());
    }
    for field in ["path", "roles", "sha256", "size_bytes"] {
        let mut missing = wire.clone();
        missing["entries"][0].as_object_mut().unwrap().remove(field);
        assert!(serde_json::from_value::<ArtifactManifestV0_1>(missing).is_err());
    }
    rejects::<ArtifactManifestV0_1>(r#"{"schema_version":"0.1","entries":[],"entries":[]}"#);
}

#[test]
fn bundle_paths_reject_machine_paths_and_traversal_without_repair() {
    for path in [
        "",
        "/root/file",
        "../file",
        "a/../b",
        "a/./b",
        "a//b",
        "a/",
        "C:/file",
        "C:file",
        "a\\b",
        "a\0b",
    ] {
        rejects::<BundleRelativePath>(&serde_json::to_string(path).unwrap());
    }
    for path in ["src/main.rs", "config/settings.json", "資料/plan.txt"] {
        assert_eq!(BundleRelativePath::new(path).unwrap().as_str(), path);
    }
}

#[test]
fn artifact_semantic_identity_has_no_implicit_resolver_or_digest_verification() {
    let wire = json!({"artifact_class":"PLANNER_ALGORITHM", "artifact_id":"deterministic-planner",
        "artifact_version":"0.1", "manifest_schema_version":"0.1", "manifest_sha256":"0".repeat(64)});
    let identity: ArtifactSemanticIdentityV0_1 = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(serde_json::to_value(identity).unwrap(), wire);
    let mut extra = wire.clone();
    extra["immutable_ref"] = json!("unrecognized");
    assert!(serde_json::from_value::<ArtifactSemanticIdentityV0_1>(extra).is_err());
    let mut invalid = wire;
    invalid["artifact_class"] = json!("PlannerAlgorithm");
    assert!(serde_json::from_value::<ArtifactSemanticIdentityV0_1>(invalid).is_err());
}

#[test]
fn run_binding_requires_presence_and_preserves_new_versus_reused_run() {
    for (run_use, source) in [("NEW_RUN", Value::Null), ("REUSED_RUN", json!("parent-1"))] {
        let wire =
            json!({"planner_run_id":"run-1", "run_use":run_use, "reused_from_episode_id":source});
        let binding: PlannerRunBindingV0_1 = serde_json::from_value(wire.clone()).unwrap();
        assert_eq!(serde_json::to_value(binding).unwrap(), wire);
    }
    for wire in [
        json!({"planner_run_id":"run-1", "run_use":"NEW_RUN"}),
        json!({"planner_run_id":"run-1", "run_use":"REUSED_RUN"}),
        json!({"planner_run_id":"run-1", "run_use":"NEW_RUN", "reused_from_episode_id":"parent-1"}),
        json!({"planner_run_id":"run-1", "run_use":"REUSED_RUN", "reused_from_episode_id":null}),
        json!({"planner_run_id":"run-1", "run_use":"UNKNOWN", "reused_from_episode_id":null}),
        json!({"planner_run_id":"run-1", "run_use":"NEW_RUN", "reused_from_episode_id":null, "extra":false}),
    ] {
        assert!(serde_json::from_value::<PlannerRunBindingV0_1>(wire).is_err());
    }
    assert!(
        PlannerRunBindingV0_1::new(
            IdentifierString::new("run-1").unwrap(),
            PlannerRunUseToken::ReusedRun,
            None
        )
        .is_err()
    );
}

#[test]
fn planner_run_timing_allows_equality_but_rejects_reversed_order() {
    let start: Instant = "2026-09-27T08:00:00Z".parse().unwrap();
    let end: Instant = "2026-09-27T08:00:00.000000001Z".parse().unwrap();
    assert!(PlannerRunTimestampsV0_1::new(start, start).is_ok());
    assert!(PlannerRunTimestampsV0_1::new(start, end).is_ok());
    assert!(PlannerRunTimestampsV0_1::new(end, start).is_err());
    assert!(
        serde_json::from_value::<PlannerRunTimestampsV0_1>(
            json!({"context_captured_at":end,"generation_completed_at":start})
        )
        .is_err()
    );
    assert!(serde_json::from_value::<PlannerRunTimestampsV0_1>(json!({"context_captured_at":start,"generation_completed_at":end,"episode_started_at":start})).is_err());
}

#[test]
fn lineage_and_lifecycle_wire_require_all_nullable_members_before_later_validation() {
    let lineage = json!({"family_id":"family-1", "parent_episode_id":null, "transition_cause":null, "transition_reasons":[]});
    let parsed: EpisodeLineageWireV0_1 = serde_json::from_value(lineage.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), lineage);
    for field in ["parent_episode_id", "transition_cause"] {
        let mut missing = lineage.clone();
        missing.as_object_mut().unwrap().remove(field);
        assert!(serde_json::from_value::<EpisodeLineageWireV0_1>(missing).is_err());
    }
    for cause in [
        json!({"context_changed":false}),
        json!({"context_changed":false,"explicit_replan":false,"new_visible_policy_application":true,"new_generator_run":false}),
    ] {
        assert!(serde_json::from_value::<LineageTransitionCauseObservationV0_1>(cause).is_err());
    }
    let timestamps = json!({"episode_started_at":"2026-09-27T08:00:00Z", "policy_application_at":null,
        "closed_at":null, "episode_recorded_at":"2026-09-27T08:00:00Z"});
    let parsed: EpisodeLifecycleTimestampsWireV0_1 =
        serde_json::from_value(timestamps.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), timestamps);
    for field in ["policy_application_at", "closed_at"] {
        let mut missing = timestamps.clone();
        missing.as_object_mut().unwrap().remove(field);
        assert!(serde_json::from_value::<EpisodeLifecycleTimestampsWireV0_1>(missing).is_err());
    }
}

#[test]
fn canonical_time_range_has_exact_typed_shape_order_and_positive_extent() {
    let start = json!({"t":"INSTANT", "s":"-1", "ns":"999999999"});
    let end = json!({"t":"INSTANT", "s":"0", "ns":"000000000"});
    let wire =
        json!({"t":"OBJECT", "v":[{"key":"end", "value":end}, {"key":"start", "value":start}]});
    let range: CanonicalTimeRangeValueV1 = serde_json::from_value(wire.clone()).unwrap();
    assert!(range.start() < range.end());
    assert_eq!(serde_json::to_value(range).unwrap(), wire);
    for change in [
        "reorder",
        "duplicate",
        "empty",
        "reverse",
        "extra",
        "wrong-tag",
        "plain-range",
    ] {
        let mut bad = wire.clone();
        match change {
            "reorder" => bad["v"].as_array_mut().unwrap().swap(0, 1),
            "duplicate" => bad["v"][1]["key"] = json!("end"),
            "empty" => bad["v"][1]["value"] = end.clone(),
            "reverse" => {
                bad["v"][0]["value"] = start.clone();
                bad["v"][1]["value"] = end.clone();
            }
            "extra" => bad["v"]
                .as_array_mut()
                .unwrap()
                .push(json!({"key":"other", "value":start})),
            "wrong-tag" => bad["v"][0]["value"]["t"] = json!("TEXT"),
            "plain-range" => bad = json!({"start":start,"end":end}),
            _ => unreachable!(),
        }
        assert!(
            serde_json::from_value::<CanonicalTimeRangeValueV1>(bad).is_err(),
            "{change}"
        );
    }
}

#[test]
fn software_provenance_accepts_full_commit_ids_and_excludes_planner_authority() {
    for commit in [
        "f1934417445099388e8123676879d51b3cd029ab".to_owned(),
        "b".repeat(64),
    ] {
        let wire = json!({"git_commit_sha":commit,"workspace_version":"0.2.0", "cpir_schema_version":"0.2", "ranking_feature_schema_version":"0.1"});
        let provenance: SoftwareProvenanceV0_1 = serde_json::from_value(wire.clone()).unwrap();
        assert_eq!(serde_json::to_value(provenance).unwrap(), wire);
        let mut duplicate_authority = wire;
        duplicate_authority["planner_artifact"] = json!({});
        assert!(serde_json::from_value::<SoftwareProvenanceV0_1>(duplicate_authority).is_err());
    }
    for commit in [
        "f193441".to_owned(),
        "A".repeat(40),
        "g".repeat(64),
        "b".repeat(41),
    ] {
        assert!(RepositoryCommitHex::new(commit).is_err());
    }
}

#[test]
fn privacy_component_rejects_content_and_requires_explicit_false() {
    let wire = json!({"classification":"synthetic", "retention_policy_id":"fixture", "retention_policy_version":"1", "content_fields_present":false});
    let privacy: PrivacyV0_1 = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(serde_json::to_value(privacy).unwrap(), wire);
    let mut content = wire.clone();
    content["content_fields_present"] = json!(true);
    assert!(serde_json::from_value::<PrivacyV0_1>(content).is_err());
    let mut missing = wire.clone();
    missing
        .as_object_mut()
        .unwrap()
        .remove("content_fields_present");
    assert!(serde_json::from_value::<PrivacyV0_1>(missing).is_err());
    let mut raw = wire;
    raw["event_title"] = json!("not permitted");
    assert!(serde_json::from_value::<PrivacyV0_1>(raw).is_err());
    rejects::<PrincipalGroupRef>(
        r#"{"group_id":"synthetic-group","key_version":"1","account_id":"raw"}"#,
    );
}
