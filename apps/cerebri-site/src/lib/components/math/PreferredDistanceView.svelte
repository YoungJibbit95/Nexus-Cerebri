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
  <header class="chapter-heading"><span class="chapter-number">02</span><div><small>A SECOND EXAMPLE / PREFERRED START</small><h3 id="preferred-title">What if you would prefer 10:45?</h3></div></header>
  <SemanticMark kind="preference" label="A preference helps compare valid times" detail="A time further away can still be valid. Required rules always apply." />
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
      <label for="math-candidate">Choose a possible start</label>
      <select id="math-candidate" bind:value={selected}>{#each fixture.candidates as entry, index}<option value={index}>{timeLabel(entry.start)} UTC</option>{/each}</select>
      <p>A start at {timeLabel(candidate.start)} is <strong>{candidate.ranking_features.preferred_start_distance_seconds} seconds</strong> from the preferred {timeLabel(fixture.preferred_start)} UTC. For example, 900 seconds is 15 minutes.</p>
      <small>Every listed time has passed the required checks. The Rust planner has already put them in order. This separate example adds a preference; the earlier example has none.</small>
    </div>
  </div>
  <details class="math-disclosure"><summary>Show the distance and its source</summary><div class="technical-reading">
    <code>preferred_start_distance_seconds = {candidate.ranking_features.preferred_start_distance_seconds}</code>
    <code>preferred_start_source = {candidate.ranking_features.preferred_start_source}</code>
    <p>These fields record the distance and where the preferred start came from. The source is recorded separately from the numeric value used for comparison.</p>
  </div></details>
  <SourceLink path={fixture.source} label="Input data for the preferred-start example" />
</section>
