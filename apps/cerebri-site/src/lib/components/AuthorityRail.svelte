<script lang="ts">
  type AuthorityStage = 'proposal' | 'validated' | 'action' | 'authorized' | 'executed';

  let { currentStage = null, compact = false, label = 'Planning and execution authority states' }: {
    currentStage?: AuthorityStage | null; compact?: boolean; label?: string;
  } = $props();

  const stages: Array<{ key: AuthorityStage; short: string; technical: string }> = [
    { key: 'proposal', short: 'Proposed', technical: 'ProposedPlan' },
    { key: 'validated', short: 'Validated', technical: 'ValidatedPlan' },
    { key: 'action', short: 'Action plan', technical: 'ActionPlan' },
    { key: 'authorized', short: 'Authorized', technical: 'AuthorizedActionPlan' },
    { key: 'executed', short: 'Executed', technical: 'ExecutionResult' }
  ];
  const currentIndex = $derived(currentStage ? stages.findIndex((stage) => stage.key === currentStage) : -1);
</script>

<div class:compact class="authority-path" aria-label={label}>
  {#each stages as stage, index}
    <div
      class:current={stage.key === currentStage}
      class:reached={currentIndex >= 0 && index <= currentIndex}
      class="authority-state"
      data-authority-stage={stage.key}
    >
      <span class="authority-seal" aria-hidden="true"><i></i><b>{String(index + 1).padStart(2, '0')}</b></span>
      <span><strong>{stage.short}</strong><small>{stage.technical}</small></span>
    </div>
    {#if index < stages.length - 1}<span class="authority-connector" aria-hidden="true"><i></i></span>{/if}
  {/each}
</div>
