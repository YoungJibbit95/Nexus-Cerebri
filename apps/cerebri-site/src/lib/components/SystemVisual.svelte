<script lang="ts">
  import { runtimeData } from '$lib/generated/runtime-data';

  let { visual }: { visual: string } = $props();
  const request = runtimeData.request as any;
  const planner = runtimeData.plannerResult as any;
  const temporal = runtimeData.temporalResult as any;
  const candidates = (planner.candidates ?? []).slice(0, 7) as any[];
  const candidateCount = planner.candidates?.length ?? 0;
  const firstRange = planner.candidates?.[0]?.proposed?.placements?.[0]?.range;
  const scopeStart = new Date(request.scope.time_range.start).getTime();
  const scopeEnd = new Date(request.scope.time_range.end).getTime();
  const scopeSpan = scopeEnd - scopeStart;
  const busy = request.context.objects?.find((object: any) => object.id === 'busy')?.time?.value?.knowledge?.data;

  const pct = (value: string) => Math.max(0, Math.min(100, ((new Date(value).getTime() - scopeStart) / scopeSpan) * 100));
  const widthPct = (start: string, end: string) => Math.max(1, ((new Date(end).getTime() - new Date(start).getTime()) / scopeSpan) * 100);
  const time = (value: string) => new Date(value).toLocaleTimeString('en-GB', { hour: '2-digit', minute: '2-digit', timeZone: 'UTC' });
</script>

{#if visual === 'atlas'}
  <div class="atlas system-map" aria-hidden="true">
    <div class="atlas-grid"></div>
    <svg class="atlas-links" viewBox="0 0 1000 610" preserveAspectRatio="none">
      <path d="M500 306 C390 260 305 175 235 105" />
      <path d="M500 306 C615 255 702 170 777 112" />
      <path d="M500 306 C648 312 745 315 865 318" />
      <path d="M500 306 C615 382 675 455 732 528" />
      <path d="M500 306 C385 390 323 458 255 528" />
      <path class="atlas-trace" d="M235 105 C305 175 390 260 500 306 C648 312 745 315 865 318" />
      <path class="atlas-trace atlas-trace-b" d="M255 528 C323 458 385 390 500 306 C615 255 702 170 777 112" />
    </svg>
    <div class="verified-boundary"><span>VERIFIED CORE BOUNDARY</span></div>
    <div class="atlas-core"><small>00 / AUTHORITY</small><strong>CEREBRI</strong><span>coordinate field</span><i></i></div>
    <div class="atlas-node n1"><b>01</b><strong>Temporal</strong><small>typed time</small></div>
    <div class="atlas-node n2"><b>02</b><strong>CPIR</strong><small>evidence</small></div>
    <div class="atlas-node n3"><b>03</b><strong>Constraints</strong><small>hard validity</small></div>
    <div class="atlas-node n4"><b>04</b><strong>Planner</strong><small>bounded search</small></div>
    <div class="atlas-node n5"><b>05</b><strong>Validation</strong><small>deterministic proof</small></div>
    <div class="atlas-node n6 future"><b>R</b><strong>Research</strong><small>future learning</small><em>outside core</em></div>
    <span class="map-coordinate map-x">X / ownership</span><span class="map-coordinate map-y">Y / authority</span>
  </div>
{:else if visual === 'planning'}
  <div class="planning-field">
    <div class="planning-header"><div><small>SEARCH SPACE</small><strong>{time(request.scope.time_range.start)} → {time(request.scope.time_range.end)} UTC</strong><span>The planner may only choose inside this window.</span></div><div><small>ASSESSMENT</small><strong>{planner.assessment ?? 'Assessment'}</strong><span class="technical-only">Rust planner claim for this bounded run</span></div></div>
    <div class="planning-legend" aria-label="Planning field legend"><span><i class="legend-busy"></i>blocked</span><span><i class="legend-candidate"></i>feasible candidate</span><span><i class="legend-selected"></i>selected result</span><span class="technical-only"><i class="legend-grid"></i>discrete grid</span></div>
    <div class="search-ruler"><span>09:00</span><span>10:00</span><span>11:00</span><span>12:00</span></div>
    <div class="search-grid">
      <div class="grid-lines"></div>
      <svg class="planning-trajectories" viewBox="0 0 1000 330" preserveAspectRatio="none" aria-hidden="true"><path d="M20 275 C180 95 360 80 520 184 S760 310 980 84"/><path d="M80 300 C250 210 380 230 520 150 S760 38 940 138"/></svg>
      {#if busy}<div class="busy-block" style={`left:${pct(busy.start)}%;width:${widthPct(busy.start,busy.end)}%`}><span>KNOWN BUSY</span><small>{time(busy.start)}—{time(busy.end)}</small></div>{/if}
      {#each candidates as candidate, i}
        {@const range = candidate.proposed?.placements?.[0]?.range}
        {#if range}
          <div class:selected={i === 0} class="candidate" style={`--i:${i};left:${pct(range.start)}%;width:${widthPct(range.start,range.end)}%`}><span>C{i + 1}</span><small>{time(range.start)}</small></div>
        {/if}
      {/each}
      <div class="search-scan" aria-hidden="true"></div>
    </div>
    <div class="planning-proof">
      <div><small>OUTCOME</small><strong>{planner.outcome ?? 'Solution'}</strong></div>
      <div><small>FEASIBLE</small><strong>{candidateCount}</strong><span>candidates remain valid</span></div>
      <div class="proof-selected"><small>SELECTED</small><strong>{firstRange ? time(firstRange.start) + ' UTC' : 'Rust result'}</strong><span>chosen deterministically</span></div>
      <div><small>AUTHORITY</small><strong>Rust core output</strong><span>rendered, not recomputed</span></div>
    </div>
  </div>
{:else if visual === 'time'}
  <div class="temporal-lens">
    <div class="temporal-axis"><span>UTC INSTANT</span><i></i><span>LOCAL INTERPRETATION</span></div>
    <div class="time-model half-open">
      <div class="time-rail"><i class="point closed"></i><span></span><i class="point open"></i></div>
      <small>01 / INTERVAL MODEL</small><strong>[start, end)</strong><p>Touching endpoints do not overlap.</p>
    </div>
    <div class="time-model gap">
      <div class="time-rail broken"><i></i><span></span><b>GAP</b><span></span><i></i></div>
      <small>02 / DST GAP</small><strong>02:30 → nonexistent</strong><p>Policy must reject or skip.</p>
    </div>
    <div class="time-model fold">
      <div class="fold-path"><span></span><i></i><span></span></div>
      <small>03 / DST FOLD</small><strong>02:30 → ambiguous</strong><p>Policy must reject, earlier or later.</p>
    </div>
    <details class="raw-core-output"><summary><span>Rust temporal fixture output</span><i>+</i></summary><pre><code>{JSON.stringify(temporal, null, 2)}</code></pre></details>
  </div>
{:else if visual === 'safety'}
  <div class="proof-pipeline">
    <div class="proof-axis"><span>LESS BOUND</span><i></i><span>MORE BOUND</span></div>
    <div class="proof-chain">
      {#each ['PlanningRequest', 'ProposedPlan', 'ValidatedPlan', 'ActionPlan', 'AuthorizedActionPlan', 'ExecutionResult'] as step, index}
        <div class:authorized={step === 'AuthorizedActionPlan'} class:execution={step === 'ExecutionResult'} class="proof-node">
          <small>{String(index + 1).padStart(2, '0')}</small><span class="seal"></span><strong>{step}</strong>
          <em>{step === 'ValidatedPlan' ? 'valid ≠ permitted' : step === 'AuthorizedActionPlan' ? 'permission bound' : step === 'ExecutionResult' ? 'provider outcome' : 'typed state'}</em>
        </div>
        {#if index < 5}<span class="proof-arrow" aria-hidden="true"><i></i></span>{/if}
      {/each}
    </div>
    <div class="executor-boundary"><span>PLANNER</span><i></i><b>EXECUTOR BOUNDARY</b><i></i><span>EXECUTION</span></div>
  </div>
{:else if visual === 'roadmap'}
  <div class="roadmap-strata">
    <div class="roadmap-nebula" aria-hidden="true"></div>
    <div class="roadmap-axis"><span>VERIFIED FOUNDATION</span><i></i><span>RESEARCH HORIZON</span></div>
    <div class="stratum done"><span>0.1</span><div><strong>Foundation</strong><small>implemented</small></div><i></i></div>
    <div class="stratum done"><span>0.2</span><div><strong>Temporal Core</strong><small>implemented</small></div><i></i></div>
    <div class="stratum next"><span>0.3</span><div><strong>Deterministic Planner</strong><small>verification implemented · release qualification in progress</small></div><i></i></div>
    <div class="stratum future"><span>later</span><div><strong>Providers · Repair · Learning · Neural</strong><small>planned / research — not current features</small></div><i></i></div>
  </div>
{:else if visual === 'developers'}
  <div class="developer-surface">
    <div class="developer-signal" aria-hidden="true"><i></i><span></span></div>
    <div class="code-window">
      <div class="window-bar"><span>API / V1</span><small>transport boundary</small></div>
      <pre><code><span class="code-method">POST</span> /v1/validate
<span class="code-method">POST</span> /v1/plan
<span class="code-method">POST</span> /v1/temporal

<span class="code-comment">// intentionally absent:</span>
<span class="code-absent">POST /v1/execute</span></code></pre>
    </div>
    <div class="transport-flow">
      <div><small>01 / ADAPTER</small><span>HTTP / Node</span></div><b aria-hidden="true">→</b>
      <div class="core-step"><small>02 / AUTHORITY</small><span>cerebri-core</span></div><b aria-hidden="true">→</b>
      <div><small>03 / DOMAIN</small><span>typed Rust domain</span></div>
      <p>Transport delegates. Domain truth does not move outward.</p>
    </div>
  </div>
{:else if visual === 'lab'}
  <div class="lab-handoff">
    <div class="lab-bridge"><span>OFFICIAL SITE</span><i></i><span>SEPARATE LAB WORKSPACE</span></div>
    <div class="lab-pane"><small>01 / PLANNER</small><strong>Structured result</strong><span>candidates · cost · trace</span><i>REAL</i></div>
    <div class="lab-pane"><small>02 / TEMPORAL</small><strong>Diagnostics</strong><span>DST · recurrence · free/busy</span><i>REAL</i></div>
    <div class="lab-pane muted future"><small>03 / ML + DATASET</small><strong>Reserved</strong><span>no fake inference · no fake training</span><i>FUTURE</i></div>
  </div>
{/if}
