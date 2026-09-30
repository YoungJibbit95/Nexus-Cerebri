//! Bounded canonical labeling (§§13–16). Opaque identities affect only the secondary
//! binding tie-break after equality of complete semantic payload bytes.
use super::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const FINGERPRINT_CANONICALIZATION_STATE_LIMIT: usize = 100_000;

pub(crate) fn vertex_rank(v: GraphVertexTypeToken) -> u8 {
    match v {
        GraphVertexTypeToken::Object => 0,
        GraphVertexTypeToken::Fact => 1,
        GraphVertexTypeToken::Calendar => 2,
        GraphVertexTypeToken::Integration => 3,
        GraphVertexTypeToken::Resource => 4,
        GraphVertexTypeToken::Series => 5,
    }
}
pub fn edge_type_rank(v: GraphEdgeTypeToken) -> u8 {
    match v {
        GraphEdgeTypeToken::DependsOnObject => 0,
        GraphEdgeTypeToken::FactSubject => 1,
        GraphEdgeTypeToken::ObjectCalendar => 2,
        GraphEdgeTypeToken::ObjectIntegration => 3,
        GraphEdgeTypeToken::ObjectResource => 4,
        GraphEdgeTypeToken::EvidenceFactReference => 5,
    }
}
pub(crate) fn identity_type(v: GraphVertexTypeToken) -> IdentityTypeV1 {
    match v {
        GraphVertexTypeToken::Object => IdentityTypeV1::PlanningObjectId,
        GraphVertexTypeToken::Fact => IdentityTypeV1::FactId,
        GraphVertexTypeToken::Calendar => IdentityTypeV1::CalendarId,
        GraphVertexTypeToken::Integration => IdentityTypeV1::IntegrationId,
        GraphVertexTypeToken::Resource => IdentityTypeV1::ResourceId,
        GraphVertexTypeToken::Series => IdentityTypeV1::SeriesId,
    }
}
closed_wire! {
pub struct IdentityBindingV1 {
    pub alias: CanonicalAlias,
    pub identity_type: IdentityTypeV1,
    pub source_id: IdentifierString,
}
}
closed_wire! {
pub struct GraphSourceVertexV1 {
    pub source_id: IdentifierString,
    #[serde(rename="type")]
    pub vertex_type: GraphVertexTypeToken,
    pub attributes: Vec<CanonicalAttributeV1>,
}
}
impl GraphSourceVertexV1 {
    /// An identity mentioned by a relation or external REF-bearing payload, without
    /// a declared entity. No source ID is copied into intrinsic attributes.
    pub fn reference_only(vertex_type: GraphVertexTypeToken, source_id: IdentifierString) -> Self {
        Self {
            vertex_type,
            source_id,
            attributes: super::graph_registry::reference_attributes(vertex_type),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphSourceIdentityV1 {
    pub vertex_type: GraphVertexTypeToken,
    pub source_id: IdentifierString,
}
/// Identity-based input relation; source indices are assigned only inside the graph boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphSourceRelationV1 {
    pub src: GraphSourceIdentityV1,
    pub dst: GraphSourceIdentityV1,
    pub edge_type: GraphEdgeTypeToken,
    pub payload: Vec<CanonicalAttributeV1>,
    pub multiplicity: u64,
}
/// Source indices preserve identity relations only; encounter order is never a tie-break.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphSourceEdgeV1 {
    pub src: usize,
    pub dst: usize,
    pub edge_type: GraphEdgeTypeToken,
    pub payload: Vec<CanonicalAttributeV1>,
    pub multiplicity: u64,
}
closed_wire! {
pub struct CanonicalVertexV1 {
    pub alias: CanonicalAlias,
    pub attributes: Vec<CanonicalAttributeV1>,
    #[serde(rename="type")]
    pub vertex_type: GraphVertexTypeToken,
}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanonicalEdgeDirectionV1 {
    #[serde(rename = "DIRECTED")]
    Directed,
}
closed_wire! {
pub struct CanonicalEdgeV1 {
    pub direction: CanonicalEdgeDirectionV1,
    pub src: CanonicalAlias,
    pub dst: CanonicalAlias,
    #[serde(rename="type")]
    pub edge_type: GraphEdgeTypeToken,
    pub payload: Vec<CanonicalAttributeV1>,
    pub multiplicity: CanonicalU64Decimal,
}
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CanonicalGraphSerializationV1 {
    schema_version: &'static str,
    vertices: Vec<CanonicalVertexV1>,
    edges: Vec<CanonicalEdgeV1>,
}
impl CanonicalGraphSerializationV1 {
    pub fn vertices(&self) -> &[CanonicalVertexV1] {
        &self.vertices
    }
    pub fn edges(&self) -> &[CanonicalEdgeV1] {
        &self.edges
    }
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, EvaluationContractError> {
        canonical_json_bytes(self)
    }
}
#[derive(Debug, Clone)]
pub struct CanonicalGraphSourceV1 {
    vertices: Vec<GraphSourceVertexV1>,
    edges: Vec<GraphSourceEdgeV1>,
    payload_bytes: Vec<Vec<u8>>,
}
#[derive(Debug, Clone)]
pub struct CanonicalLabelingV1 {
    pub graph: CanonicalGraphSerializationV1,
    pub identity_bindings: Vec<IdentityBindingV1>,
    pub payload_bytes: Vec<u8>,
}
impl CanonicalGraphSourceV1 {
    /// Declared/source vertices are registered first. Missing relation endpoints and
    /// extra references become REFERENCE_ONLY vertices. The caller identifies the
    /// architecture-defined relations; no PlanningRequest or BSF projection is implied.
    pub fn from_relations(
        mut vertices: Vec<GraphSourceVertexV1>,
        relations: Vec<GraphSourceRelationV1>,
        references: Vec<GraphSourceIdentityV1>,
    ) -> Result<Self, EvaluationContractError> {
        let mut identities = BTreeMap::new();
        for (index, vertex) in vertices.iter().enumerate() {
            if identities
                .insert(
                    (vertex_rank(vertex.vertex_type), vertex.source_id.clone()),
                    index,
                )
                .is_some()
            {
                return Err(EvaluationContractError("duplicate graph source identity"));
            }
        }
        let mut index_of = |identity: GraphSourceIdentityV1| {
            *identities
                .entry((
                    vertex_rank(identity.vertex_type),
                    identity.source_id.clone(),
                ))
                .or_insert_with(|| {
                    let index = vertices.len();
                    vertices.push(GraphSourceVertexV1::reference_only(
                        identity.vertex_type,
                        identity.source_id,
                    ));
                    index
                })
        };
        let mut edges = Vec::new();
        for relation in relations {
            edges.push(GraphSourceEdgeV1 {
                src: index_of(relation.src),
                dst: index_of(relation.dst),
                edge_type: relation.edge_type,
                payload: relation.payload,
                multiplicity: relation.multiplicity,
            });
        }
        for reference in references {
            index_of(reference);
        }
        Self::new(vertices, edges)
    }
    pub fn new(
        mut vertices: Vec<GraphSourceVertexV1>,
        mut edges: Vec<GraphSourceEdgeV1>,
    ) -> Result<Self, EvaluationContractError> {
        if vertices.len() > 1_000_000 {
            return Err(EvaluationContractError("canonical alias domain exceeded"));
        }
        let mut identities = BTreeSet::new();
        for vertex in &mut vertices {
            vertex.attributes.sort_by(|a, b| a.key.cmp(&b.key));
            super::graph_registry::validate_vertex(vertex)?;
            if !identities.insert((vertex_rank(vertex.vertex_type), vertex.source_id.clone())) {
                return Err(EvaluationContractError("duplicate graph source identity"));
            }
        }
        let mut aggregate: BTreeMap<_, GraphSourceEdgeV1> = BTreeMap::new();
        for mut edge in edges.drain(..) {
            edge.payload.sort_by(|a, b| a.key.cmp(&b.key));
            super::graph_registry::validate_edge(&vertices, &edge)?;
            let key = (
                edge.src,
                edge_type_rank(edge.edge_type),
                edge.dst,
                canonical_json_bytes(&edge.payload)?,
            );
            if let Some(prior) = aggregate.get_mut(&key) {
                prior.multiplicity = prior
                    .multiplicity
                    .checked_add(edge.multiplicity)
                    .ok_or(EvaluationContractError("edge multiplicity overflow"))?;
            } else {
                aggregate.insert(key, edge);
            }
        }
        let edges: Vec<_> = aggregate.into_values().collect();
        for (index, vertex) in vertices
            .iter()
            .enumerate()
            .filter(|(_, v)| v.vertex_type == GraphVertexTypeToken::Fact)
        {
            let subjects: Vec<_> = edges
                .iter()
                .filter(|e| e.src == index && e.edge_type == GraphEdgeTypeToken::FactSubject)
                .collect();
            let declared = super::graph_registry::declared(&vertex.attributes)?;
            if (declared && (subjects.len() != 1 || subjects[0].multiplicity != 1))
                || (!declared && !subjects.is_empty())
            {
                return Err(EvaluationContractError(
                    "declared fact requires exactly one subject",
                ));
            }
        }
        let payload_bytes = edges
            .iter()
            .map(|e| canonical_json_bytes(&e.payload))
            .collect::<Result<_, _>>()?;
        Ok(Self {
            vertices,
            edges,
            payload_bytes,
        })
    }
    /// Graph-only conformance helper (G1/G2/G3), not the normative BSF labeling rule.
    /// External semantic references require `canonicalize_with_payload` instead.
    pub fn canonicalize(&self) -> Result<CanonicalLabelingV1, EvaluationContractError> {
        self.canonicalize_with_payload(|graph, _| graph.canonical_bytes())
    }
    /// Minimize architecture-controlled complete payload bytes for every complete
    /// labeling. The renderer must be pure, deterministic, use canonical JCS bytes,
    /// include the graph and every semantic external REF, and exclude opaque IDs.
    /// Source IDs participate only in the secondary tie-break after payload equality.
    /// Phase B.2 will supply the actual BaseScenario renderer; this is not BSF support.
    pub fn canonicalize_with_payload<F>(
        &self,
        render: F,
    ) -> Result<CanonicalLabelingV1, EvaluationContractError>
    where
        F: Fn(
            &CanonicalGraphSerializationV1,
            &[IdentityBindingV1],
        ) -> Result<Vec<u8>, EvaluationContractError>,
    {
        let mut cells: BTreeMap<_, Vec<usize>> = BTreeMap::new();
        for (index, vertex) in self.vertices.iter().enumerate() {
            cells
                .entry((
                    vertex_rank(vertex.vertex_type),
                    canonical_json_bytes(&vertex.attributes)?,
                ))
                .or_default()
                .push(index);
        }
        let mut search = LabelSearch {
            source: self,
            render,
            states: 0,
            best: None,
        };
        search.visit(cells.into_values().collect())?;
        search
            .best
            .map(|(_, v)| v)
            .ok_or(EvaluationContractError("no canonical labeling"))
    }
    fn refine(&self, mut cells: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        loop {
            let mut cell_of = vec![0; self.vertices.len()];
            for (cell, members) in cells.iter().enumerate() {
                for &v in members {
                    cell_of[v] = cell;
                }
            }
            let mut refined = Vec::new();
            for members in &cells {
                let mut groups: BTreeMap<IncidentMultiset, Vec<usize>> = BTreeMap::new();
                for &v in members {
                    let mut signatures = BTreeMap::new();
                    for (edge, payload) in self.edges.iter().zip(&self.payload_bytes) {
                        for (direction, neighbor) in [
                            (0, (edge.src == v).then_some(edge.dst)),
                            (1, (edge.dst == v).then_some(edge.src)),
                        ] {
                            if let Some(neighbor) = neighbor {
                                *signatures
                                    .entry((
                                        direction,
                                        edge_type_rank(edge.edge_type),
                                        payload.clone(),
                                        cell_of[neighbor],
                                    ))
                                    .or_insert(0u128) += u128::from(edge.multiplicity);
                            }
                        }
                    }
                    groups
                        .entry(IncidentMultiset(signatures.into_iter().collect()))
                        .or_default()
                        .push(v);
                }
                refined.extend(groups.into_values());
            }
            if refined.len() == cells.len() {
                return refined;
            }
            cells = refined;
        }
    }
    fn render(
        &self,
        order: &[usize],
    ) -> Result<(CanonicalGraphSerializationV1, Vec<IdentityBindingV1>), EvaluationContractError>
    {
        let mut aliases = vec![CanonicalAlias::new(0)?; order.len()];
        let mut vertices = Vec::new();
        let mut bindings = Vec::new();
        for (index, &source) in order.iter().enumerate() {
            let alias = CanonicalAlias::new(index as u32)?;
            aliases[source] = alias.clone();
            let vertex = &self.vertices[source];
            vertices.push(CanonicalVertexV1 {
                alias: alias.clone(),
                vertex_type: vertex.vertex_type,
                attributes: vertex.attributes.clone(),
            });
            bindings.push(IdentityBindingV1 {
                alias,
                identity_type: identity_type(vertex.vertex_type),
                source_id: vertex.source_id.clone(),
            });
        }
        let mut edges: Vec<_> = self
            .edges
            .iter()
            .zip(&self.payload_bytes)
            .map(|(e, p)| {
                (
                    (
                        aliases[e.src].clone(),
                        edge_type_rank(e.edge_type),
                        aliases[e.dst].clone(),
                        p,
                    ),
                    CanonicalEdgeV1 {
                        direction: CanonicalEdgeDirectionV1::Directed,
                        src: aliases[e.src].clone(),
                        dst: aliases[e.dst].clone(),
                        edge_type: e.edge_type,
                        payload: e.payload.clone(),
                        multiplicity: CanonicalU64Decimal::new(e.multiplicity).expect("u64"),
                    },
                )
            })
            .collect();
        edges.sort_by(|a, b| a.0.cmp(&b.0));
        Ok((
            CanonicalGraphSerializationV1 {
                schema_version: "1",
                vertices,
                edges: edges.into_iter().map(|(_, e)| e).collect(),
            },
            bindings,
        ))
    }
}
// Compare the expanded sorted multiset without allocating multiplicity-many entries.
type IncidentSignature = (u8, u8, Vec<u8>, usize);
#[derive(Debug, Clone, PartialEq, Eq)]
struct IncidentMultiset(Vec<(IncidentSignature, u128)>);
impl PartialOrd for IncidentMultiset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for IncidentMultiset {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        let (mut a, mut b) = (0, 0);
        let (mut ar, mut br) = (0, 0);
        while a < self.0.len() && b < other.0.len() {
            let cmp = self.0[a].0.cmp(&other.0[b].0);
            if cmp != Ordering::Equal {
                return cmp;
            }
            if ar == 0 {
                ar = self.0[a].1;
            }
            if br == 0 {
                br = other.0[b].1;
            }
            let n = ar.min(br);
            ar -= n;
            br -= n;
            if ar == 0 {
                a += 1;
            }
            if br == 0 {
                b += 1;
            }
        }
        (self.0.len() - a).cmp(&(other.0.len() - b))
    }
}
struct LabelSearch<'a, F> {
    source: &'a CanonicalGraphSourceV1,
    render: F,
    states: usize,
    best: Option<(Vec<u8>, CanonicalLabelingV1)>,
}

impl<F> LabelSearch<'_, F>
where
    F: Fn(
        &CanonicalGraphSerializationV1,
        &[IdentityBindingV1],
    ) -> Result<Vec<u8>, EvaluationContractError>,
{
    fn visit(&mut self, cells: Vec<Vec<usize>>) -> Result<(), EvaluationContractError> {
        // Explicit DFS frames avoid consuming the process stack for large source graphs.
        // Each frame retains only its partition and next choice, not all child states.
        let mut frames: Vec<(Vec<Vec<usize>>, usize, usize)> = Vec::new();
        let mut pending = cells;
        loop {
            self.states += 1;
            if self.states > FINGERPRINT_CANONICALIZATION_STATE_LIMIT {
                return Err(EvaluationContractError(
                    "FINGERPRINT_CANONICALIZATION_LIMIT",
                ));
            }
            let cells = self.source.refine(pending);
            if let Some(index) = cells.iter().position(|c| c.len() > 1) {
                frames.push((cells, index, 0));
            } else {
                self.consider(&cells)?;
            }
            pending = loop {
                let Some((cells, index, next)) = frames.last_mut() else {
                    return Ok(());
                };
                if *next == cells[*index].len() {
                    frames.pop();
                    continue;
                }
                let chosen = cells[*index][*next];
                *next += 1;
                let index = *index;
                let mut child = cells[..index].to_vec();
                child.push(vec![chosen]);
                child.push(
                    cells[index]
                        .iter()
                        .copied()
                        .filter(|v| *v != chosen)
                        .collect(),
                );
                child.extend_from_slice(&cells[index + 1..]);
                break child;
            };
        }
    }
    fn consider(&mut self, cells: &[Vec<usize>]) -> Result<(), EvaluationContractError> {
        let order: Vec<_> = cells.iter().flatten().copied().collect();
        let (graph, identity_bindings) = self.source.render(&order)?;
        let payload_bytes = (self.render)(&graph, &identity_bindings)?;
        #[derive(Serialize)]
        struct Tie<'a> {
            bindings: &'a [IdentityBindingV1],
        }
        let tie = canonical_json_bytes(&Tie {
            bindings: &identity_bindings,
        })?;
        if self
            .best
            .as_ref()
            .is_none_or(|(old_tie, old)| (&payload_bytes, &tie) < (&old.payload_bytes, old_tie))
        {
            self.best = Some((
                tie,
                CanonicalLabelingV1 {
                    graph,
                    identity_bindings,
                    payload_bytes,
                },
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_100000_is_allowed_and_100001_never_renders() {
        let source = CanonicalGraphSourceV1::new(vec![], vec![]).unwrap();
        for (previous, allowed) in [(99_999, true), (100_000, false)] {
            let calls = std::cell::Cell::new(0);
            let mut search = LabelSearch {
                source: &source,
                render: |g: &CanonicalGraphSerializationV1, _: &[IdentityBindingV1]| {
                    calls.set(calls.get() + 1);
                    g.canonical_bytes()
                },
                states: previous,
                best: None,
            };
            assert_eq!(search.visit(vec![]).is_ok(), allowed);
            assert_eq!(calls.get(), usize::from(allowed));
            assert_eq!(search.best.is_some(), allowed);
        }
    }

    #[test]
    fn incident_multiset_comparison_matches_expanded_lexicographic_order() {
        let signatures = [(0, 0, vec![], 0), (0, 4, vec![], 1), (1, 0, vec![], 0)];
        let mut cases = Vec::new();
        for a in 0..3 {
            for b in 0..3 {
                for c in 0..3 {
                    let counts = [a, b, c];
                    let compressed = IncidentMultiset(
                        signatures
                            .iter()
                            .zip(counts)
                            .filter(|(_, n)| *n > 0)
                            .map(|(s, n)| (s.clone(), n as u128))
                            .collect(),
                    );
                    let expanded: Vec<_> = signatures
                        .iter()
                        .zip(counts)
                        .flat_map(|(s, n)| std::iter::repeat_n(s.clone(), n))
                        .collect();
                    cases.push((compressed, expanded));
                }
            }
        }
        for (a, expanded_a) in &cases {
            for (b, expanded_b) in &cases {
                assert_eq!(a.cmp(b), expanded_a.cmp(expanded_b));
            }
        }
    }
}
