#[test]
fn provider_revision_race_begins_no_mutation_and_is_stale() {
    struct Race(InMemoryAdapter);
    impl IntegrationAdapter for Race {
        fn capabilities(&self) -> Vec<Capability> {
            self.0.capabilities()
        }
        fn preflight(&self, p: &ActionPlan) -> Result<ExecutionPreflight, AdapterError> {
            self.0.preflight(p)
        }
        fn apply(&mut self, a: &Action) -> Result<ProviderReceipt, AdapterError> {
            self.0
                .context
                .objects
                .iter_mut()
                .find(|o| o.id == a.target)
                .unwrap()
                .revision = Some(Revision(99));
            self.0.apply(a)
        }
    }
    let (plan, adapter, mut ledger) = setup();
    let mut adapter = Race(adapter);
    let result = execute(plan, &mut adapter, &mut ledger);
    assert_eq!(result.status, ExecutionStatus::Stale);
    assert!(adapter.0.applied.is_empty());
}
#[test]
fn indeterminate_provider_response_requires_reconciliation() {
    struct Unknown(InMemoryAdapter);
    impl IntegrationAdapter for Unknown {
        fn capabilities(&self) -> Vec<Capability> {
            self.0.capabilities()
        }
        fn preflight(&self, p: &ActionPlan) -> Result<ExecutionPreflight, AdapterError> {
            self.0.preflight(p)
        }
        fn apply(&mut self, _: &Action) -> Result<ProviderReceipt, AdapterError> {
            Err(AdapterError::Indeterminate)
        }
    }
    let (plan, adapter, mut ledger) = setup();
    let mut adapter = Unknown(adapter);
    let result = execute(plan, &mut adapter, &mut ledger);
    assert_eq!(result.status, ExecutionStatus::RecoveryRequired);
    assert_eq!(
        ledger.state(&result.plan_id),
        Some(OperationalState::Executing)
    );
}
#[path = "../../../tests/support/mod.rs"]
mod support;
use cerebri_integrations::*;
use cerebri_planner::*;
use cerebri_types::*;
use support::*;

fn setup() -> (AuthorizedActionPlan, InMemoryAdapter, InMemoryActionLedger) {
    let input = request();
    let plan = action_plan(&input);
    let auth = authorization(&input, &plan);
    (
        plan.authorize(&auth).unwrap(),
        InMemoryAdapter::new(input.context, auth),
        InMemoryActionLedger::default(),
    )
}

#[test]
fn provider_move_refreshes_snapshot_facts_for_subsequent_planning() {
    use cerebri_constraints::{Fact, FactValue};
    let mut input = request();
    input.operation = Operation::Move;
    input.context.objects.remove(0);
    let object = input.context.objects[0].clone();
    input.target_ids = vec![object.id.clone()];
    input.duration.value = FieldState::known(cerebri_temporal::Duration::seconds(3600).unwrap());
    input.planning_capability.mutations = vec![MutationGrant {
        object_id: object.id.clone(),
        calendar_id: object.calendar_id,
        integration_id: object.integration_id,
        kind: MutationKind::MoveEvent,
    }];
    input.context.facts.push(Fact {
        id: FactId::new("scheduled").unwrap(),
        object_id: object.id,
        value: FactValue::ScheduledTime(range("2026-10-01T09:00:00Z", "2026-10-01T10:00:00Z")),
        provenance: Provenance::IntegrationFact,
    });
    let proposed = ProposedPlan::from_placements(
        &input,
        vec![placement(
            "busy",
            "2026-10-01T10:00:00Z",
            "2026-10-01T11:00:00Z",
        )],
    );
    let plan = proposed
        .validate(&input.context)
        .unwrap()
        .into_action_plan()
        .unwrap();
    let auth = authorization(&input, &plan);
    let mut adapter = InMemoryAdapter::new(input.context.clone(), auth.clone());
    let result = execute(
        plan.authorize(&auth).unwrap(),
        &mut adapter,
        &mut InMemoryActionLedger::default(),
    );
    assert_eq!(result.status, ExecutionStatus::Executed);
    input.context = adapter.context;
    assert_eq!(validate_request(&input).state, ValidationState::Valid);
    assert_eq!(input.context.revision, Revision(2));
}
#[test]
fn authorized_plan_executes_and_replay_begins_no_new_mutation() {
    let (plan, mut adapter, mut ledger) = setup();
    let input = request();
    let duplicate = action_plan(&input)
        .authorize(&adapter.authorization)
        .unwrap();
    let result = execute(plan, &mut adapter, &mut ledger);
    assert_eq!(result.status, ExecutionStatus::Executed);
    assert_eq!(result.actions[0].status, ActionStatus::Succeeded);
    assert_eq!(
        ledger.state(&result.plan_id),
        Some(OperationalState::Executed)
    );
    assert_eq!(ledger.result(&result.plan_id), Some(&result));
    let replay = execute(duplicate, &mut adapter, &mut ledger);
    assert_eq!(
        replay.error,
        Some(ExecutionError::Ledger(LedgerError::Replay))
    );
    assert_eq!(adapter.applied.len(), 1);
}
#[test]
fn stale_authorized_plan_is_rejected_before_side_effect() {
    let (plan, mut adapter, mut ledger) = setup();
    adapter.context.revision.0 += 1;
    let result = execute(plan, &mut adapter, &mut ledger);
    assert_eq!(result.status, ExecutionStatus::Stale);
    assert!(adapter.applied.is_empty());
    assert!(
        result
            .actions
            .iter()
            .all(|a| a.status == ActionStatus::Skipped)
    );
}
#[test]
fn revoked_authorization_confirmation_policy_or_adapter_capability_blocks_execution() {
    for variant in 0..4 {
        let (plan, mut adapter, mut ledger) = setup();
        match variant {
            0 => adapter.authorization.execution_grants.clear(),
            1 => adapter.authorization.confirmed_plan_ids.clear(),
            2 => adapter.authorization.policy.policy_version.0 += 1,
            _ => adapter.supported.clear(),
        }
        let result = execute(plan, &mut adapter, &mut ledger);
        assert_eq!(result.status, ExecutionStatus::Rejected);
        assert!(adapter.applied.is_empty());
    }
}
#[test]
fn multi_action_partial_failure_preserves_order_and_skips_remaining_actions() {
    let mut input = request();
    let original = input.context.objects[0].clone();
    for id in ["second", "third"] {
        let mut object = original.clone();
        object.id = PlanningObjectId::new(id).unwrap();
        input.target_ids.push(object.id.clone());
        let mut grant = input.planning_capability.mutations[0].clone();
        grant.object_id = object.id.clone();
        input.planning_capability.mutations.push(grant);
        input.context.objects.push(object);
    }
    let proposed = ProposedPlan::from_placements(
        &input,
        vec![
            placement("new-event", "2026-10-01T10:00:00Z", "2026-10-01T10:30:00Z"),
            placement("second", "2026-10-01T10:30:00Z", "2026-10-01T11:00:00Z"),
            placement("third", "2026-10-01T11:00:00Z", "2026-10-01T11:30:00Z"),
        ],
    );
    let plan = proposed
        .validate(&input.context)
        .unwrap()
        .into_action_plan()
        .unwrap();
    let auth = authorization(&input, &plan);
    let mut adapter = InMemoryAdapter::new(input.context, auth.clone());
    adapter.fail_on = Some(PlanningObjectId::new("second").unwrap());
    let mut ledger = InMemoryActionLedger::default();
    let result = execute(plan.authorize(&auth).unwrap(), &mut adapter, &mut ledger);
    assert_eq!(result.status, ExecutionStatus::Partial);
    assert_eq!(
        result
            .actions
            .iter()
            .map(|a| a.status.clone())
            .collect::<Vec<_>>(),
        vec![
            ActionStatus::Succeeded,
            ActionStatus::Failed,
            ActionStatus::Skipped
        ]
    );
    assert_eq!(adapter.applied.len(), 1);
    assert_eq!(
        ledger.state(&result.plan_id),
        Some(OperationalState::Failed)
    );
}
#[test]
fn provider_checks_object_revision_atomically_and_deduplicates() {
    let input = request();
    let plan = action_plan(&input);
    let auth = authorization(&input, &plan);
    let mut adapter = InMemoryAdapter::new(input.context, auth);
    let action = &plan.actions()[0];
    let first = adapter.apply(action).unwrap();
    assert_eq!(adapter.apply(action).unwrap(), first);
    assert_eq!(adapter.applied.len(), 1);
    let mut changed = action.clone();
    changed.idempotency_key = IdempotencyKey::new("different-key").unwrap();
    assert_eq!(adapter.apply(&changed), Err(AdapterError::Stale));
}
#[test]
fn create_promotes_prospective_identity_to_existing_state() {
    let input = request();
    let plan = action_plan(&input);
    let auth = authorization(&input, &plan);
    let mut adapter = InMemoryAdapter::new(input.context, auth);
    let action = &plan.actions()[0];
    assert!(action.expected_revision.is_none());
    assert!(
        adapter
            .context
            .objects
            .iter()
            .find(|o| o.id == action.target)
            .unwrap()
            .revision
            .is_none()
    );
    let receipt = adapter.apply(action).unwrap();
    let object = adapter
        .context
        .objects
        .iter()
        .find(|o| o.id == action.target)
        .unwrap();
    assert_eq!(object.revision, Some(receipt.revision));
    assert_eq!(object.time.provenance, Provenance::IntegrationFact);
}
#[test]
fn mock_rejects_existing_create_and_prospective_move_update_delete() {
    let input = request();
    let plan = action_plan(&input);
    let base = &plan.actions()[0];
    let range = range("2026-10-01T10:00:00Z", "2026-10-01T10:30:00Z");
    for kind in [
        EventAction::MoveEvent { range },
        EventAction::UpdateEvent { range },
        EventAction::DeleteEvent,
        EventAction::CreateEvent { range },
    ] {
        let mut adapter = InMemoryAdapter::new(input.context.clone(), authorization(&input, &plan));
        let mut action = base.clone();
        action.action = kind;
        if matches!(action.action, EventAction::CreateEvent { .. }) {
            action.expected_revision = Some(Revision(7));
            adapter
                .context
                .objects
                .iter_mut()
                .find(|o| o.id == action.target)
                .unwrap()
                .revision = action.expected_revision;
        }
        let before = adapter.context.clone();
        assert_eq!(adapter.apply(&action), Err(AdapterError::Stale));
        assert_eq!(adapter.context, before);
        assert!(adapter.applied.is_empty());
    }
}
#[test]
fn unavailable_ledger_prevents_side_effects() {
    struct Unavailable;
    impl ActionLedger for Unavailable {
        fn claim(&mut self, _: &ActionPlan) -> Result<(), LedgerError> {
            Err(LedgerError::Unavailable)
        }
        fn finish(&mut self, _: &ExecutionResult) -> Result<(), LedgerError> {
            Err(LedgerError::Unavailable)
        }
    }
    let (plan, mut adapter, _) = setup();
    let result = execute(plan, &mut adapter, &mut Unavailable);
    assert_eq!(result.status, ExecutionStatus::Rejected);
    assert!(adapter.applied.is_empty());
}
#[test]
fn lost_completion_record_requires_recovery_and_retains_replay_claim() {
    struct LostFinish(InMemoryActionLedger);
    impl ActionLedger for LostFinish {
        fn claim(&mut self, p: &ActionPlan) -> Result<(), LedgerError> {
            self.0.claim(p)
        }
        fn finish(&mut self, _: &ExecutionResult) -> Result<(), LedgerError> {
            Err(LedgerError::Unavailable)
        }
    }
    let (plan, mut adapter, ledger) = setup();
    let input = request();
    let replay = action_plan(&input)
        .authorize(&adapter.authorization)
        .unwrap();
    let mut ledger = LostFinish(ledger);
    let result = execute(plan, &mut adapter, &mut ledger);
    assert_eq!(result.status, ExecutionStatus::RecoveryRequired);
    assert_eq!(result.actions[0].status, ActionStatus::Succeeded);
    assert_eq!(
        execute(replay, &mut adapter, &mut ledger).error,
        Some(ExecutionError::Ledger(LedgerError::Replay))
    );
    assert_eq!(adapter.applied.len(), 1);
}
