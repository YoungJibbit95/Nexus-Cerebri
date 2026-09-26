//! Build-time inspection of existing fixtures and ADR-0014 contract probes.
use cerebri_core::{PlanningRequest, plan};
use cerebri_temporal::{Instant, TimeDelta, TimeRange};
use serde_json::{Value, json};

const PREFERRED: &str = include_str!("../../../examples/02-ranking-preferred-start.json");
const TOUCHING: &str = include_str!("../../../examples/03-halfopen-touching-boundary.json");
const OVERLAP: &str = include_str!("../../../examples/04-halfopen-one-second-overlap.json");

fn interval_case(input: &str, source: &str) -> Value {
    let wire: Value = serde_json::from_str(input).unwrap();
    let request: PlanningRequest = serde_json::from_str(input).unwrap();
    let busy = request
        .context
        .objects
        .iter()
        .find(|o| o.id.as_str() == "busy")
        .unwrap();
    let a = *busy.time.value.required(false).unwrap().0;
    let b: TimeRange =
        serde_json::from_value(wire["constraints"][0]["rule"]["value"].clone()).unwrap();
    let result = plan(request);
    let detail_ticks = [-1, 0, 1, 2].map(|seconds| b.start() + TimeDelta::seconds(seconds));
    json!({
        "source": source, "a": a, "b": b,
        "overlaps": a.overlaps(b), "intersection": a.intersection(b),
        "relation": a.relation(b), "outcome": result.outcome,
        "rejections": result.conflicts.rejections,
        "detail_ticks": detail_ticks,
        "shared_endpoint_in_a": a.contains(b.start()),
        "shared_endpoint_in_b": b.contains(b.start())
    })
}

fn inspection_data() -> Value {
    let request: PlanningRequest = serde_json::from_str(PREFERRED).unwrap();
    let preferred = request
        .preferences
        .preferred_start()
        .unwrap()
        .preferred_start;
    let result = plan(request.clone());
    let candidates: Vec<_> = result
        .candidates
        .iter()
        .map(|c| {
            json!({
                "start": c.start, "range": c.proposed.placements()[0].range,
                "actual_delta_ms": (c.start - preferred).num_milliseconds(),
                "ranking_features": c.ranking_features, "ordering_key": c.ordering_key
            })
        })
        .collect();

    // Same signed offsets as whole_second_distance_and_shift_preserve_missingness_and_quantization.
    // These exercise the production observation function, not planner search or provider state.
    let mut probe = request.clone();
    let origin = Instant::from_timestamp(0, 0).unwrap();
    probe.preferences.preferences[0].preferred_start = origin;
    let samples: Vec<_> = [0, 800, -800, 1000, -1000, 12345, -12345].into_iter().map(|ms| {
        let candidate = origin + TimeDelta::milliseconds(ms);
        json!({
            "preferred_start": origin, "original_start": origin, "candidate_start": candidate,
            "actual_delta_ms": (candidate - origin).num_milliseconds(),
            "ranking_features": probe.preferences.ranking_features(candidate, 1, Some(origin)),
            "ordering_projection": probe.preferences.features(candidate).distance_from_preferred_start_seconds
        })
    }).collect();
    let mut absent = probe.clone();
    absent.preferences.preferences.clear();
    json!({
        "intervals": [
            interval_case(TOUCHING, "examples/03-halfopen-touching-boundary.json"),
            interval_case(OVERLAP, "examples/04-halfopen-one-second-overlap.json")
        ],
        "preferred": {
            "source": "examples/02-ranking-preferred-start.json", "preferred_start": preferred,
            "scope": request.scope.time_range, "candidates": candidates
        },
        "measurement": {
            "source": "crates/cerebri-preferences/tests/ranking.rs",
            "samples": samples,
            "absent": {
                "original_start": null, "preferred_start": null, "candidate_start": origin,
                "ranking_features": absent.preferences.ranking_features(origin, 1, None),
                "ordering_projection": absent.preferences.features(origin).distance_from_preferred_start_seconds
            }
        }
    })
}

fn main() {
    println!("{}", inspection_data());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exported_intersections_match_the_paired_fixture_oracles() {
        let data = inspection_data();
        let touching = &data["intervals"][0];
        assert_eq!(touching["relation"], "Touches");
        assert_eq!(touching["overlaps"], false);
        assert!(touching["intersection"].is_null());
        assert_eq!(touching["shared_endpoint_in_a"], false);
        assert_eq!(touching["shared_endpoint_in_b"], true);
        let overlap = &data["intervals"][1];
        assert_eq!(overlap["overlaps"], true);
        assert_eq!(overlap["intersection"]["start"], "2026-10-01T10:00:00Z");
        assert_eq!(overlap["intersection"]["end"], "2026-10-01T10:00:01Z");
        assert_eq!(overlap["outcome"], "NoSolution");
        assert!(!overlap["rejections"].as_array().unwrap().is_empty());
    }

    #[test]
    fn export_preserves_evidence_subseconds_and_rust_candidate_order() {
        let data = inspection_data();
        assert_eq!(data["preferred"]["candidates"].as_array().unwrap().len(), 7);
        assert_eq!(
            data["preferred"]["candidates"][0]["start"],
            "2026-10-01T10:45:00Z"
        );
        let samples = &data["measurement"]["samples"];
        for (i, expected) in [0, 0, 0, 1, 1, 12, 12].into_iter().enumerate() {
            assert_eq!(
                samples[i]["ranking_features"]["preferred_start_distance_seconds"],
                expected
            );
            assert_eq!(samples[i]["ranking_features"]["shift_seconds"], expected);
            assert_eq!(samples[i]["ordering_projection"], expected);
        }
        assert_eq!(samples[1]["candidate_start"], "1970-01-01T00:00:00.800Z");
        assert_eq!(samples[1]["actual_delta_ms"], 800);
        assert!(
            data["measurement"]["absent"]["ranking_features"]["preferred_start_distance_seconds"]
                .is_null()
        );
        assert!(data["measurement"]["absent"]["original_start"].is_null());
        assert_eq!(data["measurement"]["absent"]["ordering_projection"], 0);
    }
}
