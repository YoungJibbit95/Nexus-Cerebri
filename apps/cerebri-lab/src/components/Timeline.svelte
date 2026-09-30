<script lang="ts">
  import { intervalStyle, known, time, date, rejectionLabel, readable } from '../lib/presentation.ts';
  import { blockerIds, visibleCandidates } from '../lib/planner-presentation.ts';
  import SemanticLegend from './SemanticLegend.svelte';
  import JsonPanel from './JsonPanel.svelte';
  import type { ExplanationMode, PlanningRequest, PlanningResult } from '../lib/contracts.ts';
  let { request, result, selected, onselect, mode, stage = 5 }: {
    request: PlanningRequest | null; result: PlanningResult | null; selected: number;
    onselect: (index: number) => void; mode: ExplanationMode; stage?: number;
  } = $props();
  let rejected = $state<number | null>(null);
  const horizon = $derived(result?.search_space.horizon ?? request!.scope.time_range);
  const ticks = $derived(Array.from({ length: 7 }, (_, i) => new Date(Date.parse(horizon.start) + (Date.parse(horizon.end) - Date.parse(horizon.start)) * i / 6).toISOString()));
  const busy = $derived(request?.context.objects.flatMap((object) => {
    const range = known(object.time);
    return range && ['EVENT', 'TASK'].includes(object.kind.kind) ? [{ id: object.id, range, provenance: object.time.provenance }] : [];
  }) ?? []);
  const candidates = $derived(visibleCandidates(result, selected));
  const blockers = $derived(blockerIds(result, rejected));
  const rejection = $derived(rejected === null ? undefined : result?.conflicts.rejections[rejected]);
  const duration = $derived(request ? known(request.duration) : undefined);
  $effect(() => { void result; rejected = null; });
  function position(value: string) {
    return `left:${Math.max(0, Math.min(100, (Date.parse(value) - Date.parse(horizon.start)) / (Date.parse(horizon.end) - Date.parse(horizon.start)) * 100))}%`;
  }
  function inHorizon(value: string) {
    return Date.parse(value) >= Date.parse(horizon.start) && Date.parse(value) <= Date.parse(horizon.end);
  }
</script>
<section class="panel time-field" aria-labelledby="timeline-title" data-selected-candidate={selected + 1}>
  <div class="panel-heading"><div><span class="eyebrow">SCOPE / FACTS / POSSIBILITIES</span><h2 id="timeline-title">The planning field</h2></div><span class="badge subtle">UTC</span></div>
  <p class="panel-description">{date(horizon.start)}{#if date(horizon.start) !== date(horizon.end)} → {date(horizon.end)}{/if} · {time(horizon.start)}–{time(horizon.end)} UTC. {mode === 'Simple' ? 'Every block keeps its real position in time.' : 'Half-open intervals [start, end); touching endpoints do not overlap.'}</p>
  <div class="time-field-scroll"><div class="time-field-canvas">
    <div class="timeline-axis"><span>HORIZON</span><div>{#each ticks as tick}<span>{time(tick)}</span>{/each}</div></div>
    {#if duration !== undefined}<div class="timeline-row"><span class="row-label">Requested duration</span><div class="track duration-track"><span class="time-block duration-block" style={intervalStyle({ start: horizon.start, end: new Date(Date.parse(horizon.start) + duration * 1000).toISOString() }, horizon)}>{duration / 60} min</span></div></div>{/if}
    {#if request}
      {#each request.preferences.preferences.slice(0, 8) as preference}<div class="timeline-row preference-row"><span class="row-label">◇ Supplied preference{#if mode !== 'Simple'}<small>{readable(preference.source)}</small>{/if}</span><div class="track">{#if inHorizon(preference.preferred_start)}<span class="preference-marker" style={position(preference.preferred_start)} title={preference.preferred_start}>◇<small>{time(preference.preferred_start)}</small></span>{:else}<span class="out-of-scope-preference">◇ {date(preference.preferred_start)} · {time(preference.preferred_start)} UTC · outside the horizon</span>{/if}</div></div>{/each}
      {#each busy.slice(0, 8) as item, index}<div class="timeline-row" class:blocking={blockers.includes(item.id)}><span class="row-label">{mode === 'Simple' ? `▰ Occupied ${index + 1}` : item.id}<small>{mode === 'Simple' ? 'Known fact' : readable(item.provenance)}</small></span><div class="track"><span class="time-block busy-block" style={intervalStyle(item.range, horizon)} title={item.id + ': [' + item.range.start + ', ' + item.range.end + ')'}>{time(item.range.start)}–{time(item.range.end)}</span></div></div>{/each}
      {#each busy.slice(8).filter((item) => blockers.includes(item.id)).slice(0, 8) as item}<div class="timeline-row blocking"><span class="row-label">Blocking fact{#if mode !== 'Simple'}<small>{item.id}</small>{/if}</span><div class="track"><span class="time-block busy-block" style={intervalStyle(item.range, horizon)}>{time(item.range.start)}–{time(item.range.end)}</span></div></div>{/each}
    {:else}<p class="fine-print">Imported result: no matching source request is attached. Request facts and preferences cannot be reconstructed.</p>{/if}
    {#each result?.compilation?.occurrences.slice(0, 8) ?? [] as item, index}<div class="timeline-row" class:blocking={blockers.includes(item.id)}><span class="row-label">{mode === 'Simple' ? '▥ Occurrence ' + (index + 1) : item.series_id}<small>Materialized recurrence</small></span><div class="track"><span class="time-block recurrence-block" style={intervalStyle(item.occurrence.visible_range, horizon)} title={item.id + ': ' + item.occurrence.range.start + ' to ' + item.occurrence.range.end}>{time(item.occurrence.range.start)}–{time(item.occurrence.range.end)}</span></div></div>{/each}
    {#each result?.compilation?.occurrences.slice(8).filter((item) => blockers.includes(item.id)).slice(0, 8) ?? [] as item}<div class="timeline-row blocking"><span class="row-label">Blocking occurrence{#if mode !== 'Simple'}<small>{item.series_id}</small>{/if}</span><div class="track"><span class="time-block recurrence-block" style={intervalStyle(item.occurrence.visible_range, horizon)}>{time(item.occurrence.range.start)}–{time(item.occurrence.range.end)}</span></div></div>{/each}
    {#if result?.compilation}
      <div class="timeline-row"><span class="row-label">Coverage<small>{result.compilation.availability.coverage}</small></span><div class="track">{#each result.compilation.availability.unknown.slice(0, 32) as range}<span class="time-block unknown-block" style={intervalStyle(range, horizon)} title={'Unknown: ' + range.start + ' to ' + range.end}>? Unknown</span>{/each}{#each result.compilation.availability.free.slice(0, 32) as range}<span class="time-block free-block" style={intervalStyle(range, horizon)} title={'Free: ' + range.start + ' to ' + range.end}>Free</span>{/each}</div></div>
    {/if}
    <div class="timeline-divider"><span>RETURNED POSITIONS</span><span>{result ? result.search_space.evaluated + ' evaluated' : 'Awaiting completed run'}</span></div>
    {#if result}
      <div class="timeline-row"><span class="row-label">Constraint filter<small>{result.conflicts.rejections.length} rejected</small></span><div class="track rejection-track" class:filter-stage={stage === 2}>{#each result.conflicts.rejections.slice(0, 48) as item, index}<button class="rejection-marker" class:active={rejected === index} style={position(item.start)} aria-pressed={rejected === index} aria-label={'Rejected position ' + (index + 1) + ', ' + time(item.start) + ' UTC'} onclick={() => rejected = index} title={item.reasons.map(rejectionLabel).join('; ')}>×</button>{/each}</div></div>
      {#if rejected !== null && rejected >= 48 && rejection}<div class="timeline-row"><span class="row-label">Selected rejection</span><div class="track"><span class="rejection-marker active" style={position(rejection.start)}>×</span></div></div>{/if}
    {/if}
    {#each candidates as { candidate, index }}
      <div class="timeline-row candidate-row" class:chosen={selected === index}><button class="candidate-label" aria-pressed={selected === index} onclick={() => onselect(index)}><span class="candidate-index">{index + 1}</span>{index === 0 ? '★ First proposal' : '○ Candidate ' + (index + 1)}</button><div class="track">{#each candidate.proposed.placements as placement}<button class="time-block candidate-block" class:selected={selected === index} class:first-proposal={index === 0} style={intervalStyle(placement.range, horizon)} onclick={() => onselect(index)} aria-label={'Select candidate ' + (index + 1) + ', ' + time(placement.range.start) + ' to ' + time(placement.range.end) + ' UTC'} aria-pressed={selected === index} title={'[' + placement.range.start + ', ' + placement.range.end + ')'}>{time(placement.range.start)}–{time(placement.range.end)}</button>{/each}</div></div>
    {:else}<div class="empty-timeline"><strong>{result ? 'No feasible proposal returned' : 'A request waiting to become visible'}</strong><p>{result ? 'Validation and rejection evidence remain available in the inspector.' : 'Run the Rust planner to reveal the actual search result.'}</p></div>{/each}
  </div></div>
  {#if result?.candidates.length}<label class="candidate-picker">Inspect candidate<select aria-label="Inspect candidate" value={selected} onchange={(event) => onselect(Number(event.currentTarget.value))}>{#each result.candidates as candidate, index}<option value={index}>#{index + 1} · {time(candidate.start)} UTC{index === 0 ? ' · first proposal' : ''}</option>{/each}</select></label>{/if}
  {#if result?.conflicts.rejections.length}<label class="candidate-picker">Inspect rejection<select aria-label="Inspect rejection" value={rejected ?? ''} onchange={(event) => rejected = event.currentTarget.value === '' ? null : Number(event.currentTarget.value)}><option value="">Select a returned rejection…</option>{#each result.conflicts.rejections as item, index}<option value={index}>× {index + 1} · {time(item.start)} UTC</option>{/each}</select></label>{/if}
  {#if rejection}<div class="rejection-evidence" role="status"><strong>× Rejected at {time(rejection.start)} UTC</strong>{#each rejection.reasons as reason}<p>{mode === 'Simple' && typeof reason !== 'string' && 'HardConstraint' in reason ? readable(reason.HardConstraint.reason) : rejectionLabel(reason)}</p>{/each}<p class="fine-print">{blockers.length ? 'Blocking facts are outlined in the field where source intervals are available.' : 'The returned reason supplies no blocking interval identities.'}</p>{#if mode !== 'Simple'}<JsonPanel title="Selected rejection / exact evidence" value={rejection} />{/if}</div>{/if}
  <SemanticLegend />
  <p class="fine-print">{result ? 'Positions evaluated by the completed planner run.' : 'Returned positions will appear after the run.'} Shapes show returned placements; × marks returned rejection starts. Up to 8 facts, 8 occurrences, 8 additional blockers per type, 8 preferences, 8 candidates plus selection, 48 rejection markers and 32 coverage intervals per type are painted. No unreported search trace is inferred.</p>
  {#if mode !== 'Simple'}<p class="fine-print">Exact horizon: [{horizon.start}, {horizon.end}). Granularity: {result?.search_space.granularity ?? request?.granularity} seconds.</p>{/if}
</section>
