<script lang="ts">
  import ManualSteps from './ManualSteps.svelte';
  import JsonPanel from './JsonPanel.svelte';
  import { readable, time } from '../lib/presentation.ts';
  import { localPosition, nominalEvidence } from '../lib/l3-presentation.ts';
  import type { TemporalReport, TemporalRequest } from '../lib/temporal.ts';
  import type { ExplanationMode } from '../lib/contracts.ts';
  let { report, request, mode }: { report: TemporalReport; request: TemporalRequest; mode: ExplanationMode } = $props();
  let step = $state(0), recurrence = $state(0), selected = $state('');
  const labels = ['Nominal local time', 'Timezone mapping', 'DST discontinuity', 'Policy decision', 'UTC result', 'Availability effect'];
  const expansion = $derived(report.expansions[recurrence]);
  const rule = $derived(request.recurrences[recurrence]);
  const events = $derived(expansion ? nominalEvidence(expansion, rule) : []);
  const current = $derived(events.find((item) => item.key === selected) ?? events.find((item) => item.skip) ?? events[0]);
  $effect(() => { void report; recurrence = 0; selected = ''; step = 0; });
  const narration = $derived(!current ? 'The core returned no visible occurrence or skipped nominal date for this recurrence.' : [
    `This event repeats at ${current.localTime ?? 'an unattached local time'} on a local clock. Selected date: ${current.date}.`,
    `The core interprets the local clock in ${expansion.timezone}; local time and UTC are separate views.`,
    current.skip ? 'This local time does not exist on this date. The core reports a DST gap.' : current.occurrence?.resolution === 'Unique' ? 'The core returned one unambiguous mapping for this local point.' : 'This local time occurs twice. The core returned the explicitly selected fold mapping.',
    current.skip ? `The configured ${rule.gap_policy} policy branches this nominal point to SKIPPED.` : `Gap policy: ${rule.gap_policy}. Fold policy: ${rule.fold_policy}. Returned resolution: ${readable(current.occurrence!.resolution)}.`,
    current.skip ? 'Skipped means no UTC occurrence was emitted for this nominal date.' : `The returned UTC occurrence runs from ${time(current.occurrence!.range.start)} to ${time(current.occurrence!.range.end)}.`,
    report.availability.coverage === 'Complete' ? 'Complete coverage allows the core to label the complement as free.' : 'Incomplete coverage leaves unoccupied gaps unknown. Unknown is never free.',
  ][step]);
</script>
<section class="l3-instrument temporal-story" data-step={step} aria-labelledby="temporal-story-title">
  <div class="panel-heading"><div><span class="eyebrow">LOCAL CLOCK → CORE RESOLUTION → UTC</span><h2 id="temporal-story-title">What happened at this clock time?</h2></div><span class="badge subtle">Manual story</span></div>
  <ManualSteps {labels} selected={step} onchange={(index) => step = index} name="Temporal explanation steps" />
  <div class="l3-narration" role="status"><strong>{labels[step]}</strong><p>{narration}</p></div>
  {#if report.expansions.length}<label class="candidate-picker">Inspect recurrence<select aria-label="Inspect recurrence" value={recurrence} onchange={(event) => { recurrence = Number(event.currentTarget.value); selected = ''; }} >{#each report.expansions as item, index}<option value={index}>Recurrence {index + 1} · {item.timezone}</option>{/each}</select></label>{/if}
  {#if expansion}
    <p class="panel-description">{expansion.examined_dates} dates examined · {expansion.occurrences.length} visible occurrences · {expansion.skipped.length} skipped</p>
    {#if events.length}<label class="candidate-picker">Inspect nominal evidence<select aria-label="Inspect nominal evidence" value={current?.key} onchange={(event) => selected = event.currentTarget.value}>{#each events as item}<option value={item.key}>{item.date} · {item.localTime ?? 'time unattached'} · {item.skip ? 'Skipped' : readable(item.occurrence!.resolution)}</option>{/each}</select></label>{/if}
    {#if current}
      <div class="clock-bridge" class:gap={Boolean(current.skip)} class:fold={current.occurrence?.resolution !== 'Unique' && !current.skip}>
        <div class="local-clock"><span class="eyebrow">{current.date} / LOCAL CLOCK</span><div class="clock-axis"><span>00:00</span><span>06:00</span><span>12:00</span><span>18:00</span><span>24:00</span></div><div class="clock-track">
          {#if current.localTime}<span class="nominal-point" class:resolved={step >= 3} style={'left:' + localPosition(current.localTime) + '%'}>◇<small>{current.localTime.slice(0, 5)}</small></span>{/if}
          {#if current.skip && step >= 2}<span class="clock-discontinuity" style={'left:' + (current.localTime ? localPosition(current.localTime) : 50) + '%'}><strong>× GAP</strong></span>{/if}
        </div><p>{current.skip && step >= 2 ? 'This local point does not exist.' : current.occurrence && current.occurrence.resolution !== 'Unique' && step >= 2 ? '↔ A repeated local point: fold mapping is explicit.' : '◇ Nominal local point'}</p></div>
        <div class="timezone-bridge"><span>↓</span><strong>{expansion.timezone}</strong>{#if mode !== 'Simple'}<small>Gap: {rule.gap_policy} · Fold: {rule.fold_policy}</small>{/if}</div>
        <div class="resolution-branch" class:emphasized={step >= 3}>{#if current.skip}<span>↳ × SKIPPED</span><strong>No UTC occurrence</strong><p>{mode === 'Simple' ? 'The configured policy skips this missing clock time.' : readable(current.skip.reason) + ' · sequence ' + current.sequence}</p>{:else}<span>↳ ▥ UTC OCCURRENCE</span><strong>{time(current.occurrence!.range.start)} → {time(current.occurrence!.range.end)} UTC</strong><p>{readable(current.occurrence!.resolution)}{#if mode !== 'Simple'} · sequence {current.sequence}{/if}</p>{/if}</div>
      </div>
      {#if mode !== 'Simple'}<p class="fine-print">The gap glyph marks the returned nonexistent nominal point; the API does not return the transition's full local boundaries. No transition interval is inferred.</p>{#if current.occurrence}<p class="exact-interval">Full UTC [{current.occurrence.range.start}, {current.occurrence.range.end})<br />Visible [{current.occurrence.visible_range.start}, {current.occurrence.visible_range.end})</p>{/if}<JsonPanel title="Selected nominal evidence / exact core output" value={current.skip ?? current.occurrence} />{/if}
    {/if}
  {:else}<p class="panel-description">No recurrence expansion was returned. Availability still reflects the supplied busy inputs.</p>{/if}
</section>
