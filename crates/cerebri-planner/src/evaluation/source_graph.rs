//! Exhaustive identity/reference projection of the current PlanningRequest source model.
use super::graph_registry::cv as c;
use super::scenario::{ProjectionResult, instant, knowledge, provenance};
use super::*;
use crate::*;
use CanonicalValueKindV1 as K;
use GraphEdgeTypeToken as E;
use GraphVertexTypeToken as V;
use cerebri_constraints::{FactValue, HardConstraint};
use cerebri_temporal::TimeRange;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
struct Source {
    vertices: Vec<GraphSourceVertexV1>,
    identities: BTreeMap<(u8, String), usize>,
    edges: Vec<GraphSourceEdgeV1>,
}
impl Source {
    fn add(
        &mut self,
        kind: V,
        id: &str,
        attrs: Vec<CanonicalAttributeV1>,
    ) -> ProjectionResult<usize> {
        let key = (super::graph::vertex_rank(kind), id.to_owned());
        if self.identities.contains_key(&key) {
            return Err(EvaluationContractError(match kind {
                V::Object => "DUPLICATE_OBJECT",
                V::Fact => "DUPLICATE_FACT",
                V::Series => "COMPILATION_DUPLICATE_SERIES",
                _ => "duplicate declared source identity",
            }));
        }
        let index = self.vertices.len();
        self.vertices.push(GraphSourceVertexV1 {
            source_id: IdentifierString::new(id)?,
            vertex_type: kind,
            attributes: attrs,
        });
        self.identities.insert(key, index);
        Ok(index)
    }
    fn reference(&mut self, kind: V, id: &str) -> ProjectionResult<usize> {
        if let Some(index) = self
            .identities
            .get(&(super::graph::vertex_rank(kind), id.to_owned()))
        {
            return Ok(*index);
        }
        let vertex = GraphSourceVertexV1::reference_only(kind, IdentifierString::new(id)?);
        self.add(kind, id, vertex.attributes)
    }
    fn link(
        &mut self,
        src: usize,
        kind: E,
        target_kind: V,
        target: &str,
        role: Option<&str>,
    ) -> ProjectionResult<()> {
        let dst = self.reference(target_kind, target)?;
        let payload =
            role.map_or_else(Vec::new, |r| c::attrs(vec![("evidence_role", c::token(r))]));
        let e = GraphSourceEdgeV1 {
            src,
            dst,
            edge_type: kind,
            payload,
            multiplicity: 1,
        };
        // These source relationships are semantic sets. General graph inputs can
        // still contain meaningful parallel edges and are aggregated by the graph kernel.
        if !self.edges.contains(&e) {
            self.edges.push(e);
        }
        Ok(())
    }
    fn facts(&mut self, ids: &[cerebri_types::FactId]) -> ProjectionResult<()> {
        for id in ids {
            self.reference(V::Fact, id.as_str())?;
        }
        Ok(())
    }
}
fn instant_value(value: cerebri_temporal::Instant) -> ProjectionResult<CanonicalValueV1> {
    let value = instant(value)?;
    CanonicalValueV1::new(K::Instant {
        s: value.unix_seconds,
        ns: value.nanoseconds,
    })
}
fn range(v: TimeRange) -> ProjectionResult<CanonicalValueV1> {
    Ok(c::object(vec![
        ("start", instant_value(v.start())?),
        ("end", instant_value(v.end())?),
    ]))
}
fn object_attrs(o: &PlanningObject) -> ProjectionResult<Vec<CanonicalAttributeV1>> {
    let PlanningObject {
        id: _,
        revision: _,
        kind,
        calendar_id: _,
        integration_id: _,
        resource_ids: _,
        time,
        timezone,
        semantics: _,
    } = o;
    let (kind, details) = match kind {
        PlanningObjectKind::Event => ("EVENT", c::none()),
        PlanningObjectKind::Availability => ("AVAILABILITY", c::none()),
        PlanningObjectKind::Resource => ("RESOURCE", c::none()),
        PlanningObjectKind::Deadline(d) => (
            "DEADLINE",
            c::object(vec![("deadline", instant_value(d.0)?)]),
        ),
        PlanningObjectKind::Task(t) => (
            "TASK",
            c::object(vec![
                ("splittable", c::value(K::Bool { v: t.splittable })),
                ("interruptible", c::value(K::Bool { v: t.interruptible })),
                (
                    "minimum_chunk_seconds",
                    t.minimum_chunk
                        .map_or_else(c::none, |v| c::unsigned(v.as_seconds() as u64)),
                ),
                (
                    "preferred_chunk_seconds",
                    t.preferred_chunk
                        .map_or_else(c::none, |v| c::unsigned(v.as_seconds() as u64)),
                ),
                (
                    "maximum_chunk_count",
                    t.maximum_chunk_count
                        .map_or_else(c::none, |v| c::unsigned(u64::from(v))),
                ),
            ]),
        ),
    };
    let (state, values) = knowledge(&time.value);
    let mut values = values
        .into_iter()
        .map(|v| range(*v))
        .collect::<ProjectionResult<Vec<_>>>()?;
    values.sort();
    values.dedup();
    Ok(c::attrs(vec![
        ("entity_presence", c::token("DECLARED")),
        ("object_kind", c::token(kind)),
        ("object_kind_details", details),
        ("object_timezone", c::text(timezone.name())),
        (
            "object_time_evidence",
            c::object(vec![
                ("knowledge_state", c::token(state.as_str())),
                ("provenance", c::token(provenance(time.provenance).as_str())),
                ("values", c::value(K::List { v: values })),
            ]),
        ),
    ]))
}
pub(crate) fn project(r: &PlanningRequest) -> ProjectionResult<CanonicalGraphSourceV1> {
    let mut s = Source::default();
    let object_ids: BTreeSet<_> = r.context.objects.iter().map(|o| o.id.as_str()).collect();
    for o in &r.context.objects {
        s.add(V::Object, o.id.as_str(), object_attrs(o)?)?;
    }
    for f in &r.context.facts {
        let (kind, value) = match &f.value {
            FactValue::ScheduledTime(v) => ("SCHEDULED_TIME", range(*v)?),
            FactValue::Availability(v) => ("AVAILABILITY", range(*v)?),
            FactValue::Deadline(v) => ("DEADLINE", instant_value(v.0)?),
            FactValue::ExternalLock => ("EXTERNAL_LOCK", c::none()),
        };
        s.add(
            V::Fact,
            f.id.as_str(),
            c::attrs(vec![
                ("entity_presence", c::token("DECLARED")),
                (
                    "fact_provenance",
                    c::token(provenance(f.provenance).as_str()),
                ),
                (
                    "fact_value",
                    c::object(vec![("kind", c::token(kind)), ("value", value)]),
                ),
            ]),
        )?;
    }
    if let Some(t) = &r.context.temporal {
        for series in &t.series {
            if object_ids.contains(series.id.as_str()) {
                return Err(EvaluationContractError("COMPILATION_IDENTITY_COLLISION"));
            }
            s.add(
                V::Series,
                series.id.as_str(),
                c::attrs(vec![
                    ("entity_presence", c::token("DECLARED")),
                    (
                        "series_provenance",
                        c::token(provenance(series.provenance).as_str()),
                    ),
                    (
                        "series_recurrence_rule",
                        canonical_graph_recurrence_v1(&series.rule)?,
                    ),
                    (
                        "series_state",
                        c::token(match series.state {
                            SeriesState::Existing { .. } => "EXISTING",
                            SeriesState::Prospective => "PROSPECTIVE",
                        }),
                    ),
                ]),
            )?;
        }
    }
    for o in &r.context.objects {
        let src = s.reference(V::Object, o.id.as_str())?;
        s.link(
            src,
            E::ObjectCalendar,
            V::Calendar,
            o.calendar_id.as_str(),
            None,
        )?;
        s.link(
            src,
            E::ObjectIntegration,
            V::Integration,
            o.integration_id.as_str(),
            None,
        )?;
        for id in &o.resource_ids {
            s.link(src, E::ObjectResource, V::Resource, id.as_str(), None)?;
        }
        for id in &o.time.evidence {
            s.link(
                src,
                E::EvidenceFactReference,
                V::Fact,
                id.as_str(),
                Some("OBJECT_TIME_EVIDENCE"),
            )?;
        }
    }
    for f in &r.context.facts {
        let src = s.reference(V::Fact, f.id.as_str())?;
        s.link(src, E::FactSubject, V::Object, f.object_id.as_str(), None)?;
    }
    if let Some(t) = &r.context.temporal {
        for series in &t.series {
            let src = s.reference(V::Series, series.id.as_str())?;
            for id in &series.evidence {
                s.link(
                    src,
                    E::EvidenceFactReference,
                    V::Fact,
                    id.as_str(),
                    Some("SERIES_EVIDENCE"),
                )?;
            }
        }
    }
    for id in &r.target_ids {
        s.reference(V::Object, id.as_str())?;
    }
    for id in r
        .scope
        .object_ids
        .iter()
        .flatten()
        .chain(r.scope.movable_object_ids.iter().flatten())
    {
        s.reference(V::Object, id.as_str())?;
    }
    for id in r.scope.calendar_ids.iter().flatten() {
        s.reference(V::Calendar, id.as_str())?;
    }
    for id in r.scope.integration_ids.iter().flatten() {
        s.reference(V::Integration, id.as_str())?;
    }
    for id in r.scope.resource_ids.iter().flatten() {
        s.reference(V::Resource, id.as_str())?;
    }
    s.facts(&r.duration.evidence)?;
    for constraint in &r.constraints {
        let src = s.reference(V::Object, constraint.object_id.as_str())?;
        if let HardConstraint::DependencyOrder(id) = &constraint.rule {
            s.link(src, E::DependsOnObject, V::Object, id.as_str(), None)?;
        }
        s.facts(&constraint.evidence)?;
    }
    for p in &r.preferences.preferences {
        s.facts(&p.evidence)?;
    }
    for g in &r.planning_capability.mutations {
        s.reference(V::Object, g.object_id.as_str())?;
        s.reference(V::Calendar, g.calendar_id.as_str())?;
        s.reference(V::Integration, g.integration_id.as_str())?;
    }
    CanonicalGraphSourceV1::new(s.vertices, s.edges)
}
