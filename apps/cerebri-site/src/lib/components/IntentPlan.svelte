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

<section data-world="planning" class="intent-section cosmic-section" aria-labelledby="intent-title">
  <div class="intent-nebula" aria-hidden="true"></div>
  <div class="section-copy intent-copy">
    <span class="section-index">CONTINUE THE SEARCH / INSPECT THE RESULT</span>
    <span class="kicker">VALIDITY → ORDERING → PROPOSAL</span>
    <h2 id="intent-title">From possible times.<br /><em>To one proposal.</em></h2>
    <p>
      Keep the same {evaluated} positions in view. First inspect a conflict, then compare two valid candidates.
      Every classification and ordering key below is a result from the Rust planner.
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

      <dl class="fixture-readout" aria-label="Planning fixture overview">
        <div><dt>Request</dt><dd>{request.operation} · {request.duration.value.knowledge.data / 60} minutes</dd></div>
        <div><dt>Evaluated</dt><dd>{evaluated} positions</dd></div>
        <div><dt>Result</dt><dd>{feasibleCount} valid · {rejectedCount} rejected</dd></div>
        <div><dt>First proposal</dt><dd>{winner ? formatTime(new Date(winner.start).getTime()) + ' UTC' : 'See result'}</dd></div>
      </dl>

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

<style>
  .fixture-readout{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:1.5rem;margin:1.5rem 0 0;padding:1.5rem 0;border-block:1px solid var(--line)}.fixture-readout dt{font:.8125rem var(--mono);color:var(--text-3);margin-bottom:.75rem}.fixture-readout dd{margin:0;font-size:1.125rem;font-weight:600;line-height:1.5}.fixture-readout>div:last-child dd{color:var(--cyan)}
  @media(max-width:760px){.fixture-readout{grid-template-columns:1fr 1fr;gap:1.5rem 1rem}.fixture-readout dd{font-size:1rem}}
  @media(max-width:350px){.fixture-readout{grid-template-columns:1fr}}
</style>
