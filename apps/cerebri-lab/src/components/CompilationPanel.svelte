<script lang="ts">
  import AvailabilityTimeline from './AvailabilityTimeline.svelte';
  import JsonPanel from './JsonPanel.svelte';
  import { readable } from '../lib/presentation.ts';
  import type { CompiledContextSnapshot } from '../lib/compilation.ts';
  let { compiled }: { compiled: CompiledContextSnapshot } = $props();
  const skipped = $derived(compiled.series.flatMap(({ series, expansion }) => expansion.skipped.map((skip) => ({ series, skip }))));
</script>

<section class="panel inspector-panel compilation-panel" aria-labelledby="compilation-title">
  <div class="panel-heading"><div><span class="eyebrow">CORE / COMPILED SNAPSHOT</span><h2 id="compilation-title">Occupancy and its source</h2></div><span class="badge subtle">Revision {compiled.source_revision}</span></div>
  <p class="panel-description">Materialized occurrences retain their series identity, full interval and visible horizon. These are source facts, separate from prospective planning objects.</p>
  <div class="temporal-summary"><span class="badge subtle">{compiled.availability.coverage} coverage</span><span>{compiled.series.length} series</span><span>{compiled.occurrences.length} occurrences</span><span>{skipped.length} DST skips</span></div>
  <AvailabilityTimeline availability={compiled.availability} />
  <p class="fine-print">Every occupied, free and unknown interval above comes from the compiler report. Incomplete coverage leaves the complement unknown.</p>
  {#if compiled.occurrences.length}
    <h3 class="subheading">Materialized occurrences</h3>
    <div class="table-wrap"><table><thead><tr><th>Occurrence / series</th><th>Local date / time</th><th>Full UTC interval</th><th>Visible UTC interval</th><th>Source / resolution</th></tr></thead><tbody>
      {#each compiled.occurrences.slice(0, 100) as item}<tr><td><code class="occurrence-id" title={item.id}>{item.id}</code><br />{item.series_id}<br />Sequence {item.occurrence.sequence}</td><td>{item.occurrence.date}<br />{item.occurrence.local_time}</td><td>{item.occurrence.range.start}<br />{item.occurrence.range.end}</td><td>{item.occurrence.visible_range.start}<br />{item.occurrence.visible_range.end}</td><td>{readable(item.provenance)} · rev {item.source_revision}<br />{readable(item.occurrence.resolution)}<br /><span class="muted">Evidence: {item.evidence.join(', ') || 'No evidence IDs supplied'}</span></td></tr>{/each}
    </tbody></table></div>
  {/if}
  {#if skipped.length}
    <h3 class="subheading">Skipped nominal occurrences</h3>
    <div class="table-wrap"><table><thead><tr><th>Series / sequence</th><th>Local date / time</th><th>Timezone / policy</th><th>Core reason</th></tr></thead><tbody>
      {#each skipped.slice(0, 100) as { series, skip }}<tr><td>{series.id} · {skip.sequence}</td><td>{skip.date} · {series.rule.local_time}</td><td>{series.rule.timezone} · {series.rule.gap_policy}</td><td>{readable(skip.reason)}</td></tr>{/each}
    </tbody></table></div>
  {/if}
  <p class="fine-print">Tables show at most 100 occurrences and 100 skips. Full identities, recurrence definitions, evidence and buffer/clipping traces remain in the structured report.</p>
  <JsonPanel title="Compiled snapshot / complete source data" value={compiled} />
</section>
