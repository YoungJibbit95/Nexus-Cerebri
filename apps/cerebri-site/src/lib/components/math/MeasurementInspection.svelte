<script lang="ts">
  import { mathData, secondsLabel } from '$lib/math-inspection';
  import SourceLink from '../SourceLink.svelte';

  const samples = mathData.measurement.samples;
  let sampleIndex = $state(1);
  let measured = $state(false);
  let shiftIndex = $state(5);
  const sample = $derived(samples[sampleIndex]);
  const shift = $derived(shiftIndex < 0 ? null : samples[shiftIndex]);
</script>

<section class="math-chapter quantization-inspection" aria-labelledby="quantization-title" data-measured={measured}>
  <header class="chapter-heading"><span class="chapter-number">03</span><div><small>CURRENT / RUST CONTRACT PROBES</small><h3 id="quantization-title">Measure whole seconds. Keep the instant.</h3></div></header>
  <p class="probe-caption">Synthetic inputs exercise the current observation function. These are contract probes, separate from planner search.</p>
  <div class="inspection-controls" role="group" aria-label="Actual temporal difference">{#each samples.slice(0, 5) as entry, index}
    <button aria-pressed={sampleIndex === index} onclick={() => sampleIndex = index}>{secondsLabel(entry.actual_delta_ms)} s</button>
  {/each}</div>
  <div class="measurement-composition">
    <div class="measurement-world" aria-hidden="true">
      <div class="measurement-lattice"><i></i><i></i><i></i></div>
      <div class="measurement-axis"></div>
      <div class="measurement-origin"><i></i><span>preferred / 0</span></div>
      <div class="exact-instant" style:left={50 + sample.actual_delta_ms / 25 + '%'}><i class="closed"></i><span>{secondsLabel(sample.actual_delta_ms)} s</span></div>
      <div class="measurement-ticks"><span>−1 s</span><span>0 s</span><span>+1 s</span></div>
    </div>
    <div class="observation-reading"><small>WHOLE-SECOND OBSERVATION</small><strong>{sample.ranking_features.preferred_start_distance_seconds} s</strong><span>The marker stays at {secondsLabel(sample.actual_delta_ms)} s.</span></div>
  </div>
  <button class="notation-toggle" aria-pressed={measured} onclick={() => measured = !measured}>{measured ? 'Hide measurement lattice' : 'Reveal measurement lattice'}</button>
  <p class="measurement-alternative">Actual signed difference: {secondsLabel(sample.actual_delta_ms)} seconds; whole-second ranking observation: {sample.ranking_features.preferred_start_distance_seconds} seconds. The instant is not snapped.</p>
  <details class="math-disclosure"><summary>Inspect exact instants and measurement</summary><div class="technical-reading">
    <code>candidate_start = {sample.candidate_start}</code><code>preferred_start = {sample.preferred_start}</code>
    <code>(candidate_start - preferred_start).num_seconds().unsigned_abs()</code>
    <p>The signed duration is truncated toward zero to whole seconds before taking its magnitude.</p>
  </div></details>
  <SourceLink path={mathData.measurement.source} label="Signed subsecond contract tests" />
</section>

<section class="math-chapter shift-inspection" aria-labelledby="shift-title" data-original={shift ? 'present' : 'absent'}>
  <header class="chapter-heading"><span class="chapter-number">04</span><div><small>CURRENT / PLACEMENT OBSERVATION</small><h3 id="shift-title">Displacement needs an origin.</h3></div></header>
  <label for="math-shift">Inspect original placement</label>
  <select id="math-shift" bind:value={shiftIndex}><option value={5}>Present · 12.345 s displacement</option><option value={1}>Present · 0.8 s displacement</option><option value={0}>Present · unchanged</option><option value={-1}>Absent</option></select>
  <div class="shift-composition">
    <div class="shift-world" aria-hidden="true">
      {#if shift}
        <div class="ghost-placement"><i></i><span>ORIGINAL START</span></div>
        <div class="shift-trace" style:width={shift.actual_delta_ms / 12345 * 66 + '%'}></div>
        <div class="shift-candidate" style:left={12 + shift.actual_delta_ms / 12345 * 66 + '%'}><i class="closed"></i><span>CANDIDATE START</span></div>
      {:else}<div class="absent-origin">∅<span>Original placement absent</span></div>{/if}
    </div>
    <div class="shift-reading"><small>shift_seconds</small><strong>{shift?.ranking_features.shift_seconds ?? mathData.measurement.absent.ranking_features.shift_seconds} s</strong>
      <p>{shift ? 'Actual displacement: ' + secondsLabel(shift.actual_delta_ms) + ' seconds. Dashed geometry records the original start.' : 'Original placement = absent. Numeric zero is the legacy observation; it does not establish an unchanged placement.'}</p>
    </div>
  </div>
  <details class="math-disclosure"><summary>Inspect original and candidate instants</summary><div class="technical-reading">
    <code>original_start = {shift?.original_start ?? 'None'}</code><code>candidate_start = {shift?.candidate_start ?? mathData.measurement.absent.candidate_start}</code>
    <p>Contract probe of the current observation function; no provider movement or execution is represented.</p>
  </div></details>
</section>
