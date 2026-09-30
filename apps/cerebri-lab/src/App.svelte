<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import fixture from '../../../examples/request.json';
  import Sidebar from './components/Sidebar.svelte';
  import Icon from './components/Icon.svelte';
  import RequestPanel from './components/RequestPanel.svelte';
  import DepthControl from './components/DepthControl.svelte';
  import PlannerScene from './components/PlannerScene.svelte';
  import InspectorDrawer from './components/InspectorDrawer.svelte';
  import OutputConsole from './components/OutputConsole.svelte';
  import InspectorViews from './components/InspectorViews.svelte';
  import TemporalView from './components/TemporalView.svelte';
  import TraceView from './components/TraceView.svelte';
  import CompilationPanel from './components/CompilationPanel.svelte';
  import DependencyPanel from './components/DependencyPanel.svelte';
  import JsonPanel from './components/JsonPanel.svelte';
  import type { ConsoleEntry, ExplanationMode, LabView, PlanningRequest, PlanningResult, ValidationReport } from './lib/contracts.ts';
  import { downloadJson, parseRequest, parseResult, parseValidation, postJson, readJsonFile } from './lib/transport.ts';
  import { readable } from './lib/presentation.ts';
  import { scenarioRequest } from './lib/scenarios.ts';
  import { DEPTH_CONTEXT, DEPTH_STORAGE_KEY, storedDepth } from './lib/depth.ts';

  function demo(): PlanningRequest {
    const value = structuredClone(fixture);
    return parseRequest({ ...value, preferences: { preferences: [{ source: 'ExplicitCurrentRequest', preferred_start: '2026-10-01T10:45:00Z', evidence: [] }] } });
  }
  let view = $state<LabView>('Planner');
  let mode = $state<ExplanationMode>('Simple');
  setContext(DEPTH_CONTEXT, () => mode);
  let runId = $state(0);
  let theme = $state<'dark' | 'light'>('dark');
  let request = $state<PlanningRequest>(demo());
  let resultRequest = $state<PlanningRequest | null>(null);
  let requestLabel = $state('Synthetic focus session');
  let result = $state<PlanningResult | null>(null);
  let validation = $state<ValidationReport | null>(null);
  let selected = $state(0);
  let rejected = $state<number | null>(null);
  let busy = $state(false);
  let api = $state('Checking API');
  let version = $state('');
  let status = $state('Synthetic request ready. Run the planner to explore its proposals.');
  let error = $state(false);
  let entries = $state<ConsoleEntry[]>([]);
  let sequence = 0;
  let resultInput: HTMLInputElement;
  const titles: Record<LabView, [string, string]> = {
    Planner: ['Make planning visible.', 'Explore possibilities. Inspect every decision.'],
    Temporal: ['Time deserves a closer look.', 'Inspect recurrence, availability and explicit DST decisions.'],
    Compilation: ['From a rule to occupied time.', 'Explore the source, materialization and visibility of planner occupancy.'],
    Trace: ['Every decision has a reason.', 'Follow evidence from request to proposal.'],
    Semantics: ['Meaning, with its evidence.', 'Keep facts and inference in clear view.'],
    Preferences: ['Understand the tradeoffs.', 'Transparent priorities. Explainable costs.'],
    ML: ['Build intuition. Verify reality.', 'A workspace for the learning milestones ahead.'],
    Dataset: ['Start with trustworthy data.', 'Prepare the foundations of reproducible research.'],
  };
  function log(source: ConsoleEntry['source'], message: string, data?: unknown, level: ConsoleEntry['level'] = 'info') {
    entries = [...entries, { id: ++sequence, at: new Date().toISOString(), level, source, message, data }].slice(-40);
  }
  function fail(reason: unknown) { status = reason instanceof Error ? reason.message : String(reason); error = true; log('lab', status, undefined, 'error'); }
  async function checkApi() {
    try {
      const response = await fetch('/health', { signal: AbortSignal.timeout(4000) });
      if (!response.ok) throw new Error('Unavailable');
      const health = await response.json() as { status?: string; software_version?: string };
      if (health.status !== 'ok') throw new Error('Unexpected health response');
      api = 'Connected'; version = typeof health.software_version === 'string' ? health.software_version : '';
    } catch { api = 'API offline'; }
  }
  onMount(() => {
    try { mode = storedDepth(localStorage.getItem(DEPTH_STORAGE_KEY)); } catch { /* Storage may be disabled; depth remains usable. */ }
    void checkApi(); log('lab', 'Workspace ready. Synthetic CPIR loaded; no planning result yet.');
  });
  function changeDepth(next: ExplanationMode) {
    mode = next;
    try { localStorage.setItem(DEPTH_STORAGE_KEY, next); } catch { /* Persistence is optional. */ }
  }
  function loadDemo() {
    rejected = null;
    request = demo(); requestLabel = 'Synthetic focus session'; result = null; resultRequest = null; validation = null; selected = 0; runId = 0;
    status = 'Synthetic request loaded with an explicit 10:45 UTC preference.'; error = false; log('lab', status);
  }
  function loadScenario(file: string) {
    rejected = null;
    request = scenarioRequest(file); requestLabel = file.replace('.json', '').replaceAll('-', ' ');
    result = null; resultRequest = null; validation = null; selected = 0; runId = 0; error = false;
    status = `Synthetic scenario loaded: ${requestLabel}. Run the Rust planner to inspect the result.`;
    log('lab', status, { file, request_id: request.request_id });
  }
  async function importRequest(file: File) {
    try {
      const parsed = parseRequest(await readJsonFile(file, 256 * 1024));
      rejected = null;
      request = parsed; requestLabel = file.name; result = null; resultRequest = null; validation = null; selected = 0; runId = 0;
      status = 'CPIR imported. Use Validate CPIR for authoritative Rust validation.'; error = false; log('lab', status, { file: file.name, request_id: parsed.request_id });
    } catch (reason) { fail(reason); }
  }
  async function importResult(file: File) {
    try {
      result = parseResult(await readJsonFile(file)); resultRequest = null; validation = null; selected = 0; runId = 0;
      rejected = null;
      status = 'PlanningResult imported. Its source request is not attached.'; error = false;
      log('planner', `Imported result: ${file.name}`, result, 'success');
    } catch (reason) { fail(reason); }
  }
  async function run(action: 'plan' | 'validate') {
    busy = true; error = false; status = action === 'plan' ? 'Rust core is evaluating the request…' : 'Rust core is validating CPIR…';
    const snapshot = parseRequest(JSON.parse(JSON.stringify(request)));
    try {
      const data = await postJson(`/v1/${action}`, snapshot);
      api = 'Connected';
      if (action === 'plan') {
        result = parseResult(data); resultRequest = snapshot; validation = result.validation; selected = 0; runId++;
        rejected = null;
        status = `${readable(result.outcome)} · ${result.candidates.length} feasible candidates · ${result.search_space.evaluated} evaluated positions.`;
        log('planner', status, data, 'success');
      } else {
        validation = parseValidation(data); status = `Request validation: ${readable(validation.state)}. ${validation.issues.length} issues.`;
        log('validator', status, data, validation.issues.length ? 'info' : 'success');
      }
    } catch (reason) { fail(reason); void checkApi(); }
    finally { busy = false; }
  }
</script>

<svelte:head><meta name="color-scheme" content={theme} /></svelte:head>
<div class="app-shell" data-theme={theme} data-depth={mode}>
  <a href="#workspace" class="skip-link">Skip to workspace</a>
  <Sidebar {view} onselect={(next) => view = next} {api} {version} />
  <div class="workspace-shell">
    <header class="topbar"><div class="breadcrumb">Workspace <span>/</span> <strong>{view}</strong></div><div class="topbar-actions"><span class="environment"><span class="status-dot online"></span>LOCAL RESEARCH</span><button class="icon-button" aria-label={`Switch to ${theme === 'dark' ? 'light' : 'dark'} theme`} onclick={() => theme = theme === 'dark' ? 'light' : 'dark'}><Icon name="sun" size={18} /></button><span class="avatar" aria-label="Nexus workspace">N</span></div></header>
    <main id="workspace" tabindex="-1">
      <div class="hero"><div><span class="eyebrow">NEXUS CEREBRI / {view.toUpperCase()}</span><h1>{titles[view][0]}</h1><p>{titles[view][1]}</p></div><div class="hero-actions"><button class="secondary" onclick={() => resultInput.click()} disabled={busy}><Icon name="upload" size={16} />Import result</button><button class="secondary" disabled={!result} onclick={() => result && downloadJson(result, 'cerebri-planning-result.json')}><Icon name="download" size={16} />Export JSON</button></div></div>
      <input bind:this={resultInput} class="visually-hidden" tabindex="-1" type="file" accept=".json,application/json" aria-label="Import PlanningResult" onchange={(event) => { const file = event.currentTarget.files?.[0]; if (file) void importResult(file); event.currentTarget.value = ''; }} />
      <div class="workspace-controls"><div class="view-context"><span class="status-dot" class:online={result !== null}></span>{result ? 'RESULT LOADED' : 'READY TO EXPLORE'}<span class="context-separator">/</span>{mode === 'Simple' ? 'Visualize → explain → inspect' : `Input CPIR ${request.schema_version.major}.${request.schema_version.minor}`}</div></div>
      <DepthControl {mode} onchange={changeDepth} />
      <p class="status-message" class:error role="status">{error && mode === 'Simple' ? 'This input could not be loaded or evaluated. Check the input and local API connection; Technical depth shows the exact diagnostic.' : status}</p>
      {#if view === 'Planner'}
        <div class="planner-workbench"><RequestPanel {request} label={requestLabel} {busy} {validation} {mode} ondemo={loadDemo} onscenario={loadScenario} onimport={importRequest} onvalidate={() => run('validate')} onplan={() => run('plan')} /><PlannerScene request={result ? resultRequest : request} {result} {selected} onselect={(index) => selected = index} {mode} {runId} {rejected} onreject={(index) => rejected = index} /></div>
        {#if result && mode === 'Simple' && (!result.candidates.length || result.validation.issues.length || result.dependency_graph.issues.length)}<p class="status-message">{readable(result.validation.state)} · {result.validation.issues.length} validation issues · {result.dependency_graph.issues.length} dependency issues. Switch to Technical for exact diagnostics.</p>{/if}
        {#if result && mode !== 'Simple'}
          <InspectorDrawer title="Validation, compilation and dependency evidence" open={mode === 'Research'}>
            <JsonPanel title="Request validation" value={result.validation} />
            {#if result.compilation}<CompilationPanel compiled={result.compilation} {mode} />{/if}
            <DependencyPanel graph={result.dependency_graph} {mode} />
          </InspectorDrawer>
          <section class="research-context"><p>Optimality covers the bounded discrete grid, stated objective and stable tie breaks. It does not prove a continuous-time or global scheduling optimum.</p><JsonPanel title="Complete PlanningResult / all returned source data" value={result} />{#if resultRequest}<JsonPanel title="Exact request sent for this result" value={resultRequest} />{/if}</section>
        {/if}
      {:else if view === 'Temporal'}<TemporalView {mode} onlog={log} />
      {:else if view === 'Compilation'}
        {#if result?.compilation}<CompilationPanel compiled={result.compilation} {mode} />{:else}<section class="panel inspector-panel"><h2>No compiled snapshot is attached.</h2><p class="panel-description">Run a Planner request with temporal context. Its returned compiled snapshot supplies this instrument.</p><button class="secondary" disabled={busy} onclick={() => { loadScenario('recurrence-busy.json'); view = 'Planner'; }}>Load recurrence planning scenario</button></section>{/if}
      {:else if view === 'Trace'}<TraceView {result} request={resultRequest} {mode} {selected} onselect={(index) => selected = index} {rejected} onreject={(index) => rejected = index} />
      {:else}<InspectorViews {view} {result} request={resultRequest ?? request} {mode} sourceAttached={!result || resultRequest !== null} />{/if}
      <OutputConsole {entries} {mode} onclear={() => entries = []} />
      <footer class="workspace-footer"><span>Neural intuition. Symbolic verification.</span><span>Built in the open · Human + AI collaboration</span></footer>
    </main>
  </div>
</div>
