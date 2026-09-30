//! Closed intrinsic graph registry. General CanonicalValue validity alone is insufficient.
use super::*;
use CanonicalValueKindV1 as K;

pub(crate) fn require(valid: bool) -> Result<(), EvaluationContractError> {
    if valid {
        Ok(())
    } else {
        Err(EvaluationContractError(
            "invalid canonical graph registry value",
        ))
    }
}
fn fields<'a>(
    v: &'a CanonicalValueV1,
    keys: &[&str],
) -> Result<&'a [CanonicalAttributeV1], EvaluationContractError> {
    if let K::Object { v } = v.kind() {
        exact(v, keys)?;
        Ok(v)
    } else {
        Err(EvaluationContractError("expected canonical OBJECT"))
    }
}
fn exact(attrs: &[CanonicalAttributeV1], keys: &[&str]) -> Result<(), EvaluationContractError> {
    require(attrs.len() == keys.len() && attrs.iter().zip(keys).all(|(a, k)| a.key == *k))
}
fn token(v: &CanonicalValueV1) -> Option<&str> {
    if let K::Token { v } = v.kind() {
        Some(v.as_str())
    } else {
        None
    }
}
fn is_token(v: &CanonicalValueV1, tokens: &[&str]) -> bool {
    token(v).is_some_and(|t| tokens.contains(&t))
}
fn none(v: &CanonicalValueV1) -> bool {
    matches!(v.kind(), K::None {})
}
fn range(v: &CanonicalValueV1) -> bool {
    serde_json::to_value(v)
        .ok()
        .and_then(|v| serde_json::from_value::<CanonicalTimeRangeValueV1>(v).ok())
        .is_some()
}
fn timezone(v: &CanonicalValueV1) -> bool {
    if let K::Text { v } = v.kind() {
        CanonicalTimezoneV1::try_from(v.clone()).is_ok()
    } else {
        false
    }
}
pub(crate) fn declared(attrs: &[CanonicalAttributeV1]) -> Result<bool, EvaluationContractError> {
    match attrs
        .first()
        .filter(|v| v.key == "entity_presence")
        .and_then(|v| token(&v.value))
    {
        Some("DECLARED") => Ok(true),
        Some("REFERENCE_ONLY") => Ok(false),
        _ => Err(EvaluationContractError("invalid entity presence")),
    }
}
pub(crate) fn validate_vertex(vertex: &GraphSourceVertexV1) -> Result<(), EvaluationContractError> {
    let attrs = &vertex.attributes;
    let keys: &[&str] = match vertex.vertex_type {
        GraphVertexTypeToken::Object => &[
            "entity_presence",
            "object_kind",
            "object_kind_details",
            "object_time_evidence",
            "object_timezone",
        ],
        GraphVertexTypeToken::Fact => &["entity_presence", "fact_provenance", "fact_value"],
        GraphVertexTypeToken::Series => &[
            "entity_presence",
            "series_provenance",
            "series_recurrence_rule",
            "series_state",
        ],
        _ => &["entity_presence"],
    };
    exact(attrs, keys)?;
    if !declared(attrs)? {
        return require(attrs[1..].iter().all(|v| none(&v.value)));
    }
    match vertex.vertex_type {
        GraphVertexTypeToken::Object => {
            require(is_token(&attrs[1].value, PlanningObjectKindToken::TOKENS))?;
            match token(&attrs[1].value) {
                Some("TASK") => {
                    let d = fields(
                        &attrs[2].value,
                        &[
                            "interruptible",
                            "maximum_chunk_count",
                            "minimum_chunk_seconds",
                            "preferred_chunk_seconds",
                            "splittable",
                        ],
                    )?;
                    require(
                        matches!(d[0].value.kind(), K::Bool { .. })
                            && matches!(d[4].value.kind(), K::Bool { .. })
                            && d[1..4]
                                .iter()
                                .all(|a| none(&a.value) || matches!(a.value.kind(), K::U64 { .. })),
                    )?;
                }
                Some("DEADLINE") => {
                    let d = fields(&attrs[2].value, &["deadline"])?;
                    require(matches!(d[0].value.kind(), K::Instant { .. }))?;
                }
                _ => require(none(&attrs[2].value))?,
            }
            let evidence = fields(
                &attrs[3].value,
                &["knowledge_state", "provenance", "values"],
            )?;
            require(
                is_token(&evidence[0].value, KnowledgeStateToken::TOKENS)
                    && is_token(&evidence[1].value, ProvenanceToken::TOKENS),
            )?;
            let K::List { v } = evidence[2].value.kind() else {
                return require(false);
            };
            require(v.iter().all(range))?;
            require(match token(&evidence[0].value) {
                Some("KNOWN" | "UNCERTAIN") => v.len() == 1,
                Some("AMBIGUOUS") => v.windows(2).all(|w| w[0] < w[1]),
                _ => v.is_empty(),
            })?;
            require(timezone(&attrs[4].value))?;
        }
        GraphVertexTypeToken::Fact => {
            require(is_token(&attrs[1].value, ProvenanceToken::TOKENS))?;
            let fact = fields(&attrs[2].value, &["kind", "value"])?;
            require(match token(&fact[0].value) {
                Some("SCHEDULED_TIME" | "AVAILABILITY") => range(&fact[1].value),
                Some("DEADLINE") => matches!(fact[1].value.kind(), K::Instant { .. }),
                Some("EXTERNAL_LOCK") => none(&fact[1].value),
                _ => false,
            })?;
        }
        GraphVertexTypeToken::Series => {
            require(
                is_token(&attrs[1].value, ProvenanceToken::TOKENS)
                    && is_token(&attrs[3].value, SeriesStateToken::TOKENS),
            )?;
            let r = fields(
                &attrs[2].value,
                &[
                    "count",
                    "duration_seconds",
                    "fold_policy",
                    "frequency",
                    "gap_policy",
                    "interval",
                    "local_time",
                    "start_date",
                    "timezone",
                    "until",
                    "weekdays",
                ],
            )?;
            require(
                none(&r[0].value)
                    || matches!(r[0].value.kind(),K::U64 {v} if (1..=u64::from(u32::MAX)).contains(&v.value())),
            )?;
            require(
                matches!(r[1].value.kind(),K::U64 {v} if v.value()>0)
                    && is_token(&r[2].value, FoldPolicyToken::TOKENS)
                    && is_token(&r[3].value, RecurrenceFrequencyToken::TOKENS)
                    && is_token(&r[4].value, GapPolicyToken::TOKENS)
                    && matches!(r[5].value.kind(),K::U64 {v} if (1..=65535).contains(&v.value()))
                    && matches!(r[6].value.kind(), K::LocalTime { .. })
                    && matches!(r[7].value.kind(), K::Date { .. })
                    && timezone(&r[8].value)
                    && (none(&r[9].value) || matches!(r[9].value.kind(), K::Date { .. })),
            )?;
            let K::Set { v } = r[10].value.kind() else {
                return require(false);
            };
            require(
                v.iter().all(|d| is_token(d, WeekdayToken::TOKENS))
                    && (v.is_empty() == (token(&r[3].value) == Some("DAILY"))),
            )?;
        }
        _ => {}
    }
    Ok(())
}
pub(crate) fn validate_edge(
    vertices: &[GraphSourceVertexV1],
    e: &GraphSourceEdgeV1,
) -> Result<(), EvaluationContractError> {
    require(e.src < vertices.len() && e.dst < vertices.len() && e.multiplicity > 0)?;
    use GraphEdgeTypeToken as E;
    use GraphVertexTypeToken as V;
    let (src, dst) = (vertices[e.src].vertex_type, vertices[e.dst].vertex_type);
    if e.edge_type == E::EvidenceFactReference {
        exact(&e.payload, &["evidence_role"])?;
        return require(
            dst == V::Fact
                && match src {
                    V::Object => token(&e.payload[0].value) == Some("OBJECT_TIME_EVIDENCE"),
                    V::Series => token(&e.payload[0].value) == Some("SERIES_EVIDENCE"),
                    _ => false,
                },
        );
    }
    require(
        e.payload.is_empty()
            && match e.edge_type {
                E::DependsOnObject => src == V::Object && dst == V::Object,
                E::FactSubject => src == V::Fact && dst == V::Object,
                E::ObjectCalendar => src == V::Object && dst == V::Calendar,
                E::ObjectIntegration => src == V::Object && dst == V::Integration,
                E::ObjectResource => src == V::Object && dst == V::Resource,
                E::EvidenceFactReference => unreachable!(),
            },
    )
}

pub(crate) fn reference_attributes(kind: GraphVertexTypeToken) -> Vec<CanonicalAttributeV1> {
    let mut attrs = vec![("entity_presence", cv::token("REFERENCE_ONLY"))];
    let keys: &[&str] = match kind {
        GraphVertexTypeToken::Object => &[
            "object_kind",
            "object_kind_details",
            "object_time_evidence",
            "object_timezone",
        ],
        GraphVertexTypeToken::Fact => &["fact_provenance", "fact_value"],
        GraphVertexTypeToken::Series => &[
            "series_provenance",
            "series_recurrence_rule",
            "series_state",
        ],
        GraphVertexTypeToken::Calendar
        | GraphVertexTypeToken::Integration
        | GraphVertexTypeToken::Resource => &[],
    };
    attrs.extend(keys.iter().map(|key| (*key, cv::none())));
    cv::attrs(attrs)
}

// Architecture-owned intrinsic fields; all dynamic values are typed before use.
pub(crate) mod cv {
    use super::*;
    pub fn value(kind: K) -> CanonicalValueV1 {
        CanonicalValueV1::new(kind).expect("architecture-owned canonical fields")
    }
    pub fn none() -> CanonicalValueV1 {
        value(K::None {})
    }
    pub fn token(s: &str) -> CanonicalValueV1 {
        value(K::Token {
            v: CanonicalTokenV1::new(s).expect("architecture token"),
        })
    }
    pub fn text(s: impl Into<String>) -> CanonicalValueV1 {
        value(K::Text { v: s.into() })
    }
    pub fn unsigned(v: u64) -> CanonicalValueV1 {
        value(K::U64 {
            v: CanonicalU64Decimal::new(v).expect("u64"),
        })
    }
    pub fn set(v: Vec<CanonicalValueV1>) -> CanonicalValueV1 {
        value(K::Set { v })
    }
    pub fn attrs(entries: Vec<(&str, CanonicalValueV1)>) -> Vec<CanonicalAttributeV1> {
        let mut attrs: Vec<_> = entries
            .into_iter()
            .map(|(key, value)| CanonicalAttributeV1 {
                key: key.into(),
                value,
            })
            .collect();
        attrs.sort_by(|a, b| a.key.cmp(&b.key));
        attrs
    }
    pub fn object(v: Vec<(&str, CanonicalValueV1)>) -> CanonicalValueV1 {
        value(K::Object { v: attrs(v) })
    }
}
