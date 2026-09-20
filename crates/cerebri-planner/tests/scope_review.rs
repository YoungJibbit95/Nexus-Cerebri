#[path = "../../../tests/support/mod.rs"]
mod support;
use cerebri_planner::*;
use cerebri_types::*;
fn selected(filter: Option<&[&str]>, resources: &[&str]) -> bool {
    let mut input = support::request();
    input.scope.resource_ids =
        filter.map(|ids| ids.iter().map(|id| ResourceId::new(*id).unwrap()).collect());
    let mut object = input.context.objects[0].clone();
    object.resource_ids = resources
        .iter()
        .map(|id| ResourceId::new(*id).unwrap())
        .collect();
    input.scope.selects(&object)
}
#[test]
fn omitted_resource_filter_does_not_restrict() {
    assert!(selected(None, &["a", "b"]));
    assert!(selected(None, &[]));
}
#[test]
fn empty_resource_filter_selects_nothing() {
    assert!(!selected(Some(&[]), &["a"]));
    assert!(!selected(Some(&[]), &[]));
}
#[test]
fn single_matching_resource_is_selected() {
    assert!(selected(Some(&["a"]), &["a"]));
}
#[test]
fn partial_resource_overlap_is_excluded() {
    assert!(!selected(Some(&["a"]), &["a", "b"]));
}
#[test]
fn complete_resource_coverage_is_selected() {
    assert!(selected(Some(&["a", "b", "c"]), &["a", "b"]));
}
#[test]
fn disjoint_resources_are_excluded() {
    assert!(!selected(Some(&["c"]), &["a", "b"]));
}
#[test]
fn listed_resources_exclude_resource_free_objects() {
    assert!(!selected(Some(&["a"]), &[]));
}
#[test]
fn selecting_resources_does_not_grant_mutation() {
    let mut input = support::request();
    input.scope.resource_ids = None;
    assert!(input.scope.selects(&input.context.objects[0]));
    input.planning_capability.mutations.clear();
    assert_ne!(validate_request(&input).state, ValidationState::Valid);
}
#[test]
fn wire_omitted_and_null_remain_distinct_from_empty() {
    let source = serde_json::to_value(support::request().scope).unwrap();
    for field in [
        "calendar_ids",
        "object_ids",
        "resource_ids",
        "integration_ids",
        "movable_object_ids",
    ] {
        let mut absent = source.clone();
        absent.as_object_mut().unwrap().remove(field);
        let omitted: PlanningScope = serde_json::from_value(absent).unwrap();
        let mut null = source.clone();
        null[field] = serde_json::Value::Null;
        assert_eq!(
            omitted,
            serde_json::from_value::<PlanningScope>(null).unwrap()
        );
        let mut empty = source.clone();
        empty[field] = serde_json::json!([]);
        let empty: PlanningScope = serde_json::from_value(empty).unwrap();
        assert_ne!(omitted, empty);
        assert_eq!(
            serde_json::to_value(empty).unwrap()[field],
            serde_json::json!([])
        );
    }
}
