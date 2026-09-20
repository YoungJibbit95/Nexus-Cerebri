<script lang="ts">
  import { date, intervalStyle, time } from '../lib/presentation.ts';
  import type { AvailabilityReport } from '../lib/temporal.ts';
  let { availability }: { availability: AvailabilityReport } = $props();
  const kinds = ['busy', 'free', 'unknown'] as const;
  const ticks = $derived(Array.from({ length: 4 }, (_, index) => new Date(Date.parse(availability.horizon.start) + (Date.parse(availability.horizon.end) - Date.parse(availability.horizon.start)) * index / 3).toISOString()));
</script>

<div class="timeline-scroll availability-timeline"><div class="timeline">
  <div class="timeline-axis"><span>TIME / UTC</span><div>{#each ticks as tick}<span>{date(tick).slice(0, 6)}<br />{time(tick)}</span>{/each}</div></div>
  {#each kinds as kind}
    <div class="timeline-row"><span class="row-label">{kind === 'busy' ? 'Occupied' : kind === 'free' ? 'Verified free' : 'Unknown'}<small>{availability[kind].length} intervals</small></span><div class="track">{#each availability[kind] as range}<span class={`time-block temporal-block ${kind}-block`} style={intervalStyle(range, availability.horizon)} title={`${kind}: ${range.start} — ${range.end}`}></span>{/each}</div></div>
  {/each}
</div></div>
