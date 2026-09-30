#[path = "../../../tests/support/mod.rs"]
mod support;
use cerebri_planner::{evaluation::*, *};
use cerebri_types::{FieldState, Knowledge, RequiredFieldError};
use serde_json::{Value, json};

fn n(v: u64) -> CanonicalU64Decimal {
    CanonicalU64Decimal::new(v).unwrap()
}
fn id(v: &str) -> IdentifierString {
    IdentifierString::new(v).unwrap()
}
fn hash(v: &str) -> SHA256Hex {
    SHA256Hex::new(v).unwrap()
}
fn instant(s: i64) -> CanonicalInstantV1 {
    CanonicalInstantV1 {
        unix_seconds: CanonicalI64Decimal::new(s).unwrap(),
        nanoseconds: NineDigitNanoseconds::new(0).unwrap(),
    }
}
fn h(start: i64) -> CanonicalSearchHypothesisV0_1 {
    CanonicalSearchHypothesisV0_1 {
        schema_version: SearchHypothesisSchemaVersion::V0_1,
        placements: [DigestPlacementV1 {
            object_id: id("new-event"),
            start: instant(start),
            end: instant(start + 1800),
        }],
    }
}
fn admission(valid: bool) -> PlannerAdmissionObservationV0_1 {
    PlannerAdmissionObservationV0_1 {
        admission_class: if valid {
            PlannerAdmissionClassToken::Valid
        } else {
            PlannerAdmissionClassToken::InsufficientInformation
        },
        issue_codes: if valid {
            vec![]
        } else {
            vec![PlannerAdmissionIssueCodeToken::RequiredDurationMissing]
        },
    }
}
fn o1() -> DecisionObservationDigestWireV1 {
    // Independently constructed §56.9 semantic vector, not parsed from o1.json.
    // Its aa fingerprint is prevalidated serialization-fixture input, not a placement hash.
    let fp = hash(&"a".repeat(64));
    DecisionObservationDigestWireV1 {
        schema_version: DecisionObservationSchemaVersionV1::V1,
        decision_input_fingerprint: hash(
            "6b7e9ae1e69b83ac91462dcf74cec100d6087c9ee56daf3f1b226b7e9642674e",
        ),
        planner_admission: admission(true),
        generation: DigestGenerationV1::Searched {
            declared_hypotheses: vec![h(0)],
            visited_hypotheses: vec![h(0)],
            declared_hypothesis_count: n(1),
            evaluated_positions: n(1),
            discovered_candidate_count: n(1),
            declared_set_identity_digest: hash(
                "af7246d909b465f94bfa3df26a8013a12ecad3fd13c1dda854103db96af0ae53",
            ),
            visited_set_identity_digest: hash(
                "af7246d909b465f94bfa3df26a8013a12ecad3fd13c1dda854103db96af0ae53",
            ),
            visited_order_digest: hash(
                "45e2b66829944819284975e4db130570ac5e426ded8886688178dde82b58072f",
            ),
            outcome: PlanningOutcomeToken::Solution,
            search_assessment: SearchAssessmentToken::ProvenOptimal,
            search_exhausted: true,
            proof_claim: ProofClaimToken::ProvenOptimalDeterministicObjective,
        },
        candidates: vec![DigestCandidateV1 {
            candidate_fingerprint: fp.clone(),
            placements: h(0).placements,
            ranking_feature_snapshot: DigestRankingFeatureSetV0_1 {
                schema_version: DigestRankingFeatureSchemaV0_1 {
                    major: n(0),
                    minor: n(1),
                },
                preferred_start_distance_seconds: CanonicalOptionalV1::None {},
                preferred_start_source: CanonicalOptionalV1::None {},
                mutation_count: n(0),
                shift_seconds: n(0),
            },
            deterministic_rank_in_discovered: n(1),
            deterministic_ordering_key: DigestDeterministicOrderingKeyV1 {
                preference_distance_seconds: n(0),
                mutation_count: n(0),
                shift_seconds: n(0),
                start: instant(0),
                object_id: id("new-event"),
            },
        }],
        product_eligibility: DigestProductEligibilityV1::Applied {
            records: vec![DigestProductEligibilityRecordV1 {
                candidate_fingerprint: fp.clone(),
                eligible: true,
                exclusion_rule_id: CanonicalOptionalV1::None {},
            }],
        },
        ranking_observation: DigestRankingObservationV1::Applied {
            ranked_eligible_order: vec![fp.clone()],
        },
        display_decision: DigestDisplayDecisionV1::Applied {
            display_policy_output_order: vec![fp],
        },
    }
}
fn o2() -> DecisionObservationDigestWireV1 {
    DecisionObservationDigestWireV1 {
        schema_version: DecisionObservationSchemaVersionV1::V1,
        decision_input_fingerprint: hash(
            "28e024305db00a3e919f32074a39f155dc4e5e061c2c9b68d7caa14e6a4bf480",
        ),
        planner_admission: admission(false),
        generation: DigestGenerationV1::NotAdmitted {
            outcome: NotAdmittedOutcomeV0_1::InsufficientInformation,
            proof_claim: ProofClaimToken::None,
        },
        candidates: vec![],
        product_eligibility: DigestProductEligibilityV1::NotApplicable {
            reason: ProductDecisionReasonToken::PlanningNotAdmitted,
        },
        ranking_observation: DigestRankingObservationV1::NotApplicable {
            reason: ProductDecisionReasonToken::PlanningNotAdmitted,
        },
        display_decision: DigestDisplayDecisionV1::NotApplicable {
            reason: ProductDecisionReasonToken::PlanningNotAdmitted,
        },
    }
}
#[test]
fn external_o1_o2_typed_semantics_to_exact_bytes_and_digests() {
    for (source, literal, length, expected) in [
        (
            o1(),
            include_bytes!("fixtures/evaluation/o1.json").as_slice(),
            2295,
            "5fc321286e7e64467abb2692052f9d47f1d5a6cd66ba093d09ef07084b21076b",
        ),
        (
            o2(),
            include_bytes!("fixtures/evaluation/o2.json").as_slice(),
            584,
            "410cfb50881dd7bcabb341c44eec875bed1594917861c326447456903b0dd5bf",
        ),
    ] {
        let payload = DecisionObservationDigestPayloadV1::try_from(source).unwrap();
        let bytes = payload.canonical_bytes().unwrap();
        assert_eq!(bytes.len(), length);
        assert_eq!(bytes, literal);
        assert_eq!(payload.digest().unwrap().as_str(), expected);
        assert_eq!(
            serde_json::from_slice::<DecisionObservationDigestPayloadV1>(&bytes).unwrap(),
            payload
        );
    }
}

#[derive(Clone)]
struct Native {
    generation: CandidateGenerationObservationV0_1,
    candidates: Vec<CandidateObservationV0_1>,
    eligibility: ProductEligibilityObservationV0_1,
    ranking: RankingObservationV0_1,
    display: DisplayDecisionV0_1,
}
impl Native {
    fn project(&self) -> Result<DecisionObservationDigestPayloadV1, EvaluationContractError> {
        DecisionObservationProjectionV1::from_observations(
            hash(&"b".repeat(64)),
            &self.generation,
            &self.candidates,
            &self.eligibility,
            &self.ranking,
            &self.display,
        )
        .map(|p| p.payload().clone())
    }
    fn visit(&mut self) -> &mut SearchVisitObservationV0_1 {
        match &mut self.generation {
            CandidateGenerationObservationV0_1::Searched { search_visit, .. } => search_visit,
            _ => unreachable!(),
        }
    }
    fn eligible(&mut self) -> &mut Vec<ProductEligibilityRecordV0_1> {
        match &mut self.eligibility {
            ProductEligibilityObservationV0_1::Applied { records } => records,
            _ => unreachable!(),
        }
    }
    fn ranks(&mut self) -> &mut Vec<CandidateId> {
        match &mut self.ranking {
            RankingObservationV0_1::Applied {
                ranked_eligible_order,
            } => ranked_eligible_order,
            _ => unreachable!(),
        }
    }
    fn displays(&mut self) -> &mut Vec<CandidateId> {
        match &mut self.display {
            DisplayDecisionV0_1::Applied {
                display_policy_output_order,
            } => display_policy_output_order,
            _ => unreachable!(),
        }
    }
}
fn native(count: usize) -> Native {
    let mut hypotheses: Vec<_> = (0..count).map(|i| h(i as i64 * 60)).collect();
    let candidates: Vec<_> = hypotheses.iter().enumerate().map(|(i,h)| CandidateObservationV0_1 {
        candidate_id: CandidateId::new(format!("local-{i}")).unwrap(),
        candidate_fingerprint: CandidateFingerprintPayloadV1::new(h.placements[0].clone()).unwrap().fingerprint().unwrap(),
        placements: h.placements.clone(),
        ranking_feature_snapshot: serde_json::from_value(json!({"schema_version":{"major":0,"minor":1},
            "preferred_start_distance_seconds":null,"preferred_start_source":null,"mutation_count":0,"shift_seconds":0})).unwrap(),
        deterministic_rank_in_discovered: n(i as u64 + 1),
        deterministic_ordering_key: DeterministicOrderingKeyV0_1 {
            preference_distance_seconds: 0, mutation_count: 0, shift_seconds: 0,
            start: format!("1970-01-01T00:{i:02}:00Z").parse().unwrap(), object_id: id("new-event"),
        },
    }).collect();
    hypotheses.sort_by_cached_key(|h| h.canonical_bytes().unwrap());
    let ids: Vec<_> = candidates.iter().map(|c| c.candidate_id.clone()).collect();
    Native {
        generation: CandidateGenerationObservationV0_1::Searched {
            planner_admission: admission(true),
            search_visit: SearchVisitObservationV0_1 {
                schema_version: SearchVisitSchemaVersion::V0_1,
                hypothesis_schema_version: SearchHypothesisSchemaVersion::V0_1,
                declared_hypothesis_count: n(count as u64),
                evaluated_positions: n(count as u64),
                declared_set_identity_digest: hypothesis_set_identity(&hypotheses).unwrap(),
                visited_set_identity_digest: hypothesis_set_identity(&hypotheses).unwrap(),
                visited_order_digest: visit_order_identity(&hypotheses).unwrap(),
                declared_hypotheses: hypotheses.clone(),
                visited_hypotheses: hypotheses,
            },
            outcome: if count == 0 {
                PlanningOutcomeToken::NoSolution
            } else {
                PlanningOutcomeToken::Solution
            },
            search_assessment: if count == 0 {
                SearchAssessmentToken::Complete
            } else {
                SearchAssessmentToken::ProvenOptimal
            },
            search_exhausted: true,
            proof_claim: if count == 0 {
                ProofClaimToken::CompleteTraversal
            } else {
                ProofClaimToken::ProvenOptimalDeterministicObjective
            },
        },
        eligibility: ProductEligibilityObservationV0_1::Applied {
            records: ids
                .iter()
                .map(|candidate_id| ProductEligibilityRecordV0_1 {
                    candidate_id: candidate_id.clone(),
                    eligible: true,
                    exclusion_rule_id: None,
                })
                .collect(),
        },
        ranking: RankingObservationV0_1::Applied {
            ranked_eligible_order: ids.clone(),
        },
        display: DisplayDecisionV0_1::Applied {
            display_policy_output_order: ids,
        },
        candidates,
    }
}
#[test]
fn native_ids_and_source_collection_order_do_not_enter_digest() {
    let original = native(3);
    let expected = original.project().unwrap();
    let mut renamed = original.clone();
    for c in &mut renamed.candidates {
        c.candidate_id = CandidateId::new(format!("renamed-{}", c.candidate_id.as_str())).unwrap();
    }
    for r in renamed.eligible() {
        r.candidate_id = CandidateId::new(format!("renamed-{}", r.candidate_id.as_str())).unwrap();
    }
    for r in renamed.ranks() {
        *r = CandidateId::new(format!("renamed-{}", r.as_str())).unwrap();
    }
    for r in renamed.displays() {
        *r = CandidateId::new(format!("renamed-{}", r.as_str())).unwrap();
    }
    renamed.candidates.reverse();
    renamed.eligible().reverse();
    let actual = renamed.project().unwrap();
    assert_eq!(
        actual.canonical_bytes().unwrap(),
        expected.canonical_bytes().unwrap()
    );
    assert_eq!(actual.digest().unwrap(), expected.digest().unwrap());
    assert!(
        !String::from_utf8(actual.canonical_bytes().unwrap())
            .unwrap()
            .contains("candidate_id")
    );
}
#[test]
fn actual_exposure_is_excluded_but_display_policy_l_is_bound() {
    // Native capture-shaped test envelope only; no lifecycle validation/runtime capture.
    struct Capture {
        output: Native,
        exposure: ExposureObservationV0_1,
    }
    let mut capture = Capture {
        output: native(2),
        exposure: ExposureObservationV0_1::Exposed {
            surface_id: id("surface-a"),
            surface_version: id("1"),
            displayed_order: native(2).displays().clone(),
            viewport_visible_candidate_ids: None,
            exposed_at: "1970-01-01T01:00:00Z".parse().unwrap(),
        },
    };
    let expected = capture.output.project().unwrap();
    capture.exposure = ExposureObservationV0_1::Exposed {
        surface_id: id("surface-b"),
        surface_version: id("2"),
        displayed_order: vec![CandidateId::new("local-1").unwrap()],
        viewport_visible_candidate_ids: Some(vec![]),
        exposed_at: "1970-01-01T02:00:00Z".parse().unwrap(),
    };
    assert_eq!(expected, capture.output.project().unwrap());
    capture.output.displays().reverse();
    assert_ne!(
        expected.digest().unwrap(),
        capture.output.project().unwrap().digest().unwrap()
    );
    capture.output.displays().pop();
    assert_ne!(
        expected.digest().unwrap(),
        capture.output.project().unwrap().digest().unwrap()
    );
}
#[test]
fn native_fingerprints_ids_references_and_eligibility_coverage_reject() {
    let good = native(2);
    let mut v = good.clone();
    v.candidates[0].candidate_fingerprint = hash(&"a".repeat(64));
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.candidates[1].candidate_id = v.candidates[0].candidate_id.clone();
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.candidates[1] = v.candidates[0].clone();
    v.candidates[1].candidate_id = CandidateId::new("different").unwrap();
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.eligible().pop();
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.eligible()[1] = v.eligible()[0].clone();
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.eligible()[0].candidate_id = CandidateId::new("unknown").unwrap();
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.ranks()[0] = CandidateId::new("unknown").unwrap();
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.displays()[0] = CandidateId::new("unknown").unwrap();
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.eligible()[0].eligible = false;
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.eligible()[0].exclusion_rule_id = Some(RuleIdV1::new("rule").unwrap());
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.ranks().reverse();
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.ranks().pop();
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.displays()[1] = v.displays()[0].clone();
    assert!(v.project().is_err());
}
#[test]
fn search_count_set_order_subset_exhaustion_and_proof_consistency() {
    let good = native(2);
    let mut v = good.clone();
    v.visit().declared_hypothesis_count = n(1);
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.visit().evaluated_positions = n(1);
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.visit().declared_hypotheses.reverse();
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.visit().visited_hypotheses[1] = h(0);
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.visit().declared_hypotheses[1] = h(0);
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.visit().visited_hypotheses[1] = h(120);
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.visit().declared_set_identity_digest = hash(&"a".repeat(64));
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.visit().visited_set_identity_digest = hash(&"a".repeat(64));
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.visit().visited_order_digest = hash(&"a".repeat(64));
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.visit().visited_hypotheses.reverse();
    assert!(v.project().is_err());
    v.visit().visited_order_digest = visit_order_identity(&v.visit().visited_hypotheses).unwrap();
    assert_ne!(
        good.project().unwrap().digest().unwrap(),
        v.project().unwrap().digest().unwrap()
    );
    for (assessment, proof, exhausted) in [
        (
            SearchAssessmentToken::BestFound,
            ProofClaimToken::None,
            false,
        ),
        (
            SearchAssessmentToken::Complete,
            ProofClaimToken::CompleteTraversal,
            true,
        ),
        (
            SearchAssessmentToken::ProvenOptimal,
            ProofClaimToken::None,
            true,
        ),
    ] {
        let mut v = good.clone();
        if let CandidateGenerationObservationV0_1::Searched {
            search_assessment,
            proof_claim,
            search_exhausted,
            ..
        } = &mut v.generation
        {
            *search_assessment = assessment;
            *proof_claim = proof;
            *search_exhausted = exhausted;
        }
        assert!(v.project().is_err());
    }
    // A genuinely partial traversal is BEST_FOUND/NONE, even with a discovered solution.
    let mut partial = native(1);
    partial.visit().declared_hypotheses.push(h(60));
    partial.visit().declared_hypothesis_count = n(2);
    partial.visit().declared_set_identity_digest =
        hypothesis_set_identity(&partial.visit().declared_hypotheses).unwrap();
    if let CandidateGenerationObservationV0_1::Searched {
        search_assessment,
        proof_claim,
        search_exhausted,
        ..
    } = &mut partial.generation
    {
        *search_assessment = SearchAssessmentToken::BestFound;
        *proof_claim = ProofClaimToken::None;
        *search_exhausted = false;
    }
    assert_ne!(
        native(1).project().unwrap().digest().unwrap(),
        partial.project().unwrap().digest().unwrap()
    );
    let mut unvisited = native(2);
    unvisited.visit().visited_hypotheses.pop();
    unvisited.visit().evaluated_positions = n(1);
    unvisited.visit().visited_set_identity_digest = hypothesis_set_identity(&[h(0)]).unwrap();
    unvisited.visit().visited_order_digest = visit_order_identity(&[h(0)]).unwrap();
    if let CandidateGenerationObservationV0_1::Searched {
        search_assessment,
        proof_claim,
        search_exhausted,
        ..
    } = &mut unvisited.generation
    {
        *search_assessment = SearchAssessmentToken::BestFound;
        *proof_claim = ProofClaimToken::None;
        *search_exhausted = false;
    }
    assert!(unvisited.project().is_err()); // candidate outside S
    assert!(native(0).project().is_ok());
}
#[test]
fn candidate_rank_key_feature_parity_and_complete_tuple() {
    let good = native(2);
    let mut v = good.clone();
    v.candidates[0].deterministic_rank_in_discovered = n(0);
    assert!(v.project().is_err());
    let mut v = good.clone();
    v.candidates[1].deterministic_rank_in_discovered = n(1);
    assert!(v.project().is_err());
    for field in [
        "preference_distance_seconds",
        "mutation_count",
        "shift_seconds",
        "object_id",
        "start",
    ] {
        let mut c = serde_json::to_value(&good.candidates[0]).unwrap();
        c["deterministic_ordering_key"][field] = match field {
            "object_id" => json!("wrong"),
            "start" => json!("1970-01-01T00:01:00Z"),
            _ => json!(1),
        };
        let mut v = good.clone();
        v.candidates[0] = serde_json::from_value(c).unwrap();
        assert!(v.project().is_err());
    }
    // Same numeric costs/start, distinct object IDs: ID is the final qualified tie-break.
    let mut v = good.clone();
    v.candidates[1].placements = h(0).placements;
    v.candidates[1].placements[0].object_id = id("z-event");
    v.candidates[1].candidate_fingerprint =
        CandidateFingerprintPayloadV1::new(v.candidates[1].placements[0].clone())
            .unwrap()
            .fingerprint()
            .unwrap();
    v.candidates[1].deterministic_ordering_key.start =
        v.candidates[0].deterministic_ordering_key.start;
    v.candidates[1].deterministic_ordering_key.object_id = id("z-event");
    let mut hs = vec![
        h(0),
        CanonicalSearchHypothesisV0_1 {
            schema_version: SearchHypothesisSchemaVersion::V0_1,
            placements: v.candidates[1].placements.clone(),
        },
    ];
    hs.sort_by_cached_key(|h| h.canonical_bytes().unwrap());
    v.visit().declared_hypotheses = hs.clone();
    v.visit().visited_hypotheses = hs.clone();
    v.visit().declared_set_identity_digest = hypothesis_set_identity(&hs).unwrap();
    v.visit().visited_set_identity_digest = hypothesis_set_identity(&hs).unwrap();
    v.visit().visited_order_digest = visit_order_identity(&hs).unwrap();
    assert!(v.project().is_ok());
    v.candidates[0].deterministic_rank_in_discovered = n(2);
    v.candidates[1].deterministic_rank_in_discovered = n(1);
    v.ranks().reverse();
    assert!(v.project().is_err());
}
#[test]
fn valid_product_rule_ranking_feature_placement_and_outcome_changes_affect_digest() {
    let good = native(2);
    let original = good.project().unwrap().digest().unwrap();
    let mut excluded = good.clone();
    excluded.eligible()[0].eligible = false;
    excluded.eligible()[0].exclusion_rule_id = Some(RuleIdV1::new("rule-a").unwrap());
    excluded.ranks().remove(0);
    excluded.displays().remove(0);
    let changed = excluded.project().unwrap().digest().unwrap();
    assert_ne!(original, changed);
    excluded.eligible()[0].exclusion_rule_id = Some(RuleIdV1::new("rule-b").unwrap());
    assert_ne!(changed, excluded.project().unwrap().digest().unwrap());
    let mut v = good.clone();
    let mut f = serde_json::to_value(&v.candidates[0].ranking_feature_snapshot).unwrap();
    f["preferred_start_distance_seconds"] = json!(2);
    f["preferred_start_source"] = json!("ExplicitCurrentRequest");
    v.candidates[0].ranking_feature_snapshot = serde_json::from_value(f.clone()).unwrap();
    v.candidates[0]
        .deterministic_ordering_key
        .preference_distance_seconds = 2;
    v.candidates[0].deterministic_rank_in_discovered = n(2);
    v.candidates[1].deterministic_rank_in_discovered = n(1);
    v.ranks().reverse();
    let changed = v.project().unwrap().digest().unwrap();
    assert_ne!(original, changed);
    f["preferred_start_source"] = json!("Default");
    v.candidates[0].ranking_feature_snapshot = serde_json::from_value(f).unwrap();
    assert_ne!(changed, v.project().unwrap().digest().unwrap()); // provenance binds but never ranks
    let mut one = native(1);
    let a = one.project().unwrap().digest().unwrap();
    one.candidates[0].placements[0].end = instant(1900);
    one.candidates[0].candidate_fingerprint =
        CandidateFingerprintPayloadV1::new(one.candidates[0].placements[0].clone())
            .unwrap()
            .fingerprint()
            .unwrap();
    let hs = vec![CanonicalSearchHypothesisV0_1 {
        schema_version: SearchHypothesisSchemaVersion::V0_1,
        placements: one.candidates[0].placements.clone(),
    }];
    one.visit().declared_hypotheses = hs.clone();
    one.visit().visited_hypotheses = hs.clone();
    one.visit().declared_set_identity_digest = hypothesis_set_identity(&hs).unwrap();
    one.visit().visited_set_identity_digest = hypothesis_set_identity(&hs).unwrap();
    one.visit().visited_order_digest = visit_order_identity(&hs).unwrap();
    assert_ne!(a, one.project().unwrap().digest().unwrap());
    let mut empty = native(0);
    let a = empty.project().unwrap().digest().unwrap();
    if let CandidateGenerationObservationV0_1::Searched { outcome, .. } = &mut empty.generation {
        *outcome = PlanningOutcomeToken::NeedsRelaxation;
    }
    assert_ne!(a, empty.project().unwrap().digest().unwrap());
}

fn artifact(class: ArtifactClassToken) -> ArtifactSemanticIdentityV0_1 {
    ArtifactSemanticIdentityV0_1 {
        artifact_class: class,
        artifact_id: id("fixture"),
        artifact_version: id("0.1"),
        manifest_schema_version: id("0.1"),
        manifest_sha256: hash(&"a".repeat(64)),
    }
}
#[test]
fn missing_duration_is_fingerprintable_and_not_admitted_without_search() {
    let mut r = support::request();
    r.duration.value = FieldState::Resolved(Knowledge::Missing);
    assert!(
        validate_request(&r)
            .issues
            .contains(&ValidationIssue::RequiredDuration(
                RequiredFieldError::Missing
            ))
    );
    let base = BaseScenarioProjectionV1::from_request(&r).unwrap();
    let di = DecisionInputProjectionV1::from_request(
        &r,
        DecisionInputArtifactsV1 {
            planner_artifact: artifact(ArtifactClassToken::PlannerAlgorithm),
            generator_artifact: artifact(ArtifactClassToken::GeneratorPolicy),
            generator_configuration_artifact: CanonicalOptionalV1::None {},
            product_eligibility_policy: artifact(ArtifactClassToken::ProductEligibilityPolicy),
            ranking_policy: artifact(ArtifactClassToken::RankingPolicy),
            display_policy: artifact(ArtifactClassToken::DisplayPolicy),
        },
    )
    .unwrap();
    assert_eq!(
        di.payload().wire().base_scenario_fingerprint,
        base.payload().fingerprint().unwrap()
    );
    let p = DecisionObservationProjectionV1::from_observations(
        di.payload().fingerprint().unwrap(),
        &CandidateGenerationObservationV0_1::NotAdmitted {
            planner_admission: admission(false),
            outcome: NotAdmittedOutcomeV0_1::InsufficientInformation,
        },
        &[],
        &ProductEligibilityObservationV0_1::NotApplicable {
            reason: ProductDecisionReasonToken::PlanningNotAdmitted,
        },
        &RankingObservationV0_1::NotApplicable {
            reason: ProductDecisionReasonToken::PlanningNotAdmitted,
        },
        &DisplayDecisionV0_1::NotApplicable {
            reason: ProductDecisionReasonToken::PlanningNotAdmitted,
        },
    )
    .unwrap();
    let mut oracle = o2();
    oracle.decision_input_fingerprint = di.payload().fingerprint().unwrap();
    assert_eq!(
        p.payload(),
        &DecisionObservationDigestPayloadV1::try_from(oracle).unwrap()
    );
}
fn rejects(v: Value) {
    assert!(
        serde_json::from_value::<DecisionObservationDigestPayloadV1>(v.clone()).is_err(),
        "accepted {v}"
    );
}
#[test]
fn strict_digest_wire_and_cross_field_negatives() {
    for wire in [o1(), o2()] {
        let good = serde_json::to_value(wire).unwrap();
        for field in good.as_object().unwrap().keys() {
            let mut v = good.clone();
            v.as_object_mut().unwrap().remove(field);
            rejects(v);
            let mut v = good.clone();
            v[field] = Value::Null;
            rejects(v);
        }
        let mut v = good.clone();
        v["candidate_id"] = json!("forbidden");
        rejects(v);
        rejects(json!(
            good.as_object().unwrap().values().collect::<Vec<_>>()
        ));
        let text = serde_json::to_string(&good).unwrap();
        let duplicate = text.replacen(
            "\"schema_version\":\"1\"",
            "\"schema_version\":\"1\",\"schema_version\":\"1\"",
            1,
        );
        assert!(serde_json::from_str::<DecisionObservationDigestPayloadV1>(&duplicate).is_err());
    }
    let good = serde_json::to_value(o1()).unwrap();
    for (path, value) in [
        ("/schema_version", json!("2")),
        ("/schema_version", json!({"1":null})),
        ("/decision_input_fingerprint", json!("BAD")),
        (
            "/planner_admission/admission_class",
            json!("INSUFFICIENT_INFORMATION"),
        ),
        (
            "/planner_admission/issue_codes",
            json!(["REQUIRED_DURATION_UNKNOWN", "REQUIRED_DURATION_MISSING"]),
        ),
        (
            "/planner_admission/issue_codes",
            json!(["REQUIRED_DURATION_MISSING", "REQUIRED_DURATION_MISSING"]),
        ),
        (
            "/candidates/0/deterministic_rank_in_discovered",
            json!("01"),
        ),
        ("/candidates/0/deterministic_rank_in_discovered", json!("2")),
        ("/candidates/0/placements", json!([])),
        (
            "/candidates/0/placements/0/end",
            json!({"unix_seconds":"0","nanoseconds":"000000000"}),
        ),
        (
            "/candidates/0/ranking_feature_snapshot/schema_version/minor",
            json!("2"),
        ),
        (
            "/candidates/0/ranking_feature_snapshot/preferred_start_distance_seconds",
            json!({"state":"SOME","value":"0"}),
        ),
        (
            "/candidates/0/ranking_feature_snapshot/preferred_start_source",
            json!({"state":"SOME","value":"DEFAULT"}),
        ),
        (
            "/candidates/0/ranking_feature_snapshot/preferred_start_source",
            json!({"state":"NONE","value":"DEFAULT"}),
        ),
        (
            "/product_eligibility/records/0/exclusion_rule_id",
            json!({"state":"SOME","value":"bad rule"}),
        ),
        (
            "/product_eligibility/records/0/exclusion_rule_id",
            json!({"state":"SOME","value":"valid-rule"}),
        ),
        ("/generation/discovered_candidate_count", json!("0")),
        ("/generation/evaluated_positions", json!(1)),
        ("/generation/outcome", json!("NO_SOLUTION")),
        ("/generation/proof_claim", json!("NONE")),
        (
            "/display_decision",
            json!({"state":"NOT_APPLICABLE","reason":"PLANNING_NOT_ADMITTED"}),
        ),
    ] {
        let mut v = good.clone();
        *v.pointer_mut(path).unwrap() = value;
        rejects(v);
    }
    for field in good["generation"].as_object().unwrap().keys() {
        let mut v = good.clone();
        v["generation"].as_object_mut().unwrap().remove(field);
        rejects(v);
    }
    for path in [
        "/planner_admission",
        "/generation",
        "/candidates/0",
        "/candidates/0/placements/0",
        "/candidates/0/placements/0/start",
        "/candidates/0/ranking_feature_snapshot",
        "/candidates/0/ranking_feature_snapshot/schema_version",
        "/candidates/0/ranking_feature_snapshot/preferred_start_distance_seconds",
        "/candidates/0/ranking_feature_snapshot/preferred_start_source",
        "/candidates/0/deterministic_ordering_key",
        "/generation/declared_hypotheses/0",
        "/product_eligibility",
        "/product_eligibility/records/0",
        "/product_eligibility/records/0/exclusion_rule_id",
        "/ranking_observation",
        "/display_decision",
    ] {
        for field in good.pointer(path).unwrap().as_object().unwrap().keys() {
            let mut v = good.clone();
            v.pointer_mut(path)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove(field);
            rejects(v);
        }
        let mut v = good.clone();
        v.pointer_mut(path).unwrap()["extra"] = json!("unexpected");
        rejects(v);
        let mut v = good.clone();
        let fields = v
            .pointer(path)
            .unwrap()
            .as_object()
            .unwrap()
            .values()
            .cloned()
            .collect::<Vec<_>>();
        *v.pointer_mut(path).unwrap() = json!(fields);
        rejects(v);
    }
    let mut v = good.clone();
    v["candidates"]
        .as_array_mut()
        .unwrap()
        .push(good["candidates"][0].clone());
    rejects(v);
    let mut v = serde_json::to_value(native(2).project().unwrap()).unwrap();
    v["product_eligibility"]["records"]
        .as_array_mut()
        .unwrap()
        .reverse();
    rejects(v);
    let text = serde_json::to_string(&good).unwrap();
    let duplicate = text.replacen(
        "\"eligible\":true",
        "\"eligible\":true,\"eligible\":true",
        1,
    );
    assert!(serde_json::from_str::<DecisionObservationDigestPayloadV1>(&duplicate).is_err());
    let good = serde_json::to_value(o2()).unwrap();
    for (path, value) in [
        ("/generation/proof_claim", json!("COMPLETE_TRAVERSAL")),
        ("/planner_admission/admission_class", json!("VALID")),
        (
            "/planner_admission/issue_codes",
            json!(["DUPLICATE_OBJECT"]),
        ),
        (
            "/product_eligibility",
            json!({"state":"APPLIED","records":[]}),
        ),
        (
            "/ranking_observation",
            json!({"state":"APPLIED","ranked_eligible_order":[]}),
        ),
        (
            "/display_decision",
            json!({"state":"APPLIED","display_policy_output_order":[]}),
        ),
    ] {
        let mut v = good.clone();
        *v.pointer_mut(path).unwrap() = value;
        rejects(v);
    }
    for field in [
        "declared_hypotheses",
        "visited_hypotheses",
        "evaluated_positions",
        "search_exhausted",
        "search_visit",
    ] {
        let mut v = good.clone();
        v["generation"][field] = json!([]);
        rejects(v);
    }
}
