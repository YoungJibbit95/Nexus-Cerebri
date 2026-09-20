<script lang="ts">
  import { runtimeData } from '$lib/generated/runtime-data';

  let { visual }: { visual: string } = $props();
  const request = runtimeData.request as any;
  const planner = runtimeData.plannerResult as any;
  const temporal = runtimeData.temporalResult as any;
  const candidateCount = planner.candidates?.length ?? 0;
  const firstRange = planner.candidates?.[0]?.proposed?.placements?.[0]?.range;
</script>

{#if visual === 'atlas'}
  <div class="atlas" aria-hidden="true">
    <div class="atlas-core">CEREBRI<small>coordinate field</small></div>
    <div class="atlas-node n1">Temporal<small>typed time</small></div>
    <div class="atlas-node n2">CPIR<small>evidence</small></div>
    <div class="atlas-node n3">Constraints<small>hard validity</small></div>
    <div class="atlas-node n4">Planner<small>bounded search</small></div>
    <div class="atlas-node n5">Validation<small>deterministic proof</small></div>
    <div class="atlas-node n6 future">Research<small>future learning</small></div>
  </div>
{:else if visual === 'cpir'}
  <div class="cpir-board">
    <div class="cpir-column"><small>IDENTITY</small><strong>{request.request_id}</strong><span>operation · {request.operation}</span><span>target · {request.target_ids?.[0]}</span></div>
    <div class="cpir-column"><small>KNOWLEDGE</small><strong>{request.context.objects?.[0]?.time?.value?.knowledge?.state}</strong><span>{request.context.objects?.[0]?.time?.provenance}</span><span>processing · {request.context.objects?.[0]?.time?.value?.processing}</span></div>
    <div class="cpir-column"><small>POLICY</small><strong>{request.policy.snapshot.mode}</strong><span>max mutations · {request.policy.snapshot.mutation.max_mutations}</span><span>confirm mutations · {String(request.policy.snapshot.confirmation.all_mutations)}</span></div>
    <div class="cpir-column"><small>BUDGET</small><strong>{request.budget.max_candidates} candidates</strong><span>granularity · {request.granularity}s</span><span>repairs · {request.budget.max_repairs}</span></div>
  </div>
{:else if visual === 'planning'}
  <div class="planning-field">
    <div class="timeline-labels"><span>09:00</span><span>10:00</span><span>11:00</span><span>12:00</span></div>
    <div class="timeline">
      <div class="busy-block" title="Known busy interval">busy</div>
      {#each Array(Math.min(candidateCount, 7)) as _, i}
        <div class="candidate" style={'--i:' + i}><span>{i + 1}</span></div>
      {/each}
    </div>
    <div class="planning-proof">
      <span>{planner.outcome ?? 'Solution'}</span><strong>{planner.assessment ?? 'Assessment'}</strong>
      <span>{candidateCount} feasible candidates</span><span>{firstRange ? firstRange.start : 'Candidate range from Rust output'}</span>
    </div>
  </div>
{:else if visual === 'time'}
  <div class="temporal-lens">
    <div class="time-card"><small>INTERVAL MODEL</small><strong>[start, end)</strong><span>Touching endpoints do not overlap.</span></div>
    <div class="time-card gap"><small>DST GAP</small><strong>02:30 → nonexistent</strong><span>Policy must reject or skip.</span></div>
    <div class="time-card fold"><small>DST FOLD</small><strong>02:30 → ambiguous</strong><span>Policy must reject, earlier or later.</span></div>
    <details class="raw-core-output"><summary>Rust temporal fixture output</summary><pre><code>{JSON.stringify(temporal, null, 2)}</code></pre></details>
  </div>
{:else if visual === 'safety'}
  <div class="proof-chain">
    {#each ['PlanningRequest', 'ProposedPlan', 'ValidatedPlan', 'ActionPlan', 'AuthorizedActionPlan', 'ExecutionResult'] as step, index}
      <div class:authorized={step === 'AuthorizedActionPlan'} class="proof-node"><small>{String(index + 1).padStart(2, '0')}</small><strong>{step}</strong></div>
      {#if index < 5}<span class="proof-arrow" aria-hidden="true">→</span>{/if}
    {/each}
  </div>
{:else if visual === 'architecture'}
  <div class="architecture-map">
    <div class="layer foundation"><strong>types</strong><strong>temporal</strong></div>
    <div class="layer middle"><strong>constraints</strong><strong>semantics</strong><strong>preferences</strong></div>
    <div class="layer planner-layer"><strong>planner</strong></div>
    <div class="layer facade"><strong>core facade</strong></div>
    <div class="layer transports"><strong>REST API</strong><strong>Node bridge</strong><strong>Site fixtures</strong></div>
  </div>
{:else if visual === 'roadmap'}
  <div class="roadmap-strata">
    <div class="stratum done"><span>0.1</span><strong>Foundation</strong><small>implemented</small></div>
    <div class="stratum done"><span>0.2</span><strong>Temporal Core</strong><small>implemented</small></div>
    <div class="stratum next"><span>0.3</span><strong>Deterministic Planner</strong><small>next target</small></div>
    <div class="stratum future"><span>later</span><strong>Providers · Repair · Learning · Neural</strong><small>planned / research</small></div>
  </div>
{:else if visual === 'developers'}
  <div class="developer-surface">
    <div class="code-window">
      <div class="window-bar"><span></span><span></span><span></span><small>transport boundary</small></div>
      <pre><code>POST /v1/validate
POST /v1/plan
POST /v1/temporal

// intentionally absent:
POST /v1/execute</code></pre>
    </div>
    <div class="transport-flow"><span>HTTP / Node</span><b>→</b><span>cerebri-core</span><b>→</b><span>typed Rust domain</span></div>
  </div>
{:else if visual === 'lab'}
  <div class="lab-handoff">
    <div class="lab-pane"><small>PLANNER</small><strong>Structured result</strong><span>candidates · cost · trace</span></div>
    <div class="lab-pane"><small>TEMPORAL</small><strong>Diagnostics</strong><span>DST · recurrence · free/busy</span></div>
    <div class="lab-pane muted"><small>ML / DATASET</small><strong>Reserved</strong><span>no fake backend</span></div>
  </div>
{/if}
