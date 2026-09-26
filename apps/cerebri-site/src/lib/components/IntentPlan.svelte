<script lang="ts">
  import { runtimeData } from '$lib/generated/runtime-data';
  import PlanningWorkbench from './PlanningWorkbench.svelte';
  import VisualizationFrame from './VisualizationFrame.svelte';

  const planner = runtimeData.plannerResult as any;
  const request = runtimeData.request as any;
  const evaluated = Number(planner.search_space?.evaluated ?? 0);
  const rejectedCount = planner.conflicts?.rejections?.length ?? 0;
  const feasibleCount = planner.candidates?.length ?? 0;
  const visualAlternative =
    'A planning workbench shows structured scope, known busy state, explicit constraint and preference state, ' +
    evaluated +
    ' Rust-evaluated candidate positions, ' +
    rejectedCount +
    ' rejections, ' +
    feasibleCount +
    ' valid candidates, the deterministic ordering key and the authority boundary after the first proposal.';
  let expanded = $state(false);
</script>

<section class="intent-section cosmic-section" aria-labelledby="intent-title">
  <div class="intent-nebula" aria-hidden="true"></div>
  <div class="section-copy intent-copy">
    <span class="section-index">ACT 03 / REAL FIXTURE → PLAN</span>
    <span class="kicker">EVIDENCE → CANDIDATES → ORDER → PROPOSAL</span>
    <h2 id="intent-title">One real request.<br /><em>{evaluated} evaluated positions.</em></h2>
    <p>
      The site build runs the Rust planner against the repository's current CPIR {runtimeData.metadata.cpirVersion} fixture.
      The request is already structured before planning begins; the browser renders the generated result rather than recreating planner logic.
    </p>
  </div>

  <VisualizationFrame
    kind="REAL"
    label="Current CPIR planning fixture"
    caption="Generated from examples/request.json and the Rust planner during the site build."
    textAlternative={visualAlternative}
  >
    <PlanningWorkbench />

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
