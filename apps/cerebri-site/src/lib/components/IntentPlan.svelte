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
  const visualAlternative = 'The planner checks ' + evaluated + ' possible start times for a ' + request.duration.value.knowledge.data / 60 + '-minute appointment. ' + rejectedCount + ' overlap the existing appointment and are rejected. ' + feasibleCount + ' pass the checks. The first option becomes a proposal; nothing is booked.';
  let expanded = $state(false);
</script>

<section data-world="planning" class="intent-section cosmic-section" aria-labelledby="intent-title">
  <div class="intent-nebula" aria-hidden="true"></div>
  <div class="section-copy intent-copy">
    <span class="section-index">AN EXAMPLE YOU CAN CHECK</span>
    <span class="kicker">30 MINUTES BETWEEN 09:00 AND 12:00 UTC</span>
    <h2 id="intent-title">Where can it fit?<br /><em>{evaluated} possible starts.</em></h2>
    <p>
      The planner checks start times every {request.granularity / 60} minutes.
      Starts that overlap the existing 09:00–10:00 appointment are rejected.
      The last possible start is 11:30, so the full {request.duration.value.knowledge.data / 60} minutes fit before 12:00.
      The results below come from running the Rust planner on this example’s structured input.
    </p>
  </div>

  <VisualizationFrame
    kind="REAL"
    label="A 30-minute appointment: input and result"
    caption="The website build runs the Rust planner on examples/request.json (CPIR 0.2). The browser displays that result."
    textAlternative={visualAlternative}
  >
    <div class="intent-field fixture-field">
      <div class="trajectory-glow" aria-hidden="true"></div>
      <div class="intent-stage-summary">
        <div><small>EXAMPLE INPUT</small><strong>{formatTime(scopeStart)}–{formatTime(scopeEnd)} UTC · {request.duration.value.knowledge.data / 60}-minute appointment · starts every {request.granularity / 60} minutes</strong></div>
        <div class="depth-technical"><small>PREFERRED START</small><strong>{preferenceSource ? 'Preference source: ' + preferenceSource : 'None supplied. The observation records missing distance as None.'}</strong></div>
      </div>

      <div class="intent-axis" aria-hidden="true"><span>APPOINTMENT DETAILS</span><i></i><span>PLANNING RESULT</span></div>
      <div class="intent-flow">
        <article class="intent-node">
          <span class="intent-planet cpir-orb" aria-hidden="true"></span>
          <small>01 · NEW APPOINTMENT</small>
          <strong>{request.duration.value.knowledge.data / 60} minutes</strong>
          <span>Between {formatTime(scopeStart)} and {formatTime(scopeEnd)} UTC</span>
          <i class="node-state">CPIR input</i>
        </article>
        <div class="intent-trace" aria-hidden="true"><i></i><span>list starts</span><b></b></div>
        <article class="intent-node">
          <span class="intent-planet search-orb" aria-hidden="true"></span>
          <small>02 · POSSIBLE STARTS</small>
          <strong>{evaluated} times checked</strong>
          <span>One start every {request.granularity / 60} minutes</span>
          <i class="node-state">within window</i>
        </article>
        <div class="intent-trace" aria-hidden="true"><i></i><span>check</span><b></b></div>
        <article class="intent-node">
          <span class="intent-planet" aria-hidden="true"></span>
          <small>03 · CHECK FOR CONFLICTS</small>
          <strong>{feasibleCount} valid · {rejectedCount} rejected</strong>
          <span>Overlapping times are rejected</span>
          <i class="node-state">checked</i>
        </article>
        <div class="intent-trace" aria-hidden="true"><i></i><span>compare</span><b></b></div>
        <article class="intent-node intent-resolved">
          <span class="intent-planet proof-orb" aria-hidden="true"></span>
          <small>04 · PROPOSED START</small>
          <strong>{winner ? formatTime(new Date(winner.start).getTime()) + ' UTC' : 'See result'}</strong>
          <span>A proposal; nothing has been booked</span>
          <i class="node-state">proposed</i>
        </article>
      </div>

      <PlanningInstrument />

      <div class="intent-boundary-note">
        <span>{planner.assessment ?? 'SEARCH ASSESSMENT'}</span>
        <p>
          {planner.search_space?.exhausted
            ? 'Every start in this grid was checked. The first proposal ranks best under the stated comparison rules. This claim applies only to this grid and these rules.'
            : 'The search stopped before checking every start in the grid. A better option may remain among the unchecked times.'}
        </p>
      </div>
    </div>

    <button class="inspect-button" aria-expanded={expanded} onclick={() => (expanded = !expanded)}>
      <span>{expanded ? 'Hide input and result data' : 'Show input and result data'}</span><i aria-hidden="true">{expanded ? '−' : '+'}</i>
    </button>
    {#if expanded}
      <div class="evidence-grid">
        <div><small>SEARCH LIMITS / CPIR INPUT</small><pre><code>{JSON.stringify(request.scope, null, 2)}</code></pre></div>
        <div><small>FIRST CANDIDATE / RUST RESULT</small><pre><code>{JSON.stringify(planner.candidates?.[0] ?? planner, null, 2)}</code></pre></div>
      </div>
    {/if}
  </VisualizationFrame>
</section>
