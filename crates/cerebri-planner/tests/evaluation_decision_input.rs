#[path = "../../../tests/support/mod.rs"]
mod support;
use cerebri_constraints::{ConstraintSpec, HardConstraint};
use cerebri_planner::{evaluation::*, *};
use cerebri_preferences::{PreferenceEvidence, PreferenceSource};
use cerebri_temporal::{Coverage, ExpansionLimits, PlanningHorizon};
use cerebri_types::*;
use serde_json::{Value, json};

fn decimal(v: u64) -> CanonicalU64Decimal {
    CanonicalU64Decimal::new(v).unwrap()
}
fn id(v: &str) -> IdentifierString {
    IdentifierString::new(v).unwrap()
}
fn artifact(
    class: ArtifactClassToken,
    name: &str,
    version: &str,
    digit: &str,
) -> ArtifactSemanticIdentityV0_1 {
    ArtifactSemanticIdentityV0_1 {
        artifact_class: class,
        artifact_id: id(name),
        artifact_version: id(version),
        manifest_schema_version: id("0.1"),
        manifest_sha256: SHA256Hex::new(digit.repeat(64)).unwrap(),
    }
}
// External architecture-vector identities, never live manifests or production defaults.
fn artifacts() -> DecisionInputArtifactsV1 {
    DecisionInputArtifactsV1 {
        planner_artifact: artifact(
            ArtifactClassToken::PlannerAlgorithm,
            "cerebri-planner",
            "0.2.0",
            "1",
        ),
        generator_artifact: artifact(
            ArtifactClassToken::GeneratorPolicy,
            "grid-generator",
            "0.1",
            "2",
        ),
        generator_configuration_artifact: CanonicalOptionalV1::None {},
        product_eligibility_policy: artifact(
            ArtifactClassToken::ProductEligibilityPolicy,
            "deterministic-eligibility",
            "0.1",
            "3",
        ),
        ranking_policy: artifact(
            ArtifactClassToken::RankingPolicy,
            "legacy-deterministic-ranking",
            "0.1",
            "4",
        ),
        display_policy: artifact(
            ArtifactClassToken::DisplayPolicy,
            "deterministic-display",
            "0.1",
            "5",
        ),
    }
}
fn project(request: &PlanningRequest) -> DecisionInputPayloadV1 {
    DecisionInputProjectionV1::from_request(request, artifacts())
        .unwrap()
        .payload()
        .clone()
}
fn d1() -> DecisionInputPayloadV1 {
    let a = artifacts();
    // Construct §56.5's semantic input independently of d1.json and source measurement.
    DecisionInputWireV1 {
        schema_version: DecisionInputSchemaVersionV1::V1,
        base_scenario_fingerprint: SHA256Hex::new(
            "9989e8381324dea03f300aec98ba28cfb4851a941eb8cd0f2e85682adb504b0c",
        )
        .unwrap(),
        identity_bindings: [
            (IdentityTypeV1::PlanningObjectId, "new-event"),
            (IdentityTypeV1::CalendarId, "primary-calendar"),
            (IdentityTypeV1::IntegrationId, "calendar-provider"),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, (identity_type, source))| IdentityBindingV1 {
            alias: CanonicalAlias::new(i as u32).unwrap(),
            identity_type,
            source_id: id(source),
        })
        .collect(),
        revision_bindings: vec![RevisionBindingV1 {
            alias: CanonicalAlias::new(0).unwrap(),
            entity_type: RevisionEntityTypeV1::PlanningObject,
            revision: CanonicalOptionalV1::None {},
        }],
        context_source_revision: decimal(1),
        planner_artifact: a.planner_artifact,
        generator_artifact: a.generator_artifact,
        generator_configuration_artifact: a.generator_configuration_artifact,
        search_budget: CanonicalSearchBudgetV1 {
            max_candidates: decimal(256),
            max_depth: decimal(0),
            max_moved_objects: decimal(1),
            max_repairs: decimal(0),
        },
        temporal_expansion_limits: CanonicalOptionalV1::None {},
        policy_identity: CanonicalPolicyIdentityV1 {
            policy_set_id: id("policy-default"),
            policy_version: decimal(1),
        },
        active_preferences: vec![],
        product_eligibility_policy: a.product_eligibility_policy,
        ranking_policy: a.ranking_policy,
        display_policy: a.display_policy,
        model_artifact: CanonicalOptionalV1::None {},
        experiment_assignment: CanonicalOptionalV1::None {},
        planner_admission_work: PlannerAdmissionWorkWireV0_1 {
            metric_version: PlannerAdmissionWorkMetricV0_1::RustSerdeJson1_0_151PlanningRequestV0_1,
            serialized_request_bytes: SerializedRequestMeasurementV0_1::Measured {
                value: decimal(1024),
            },
            serialized_request_limit_bytes: decimal(262144),
            candidate_weighted_bytes: CanonicalOptionalV1::Some {
                value: decimal(262144),
            },
            candidate_weighted_limit_bytes: decimal(16777216),
        }
        .try_into()
        .unwrap(),
    }
    .try_into()
    .unwrap()
}

#[test]
fn d1_semantic_input_to_external_literal_and_domain_digest() {
    let payload = d1();
    let bytes = payload.canonical_bytes().unwrap();
    assert_eq!(bytes.len(), 2389);
    assert_eq!(bytes, include_bytes!("fixtures/evaluation/d1.json"));
    assert_eq!(
        payload.fingerprint().unwrap().as_str(),
        "5c121f1a9a588ea01567042aea696691b17acabf814ee4eb3e7d115971ba48aa"
    );
    assert_eq!(
        serde_json::from_slice::<DecisionInputPayloadV1>(&bytes).unwrap(),
        payload
    );
}

#[test]
fn production_uses_fresh_bsf_selected_bindings_and_shared_work() {
    let mut r = support::request();
    // Reference-only identities must survive into DI along with declared identities.
    r.duration
        .evidence
        .push(FactId::new("ref-only-fact").unwrap());
    let b = BaseScenarioProjectionV1::from_request(&r).unwrap();
    let d = project(&r);
    assert_eq!(
        d.wire().base_scenario_fingerprint,
        b.payload().fingerprint().unwrap()
    );
    assert_eq!(d.wire().identity_bindings, b.identity_bindings());
    assert_eq!(d.wire().revision_bindings, b.revision_bindings());
    assert!(
        d.wire()
            .identity_bindings
            .iter()
            .any(|b| b.source_id.as_str() == "ref-only-fact")
    );
    assert_eq!(
        d.wire().planner_admission_work,
        PlannerAdmissionWorkObservationV0_1::measure(&r)
    );
    d.wire()
        .planner_admission_work
        .validate_budget(r.budget.max_candidates)
        .unwrap();
    let bytes = serde_json::to_vec(&r).unwrap().len() as u64;
    assert_eq!(
        d.wire()
            .planner_admission_work
            .wire()
            .serialized_request_bytes,
        SerializedRequestMeasurementV0_1::Measured {
            value: decimal(bytes)
        }
    );
    assert_ne!(bytes, 1024); // D1's supplied observation is not a production constant.
    assert_eq!(d, project(&r));
}

fn assert_di_only_changes(before: &PlanningRequest, after: &PlanningRequest) {
    let a = project(before);
    let b = project(after);
    assert_eq!(
        a.wire().base_scenario_fingerprint,
        b.wire().base_scenario_fingerprint
    );
    assert_ne!(a.canonical_bytes().unwrap(), b.canonical_bytes().unwrap());
    assert_ne!(a.fingerprint().unwrap(), b.fingerprint().unwrap());
}
#[test]
fn context_and_object_revisions_are_di_only_authorities() {
    let r = support::request();
    let mut changed = r.clone();
    changed.context.revision.0 += 1;
    assert_di_only_changes(&r, &changed);
    assert_eq!(
        project(&changed).wire().context_source_revision.value(),
        changed.context.revision.0
    );
    changed = r.clone();
    changed.context.objects[0].revision = Some(Revision(100));
    assert_di_only_changes(&r, &changed);
    assert_ne!(
        project(&r).wire().revision_bindings,
        project(&changed).wire().revision_bindings
    );
}

#[test]
fn each_source_search_budget_field_is_bound_without_affecting_bsf() {
    let r = support::request();
    for field in [
        "max_candidates",
        "max_depth",
        "max_moved_objects",
        "max_repairs",
    ] {
        let mut wire = serde_json::to_value(&r).unwrap();
        let n = wire["budget"][field].as_u64().unwrap() + 1;
        wire["budget"][field] = json!(n);
        let changed: PlanningRequest = serde_json::from_value(wire).unwrap();
        assert_di_only_changes(&r, &changed);
        assert_eq!(
            serde_json::to_value(project(&changed)).unwrap()["search_budget"][field],
            n.to_string()
        );
    }
}

fn temporal_request() -> PlanningRequest {
    let mut r = support::request();
    r.schema_version = SchemaVersion::CPIR_0_2;
    r.context.temporal = Some(TemporalContext {
        horizon: PlanningHorizon(r.scope.time_range),
        coverage: Coverage::Complete,
        limits: ExpansionLimits::default(),
        series: vec![],
    });
    r
}
#[test]
fn temporal_none_some_limits_and_existing_series_revision() {
    assert_eq!(
        project(&support::request())
            .wire()
            .temporal_expansion_limits,
        CanonicalOptionalV1::None {}
    );
    let r = temporal_request();
    assert_eq!(
        project(&r).wire().temporal_expansion_limits,
        CanonicalOptionalV1::Some {
            value: CanonicalExpansionLimitsV1 {
                max_dates: decimal(36600),
                max_occurrences: decimal(1000),
            }
        }
    );
    let mut changed = r.clone();
    changed.context.temporal.as_mut().unwrap().limits.max_dates -= 1;
    assert_di_only_changes(&r, &changed);
    changed = r.clone();
    changed
        .context
        .temporal
        .as_mut()
        .unwrap()
        .limits
        .max_occurrences -= 1;
    assert_di_only_changes(&r, &changed);

    let mut r = r;
    r.context.temporal.as_mut().unwrap().series.push(TemporalSeries {
        id: SeriesId::new("series").unwrap(), state: SeriesState::Existing { revision: Revision(7) },
        rule: serde_json::from_value(json!({
            "start_date":"2026-01-01","local_time":"09:00:00","timezone":"UTC","duration":1800,
            "pattern":{"frequency":"DAILY","every":1},"count":2,"until":null,"gap_policy":"Reject","fold_policy":"Reject"
        })).unwrap(),
        provenance: Provenance::IntegrationFact, evidence: vec![],
    });
    let mut changed = r.clone();
    changed.context.temporal.as_mut().unwrap().series[0].state = SeriesState::Existing {
        revision: Revision(8),
    };
    assert_di_only_changes(&r, &changed);
    assert_ne!(
        project(&r).wire().revision_bindings,
        project(&changed).wire().revision_bindings
    );
}

#[test]
fn both_policy_identity_fields_are_di_only_authorities() {
    let r = support::request();
    let mut changed = r.clone();
    changed.policy.policy_set_id = PolicySetId::new("other-policy").unwrap();
    assert_di_only_changes(&r, &changed);
    assert_eq!(
        project(&changed)
            .wire()
            .policy_identity
            .policy_set_id
            .as_str(),
        "other-policy"
    );
    changed = r.clone();
    changed.policy.policy_version.0 += 1;
    assert_di_only_changes(&r, &changed);
    assert_eq!(
        project(&changed)
            .wire()
            .policy_identity
            .policy_version
            .value(),
        changed.policy.policy_version.0
    );
}

fn preference_request() -> PlanningRequest {
    let mut r = support::request();
    r.preferences.preferences = [
        PreferenceSource::Default,
        PreferenceSource::SessionContext,
        PreferenceSource::ExplicitCurrentRequest,
    ]
    .into_iter()
    .map(|source| PreferenceEvidence {
        source,
        preferred_start: r.scope.time_range.start(),
        evidence: vec![
            FactId::new("fact-b").unwrap(),
            FactId::new("fact-a").unwrap(),
            FactId::new("fact-b").unwrap(),
        ],
    })
    .collect();
    r
}
#[test]
fn active_preferences_reuse_b2_order_and_reject_learned_sources() {
    let r = preference_request();
    let expected = project(&r);
    let profile = &expected.wire().active_preferences;
    assert_eq!(
        profile.iter().map(|p| p.source).collect::<Vec<_>>(),
        vec![
            PreferenceSourceToken::ExplicitCurrentRequest,
            PreferenceSourceToken::SessionContext,
            PreferenceSourceToken::Default
        ]
    );
    assert!(profile.iter().all(|p| p.evidence_fact_refs.len() == 2));
    let b = BaseScenarioProjectionV1::from_request(&r).unwrap();
    assert_eq!(
        serde_json::to_value(profile).unwrap(),
        serde_json::to_value(b.payload()).unwrap()["preferences"]
    );
    for permutation in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let mut changed = r.clone();
        changed.preferences.preferences = permutation
            .map(|i| r.preferences.preferences[i].clone())
            .to_vec();
        for p in &mut changed.preferences.preferences {
            p.evidence.reverse();
        }
        assert_eq!(expected, project(&changed)); // permutations retain serialized size as well.
        changed
            .preferences
            .preferences
            .push(changed.preferences.preferences[0].clone());
        let duplicate = project(&changed);
        assert_eq!(profile, &duplicate.wire().active_preferences);
        assert_eq!(
            expected.wire().base_scenario_fingerprint,
            duplicate.wire().base_scenario_fingerprint
        );
        // Source duplicate bytes still belong to admission work, even when semantic duplicates collapse.
        assert_ne!(
            expected.wire().planner_admission_work,
            duplicate.wire().planner_admission_work
        );
    }
    for source in [
        PreferenceSource::PersonalLearned,
        PreferenceSource::GlobalLearned,
    ] {
        let mut changed = r.clone();
        changed.preferences.preferences[0].source = source;
        assert!(DecisionInputProjectionV1::from_request(&changed, artifacts()).is_err());
    }
}

#[test]
fn every_artifact_semantic_field_and_role_participates_in_di() {
    let r = support::request();
    let mut initial = artifacts();
    initial.generator_configuration_artifact = CanonicalOptionalV1::Some {
        value: artifact(ArtifactClassToken::PlannerConfiguration, "config", "1", "6"),
    };
    let expected = DecisionInputProjectionV1::from_request(&r, initial.clone())
        .unwrap()
        .payload()
        .clone();
    assert_ne!(
        expected.fingerprint().unwrap(),
        project(&r).fingerprint().unwrap()
    );
    for role in 0..6 {
        for field in 0..5 {
            let mut a = initial.clone();
            let selected = match role {
                0 => &mut a.planner_artifact,
                1 => &mut a.generator_artifact,
                2 => match &mut a.generator_configuration_artifact {
                    CanonicalOptionalV1::Some { value } => value,
                    _ => unreachable!(),
                },
                3 => &mut a.product_eligibility_policy,
                4 => &mut a.ranking_policy,
                _ => &mut a.display_policy,
            };
            match field {
                0 => selected.artifact_id = id("other"),
                1 => selected.artifact_version = id("other"),
                2 => selected.manifest_schema_version = id("other"),
                3 => selected.manifest_sha256 = SHA256Hex::new("f".repeat(64)).unwrap(),
                _ => selected.artifact_class = ArtifactClassToken::SyntheticFixture,
            }
            let changed = DecisionInputProjectionV1::from_request(&r, a).unwrap();
            assert_eq!(
                expected.wire().base_scenario_fingerprint,
                changed.payload().wire().base_scenario_fingerprint
            );
            assert_ne!(
                expected.fingerprint().unwrap(),
                changed.payload().fingerprint().unwrap(),
                "role {role}, field {field}"
            );
        }
    }
}

#[test]
fn excluded_metadata_crosses_work_admission_boundary_but_keeps_bsf() {
    let mut r = support::request();
    r.budget.max_candidates = 256;
    r.constraints.push(ConstraintSpec {
        object_id: r.target_ids[0].clone(),
        evidence: vec![],
        rule: HardConstraint::ExternalLock {
            provenance: Provenance::SystemFact,
            reason: String::new(),
        },
    });
    let size = serde_json::to_vec(&r).unwrap().len();
    let HardConstraint::ExternalLock { reason, .. } = &mut r.constraints.last_mut().unwrap().rule
    else {
        unreachable!()
    };
    *reason = "x".repeat(65536 - size);
    let equal = project(&r);
    assert_eq!(
        equal
            .wire()
            .planner_admission_work
            .wire()
            .candidate_weighted_bytes,
        CanonicalOptionalV1::Some {
            value: decimal(16777216)
        }
    );
    assert!(equal.wire().planner_admission_work.admitted());
    assert!(
        !validate_request(&r)
            .issues
            .contains(&ValidationIssue::InputLimit)
    );
    let mut changed = r.clone();
    let HardConstraint::ExternalLock { reason, .. } =
        &mut changed.constraints.last_mut().unwrap().rule
    else {
        unreachable!()
    };
    reason.push('x');
    assert_di_only_changes(&r, &changed);
    let over = project(&changed);
    assert!(!over.wire().planner_admission_work.admitted());
    assert!(
        validate_request(&changed)
            .issues
            .contains(&ValidationIssue::InputLimit)
    );
    assert_eq!(
        over.wire().planner_admission_work,
        PlannerAdmissionWorkObservationV0_1::measure(&changed)
    );
    // Equal-length excluded prose leaves both measured work and DI identical.
    let HardConstraint::ExternalLock { reason, .. } =
        &mut changed.constraints.last_mut().unwrap().rule
    else {
        unreachable!()
    };
    *reason = "y".repeat(reason.len());
    assert_eq!(over, project(&changed));
    // A serialization-limit failure remains representable, with no invented B or W.
    let HardConstraint::ExternalLock { reason, .. } =
        &mut changed.constraints.last_mut().unwrap().rule
    else {
        unreachable!()
    };
    *reason = "x".repeat(262144);
    let too_large = project(&changed);
    assert_eq!(
        equal.wire().base_scenario_fingerprint,
        too_large.wire().base_scenario_fingerprint
    );
    assert_eq!(
        too_large
            .wire()
            .planner_admission_work
            .wire()
            .serialized_request_bytes,
        SerializedRequestMeasurementV0_1::ExceedsSerializationLimit {}
    );
    assert_eq!(
        too_large
            .wire()
            .planner_admission_work
            .wire()
            .candidate_weighted_bytes,
        CanonicalOptionalV1::None {}
    );
}

fn rejects(value: Value) {
    assert!(
        serde_json::from_value::<DecisionInputPayloadV1>(value.clone()).is_err(),
        "accepted {value}"
    );
}
#[test]
fn closed_wire_rejects_missing_null_unknown_positional_and_duplicate_fields() {
    let good = serde_json::to_value(d1()).unwrap();
    for field in good.as_object().unwrap().keys() {
        let mut v = good.clone();
        v.as_object_mut().unwrap().remove(field);
        rejects(v);
        let mut v = good.clone();
        v[field] = Value::Null;
        rejects(v);
    }
    for field in [
        "request_id",
        "trace_id",
        "principal_id",
        "resolver",
        "privacy",
        "retention",
        "planner_outputs",
    ] {
        let mut v = good.clone();
        v[field] = json!("forbidden");
        rejects(v);
    }
    rejects(json!(
        good.as_object().unwrap().values().collect::<Vec<_>>()
    ));
    let text = String::from_utf8(d1().canonical_bytes().unwrap()).unwrap();
    let duplicate = text.replacen(
        "\"schema_version\":\"1\"",
        "\"schema_version\":\"1\",\"schema_version\":\"1\"",
        1,
    );
    assert!(serde_json::from_str::<DecisionInputPayloadV1>(&duplicate).is_err());
    for field in [
        "planner_artifact",
        "generator_artifact",
        "search_budget",
        "policy_identity",
        "planner_admission_work",
    ] {
        let mut v = good.clone();
        v[field]["extra"] = json!(true);
        rejects(v);
    }
}

#[test]
fn invalid_scalar_binding_and_work_parity_imports_reject() {
    let good = serde_json::to_value(d1()).unwrap();
    for (path, value) in [
        ("/schema_version", json!("2")),
        ("/schema_version", json!({"1":null})),
        (
            "/planner_artifact/artifact_class",
            json!({"PLANNER_ALGORITHM":null}),
        ),
        (
            "/identity_bindings/0/identity_type",
            json!({"PLANNING_OBJECT_ID":null}),
        ),
        (
            "/revision_bindings/0/entity_type",
            json!({"PLANNING_OBJECT":null}),
        ),
        (
            "/planner_admission_work/metric_version",
            json!({"RUST_SERDE_JSON_1_0_151_PLANNING_REQUEST_V0_1":null}),
        ),
        ("/context_source_revision", json!("01")),
        ("/context_source_revision", json!(1)),
        ("/base_scenario_fingerprint", json!("A".repeat(64))),
        ("/planner_artifact/manifest_sha256", json!("bad")),
        ("/identity_bindings/0/alias", json!("v0")),
        ("/identity_bindings/0/alias", json!("v000001")),
        ("/identity_bindings/0/identity_type", json!("OTHER")),
        ("/revision_bindings/0/entity_type", json!("FACT")),
        ("/revision_bindings/0/entity_type", json!("TEMPORAL_SERIES")),
        ("/revision_bindings/0/alias", json!("v000099")),
        ("/revision_bindings/0/alias", json!("v000001")),
        (
            "/revision_bindings/0/revision",
            json!({"state":"SOME","value":"01"}),
        ),
        (
            "/revision_bindings/0/revision",
            json!({"state":"NONE","value":"1"}),
        ),
        ("/search_budget/max_candidates", json!("255")),
        ("/search_budget/max_candidates", json!("4294967296")),
        ("/search_budget/max_depth", json!("-1")),
        (
            "/temporal_expansion_limits",
            json!({"state":"SOME","value":{"max_dates":"1","max_occurrences":"01"}}),
        ),
        (
            "/model_artifact",
            json!({"state":"SOME","value":good["planner_artifact"]}),
        ),
        ("/experiment_assignment", json!({"state":"SOME","value":{}})),
    ] {
        let mut v = good.clone();
        *v.pointer_mut(path).unwrap() = value;
        rejects(v);
    }
    let mut v = good.clone();
    v["identity_bindings"].as_array_mut().unwrap().swap(0, 1);
    rejects(v);
    let mut v = good.clone();
    v["identity_bindings"][1] = v["identity_bindings"][0].clone();
    rejects(v);
    let mut v = good.clone();
    v["identity_bindings"][1]["identity_type"] = v["identity_bindings"][0]["identity_type"].clone();
    v["identity_bindings"][1]["source_id"] = v["identity_bindings"][0]["source_id"].clone();
    rejects(v);
    let mut v = good.clone();
    let revision = v["revision_bindings"][0].clone();
    v["revision_bindings"]
        .as_array_mut()
        .unwrap()
        .push(revision);
    rejects(v);
    // Typed fixture construction cannot bypass cross-field budget validation either.
    let mut wire = d1().wire().clone();
    wire.search_budget.max_candidates = decimal(255);
    assert!(DecisionInputPayloadV1::try_from(wire).is_err());
}

#[test]
fn nested_import_duplicate_members_do_not_disappear_in_syntax_validation() {
    let text = String::from_utf8(d1().canonical_bytes().unwrap()).unwrap();
    for (from, to) in [
        (
            "\"max_candidates\":\"256\"",
            "\"max_candidates\":\"256\",\"max_candidates\":\"256\"",
        ),
        (
            "\"state\":\"NONE\"",
            "\"state\":\"NONE\",\"state\":\"NONE\"",
        ),
        (
            "\"artifact_id\":\"cerebri-planner\"",
            "\"artifact_id\":\"cerebri-planner\",\"artifact_id\":\"cerebri-planner\"",
        ),
    ] {
        let changed = text.replacen(from, to, 1);
        assert_ne!(changed, text);
        assert!(serde_json::from_str::<DecisionInputPayloadV1>(&changed).is_err());
    }
}

#[test]
fn imported_preferences_must_already_be_canonical_and_e2() {
    let good = serde_json::to_value(project(&preference_request())).unwrap();
    assert!(serde_json::from_value::<DecisionInputPayloadV1>(good.clone()).is_ok());
    let mut v = good.clone();
    v["active_preferences"].as_array_mut().unwrap().reverse();
    rejects(v);
    let mut v = good.clone();
    let p = v["active_preferences"][0].clone();
    v["active_preferences"].as_array_mut().unwrap().insert(0, p);
    rejects(v);
    for source in ["PERSONAL_LEARNED", "GLOBAL_LEARNED", "OTHER"] {
        let mut v = good.clone();
        v["active_preferences"][0]["source"] = json!(source);
        rejects(v);
    }
    let mut v = good.clone();
    v["active_preferences"][0]["source"] = json!({"EXPLICIT_CURRENT_REQUEST":null});
    rejects(v);
    let mut v = good.clone();
    v["active_preferences"][0]["evidence_fact_refs"]
        .as_array_mut()
        .unwrap()
        .reverse();
    rejects(v);
    let mut v = good.clone();
    let a = v["active_preferences"][0]["evidence_fact_refs"][0].clone();
    v["active_preferences"][0]["evidence_fact_refs"] = json!([a, a]);
    rejects(v);
    for alias in ["v000000", "v999999"] {
        let mut v = good.clone();
        v["active_preferences"][0]["evidence_fact_refs"] = json!([alias]);
        rejects(v);
    }
}

#[test]
fn b2_nonfingerprintability_cannot_receive_a_decision_input() {
    let mut r = support::request();
    r.context.objects.push(r.context.objects[0].clone());
    assert_eq!(
        DecisionInputProjectionV1::from_request(&r, artifacts())
            .unwrap_err()
            .to_string(),
        "DUPLICATE_OBJECT"
    );
    let mut r = support::request();
    r.schema_version = SchemaVersion {
        major: 99,
        minor: 0,
    };
    assert_eq!(
        DecisionInputProjectionV1::from_request(&r, artifacts())
            .unwrap_err()
            .to_string(),
        "FINGERPRINT_SCHEMA_UNSUPPORTED"
    );
}
