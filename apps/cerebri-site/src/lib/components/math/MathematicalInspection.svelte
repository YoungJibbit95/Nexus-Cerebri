<script lang="ts">
  import { onMount } from 'svelte';
  import IntervalGeometry from './IntervalGeometry.svelte';
  import PreferredDistanceView from './PreferredDistanceView.svelte';
  import MeasurementInspection from './MeasurementInspection.svelte';
  import TechnicalInspection from './TechnicalInspection.svelte';
  import '$lib/math-inspection.css';

  let reduced = $state(true);
  onMount(() => {
    const preference = window.matchMedia('(prefers-reduced-motion: reduce)');
    const update = () => reduced = preference.matches;
    update();
    preference.addEventListener('change', update);
    return () => preference.removeEventListener('change', update);
  });
</script>

<section data-world="measurement" class="math-inspection" id="mathematical-inspection" aria-labelledby="math-title" data-motion={reduced ? 'reduced' : 'full'}>
  <header class="math-heading"><div><span class="kicker">A CLOSER LOOK AT TIME</span><h2 id="math-title">When do appointments overlap?</h2></div><p>Explore why one appointment can start exactly when another ends, and how a preferred time affects the comparison.</p></header>
  <IntervalGeometry />
  <PreferredDistanceView />
  <MeasurementInspection />
  <TechnicalInspection />
</section>
