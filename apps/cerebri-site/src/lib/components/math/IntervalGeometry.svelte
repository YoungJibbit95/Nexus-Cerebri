<script lang="ts">
  import { mathData, position, timeLabel } from '$lib/math-inspection';
  import SourceLink from '../SourceLink.svelte';

  let example = $state(0);
  let compressed = $state(false);
  const current = $derived(mathData.intervals[example]);
  const zoomStart = $derived(current.detail_ticks[0]);
  const zoomEnd = $derived(current.detail_ticks[3]);
</script>

<section class="math-chapter interval-inspection" aria-labelledby="interval-title" data-compressed={compressed}>
  <header class="chapter-heading"><span class="chapter-number">01</span><div><small>CURRENT / TEMPORAL GEOMETRY</small><h3 id="interval-title">An endpoint is a boundary.</h3></div></header>
  <div class="inspection-controls" role="group" aria-label="Interval relationship">
    <button aria-pressed={example === 0} onclick={() => example = 0}>Touching intervals</button>
    <button aria-pressed={example === 1} onclick={() => example = 1}>One-second overlap</button>
  </div>
  <div class="interval-composition">
    <div class="interval-world" aria-hidden="true">
      <div class="rail-heading"><span>A / KNOWN BUSY</span><span>B / CANDIDATE</span></div>
      <div class="interval-scale">
        <div class="extent fact" style:left="0%" style:width={position(current.a.end, current.a.start, current.b.end) + '%'}><i class="closed"></i><i class="open"></i></div>
        <div class="extent candidate" style:left={position(current.b.start, current.a.start, current.b.end) + '%'} style:width={100 - position(current.b.start, current.a.start, current.b.end) + '%'}><i class="closed"></i><i class="open"></i></div>
        <span class="scale-start">{timeLabel(current.a.start)}</span><span class="scale-end">{timeLabel(current.b.end)} UTC</span>
      </div>
      <div class="endpoint-lens">
        <small>SHARED BOUNDARY / 3-SECOND DETAIL</small>
        <div class="zoom-rails">
          <div class="zoom-line fact" style:width={position(current.a.end, zoomStart, zoomEnd) + '%'}><i class="open"></i><span>A</span></div>
          <div class="zoom-line candidate"><i class="closed"></i><span>B</span></div>
          {#if current.intersection}
            <div class="intersection-band" style:left={position(current.intersection.start, zoomStart, zoomEnd) + '%'} style:width={position(current.intersection.end, zoomStart, zoomEnd) - position(current.intersection.start, zoomStart, zoomEnd) + '%'}><i class="constraint-cut"></i></div>
          {:else}<div class="touch-guide"></div>{/if}
        </div>
        <div class="zoom-labels">{#each current.detail_ticks as tick}<span>{timeLabel(tick)}</span>{/each}</div>
      </div>
    </div>
    <div class="interval-reading">
      <p class="math-result">{current.overlaps ? 'One shared second.' : 'One shared endpoint. No overlap.'}</p>
      <p>A starts at {timeLabel(current.a.start)} inclusive and ends at {timeLabel(current.a.end)} exclusive. B starts at {timeLabel(current.b.start)} inclusive and ends at {timeLabel(current.b.end)} exclusive.</p>
      {#if current.intersection}
        <p class="intersection-explanation">Intervals overlap from {timeLabel(current.intersection.start)} to {timeLabel(current.intersection.end)} UTC. The hatched band is shared time; the cut marks the hard-constraint conflict.</p>
      {:else}<p>The shared instant belongs to B and is excluded from A. Rust reports <strong>{current.relation}</strong>.</p>{/if}
    </div>
  </div>
  <div class="notation-workspace">
    <button class="notation-toggle" aria-pressed={compressed} onclick={() => compressed = !compressed}>{compressed ? 'Expand interval geometry' : 'Compress geometry to notation'}</button>
    <div class="boundary-compression" aria-hidden="true">
      <div class="compression-start"><i class="closed"></i><b>[</b><span>start</span></div>
      <div class="compression-extent"><i></i><b>,</b></div>
      <div class="compression-end"><span>end</span><i class="open"></i><b>)</b></div>
    </div>
    <p class="notation-equivalent"><code>[start, end)</code> — filled start ↔ inclusive <code>[</code> · open end ↔ exclusive <code>)</code></p>
  </div>
  <details class="math-disclosure"><summary>Inspect the interval relation</summary><div class="technical-reading">
    <code>A.start &lt; B.end &amp;&amp; B.start &lt; A.end</code>
    <p>Rust overlap result: <strong>{String(current.overlaps)}</strong>. Planner outcome for this fixture: <strong>{current.outcome}</strong>.</p>
    <SourceLink path="crates/cerebri-temporal/src/lib.rs" label="TimeRange overlap and intersection" />
  </div></details>
  <SourceLink path={current.source} label="Paired interval fixture" />
</section>
