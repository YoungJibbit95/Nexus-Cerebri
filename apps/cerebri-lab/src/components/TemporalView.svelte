<script lang="ts">
  import fixture from '../../../../examples/temporal-request.json';
  import Icon from './Icon.svelte';
  import JsonPanel from './JsonPanel.svelte';
  import AvailabilityTimeline from './AvailabilityTimeline.svelte';
  import TemporalStory from './TemporalStory.svelte';
  import InspectorDrawer from './InspectorDrawer.svelte';
  import { readable } from '../lib/presentation.ts';
  import { downloadJson, postJson, readJsonFile } from '../lib/transport.ts';
  import { parseTemporalRequest, parseTemporalResult } from '../lib/temporal.ts';
  import type { TemporalRequest, TemporalResult } from '../lib/temporal.ts';
  import type { ConsoleEntry, ExplanationMode } from '../lib/contracts.ts';
  let { mode, onlog }: { mode: ExplanationMode; onlog: (source: ConsoleEntry['source'], message: string, data?: unknown, level?: ConsoleEntry['level']) => void } = $props();
  let request = $state<TemporalRequest>(parseTemporalRequest(structuredClone(fixture)));
  let result = $state<TemporalResult | null>(null);
  let resultRequest = $state<TemporalRequest | null>(null);
  let busy = $state(false);
  let error = $state('');
  let input: HTMLInputElement;
  const report = $derived(result?.status === 'Complete' ? result.data : null);
  async function run() {
    busy = true; error = ''; result = null; resultRequest = null;
    const snapshot = parseTemporalRequest(JSON.parse(JSON.stringify(request)));
    try {
      result = parseTemporalResult(await postJson('/v1/temporal', snapshot)); resultRequest = snapshot;
      onlog('temporal', `Temporal diagnostics: ${result.status}.`, result, result.status === 'Complete' ? 'success' : 'info');
    } catch (reason) {
      error = reason instanceof Error ? reason.message : String(reason); onlog('temporal', error, undefined, 'error');
    } finally { busy = false; }
  }
  async function importRequest(file: File) {
    try { request = parseTemporalRequest(await readJsonFile(file, 256 * 1024)); result = null; resultRequest = null; error = ''; onlog('lab', `Temporal request imported: ${file.name}`); }
    catch (reason) { error = reason instanceof Error ? reason.message : String(reason); }
  }
</script>

<section class="panel inspector-panel temporal-view" aria-labelledby="temporal-title"><div class="panel-heading"><div><span class="eyebrow">TEMPORAL CORE / DIAGNOSTICS</span><h2 id="temporal-title">Time, made explicit</h2></div><Icon name="clock" /></div><p class="panel-description">Bounded recurrence expansion, buffers and occupied / free / unknown intervals. Every displayed interval comes from the Rust temporal core.</p>
  <div class="temporal-controls"><label>Source coverage<select aria-label="Source coverage" bind:value={request.coverage} disabled={busy} onchange={() => result = null}><option value="Complete">Complete — gaps may be free</option><option value="Incomplete">Incomplete — gaps remain unknown</option></select></label><div><button class="secondary" disabled={busy} onclick={() => { request = parseTemporalRequest(structuredClone(fixture)); result = null; error = ''; }}>Load DST demo</button><button class="secondary" disabled={busy} onclick={() => input.click()}><Icon name="upload" size={14} />Import</button><button class="primary" disabled={busy} onclick={run}><Icon name="play" size={14} />{busy ? 'Evaluating…' : 'Inspect temporal data'}</button></div></div>
  <input bind:this={input} type="file" class="visually-hidden" tabindex="-1" accept=".json,application/json" aria-label="Import temporal diagnostics request" onchange={(event) => { const file = event.currentTarget.files?.[0]; if (file) void importRequest(file); event.currentTarget.value = ''; }} />
  <p class="fine-print">The DST demo repeats at 02:30 in Europe/Berlin. Run it to explore the core's explicit resolution; imported requests use their own rules.</p>
  {#if error}<p class="status-message error" role="alert">{mode === 'Simple' ? 'Temporal data could not be loaded. Check the input and local API connection; Technical depth shows the exact diagnostic.' : error}</p>{/if}
  {#if result?.status === 'Rejected'}<div class="temporal-rejection" role="status"><strong>Temporal request rejected</strong><p>No partial availability report is used. Technical depth exposes the core rejection.</p><JsonPanel title="Core rejection" value={result.data} open /></div>
  {:else if report}
    <div class="temporal-summary" role="status"><span class="badge subtle">{report.availability.coverage} coverage</span><span>{report.availability.busy.length} busy intervals</span><span>{report.availability.free.length} free intervals</span><span>{report.availability.unknown.length} unknown intervals</span><button class="quiet" onclick={() => downloadJson(result, 'cerebri-temporal-result.json')}><Icon name="download" size={13} /> Export temporal JSON</button></div>
    {#if resultRequest}<TemporalStory {report} request={resultRequest} {mode} />{/if}
    <AvailabilityTimeline availability={report.availability} />
    {#if mode !== 'Simple'}<InspectorDrawer title="Returned recurrence tables" open={mode === 'Research'}>
    {#each report.expansions as expansion, index}<h3 class="subheading">Recurrence {index + 1} · {expansion.timezone}</h3><p class="panel-description">{expansion.examined_dates} dates examined · {expansion.occurrences.length} visible occurrences · {expansion.skipped.length} skipped</p><div class="table-wrap"><table><thead><tr><th>Sequence</th><th>Local date / time</th><th>UTC interval</th><th>Resolution</th></tr></thead><tbody>{#each (mode === 'Research' ? expansion.occurrences : expansion.occurrences.slice(0, 100)) as occurrence}<tr><td>{occurrence.sequence}</td><td>{occurrence.date}<br />{occurrence.local_time}</td><td>{occurrence.range.start}<br />{occurrence.range.end}</td><td>{readable(occurrence.resolution)}</td></tr>{/each}{#each (mode === 'Research' ? expansion.skipped : expansion.skipped.slice(0, 100)) as skip}<tr><td>{skip.sequence}</td><td>{skip.date}</td><td>Skipped</td><td>{readable(skip.reason)}</td></tr>{/each}</tbody></table></div>{/each}
    </InspectorDrawer><p class="fine-print">Technical tables show up to 100 occurrences and skips per recurrence. Research retains every returned row.</p>{/if}
    <JsonPanel title="Busy, free and unknown interval data" value={report.availability} />
    <JsonPanel title="Buffer, clipping and recurrence traces" value={report} />
  {:else}<div class="large-empty"><Icon name="clock" size={34} /><h3>Inspect the shape of time</h3><p>Run the DST fixture to see occupied intervals, explicit gap handling and bounded recurrence output.</p></div>{/if}
  {#if result}<JsonPanel title="Complete TemporalResult / all returned evidence" value={result} />{/if}
  <JsonPanel title="Temporal request / policies and limits" value={resultRequest ?? request} />
</section>
