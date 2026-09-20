#[path = "../../../tests/support/mod.rs"]
mod support;
use cerebri_constraints::{ConstraintSpec, HardConstraint};
use cerebri_planner::*;
use cerebri_types::*;
use support::*;

fn graph(nodes: &[&str], edges: &[(&str, &str)]) -> DependencyGraph {
    let base = request().context.objects[1].clone();
    let objects: Vec<_> = nodes
        .iter()
        .map(|id| {
            let mut o = base.clone();
            o.id = PlanningObjectId::new(*id).unwrap();
            o
        })
        .collect();
    let constraints: Vec<_> = edges
        .iter()
        .map(|(from, to)| ConstraintSpec {
            object_id: PlanningObjectId::new(*to).unwrap(),
            rule: HardConstraint::DependencyOrder(PlanningObjectId::new(*from).unwrap()),
            evidence: vec![],
        })
        .collect();
    dependency_graph(&objects, &constraints)
}
fn names(ids: &[PlanningObjectId]) -> Vec<&str> {
    ids.iter().map(|id| id.as_str()).collect()
}

#[test]
fn chains_independent_nodes_and_diamonds_have_stable_orders() {
    for (edges, order) in [
        (
            vec![("a", "b"), ("b", "c"), ("c", "d")],
            vec!["a", "b", "c", "d"],
        ),
        (vec![], vec!["a", "b", "c", "d"]),
        (
            vec![("a", "b"), ("a", "c"), ("b", "d"), ("c", "d")],
            vec!["a", "b", "c", "d"],
        ),
    ] {
        let g = graph(&["d", "b", "c", "a"], &edges);
        assert_eq!(names(&g.order), order);
        assert!(g.issues.is_empty());
        let mut reversed = edges;
        reversed.reverse();
        assert_eq!(g, graph(&["a", "c", "b", "d"], &reversed));
    }
}
#[test]
fn missing_dependencies_have_edge_evidence_and_no_claimed_order() {
    let g = graph(&["b"], &[("a", "b")]);
    assert!(g.order.is_empty());
    assert_eq!(
        g.issues,
        vec![DependencyIssue::MissingReference(DependencyEdge {
            predecessor: PlanningObjectId::new("a").unwrap(),
            dependent: PlanningObjectId::new("b").unwrap(),
        })]
    );
}
#[test]
fn self_two_and_long_cycles_exclude_merely_blocked_descendants() {
    for (edges, expected) in [
        (vec![("a", "a"), ("a", "d")], vec!["a"]),
        (vec![("a", "b"), ("b", "a"), ("b", "d")], vec!["a", "b"]),
        (
            vec![("a", "b"), ("b", "c"), ("c", "a"), ("c", "d")],
            vec!["a", "b", "c"],
        ),
    ] {
        let g = graph(&["d", "c", "b", "a"], &edges);
        assert!(g.order.is_empty());
        let [DependencyIssue::Cycle(cycle)] = &g.issues[..] else {
            panic!("expected one SCC")
        };
        assert_eq!(names(&cycle.members), expected);
        assert_eq!(cycle.edges.len(), expected.len());
        let mut reverse = edges.clone();
        reverse.reverse();
        assert_eq!(g, graph(&["a", "b", "c", "d"], &reverse));
    }
}
#[test]
fn duplicate_edges_do_not_change_graph_and_disjoint_cycles_remain_separate() {
    assert_eq!(
        graph(&["a", "b"], &[("a", "b")]),
        graph(&["a", "b"], &[("a", "b"), ("a", "b")])
    );
    let g = graph(
        &["a", "b", "c", "d"],
        &[("a", "b"), ("b", "a"), ("c", "d"), ("d", "c")],
    );
    assert_eq!(g.issues.len(), 2);
}
#[test]
fn invalid_graph_blocks_search_and_lifecycle_even_with_forged_placements() {
    for predecessor in ["missing", "new-event"] {
        let mut input = request();
        input.constraints.push(ConstraintSpec {
            object_id: input.target_ids[0].clone(),
            rule: HardConstraint::DependencyOrder(PlanningObjectId::new(predecessor).unwrap()),
            evidence: vec![],
        });
        let result = BaselinePlanner.plan(input.clone());
        assert_eq!(result.outcome, PlanningOutcome::InsufficientInformation);
        assert_eq!(result.search_space.evaluated, 0);
        assert!(
            result
                .validation
                .issues
                .iter()
                .any(|i| matches!(i, ValidationIssue::Dependency(_)))
        );
        let proposal = ProposedPlan::from_placements(
            &input,
            vec![placement(
                "new-event",
                "2026-10-01T11:00:00Z",
                "2026-10-01T11:30:00Z",
            )],
        );
        assert!(matches!(
            proposal.validate(&input.context),
            Err(PlanError::InvalidRequest(_))
        ));
    }
}
#[test]
fn graph_cannot_bypass_scope_policy_or_capability() {
    let mut input = request();
    input.constraints.push(ConstraintSpec {
        object_id: input.target_ids[0].clone(),
        rule: HardConstraint::DependencyOrder(PlanningObjectId::new("busy").unwrap()),
        evidence: vec![],
    });
    assert_eq!(
        BaselinePlanner.plan(input.clone()).outcome,
        PlanningOutcome::Solution
    );
    for mode in 0..3 {
        let mut denied = input.clone();
        match mode {
            0 => denied.scope.object_ids = Some(vec![]),
            1 => denied.policy.snapshot.mutation.allowed_actions.clear(),
            _ => denied.planning_capability.mutations.clear(),
        }
        assert_eq!(
            BaselinePlanner.plan(denied).outcome,
            PlanningOutcome::InsufficientInformation
        );
    }
}
#[test]
fn graph_limits_reject_before_traversal() {
    let input = request();
    let graph = dependency_graph(&vec![input.context.objects[0].clone(); 257], &[]);
    assert_eq!(graph.issues, vec![DependencyIssue::InputLimit]);
    assert!(graph.nodes.is_empty());
}
