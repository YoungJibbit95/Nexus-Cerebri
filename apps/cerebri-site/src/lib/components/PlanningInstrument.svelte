<script lang="ts">
  import { onMount } from 'svelte';
  import { planner, request, candidates, ranked, timeLabel, candidateIdentity } from '$lib/planning-display';
  import { plannerSemanticLegend } from '$lib/visual-grammar';
  import AuthorityRail from './AuthorityRail.svelte';
  import SemanticLegend from './SemanticLegend.svelte';
  import CandidateField from './CandidateField.svelte';
  import CandidateDecisionPlane from './CandidateDecisionPlane.svelte';
  const selectedLabel = timeLabel(ranked[0].start);
  const validCount = ranked.length;
  const rejectedCount = planner.conflicts.rejections.length;
  const exhausted = planner.search_space.exhausted;
  let host: HTMLElement;
  let flowStage = $state(3);
  let reducedMotion = $state(true);
  onMount(() => {
    const motion = matchMedia('(prefers-reduced-motion: reduce)');
    let frame = 0;
    const update = () => {
      reducedMotion = motion.matches;
      const comparator = host.querySelector('.deterministic-comparator')?.getBoundingClientRect();
      if (reducedMotion) { flowStage = 3; return; }
      if (!comparator || comparator.top > innerHeight * .72) flowStage = 0;
      else if (comparator.top > -comparator.height * .22) flowStage = 1;
      else if (comparator.bottom > innerHeight * .24) flowStage = 2;
      else flowStage = 3;
    };
    const schedule = () => { cancelAnimationFrame(frame); frame = requestAnimationFrame(update); };
    update(); window.addEventListener('scroll', schedule, {passive:true}); window.addEventListener('resize', schedule); motion.addEventListener('change', schedule);
    return () => { cancelAnimationFrame(frame); window.removeEventListener('scroll', schedule); window.removeEventListener('resize', schedule); motion.removeEventListener('change', schedule); };
  });
</script>
<div bind:this={host} class="planning-instrument" data-exhausted={exhausted} data-flow-root="planning" data-flow-stage={flowStage} data-motion={reducedMotion ? 'reduced' : 'full'}>
  <div class="instrument-meta"><span>{validCount} valid / {rejectedCount} rejected</span><span>NO PREFERRED TIME SUPPLIED</span></div>
  <SemanticLegend items={plannerSemanticLegend} compact label="Planning visual grammar" />
  <div class="workbench-entry"><span>{candidateIdentity(ranked[0].start)} · {selectedLabel} UTC</span><i aria-hidden="true"></i><small>SAME FIELD · EXACT COORDINATES</small></div>
  <CandidateField />
  <ol class="sr-candidates" aria-label="Evaluated candidate intervals">
    {#each candidates as candidate}
      <li>
        Candidate starting {candidate.label} UTC for {request.duration.value.knowledge.data / 60} minutes.
        {candidate.state === 'rejected'
          ? 'Rejected because it overlaps known busy time.'
          : candidate.state === 'selected'
            ? 'Valid and selected as the first proposal after deterministic ordering.'
            : candidate.state === 'valid'
              ? 'Valid candidate.'
              : 'Not classified in the generated result.'}
      </li>
    {/each}
  </ol>


  <section class="semantic-flow" aria-label="Candidate to authority flow">
    <div class:reached={flowStage >= 0} class:current={flowStage === 0} class="flow-node candidates" data-flow-node="candidates">
      <span class="flow-index">01</span>
      <i aria-hidden="true"></i>
      <strong>Valid options</strong>
      <small>{validCount} pass the required checks</small>
    </div>
    <div class:reached={flowStage >= 1} class="flow-link" aria-hidden="true"><span></span><i></i></div>
    <div class:reached={flowStage >= 1} class:current={flowStage === 1} class="flow-node comparator" data-flow-node="comparator">
      <span class="flow-index">02</span>
      <i aria-hidden="true"></i>
      <strong>Compare options</strong>
      <small>See why one comes first</small>
    </div>
    <div class:reached={flowStage >= 2} class="flow-link" aria-hidden="true"><span></span><i></i></div>
    <div class:reached={flowStage >= 2} class:current={flowStage === 2} class="flow-node proposal" data-flow-node="proposal">
      <span class="flow-index">03</span>
      <i aria-hidden="true"></i>
      <strong>First proposal</strong>
      <small>{selectedLabel} UTC remains a proposal</small>
    </div>
    <div class:reached={flowStage >= 3} class="flow-link" aria-hidden="true"><span></span><i></i></div>
    <div class:reached={flowStage >= 3} class:current={flowStage === 3} class="flow-node authority" data-flow-node="authority">
      <span class="flow-index">04</span>
      <i aria-hidden="true"></i>
      <strong>Before execution</strong>
      <small>Still needs validation and authorization</small>
    </div>
  </section>

  <CandidateDecisionPlane {flowStage} {reducedMotion} />
  <div class="proposal-boundary" aria-label={"Selected proposal at " + selectedLabel + " UTC still needs validation and authorization."}>
    <span><b>{candidateIdentity(ranked[0].start)} · {selectedLabel} UTC</b><small>SELECTED PROPOSAL</small></span><i aria-hidden="true"></i><span class="authority-label"><b>PERMISSION CHECK</b><small>STILL REQUIRED</small></span><span><b>EXECUTION</b><small>nothing has been booked</small></span>
  </div>
  <section class="authority-lifecycle" aria-label="Planner authority boundary"><div><small>BEFORE ANYTHING IS BOOKED</small><strong>This example produces a proposal.</strong><p>Before a calendar can change, the plan must be validated, translated into actions and authorized. The executor checks it again before making a change.</p></div><AuthorityRail currentStage="proposal" label="The current fixture reaches proposal only; later authority states are separate" /></section>
</div>
<style>
  .planning-instrument{position:relative;display:grid;gap:24px;padding-top:26px;min-width:0}
  .instrument-meta,.workbench-entry{display:flex;align-items:center;flex-wrap:wrap;gap:16px;font:.8125rem var(--mono);color:var(--text-3)}.instrument-meta span:last-child{margin-left:auto;font-size:.8125rem}.workbench-entry{margin:32px 0 20px}.workbench-entry>span{color:var(--cyan)}.workbench-entry i{flex:1;border-top:1px solid var(--line-strong)}.workbench-entry small{font-size:.8125rem}
  .semantic-flow{display:grid;grid-template-columns:1fr 30px 1fr 30px 1fr 30px 1fr;gap:10px;align-items:start}
  .flow-node{position:relative;display:grid;gap:6px;padding:16px 0;border-top:1px solid var(--line);min-width:0;transition:border-color 400ms,color 400ms;color:var(--text-3)}.flow-node.current{border-top-color:var(--cyan);color:var(--text-1)}.flow-index{font:.8125rem var(--mono);color:var(--cyan)}.flow-node strong{font-size:1rem}.flow-node small{font-size:.8125rem;line-height:1.5}.flow-node>i{display:none}.flow-link{border-top:1px solid var(--line);margin-top:0}
  .sr-candidates{position:absolute!important;width:1px;height:1px;padding:0;margin:-1px;overflow:hidden;clip-path:inset(50%);white-space:nowrap}
  .proposal-boundary{display:grid;grid-template-columns:1fr 48px 1fr 1fr;gap:20px;align-items:center;padding:24px 0;border-bottom:1px solid var(--line)}.proposal-boundary b,.proposal-boundary small{display:block;font:.8125rem/1.8 var(--mono)}.proposal-boundary small{color:var(--text-3);font-size:.8125rem}.proposal-boundary>i{height:1px;background:var(--line-strong)}.authority-label{padding-left:16px;border-left:2px solid var(--amber);color:var(--amber)}
  .authority-lifecycle{display:grid;grid-template-columns:minmax(0,1fr);gap:36px;padding:30px 0}.authority-lifecycle small{display:block;color:var(--amber);font:.8125rem var(--mono);margin-bottom:12px}.authority-lifecycle strong{font-size:1.125rem}.authority-lifecycle p{font-size:.9375rem;color:var(--text-3)}
  @media(max-width:760px){.semantic-flow{grid-template-columns:1fr 1fr;gap:8px 20px}.flow-link{display:none}.flow-node strong{font-size:1rem}.authority-lifecycle{grid-template-columns:1fr}.proposal-boundary{grid-template-columns:1fr 24px 1fr;gap:8px}.proposal-boundary>span:last-child{grid-column:3}.proposal-boundary b{font-size:.8125rem}.workbench-entry{gap:10px}.workbench-entry small{font-size:.8125rem}.instrument-meta span:last-child{margin-left:0}}
  @media(prefers-reduced-motion:reduce){.flow-node{transition:none}}
</style>
