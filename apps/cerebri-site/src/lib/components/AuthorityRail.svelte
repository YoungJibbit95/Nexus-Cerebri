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

<style>
  .authority-path{display:grid;grid-template-columns:auto minmax(18px,1fr) auto minmax(18px,1fr) auto minmax(18px,1fr) auto minmax(18px,1fr) auto;gap:8px;align-items:center;min-width:0}
  .authority-state{display:flex;gap:9px;align-items:center;min-width:0;opacity:.48}.authority-state.reached,.authority-state.current{opacity:1}
  .authority-state.current .authority-seal{border-color:rgba(32,216,255,.65);box-shadow:0 0 22px rgba(32,216,255,.18)}.authority-state.current strong{color:#eaf9ff}
  .authority-seal{position:relative;flex:0 0 34px;width:34px;height:34px;display:grid;place-items:center;border:1px solid rgba(94,151,218,.22);border-radius:50%;background:rgba(5,17,42,.86)}
  .authority-seal i{position:absolute;inset:5px;border:1px solid rgba(78,214,232,.18);transform:rotate(45deg)}.authority-seal b{position:relative;z-index:1;color:#77a7c8;font:800 9px var(--mono)}
  .authority-state.reached .authority-seal b,.authority-state.current .authority-seal b{color:var(--cyan)}
  .authority-state>span:last-child{display:grid;gap:2px}.authority-state strong{color:#8ca4bf;font-size:11px;white-space:nowrap}.authority-state small{color:#58708f;font:600 9px var(--mono);white-space:nowrap}
  .authority-connector{min-width:10px;height:1px;background:linear-gradient(90deg,rgba(83,142,211,.22),rgba(32,216,255,.18))}.authority-connector i{display:block;width:100%;height:100%}
  .compact{gap:5px}.compact .authority-seal{flex-basis:27px;width:27px;height:27px}.compact .authority-state strong{font-size:10px}.compact .authority-state small{display:none}
  @media(max-width:760px){.authority-path,.authority-path.compact{grid-template-columns:1fr;gap:5px}.authority-connector{width:1px;height:13px;margin-left:16px;background:linear-gradient(rgba(83,142,211,.22),rgba(32,216,255,.18))}.authority-state small,.compact .authority-state small{display:block;white-space:normal}}
  @media(prefers-reduced-motion:reduce){.authority-state{transition:none!important;animation:none!important}}
</style>