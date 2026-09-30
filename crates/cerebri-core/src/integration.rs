//! Application-independent, non-executing single-event suggestion contract.
//!
//! Admission narrows CPIR; it never repairs input or grants authority. Accepted requests
//! pass unchanged to the same planner used by the general-purpose facade.
use crate::{PlanningRequest, PlanningResult};
use cerebri_planner::{DeploymentMode, Operation, PlanningObjectKind};
use cerebri_types::{RequestId, Revision, SchemaVersion, TraceId};
use serde::{Deserialize, Serialize};

/// Independent of the software, REST route, CPIR and evaluation schema versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrationVersion {
    pub major: u16,
    pub minor: u16,
}

pub const INTEGRATION_V0_1: IntegrationVersion = IntegrationVersion { major: 0, minor: 1 };
pub const MAX_INTEGRATION_REQUEST_BYTES: usize = 256 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct IntegrationManifest {
    pub integration_version: IntegrationVersion,
    pub software_version: &'static str,
    pub profile: &'static str,
    pub cpir_schema_version: SchemaVersion,
    pub operations: [Operation; 1],
    pub deployment_modes: [DeploymentMode; 3],
    pub temporal_coverage_required: bool,
    pub execution_supported: bool,
    /// Raw JSON envelope limit, not an exhaustive statement of planner admission limits.
    pub max_request_bytes: usize,
}

pub fn describe() -> IntegrationManifest {
    IntegrationManifest {
        integration_version: INTEGRATION_V0_1,
        software_version: env!("CARGO_PKG_VERSION"),
        profile: "single_event_suggestion",
        cpir_schema_version: SchemaVersion::CPIR_0_2,
        operations: [Operation::FindSlot],
        deployment_modes: [
            DeploymentMode::Test,
            DeploymentMode::Shadow,
            DeploymentMode::Suggestion,
        ],
        temporal_coverage_required: true,
        execution_supported: false,
        max_request_bytes: MAX_INTEGRATION_REQUEST_BYTES,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SuggestionRequest {
    pub integration_version: IntegrationVersion,
    pub request: PlanningRequest,
}

/// Fixed codes contain no request content or parser diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionRejection {
    InvalidRequest,
    RequestTooLarge,
    UnsupportedIntegrationVersion,
    UnsupportedCpirVersion,
    UnsupportedOperation,
    MutationAuthorityForbidden,
    UnsupportedDeploymentMode,
    ProspectiveEventRequired,
    TemporalCoverageRequired,
    UncertainDurationForbidden,
}

#[derive(Debug, Serialize)]
pub struct SuggestionResponse {
    pub integration_version: IntegrationVersion,
    #[serde(flatten)]
    pub outcome: SuggestionOutcome,
}

#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum SuggestionOutcome {
    /// `planned` means the planner returned, not that a feasible candidate exists.
    Planned {
        request_id: RequestId,
        trace_id: TraceId,
        context_revision: Revision,
        result: Box<PlanningResult>,
    },
    Rejected {
        code: SuggestionRejection,
    },
}

impl SuggestionResponse {
    pub fn rejected(code: SuggestionRejection) -> Self {
        Self {
            integration_version: INTEGRATION_V0_1,
            outcome: SuggestionOutcome::Rejected { code },
        }
    }
}

fn admit(input: &SuggestionRequest) -> Result<(), SuggestionRejection> {
    use SuggestionRejection::*;
    if input.integration_version != INTEGRATION_V0_1 {
        return Err(UnsupportedIntegrationVersion);
    }
    let request = &input.request;
    if request.schema_version != SchemaVersion::CPIR_0_2 {
        return Err(UnsupportedCpirVersion);
    }
    if request.operation != Operation::FindSlot {
        return Err(UnsupportedOperation);
    }
    if request.scope.max_mutations != 0
        || request.policy.snapshot.mutation.max_mutations != 0
        || !request.policy.snapshot.mutation.allowed_actions.is_empty()
        || !request.planning_capability.mutations.is_empty()
    {
        return Err(MutationAuthorityForbidden);
    }
    if !matches!(
        request.policy.snapshot.mode,
        DeploymentMode::Test | DeploymentMode::Shadow | DeploymentMode::Suggestion
    ) {
        return Err(UnsupportedDeploymentMode);
    }
    if request.target_ids.len() != 1
        || !request
            .object(&request.target_ids[0])
            .is_some_and(|object| {
                object.revision.is_none() && object.kind == PlanningObjectKind::Event
            })
    {
        return Err(ProspectiveEventRequired);
    }
    if request.context.temporal.is_none() {
        return Err(TemporalCoverageRequired);
    }
    if request.policy.snapshot.allow_uncertain_duration {
        return Err(UncertainDurationForbidden);
    }
    Ok(())
}

/// Read/plan capabilities, facts, scope, knowledge and all solver limits remain core checks.
pub fn suggest(input: SuggestionRequest) -> SuggestionResponse {
    if let Err(code) = admit(&input) {
        return SuggestionResponse::rejected(code);
    }
    let request = input.request;
    SuggestionResponse {
        integration_version: INTEGRATION_V0_1,
        outcome: SuggestionOutcome::Planned {
            request_id: request.request_id.clone(),
            trace_id: request.trace_id.clone(),
            context_revision: request.context.revision,
            result: Box::new(crate::plan(request)),
        },
    }
}

/// Shared decoding boundary for transports. Invalid UTF-8/JSON never escapes as diagnostics.
pub fn suggest_from_slice(input: &[u8]) -> SuggestionResponse {
    if input.len() > MAX_INTEGRATION_REQUEST_BYTES {
        return SuggestionResponse::rejected(SuggestionRejection::RequestTooLarge);
    }
    match serde_json::from_slice(input) {
        Ok(request) => suggest(request),
        Err(_) => SuggestionResponse::rejected(SuggestionRejection::InvalidRequest),
    }
}
