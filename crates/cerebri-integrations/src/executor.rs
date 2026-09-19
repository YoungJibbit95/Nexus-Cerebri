use cerebri_planner::{
    Action, ActionPlan, AuthorizationContext, AuthorizedActionPlan, ContextSnapshot, MutationKind,
    PlanError,
};
use cerebri_types::{ActionId, IdempotencyKey, PlanId, Revision};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Capability {
    ReadEvents,
    ReadAvailability,
    CreateEvent,
    UpdateEvent,
    MoveEvent,
    DeleteEvent,
    ReadTasks,
    UpdateTask,
    ReadResources,
    ReadParticipants,
}
impl From<MutationKind> for Capability {
    fn from(kind: MutationKind) -> Self {
        match kind {
            MutationKind::CreateEvent => Self::CreateEvent,
            MutationKind::MoveEvent => Self::MoveEvent,
            MutationKind::UpdateEvent => Self::UpdateEvent,
            MutationKind::DeleteEvent => Self::DeleteEvent,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
pub enum AdapterError {
    #[error("provider unavailable")]
    Unavailable,
    #[error("provider revision changed")]
    Stale,
    #[error("provider capability unsupported")]
    UnsupportedCapability,
    #[error("provider mutation denied")]
    Denied,
    #[error("provider returned an uncertain outcome; reconcile before retry")]
    Indeterminate,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderReceipt {
    pub reference: String,
    pub revision: Revision,
}
pub struct ExecutionPreflight {
    pub context: ContextSnapshot,
    pub authorization: AuthorizationContext,
}

/// Implementations map provider schemas here. apply must atomically enforce expected revision
/// and provider idempotency where supported; uncertain writes must return Indeterminate.
pub trait IntegrationAdapter {
    fn capabilities(&self) -> Vec<Capability>;
    fn preflight(&self, plan: &ActionPlan) -> Result<ExecutionPreflight, AdapterError>;
    fn apply(&mut self, action: &Action) -> Result<ProviderReceipt, AdapterError>;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationalState {
    Proposed,
    Approved,
    Executing,
    Executed,
    Failed,
    Stale,
    Rejected,
    Cancelled,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
pub enum LedgerError {
    #[error("plan or idempotency key already claimed")]
    Replay,
    #[error("ledger unavailable")]
    Unavailable,
    #[error("invalid ledger state transition")]
    InvalidTransition,
}
/// Execution-layer port. claim atomically reserves the plan AND every action key.
/// Claims survive failures: never erase keys to retry. Durable implementations must persist
/// in-flight state before writes and reconcile after crashes/uncertain outcomes.
pub trait ActionLedger {
    fn claim(&mut self, plan: &ActionPlan) -> Result<(), LedgerError>;
    fn finish(&mut self, result: &ExecutionResult) -> Result<(), LedgerError>;
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionStatus {
    Succeeded,
    Failed,
    Skipped,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionExecutionResult {
    pub action_id: ActionId,
    pub idempotency_key: IdempotencyKey,
    pub status: ActionStatus,
    pub provider: Option<ProviderReceipt>,
    pub error: Option<AdapterError>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Executed,
    Partial,
    Failed,
    Stale,
    Rejected,
    RecoveryRequired,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionError {
    Preconditions(PlanError),
    Adapter(AdapterError),
    Ledger(LedgerError),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub plan_id: PlanId,
    pub status: ExecutionStatus,
    pub actions: Vec<ActionExecutionResult>,
    pub error: Option<ExecutionError>,
}

/// The executor's single entry accepts only AuthorizedActionPlan.
///
/// ```compile_fail
/// use cerebri_integrations::{execute, InMemoryAdapter, InMemoryActionLedger};
/// use cerebri_planner::ProposedPlan;
/// fn rejected(plan: ProposedPlan, adapter: &mut InMemoryAdapter, ledger: &mut InMemoryActionLedger) {
///     execute(plan, adapter, ledger);
/// }
/// ```
/// ```compile_fail
/// use cerebri_integrations::{execute, InMemoryAdapter, InMemoryActionLedger};
/// use cerebri_planner::ValidatedPlan;
/// fn rejected(plan: ValidatedPlan, adapter: &mut InMemoryAdapter, ledger: &mut InMemoryActionLedger) {
///     execute(plan, adapter, ledger);
/// }
/// ```
/// ```compile_fail
/// use cerebri_integrations::{execute, InMemoryAdapter, InMemoryActionLedger};
/// use cerebri_planner::ActionPlan;
/// fn rejected(plan: ActionPlan, adapter: &mut InMemoryAdapter, ledger: &mut InMemoryActionLedger) {
///     execute(plan, adapter, ledger);
/// }
/// ```
pub fn execute<A: IntegrationAdapter, L: ActionLedger>(
    authorized: AuthorizedActionPlan,
    adapter: &mut A,
    ledger: &mut L,
) -> ExecutionResult {
    let plan = authorized.plan();
    let mut result = ExecutionResult {
        plan_id: plan.id().clone(),
        status: ExecutionStatus::Rejected,
        actions: plan
            .actions()
            .iter()
            .map(|a| ActionExecutionResult {
                action_id: a.id.clone(),
                idempotency_key: a.idempotency_key.clone(),
                status: ActionStatus::Skipped,
                provider: None,
                error: None,
            })
            .collect(),
        error: None,
    };
    if let Err(error) = ledger.claim(plan) {
        result.error = Some(ExecutionError::Ledger(error));
        return result;
    }
    // Read fresh inputs only after acquiring replay protection, immediately before writes.
    let check = adapter
        .preflight(plan)
        .map_err(ExecutionError::Adapter)
        .and_then(|current| {
            authorized
                .recheck(&current.context, &current.authorization)
                .map_err(ExecutionError::Preconditions)?;
            let capabilities = adapter.capabilities();
            if plan
                .actions()
                .iter()
                .any(|a| !capabilities.contains(&a.action.kind().into()))
            {
                return Err(ExecutionError::Adapter(AdapterError::UnsupportedCapability));
            }
            Ok(())
        });
    if let Err(error) = check {
        if matches!(
            error,
            ExecutionError::Preconditions(PlanError::Stale)
                | ExecutionError::Adapter(AdapterError::Stale)
        ) {
            result.status = ExecutionStatus::Stale;
        }
        result.error = Some(error);
    } else {
        result.status = ExecutionStatus::Executed;
        for (index, action) in plan.actions().iter().enumerate() {
            match adapter.apply(action) {
                Ok(receipt) => {
                    result.actions[index].status = ActionStatus::Succeeded;
                    result.actions[index].provider = Some(receipt);
                }
                Err(error) => {
                    result.actions[index].status = ActionStatus::Failed;
                    result.actions[index].error = Some(error.clone());
                    result.status = if error == AdapterError::Indeterminate {
                        ExecutionStatus::RecoveryRequired
                    } else if index > 0 {
                        ExecutionStatus::Partial
                    } else if error == AdapterError::Stale {
                        ExecutionStatus::Stale
                    } else {
                        ExecutionStatus::Failed
                    };
                    result.error = Some(ExecutionError::Adapter(error));
                    break;
                }
            }
        }
    }
    if let Err(error) = ledger.finish(&result) {
        result.status = ExecutionStatus::RecoveryRequired;
        result.error = Some(ExecutionError::Ledger(error));
    }
    result
}
