<script lang="ts">
  import ManualSteps from './ManualSteps.svelte';
  import Timeline from './Timeline.svelte';
  import ScoreChart from './ScoreChart.svelte';
  import AuthorityRail from './AuthorityRail.svelte';
  import JsonPanel from './JsonPanel.svelte';
  import { traceSections } from '../lib/l3-presentation.ts';
  import { readable } from '../lib/presentation.ts';
  import type { ExplanationMode, PlanningRequest, PlanningResult } from '../lib/contracts.ts';
  let { result, request, mode, selected, onselect, rejected, onreject }: {
    result: PlanningResult | null; request: PlanningRequest | null; mode: ExplanationMode;
    selected: number; onselect: (index: number) => void; rejected: number | null; onreject: (index: number | null) => void;
  } = $props();
  let step = $state(0);
  const sections = $derived(result ? traceSections(result, request) : []);
  const current = $derived(sections[step] ?? sections[0]);
  $effect(() => { void result; step = 0; });
</script>
{#if result}
  <section class="panel inspector-panel trace-instrument" aria-labelledby="trace-title">
    <div class="panel-heading"><div><span class="eyebrow">COMPLETED RESULT / STRUCTURAL EXPLANATION</span><h2 id="trace-title">Follow the evidence to the proposal.</h2></div><span class="badge subtle">{readable(result.outcome)}</span></div>
    <p class="panel-description">This pipeline connects returned result sections. It is a structural explanation of a completed result, not an internal runtime trace.</p>
    <ManualSteps labels={sections.map((section) => section.label)} selected={step} onchange={(index) => step = index} name="Trace evidence stages" />
    {#if !request}<p class="panel-description">Imported result: its source CPIR is not attached. No request or compiler input is reconstructed.</p>{/if}
    {#if current}<div class="l3-narration" role="status"><strong>{current.label}</strong><p>{current.meaning}</p>{#if mode !== 'Simple' && current.label === 'Search'}<p class="exact-interval">[{result.search_space.horizon.start}, {result.search_space.horizon.end}) · {result.search_space.granularity} s grid · {readable(result.assessment)}</p>{/if}</div>{/if}
    <div class="run-flow"><div><strong>{result.search_space.evaluated}</strong><span>evaluated</span></div><i aria-hidden="true">→</i><div class="flow-rejected"><strong>{result.conflicts.rejections.length}</strong><span>rejected by constraints</span></div><i aria-hidden="true">→</i><div class="flow-feasible"><strong>{result.candidates.length}</strong><span>feasible before ranking</span></div></div>
    <p class="fine-print">Select a returned rejection below to locate its start and available blocking intervals. The selection is shared with Planner for this result.</p>
  </section>
  <Timeline {request} {result} {selected} {onselect} {mode} {rejected} {onreject} />
  <ScoreChart {result} {selected} {onselect} {mode} />
  <AuthorityRail hasProposal={Boolean(result.candidates.length)} hasResult />
  <section class="research-context">{#if current}<JsonPanel title={current.label + ' / exact returned evidence'} value={current.value} />{/if}<JsonPanel title="Complete conflict set" value={result.conflicts} /><JsonPanel title="Complete search-space data" value={result.search_space} /><JsonPanel title="Complete validation report" value={result.validation} /><JsonPanel title="Complete PlanningResult / all returned source data" value={result} />{#if request}<JsonPanel title="Exact request sent for this result" value={request} />{/if}</section>
{:else}<section class="panel inspector-panel"><h2>No completed result yet</h2><p class="panel-description">Run a Planner request or import a PlanningResult. Trace will connect its real sections and constraint evidence.</p></section>{/if}
