use cerebri_planner::evaluation::*;

// Architecture-authored H1, §56.6; no expected value is computed by production code.
const H1: &str = r#"{"placements":[{"end":{"nanoseconds":"000000000","unix_seconds":"1800"},"object_id":"new-event","start":{"nanoseconds":"000000000","unix_seconds":"0"}}],"schema_version":"0.1"}"#;
fn hypothesis(start: i64, end: i64) -> CanonicalSearchHypothesisV0_1 {
    CanonicalSearchHypothesisV0_1 {
        schema_version: SearchHypothesisSchemaVersion::V0_1,
        placements: [DigestPlacementV1 {
            object_id: IdentifierString::new("new-event").unwrap(),
            start: CanonicalInstantV1 {
                unix_seconds: CanonicalI64Decimal::new(start).unwrap(),
                nanoseconds: NineDigitNanoseconds::new(0).unwrap(),
            },
            end: CanonicalInstantV1 {
                unix_seconds: CanonicalI64Decimal::new(end).unwrap(),
                nanoseconds: NineDigitNanoseconds::new(0).unwrap(),
            },
        }],
    }
}
#[test]
fn h1_set_and_order_match_external_oracles() {
    let h = hypothesis(0, 1800);
    assert_eq!(h.canonical_bytes().unwrap(), H1.as_bytes());
    assert_eq!(
        h.fingerprint().unwrap().as_str(),
        "d5d275a1370b9d1ed11c3b3df64dceaef826c781e11a85e09a776b6251bcde00"
    );
    assert_eq!(
        canonical_hypothesis_set_bytes(std::slice::from_ref(&h)).unwrap(),
        format!("[{H1}]").as_bytes()
    );
    assert_eq!(
        hypothesis_set_identity(std::slice::from_ref(&h))
            .unwrap()
            .as_str(),
        "af7246d909b465f94bfa3df26a8013a12ecad3fd13c1dda854103db96af0ae53"
    );
    assert_eq!(
        visit_order_identity(&[h]).unwrap().as_str(),
        "45e2b66829944819284975e4db130570ac5e426ded8886688178dde82b58072f"
    );
}
#[test]
fn domain_separation_matches_external_sanity_vectors() {
    for (domain, expected) in [
        (
            FingerprintDomainV1::BaseScenario,
            "454a669210b5a199573a525982eb895859f686564dc3561236177ea3bcf90a3f",
        ),
        (
            FingerprintDomainV1::DecisionInput,
            "86d009e6d8210f32b313df1006846c27a00c13690e1b16fd89e59d930d038491",
        ),
        (
            FingerprintDomainV1::DecisionObservation,
            "3b228667b40b8ad49d39ec95284fe03b52ee7fb548dd6b9be97ac5e3fd17b908",
        ),
        (
            FingerprintDomainV1::Candidate,
            "7845581e314700dd407bf7e9d5fcd88c8920e4ec747c4b10ff1e8c8afbc03fa5",
        ),
    ] {
        assert_eq!(domain.digest_bytes(b"{}").as_str(), expected);
    }
}
#[test]
fn hypothesis_sets_ignore_permutation_while_visits_preserve_it() {
    let a = hypothesis(0, 1800);
    let b = hypothesis(60, 1860);
    assert_eq!(
        hypothesis_set_identity(&[a.clone(), b.clone()]).unwrap(),
        hypothesis_set_identity(&[b.clone(), a.clone()]).unwrap()
    );
    assert_ne!(
        visit_order_identity(&[a.clone(), b.clone()]).unwrap(),
        visit_order_identity(&[b, a.clone()]).unwrap()
    );
    assert!(hypothesis_set_identity(&[a.clone(), a.clone()]).is_err());
    assert!(visit_order_identity(&[a.clone(), a]).is_err());
    assert!(hypothesis(1, 1).canonical_bytes().is_err());
    assert!(hypothesis(2, 1).fingerprint().is_err());
}
#[test]
fn candidate_identity_contains_only_source_placement() {
    let h = hypothesis(0, 1800);
    let candidate = CandidateFingerprintPayloadV1::new(h.placements[0].clone()).unwrap();
    assert_eq!(candidate.canonical_bytes().unwrap(),br#"{"placements":[{"end":{"nanoseconds":"000000000","unix_seconds":"1800"},"object_id":"new-event","start":{"nanoseconds":"000000000","unix_seconds":"0"}}],"schema_version":"1"}"#);
    // CandidateId, rank, policies, display and lifecycle cannot be supplied to this projection.
    assert_ne!(
        candidate.fingerprint().unwrap(),
        CandidateFingerprintPayloadV1::new(hypothesis(60, 1860).placements[0].clone())
            .unwrap()
            .fingerprint()
            .unwrap()
    );
    assert!(CandidateFingerprintPayloadV1::new(hypothesis(0, 0).placements[0].clone()).is_err());
}
