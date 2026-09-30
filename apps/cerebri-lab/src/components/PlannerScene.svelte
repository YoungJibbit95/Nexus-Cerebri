<script lang="ts">
  import { onMount, untrack } from 'svelte';
  import type { ExplanationMode, PlanningRequest, PlanningResult } from '../lib/contracts.ts';
  import { readable } from '../lib/presentation.ts';
  import { runCounts } from '../lib/planner-presentation.ts';
  import { createStoryPlayback, STORY_LAST_STAGE } from '../lib/motion.ts';
  import Timeline from './Timeline.svelte';
  import ScoreChart from './ScoreChart.svelte';
  import CandidateDetail from './CandidateDetail.svelte';
  import AuthorityRail from './AuthorityRail.svelte';
  let { request, result, selected, onselect, mode, runId }: {
    request: PlanningRequest | null; result: PlanningResult | null; selected: number;
    onselect: (index: number) => void; mode: ExplanationMode; runId: number;
  } = $props();
  let stage = $state(STORY_LAST_STAGE);
  let reduced = $state(false);
  let lastRun = untrack(() => runId);
  const playback = createStoryPlayback((next) => stage = next);
  const stages = ['Request & context', 'Search field', 'Constraint filter', 'Valid candidates', 'Deterministic order', 'Proposal'];
  const counts = $derived(result ? runCounts(result) : null);
  onMount(() => {
    const media = matchMedia('(prefers-reduced-motion: reduce)');
    reduced = media.matches;
    const change = () => { reduced = media.matches; if (reduced) playback.finish(); };
    media.addEventListener('change', change);
    return () => { media.removeEventListener('change', change); playback.cancel(); };
  });
  $effect(() => {
    // A new API response may explain itself once. Any new selection/depth/result cancels stale work.
    void selected; void mode; void result;
    if (runId > 0 && runId !== lastRun && result) { lastRun = runId; playback.play(reduced); }
    else { lastRun = runId; playback.finish(); }
    return () => playback.cancel();
  });
  function select(index: number) { playback.finish(); onselect(index); }
</script>
<div class="planner-scene" data-stage={stage} data-selected-candidate={selected + 1}>
  <section class="run-instrument" aria-label="Planner run summary">
    <div class="instrument-heading"><span class="eyebrow">{result ? 'COMPLETED RUN / RESULT EXPLANATION' : 'REQUEST → PROPOSAL'}</span><span class="badge subtle">{result ? readable(result.assessment) : 'Awaiting Rust core'}</span></div>
    <div class="run-flow">
      <div><strong>{counts?.evaluated ?? '—'}</strong><span>evaluated</span></div><i aria-hidden="true">→</i>
      <div class="flow-rejected"><strong>{counts?.rejected ?? '—'}</strong><span>rejected</span></div><i aria-hidden="true">→</i>
      <div class="flow-feasible"><strong>{counts?.feasible ?? '—'}</strong><span>feasible</span></div><i aria-hidden="true">→</i>
      <div class="flow-proposal"><strong>{result?.candidates.length ? '#1' : '—'}</strong><span>{result?.candidates.length ? 'first proposal' : result ? 'no proposal returned' : 'no proposal yet'}</span></div>
    </div>
    <p class="fine-print">{result ? `${readable(result.outcome)}. Assessment applies only to the declared discrete grid. The counts come from the completed response.` : 'Load or import a request, then run the planner. No positions or scores are simulated.'}</p>
    {#if result}
      <nav class="story-stages" aria-label="Completed result explanation stages">{#each stages as label, index}<button class:active={stage === index} aria-pressed={stage === index} onclick={() => { playback.cancel(); stage = index; }}><small>0{index + 1}</small>{index === 5 && !result.candidates.length ? 'No proposal' : label}</button>{/each}</nav>
      <div class="story-actions"><span>Post-run explanation · all results are already available</span><button onclick={() => playback.play(reduced)}>Replay explanation</button><button onclick={() => playback.finish()}>Skip to result</button></div>
    {/if}
  </section>
  <AuthorityRail hasProposal={Boolean(result?.candidates.length)} hasResult={result !== null} />
  <Timeline {request} {result} {selected} onselect={select} {mode} {stage} />
  <ScoreChart {result} {selected} onselect={select} {mode} />
  <CandidateDetail candidate={result?.candidates[selected]} rank={selected + 1} {mode} hasResult={result !== null} />
</div>
