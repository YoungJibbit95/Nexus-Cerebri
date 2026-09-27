<script lang="ts">
  import { mathData, orderingDimensions, secondsLabel, timeLabel } from '$lib/math-inspection';
  import { runtimeData } from '$lib/generated/runtime-data';
  import SourceLink from '../SourceLink.svelte';
  import { candidateIdentity } from '$lib/planning-display';
  import { base } from '$app/paths';
  import RangeField from '../technical/RangeField.svelte';
  import '../technical/technical.css';

  const absent = mathData.measurement.absent;
  const zero = mathData.measurement.samples[1];
  const candidates = runtimeData.plannerResult.candidates.slice(0, 2);
  let compact = $state(false);
  let selected = $state(0);
  const dimension = $derived(orderingDimensions[selected]);
</script>

<details class="technical-inspection math-disclosure" data-world="technical">
  <summary><span>Technical inspection</span><small>Observation → projection → ordering tuple</small></summary>
  <div class="technical-content">
    <section class="observation-projection" aria-labelledby="observation-title">
      <header class="chapter-heading"><span class="chapter-number">05</span><div><small>CURRENT / FEATURE CONTRACT 0.1</small><h3 id="observation-title">The same projection can hide different evidence.</h3></div></header>
      <div class="projection-paths">
        <div class="observation" data-observation="absent"><span class="absence-symbol" aria-hidden="true">∅</span><div><strong>None</strong><p>Preferred-start evidence absent.</p><code>distance = null</code><code>source = null</code></div></div>
        <div class="projection-connector absent-path" aria-hidden="true"><i></i></div>
        <div class="projection-value"><small>LEGACY ORDERING PROJECTION</small><strong>{absent.ordering_projection}</strong><code>preference_distance_seconds</code></div>
        <div class="observation" data-observation="zero"><span class="presence-symbol" aria-hidden="true">●</span><div><strong>Some({zero.ranking_features.preferred_start_distance_seconds})</strong><p>Preferred-start evidence present with zero measured distance.</p><code>actual difference = {secondsLabel(zero.actual_delta_ms)} s</code><code>source = {zero.ranking_features.preferred_start_source}</code></div></div>
        <div class="projection-connector present-path" aria-hidden="true"><i></i></div>
      </div>
      <p>Both project to {zero.ordering_projection}. Their observations remain different. Some(0) does not require equal instants.</p>
    </section>
    <section class="tuple-inspection" aria-labelledby="tuple-title" data-compact={compact}>
      <header class="chapter-heading"><span class="chapter-number">06</span><div><small>CURRENT / HOMEPAGE CANDIDATES</small><h3 id="tuple-title">The comparator, in compact notation.</h3></div></header>
      <p>The same first two Rust-ranked candidates shown above. Read left to right; the first differing dimension decides. No aggregate score.</p>
      <div class="tuple-dimensions" role="group" aria-label="Inspect an ordering dimension">{#each orderingDimensions as field, index}
        <button data-key-field={field.key} aria-pressed={selected === index} onclick={() => selected = index}><span>{index + 1}</span>{field.label}<b>{field.symbol}</b></button>
      {/each}</div>
      <div class="dimension-reading"><code>{dimension.key}</code><p>{dimension.description}</p></div>
      <button class="notation-toggle" aria-pressed={compact} onclick={() => compact = !compact}>{compact ? 'Expand tuple dimensions' : 'Compress dimensions into tuples'}</button>
      <div class="tuple-correspondence" aria-hidden="true">(d, m, s, t, id) ↔ (preferred distance, mutations, shift, start, object ID)</div>
      {#each candidates as candidate, index}
        <div class="tuple-record">
          <small>{candidateIdentity(candidate.start)} · {timeLabel(candidate.start)} UTC</small>
          <ol class="ordering-tuple" aria-label={'Ordering tuple for candidate ' + (index + 1)}>{#each orderingDimensions as field, fieldIndex}
            <li class:term-selected={selected === fieldIndex} data-key-field={field.key}><span class="tuple-field">{field.key}</span><strong>{candidate.ordering_key[field.key]}</strong></li>
          {/each}</ol>
        </div>
      {/each}
      <p>For this no-preference fixture, the first difference is <code>start</code>. These values preserve the existing comparator's order.</p>
      <SourceLink path="crates/cerebri-planner/src/search.rs" label="CandidateOrderingKey field order" />
    </section>
    <SourceLink path="docs/architecture/decisions/ADR-0014-ranking-feature-contract.md" label="Observation and projection contract" />
    <nav class="technical-instrument technical-descent-bridge" aria-label="Continue technical descent">
      <RangeField label="The same scope, as an exact field" path="scope.time_range" start={runtimeData.request.scope.time_range.start} end={runtimeData.request.scope.time_range.end} precision={1} />
      <a class="runtime-link" href={base + '/cpir/#scope-field'}><span>Scope geometry → CPIR input</span><span>Inspect evidence →</span></a>
      <a class="runtime-link" href={base + '/cpir/#result-title'}><span>{candidateIdentity(candidates[0].start)} → planner result record</span><code>candidates[0] →</code></a>
    </nav>
  </div>
</details>

<style>
  .technical-descent-bridge { margin-top:32px; padding-top:30px; border-top:1px solid rgba(112,179,215,.28); }
</style>
