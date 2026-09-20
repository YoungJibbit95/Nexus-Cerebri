use cerebri_temporal::*;
fn at(n: i64) -> Instant {
    Instant::from_timestamp(n, 0).unwrap()
}
fn r(a: i64, b: i64) -> TimeRange {
    TimeRange::new(at(a), at(b)).unwrap()
}
fn busy(range: TimeRange) -> BusyInterval {
    BusyInterval {
        range,
        buffers: Buffers::default(),
    }
}
#[test]
fn union_clips_sorts_deduplicates_and_joins_touching_ranges() {
    let h = PlanningHorizon(r(0, 10));
    assert_eq!(
        normalize_busy(h, &[r(7, 12), r(2, 4), r(-2, 0), r(4, 8), r(2, 4)]),
        vec![r(2, 10)]
    );
    assert!(!r(2, 4).overlaps(r(4, 8)));
    let report = free_busy(h, &[busy(r(2, 4)), busy(r(4, 8))], Coverage::Complete).unwrap();
    assert_eq!(report.free, vec![r(0, 2), r(8, 10)]);
    assert_eq!(report.busy, vec![r(2, 8)]);
}
#[test]
fn empty_and_full_coverage_do_not_invent_free_time() {
    let h = PlanningHorizon(r(0, 10));
    assert_eq!(
        free_busy(h, &[], Coverage::Complete).unwrap().free,
        vec![h.0]
    );
    let unknown = free_busy(h, &[], Coverage::Incomplete).unwrap();
    assert!(unknown.free.is_empty());
    assert_eq!(unknown.unknown, vec![h.0]);
    let full = free_busy(h, &[busy(r(-1, 11))], Coverage::Incomplete).unwrap();
    assert!(full.free.is_empty() && full.unknown.is_empty());
    assert_eq!(full.busy, vec![h.0]);
}
#[test]
fn buffers_and_travel_are_applied_before_horizon_clipping() {
    let item = BusyInterval {
        range: r(10, 20),
        buffers: Buffers {
            before_seconds: 2,
            after_seconds: 3,
            travel_seconds: 4,
        },
    };
    let report = free_busy(PlanningHorizon(r(5, 30)), &[item], Coverage::Complete).unwrap();
    assert_eq!(report.trace[0].buffered, r(4, 23));
    assert_eq!(report.busy, vec![r(5, 23)]);
    assert_eq!(report.free, vec![r(23, 30)]);
}
#[test]
fn overflow_is_a_typed_failure() {
    let range =
        TimeRange::new(Instant::MIN_UTC, Instant::MIN_UTC + TimeDelta::seconds(60)).unwrap();
    assert_eq!(
        Buffers {
            before_seconds: 1,
            ..Default::default()
        }
        .apply(range),
        Err(TemporalError::Overflow)
    );
    let range =
        TimeRange::new(Instant::MAX_UTC - TimeDelta::seconds(60), Instant::MAX_UTC).unwrap();
    assert_eq!(
        Buffers {
            after_seconds: 1,
            ..Default::default()
        }
        .apply(range),
        Err(TemporalError::Overflow)
    );
}
#[test]
fn exhaustive_partition_subtraction_and_permutation_properties() {
    let h = PlanningHorizon(r(0, 5));
    for a in -2..7 {
        for b in a + 1..8 {
            for c in -2..7 {
                for d in c + 1..8 {
                    let x = r(a, b);
                    let y = r(c, d);
                    let inputs = [busy(x), busy(y)];
                    let result = free_busy(h, &inputs, Coverage::Complete).unwrap();
                    assert_eq!(result.busy, normalize_busy(h, &[y, x]));
                    assert_eq!(result.busy, normalize_busy(h, &result.busy));
                    let total: i64 = result
                        .busy
                        .iter()
                        .chain(&result.free)
                        .map(|r| r.duration().num_seconds())
                        .sum();
                    assert_eq!(total, 5);
                    for p in 0..5 {
                        let in_busy = result.busy.iter().any(|r| r.contains(at(p)));
                        assert_eq!(in_busy, x.contains(at(p)) || y.contains(at(p)));
                        assert_eq!(result.free.iter().any(|r| r.contains(at(p))), !in_busy);
                    }
                    let difference = x.subtract(y);
                    assert!(difference.len() <= 2);
                    for p in -2..8 {
                        assert_eq!(
                            difference.iter().any(|r| r.contains(at(p))),
                            x.contains(at(p)) && !y.contains(at(p))
                        );
                    }
                }
            }
        }
    }
}
