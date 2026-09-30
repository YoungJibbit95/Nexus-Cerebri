use cerebri_planner::evaluation::*;
use serde_json::{Value, json};
#[path = "../../../tests/support/mod.rs"]
mod support;
fn val(v: Value) -> CanonicalValueV1 {
    serde_json::from_value(v).unwrap()
}
fn attr(key: &str, value: Value) -> CanonicalAttributeV1 {
    CanonicalAttributeV1 {
        key: key.into(),
        value: val(value),
    }
}
fn object(id: &str, kind: &str) -> GraphSourceVertexV1 {
    GraphSourceVertexV1 {
        source_id: IdentifierString::new(id).unwrap(),
        vertex_type: GraphVertexTypeToken::Object,
        attributes: vec![
            attr("entity_presence", json!({"t":"TOKEN","v":"DECLARED"})),
            attr("object_kind", json!({"t":"TOKEN","v":kind})),
            attr("object_kind_details", json!({"t":"NONE"})),
            attr(
                "object_time_evidence",
                json!({"t":"OBJECT","v":[
                {"key":"knowledge_state","value":{"t":"TOKEN","v":"MISSING"}},
                {"key":"provenance","value":{"t":"TOKEN","v":"USER_EXPLICIT"}},
                {"key":"values","value":{"t":"LIST","v":[]}}]}),
            ),
            attr("object_timezone", json!({"t":"TEXT","v":"UTC"})),
        ],
    }
}
fn resource(id: &str) -> GraphSourceVertexV1 {
    GraphSourceVertexV1 {
        source_id: IdentifierString::new(id).unwrap(),
        vertex_type: GraphVertexTypeToken::Resource,
        attributes: vec![attr(
            "entity_presence",
            json!({"t":"TOKEN","v":"REFERENCE_ONLY"}),
        )],
    }
}
fn fact(id: &str, seconds: Option<u64>) -> GraphSourceVertexV1 {
    let value = match seconds {
        Some(end) => json!({"t":"OBJECT","v":[
        {"key":"end","value":{"t":"INSTANT","s":end.to_string(),"ns":"000000000"}},
        {"key":"start","value":{"t":"INSTANT","s":"0","ns":"000000000"}}]}),
        None => json!({"t":"NONE"}),
    };
    GraphSourceVertexV1 {
        source_id: IdentifierString::new(id).unwrap(),
        vertex_type: GraphVertexTypeToken::Fact,
        attributes: vec![
            attr("entity_presence", json!({"t":"TOKEN","v":"DECLARED"})),
            attr("fact_provenance", json!({"t":"TOKEN","v":"SYSTEM_FACT"})),
            attr(
                "fact_value",
                json!({"t":"OBJECT","v":[{"key":"kind","value":{"t":"TOKEN","v":if seconds.is_some(){"AVAILABILITY"}else{"EXTERNAL_LOCK"}}},{"key":"value","value":value}]}),
            ),
        ],
    }
}
fn edge(src: usize, dst: usize, kind: GraphEdgeTypeToken) -> GraphSourceEdgeV1 {
    GraphSourceEdgeV1 {
        src,
        dst,
        edge_type: kind,
        multiplicity: 1,
        payload: if kind == GraphEdgeTypeToken::EvidenceFactReference {
            vec![attr(
                "evidence_role",
                json!({"t":"TOKEN","v":"OBJECT_TIME_EVIDENCE"}),
            )]
        } else {
            vec![]
        },
    }
}
type GraphFixture = (
    Vec<GraphSourceVertexV1>,
    Vec<GraphSourceEdgeV1>,
    &'static [u8],
    &'static str,
);
fn fixtures() -> Vec<GraphFixture> {
    use GraphEdgeTypeToken::*;
    vec![
        (
            vec![
                object("one", "EVENT"),
                object("two", "EVENT"),
                resource("room-a"),
                resource("room-b"),
            ],
            vec![
                edge(0, 1, DependsOnObject),
                edge(1, 0, DependsOnObject),
                edge(0, 2, ObjectResource),
                edge(1, 3, ObjectResource),
            ],
            include_bytes!("fixtures/evaluation/g1.json"),
            "623d6faba158e6e1cefeefd261bf1dcd85936319ac3243f2766bb7f8db20e9cf",
        ),
        (
            vec![
                object("target", "EVENT"),
                object("existing", "EVENT"),
                fact("availability-fact", Some(3600)),
                resource("room"),
            ],
            vec![
                edge(0, 1, DependsOnObject),
                edge(0, 3, ObjectResource),
                edge(1, 3, ObjectResource),
                edge(0, 2, EvidenceFactReference),
                edge(1, 2, EvidenceFactReference),
                edge(2, 0, FactSubject),
            ],
            include_bytes!("fixtures/evaluation/g2.json"),
            "9788f47276e36901ce5a827f87bac2f9886ca97aae364d150f33ac8dbe02703b",
        ),
        (
            vec![
                object("predecessor-low", "AVAILABILITY"),
                object("subject-b", "EVENT"),
                object("subject-a", "EVENT"),
                object("predecessor-high", "RESOURCE"),
                fact("fact-low", Some(60)),
                fact("fact-high", None),
            ],
            vec![
                edge(3, 2, DependsOnObject),
                edge(4, 2, FactSubject),
                edge(0, 1, DependsOnObject),
                edge(5, 1, FactSubject),
            ],
            include_bytes!("fixtures/evaluation/g3.json"),
            "42b7ee4c83a8068476b40ca563c80f34ce8df25dc8922f6a9b65070c2f8dac8e",
        ),
    ]
}
#[test]
fn g1_g2_g3_source_to_bytes_and_digest() {
    for (vertices, edges, expected, digest) in fixtures() {
        let result = CanonicalGraphSourceV1::new(vertices, edges)
            .unwrap()
            .canonicalize()
            .unwrap();
        assert_eq!(
            String::from_utf8(result.payload_bytes.clone()).unwrap(),
            String::from_utf8(expected.to_vec()).unwrap()
        );
        assert_eq!(
            FingerprintDomainV1::CanonicalGraph
                .digest_bytes(&result.payload_bytes)
                .as_str(),
            digest
        );
    }
}
#[test]
fn graph_permutation_and_opaque_renaming_preserve_bytes_and_stable_bindings() {
    for (vertices, edges, expected, _) in fixtures() {
        let baseline = CanonicalGraphSourceV1::new(vertices.clone(), edges.clone())
            .unwrap()
            .canonicalize()
            .unwrap();
        for rename in [false, true] {
            let n = vertices.len();
            let mut vs = vertices.clone();
            vs.reverse();
            for (i, v) in vs.iter_mut().enumerate() {
                v.attributes.reverse();
                if rename {
                    v.source_id = IdentifierString::new(format!("renamed-{i}")).unwrap();
                }
            }
            let es = edges
                .iter()
                .rev()
                .map(|e| GraphSourceEdgeV1 {
                    src: n - 1 - e.src,
                    dst: n - 1 - e.dst,
                    ..e.clone()
                })
                .collect();
            let actual = CanonicalGraphSourceV1::new(vs, es)
                .unwrap()
                .canonicalize()
                .unwrap();
            assert_eq!(actual.payload_bytes, expected);
            if !rename {
                assert_eq!(actual.identity_bindings, baseline.identity_bindings);
            }
        }
    }
}
#[test]
fn registry_rejects_unsupported_shapes_relations_and_id_collisions() {
    assert!(serde_json::from_str::<GraphEdgeTypeToken>("\"OBJECT_SERIES\"").is_err());
    assert!(serde_json::from_str::<GraphVertexTypeToken>("\"OCCURRENCE\"").is_err());
    let mut v = object("a", "EVENT");
    v.attributes.push(attr("unknown", json!({"t":"NONE"})));
    assert!(CanonicalGraphSourceV1::new(vec![v], vec![]).is_err());
    assert!(
        CanonicalGraphSourceV1::new(vec![object("a", "EVENT"), object("a", "EVENT")], vec![])
            .is_err()
    );
    assert!(CanonicalGraphSourceV1::new(vec![fact("f", None)], vec![]).is_err());
    assert!(
        CanonicalGraphSourceV1::new(
            vec![resource("r"), object("a", "EVENT")],
            vec![edge(0, 1, GraphEdgeTypeToken::ObjectResource)]
        )
        .is_err()
    );
    let mut e = edge(0, 1, GraphEdgeTypeToken::ObjectResource);
    e.multiplicity = 0;
    assert!(
        CanonicalGraphSourceV1::new(vec![object("a", "EVENT"), resource("r")], vec![e]).is_err()
    );
}
#[test]
fn declared_presence_and_multiplicity_are_semantic() {
    let a = CanonicalGraphSourceV1::new(vec![resource("r")], vec![])
        .unwrap()
        .canonicalize()
        .unwrap();
    let mut v = resource("r");
    v.attributes[0].value = val(json!({"t":"TOKEN","v":"DECLARED"}));
    let b = CanonicalGraphSourceV1::new(vec![v], vec![])
        .unwrap()
        .canonicalize()
        .unwrap();
    assert_ne!(a.payload_bytes, b.payload_bytes);
    let vs = vec![object("a", "EVENT"), resource("r")];
    let e = edge(0, 1, GraphEdgeTypeToken::ObjectResource);
    let a = CanonicalGraphSourceV1::new(vs.clone(), vec![e.clone()])
        .unwrap()
        .canonicalize()
        .unwrap();
    let b = CanonicalGraphSourceV1::new(vs, vec![e.clone(), e])
        .unwrap()
        .canonicalize()
        .unwrap();
    assert_ne!(a.payload_bytes, b.payload_bytes);
    assert_eq!(b.graph.edges()[0].multiplicity.value(), 2);
}
#[test]
fn actual_100000_state_limit_returns_no_partial_fingerprint() {
    use cerebri_planner::{BaselinePlanner, Planner};
    let request = support::request();
    let before = serde_json::to_value(BaselinePlanner.plan(request.clone())).unwrap();
    let graph =
        CanonicalGraphSourceV1::new((0..9).map(|i| resource(&format!("r{i}"))).collect(), vec![])
            .unwrap();
    assert_eq!(
        graph.canonicalize().unwrap_err().to_string(),
        "FINGERPRINT_CANONICALIZATION_LIMIT"
    );
    assert_eq!(FINGERPRINT_CANONICALIZATION_STATE_LIMIT, 100000);
    assert_eq!(
        before,
        serde_json::to_value(BaselinePlanner.plan(request)).unwrap()
    );
}

fn identity(kind: GraphVertexTypeToken, id: &str) -> GraphSourceIdentityV1 {
    GraphSourceIdentityV1 {
        vertex_type: kind,
        source_id: IdentifierString::new(id).unwrap(),
    }
}
fn series(rule: &cerebri_temporal::RecurrenceRule) -> GraphSourceVertexV1 {
    GraphSourceVertexV1 {
        source_id: IdentifierString::new("series-opaque").unwrap(),
        vertex_type: GraphVertexTypeToken::Series,
        attributes: vec![
            attr("entity_presence", json!({"t":"TOKEN","v":"DECLARED"})),
            attr(
                "series_provenance",
                json!({"t":"TOKEN","v":"USER_EXPLICIT"}),
            ),
            CanonicalAttributeV1 {
                key: "series_recurrence_rule".into(),
                value: canonical_graph_recurrence_v1(rule).unwrap(),
            },
            attr("series_state", json!({"t":"TOKEN","v":"EXISTING"})),
        ],
    }
}
fn recurrence(weekly: bool) -> cerebri_temporal::RecurrenceRule {
    serde_json::from_value(json!({"start_date":"2026-09-28","local_time":"09:30:00","timezone":"Europe/Berlin","duration":1800,
        "pattern":if weekly {json!({"frequency":"WEEKLY","every":2,"weekdays":["Wed","Mon"]})} else {json!({"frequency":"DAILY","every":1})},
        "count":10,"until":"2026-12-31","gap_policy":"Skip","fold_policy":"Earlier"})).unwrap()
}
#[test]
fn identity_relations_create_closed_reference_only_vertices_and_keep_declared_entities() {
    use GraphVertexTypeToken as V;
    let relations = vec![
        GraphSourceRelationV1 {
            src: identity(V::Object, "a"),
            dst: identity(V::Resource, "missing-room"),
            edge_type: GraphEdgeTypeToken::ObjectResource,
            payload: vec![],
            multiplicity: 1,
        },
        GraphSourceRelationV1 {
            src: identity(V::Object, "missing-object"),
            dst: identity(V::Fact, "missing-fact"),
            edge_type: GraphEdgeTypeToken::EvidenceFactReference,
            payload: edge(0, 0, GraphEdgeTypeToken::EvidenceFactReference).payload,
            multiplicity: 1,
        },
    ];
    let references = vec![
        identity(V::Calendar, "missing-calendar"),
        identity(V::Integration, "missing-integration"),
        identity(V::Series, "missing-series"),
    ];
    let result = CanonicalGraphSourceV1::from_relations(
        vec![object("a", "EVENT")],
        relations.clone(),
        references.clone(),
    )
    .unwrap()
    .canonicalize()
    .unwrap();
    assert_eq!(result.graph.vertices().len(), 7);
    for v in result.graph.vertices() {
        let presence = serde_json::to_value(&v.attributes[0].value).unwrap();
        if v.alias
            == result
                .identity_bindings
                .iter()
                .find(|b| b.source_id.as_str() == "a")
                .unwrap()
                .alias
        {
            assert_eq!(presence["v"], "DECLARED");
        } else {
            assert_eq!(presence["v"], "REFERENCE_ONLY");
            assert!(
                v.attributes[1..]
                    .iter()
                    .all(|a| matches!(a.value.kind(), CanonicalValueKindV1::None {}))
            );
        }
    }
    let mut permuted = relations;
    permuted.reverse();
    let mut refs = references;
    refs.reverse();
    refs.push(identity(V::Resource, "missing-room"));
    let other = CanonicalGraphSourceV1::from_relations(vec![object("a", "EVENT")], permuted, refs)
        .unwrap()
        .canonicalize()
        .unwrap();
    assert_eq!(result.payload_bytes, other.payload_bytes);
    assert_eq!(result.identity_bindings, other.identity_bindings);
    assert!(
        !String::from_utf8(result.payload_bytes)
            .unwrap()
            .contains("missing-")
    );
}
#[test]
fn series_recurrence_uses_graph_object_and_weekday_sets_not_the_outer_grammar() {
    for weekly in [false, true] {
        let rule = recurrence(weekly);
        let graph = canonical_graph_recurrence_v1(&rule).unwrap();
        let value = serde_json::to_value(&graph).unwrap();
        assert_eq!(value["t"], "OBJECT");
        let attrs = value["v"].as_array().unwrap();
        let keys: Vec<_> = attrs.iter().map(|a| a["key"].as_str().unwrap()).collect();
        assert_eq!(
            keys,
            [
                "count",
                "duration_seconds",
                "fold_policy",
                "frequency",
                "gap_policy",
                "interval",
                "local_time",
                "start_date",
                "timezone",
                "until",
                "weekdays"
            ]
        );
        assert_eq!(
            attrs[10]["value"],
            if weekly {
                json!({"t":"SET","v":[{"t":"TOKEN","v":"MONDAY"},{"t":"TOKEN","v":"WEDNESDAY"}]})
            } else {
                json!({"t":"SET","v":[]})
            }
        );
        let outer: CanonicalRecurrenceRuleV1 = (&rule).try_into().unwrap();
        assert_ne!(
            canonical_json_bytes(&graph).unwrap(),
            canonical_json_bytes(&outer).unwrap()
        );
        CanonicalGraphSourceV1::new(vec![series(&rule)], vec![])
            .unwrap()
            .canonicalize()
            .unwrap();
        let mut bad = serde_json::to_value(series(&rule)).unwrap();
        let fields = bad["attributes"][2]["value"]["v"].as_array_mut().unwrap();
        fields[10]["value"] = if weekly {
            json!({"t":"SET","v":[]})
        } else {
            json!({"t":"SET","v":[{"t":"TOKEN","v":"MONDAY"}]})
        };
        let bad: GraphSourceVertexV1 = serde_json::from_value(bad).unwrap();
        assert!(CanonicalGraphSourceV1::new(vec![bad], vec![]).is_err());
    }
}
#[test]
fn external_ref_payload_breaks_graph_symmetry_before_opaque_binding_ties() {
    let (vertices, edges, _, _) = fixtures().remove(0);
    let source = CanonicalGraphSourceV1::new(vertices.clone(), edges.clone()).unwrap();
    let graph_only = source.canonicalize().unwrap();
    assert_eq!(graph_only.identity_bindings[0].source_id.as_str(), "one");
    let seen = std::cell::RefCell::new(std::collections::BTreeSet::new());
    let actual = source
        .canonicalize_with_payload(|graph, bindings| {
            let target = &bindings
                .iter()
                .find(|b| b.source_id.as_str() == "two")
                .unwrap()
                .alias;
            seen.borrow_mut().insert(target.clone());
            canonical_json_bytes(
                &json!({"graph":graph,"targets":[{"object":target,"role":"TARGET"}]}),
            )
        })
        .unwrap();
    assert_eq!(seen.borrow().len(), 2);
    assert_eq!(actual.identity_bindings[0].source_id.as_str(), "two");
    assert_eq!(
        actual.graph.canonical_bytes().unwrap(),
        graph_only.graph.canonical_bytes().unwrap()
    );
    let payload: Value = serde_json::from_slice(&actual.payload_bytes).unwrap();
    assert_eq!(payload["targets"][0]["object"], "v000000");
    // Consistent rename deliberately reverses opaque spelling, while preserving the target relation.
    let mut renamed = vertices;
    for (i, v) in renamed.iter_mut().enumerate() {
        v.source_id = IdentifierString::new(format!("renamed-{}", 9 - i)).unwrap();
    }
    let renamed = CanonicalGraphSourceV1::new(renamed, edges)
        .unwrap()
        .canonicalize_with_payload(|graph, bindings| {
            let target = &bindings
                .iter()
                .find(|b| b.source_id.as_str() == "renamed-8")
                .unwrap()
                .alias;
            canonical_json_bytes(
                &json!({"graph":graph,"targets":[{"object":target,"role":"TARGET"}]}),
            )
        })
        .unwrap();
    assert_eq!(actual.payload_bytes, renamed.payload_bytes);
}
#[test]
fn fact_subject_and_directed_evidence_roles_preserve_semantics() {
    use GraphEdgeTypeToken as E;
    let vs = vec![
        object("a", "EVENT"),
        object("b", "AVAILABILITY"),
        fact("f", None),
        series(&recurrence(false)),
    ];
    let subject = edge(2, 0, E::FactSubject);
    let object_evidence = edge(0, 2, E::EvidenceFactReference);
    let mut series_evidence = edge(3, 2, E::EvidenceFactReference);
    series_evidence.payload[0].value = val(json!({"t":"TOKEN","v":"SERIES_EVIDENCE"}));
    let es = vec![
        subject.clone(),
        object_evidence.clone(),
        series_evidence.clone(),
    ];
    let base = CanonicalGraphSourceV1::new(vs.clone(), es.clone())
        .unwrap()
        .canonicalize()
        .unwrap();
    let mut changed = es.clone();
    changed[0].dst = 1;
    assert_ne!(
        base.payload_bytes,
        CanonicalGraphSourceV1::new(vs.clone(), changed)
            .unwrap()
            .canonicalize()
            .unwrap()
            .payload_bytes
    );
    let bytes = String::from_utf8(base.payload_bytes).unwrap();
    assert!(bytes.contains("OBJECT_TIME_EVIDENCE"));
    assert!(bytes.contains("SERIES_EVIDENCE"));
    for (index, role) in [(1, "SERIES_EVIDENCE"), (2, "OBJECT_TIME_EVIDENCE")] {
        let mut invalid = es.clone();
        invalid[index].payload[0].value = val(json!({"t":"TOKEN","v":role}));
        assert!(CanonicalGraphSourceV1::new(vs.clone(), invalid).is_err());
    }
    let mut invalid = es;
    invalid[1].src = 2;
    invalid[1].dst = 0;
    assert!(CanonicalGraphSourceV1::new(vs, invalid).is_err());
}
#[test]
fn edge_order_is_explicit_rank_then_destination_not_enum_or_token_order() {
    use GraphEdgeTypeToken as E;
    let tokens = [
        ("DEPENDS_ON_OBJECT", 0),
        ("FACT_SUBJECT", 1),
        ("OBJECT_CALENDAR", 2),
        ("OBJECT_INTEGRATION", 3),
        ("OBJECT_RESOURCE", 4),
        ("EVIDENCE_FACT_REFERENCE", 5),
    ];
    for (token, rank) in tokens {
        let t: GraphEdgeTypeToken = serde_json::from_value(json!(token)).unwrap();
        assert_eq!(edge_type_rank(t), rank);
    }
    let vs = vec![
        object("a", "EVENT"),
        object("b", "AVAILABILITY"),
        fact("f", None),
        GraphSourceVertexV1::reference_only(
            GraphVertexTypeToken::Calendar,
            IdentifierString::new("cal").unwrap(),
        ),
        GraphSourceVertexV1::reference_only(
            GraphVertexTypeToken::Integration,
            IdentifierString::new("int").unwrap(),
        ),
        resource("room"),
    ];
    let es = vec![
        edge(0, 2, E::EvidenceFactReference),
        edge(0, 5, E::ObjectResource),
        edge(0, 4, E::ObjectIntegration),
        edge(0, 3, E::ObjectCalendar),
        edge(0, 1, E::DependsOnObject),
        edge(2, 0, E::FactSubject),
    ];
    let result = CanonicalGraphSourceV1::new(vs, es)
        .unwrap()
        .canonicalize()
        .unwrap();
    let a = &result
        .identity_bindings
        .iter()
        .find(|b| b.source_id.as_str() == "a")
        .unwrap()
        .alias;
    let types: Vec<_> = result
        .graph
        .edges()
        .iter()
        .filter(|e| &e.src == a)
        .map(|e| e.edge_type.as_str())
        .collect();
    assert_eq!(
        types,
        [
            "DEPENDS_ON_OBJECT",
            "OBJECT_CALENDAR",
            "OBJECT_INTEGRATION",
            "OBJECT_RESOURCE",
            "EVIDENCE_FACT_REFERENCE"
        ]
    );
}

#[test]
fn closed_graph_rejects_payload_direction_presence_and_multiplicity_corruption() {
    use GraphEdgeTypeToken as E;
    let vs = vec![object("o", "EVENT"), resource("r"), fact("f", None)];
    let subject = edge(2, 0, E::FactSubject);
    let mut invalid = edge(0, 1, E::ObjectResource);
    invalid.payload = vec![attr(
        "evidence_role",
        json!({"t":"TOKEN","v":"OBJECT_TIME_EVIDENCE"}),
    )];
    assert!(CanonicalGraphSourceV1::new(vs.clone(), vec![subject.clone(), invalid]).is_err());
    let mut missing_payload = edge(0, 2, E::EvidenceFactReference);
    missing_payload.payload.clear();
    assert!(
        CanonicalGraphSourceV1::new(vs.clone(), vec![subject.clone(), missing_payload]).is_err()
    );
    assert!(
        CanonicalGraphSourceV1::new(vs.clone(), vec![subject.clone(), subject.clone()]).is_err()
    );
    let mut huge = edge(0, 1, E::ObjectResource);
    huge.multiplicity = u64::MAX;
    assert!(
        CanonicalGraphSourceV1::new(
            vs.clone(),
            vec![subject, huge, edge(0, 1, E::ObjectResource)]
        )
        .is_err()
    );
    let mut reference = GraphSourceVertexV1::reference_only(
        GraphVertexTypeToken::Object,
        IdentifierString::new("ref").unwrap(),
    );
    reference.attributes[1].value = val(json!({"t":"TOKEN","v":"EVENT"}));
    assert!(CanonicalGraphSourceV1::new(vec![reference], vec![]).is_err());
    let mut duplicate = object("o", "EVENT");
    duplicate.attributes[1] = duplicate.attributes[0].clone();
    assert!(CanonicalGraphSourceV1::new(vec![duplicate], vec![]).is_err());
    let mut serialized = json!({"direction":"UNDIRECTED","src":"v000000","dst":"v000001","type":"OBJECT_RESOURCE","payload":[],"multiplicity":"1"});
    assert!(serde_json::from_value::<CanonicalEdgeV1>(serialized.clone()).is_err());
    serialized["direction"] = json!("DIRECTED");
    serialized["multiplicity"] = json!(1);
    assert!(serde_json::from_value::<CanonicalEdgeV1>(serialized).is_err());
}
