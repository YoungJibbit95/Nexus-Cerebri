<script lang="ts">
  import JsonPanel from './JsonPanel.svelte';
  import { reasonLabel, readable, time } from '../lib/presentation.ts';
  import type { RankedCandidate, ExplanationMode } from '../lib/contracts.ts';
  let { candidate, rank, mode, hasResult = false }: { candidate: RankedCandidate | undefined; rank: number; mode: ExplanationMode; hasResult?: boolean } = $props();
</script>
<section class="panel proposal-inspector" aria-labelledby="candidate-title" data-selected-candidate={rank}>
  <div class="panel-heading"><div><span class="eyebrow">SELECTED / {mode === 'Simple' ? 'UNDERSTAND' : mode.toUpperCase()}</span><h2 id="candidate-title">{candidate ? 'Candidate ' + rank + (rank === 1 ? ' · first proposal' : ' · alternative') : 'Inside the proposal'}</h2></div></div>
  {#if candidate}
    <p class="visually-hidden" role="status">Selected candidate {rank} at {time(candidate.start)} UTC. This remains a proposal.</p>
    <div class="proposal-placement">{#each candidate.proposed.placements as placement}<div><strong>{time(placement.range.start)} → {time(placement.range.end)}</strong><span>UTC · proposed placement</span>{#if mode !== 'Simple'}<code>{placement.object_id} · [{placement.range.start}, {placement.range.end})</code>{/if}</div>{/each}</div>
    <p class="panel-description">{candidate.ranking_features.preferred_start_distance_seconds === null ? 'No preferred time was supplied for ranking.' : candidate.ranking_features.preferred_start_distance_seconds / 60 + ' minutes from the effective preferred time (whole-second projection).'} {candidate.ranking_features.mutation_count} proposed changes; {candidate.ranking_features.shift_seconds} seconds of shift.</p>
    {#if mode !== 'Simple'}<h3 class="subheading">Core evidence</h3><ul class="reason-list">{#each candidate.explanation as component}<li><span>{reasonLabel(component.reason)}</span><strong>{component.cost} s</strong></li>{/each}</ul><p class="fine-print">RankingFeatureSet {candidate.ranking_features.schema_version.major}.{candidate.ranking_features.schema_version.minor} · provenance: {candidate.ranking_features.preferred_start_source ? readable(candidate.ranking_features.preferred_start_source) : 'None'}. Source records provenance only.</p>{/if}
    {#if mode === 'Research'}<p class="fine-print">ProposedPlan {candidate.proposed.id} · source revision {candidate.proposed.source_revision} · start {candidate.start} · object {candidate.object_id}</p>{/if}
    <JsonPanel title="Selected candidate / complete placement, features and key" value={candidate} />
  {:else}<div class="detail-placeholder"><p>{hasResult ? 'The completed run returned no candidate. Inspect validation, dependency diagnostics and rejected positions.' : 'Run the planner to see a proposal and its evidence.'}</p></div>{/if}
</section>
