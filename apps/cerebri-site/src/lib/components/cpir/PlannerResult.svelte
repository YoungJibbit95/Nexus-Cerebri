<script lang="ts">
  import { base } from '$app/paths';
  import { planner, ranked, candidateIdentity, keyFields } from '$lib/planning-display';
  import RangeField from '../technical/RangeField.svelte';
  let { precision = 0 }: { precision?: number } = $props();
  const candidate = ranked[0];
  const placement = candidate.proposed.placements[0];
</script>

<section class="planner-result" aria-labelledby="result-title" data-schema="result">
  <header class="instrument-heading"><div><small>OUTPUT / RUST-GENERATED</small><h2 id="result-title">Planner result</h2></div><span>NEW COORDINATE SYSTEM</span></header>
  <dl class="result-evidence">
    <div><dt><code>outcome</code></dt><dd>{planner.outcome}</dd></div>
    <div><dt><code>assessment</code></dt><dd>{planner.assessment}</dd></div>
    <div><dt><code>search_space.evaluated</code></dt><dd>{planner.search_space.evaluated}</dd></div>
    <div><dt><code>search_space.exhausted</code></dt><dd>{String(planner.search_space.exhausted)}</dd></div>
  </dl>
  <div class="result-record" data-result-record="candidates[0]" data-candidate-id={candidateIdentity(candidate.start)}>
    <div class="candidate-coordinate"><strong>{candidateIdentity(candidate.start)}</strong><span>DISPLAY IDENTITY</span><code>candidates[0]</code><p>Display coordinate, not a Rust object ID.</p></div>
    <div class="placement">
      <RangeField label="Proposed placement" path="candidates[0].proposed.placements[0].range" start={placement.range.start} end={placement.range.end} {precision} kind="candidate" />
      <dl class="field-detail"><div><dt><code>candidates[0].proposed.placements[0].object_id</code></dt><dd>{placement.object_id}</dd></div></dl>
    </div>
  </div>
  <div class="result-structure" class:precise={precision > 0}>
    <section aria-labelledby="ordering-descent-title">
      <h3 id="ordering-descent-title">Ordering key</h3><p>Existing comparator dimensions → exact record. Rust order is retained.</p>
      <ol class="key-descent" aria-label="Candidate ordering key">
        {#each keyFields as field, index}
          <li data-key-field={field.key}><span>{String(index + 1).padStart(2, '0')}</span><div><strong>{field.label}</strong><code>candidates[0].ordering_key.{field.key}</code></div><b>{candidate.ordering_key[field.key]}</b></li>
        {/each}
      </ol>
    </section>
    <section aria-labelledby="features-descent-title">
      <h3 id="features-descent-title">Ranking observations</h3><p>Absent evidence remains null; the ordering projection remains a separate field.</p>
      <dl class="field-detail">
        {#each Object.entries(candidate.ranking_features) as [key, value]}
          <div><dt><code>candidates[0].ranking_features.{key}</code></dt><dd>{JSON.stringify(value)}</dd></div>
        {/each}
        <div><dt><code>conflicts.rejections</code> · entry count</dt><dd>{planner.conflicts.rejections.length} rejected positions</dd></div>
      </dl>
    </section>
  </div>
  <p class="lifecycle-note">ProposedPlan → separate lifecycle validation → ValidatedPlan. Execution still requires later authorization and preflight.</p>
  <a class="runtime-link" href={base + '/architecture/#runtime-planner'}><span>Result structure → runtime ownership</span><code>cerebri-planner →</code></a>
  <details><summary>Rust-generated result · raw data</summary><textarea readonly rows="12" aria-label="Planner result JSON" value={JSON.stringify(planner, null, 2)}></textarea></details>
</section>

<style>
  .planner-result { padding-top:40px; }
  .result-evidence { display:grid; grid-template-columns:repeat(4,minmax(0,1fr)); border-block:1px solid var(--line); padding:20px 0; gap:20px; }
  .result-evidence dd { font:18px/1.7 var(--mono); }
  .result-record { display:grid; grid-template-columns:190px minmax(0,1fr); gap:35px; padding:42px 0; }
  .candidate-coordinate { align-self:start; padding:8px 0 15px 24px; border-left:3px solid var(--field); }
  .candidate-coordinate strong { font:46px/1 var(--mono); color:var(--field); display:block; }
  .candidate-coordinate span { display:block; font:10px var(--mono); color:var(--muted); margin:13px 0; }
  .candidate-coordinate p { font-size:12px; }
  .result-structure { display:grid; grid-template-columns:1.25fr 1fr; gap:40px; }
  .result-structure p { font-size:12px; min-height:40px; }
  .key-descent { margin:20px 0 0; padding:0; list-style:none; }
  .key-descent li { display:grid; grid-template-columns:22px minmax(0,1fr); gap:3px 10px; padding:13px 0; border-top:1px solid var(--line); }
  .key-descent li>span { font:11px var(--mono); color:var(--muted); }
  .key-descent strong { font-size:12px; font-weight:500; }
  .key-descent code { display:none; font-size:10px; }
  .precise .key-descent code { display:block; }
  .key-descent b { grid-column:2; font:13px/1.7 var(--mono); color:#e4f5f6; overflow-wrap:anywhere; }
  .lifecycle-note { margin:32px 0; padding-left:20px; border-left:1px dashed #ecd6a1; font-size:12px; }
  @media(max-width:700px) {
    .result-evidence { grid-template-columns:1fr 1fr; }
    .result-evidence dd { font-size:14px; }
    .result-record,.result-structure { grid-template-columns:1fr; gap:26px; }
    .candidate-coordinate { display:grid; grid-template-columns:85px 1fr; }
    .candidate-coordinate>code { grid-column:1/-1; }
    .candidate-coordinate p { grid-column:1/-1; margin-bottom:0; }
  }
</style>
