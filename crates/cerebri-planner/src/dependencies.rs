//! Bounded, iterative graph inspection. No workflow execution or mutation permissions.
use crate::PlanningObject;
use cerebri_constraints::{ConstraintSpec, HardConstraint};
use cerebri_types::PlanningObjectId;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DependencyEdge {
    pub predecessor: PlanningObjectId,
    pub dependent: PlanningObjectId,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyCycle {
    pub members: Vec<PlanningObjectId>,
    pub edges: Vec<DependencyEdge>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyIssue {
    InputLimit,
    MissingReference(DependencyEdge),
    Cycle(DependencyCycle),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub nodes: Vec<PlanningObjectId>,
    pub edges: Vec<DependencyEdge>,
    /// Empty when any issue exists; never a complete order for an invalid graph.
    pub order: Vec<PlanningObjectId>,
    pub issues: Vec<DependencyIssue>,
}

pub fn dependency_graph(
    objects: &[PlanningObject],
    constraints: &[ConstraintSpec],
) -> DependencyGraph {
    let mut graph = DependencyGraph {
        nodes: vec![],
        edges: vec![],
        order: vec![],
        issues: vec![],
    };
    if objects.len() > 256 || constraints.len() > 1_024 {
        graph.issues.push(DependencyIssue::InputLimit);
        return graph;
    }
    graph.nodes = objects
        .iter()
        .map(|o| o.id.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    graph.edges = constraints
        .iter()
        .filter_map(|c| {
            if let HardConstraint::DependencyOrder(predecessor) = &c.rule {
                Some(DependencyEdge {
                    predecessor: predecessor.clone(),
                    dependent: c.object_id.clone(),
                })
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let indices: BTreeMap<_, _> = graph
        .nodes
        .iter()
        .enumerate()
        .map(|(i, id)| (id, i))
        .collect();
    let n = graph.nodes.len();
    let mut successors = vec![Vec::new(); n];
    let mut indegree = vec![0usize; n];
    for edge in &graph.edges {
        if let (Some(&from), Some(&to)) =
            (indices.get(&edge.predecessor), indices.get(&edge.dependent))
        {
            successors[from].push(to);
            indegree[to] += 1;
        } else {
            graph
                .issues
                .push(DependencyIssue::MissingReference(edge.clone()));
        }
    }
    // At most 256 traversals of 256 nodes / 1,024 edges. No recursive stack.
    let mut reachable = vec![vec![false; n]; n];
    for (root, row) in reachable.iter_mut().enumerate() {
        let mut stack = successors[root].clone();
        while let Some(node) = stack.pop() {
            if row[node] {
                continue;
            }
            row[node] = true;
            stack.extend(&successors[node]);
        }
    }
    let mut assigned: BTreeSet<usize> = BTreeSet::new();
    for (i, row) in reachable.iter().enumerate() {
        if !row[i] || assigned.contains(&i) {
            continue;
        }
        let component: Vec<_> = (0..n).filter(|&j| row[j] && reachable[j][i]).collect();
        assigned.extend(&component);
        let members: Vec<_> = component
            .into_iter()
            .map(|j| graph.nodes[j].clone())
            .collect();
        let edges = graph
            .edges
            .iter()
            .filter(|e| members.contains(&e.predecessor) && members.contains(&e.dependent))
            .cloned()
            .collect();
        graph
            .issues
            .push(DependencyIssue::Cycle(DependencyCycle { members, edges }));
    }
    if graph.issues.is_empty() {
        let mut ready: BTreeSet<_> = (0..n).filter(|&i| indegree[i] == 0).collect();
        while let Some(node) = ready.pop_first() {
            graph.order.push(graph.nodes[node].clone());
            for &next in &successors[node] {
                indegree[next] -= 1;
                if indegree[next] == 0 {
                    ready.insert(next);
                }
            }
        }
    }
    graph
}
