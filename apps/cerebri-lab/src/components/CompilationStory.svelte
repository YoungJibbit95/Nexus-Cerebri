<script lang="ts">
  import ManualSteps from './ManualSteps.svelte';
  import JsonPanel from './JsonPanel.svelte';
  import { readable, intervalStyle, time } from '../lib/presentation.ts';
  import { comparisonFrame } from '../lib/l3-presentation.ts';
  import type { CompiledContextSnapshot } from '../lib/compilation.ts';
  import type { ExplanationMode } from '../lib/contracts.ts';
  let { compiled, mode }: { compiled: CompiledContextSnapshot; mode: ExplanationMode } = $props();
  let step = $state(0), series = $state(0), selected = $state('');
  const labels = ['Temporal series', 'Recurrence rule', 'Returned nominal evidence', 'Materialized occurrences', 'Clipping / visibility', 'Compiled occupancy'];
  const source = $derived(compiled.series[series]);
  const materialized = $derived(compiled.occurrences.filter((item) => item.series_id === source?.series.id));
  const current = $derived(materialized.find((item) => item.id === selected) ?? materialized[0]);
  const visible = $derived([...materialized.slice(0, 8), ...(current && materialized.indexOf(current) >= 8 ? [current] : [])]);
  const frame = $derived(current ? comparisonFrame(current.occurrence.range, compiled.horizon) : compiled.horizon);
  $effect(() => { void compiled; series = 0; selected = ''; step = 0; });
</script>
<section class="l3-instrument compilation-story" data-step={step} aria-labelledby="compilation-story-title">
  <div class="panel-heading"><div><span class="eyebrow">SOURCE → MATERIALIZATION → OCCUPANCY</span><h2 id="compilation-story-title">One rule becomes occupied times.</h2></div><span class="badge subtle">Core-produced snapshot</span></div>
  <ManualSteps {labels} selected={step} onchange={(index) => step = index} name="Compilation explanation steps" />
  {#if source}
    <label class="candidate-picker">Inspect series<select aria-label="Inspect series" value={series} onchange={(event) => { series = Number(event.currentTarget.value); selected = ''; }}>{#each compiled.series as item, index}<option value={index}>{mode === 'Simple' ? 'Series ' + (index + 1) : item.series.id}</option>{/each}</select></label>
    <p class="panel-description">{source.expansion.occurrences.length} visible occurrences · {materialized.length} identified occurrences · {source.expansion.skipped.length} skipped nominal dates</p>
    <div class="l3-narration" role="status"><strong>{labels[step]}</strong><p>{[
      'This existing repeating source is a fact; it is separate from the target being planned.',
      `Its ${readable(source.series.rule.pattern.frequency).toLowerCase()} rule uses ${source.series.rule.local_time} in ${source.series.rule.timezone}.`,
      `${source.expansion.occurrences.length} visible occurrences and ${source.expansion.skipped.length} skipped nominal dates were returned. Dates outside the report are not reconstructed.`,
      `${materialized.length} identified occurrences from this series became occupancy evidence.`,
      'Full intervals preserve occurrence identity. Visible intervals retain only the part inside the horizon.',
      compiled.availability.coverage === 'Complete' ? 'The core combines all occupancy; its remaining gaps are free under Complete coverage.' : 'The core combines all occupancy; remaining gaps stay unknown under Incomplete coverage.',
    ][step]}</p></div>
    <div class="series-fanout">
      <article class="series-source"><span>▥ SOURCE SERIES</span><strong>{mode === 'Simple' ? 'Repeating rule ' + (series + 1) : source.series.id}</strong><p>{source.series.rule.local_time.slice(0, 5)} · {source.series.rule.timezone}</p>{#if mode !== 'Simple'}<small>{readable(source.series.provenance)} · source revision {source.series.state.kind === 'Existing' ? source.series.state.revision : 'Prospective'}</small>{/if}</article>
      <div class="fanout-branches" aria-label="Returned materialization branches">
        {#each visible as item}<button class="occurrence-node" class:active={item.id === current?.id} aria-pressed={item.id === current?.id} onclick={() => selected = item.id}><span>↳ ▥ {item.occurrence.date}</span><strong>{time(item.occurrence.range.start)} → {time(item.occurrence.range.end)} UTC</strong><small>{readable(item.occurrence.resolution)}{#if mode !== 'Simple'} · sequence {item.occurrence.sequence}{/if}</small></button>{/each}
        {#each source.expansion.skipped.slice(0, 8) as skip}<article class="skip-node"><span>↳ × SKIPPED · {skip.date}</span><strong>{source.series.rule.local_time.slice(0, 5)} does not exist</strong><small>{mode === 'Simple' ? 'Configured policy skips this nominal date.' : readable(skip.reason) + ' · sequence ' + skip.sequence + ' · ' + source.series.rule.gap_policy}</small></article>{/each}
        {#if !materialized.length && !source.expansion.skipped.length}<p class="panel-description">No visible occurrence or skipped nominal date was returned for this series.</p>{/if}
      </div>
    </div>
    {#if materialized.length}<label class="candidate-picker">Inspect materialized occurrence<select aria-label="Inspect materialized occurrence" value={current?.id} onchange={(event) => selected = event.currentTarget.value}>{#each materialized as item}<option value={item.id}>{item.occurrence.date} · sequence {item.occurrence.sequence}</option>{/each}</select></label>{/if}
    {#if current}
      <div class="clipping-instrument"><h3>Full occurrence → visible occupancy</h3><p class="panel-description">{frame.start} → {frame.end} UTC</p>
        <div class="timeline-row"><span class="row-label">▥ Full interval</span><div class="track"><span class="time-block recurrence-block" style={intervalStyle(current.occurrence.range, frame)}></span></div></div>
        <div class="timeline-row"><span class="row-label">Scope / horizon</span><div class="track"><span class="time-block scope-block" style={intervalStyle(compiled.horizon, frame)}></span></div></div>
        <div class="timeline-row"><span class="row-label">▰ Visible part</span><div class="track"><span class="time-block busy-block" style={intervalStyle(current.occurrence.visible_range, frame)}></span></div></div>
        <p class="panel-description">{current.occurrence.range.start !== current.occurrence.visible_range.start || current.occurrence.range.end !== current.occurrence.visible_range.end ? 'The horizon clips this occurrence. Its full identity and range remain intact.' : 'This occurrence is fully inside the horizon; its visible range equals its full range.'}</p>
        {#if mode !== 'Simple'}<p class="exact-interval">Full [{current.occurrence.range.start}, {current.occurrence.range.end})<br />Visible [{current.occurrence.visible_range.start}, {current.occurrence.visible_range.end})</p><p class="fine-print">{current.id} · {readable(current.provenance)} · revision {current.source_revision} · evidence: {current.evidence.join(', ') || 'None supplied'}</p><JsonPanel title="Selected materialized occurrence" value={current} />{/if}
      </div>
    {/if}
    <JsonPanel title="Selected source series and expansion" value={source} />
  {:else}<p class="panel-description">No temporal series were supplied. The compiled occupancy can still contain ordinary context facts.</p>{/if}
  <p class="fine-print">Diagram: one selected series, first 8 materialized occurrences plus selection and first 8 skips. Complete counts and source evidence remain available in Research and exports.</p>
</section>
