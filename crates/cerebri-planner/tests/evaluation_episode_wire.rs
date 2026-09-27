//! Phase-A decoding fixtures only: hashes are opaque test values, not replay evidence.
use cerebri_planner::evaluation::*;
use serde_json::{Value, json};

fn binding(class: &str) -> Value {
    json!({
        "artifact_semantic_identity": {
            "artifact_class": class, "artifact_id": "fixture", "artifact_version": "0.1",
            "manifest_schema_version": "0.1", "manifest_sha256": "a".repeat(64)
        },
        "immutable_ref": {"kind": "SYNTHETIC_FIXTURE", "fixture_id": "fixture",
            "manifest_sha256": "a".repeat(64)}
    })
}
fn placement() -> Value {
    json!({"object_id": "object-1",
        "start": {"unix_seconds": "10", "nanoseconds": "000000001"},
        "end": {"unix_seconds": "20", "nanoseconds": "000000002"}})
}
fn ranking() -> Value {
    json!({"schema_version": {"major": 0, "minor": 1},
        "preferred_start_distance_seconds": null, "preferred_start_source": null,
        "mutation_count": 0, "shift_seconds": 0})
}
fn episode() -> Value {
    let hypothesis = json!({"schema_version": "0.1", "placements": [placement()]});
    json!({
        "schema_version": {"major": 0, "minor": 1}, "episode_id": "episode-1",
        "lineage": {"family_id": "family-1", "parent_episode_id": null,
            "transition_cause": null, "transition_reasons": []},
        "data_scope": "SYNTHETIC", "principal_group_ref": null,
        "planner_run_binding": {"planner_run_id": "run-1", "run_use": "NEW_RUN",
            "reused_from_episode_id": null},
        "planner_run_timestamps": {"context_captured_at": "1970-01-01T00:00:01Z",
            "generation_completed_at": "1970-01-01T00:00:02Z"},
        "lifecycle_timestamps": {"episode_started_at": "1970-01-01T00:00:00Z",
            "policy_application_at": "1970-01-01T00:00:03Z", "closed_at": "1970-01-01T00:00:05Z",
            "episode_recorded_at": "1970-01-01T00:00:06Z"},
        "base_scenario_fingerprint": "a".repeat(64), "decision_input_fingerprint": "b".repeat(64),
        "decision_observation_digest": "c".repeat(64),
        "software_provenance": {"git_commit_sha": "a".repeat(40), "workspace_version": "0.2.0",
            "cpir_schema_version": "0.2", "ranking_feature_schema_version": "0.1"},
        "replay_provenance": {
            "source_snapshot_ref": {"artifact_binding": binding("SOURCE_SNAPSHOT"),
                "snapshot_schema_id": "typed-request", "snapshot_schema_version": "0.1",
                "privacy_classification": "synthetic", "retention_policy_id": "fixture-retention",
                "retention_policy_version": "0.1", "content_kind": "synthetic-request",
                "resolution_state": "RESOLVABLE_AND_VERIFIED"},
            "planner_artifact": binding("PLANNER_ALGORITHM"),
            "generator_artifact": binding("GENERATOR_POLICY"),
            "generator_configuration_artifact": null,
            "product_eligibility_policy": binding("PRODUCT_ELIGIBILITY_POLICY"),
            "ranking_policy": binding("RANKING_POLICY"), "display_policy": binding("DISPLAY_POLICY"),
            "model_artifact": null, "experiment_assignment": null},
        "candidate_generation": {"state": "SEARCHED",
            "planner_admission": {"admission_class": "VALID", "issue_codes": []},
            "search_visit": {"schema_version": "0.1", "hypothesis_schema_version": "0.1",
                "declared_hypotheses": [hypothesis.clone()], "visited_hypotheses": [hypothesis],
                "declared_hypothesis_count": "1", "evaluated_positions": "1",
                "declared_set_identity_digest": "d".repeat(64), "visited_set_identity_digest": "d".repeat(64),
                "visited_order_digest": "e".repeat(64)},
            "outcome": "SOLUTION", "search_assessment": "PROVEN_OPTIMAL", "search_exhausted": true,
            "proof_claim": "PROVEN_OPTIMAL_DETERMINISTIC_OBJECTIVE"},
        "candidates": [{"candidate_id": "candidate-1", "candidate_fingerprint": "f".repeat(64),
            "placements": [placement()], "ranking_feature_snapshot": ranking(),
            "deterministic_rank_in_discovered": "1",
            "deterministic_ordering_key": {"preference_distance_seconds": 0, "mutation_count": 0,
                "shift_seconds": 0, "start": "1970-01-01T00:00:10.000000001Z", "object_id": "object-1"}}],
        "product_eligibility": {"state": "APPLIED", "records": [{"candidate_id": "candidate-1",
            "eligible": true, "exclusion_rule_id": null}]},
        "ranking_observation": {"state": "APPLIED", "ranked_eligible_order": ["candidate-1"]},
        "display_decision": {"state": "APPLIED", "display_policy_output_order": ["candidate-1"]},
        "exposure": {"state": "EXPOSED", "surface_id": "fixture", "surface_version": "0.1",
            "displayed_order": ["candidate-1"], "viewport_visible_candidate_ids": null,
            "exposed_at": "1970-01-01T00:00:04Z"},
        "manual_replacements": [],
        "interaction_events": [{"sequence": "1", "at": "1970-01-01T00:00:05Z", "kind": "SELECTED",
            "candidate_id": "candidate-1", "replacement_id": null, "rejection_scope": null,
            "correction_kind": null}],
        "terminal_state": "CLOSED_SELECTED", "capture_completeness": {"state": "COMPLETE"},
        "outcome": null,
        "privacy": {"classification": "synthetic", "retention_policy_id": "fixture-retention",
            "retention_policy_version": "0.1", "content_fields_present": false}
    })
}
fn decode(value: Value) -> Result<EvaluationEpisodeV0_1, serde_json::Error> {
    // Exercise the real streaming JSON boundary, not only Value's deserializer.
    serde_json::from_str(&serde_json::to_string(&value).unwrap())
}
fn reject_at(path: &str, value: Value) {
    let mut fixture = episode();
    *fixture.pointer_mut(path).expect(path) = value;
    assert!(decode(fixture).is_err(), "accepted invalid {path}");
}

#[test]
fn complete_searched_wire_roundtrips_without_recomputing_observations() {
    let original = episode();
    let domain = decode(original.clone()).unwrap();
    assert!(domain.as_wire().outcome.is_none());
    assert_eq!(serde_json::to_value(&domain).unwrap(), original);
    let raw = domain.into_wire();
    assert_eq!(
        EvaluationEpisodeV0_1::try_from(raw)
            .unwrap()
            .as_wire()
            .schema_version,
        EvaluationEpisodeSchemaVersionV0_1::V0_1
    );
}

#[test]
fn every_episode_member_is_required_and_top_level_is_closed() {
    let fixture = episode();
    for key in fixture.as_object().unwrap().keys() {
        let mut missing = fixture.clone();
        missing.as_object_mut().unwrap().remove(key);
        assert!(decode(missing).is_err(), "accepted missing {key}");
    }
    let mut extra = fixture;
    extra["training_eligible"] = json!(true);
    assert!(decode(extra).is_err());
}

#[test]
fn episode_version_is_exact_required_u16_object() {
    for version in [
        json!("0.1"),
        json!(0.1),
        json!([0, 1]),
        json!(null),
        json!({"version": "0.1"}),
        json!({"major": 0}),
        json!({"minor": 1}),
        json!({"major": 0, "minor": 1, "patch": 0}),
        json!({"major": 0, "minor": 2}),
        json!({"major": 65535, "minor": 1}),
        json!({"major": -1, "minor": 1}),
        json!({"major": 0, "minor": 65536}),
        json!({"major": 0.0, "minor": 1}),
        json!({"major": false, "minor": 1}),
        json!({"major": "0", "minor": 1}),
    ] {
        reject_at("/schema_version", version);
    }
    assert_eq!(
        serde_json::to_string(&EvaluationEpisodeSchemaVersionV0_1::V0_1).unwrap(),
        r#"{"major":0,"minor":1}"#
    );
}

#[test]
fn closed_objects_reject_positional_array_equivalents() {
    reject_at("/lineage", json!(["family-1", null, null, []]));
    reject_at("/principal_group_ref", json!(["synthetic-group", "v1"]));
    reject_at("/planner_run_binding", json!(["run-1", "NEW_RUN", null]));
    reject_at(
        "/planner_run_timestamps",
        json!(["1970-01-01T00:00:01Z", "1970-01-01T00:00:02Z"]),
    );
    reject_at(
        "/lifecycle_timestamps",
        json!([
            "1970-01-01T00:00:00Z",
            "1970-01-01T00:00:03Z",
            "1970-01-01T00:00:05Z",
            "1970-01-01T00:00:06Z"
        ]),
    );
    reject_at(
        "/software_provenance",
        json!(["a".repeat(40), "0.2.0", "0.2", "0.1"]),
    );
    reject_at(
        "/privacy",
        json!(["synthetic", "fixture-retention", "0.1", false]),
    );
    reject_at(
        "/product_eligibility/records/0",
        json!(["candidate-1", true, null]),
    );
    reject_at(
        "/candidate_generation/planner_admission",
        json!(["VALID", []]),
    );
    reject_at(
        "/candidates/0/placements/0/start",
        json!(["10", "000000001"]),
    );
    reject_at(
        "/replay_provenance/planner_artifact/artifact_semantic_identity",
        json!(["PLANNER_ALGORITHM", "fixture", "0.1", "0.1", "a".repeat(64)]),
    );
    let fixture = episode();
    // The exact declaration order would otherwise deserialize as a Serde struct sequence.
    let fields = [
        "schema_version",
        "episode_id",
        "lineage",
        "data_scope",
        "principal_group_ref",
        "planner_run_binding",
        "planner_run_timestamps",
        "lifecycle_timestamps",
        "base_scenario_fingerprint",
        "decision_input_fingerprint",
        "decision_observation_digest",
        "software_provenance",
        "replay_provenance",
        "candidate_generation",
        "candidates",
        "product_eligibility",
        "ranking_observation",
        "display_decision",
        "exposure",
        "manual_replacements",
        "interaction_events",
        "terminal_state",
        "capture_completeness",
        "outcome",
        "privacy",
    ];
    assert!(
        decode(Value::Array(
            fields.iter().map(|field| fixture[field].clone()).collect()
        ))
        .is_err()
    );
}

#[test]
fn standalone_foundation_objects_also_reject_positional_arrays() {
    assert!(serde_json::from_value::<ArtifactManifestV0_1>(json!(["0.1", []])).is_err());
    assert!(
        serde_json::from_value::<ArtifactManifestEntryV0_1>(json!([
            "manifest.json",
            ["SCHEMA"],
            "a".repeat(64),
            "1"
        ]))
        .is_err()
    );
    assert!(
        serde_json::from_value::<PlannerRunBindingV0_1>(json!(["run-1", "NEW_RUN", null])).is_err()
    );
    assert!(
        serde_json::from_value::<PlannerRunTimestampsV0_1>(json!([
            "1970-01-01T00:00:01Z",
            "1970-01-01T00:00:02Z"
        ]))
        .is_err()
    );
    assert!(
        serde_json::from_value::<PrivacyV0_1>(json!(["synthetic", "retention", "v1", false]))
            .is_err()
    );
    assert!(serde_json::from_value::<CanonicalInstantV1>(json!(["10", "000000001"])).is_err());
}

#[test]
fn e2_null_only_fields_reject_every_non_null_json_kind() {
    for path in ["/outcome", "/replay_provenance/experiment_assignment"] {
        for invalid in [
            json!({}),
            json!([]),
            json!("NONE"),
            json!(0),
            json!(false),
            json!({"state": "NONE"}),
        ] {
            reject_at(path, invalid);
        }
    }
    reject_at("/replay_provenance/model_artifact", binding("MODEL"));
    reject_at("/replay_provenance/source_snapshot_ref", json!(null));
    for scope in ["PERSONAL_LOCAL", "GLOBAL_SHARED", "REAL", "synthetic"] {
        reject_at("/data_scope", json!(scope));
    }
    reject_at("/privacy/content_fields_present", json!(true));
}

#[test]
fn all_nested_required_nullable_members_reject_absence() {
    for (parent, key) in [
        ("/lineage", "parent_episode_id"),
        ("/lineage", "transition_cause"),
        ("/planner_run_binding", "reused_from_episode_id"),
        ("/lifecycle_timestamps", "policy_application_at"),
        ("/lifecycle_timestamps", "closed_at"),
        ("/replay_provenance", "source_snapshot_ref"),
        ("/replay_provenance", "generator_configuration_artifact"),
        ("/replay_provenance", "model_artifact"),
        ("/replay_provenance", "experiment_assignment"),
        ("/product_eligibility/records/0", "exclusion_rule_id"),
        ("/exposure", "viewport_visible_candidate_ids"),
        ("/interaction_events/0", "candidate_id"),
        ("/interaction_events/0", "replacement_id"),
        ("/interaction_events/0", "rejection_scope"),
        ("/interaction_events/0", "correction_kind"),
        (
            "/candidates/0/ranking_feature_snapshot",
            "preferred_start_distance_seconds",
        ),
        (
            "/candidates/0/ranking_feature_snapshot",
            "preferred_start_source",
        ),
    ] {
        let mut fixture = episode();
        fixture
            .pointer_mut(parent)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .remove(key);
        assert!(decode(fixture).is_err(), "accepted missing {parent}/{key}");
    }
}

#[test]
fn null_empty_and_value_are_preserved_for_permitted_optional_fields() {
    for visibility in [json!(null), json!([]), json!(["candidate-1"])] {
        let mut fixture = episode();
        fixture["exposure"]["viewport_visible_candidate_ids"] = visibility.clone();
        fixture["principal_group_ref"] =
            json!({"group_id": "synthetic-group", "key_version": "v1"});
        fixture["replay_provenance"]["generator_configuration_artifact"] =
            binding("PLANNER_CONFIGURATION");
        let decoded = decode(fixture).unwrap();
        assert_eq!(
            serde_json::to_value(decoded).unwrap()["exposure"]["viewport_visible_candidate_ids"],
            visibility
        );
    }
}

#[test]
fn closed_nested_shapes_reject_unknown_and_wrong_variant_fields() {
    for path in [
        "/lineage",
        "/planner_run_binding",
        "/planner_run_timestamps",
        "/lifecycle_timestamps",
        "/software_provenance",
        "/replay_provenance",
        "/replay_provenance/source_snapshot_ref",
        "/replay_provenance/planner_artifact",
        "/replay_provenance/planner_artifact/immutable_ref",
        "/candidate_generation",
        "/candidate_generation/planner_admission",
        "/candidate_generation/search_visit",
        "/candidate_generation/search_visit/declared_hypotheses/0",
        "/candidates/0",
        "/candidates/0/placements/0",
        "/candidates/0/deterministic_ordering_key",
        "/product_eligibility",
        "/product_eligibility/records/0",
        "/ranking_observation",
        "/display_decision",
        "/exposure",
        "/interaction_events/0",
        "/capture_completeness",
        "/privacy",
    ] {
        let mut fixture = episode();
        fixture.pointer_mut(path).unwrap()["unexpected"] = json!(true);
        assert!(decode(fixture).is_err(), "accepted extra field at {path}");
    }
    for path in [
        "/product_eligibility",
        "/ranking_observation",
        "/display_decision",
        "/exposure",
    ] {
        reject_at(
            path,
            json!({"state": "NOT_APPLICABLE", "reason": "PLANNING_NOT_ADMITTED", "records": []}),
        );
    }
    reject_at(
        "/capture_completeness",
        json!({"state": "COMPLETE", "reasons": []}),
    );
}

#[test]
fn duplicate_members_reject_before_acquiring_domain_status() {
    let serialized = serde_json::to_string(&episode()).unwrap();
    for needle in [
        r#""outcome":null"#,
        r#""parent_episode_id":null"#,
        r#""model_artifact":null"#,
        r#""candidate_id":"candidate-1""#,
        r#""kind":"SYNTHETIC_FIXTURE""#,
        r#""major":0"#,
    ] {
        let duplicate = serialized.replacen(needle, &format!("{needle},{needle}"), 1);
        assert_ne!(serialized, duplicate);
        assert!(
            serde_json::from_str::<EvaluationEpisodeV0_1>(&duplicate).is_err(),
            "accepted duplicate {needle}"
        );
    }
}

#[test]
fn native_counters_are_canonical_u64_strings_but_slice_one_features_stay_numeric() {
    for path in [
        "/interaction_events/0/sequence",
        "/candidates/0/deterministic_rank_in_discovered",
        "/candidate_generation/search_visit/declared_hypothesis_count",
        "/candidate_generation/search_visit/evaluated_positions",
    ] {
        for bad in [
            json!(1),
            json!(-1),
            json!("01"),
            json!("+1"),
            json!("18446744073709551616"),
        ] {
            reject_at(path, bad);
        }
    }
    let maximum: CanonicalU64 = serde_json::from_value(json!(u64::MAX.to_string())).unwrap();
    assert_eq!(maximum.value(), u64::MAX);
    reject_at(
        "/candidates/0/ranking_feature_snapshot/mutation_count",
        json!("0"),
    );
    reject_at(
        "/candidates/0/deterministic_ordering_key/preference_distance_seconds",
        json!("0"),
    );
}

#[test]
fn immutable_reference_variants_and_fixture_digest_binding_are_closed() {
    let synthetic = binding("PLANNER_ALGORITHM");
    for reference in [
        synthetic["immutable_ref"].clone(),
        json!({"kind": "GIT_COMMIT", "repository_id": "cerebri", "commit_sha": "a".repeat(40),
            "manifest_path": "artifacts/manifest.json"}),
        json!({"kind": "CONTENT_ADDRESSED", "locator": "https://store.invalid/bundles",
            "content_address": format!("sha256:{}", "a".repeat(64))}),
    ] {
        let mut fixture = synthetic.clone();
        fixture["immutable_ref"] = reference.clone();
        assert!(serde_json::from_value::<ArtifactBindingV0_1>(fixture.clone()).is_ok());
        for key in reference.as_object().unwrap().keys() {
            let mut missing = fixture.clone();
            missing["immutable_ref"]
                .as_object_mut()
                .unwrap()
                .remove(key);
            assert!(serde_json::from_value::<ArtifactBindingV0_1>(missing).is_err());
        }
        fixture["immutable_ref"]["wrong_variant_field"] = json!(true);
        assert!(serde_json::from_value::<ArtifactBindingV0_1>(fixture).is_err());
    }
    let mut mismatch = synthetic;
    mismatch["immutable_ref"]["manifest_sha256"] = json!("b".repeat(64));
    assert!(serde_json::from_value::<ArtifactBindingV0_1>(mismatch).is_err());
    for reference in [
        json!("main"),
        json!("v0.1"),
        json!("https://store.invalid/latest"),
        json!("C:\\local.json"),
        json!({"kind": "GIT_COMMIT", "repository_id": "cerebri", "commit_sha": "main", "manifest_path": "manifest.json"}),
        json!({"kind": "GIT_COMMIT", "repository_id": "cerebri", "commit_sha": "a".repeat(40), "manifest_path": "../manifest.json"}),
        json!({"kind": "CONTENT_ADDRESSED", "locator": "store", "content_address": "https://store.invalid/latest"}),
        json!({"kind": "CONTENT_ADDRESSED", "locator": "C:\\store", "content_address": "pin"}),
        json!({"kind": "CONTENT_ADDRESSED", "locator": "store", "content_address": ""}),
    ] {
        let mut fixture = binding("PLANNER_ALGORITHM");
        fixture["immutable_ref"] = reference;
        assert!(serde_json::from_value::<ArtifactBindingV0_1>(fixture).is_err());
    }
}

#[test]
fn not_admitted_variant_has_no_search_or_proof_members() {
    let mut fixture = episode();
    fixture["candidate_generation"] = json!({"state": "NOT_ADMITTED",
        "planner_admission": {"admission_class": "INSUFFICIENT_INFORMATION", "issue_codes": ["REQUIRED_DURATION_MISSING"]},
        "outcome": "INSUFFICIENT_INFORMATION"});
    fixture["lifecycle_timestamps"]["policy_application_at"] = json!(null);
    fixture["candidates"] = json!([]);
    fixture["interaction_events"] = json!([]);
    fixture["terminal_state"] = json!("CLOSED_NOT_ADMITTED");
    for field in [
        "product_eligibility",
        "ranking_observation",
        "display_decision",
        "exposure",
    ] {
        fixture[field] = json!({"state": "NOT_APPLICABLE", "reason": "PLANNING_NOT_ADMITTED"});
    }
    assert!(decode(fixture.clone()).is_ok());
    for field in [
        "search_visit",
        "search_assessment",
        "search_exhausted",
        "proof_claim",
        "planner_run_id",
    ] {
        let mut bad = fixture.clone();
        bad["candidate_generation"][field] = json!(null);
        assert!(decode(bad).is_err());
    }
    fixture["candidate_generation"]["planner_admission"]["admission_class"] = json!("VALID");
    assert!(decode(fixture).is_err());
}

#[test]
fn foundation_validation_cannot_be_bypassed_by_constructing_wire_in_rust() {
    let mut raw: EvaluationEpisodeWireV0_1 = serde_json::from_value(episode()).unwrap();
    raw.lineage.parent_episode_id = Some(EpisodeId::new("parent").unwrap());
    assert!(EvaluationEpisodeV0_1::try_from(raw).is_err());
    let mut raw: EvaluationEpisodeWireV0_1 = serde_json::from_value(episode()).unwrap();
    raw.candidates[0].placements[0].end = raw.candidates[0].placements[0].start;
    assert!(EvaluationEpisodeV0_1::try_from(raw).is_err());
    reject_at("/lifecycle_timestamps/policy_application_at", json!(null));
    reject_at(
        "/product_eligibility/records/0/exclusion_rule_id",
        json!("rule-1"),
    );
}

#[test]
fn reused_run_allows_original_timestamps_before_child_start() {
    let mut child = episode();
    child["lineage"] = json!({"family_id": "family-1", "parent_episode_id": "parent",
        "transition_cause": {"context_changed": false, "explicit_replan": false, "new_visible_policy_application": true},
        "transition_reasons": ["NEW_VISIBLE_POLICY_APPLICATION"]});
    child["planner_run_binding"]["run_use"] = json!("REUSED_RUN");
    child["planner_run_binding"]["reused_from_episode_id"] = json!("parent");
    child["lifecycle_timestamps"]["episode_started_at"] = json!("1970-01-01T00:00:03Z");
    assert!(decode(child.clone()).is_ok());
    child["planner_run_binding"]["reused_from_episode_id"] = json!("different-parent");
    assert!(decode(child).is_err());
}

#[test]
fn manual_replacement_and_unexposed_shapes_remain_closed() {
    let replacement = json!({"replacement_id": "replacement-1", "observed_at": "1970-01-01T00:00:04Z",
        "provenance": "USER_EXPLICIT", "placements": [placement()], "validation_admission_class": "VALID",
        "ranking_feature_snapshot": ranking()});
    assert!(
        serde_json::from_value::<ManualReplacementObservationV0_1>(replacement.clone()).is_ok()
    );
    let mut bad = replacement;
    bad["validation_admission_class"] = json!("INSUFFICIENT_INFORMATION");
    assert!(serde_json::from_value::<ManualReplacementObservationV0_1>(bad).is_err());
    assert!(
        serde_json::from_value::<ExposureObservationV0_1>(
            json!({"state": "NOT_EXPOSED", "reason": "shadow"})
        )
        .is_ok()
    );
    assert!(
        serde_json::from_value::<ExposureObservationV0_1>(
            json!({"state": "NOT_EXPOSED", "reason": "shadow", "displayed_order": []})
        )
        .is_err()
    );
}
