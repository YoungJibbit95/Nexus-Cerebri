use cerebri_temporal::*;
use chrono::{Datelike, Days};
fn h(a: &str, b: &str) -> PlanningHorizon {
    PlanningHorizon(TimeRange::new(a.parse().unwrap(), b.parse().unwrap()).unwrap())
}
fn def(date: &str, time: &str, zone: &str) -> RecurrenceDefinition {
    RecurrenceDefinition {
        start_date: date.parse().unwrap(),
        local_time: time.parse().unwrap(),
        timezone: zone.parse().unwrap(),
        duration: Duration::seconds(1800).unwrap(),
        pattern: RecurrencePattern::Daily {
            every: 1.try_into().unwrap(),
        },
        until: None,
        count: None,
        gap_policy: GapPolicy::Reject,
        fold_policy: FoldPolicy::Reject,
    }
}
fn expand(d: RecurrenceDefinition, h: PlanningHorizon) -> Result<ExpansionReport, TemporalError> {
    RecurrenceRule::new(d)?.expand_report(h, ExpansionLimits::default())
}
#[test]
fn golden_berlin_gap_policies_and_nominal_count() {
    let horizon = h("2026-03-27T23:00:00Z", "2026-03-31T00:00:00Z");
    let mut d = def("2026-03-28", "02:30:00", "Europe/Berlin");
    d.count = Some(3.try_into().unwrap());
    assert_eq!(
        expand(d.clone(), horizon),
        Err(TemporalError::NonexistentLocalTime)
    );
    d.gap_policy = GapPolicy::Skip;
    let r = expand(d, horizon).unwrap();
    assert_eq!(
        r.occurrences
            .iter()
            .map(|o| (o.sequence, o.range.start().to_rfc3339()))
            .collect::<Vec<_>>(),
        vec![
            (1, "2026-03-28T01:30:00+00:00".into()),
            (3, "2026-03-30T00:30:00+00:00".into())
        ]
    );
    assert_eq!(r.skipped.len(), 1);
    assert_eq!(r.skipped[0].sequence, 2);
}
#[test]
fn golden_berlin_fold_policies_choose_distinct_instants() {
    let horizon = h("2026-10-25T00:00:00Z", "2026-10-25T04:00:00Z");
    let mut d = def("2026-10-25", "02:30:00", "Europe/Berlin");
    assert_eq!(
        expand(d.clone(), horizon),
        Err(TemporalError::AmbiguousLocalTime)
    );
    for (policy, instant, resolution) in [
        (
            FoldPolicy::Earlier,
            "2026-10-25T00:30:00Z",
            OccurrenceResolution::EarlierFold,
        ),
        (
            FoldPolicy::Later,
            "2026-10-25T01:30:00Z",
            OccurrenceResolution::LaterFold,
        ),
    ] {
        d.fold_policy = policy;
        let r = expand(d.clone(), horizon).unwrap();
        assert_eq!(
            r.occurrences[0].range.start(),
            instant.parse::<Instant>().unwrap()
        );
        assert_eq!(r.occurrences[0].resolution, resolution);
    }
}
#[test]
fn local_daily_time_survives_23_hour_day_and_half_hour_fold() {
    let r = expand(
        def("2026-03-28", "09:00:00", "Europe/Berlin"),
        h("2026-03-28T00:00:00Z", "2026-03-30T00:00:00Z"),
    )
    .unwrap();
    assert_eq!(
        (r.occurrences[1].range.start() - r.occurrences[0].range.start()).num_hours(),
        23
    );
    let mut d = def("2026-04-05", "01:45:00", "Australia/Lord_Howe");
    d.fold_policy = FoldPolicy::Earlier;
    let horizon = h("2026-04-04T13:00:00Z", "2026-04-04T17:00:00Z");
    let early = expand(d.clone(), horizon).unwrap();
    d.fold_policy = FoldPolicy::Later;
    let late = expand(d, horizon).unwrap();
    assert_eq!(
        (late.occurrences[0].range.start() - early.occurrences[0].range.start()).num_minutes(),
        30
    );
}
#[test]
fn date_line_gap_is_explicit() {
    let mut d = def("2011-12-29", "12:00:00", "Pacific/Apia");
    d.count = Some(3.try_into().unwrap());
    d.gap_policy = GapPolicy::Skip;
    let r = expand(d, h("2011-12-29T00:00:00Z", "2012-01-01T00:00:00Z")).unwrap();
    assert_eq!(
        r.skipped[0].date,
        "2011-12-30".parse::<LocalDate>().unwrap()
    );
    assert_eq!(r.occurrences.len(), 2);
}
#[test]
fn horizon_crossing_retains_identity_and_clips_visibility() {
    let d = def("2026-01-01", "23:45:00", "UTC");
    let horizon = h("2026-01-02T00:00:00Z", "2026-01-02T23:45:00Z");
    let rule = RecurrenceRule::new(d).unwrap();
    let report = rule
        .expand_report(horizon, ExpansionLimits::default())
        .unwrap();
    assert_eq!(report.occurrences.len(), 1);
    let o = &report.occurrences[0];
    assert_eq!(o.range.duration().num_minutes(), 30);
    assert_eq!(o.visible_range.duration().num_minutes(), 15);
    assert_eq!(rule.expand(horizon, 10).unwrap(), vec![o.visible_range]);
}
#[test]
fn distant_anchor_count_until_and_daily_interval() {
    let mut d = def("1900-01-01", "10:00:00", "UTC");
    d.pattern = RecurrencePattern::Daily {
        every: 2.try_into().unwrap(),
    };
    let horizon = h("2026-01-01T00:00:00Z", "2026-01-08T00:00:00Z");
    let r = expand(d.clone(), horizon).unwrap();
    assert!(r.examined_dates < 15);
    assert!(!r.occurrences.is_empty());
    d.count = Some(10.try_into().unwrap());
    assert!(expand(d, horizon).unwrap().occurrences.is_empty());
    let mut d = def("2026-01-01", "10:00:00", "UTC");
    d.until = Some("2026-01-02".parse().unwrap());
    assert_eq!(expand(d, horizon).unwrap().occurrences.len(), 2);
}
#[test]
fn weekly_closed_form_matches_day_by_day_oracle() {
    let origin: LocalDate = "2026-01-05".parse().unwrap();
    for anchor in 0..7 {
        for every in 1..4 {
            for mask in 1u8..128 {
                let start = origin.checked_add_days(Days::new(anchor)).unwrap();
                let weekdays = [
                    Weekday::Mon,
                    Weekday::Tue,
                    Weekday::Wed,
                    Weekday::Thu,
                    Weekday::Fri,
                    Weekday::Sat,
                    Weekday::Sun,
                ];
                let selected: Vec<_> = weekdays
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, d)| ((mask >> i) & 1 == 1).then_some(d))
                    .collect();
                let mut d = def(&start.to_string(), "10:00:00", "UTC");
                d.pattern = RecurrencePattern::Weekly {
                    every: (every as u16).try_into().unwrap(),
                    weekdays: selected.clone(),
                };
                d.count = Some(12.try_into().unwrap());
                let report = expand(d, h("2026-01-20T00:00:00Z", "2026-02-20T00:00:00Z")).unwrap();
                let mut ordinal = 0;
                let mut expected = vec![];
                for offset in 0..50 {
                    let date = start.checked_add_days(Days::new(offset)).unwrap();
                    let week = (offset + anchor) / 7;
                    if week % every == 0 && selected.contains(&date.weekday()) {
                        ordinal += 1;
                        if ordinal <= 12
                            && date >= "2026-01-20".parse::<LocalDate>().unwrap()
                            && date < "2026-02-20".parse::<LocalDate>().unwrap()
                        {
                            expected.push((ordinal, date));
                        }
                    }
                }
                assert_eq!(
                    report
                        .occurrences
                        .iter()
                        .map(|o| (o.sequence, o.date))
                        .collect::<Vec<_>>(),
                    expected
                );
            }
        }
    }
}
#[test]
fn limits_fail_closed_and_bad_definitions_are_rejected() {
    let d = def("2026-01-01", "10:00:00", "UTC");
    let rule = RecurrenceRule::new(d.clone()).unwrap();
    let horizon = h("2026-01-01T00:00:00Z", "2026-01-03T00:00:00Z");
    assert_eq!(
        rule.expand(horizon, 1),
        Err(TemporalError::OccurrenceLimitExceeded)
    );
    assert_eq!(
        rule.expand_report(
            horizon,
            ExpansionLimits {
                max_dates: 1,
                max_occurrences: 100
            }
        ),
        Err(TemporalError::DateLimitExceeded)
    );
    let mut invalid = d.clone();
    invalid.until = Some("2025-01-01".parse().unwrap());
    assert!(RecurrenceRule::new(invalid).is_err());
    for weekdays in [vec![], vec![Weekday::Mon, Weekday::Mon]] {
        let mut invalid = d.clone();
        invalid.pattern = RecurrencePattern::Weekly {
            every: 1.try_into().unwrap(),
            weekdays,
        };
        assert!(RecurrenceRule::new(invalid).is_err());
    }
    let mut json = serde_json::to_value(d).unwrap();
    json["pattern"]["every"] = 0.into();
    assert!(serde_json::from_value::<RecurrenceRule>(json).is_err());
}
#[test]
fn extreme_dates_fail_without_panics() {
    let d = def("2026-01-01", "10:00:00", "Pacific/Kiritimati");
    let horizon = PlanningHorizon(
        TimeRange::new(Instant::MAX_UTC - TimeDelta::days(1), Instant::MAX_UTC).unwrap(),
    );
    assert_eq!(expand(d, horizon), Err(TemporalError::Overflow));
    let d = def("2026-01-01", "10:00:00", "UTC");
    let horizon = PlanningHorizon(
        TimeRange::new(Instant::MIN_UTC, Instant::MIN_UTC + TimeDelta::days(1)).unwrap(),
    );
    assert_eq!(expand(d, horizon), Err(TemporalError::Overflow));
}
#[test]
fn seeded_malformed_json_never_panics() {
    let mut state = 42u64;
    for length in 0..512 {
        let bytes: Vec<_> = (0..length)
            .map(|_| {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                (state >> 32) as u8
            })
            .collect();
        let _ = serde_json::from_slice::<TemporalRequest>(&bytes);
    }
}
