<script lang="ts">
  import { time } from '../lib/presentation.ts';
  import type { PlanningResult } from '../lib/contracts.ts';
  let { result, selected, onselect }: { result: PlanningResult | null; selected: number; onselect: (index: number) => void } = $props();
  const candidates = $derived(result?.candidates.slice(0, 12) ?? []);
  const ceiling = $derived(Math.max(1, ...candidates.map((candidate) => candidate.cost)));
</script>
<section class="panel score-panel" aria-labelledby="score-title">
  <div class="panel-heading"><div><span class="eyebrow">OBJECTIVE / LOWER IS BETTER</span><h2 id="score-title">Candidate cost</h2></div><span class="badge subtle">seconds</span></div>
  <p class="panel-description">Distance from the effective preferred start. Zero-cost ties use deterministic ordering.</p>
  {#if candidates.length}<div class="score-chart"><div class="chart-y"><span>{ceiling === 1 && candidates.every((c) => c.cost === 0) ? '0' : ceiling}</span><span>0 s</span></div><div class="bars">{#each candidates as candidate, index}<button class="bar-column" class:active={index === selected} onclick={() => onselect(index)} aria-pressed={index === selected} aria-label={`Candidate ${index + 1}: ${candidate.cost} seconds, ${time(candidate.start)} UTC`}><span class="bar-value">{candidate.cost}</span><span class="bar-space"><span class="bar-fill" style={`height:${Math.max(2, candidate.cost / ceiling * 100)}%`}></span></span><span class="bar-tick">{String(index + 1).padStart(2, '0')}</span></button>{/each}</div></div>{:else}<div class="chart-empty"><span class="chart-empty-line"></span><p>Cost values appear after planning.</p><small>No simulated scores</small></div>{/if}
  <p class="fine-print">Candidate rank → · Showing the first 12 candidates. Select a bar to reveal the score components.</p>
</section>
