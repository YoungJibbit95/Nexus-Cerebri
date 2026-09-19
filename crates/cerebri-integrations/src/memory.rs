use crate::*;
use cerebri_constraints::FactValue;
use cerebri_planner::{Action, ActionPlan, AuthorizationContext, ContextSnapshot, EventAction};
use cerebri_types::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct InMemoryActionLedger {
    records: BTreeMap<PlanId, (OperationalState, Option<ExecutionResult>)>,
    keys: BTreeSet<IdempotencyKey>,
}
impl InMemoryActionLedger {
    pub fn state(&self, id: &PlanId) -> Option<OperationalState> {
        self.records.get(id).map(|v| v.0)
    }
    pub fn result(&self, id: &PlanId) -> Option<&ExecutionResult> {
        self.records.get(id).and_then(|v| v.1.as_ref())
    }
}
impl ActionLedger for InMemoryActionLedger {
    fn claim(&mut self, plan: &ActionPlan) -> Result<(), LedgerError> {
        let keys: BTreeSet<_> = plan
            .actions()
            .iter()
            .map(|a| a.idempotency_key.clone())
            .collect();
        if self.records.contains_key(plan.id())
            || keys.len() != plan.actions().len()
            || keys.iter().any(|key| self.keys.contains(key))
        {
            return Err(LedgerError::Replay);
        }
        self.keys.extend(keys);
        self.records
            .insert(plan.id().clone(), (OperationalState::Executing, None));
        Ok(())
    }
    fn finish(&mut self, result: &ExecutionResult) -> Result<(), LedgerError> {
        let record = self
            .records
            .get_mut(&result.plan_id)
            .ok_or(LedgerError::InvalidTransition)?;
        if record.0 != OperationalState::Executing || record.1.is_some() {
            return Err(LedgerError::InvalidTransition);
        }
        record.0 = match result.status {
            ExecutionStatus::Executed => OperationalState::Executed,
            ExecutionStatus::Stale => OperationalState::Stale,
            ExecutionStatus::Rejected => OperationalState::Rejected,
            ExecutionStatus::RecoveryRequired => OperationalState::Executing,
            _ => OperationalState::Failed,
        };
        record.1 = Some(result.clone());
        Ok(())
    }
}

/// Explicit test fixture. State/configuration is intentionally inspectable for fault injection.
pub struct InMemoryAdapter {
    pub context: ContextSnapshot,
    pub authorization: AuthorizationContext,
    pub supported: Vec<Capability>,
    pub fail_on: Option<PlanningObjectId>,
    pub applied: Vec<ActionId>,
    receipts: BTreeMap<IdempotencyKey, ProviderReceipt>,
}
impl InMemoryAdapter {
    pub fn new(context: ContextSnapshot, authorization: AuthorizationContext) -> Self {
        Self {
            context,
            authorization,
            supported: vec![
                Capability::ReadEvents,
                Capability::ReadAvailability,
                Capability::CreateEvent,
                Capability::MoveEvent,
                Capability::UpdateEvent,
                Capability::DeleteEvent,
            ],
            fail_on: None,
            applied: vec![],
            receipts: BTreeMap::new(),
        }
    }
}
impl IntegrationAdapter for InMemoryAdapter {
    fn capabilities(&self) -> Vec<Capability> {
        self.supported.clone()
    }
    fn preflight(&self, _: &ActionPlan) -> Result<ExecutionPreflight, AdapterError> {
        Ok(ExecutionPreflight {
            context: self.context.clone(),
            authorization: self.authorization.clone(),
        })
    }
    fn apply(&mut self, action: &Action) -> Result<ProviderReceipt, AdapterError> {
        if let Some(receipt) = self.receipts.get(&action.idempotency_key) {
            return Ok(receipt.clone());
        }
        if !self.supported.contains(&action.action.kind().into()) {
            return Err(AdapterError::UnsupportedCapability);
        }
        if self.fail_on.as_ref() == Some(&action.target) {
            return Err(AdapterError::Unavailable);
        }
        let object = self
            .context
            .objects
            .iter_mut()
            .find(|o| o.id == action.target)
            .ok_or(AdapterError::Stale)?;
        if object.revision != action.expected_revision
            || object.calendar_id != action.calendar_id
            || object.integration_id != action.integration_id
        {
            return Err(AdapterError::Stale);
        }
        let next = object
            .revision
            .map_or(Some(1), |r| r.0.checked_add(1))
            .ok_or(AdapterError::Stale)?;
        let next_context = self
            .context
            .revision
            .0
            .checked_add(1)
            .ok_or(AdapterError::Stale)?;
        match action.action {
            EventAction::CreateEvent { range }
            | EventAction::MoveEvent { range }
            | EventAction::UpdateEvent { range } => {
                object.time.value = FieldState::known(range);
                object.time.provenance = Provenance::IntegrationFact;
                object.revision = Some(Revision(next));
                for fact in self
                    .context
                    .facts
                    .iter_mut()
                    .filter(|f| f.object_id == action.target)
                {
                    if matches!(fact.value, FactValue::ScheduledTime(_)) {
                        fact.value = FactValue::ScheduledTime(range);
                        fact.provenance = Provenance::IntegrationFact;
                    }
                }
            }
            EventAction::DeleteEvent => {
                self.context.objects.retain(|o| o.id != action.target);
                self.context.facts.retain(|f| f.object_id != action.target);
            }
        }
        self.context.revision = Revision(next_context);
        self.authorization.source_revision = self.context.revision;
        self.applied.push(action.id.clone());
        let receipt = ProviderReceipt {
            reference: format!("mock-{}", action.id.as_str()),
            revision: Revision(next),
        };
        self.receipts
            .insert(action.idempotency_key.clone(), receipt.clone());
        Ok(receipt)
    }
}
