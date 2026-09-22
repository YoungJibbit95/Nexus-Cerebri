<script lang="ts">
  import { runtimeData } from '$lib/generated/runtime-data';
  import VisualizationFrame from './VisualizationFrame.svelte';

  const planner = runtimeData.plannerResult as any;
  const request = runtimeData.request as any;
  const winner = planner.candidates?.[0]?.proposed?.placements?.[0]?.range;
  let expanded = $state(false);
</script>

<section class="intent-section cosmic-section" aria-labelledby="intent-title">
  <div class="intent-nebula" aria-hidden="true"></div>
  <div class="section-copy intent-copy">
    <span class="section-index">ACT 03 / INTENT → PLAN</span>
    <span class="kicker">FROM LANGUAGE TO BOUNDED EVIDENCE</span>
    <h2 id="intent-title">A thought becomes<br /><em>a trajectory.</em></h2>
    <p>
      This educational walkthrough follows the canonical synthetic request into typed CPIR evidence,
      deterministic candidate search and a proposal. It does not parse free-form language or simulate a browser planner.
    </p>
  </div>

  <VisualizationFrame
    kind="EDUCATIONAL"
    label="Canonical intent trajectory"
    caption="The CPIR and planner result shown here are repository fixtures; the human sentence is only an explanatory label."
    textAlternative="A four-stage chain shows an example human request, the CPIR request, deterministic candidate search and the selected proposed time."
  >
    <div class="intent-field">
      <div class="trajectory-glow" aria-hidden="true"></div>
      <div class="intent-stage-summary">
        <div><small>PLAIN LANGUAGE</small><strong>The request starts open-ended, then becomes more constrained at every step.</strong></div>
        <div class="depth-technical"><small>TECHNICAL READING</small><strong>Human wording is illustrative; CPIR and planner output are the structured repository-backed states.</strong></div>
      </div>
      <div class="intent-axis" aria-hidden="true"><span>HUMAN / PROBABILISTIC</span><i></i><span>DETERMINISTIC / PROOF-BOUND</span></div>
      <div class="intent-flow">
        <article class="intent-node intent-human">
          <span class="intent-planet" aria-hidden="true"></span>
          <small>01 · INTENT</small>
          <strong>“Find a 30 minute slot this morning.”</strong>
          <span>Explanatory wording only</span>
          <i class="node-state">unbound</i>
        </article>
        <div class="intent-trace" aria-hidden="true"><i></i><span>structure</span><b></b></div>
        <article class="intent-node">
          <span class="intent-planet cpir-orb" aria-hidden="true"></span>
          <small>02 · CPIR</small>
          <strong>{request.operation} · {request.duration.value.knowledge.data}s</strong>
          <span>Schema {runtimeData.metadata.cpirVersion}</span>
          <i class="node-state">typed</i>
        </article>
        <div class="intent-trace" aria-hidden="true"><i></i><span>search</span><b></b></div>
        <article class="intent-node">
          <span class="intent-planet search-orb" aria-hidden="true"></span>
          <small>03 · SEARCH</small>
          <strong>{planner.candidates?.length ?? 0} feasible candidates</strong>
          <span>{planner.assessment ?? 'Structured assessment'}</span>
          <i class="node-state">evaluated</i>
        </article>
        <div class="intent-trace" aria-hidden="true"><i></i><span>resolve</span><b></b></div>
        <article class="intent-node intent-resolved">
          <span class="intent-planet proof-orb" aria-hidden="true"></span>
          <small>04 · PROPOSAL</small>
          <strong>{winner ? new Date(winner.start).toLocaleTimeString('en-GB', { hour: '2-digit', minute: '2-digit', timeZone: 'UTC' }) + ' UTC' : 'See result'}</strong>
          <span>Proposal, not execution</span>
          <i class="node-state">proof-bound</i>
        </article>
      </div>
      <div class="intent-boundary-note"><span>CPIR BOUNDARY</span><p>Facts, scope, policy and capability stay explicit through the transformation.</p></div>
      <div class="intent-legend" aria-label="Trajectory legend">
        <span><i class="legend-open"></i>open interpretation</span>
        <span><i class="legend-bound"></i>typed boundary</span>
        <span><i class="legend-search"></i>evaluated search</span>
        <span><i class="legend-proof"></i>proof-bound proposal</span>
      </div>
    </div>
    <button class="inspect-button" aria-expanded={expanded} onclick={() => (expanded = !expanded)}>
      <span>{expanded ? 'Hide structured evidence' : 'Inspect structured evidence'}</span><i aria-hidden="true">{expanded ? '−' : '+'}</i>
    </button>
    {#if expanded}
      <div class="evidence-grid">
        <div><small>SCOPE / CANONICAL INPUT</small><pre><code>{JSON.stringify(request.scope, null, 2)}</code></pre></div>
        <div><small>SELECTED CANDIDATE / RUST OUTPUT</small><pre><code>{JSON.stringify(planner.candidates?.[0] ?? planner, null, 2)}</code></pre></div>
      </div>
    {/if}
  </VisualizationFrame>
</section>
