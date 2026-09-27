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
  <header class="chapter-heading"><span class="chapter-number">01</span><div><small>EXAMPLE / ADJACENT APPOINTMENTS</small><h3 id="interval-title">One ends. The next can begin.</h3></div></header>
  <div class="inspection-controls" role="group" aria-label="Interval relationship">
    <button aria-pressed={example === 0} onclick={() => example = 0}>Back-to-back appointments</button>
    <button aria-pressed={example === 1} onclick={() => example = 1}>One-second overlap</button>
  </div>
  <div class="interval-composition">
    <div class="interval-world" aria-hidden="true">
      <div class="rail-heading"><span>A / EXISTING APPOINTMENT</span><span>B / POSSIBLE NEW APPOINTMENT</span></div>
      <div class="interval-scale">
        <div class="extent fact" style:left="0%" style:width={position(current.a.end, current.a.start, current.b.end) + '%'}><i class="closed"></i><i class="open"></i></div>
        <div class="extent candidate" style:left={position(current.b.start, current.a.start, current.b.end) + '%'} style:width={100 - position(current.b.start, current.a.start, current.b.end) + '%'}><i class="closed"></i><i class="open"></i></div>
        <span class="scale-start">{timeLabel(current.a.start)}</span><span class="scale-end">{timeLabel(current.b.end)} UTC</span>
      </div>
      <div class="endpoint-lens">
        <small>THREE SECONDS AROUND THE CHANGEOVER</small>
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
      <p class="math-result">{current.overlaps ? 'These appointments overlap by one second.' : 'These appointments fit back to back.'}</p>
      <p>A runs from {timeLabel(current.a.start)} to {timeLabel(current.a.end)}. B runs from {timeLabel(current.b.start)} to {timeLabel(current.b.end)} UTC. Each appointment includes its start, but no longer occupies time at its end.</p>
      {#if current.intersection}
        <p class="intersection-explanation">Both occupy {timeLabel(current.intersection.start)}–{timeLabel(current.intersection.end)} UTC. The hatched area shows that overlap. B fails the no-overlap check.</p>
      {:else}<p>At the changeover, A has ended and B begins. There is no overlap. The code calls this relationship <strong>{current.relation}</strong>.</p>{/if}
    </div>
  </div>
  <div class="notation-workspace">
    <button class="notation-toggle" aria-pressed={compressed} onclick={() => compressed = !compressed}>{compressed ? 'Show the timeline' : 'Show the interval notation'}</button>
    <div class="boundary-compression" aria-hidden="true">
      <div class="compression-start"><i class="closed"></i><span>start</span></div>
      <div class="compression-extent"><i></i><b>,</b></div>
      <div class="compression-end"><span>end</span><i class="open"></i></div>
    </div>
    <p class="notation-equivalent"><code>[start, end)</code> means the start is included and the end is excluded. The filled and open markers show the same rule. This is a half-open interval.</p>
  </div>
  <details class="math-disclosure"><summary>Show the overlap check</summary><div class="technical-reading">
    <code>A.start &lt; B.end &amp;&amp; B.start &lt; A.end</code>
    <p>Do these intervals overlap? <strong>{String(current.overlaps)}</strong>. Planner result for this example: <strong>{current.outcome}</strong>.</p>
    <SourceLink path="crates/cerebri-temporal/src/lib.rs" label="TimeRange overlap and intersection" />
  </div></details>
  <SourceLink path={current.source} label="Input data for the interval examples" />
</section>
