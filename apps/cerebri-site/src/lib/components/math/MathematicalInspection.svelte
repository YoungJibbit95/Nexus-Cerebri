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
  <header class="math-heading"><div><span class="kicker">CURRENT / MATHEMATICAL INSPECTION</span><h2 id="math-title">Geometry. Measurement. Notation.</h2></div><p>Inspect the temporal relationships behind the current deterministic planner.</p></header>
  <IntervalGeometry />
  <PreferredDistanceView />
  <MeasurementInspection />
  <TechnicalInspection />
</section>
