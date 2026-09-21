#[path = "../../../tests/support/generator.rs"]
mod generator;
#[path = "../../../tests/support/mod.rs"]
mod support;
use cerebri_constraints::{ConstraintSpec, HardConstraint};
use cerebri_planner::*;
use cerebri_types::*;
use generator::Generator;

fn permutations(xs: &mut [usize], offset: usize, out: &mut Vec<Vec<usize>>) {
    if offset == xs.len() {
        out.push(xs.to_vec());
    }
    for i in offset..xs.len() {
        xs.swap(i, offset);
        permutations(xs, offset + 1, out);
        xs.swap(i, offset);
    }
}

#[test]
fn exhaustive_three_node_and_seeded_six_node_graphs_match_independent_oracles() {
    for seed in 0..768 {
        let mut rng = Generator(seed);
        let n = if seed < 512 { 3 } else { rng.pick(7) as usize };
        let ids: Vec<_> = (0..n)
            .map(|i| PlanningObjectId::new(format!("node-{i}")).unwrap())
            .collect();
        let mut objects: Vec<_> = ids
            .iter()
            .map(|id| {
                let mut object = support::request().context.objects[1].clone();
                object.id = id.clone();
                object
            })
            .collect();
        let mut edges = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if if seed < 512 {
                    seed & (1 << (i * n + j)) != 0
                } else {
                    rng.pick(4) == 0
                } {
                    edges.push((i, j));
                }
            }
        }
        let mut constraints: Vec<_> = edges
            .iter()
            .map(|&(a, b)| ConstraintSpec {
                object_id: ids[b].clone(),
                rule: HardConstraint::DependencyOrder(ids[a].clone()),
                evidence: vec![],
            })
            .collect();
        // Floyd-Warshall transitive closure, independent of production's per-root DFS.
        let mut reachable = vec![vec![false; n]; n];
        for &(a, b) in &edges {
            reachable[a][b] = true;
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    reachable[i][j] |= reachable[i][k] && reachable[k][j];
                }
            }
        }
        let mut components: Vec<Vec<PlanningObjectId>> = Vec::new();
        for (i, row) in reachable.iter().enumerate() {
            if row[i] {
                let members: Vec<_> = (0..n)
                    .filter(|&j| reachable[i][j] && reachable[j][i])
                    .map(|j| ids[j].clone())
                    .collect();
                if !components.contains(&members) {
                    components.push(members);
                }
            }
        }
        let missing = seed >= 512 && n > 0 && rng.pick(4) == 0;
        if missing {
            constraints.push(ConstraintSpec {
                object_id: ids[0].clone(),
                rule: HardConstraint::DependencyOrder(PlanningObjectId::new("missing").unwrap()),
                evidence: vec![],
            });
        }
        let graph = dependency_graph(&objects, &constraints);
        let cycles: Vec<_> = graph
            .issues
            .iter()
            .filter_map(|issue| match issue {
                DependencyIssue::Cycle(c) => Some(c),
                _ => None,
            })
            .collect();
        assert_eq!(
            cycles.iter().map(|c| c.members.clone()).collect::<Vec<_>>(),
            components,
            "seed={seed}; edges={edges:?}"
        );
        for cycle in cycles {
            let expected: Vec<_> = graph
                .edges
                .iter()
                .filter(|e| {
                    cycle.members.contains(&e.predecessor) && cycle.members.contains(&e.dependent)
                })
                .cloned()
                .collect();
            assert_eq!(cycle.edges, expected, "seed={seed}");
        }
        assert_eq!(
            graph
                .issues
                .iter()
                .filter(|i| matches!(i, DependencyIssue::MissingReference(_)))
                .count(),
            usize::from(missing)
        );
        // Enumerate every ordering rather than implementing another topological sorter.
        let mut orders = Vec::new();
        permutations(&mut (0..n).collect::<Vec<_>>(), 0, &mut orders);
        let expected = if missing {
            None
        } else {
            orders
                .into_iter()
                .filter(|p| {
                    edges
                        .iter()
                        .all(|(a, b)| p.iter().position(|v| v == a) < p.iter().position(|v| v == b))
                })
                .min()
        };
        assert_eq!(
            graph.order,
            expected
                .unwrap_or_default()
                .into_iter()
                .map(|i| ids[i].clone())
                .collect::<Vec<_>>(),
            "seed={seed}"
        );
        if !graph.issues.is_empty() && n > 0 {
            let mut input = support::request();
            input.operation = Operation::FindSlot;
            input.target_ids = vec![ids[0].clone()];
            input.context.objects = objects.clone();
            input.constraints = constraints.clone();
            let result = BaselinePlanner.plan(input.clone());
            assert_eq!(
                result.outcome,
                PlanningOutcome::InsufficientInformation,
                "seed={seed}"
            );
            assert_eq!(result.search_space.evaluated, 0);
            assert!(
                ProposedPlan::from_placements(&input, vec![])
                    .validate(&input.context)
                    .is_err()
            );
        }
        // Duplicate declarations are idempotent for the graph; not for request identity.
        if let Some(edge) = constraints.first().cloned() {
            constraints.push(edge);
        }
        rng.shuffle(&mut objects);
        rng.shuffle(&mut constraints);
        assert_eq!(
            dependency_graph(&objects, &constraints),
            graph,
            "permutation seed={seed}"
        );
    }
}

#[test]
fn maximum_graph_and_over_limit_are_bounded() {
    let mut input = support::request();
    let template = input.context.objects[1].clone();
    input.context.objects = (0..256)
        .map(|i| {
            let mut o = template.clone();
            o.id = PlanningObjectId::new(format!("n{i:03}")).unwrap();
            o
        })
        .collect();
    input.constraints = (0..1024)
        .map(|i| ConstraintSpec {
            object_id: input.context.objects[(i + 1) % 256].id.clone(),
            rule: HardConstraint::DependencyOrder(input.context.objects[i % 256].id.clone()),
            evidence: vec![],
        })
        .collect();
    let graph = dependency_graph(&input.context.objects, &input.constraints);
    assert_eq!(graph.nodes.len(), 256);
    assert_eq!(graph.edges.len(), 256);
    assert!(matches!(&graph.issues[..], [DependencyIssue::Cycle(c)] if c.members.len()==256));
    input.constraints.push(input.constraints[0].clone());
    assert_eq!(
        dependency_graph(&input.context.objects, &input.constraints).issues,
        vec![DependencyIssue::InputLimit]
    );
}
