<script lang="ts">
  import Icon from './Icon.svelte';
  import JsonPanel from './JsonPanel.svelte';
  import { date, known, time } from '../lib/presentation.ts';
  import type { PlanningRequest, ValidationReport } from '../lib/contracts.ts';
  let { request, label, busy, validation, ondemo, onimport, onvalidate, onplan }: {
    request: PlanningRequest; label: string; busy: boolean; validation: ValidationReport | null;
    ondemo: () => void; onimport: (file: File) => void; onvalidate: () => void; onplan: () => void;
  } = $props();
  let input: HTMLInputElement;
  const duration = $derived(known(request.duration));
</script>

<section class="panel request-panel" aria-labelledby="request-title">
  <div class="panel-heading"><div><span class="eyebrow">INPUT / CPIR {request.schema_version.major}.{request.schema_version.minor}</span><h2 id="request-title">Planning context</h2></div><span class="badge subtle">{request.operation}</span></div>
  <div class="scenario-title"><span class="scenario-icon"><Icon name="clock" size={22} /></span><div><strong>{label}</strong><small>{request.request_id}</small></div></div>
  <dl class="context-list"><div><dt>Horizon</dt><dd>{date(request.scope.time_range.start)}<small>{time(request.scope.time_range.start)} — {time(request.scope.time_range.end)} UTC</small></dd></div><div><dt>Duration / grid</dt><dd>{duration === undefined ? 'Unresolved' : `${duration / 60} min`}<span class="muted"> / {request.granularity / 60} min</span></dd></div><div><dt>Snapshot</dt><dd>{request.context.objects.length} objects <span class="muted">· rev {request.context.revision}</span></dd></div><div><dt>Search budget</dt><dd>{request.budget.max_candidates} positions</dd></div></dl>
  {#if validation}<p class="validation-state"><Icon name="shield" size={15} />Request: {validation.state} · {validation.issues.length} issues</p>{/if}
  <div class="request-buttons"><button class="primary" onclick={onplan} disabled={busy}><Icon name="play" size={17} />{busy ? 'Working…' : 'Run planner'}<span>↗</span></button><button class="secondary" onclick={onvalidate} disabled={busy}>Validate CPIR</button></div>
  <div class="text-actions"><button onclick={ondemo} disabled={busy}>Load synthetic demo</button><button onclick={() => input.click()} disabled={busy}><Icon name="upload" size={14} />Import CPIR</button></div>
  <input bind:this={input} class="visually-hidden" tabindex="-1" type="file" accept=".json,application/json" aria-label="Import CPIR request" onchange={(event) => { const file = event.currentTarget.files?.[0]; if (file) onimport(file); event.currentTarget.value = ''; }} />
  <JsonPanel title="Inspect request" value={request} />
  <p class="fine-print">Synthetic demo includes an explicit 10:45 UTC start preference. Requests and results stay in this page’s memory.</p>
</section>
