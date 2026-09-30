<script lang="ts">
  import { time } from '../lib/presentation.ts';
  import { firstDifference, orderingFields, visibleCandidates } from '../lib/planner-presentation.ts';
  import type { ExplanationMode, PlanningResult } from '../lib/contracts.ts';
  let { result, selected, onselect, mode }: { result: PlanningResult | null; selected: number; onselect: (index: number) => void; mode: ExplanationMode } = $props();
  const candidates = $derived(visibleCandidates(result, selected));
  const otherIndex = $derived(selected === 0 ? 1 : 0);
  const current = $derived(result?.candidates[selected]);
  const other = $derived(result?.candidates[otherIndex]);
  const difference = $derived(current && other ? firstDifference(current, other) : -1);
  const ceiling = $derived(Math.max(1, ...candidates.map(({ candidate }) => candidate.ordering_key.preference_distance_seconds)));
</script>
<section class="panel comparison-instrument" aria-labelledby="score-title" data-selected-candidate={selected + 1}>
  <div class="panel-heading"><div><span class="eyebrow">COMPARE / FIRST DIFFERENCE DECIDES</span><h2 id="score-title">Why this order?</h2></div><span class="badge subtle">Lexicographic</span></div>
  {#if current}
    <p class="comparison-explanation">{#if other && difference >= 0}Candidate {selected === 0 ? 1 : otherIndex + 1} ranks before candidate {selected === 0 ? 2 : selected + 1}: {orderingFields[difference].explanation.toLowerCase()}.{#if mode !== 'Simple'} <strong>{orderingFields[difference].label}: {selected === 0 ? current.ordering_key[orderingFields[difference].key] : other.ordering_key[orderingFields[difference].key]} vs {selected === 0 ? other.ordering_key[orderingFields[difference].key] : current.ordering_key[orderingFields[difference].key]}.</strong>{/if}{:else if other}The complete returned ordering keys tie. No additional deciding rule is inferred.{:else}One feasible candidate was returned. There is no second candidate to compare.{/if}</p>
    {#if mode === 'Simple'}<div class="distance-comparison" role="group" aria-label="Candidate distance from preferred time">{#each candidates as { candidate, index }}<button class:active={selected === index} aria-pressed={selected === index} onclick={() => onselect(index)} aria-label={'Compare candidate ' + (index + 1)}><strong>#{index + 1}<small>{time(candidate.start)} UTC</small></strong><span class="distance-track"><i style={'width:' + (candidate.ordering_key.preference_distance_seconds / ceiling * 100) + '%'}></i></span><span>{candidate.ranking_features.preferred_start_distance_seconds === null ? 'No preference' : candidate.ranking_features.preferred_start_distance_seconds / 60 + ' min away'}{#if (index === selected || index === otherIndex) && difference === 0}<small>◆ first difference</small>{/if}</span></button>{/each}</div>{:else}
    <div class="comparison-scroll"><div class="comparison-grid" role="group" aria-label="Returned candidate ordering keys">
      <div class="comparison-header"><span>Candidate</span>{#each orderingFields as field, index}<span class:deciding={index === difference}>{field.label}{#if index === difference}<small>FIRST DIFFERENCE</small>{/if}</span>{/each}</div>
      {#each candidates as { candidate, index }}<button class="comparison-row" class:active={index === selected} aria-pressed={index === selected} onclick={() => onselect(index)} aria-label={'Compare candidate ' + (index + 1)}><strong>#{index + 1}<small>{time(candidate.start)} UTC</small></strong>{#each orderingFields as field, fieldIndex}<span class:deciding={fieldIndex === difference && (index === selected || index === otherIndex)}>{#if field.key === 'start'}{mode === 'Research' ? candidate.ordering_key.start : time(candidate.ordering_key.start)}{:else if field.key === 'object_id'}{candidate.ordering_key.object_id}{:else}{candidate.ordering_key[field.key]}{field.key !== 'mutation_count' ? ' s' : ''}{/if}{#if fieldIndex === difference && (index === selected || index === otherIndex)}<small>◆ decides</small>{/if}</span>{/each}</button>{/each}
    </div></div>{/if}
    <p class="fine-print">Read left to right. The first unequal field decides; later fields break ties. These are the Rust-produced ordering keys. No weighted total is computed.</p>
    {#if current.ranking_features.preferred_start_distance_seconds === null}<p class="fine-print">No preferred-time evidence: the ordering distance is zero. Zero does not imply a supplied preference.</p>{:else if mode !== 'Simple'}<p class="fine-print">Distance uses the core's whole-second projection. Zero can include a subsecond distance; source is provenance and does not rank.</p>{/if}
  {:else}<div class="chart-empty"><p>{result ? 'No feasible candidates to compare.' : 'Feasible candidates will be compared here.'}</p><small>Actual ordering keys only</small></div>{/if}
</section>
