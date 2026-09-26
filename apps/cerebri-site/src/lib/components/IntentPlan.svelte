<script lang="ts">
  import { runtimeData } from '$lib/generated/runtime-data';
  import VisualizationFrame from './VisualizationFrame.svelte';
  import PlanningInstrument from './PlanningInstrument.svelte';

  const planner = runtimeData.plannerResult as any;
  const request = runtimeData.request as any;
  const winner = planner.candidates?.[0]?.proposed?.placements?.[0]?.range;
  const winnerFeatures = planner.candidates?.[0]?.ranking_features;
  const evaluated = Number(planner.search_space?.evaluated ?? 0);
  const rejectedCount = planner.conflicts?.rejections?.length ?? 0;
  const feasibleCount = planner.candidates?.length ?? 0;
  const scopeStart = new Date(request.scope.time_range.start).getTime();
  const scopeEnd = new Date(request.scope.time_range.end).getTime();
  const formatTime = (instant: number) => new Date(instant).toLocaleTimeString('en-GB', { hour: '2-digit', minute: '2-digit', timeZone: 'UTC' });
  const preferenceSource = winnerFeatures?.preferred_start_source ?? null;
  const visualAlternative = 'A bounded temporal coordinate shows the structured CPIR request, ' + evaluated + ' evaluated candidate intervals, ' + rejectedCount + ' overlap rejections, ' + feasibleCount + ' valid candidates and the selected proposal stopping before a separate authority gate.';
  let expanded = $state(false);
</script>

<section class="intent-section cosmic-section" aria-labelledby="intent-title">
  <div class="intent-nebula" aria-hidden="true"></div>
  <div class="section-copy intent-copy">
    <span class="section-index">ACT 03 / REAL FIXTURE → PLAN</span>
    <span class="kicker">CPIR → GRID → VALIDITY → PROPOSAL</span>
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
    <div class="intent-field fixture-field">
      <div class="trajectory-glow" aria-hidden="true"></div>
      <div class="intent-stage-summary">
        <div><small>CURRENT FIXTURE</small><strong>{formatTime(scopeStart)}–{formatTime(scopeEnd)} UTC · {request.duration.value.knowledge.data / 60} minute target · {request.granularity / 60} minute grid</strong></div>
        <div class="depth-technical"><small>ORDERING STATE</small><strong>{preferenceSource ? 'Preferred-start source: ' + preferenceSource : 'No preferred-start evidence; absent distance stays None in the feature contract.'}</strong></div>
      </div>

      <div class="intent-axis" aria-hidden="true"><span>STRUCTURED INPUT</span><i></i><span>DETERMINISTIC OUTPUT</span></div>
      <div class="intent-flow">
        <article class="intent-node">
          <span class="intent-planet cpir-orb" aria-hidden="true"></span>
          <small>01 · CPIR REQUEST</small>
          <strong>{request.operation} · {request.duration.value.knowledge.data}s</strong>
          <span>Scope {formatTime(scopeStart)}–{formatTime(scopeEnd)} UTC</span>
          <i class="node-state">structured</i>
        </article>
        <div class="intent-trace" aria-hidden="true"><i></i><span>enumerate</span><b></b></div>
        <article class="intent-node">
          <span class="intent-planet search-orb" aria-hidden="true"></span>
          <small>02 · DECLARED GRID</small>
          <strong>{evaluated} evaluated positions</strong>
          <span>{request.granularity / 60} minute granularity</span>
          <i class="node-state">bounded</i>
        </article>
        <div class="intent-trace" aria-hidden="true"><i></i><span>validate</span><b></b></div>
        <article class="intent-node">
          <span class="intent-planet" aria-hidden="true"></span>
          <small>03 · VALIDITY</small>
          <strong>{feasibleCount} valid · {rejectedCount} rejected</strong>
          <span>Known busy state removes overlaps</span>
          <i class="node-state">checked</i>
        </article>
        <div class="intent-trace" aria-hidden="true"><i></i><span>order</span><b></b></div>
        <article class="intent-node intent-resolved">
          <span class="intent-planet proof-orb" aria-hidden="true"></span>
          <small>04 · FIRST PROPOSAL</small>
          <strong>{winner ? formatTime(new Date(winner.start).getTime()) + ' UTC' : 'See result'}</strong>
          <span>{planner.assessment ?? 'Structured assessment'} · proposal, not execution</span>
          <i class="node-state">proposed</i>
        </article>
      </div>

      <PlanningInstrument />

      <div class="intent-boundary-note">
        <span>{planner.assessment ?? 'SEARCH ASSESSMENT'}</span>
        <p>
          {planner.search_space?.exhausted
            ? 'The declared grid was fully traversed. The assessment applies to this discrete grid and the current deterministic objective.'
            : 'The declared grid was not fully traversed, so the result is not a proof over every declared position.'}
        </p>
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
