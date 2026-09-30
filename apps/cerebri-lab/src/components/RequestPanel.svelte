<script lang="ts">
  import Icon from './Icon.svelte';
  import JsonPanel from './JsonPanel.svelte';
  import { date, known, time } from '../lib/presentation.ts';
  import type { PlanningRequest, ValidationReport } from '../lib/contracts.ts';
  import { plannerScenarios } from '../lib/scenarios.ts';
  import { requestSentence } from '../lib/planner-presentation.ts';
  import type { ExplanationMode } from '../lib/contracts.ts';
  let { request, label, busy, validation, mode, ondemo, onscenario, onimport, onvalidate, onplan }: {
    request: PlanningRequest; label: string; busy: boolean; validation: ValidationReport | null;
    mode: ExplanationMode;
    ondemo: () => void; onscenario: (file: string) => void; onimport: (file: File) => void; onvalidate: () => void; onplan: () => void;
  } = $props();
  let input: HTMLInputElement;
  const duration = $derived(known(request.duration));
</script>

<section class="panel request-panel" aria-labelledby="request-title">
  <div class="panel-heading"><div><span class="eyebrow">CURRENT INPUT / REQUEST</span><h2 id="request-title">What should be planned?</h2></div>{#if mode !== 'Simple'}<span class="badge subtle">{request.operation}</span>{/if}</div>
  <p class="request-sentence">{requestSentence(request)}</p>
  <p class="request-window">{time(request.scope.time_range.start)} <span>→</span> {time(request.scope.time_range.end)} <small>UTC</small></p>
  <p class="panel-description">{date(request.scope.time_range.start)}{#if date(request.scope.time_range.start) !== date(request.scope.time_range.end)} → {date(request.scope.time_range.end)}{/if}. {request.target_ids.length} target{request.target_ids.length === 1 ? '' : 's'}.{' '}{#if request.preferences.preferences.length === 1}{time(request.preferences.preferences[0].preferred_start)} UTC is a supplied preferred start.{:else if request.preferences.preferences.length}{request.preferences.preferences.length} time preferences supplied.{:else}No preferred time supplied.{/if}</p>
  <div class="scenario-title"><span class="scenario-icon"><Icon name="clock" size={22} /></span><div><strong>{label}</strong>{#if mode !== 'Simple'}<small>{request.request_id}</small>{/if}</div></div>
  <label class="scenario-select">Source scenario<select disabled={busy} onchange={(event) => { if (event.currentTarget.value) onscenario(event.currentTarget.value); event.currentTarget.value = ''; }}><option value="">Load a checked-in CPIR scenario…</option>{#each plannerScenarios as scenario}<option value={scenario.file}>{scenario.name.replaceAll('-', ' ')}</option>{/each}</select></label>
  {#if mode !== 'Simple'}<dl class="context-list"><div><dt>CPIR / target</dt><dd>{request.schema_version.major}.{request.schema_version.minor}<small>{request.target_ids.join(', ')}</small></dd></div><div><dt>Duration / grid</dt><dd>{duration === undefined ? 'Not known' : `${duration} s`}<span class="muted"> / {request.granularity} s</span></dd></div><div><dt>Snapshot</dt><dd>{request.context.objects.length} objects <span class="muted">· rev {request.context.revision}</span></dd></div><div><dt>Search budget</dt><dd>{request.budget.max_candidates} positions</dd></div></dl>{/if}
  {#if validation}<p class="validation-state"><Icon name="shield" size={15} />Request: {validation.state} · {validation.issues.length} issues</p>{/if}
  <div class="request-buttons"><button class="primary" onclick={onplan} disabled={busy}><Icon name="play" size={17} />{busy ? 'Working…' : 'Run planner'}<span>↗</span></button><button class="secondary" onclick={onvalidate} disabled={busy}>Validate CPIR</button></div>
  <div class="text-actions"><button onclick={ondemo} disabled={busy}>Load synthetic demo</button><button onclick={() => input.click()} disabled={busy}><Icon name="upload" size={14} />Import CPIR</button></div>
  <input bind:this={input} class="visually-hidden" tabindex="-1" type="file" accept=".json,application/json" aria-label="Import CPIR request" onchange={(event) => { const file = event.currentTarget.files?.[0]; if (file) onimport(file); event.currentTarget.value = ''; }} />
  <JsonPanel title="Inspect request" value={request} />
  <p class="fine-print">Scenarios load synthetic source requests from examples/planner. Run the planner to obtain actual Core results. Requests and results stay in this page’s memory.</p>
</section>
