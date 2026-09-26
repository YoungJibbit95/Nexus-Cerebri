<script lang="ts">
  import { mathData, position, timeLabel } from '$lib/math-inspection';
  import SemanticMark from '../SemanticMark.svelte';
  import SourceLink from '../SourceLink.svelte';

  const fixture = mathData.preferred;
  let selected = $state(1);
  const candidate = $derived(fixture.candidates[selected]);
  const preferredX = position(fixture.preferred_start, fixture.scope.start, fixture.scope.end);
  const candidateX = $derived(position(candidate.start, fixture.scope.start, fixture.scope.end));
</script>

<section class="math-chapter preferred-inspection" aria-labelledby="preferred-title">
  <header class="chapter-heading"><span class="chapter-number">02</span><div><small>CURRENT / SEPARATE PREFERENCE FIXTURE</small><h3 id="preferred-title">Measure from a preference.</h3></div></header>
  <SemanticMark kind="preference" label="Preference ≠ hard constraint" detail="Measurement influences order, never blocks this candidate." />
  <div class="distance-composition">
    <div class="distance-world" aria-hidden="true">
      <div class="preference-field" style:left={preferredX + '%'}></div>
      <div class="distance-axis"></div>
      <div class="preferred-point" style:left={preferredX + '%'}><i></i><span>PREFERRED<br />{timeLabel(fixture.preferred_start)}</span></div>
      <div class="distance-candidate" style:left={candidateX + '%'}><i class="closed"></i><span>CANDIDATE<br />{timeLabel(candidate.start)}</span></div>
      <div class="distance-bracket" style:left={Math.min(preferredX, candidateX) + '%'} style:width={Math.abs(preferredX - candidateX) + '%'}><span>{candidate.ranking_features.preferred_start_distance_seconds} s</span></div>
      <div class="distance-limits"><span>{timeLabel(fixture.scope.start)}</span><span>{timeLabel(fixture.scope.end)} UTC</span></div>
    </div>
    <div class="measurement-reading">
      <label for="math-candidate">Inspect a valid candidate</label>
      <select id="math-candidate" bind:value={selected}>{#each fixture.candidates as entry, index}<option value={index}>{timeLabel(entry.start)} UTC</option>{/each}</select>
      <p>Candidate {timeLabel(candidate.start)} is <strong>{candidate.ranking_features.preferred_start_distance_seconds} seconds</strong> from preferred start {timeLabel(fixture.preferred_start)} UTC.</p>
      <small>All listed candidates are valid. Their sequence is the Rust-produced order.</small>
    </div>
  </div>
  <details class="math-disclosure"><summary>Inspect the distance observation</summary><div class="technical-reading">
    <code>preferred_start_distance_seconds = {candidate.ranking_features.preferred_start_distance_seconds}</code>
    <code>preferred_start_source = {candidate.ranking_features.preferred_start_source}</code>
    <p>The homepage fixture above still has no preference evidence. This separate fixture explicitly supplies the preferred start.</p>
  </div></details>
  <SourceLink path={fixture.source} label="Preferred-start fixture" />
</section>
