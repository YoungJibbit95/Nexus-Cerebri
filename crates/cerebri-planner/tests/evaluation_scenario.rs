#[path = "../../../tests/support/mod.rs"]
mod support;
use cerebri_constraints::{ConstraintSpec, Fact, FactValue, HardConstraint};
use cerebri_planner::{evaluation::*, *};
use cerebri_temporal::{Duration, TimeRange};
use cerebri_types::*;
use serde_json::{Value, json};

fn b1() -> PlanningRequest {
    let mut r = support::request();
    let mut o = r.context.objects[0].clone();
    o.id = PlanningObjectId::new("new-event").unwrap();
    o.revision = None;
    o.calendar_id = CalendarId::new("primary-calendar").unwrap();
    o.integration_id = IntegrationId::new("calendar-provider").unwrap();
    o.resource_ids.clear();
    o.kind = PlanningObjectKind::Event;
    o.time = EvidenceField {
        value: FieldState::Resolved(Knowledge::Missing),
        provenance: Provenance::UserExplicit,
        confidence: None,
        evidence: vec![],
    };
    o.timezone = cerebri_temporal::TimeZoneId::UTC;
    r.schema_version = SchemaVersion { major: 0, minor: 2 };
    r.operation = Operation::Analyze;
    r.scope = PlanningScope {
        time_range: TimeRange::new(
            "1970-01-01T00:00:00Z".parse().unwrap(),
            "1970-01-01T01:00:00Z".parse().unwrap(),
        )
        .unwrap(),
        calendar_ids: None,
        object_ids: None,
        resource_ids: None,
        integration_ids: None,
        movable_object_ids: None,
        max_mutations: 0,
    };
    r.target_ids = vec![o.id.clone()];
    r.context.objects = vec![o];
    r.context.facts.clear();
    r.context.temporal = None;
    r.context.revision = Revision(1);
    r.duration = EvidenceField {
        value: FieldState::known(Duration::seconds(1800).unwrap()),
        provenance: Provenance::UserExplicit,
        confidence: None,
        evidence: vec![],
    };
    r.granularity = Duration::seconds(900).unwrap();
    r.constraints.clear();
    r.preferences.preferences.clear();
    r.policy.policy_set_id = PolicySetId::new("policy-default").unwrap();
    r.policy.policy_version = Revision(1);
    r.policy.snapshot = PolicySnapshot {
        allow_uncertain_duration: false,
        confirmation: ConfirmationRequirements {
            all_mutations: false,
            deletion: false,
        },
        mode: DeploymentMode::Suggestion,
        mutation: MutationPolicy {
            allowed_actions: vec![],
            max_mutations: 0,
        },
    };
    r.planning_capability = PlanningCapability {
        read: true,
        plan: true,
        mutations: vec![],
    };
    r.budget = SearchBudget::default();
    r
}
fn projection(r: &PlanningRequest) -> BaseScenarioProjectionV1 {
    BaseScenarioProjectionV1::from_request(r).unwrap()
}
fn hash(r: &PlanningRequest) -> SHA256Hex {
    projection(r).payload().fingerprint().unwrap()
}
#[test]
fn b1_source_to_exact_bytes_and_digest() {
    let p = projection(&b1());
    assert_eq!(
        p.payload().canonical_bytes().unwrap(),
        include_bytes!("fixtures/evaluation/b1.json")
    );
    assert_eq!(
        p.payload().fingerprint().unwrap().as_str(),
        "9989e8381324dea03f300aec98ba28cfb4851a941eb8cd0f2e85682adb504b0c"
    );
    assert_eq!(p.identity_bindings().len(), 3);
    assert_eq!(p.revision_bindings().len(), 1);
    assert_eq!(
        serde_json::to_value(p.identity_bindings()).unwrap(),
        json!([
            {"alias":"v000000","identity_type":"PLANNING_OBJECT_ID","source_id":"new-event"},
            {"alias":"v000001","identity_type":"CALENDAR_ID","source_id":"primary-calendar"},
            {"alias":"v000002","identity_type":"INTEGRATION_ID","source_id":"calendar-provider"}
        ])
    );
    assert_eq!(
        serde_json::to_value(p.revision_bindings()).unwrap(),
        json!([
            {"alias":"v000000","entity_type":"PLANNING_OBJECT","revision":{"state":"NONE"}}
        ])
    );
}
#[test]
fn payload_aware_symmetry_uses_complete_scenario_before_binding_tie_break() {
    let mut r = b1();
    let mut second = r.context.objects[0].clone();
    second.id = PlanningObjectId::new("aaa-interchangeable").unwrap();
    r.context.objects.push(second);
    let p = projection(&r);
    let v = serde_json::to_value(p.payload()).unwrap();
    assert_eq!(v["targets"][0]["object"], "v000000"); // ID sorting alone would select the other object.
    assert_eq!(p.identity_bindings()[0].source_id.as_str(), "new-event");
    r.context.objects.reverse();
    assert_eq!(p.identity_bindings(), projection(&r).identity_bindings());
    r.target_ids[0] = r.context.objects[0].id.clone();
    assert_eq!(hash(&r), p.payload().fingerprint().unwrap());
    // Distinguishing an object semantically makes a target change observable.
    r.context.objects[0].kind = PlanningObjectKind::Availability;
    let before = hash(&r);
    r.target_ids[0] = r.context.objects[1].id.clone();
    assert_ne!(before, hash(&r));
}
#[test]
fn excluded_metadata_and_opaque_rename_do_not_change_bsf() {
    let r = b1();
    let expected = hash(&r);
    let mut changed = r.clone();
    changed.request_id = RequestId::new("different-request").unwrap();
    changed.trace_id = TraceId::new("different-trace").unwrap();
    changed.principal_id = PrincipalId::new("different-principal").unwrap();
    changed.context.revision = Revision(999);
    changed.context.objects[0].revision = Some(Revision(22));
    changed.context.captured_at = "2030-01-01T00:00:00Z".parse().unwrap();
    assert_eq!(expected, hash(&changed));
    assert_ne!(
        projection(&r).revision_bindings(),
        projection(&changed).revision_bindings()
    );
    changed.context.objects[0].id = PlanningObjectId::new("renamed-object").unwrap();
    changed.target_ids = vec![changed.context.objects[0].id.clone()];
    changed.context.objects[0].calendar_id = CalendarId::new("renamed-calendar").unwrap();
    changed.context.objects[0].integration_id = IntegrationId::new("renamed-integration").unwrap();
    assert_eq!(expected, hash(&changed));
    assert_ne!(
        projection(&r).identity_bindings(),
        projection(&changed).identity_bindings()
    );
}
#[test]
fn scope_duration_and_fact_subject_are_semantic_and_nested_references_are_bound() {
    let r = b1();
    let expected = hash(&r);
    let mut changed = r.clone();
    changed.scope.movable_object_ids = Some(vec![]);
    assert_ne!(expected, hash(&changed));
    changed = r.clone();
    changed.duration.value = FieldState::known(Duration::seconds(3600).unwrap());
    assert_ne!(expected, hash(&changed));
    changed = r.clone();
    changed.context.facts.push(Fact {
        id: FactId::new("fact").unwrap(),
        object_id: changed.target_ids[0].clone(),
        value: FactValue::ExternalLock,
        provenance: Provenance::IntegrationFact,
    });
    changed.duration.evidence = vec![FactId::new("ref-only-fact").unwrap()];
    changed.constraints.push(ConstraintSpec {
        object_id: changed.target_ids[0].clone(),
        rule: HardConstraint::DependencyOrder(PlanningObjectId::new("ref-only-object").unwrap()),
        evidence: vec![FactId::new("fact").unwrap()],
    });
    let before = hash(&changed);
    changed.context.facts[0].object_id = PlanningObjectId::new("ref-only-object").unwrap();
    assert_ne!(before, hash(&changed));
    let p = projection(&changed);
    assert!(
        p.identity_bindings()
            .iter()
            .any(|b| b.source_id.as_str() == "ref-only-fact")
    );
    changed.constraints.push(changed.constraints[0].clone());
    changed
        .duration
        .evidence
        .push(changed.duration.evidence[0].clone());
    assert_eq!(hash(&changed), p.payload().fingerprint().unwrap());
}
#[test]
fn duration_states_and_invalid_identity_projection() {
    let mut r = b1();
    let mut seen = std::collections::BTreeSet::new();
    for value in [
        FieldState::known(Duration::seconds(1800).unwrap()),
        FieldState::Resolved(Knowledge::Missing),
        FieldState::Resolved(Knowledge::Unknown),
        FieldState::Unresolved,
        FieldState::Resolved(Knowledge::Uncertain {
            value: Duration::seconds(1800).unwrap(),
            confidence: Confidence::new(0.5).unwrap(),
        }),
        FieldState::Resolved(Knowledge::Ambiguous(vec![
            Duration::seconds(2).unwrap(),
            Duration::seconds(10).unwrap(),
            Duration::seconds(2).unwrap(),
        ])),
    ] {
        r.duration.value = value;
        assert!(seen.insert(hash(&r)));
    }
    let v: Value = serde_json::to_value(projection(&r).payload()).unwrap();
    assert_eq!(
        v["request_duration"]["knowledge"]["seconds"],
        json!(["2", "10"])
    );
    r.duration.value =
        FieldState::Resolved(Knowledge::Ambiguous(vec![Duration::seconds(1).unwrap(); 2]));
    assert!(BaseScenarioProjectionV1::from_request(&r).is_err());
    r = b1();
    r.target_ids.push(r.target_ids[0].clone());
    assert!(BaseScenarioProjectionV1::from_request(&r).is_err());
    r = b1();
    r.context.objects.push(r.context.objects[0].clone());
    assert!(BaseScenarioProjectionV1::from_request(&r).is_err());
}

#[test]
fn complete_constraint_registry_projects_from_typed_sources() {
    use cerebri_temporal::Deadline;
    let r = b1();
    let at = r.scope.time_range.start();
    let span = r.scope.time_range;
    let rules = vec![
        HardConstraint::NoOverlap,
        HardConstraint::ExplicitTime(span),
        HardConstraint::ExplicitDate(at.date_naive()),
        HardConstraint::EarliestStart(at),
        HardConstraint::LatestEnd(span.end()),
        HardConstraint::Deadline(Deadline(at)),
        HardConstraint::MinDuration(Duration::seconds(1).unwrap()),
        HardConstraint::FixedDuration(Duration::seconds(2).unwrap()),
        HardConstraint::AvailabilityWindow(span),
        HardConstraint::DependencyOrder(PlanningObjectId::new("predecessor").unwrap()),
        HardConstraint::RequiredBuffer(Duration::seconds(3).unwrap()),
        HardConstraint::RecurrenceRule,
        HardConstraint::TimezoneIntegrity(cerebri_temporal::TimeZoneId::UTC),
        HardConstraint::ExternalLock {
            provenance: Provenance::IntegrationFact,
            reason: "excluded prose".into(),
        },
    ];
    let mut hashes = std::collections::BTreeSet::new();
    for rule in rules {
        let mut request = r.clone();
        request.constraints.push(ConstraintSpec {
            object_id: r.target_ids[0].clone(),
            rule,
            evidence: vec![],
        });
        let p = projection(&request);
        let value = serde_json::to_value(p.payload()).unwrap();
        let _: CanonicalHardConstraintRuleV1 =
            serde_json::from_value(value["hard_constraints"][0]["rule"].clone()).unwrap();
        assert!(hashes.insert(p.payload().fingerprint().unwrap()));
        if let HardConstraint::ExternalLock { reason, .. } = &mut request.constraints[0].rule {
            *reason = "different excluded prose".into();
            assert_eq!(hash(&request), p.payload().fingerprint().unwrap());
        }
    }
}

#[test]
fn temporal_series_projection_is_distinct_from_graph_and_revision_binding() {
    use cerebri_temporal::{Coverage, ExpansionLimits, PlanningHorizon};
    let mut r = b1();
    r.context.temporal=Some(TemporalContext {horizon:PlanningHorizon(r.scope.time_range),coverage:Coverage::Complete,limits:ExpansionLimits::default(),series:vec![TemporalSeries {
        id:SeriesId::new("weekly").unwrap(),state:SeriesState::Existing {revision:Revision(1)},provenance:Provenance::UserExplicit,evidence:vec![FactId::new("source").unwrap()],
        rule:serde_json::from_value(json!({"start_date":"2026-09-28","local_time":"09:30:00","timezone":"Europe/Berlin","duration":1800,"pattern":{"frequency":"WEEKLY","every":2,"weekdays":["Wed","Mon"]},"count":10,"until":"2026-12-31","gap_policy":"Skip","fold_policy":"Earlier"})).unwrap()
    }]});
    let p = projection(&r);
    let v = serde_json::to_value(p.payload()).unwrap();
    let outer = &v["temporal_context"]["series"][0];
    assert_eq!(
        outer["rule"]["pattern"]["weekdays"],
        json!(["MONDAY", "WEDNESDAY"])
    );
    let _: CanonicalTemporalSeriesV1 = serde_json::from_value(outer.clone()).unwrap();
    let graph_series = v["graph"]["vertices"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["type"] == "SERIES")
        .unwrap();
    assert_eq!(
        graph_series["attributes"][2]["key"],
        "series_recurrence_rule"
    );
    let fields = graph_series["attributes"][2]["value"]["v"]
        .as_array()
        .unwrap();
    assert!(
        fields
            .iter()
            .any(|v| v["key"] == "interval" && v["value"]["v"] == "2")
    );
    assert!(!fields.iter().any(|v| v["key"] == "pattern"));
    r.context.temporal.as_mut().unwrap().series[0].state = SeriesState::Existing {
        revision: Revision(2),
    };
    r.context.temporal.as_mut().unwrap().limits.max_dates = 1;
    assert_eq!(hash(&r), p.payload().fingerprint().unwrap());
    assert_ne!(projection(&r).revision_bindings(), p.revision_bindings());
    r.context.temporal.as_mut().unwrap().series[0].id = SeriesId::new("renamed-weekly").unwrap();
    assert_eq!(hash(&r), p.payload().fingerprint().unwrap());
    r.context.temporal.as_mut().unwrap().series[0].state = SeriesState::Prospective;
    assert_ne!(hash(&r), p.payload().fingerprint().unwrap());
    let series = r.context.temporal.as_ref().unwrap().series[0].clone();
    r.context.temporal.as_mut().unwrap().series.push(series);
    assert!(BaseScenarioProjectionV1::from_request(&r).is_err());
    r.context.temporal.as_mut().unwrap().series.pop();
    r.context.temporal.as_mut().unwrap().series[0].id = SeriesId::new("new-event").unwrap();
    assert!(BaseScenarioProjectionV1::from_request(&r).is_err());
}

#[test]
fn all_source_collections_normalize_without_losing_semantic_inputs() {
    use cerebri_preferences::{PreferenceEvidence, PreferenceSource};
    let mut r = b1();
    let mut second = r.context.objects[0].clone();
    second.id = PlanningObjectId::new("other").unwrap();
    second.kind = PlanningObjectKind::Task(TaskDetails {
        splittable: true,
        interruptible: false,
        minimum_chunk: Some(Duration::seconds(60).unwrap()),
        preferred_chunk: None,
        maximum_chunk_count: Some(3),
    });
    r.context.objects.push(second);
    r.context.objects[0].resource_ids = vec![
        ResourceId::new("room-z").unwrap(),
        ResourceId::new("room-a").unwrap(),
    ];
    r.context.facts = vec![
        Fact {
            id: FactId::new("a").unwrap(),
            object_id: r.target_ids[0].clone(),
            value: FactValue::Availability(r.scope.time_range),
            provenance: Provenance::SystemFact,
        },
        Fact {
            id: FactId::new("b").unwrap(),
            object_id: r.context.objects[1].id.clone(),
            value: FactValue::ScheduledTime(r.scope.time_range),
            provenance: Provenance::UserExplicit,
        },
    ];
    r.duration.evidence = r.context.facts.iter().map(|f| f.id.clone()).collect();
    r.context.objects[0].time.evidence = r.duration.evidence.clone();
    r.scope.object_ids = Some(r.context.objects.iter().map(|o| o.id.clone()).collect());
    r.preferences.preferences = vec![
        PreferenceEvidence {
            source: PreferenceSource::Default,
            preferred_start: r.scope.time_range.end(),
            evidence: r.duration.evidence.clone(),
        },
        PreferenceEvidence {
            source: PreferenceSource::SessionContext,
            preferred_start: r.scope.time_range.start(),
            evidence: vec![],
        },
    ];
    r.policy.snapshot.mutation.allowed_actions =
        vec![MutationKind::CreateEvent, MutationKind::MoveEvent];
    r.planning_capability.mutations = r
        .context
        .objects
        .iter()
        .map(|o| MutationGrant {
            object_id: o.id.clone(),
            calendar_id: o.calendar_id.clone(),
            integration_id: o.integration_id.clone(),
            kind: MutationKind::CreateEvent,
        })
        .collect();
    let p = projection(&r);
    let expected = p.payload().fingerprint().unwrap();
    r.context.objects.reverse();
    r.context.facts.reverse();
    r.duration.evidence.reverse();
    r.scope.object_ids.as_mut().unwrap().reverse();
    r.preferences.preferences.reverse();
    r.policy.snapshot.mutation.allowed_actions.reverse();
    r.planning_capability.mutations.reverse();
    for o in &mut r.context.objects {
        o.resource_ids.reverse();
        o.time.evidence.reverse();
    }
    for p in &mut r.preferences.preferences {
        p.evidence.reverse();
    }
    assert_eq!(hash(&r), expected);
    assert_eq!(projection(&r).identity_bindings(), p.identity_bindings());
    r.preferences.preferences[0].preferred_start = r.scope.time_range.end();
    assert_ne!(hash(&r), expected);
    r.context.facts.push(r.context.facts[0].clone());
    assert!(BaseScenarioProjectionV1::from_request(&r).is_err());
}

type RequestChange = (&'static str, fn(&mut PlanningRequest));

#[test]
fn each_excluded_metadata_field_is_independently_invariant() {
    let r = b1();
    let expected = hash(&r);
    let changes: &[RequestChange] = &[
        ("request", |r| {
            r.request_id = RequestId::new("other-request").unwrap()
        }),
        ("trace", |r| {
            r.trace_id = TraceId::new("other-trace").unwrap()
        }),
        ("principal", |r| {
            r.principal_id = PrincipalId::new("other-principal").unwrap()
        }),
        ("capture", |r| {
            r.context.captured_at = "2035-01-01T00:00:00Z".parse().unwrap()
        }),
        ("context revision", |r| {
            r.context.revision = Revision(u64::MAX)
        }),
        ("object revision", |r| {
            r.context.objects[0].revision = Some(Revision(u64::MAX))
        }),
        ("policy identity", |r| {
            r.policy.policy_set_id = PolicySetId::new("other-policy").unwrap()
        }),
        ("policy version", |r| {
            r.policy.policy_version = Revision(u64::MAX)
        }),
        ("search budget", |r| {
            r.budget = SearchBudget {
                max_candidates: 0,
                max_repairs: 100,
                max_moved_objects: 10,
                max_depth: 20,
            }
        }),
        ("duration confidence", |r| {
            r.duration.confidence = Some(Confidence::new(0.1).unwrap())
        }),
        ("object confidence", |r| {
            r.context.objects[0].time.confidence = Some(Confidence::new(0.9).unwrap())
        }),
    ];
    for (name, change) in changes {
        let mut changed = r.clone();
        change(&mut changed);
        assert_eq!(hash(&changed), expected, "{name}");
    }
    // Uncertain evidence keeps its class but excludes the raw magnitude at both levels.
    let mut r = r;
    r.duration.value = FieldState::Resolved(Knowledge::Uncertain {
        value: Duration::seconds(1800).unwrap(),
        confidence: Confidence::new(0.1).unwrap(),
    });
    r.context.objects[0].time.value = FieldState::Resolved(Knowledge::Uncertain {
        value: r.scope.time_range,
        confidence: Confidence::new(0.2).unwrap(),
    });
    let before = hash(&r);
    if let FieldState::Resolved(Knowledge::Uncertain { confidence, .. }) = &mut r.duration.value {
        *confidence = Confidence::new(0.99).unwrap();
    }
    if let FieldState::Resolved(Knowledge::Uncertain { confidence, .. }) =
        &mut r.context.objects[0].time.value
    {
        *confidence = Confidence::new(0.98).unwrap();
    }
    assert_eq!(hash(&r), before);
}

#[test]
fn duration_grammar_has_exact_six_shapes_and_rejects_collapsed_ambiguity() {
    let d = Duration::seconds(1800).unwrap();
    for (source, expected) in [
        (
            FieldState::known(d),
            json!({"state":"KNOWN","seconds":"1800"}),
        ),
        (
            FieldState::Resolved(Knowledge::Uncertain {
                value: d,
                confidence: Confidence::new(0.3).unwrap(),
            }),
            json!({"state":"UNCERTAIN","seconds":"1800"}),
        ),
        (
            FieldState::Resolved(Knowledge::Ambiguous(vec![
                Duration::seconds(10).unwrap(),
                Duration::seconds(2).unwrap(),
                Duration::seconds(10).unwrap(),
            ])),
            json!({"state":"AMBIGUOUS","seconds":["2","10"]}),
        ),
        (
            FieldState::Resolved(Knowledge::Missing),
            json!({"state":"MISSING"}),
        ),
        (
            FieldState::Resolved(Knowledge::Unknown),
            json!({"state":"UNKNOWN"}),
        ),
        (FieldState::Unresolved, json!({"state":"UNRESOLVED"})),
    ] {
        assert_eq!(
            serde_json::to_value(CanonicalDurationKnowledgeV1::try_from(&source).unwrap()).unwrap(),
            expected
        );
        let mut r = b1();
        r.duration.value = source;
        assert_eq!(
            serde_json::to_value(projection(&r).payload()).unwrap()["request_duration"]["knowledge"],
            expected
        );
    }
    for values in [vec![], vec![d], vec![d, d]] {
        let mut r = b1();
        r.duration.value = FieldState::Resolved(Knowledge::Ambiguous(values));
        assert!(BaseScenarioProjectionV1::from_request(&r).is_err());
    }
}

#[test]
fn semantic_fields_and_all_optional_scope_dimensions_affect_bsf() {
    let r = b1();
    let expected = hash(&r);
    let changes: &[RequestChange] = &[
        ("duration", |r| {
            r.duration.value = FieldState::known(Duration::seconds(3600).unwrap())
        }),
        ("operation", |r| r.operation = Operation::FindSlot),
        ("granularity", |r| {
            r.granularity = Duration::seconds(1800).unwrap()
        }),
        ("scope horizon", |r| {
            r.scope.time_range = TimeRange::new(
                r.scope.time_range.start(),
                "1970-01-01T02:00:00Z".parse().unwrap(),
            )
            .unwrap()
        }),
        ("calendar bounded empty", |r| {
            r.scope.calendar_ids = Some(vec![])
        }),
        ("object bounded empty", |r| {
            r.scope.object_ids = Some(vec![])
        }),
        ("resource bounded empty", |r| {
            r.scope.resource_ids = Some(vec![])
        }),
        ("integration bounded empty", |r| {
            r.scope.integration_ids = Some(vec![])
        }),
        ("movable bounded empty", |r| {
            r.scope.movable_object_ids = Some(vec![])
        }),
        ("movable bounded nonempty", |r| {
            r.scope.movable_object_ids = Some(r.target_ids.clone())
        }),
        ("scope mutations", |r| r.scope.max_mutations = 1),
        ("policy uncertainty", |r| {
            r.policy.snapshot.allow_uncertain_duration = true
        }),
        ("policy confirmation", |r| {
            r.policy.snapshot.confirmation.all_mutations = true
        }),
        ("policy deletion", |r| {
            r.policy.snapshot.confirmation.deletion = true
        }),
        ("policy mode", |r| {
            r.policy.snapshot.mode = DeploymentMode::Shadow
        }),
        ("policy actions", |r| {
            r.policy.snapshot.mutation.allowed_actions = vec![MutationKind::MoveEvent]
        }),
        ("policy mutation count", |r| {
            r.policy.snapshot.mutation.max_mutations = 1
        }),
        ("capability read", |r| r.planning_capability.read = false),
        ("capability plan", |r| r.planning_capability.plan = false),
        ("capability grant", |r| {
            let o = &r.context.objects[0];
            r.planning_capability.mutations.push(MutationGrant {
                object_id: o.id.clone(),
                calendar_id: o.calendar_id.clone(),
                integration_id: o.integration_id.clone(),
                kind: MutationKind::CreateEvent,
            });
        }),
        ("constraint", |r| {
            r.constraints.push(ConstraintSpec {
                object_id: r.target_ids[0].clone(),
                rule: HardConstraint::NoOverlap,
                evidence: vec![],
            })
        }),
        ("provenance", |r| {
            r.duration.provenance = Provenance::SystemFact
        }),
    ];
    for (name, change) in changes {
        let mut changed = r.clone();
        change(&mut changed);
        assert_ne!(hash(&changed), expected, "{name}");
    }
}

#[test]
fn preferences_normalize_permitted_sources_and_reject_both_learned_sources() {
    use cerebri_preferences::{PreferenceEvidence, PreferenceSource as P};
    let mut r = b1();
    let without = hash(&r);
    for source in [P::ExplicitCurrentRequest, P::SessionContext, P::Default] {
        r.preferences.preferences.push(PreferenceEvidence {
            source,
            preferred_start: r.scope.time_range.start(),
            evidence: vec![
                FactId::new("evidence-z").unwrap(),
                FactId::new("evidence-a").unwrap(),
            ],
        });
    }
    let before = hash(&r);
    assert_ne!(before, without);
    r.preferences
        .preferences
        .push(r.preferences.preferences[0].clone());
    r.preferences.preferences.reverse();
    for p in &mut r.preferences.preferences {
        p.evidence.reverse();
        p.evidence.push(p.evidence[0].clone());
    }
    assert_eq!(hash(&r), before);
    r.preferences.preferences[0].preferred_start = r.scope.time_range.end();
    assert_ne!(hash(&r), before);
    for source in [P::PersonalLearned, P::GlobalLearned] {
        r.preferences.preferences[0].source = source;
        assert!(BaseScenarioProjectionV1::from_request(&r).is_err());
    }
}

fn temporal_request() -> PlanningRequest {
    use cerebri_temporal::{Coverage, ExpansionLimits, PlanningHorizon};
    let mut r = b1();
    let rule = serde_json::from_value(json!({
        "start_date":"1970-01-01","local_time":"00:00:00","timezone":"UTC","duration":1800,
        "pattern":{"frequency":"DAILY","every":1},"count":2,"until":null,"gap_policy":"Reject","fold_policy":"Reject"
    })).unwrap();
    r.context.temporal = Some(TemporalContext {
        horizon: PlanningHorizon(r.scope.time_range),
        coverage: Coverage::Complete,
        limits: ExpansionLimits::default(),
        series: vec![TemporalSeries {
            id: SeriesId::new("daily-existing").unwrap(),
            state: SeriesState::Existing {
                revision: Revision(7),
            },
            rule,
            provenance: Provenance::IntegrationFact,
            evidence: vec![FactId::new("temporal-fact").unwrap()],
        }],
    });
    r
}

#[test]
fn temporal_presence_revision_bindings_and_closed_binding_wire() {
    let mut r = temporal_request();
    assert_ne!(hash(&r), hash(&b1()));
    r.context.objects[0].revision = Some(Revision(u64::MAX));
    let mut prospective = r.context.temporal.as_ref().unwrap().series[0].clone();
    prospective.id = SeriesId::new("daily-prospective").unwrap();
    prospective.state = SeriesState::Prospective;
    r.context
        .temporal
        .as_mut()
        .unwrap()
        .series
        .push(prospective);
    let p = projection(&r);
    assert_eq!(p.revision_bindings().len(), 3);
    let binding_for = |id| {
        p.identity_bindings()
            .iter()
            .find(|b| b.source_id.as_str() == id)
            .unwrap()
            .alias
            .clone()
    };
    for (id, entity_type, revision) in [
        (
            "new-event",
            "PLANNING_OBJECT",
            json!({"state":"SOME","value":"18446744073709551615"}),
        ),
        (
            "daily-existing",
            "TEMPORAL_SERIES",
            json!({"state":"SOME","value":"7"}),
        ),
        (
            "daily-prospective",
            "TEMPORAL_SERIES",
            json!({"state":"NONE"}),
        ),
    ] {
        let alias = binding_for(id);
        let binding = p
            .revision_bindings()
            .iter()
            .find(|b| b.alias == alias)
            .unwrap();
        assert_eq!(
            serde_json::to_value(binding).unwrap(),
            json!({"alias":alias,"entity_type":entity_type,"revision":revision})
        );
    }
    assert!(
        p.revision_bindings()
            .windows(2)
            .all(|w| w[0].alias < w[1].alias)
    );
    r.context.temporal.as_mut().unwrap().series[0].state = SeriesState::Existing {
        revision: Revision(42),
    };
    assert_eq!(hash(&r), p.payload().fingerprint().unwrap());
    assert_ne!(projection(&r).revision_bindings(), p.revision_bindings());
    for value in [
        json!({"alias":"v000000","entity_type":"FACT","revision":{"state":"NONE"}}),
        json!({"alias":"v000000","entity_type":"CALENDAR","revision":{"state":"NONE"}}),
        json!({"alias":"v000000","entity_type":"RESOURCE","revision":{"state":"NONE"}}),
        json!({"alias":"v000000","entity_type":"INTEGRATION","revision":{"state":"NONE"}}),
        json!({"alias":"v000000","entity_type":"PLANNING_OBJECT"}),
        json!({"alias":"v000000","entity_type":"PLANNING_OBJECT","revision":null}),
        json!({"alias":"v000000","entity_type":"PLANNING_OBJECT","revision":{"state":"SOME","value":"01"}}),
        json!({"alias":"v000000","entity_type":"PLANNING_OBJECT","revision":{"state":"NONE"},"extra":true}),
    ] {
        assert!(serde_json::from_value::<RevisionBindingV1>(value).is_err());
    }
}

#[test]
fn malformed_identity_projection_never_returns_a_scenario() {
    let mut r = b1();
    r.target_ids.push(r.target_ids[0].clone());
    assert_eq!(
        BaseScenarioProjectionV1::from_request(&r)
            .unwrap_err()
            .to_string(),
        "DUPLICATE_TARGET"
    );
    r = b1();
    r.context.objects.push(r.context.objects[0].clone());
    assert_eq!(
        BaseScenarioProjectionV1::from_request(&r)
            .unwrap_err()
            .to_string(),
        "DUPLICATE_OBJECT"
    );
    r = b1();
    let fact = Fact {
        id: FactId::new("fact").unwrap(),
        object_id: r.target_ids[0].clone(),
        value: FactValue::ExternalLock,
        provenance: Provenance::UserExplicit,
    };
    r.context.facts = vec![fact.clone(), fact];
    assert_eq!(
        BaseScenarioProjectionV1::from_request(&r)
            .unwrap_err()
            .to_string(),
        "DUPLICATE_FACT"
    );
    r = temporal_request();
    let s = r.context.temporal.as_ref().unwrap().series[0].clone();
    r.context.temporal.as_mut().unwrap().series.push(s);
    assert_eq!(
        BaseScenarioProjectionV1::from_request(&r)
            .unwrap_err()
            .to_string(),
        "COMPILATION_DUPLICATE_SERIES"
    );
    r.context.temporal.as_mut().unwrap().series.pop();
    r.context.temporal.as_mut().unwrap().series[0].id =
        SeriesId::new(r.target_ids[0].as_str()).unwrap();
    assert_eq!(
        BaseScenarioProjectionV1::from_request(&r)
            .unwrap_err()
            .to_string(),
        "COMPILATION_IDENTITY_COLLISION"
    );
    r = b1();
    r.schema_version = SchemaVersion {
        major: 99,
        minor: 0,
    };
    assert_eq!(
        BaseScenarioProjectionV1::from_request(&r)
            .unwrap_err()
            .to_string(),
        "FINGERPRINT_SCHEMA_UNSUPPORTED"
    );
    // The trusted typed request boundary already rejects noncanonical identifiers.
    let mut wire = serde_json::to_value(b1()).unwrap();
    wire["context"]["objects"][0]["id"] = json!("not canonical / identity");
    assert!(serde_json::from_value::<PlanningRequest>(wire).is_err());
}

#[test]
fn base_scenario_limit_failure_leaves_normal_planner_output_unchanged() {
    let mut r = support::request();
    r.scope.resource_ids = Some(
        (0..9)
            .map(|i| ResourceId::new(format!("symmetry-{i}")).unwrap())
            .collect(),
    );
    let before = serde_json::to_value(BaselinePlanner.plan(r.clone())).unwrap();
    assert_eq!(
        BaseScenarioProjectionV1::from_request(&r)
            .unwrap_err()
            .to_string(),
        "FINGERPRINT_CANONICALIZATION_LIMIT"
    );
    assert_eq!(
        before,
        serde_json::to_value(BaselinePlanner.plan(r)).unwrap()
    );
}

#[test]
fn complete_external_ref_projection_is_permutation_and_opaque_rename_invariant() {
    use cerebri_preferences::{PreferenceEvidence, PreferenceSource};
    use std::collections::BTreeMap;
    let mut r = temporal_request();
    let mut second = r.context.objects[0].clone();
    second.id = PlanningObjectId::new("second-event").unwrap();
    second.time = EvidenceField {
        value: FieldState::known(r.scope.time_range),
        provenance: Provenance::SystemFact,
        confidence: None,
        evidence: vec![
            FactId::new("declared-fact").unwrap(),
            FactId::new("time-ref").unwrap(),
        ],
    };
    second.resource_ids = vec![
        ResourceId::new("room-a").unwrap(),
        ResourceId::new("room-b").unwrap(),
    ];
    r.context.objects.push(second);
    r.target_ids.push(r.context.objects[1].id.clone());
    r.context.facts = vec![
        Fact {
            id: FactId::new("declared-fact").unwrap(),
            object_id: r.target_ids[0].clone(),
            value: FactValue::ScheduledTime(r.scope.time_range),
            provenance: Provenance::SystemFact,
        },
        Fact {
            id: FactId::new("other-fact").unwrap(),
            object_id: r.target_ids[1].clone(),
            value: FactValue::Deadline(cerebri_temporal::Deadline(r.scope.time_range.end())),
            provenance: Provenance::UserExplicit,
        },
    ];
    let evidence = vec![r.context.facts[0].id.clone(), r.context.facts[1].id.clone()];
    r.duration.evidence = evidence.clone();
    r.constraints = vec![
        ConstraintSpec {
            object_id: r.target_ids[0].clone(),
            rule: HardConstraint::DependencyOrder(r.target_ids[1].clone()),
            evidence: evidence.clone(),
        },
        ConstraintSpec {
            object_id: r.target_ids[1].clone(),
            rule: HardConstraint::ExternalLock {
                provenance: Provenance::IntegrationFact,
                reason: "excluded".into(),
            },
            evidence: evidence.clone(),
        },
    ];
    r.preferences.preferences = [
        PreferenceSource::ExplicitCurrentRequest,
        PreferenceSource::SessionContext,
    ]
    .into_iter()
    .map(|source| PreferenceEvidence {
        source,
        preferred_start: r.scope.time_range.start(),
        evidence: evidence.clone(),
    })
    .collect();
    r.scope.calendar_ids = Some(vec![
        r.context.objects[0].calendar_id.clone(),
        CalendarId::new("external-calendar").unwrap(),
    ]);
    r.scope.integration_ids = Some(vec![
        r.context.objects[0].integration_id.clone(),
        IntegrationId::new("external-integration").unwrap(),
    ]);
    r.scope.object_ids = Some(vec![
        r.target_ids[0].clone(),
        PlanningObjectId::new("external-object").unwrap(),
    ]);
    r.scope.movable_object_ids = Some(r.target_ids.clone());
    r.scope.resource_ids = Some(r.context.objects[1].resource_ids.clone());
    r.planning_capability.mutations = vec![
        MutationGrant {
            object_id: r.target_ids[0].clone(),
            calendar_id: r.context.objects[0].calendar_id.clone(),
            integration_id: r.context.objects[0].integration_id.clone(),
            kind: MutationKind::CreateEvent,
        },
        MutationGrant {
            object_id: PlanningObjectId::new("external-object").unwrap(),
            calendar_id: CalendarId::new("external-calendar").unwrap(),
            integration_id: IntegrationId::new("external-integration").unwrap(),
            kind: MutationKind::UpdateEvent,
        },
    ];
    r.policy.snapshot.mutation.allowed_actions =
        vec![MutationKind::CreateEvent, MutationKind::UpdateEvent];
    let mut second_series = r.context.temporal.as_ref().unwrap().series[0].clone();
    second_series.id = SeriesId::new("other-series").unwrap();
    second_series.provenance = Provenance::SystemFact;
    second_series.evidence = evidence;
    r.context
        .temporal
        .as_mut()
        .unwrap()
        .series
        .push(second_series);
    let p = projection(&r);
    let bytes = p.payload().canonical_bytes().unwrap();
    let mut wire = serde_json::to_value(&r).unwrap();
    fn reverse_arrays(value: &mut Value) {
        match value {
            Value::Array(values) => {
                values.reverse();
                for v in values {
                    reverse_arrays(v);
                }
            }
            Value::Object(values) => {
                for v in values.values_mut() {
                    reverse_arrays(v);
                }
            }
            _ => {}
        }
    }
    reverse_arrays(&mut wire);
    let permuted: PlanningRequest = serde_json::from_value(wire.clone()).unwrap();
    let q = projection(&permuted);
    assert_eq!(bytes, q.payload().canonical_bytes().unwrap());
    assert_eq!(p.identity_bindings(), q.identity_bindings());
    assert_eq!(p.revision_bindings(), q.revision_bindings());
    assert!(p.identity_bindings().len() > 10);
    for (i, b) in p.identity_bindings().iter().enumerate() {
        assert_eq!(b.alias.as_str(), format!("v{i:06}"));
    }
    let renames: BTreeMap<_, _> = p
        .identity_bindings()
        .iter()
        .rev()
        .enumerate()
        .map(|(i, b)| {
            (
                b.source_id.as_str().to_owned(),
                format!("opaque-renamed-{i:06}"),
            )
        })
        .collect();
    fn rename(value: &mut Value, names: &BTreeMap<String, String>) {
        match value {
            Value::String(s) => {
                if let Some(new) = names.get(s) {
                    *s = new.clone();
                }
            }
            Value::Array(values) => {
                for v in values {
                    rename(v, names);
                }
            }
            Value::Object(values) => {
                for v in values.values_mut() {
                    rename(v, names);
                }
            }
            _ => {}
        }
    }
    rename(&mut wire, &renames);
    let renamed: PlanningRequest = serde_json::from_value(wire).unwrap();
    let q = projection(&renamed);
    assert_eq!(bytes, q.payload().canonical_bytes().unwrap());
    assert_ne!(p.identity_bindings(), q.identity_bindings());
    // Every external binding is rewritten; none of the source identity strings
    // survives as a payload value (including evidence, grants and scope filters).
    let mut payload = serde_json::to_value(p.payload()).unwrap();
    let unchanged = payload.clone();
    rename(&mut payload, &renames);
    assert_eq!(payload, unchanged);
}
