<script lang="ts">
  import { runtimeData } from '$lib/generated/runtime-data';
  import { plannerSemanticLegend } from '$lib/visual-grammar';
  import AuthorityRail from './AuthorityRail.svelte';
  import SemanticLegend from './SemanticLegend.svelte';
  import SemanticMark from './SemanticMark.svelte';

  const planner = runtimeData.plannerResult as any;
  const request = runtimeData.request as any;
  const candidates = (planner.candidates ?? []) as any[];
  const rejections = (planner.conflicts?.rejections ?? []) as any[];
  const scopeStart = new Date(request.scope.time_range.start).getTime();
  const scopeEnd = new Date(request.scope.time_range.end).getTime();
  const busy = request.context.objects?.find((object: any) => object.id === 'busy')?.time?.value?.knowledge?.data;
  const winner = candidates[0];
  const runnerUp = candidates[1];

  const formatTime = (value: string | number) =>
    new Date(value).toLocaleTimeString('en-GB', { hour: '2-digit', minute: '2-digit', timeZone: 'UTC' });

  const evaluatedPositions = [
    ...rejections.map((entry: any) => ({
      instant: new Date(entry.start).getTime(),
      state: 'rejected',
      candidate: null
    })),
    ...candidates.map((candidate: any, index: number) => ({
      instant: new Date(candidate.start).getTime(),
      state: index === 0 ? 'selected' : 'valid',
      candidate
    }))
  ].sort((a, b) => a.instant - b.instant);

  const keyFields: Array<{ key: string; label: string; render: (value: any) => string }> = [
    { key: 'preference_distance_seconds', label: 'preferred distance', render: (value) => String(value) + 's' },
    { key: 'mutation_count', label: 'mutation count', render: (value) => String(value) },
    { key: 'shifted_seconds', label: 'shift seconds', render: (value) => String(value) + 's' },
    { key: 'start', label: 'start', render: (value) => formatTime(value) },
    { key: 'object_id', label: 'object id', render: (value) => String(value) }
  ];

  const decisiveField = keyFields.find((field) =>
    winner && runnerUp && String(winner.ordering_key?.[field.key]) !== String(runnerUp.ordering_key?.[field.key])
  )?.key ?? null;

  const preferenceSource = winner?.ranking_features?.preferred_start_source ?? null;
  const evaluated = Number(planner.search_space?.evaluated ?? evaluatedPositions.length);
  const feasible = candidates.length;
  const rejected = rejections.length;
</script>

<section class="planning-workbench" aria-label="Planning workbench" aria-labelledby="planning-workbench-title">
  <header class="workbench-header">
    <div>
      <small>PLANNING WORKBENCH / RUST-GENERATED FIXTURE</small>
      <h3 id="planning-workbench-title">From evidence to one deterministic first proposal.</h3>
      <p>Every validity and ordering state below comes from the generated planner result. The browser formats that evidence; it does not decide which candidate wins.</p>
    </div>
    <div class="workbench-counts"><span class="workbench-coordinate">{evaluated} evaluated positions</span><span class="workbench-summary">{feasible} valid / {rejected} rejected</span></div>
  </header>

  <SemanticLegend items={plannerSemanticLegend} compact label="Planning visual grammar" />

  <div class="workbench-layout">
    <aside class="workbench-inputs" aria-label="Structured planning evidence">
      <header><small>01 / EVIDENCE</small><strong>What enters search</strong></header>
      <SemanticMark kind="scope" label="Planning scope" detail={`${formatTime(scopeStart)}–${formatTime(scopeEnd)} UTC`} />
      <SemanticMark
        kind="fact"
        label="Known busy state"
        detail={busy ? `${formatTime(busy.start)}–${formatTime(busy.end)} UTC` : 'No busy interval in fixture'}
      />
      <SemanticMark
        kind="constraint"
        label="Explicit hard constraints"
        detail={(request.constraints?.length ?? 0) === 0 ? 'None supplied in this fixture' : `${request.constraints.length} supplied`}
      />
      <SemanticMark
        kind="preference"
        label="Preferred-start evidence"
        detail={preferenceSource ? String(preferenceSource) : 'None · nullable feature remains None'}
       />
      <div class="workbench-input-note">
        <b>Validity still uses known state.</b>
        <span>Supplied Event/Task time blocks overlap even when this fixture carries no explicit hard-constraint entry.</span>
      </div>
    </aside>

    <div class="workbench-candidate-field">
      <header>
        <div><small>02 / CANDIDATE FIELD</small><strong>Rust-evaluated starts</strong></div>
        <span>{request.granularity / 60} min grid</span>
      </header>
      <div class="candidate-axis" aria-hidden="true"><span>{formatTime(scopeStart)}</span><i></i><span>{formatTime(scopeEnd)}</span></div>
      <ol class="semantic-candidate-grid" aria-label="Candidate validity states from the Rust planner result">
        {#each evaluatedPositions as position, index}
          <li data-candidate-state={position.state}>
            <span class="candidate-index">{String(index + 1).padStart(2, '0')}</span>
            <SemanticMark
              kind={position.state === 'rejected' ? 'violation' : position.state === 'selected' ? 'result' : 'candidate'}
              label={formatTime(position.instant)}
              detail={position.state === 'rejected'
                ? 'rejected by Rust validity checks'
                : position.state === 'selected'
                  ? 'first in Rust-produced order'
                  : 'valid candidate'}
              compact
            />
          </li>
      {/each}
      </ol>
    </div>

    <aside class="workbench-comparator" aria-label="Deterministic comparator">
      <header>
        <small>03 / DETERMINISTIC COMPARATOR</small>
        <strong>First two Rust-ranked candidates</strong>
        <p>The rows are the current lexicographic ordering key. Highlighting shows the first field where these already-ranked candidates differ.</p>
      </header>

      {#if winner && runnerUp}
        <div class="comparator-head">
          <span>KEY</span><span>{formatTime(winner.start)}</span><span>{formatTime(runnerUp.start)}</span>
        </div>
        <div class="comparator-table">
          {#each keyFields as field, index}
            <div class:decisive={field.key === decisiveField} class="comparator-row" data-key-field={field.key}>
              <span><b>{String(index + 1).padStart(2, '0')}</b>{field.label}</span>
              <strong>{field.render(winner.ordering_key?.[field.key])}</strong>
              <strong>{field.render(runnerUp.ordering_key?.[field.key])}</strong>
            </div>
          {/each}
        </div>
        <div class="comparator-resolution">
          <SemanticMark kind="result" label={`${formatTime(winner.start)} UTC stays first`} detail={decisiveField ? `first differing key: ${keyFields.find((field) => field.key === decisiveField)?.label}` : 'keys are equal'} />
        </div>
      {:else}
        <p class="workbench-empty">The current result does not contain two ranked candidates to compare.</p>
      {/if}
    </aside>
  </div>

  <footer class="workbench-outcome">
    <div class="workbench-proof">
      <SemanticMark kind="result" label={planner.assessment ?? 'Search assessment'} detail="bounded claim over this declared search" />
      <p>{planner.search_space?.exhausted
        ? 'The declared grid was exhausted. This assessment does not claim global continuous-time optimality.'
        : 'The declared grid was not exhausted; the assessment is correspondingly weaker.'}</p>
    </div>
    <div class="workbench-authority">
      <div><small>04 / AUTHORITY</small><strong>The planner stops at a proposal.</strong></div>
      <AuthorityRail currentStage="proposal" compact label="The current fixture produces a proposal; later authority states are separate" />
    </div>
  </footer>
</section>
