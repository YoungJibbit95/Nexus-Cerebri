<script lang="ts">
  import { runtimeData } from '$lib/generated/runtime-data';
  import VisualizationFrame from './VisualizationFrame.svelte';

  const planner = runtimeData.plannerResult as any;
  const request = runtimeData.request as any;
  const winner = planner.candidates?.[0]?.proposed?.placements?.[0]?.range;
  let expanded = false;
</script>

<section class="intent-section" aria-labelledby="intent-title">
  <div class="section-copy">
    <span class="kicker">Intent → Plan</span>
    <h2 id="intent-title">From human intent to bounded evidence.</h2>
    <p>
      This is an educational walk through the canonical synthetic request. It does not parse
      free-form language or pretend that a model is running in the browser.
    </p>
  </div>

  <VisualizationFrame
    kind="EDUCATIONAL"
    label="Canonical intent walkthrough"
    caption="The CPIR and planner result shown here are repository fixtures; the human sentence is only an explanatory label."
    textAlternative="A four-stage chain shows an example human request, the CPIR 0.1 request, deterministic candidate search and the selected proposed time."
  >
    <div class="intent-flow">
      <article>
        <small>01 · Intent</small>
        <strong>“Find a 30 minute slot this morning.”</strong>
        <span>Explanatory wording only</span>
      </article>
      <b aria-hidden="true">→</b>
      <article>
        <small>02 · CPIR</small>
        <strong>{request.operation} · {request.duration.value.knowledge.data}s</strong>
        <span>Schema {runtimeData.metadata.cpirVersion}</span>
      </article>
      <b aria-hidden="true">→</b>
      <article>
        <small>03 · Search</small>
        <strong>{planner.candidates?.length ?? 0} feasible candidates</strong>
        <span>{planner.assessment ?? 'Structured assessment'}</span>
      </article>
      <b aria-hidden="true">→</b>
      <article>
        <small>04 · Proposal</small>
        <strong>{winner ? new Date(winner.start).toLocaleTimeString('en-GB', { hour: '2-digit', minute: '2-digit', timeZone: 'UTC' }) + ' UTC' : 'See result'}</strong>
        <span>Proposal, not execution</span>
      </article>
    </div>
    <button class="inspect-button" aria-expanded={expanded} onclick={() => (expanded = !expanded)}>
      {expanded ? 'Hide structured evidence' : 'Inspect structured evidence'}
    </button>
    {#if expanded}
      <div class="evidence-grid">
        <pre><code>{JSON.stringify(request.scope, null, 2)}</code></pre>
        <pre><code>{JSON.stringify(planner.candidates?.[0] ?? planner, null, 2)}</code></pre>
      </div>
    {/if}
  </VisualizationFrame>
</section>
