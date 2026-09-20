<script lang="ts">
  import Icon from './Icon.svelte';
  import { intervalStyle, known, time, date } from '../lib/presentation.ts';
  import type { PlanningRequest, PlanningResult } from '../lib/contracts.ts';
  let { request, result, selected, onselect }: { request: PlanningRequest | null; result: PlanningResult | null; selected: number; onselect: (index: number) => void } = $props();
  const horizon = $derived(result?.search_space.horizon ?? request!.scope.time_range);
  const ticks = $derived(Array.from({ length: 7 }, (_, i) => new Date(Date.parse(horizon.start) + (Date.parse(horizon.end) - Date.parse(horizon.start)) * i / 6).toISOString()));
  const busy = $derived(request?.context.objects.flatMap((object) => {
    const range = known(object.time);
    return range && ['EVENT', 'TASK'].includes(object.kind.kind) ? [{ id: object.id, range }] : [];
  }) ?? []);
  const candidates = $derived(result?.candidates.slice(0, 8) ?? []);
</script>

<section class="panel timeline-panel" aria-labelledby="timeline-title">
  <div class="panel-heading"><div><span class="eyebrow">TEMPORAL VIEW</span><h2 id="timeline-title">A window of possibilities</h2></div><span class="badge subtle"><Icon name="clock" size={13} /> UTC</span></div>
  <p class="panel-description">{date(horizon.start)} · Half-open intervals [start, end). Select a candidate to inspect it.</p>
  <div class="timeline-scroll"><div class="timeline">
    <div class="timeline-axis"><span>CONTEXT</span><div>{#each ticks as tick}<span>{time(tick)}</span>{/each}</div></div>
    {#if request}
      {#each busy as item}<div class="timeline-row"><span class="row-label" title={item.id}>{item.id}<small>Known busy</small></span><div class="track"><span class="time-block busy-block" style={intervalStyle(item.range, horizon)} title={`${item.id}: ${item.range.start} to ${item.range.end}`}>{time(item.range.start)} — {time(item.range.end)}</span></div></div>{/each}
    {:else}<p class="fine-print">Imported result: no matching source snapshot is attached.</p>{/if}
    <div class="timeline-divider"><span>RANKED CANDIDATES</span><span>{result ? `${result.candidates.length} feasible` : 'Awaiting the Rust planner'}</span></div>
    {#each candidates as candidate, index}
      <div class="timeline-row"><button class:chosen={selected === index} class="candidate-label" onclick={() => onselect(index)} aria-pressed={selected === index}><span class="candidate-index">{String(index + 1).padStart(2, '0')}</span>{index === 0 ? 'Best ranked' : `Candidate ${index + 1}`}</button><div class="track">{#each candidate.proposed.placements as placement}<button class="time-block candidate-block" class:selected={selected === index} style={intervalStyle(placement.range, horizon)} onclick={() => onselect(index)} aria-label={`Candidate ${index + 1}, ${time(placement.range.start)} to ${time(placement.range.end)} UTC, cost ${candidate.cost} seconds`} title={`${placement.object_id}: ${placement.range.start} to ${placement.range.end}`}>{time(placement.range.start)}</button>{/each}</div></div>
    {:else}<div class="empty-timeline"><Icon name="planner" size={28} /><strong>{result ? 'No feasible candidate returned' : 'Your next plan starts here'}</strong><p>{result ? 'Open Trace to inspect validation and rejection reasons.' : 'Run the synthetic request to reveal candidates and their tradeoffs.'}</p></div>{/each}
  </div></div>
  <div class="chart-legend"><span><i class="legend-swatch busy-swatch"></i>Known busy</span><span><i class="legend-swatch candidate-swatch"></i>Feasible proposal</span><span class="muted">First 8 candidates · all data available in JSON</span></div>
</section>
