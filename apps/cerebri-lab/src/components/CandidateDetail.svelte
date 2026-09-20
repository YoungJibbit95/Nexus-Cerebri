<script lang="ts">
  import Icon from './Icon.svelte';
  import JsonPanel from './JsonPanel.svelte';
  import { reasonLabel, time } from '../lib/presentation.ts';
  import type { RankedCandidate, ExplanationMode } from '../lib/contracts.ts';
  let { candidate, rank, mode }: { candidate: RankedCandidate | undefined; rank: number; mode: ExplanationMode } = $props();
</script>
<section class="panel detail-panel" aria-labelledby="candidate-title">
  <div class="panel-heading"><div><span class="eyebrow">INSPECT / {mode.toUpperCase()}</span><h2 id="candidate-title">{candidate ? `Candidate ${String(rank).padStart(2, '0')}` : 'Inside a proposal'}</h2></div><Icon name="semantics" /></div>
  {#if candidate}<div class="candidate-time">{time(candidate.start)}<span>UTC</span></div><div class="detail-metrics"><div><strong>{candidate.cost}<small> s</small></strong><span>Objective cost</span></div><div><strong>{candidate.mutation_count}</strong><span>Proposed mutations</span></div></div><ul class="reason-list">{#each candidate.explanation as component}<li><Icon name="check" size={15} /><span>{reasonLabel(component.reason)}</span><strong>{component.cost} s</strong></li>{/each}</ul><p class="fine-print">This is a proposal. Request validation and placement checks do not authorize execution.</p>{#if mode !== 'Simple'}<JsonPanel title="ProposedPlan + score" value={candidate} open={mode === 'Research'} />{/if}
  {:else}<div class="detail-placeholder"><Icon name="code" size={30} /><p>See the evidence behind every placement.</p><small>Costs, provenance and structured reasons come directly from the core.</small></div>{/if}
</section>
