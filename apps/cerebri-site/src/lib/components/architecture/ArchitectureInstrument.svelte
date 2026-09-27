<script lang="ts">
  import { base } from '$app/paths';
  import { onMount } from 'svelte';
  import { runtimeLayers } from '$lib/architecture-ownership';
  import SourceLink from '../SourceLink.svelte';
  import DepthLens from '../technical/DepthLens.svelte';
  import '../technical/technical.css';
  let precision = $state(0);
  let focused = $state('planner');
  const selected = $derived(runtimeLayers.find(layer => layer.id === focused) ?? runtimeLayers[2]);
  onMount(() => {
    if (location.hash.startsWith('#runtime-')) {
      const id = location.hash.slice('#runtime-'.length);
      if (runtimeLayers.some(layer => layer.id === id)) { focused = id; precision = 2; }
    }
  });
  function inspect(id: string) { focused = id; precision = 2; }
</script>

<div class="technical-instrument architecture-instrument" id="architecture-instrument" data-precision={precision} data-focused={focused}>
  <DepthLens bind:value={precision} architecture />
  <header class="instrument-heading"><div><small>EDUCATIONAL / REPOSITORY OWNERSHIP</small><h2>Exploded precision</h2></div><span>ONE SYSTEM · INCREASING RESOLUTION</span></header>
  <ol class="concept-pipeline" aria-label="Planning concept">
    <li><span>01</span><strong>Structured state</strong></li>
    <li><span>02</span><strong>Planner</strong></li>
    <li><span>03</span><strong>Candidates</strong><small>After hard-rule checks</small></li>
    <li><span>04</span><strong>Proposal</strong><small>Not yet a ValidatedPlan</small></li>
  </ol>
  <div class="architecture-drawing">
    <div class="direction-seam" aria-label="Authority direction"><span>REQUEST ↓ DELEGATE INWARD</span><span>RESULT ↑ RETURN OUTWARD</span></div>
    <ol class="runtime-strata" aria-label="Inward runtime ownership order">
      {#each runtimeLayers as layer, index (layer.id)}
        <li id={'runtime-' + layer.id} class="runtime-layer" class:focused={focused === layer.id} data-layer={layer.id} style={'--layer:' + index}>
          <div class="layer-surface">
            <header><span class="layer-coordinate">{String(index + 1).padStart(2, '0')}</span><button aria-pressed={focused === layer.id && precision === 2} aria-label={'Inspect ' + layer.concept} onclick={() => inspect(layer.id)}><strong>{layer.concept}</strong><span>{layer.role}</span><b aria-hidden="true">↗</b></button></header>
            <div class="ownership" inert={precision !== 2} aria-hidden={precision !== 2}>
              <div class="ownership-content">
              <p>{layer.correspondence}</p>
              <ul aria-label={layer.role + ' crates'}>
                {#each layer.owners as owner}
                  <li data-owner={owner.name}><SourceLink path={owner.path} label={owner.name} /><span>depends on → {owner.dependencies.length ? owner.dependencies.join(' · ') : 'no Cerebri crates'}</span></li>
                {/each}
              </ul>
              </div>
            </div>
            <div class="layer-section" aria-hidden="true"><i></i><i></i><i></i><span></span></div>
          </div>
          <p class="dependency-seam" hidden={precision !== 2}><span aria-hidden="true">↓</span>{layer.seam}</p>
        </li>
      {/each}
    </ol>
    <aside class="ownership-focus" aria-label="Focused ownership">
      <small>SECTION / {String(runtimeLayers.findIndex(layer => layer.id === focused) + 1).padStart(2, '0')}</small>
      <h3>{selected.concept}</h3><p>{selected.correspondence}</p>
      {#if precision === 2}
        <div class="focus-bracket"><small>OWNING IMPLEMENTATION</small>{#each selected.owners as owner}<code>{owner.name}</code>{/each}</div>
      {:else}
        <div class="focus-bracket"><small>INSPECTION AXIS</small><span>Concept → ownership → dependency</span></div>
      {/if}
      <p class="drawing-note">Exploded drawing of ownership. No live request or execution is being animated.</p>
      <a href={base + '/cpir/#scope-field'}>Inspect the same scope in CPIR →</a>
    </aside>
  </div>
  <section class="build-ingress" aria-labelledby="build-ingress-title">
    <div><small>SEPARATE BUILD-TIME PATH</small><h3 id="build-ingress-title">Site fixture input</h3></div>
    <ol aria-label="Build-time fixture path"><li><code>scripts/build-site-data.mjs</code></li><li><code>cerebri-core / example plan</code></li><li><code>generated runtimeData</code></li><li>Static site reads typed output</li></ol>
    <p>The site does not run the planner in the browser.</p>
  </section>
  <div class="architecture-boundaries">
    <section class="lifecycle-boundary" aria-labelledby="lifecycle-boundary-title">
      <small>LATER / EXPLICIT LIFECYCLE</small><h3 id="lifecycle-boundary-title">Validation follows proposal.</h3>
      <p>Candidate hard-rule checks belong to search. Lifecycle validation separately promotes ProposedPlan → ValidatedPlan in <code>cerebri-planner</code>.</p>
      <p><code>cerebri-integrations</code> owns execution ports and the later execution path. Authorization and preflight remain required; REST and Node do not expose execution.</p>
      <SourceLink path="crates/cerebri-planner/src/lifecycle.rs" label="Lifecycle boundary" />
    </section>
    <aside class="research-detachment" aria-labelledby="research-detachment-title" data-production-dependency="none">
      <small>FUTURE / RESEARCH</small><h3 id="research-detachment-title">Outside the runtime drawing</h3>
      <code>research/</code><p>Excluded from the production workspace. No production dependency enters this region.</p>
      <div class="metadata-boundary"><small>CURRENT WORKSPACE / SEPARATE CONTRACT</small><p><code>cerebri-ml</code> contains metadata and inference ports. It is not an active planning dependency.</p></div>
    </aside>
  </div>
  <a class="runtime-link" href={base + '/cpir/#result-title'}><span>Return to the same candidate record</span><code>candidates[0].proposed →</code></a>
</div>

<style>
  .concept-pipeline { list-style:none; margin:0 0 45px; padding:0; display:grid; grid-template-columns:repeat(4,1fr); border-block:1px solid var(--line); }
  .concept-pipeline li { position:relative; padding:22px 18px; display:grid; gap:8px; }
  .concept-pipeline li+li::before { content:'→'; position:absolute; top:42px; left:-8px; color:var(--field); }
  .concept-pipeline span { font:10px var(--mono); color:var(--muted); }
  .concept-pipeline strong { font-size:14px; }
  .concept-pipeline small { font-size:10px; }
  .architecture-drawing { position:relative; display:grid; grid-template-columns:minmax(0,1fr) 230px; column-gap:55px; padding:25px 10px 45px 50px; }
  .direction-seam { position:absolute; left:0; top:0; bottom:30px; display:flex; justify-content:space-between; width:28px; border-inline:1px solid var(--line); }
  .direction-seam span { writing-mode:vertical-rl; font:9px/1.4 var(--mono); color:#b6dbdd; padding:12px 0; }
  .direction-seam span:last-child { transform:rotate(180deg); color:#d3c4f1; }
  .runtime-strata { list-style:none; padding:0 18px 0 0; margin:0; min-width:0; perspective:1200px; }
  .runtime-layer { position:relative; scroll-margin-top:180px; z-index:calc(6 - var(--layer)); margin-top:-12px; width:100%; transition:width 750ms var(--ease-space),margin 750ms var(--ease-space),transform 750ms var(--ease-space); }
  .layer-surface { position:relative; border:1px solid var(--line); background:linear-gradient(120deg,#102b40,#07162a 80%); transform:rotateX(28deg) rotateZ(-7deg); transform-origin:50% 50%; padding:20px 24px 36px; transition:transform 750ms var(--ease-space),padding 750ms var(--ease-space),border-color 300ms; }
  .layer-surface::after { content:''; position:absolute; left:-1px; right:-1px; bottom:-9px; height:8px; border:1px solid #6998bb66; border-top:0; background:#091529; transform:skewX(-40deg); transform-origin:top; }
  .runtime-layer:nth-child(3) .layer-surface { background:linear-gradient(120deg,#123938,#09192c 85%); border-color:#76c8c277; }
  .layer-surface>header { display:flex; align-items:center; gap:18px; }
  .layer-coordinate { color:#8fb4c6; font:12px var(--mono); }
  .layer-surface button { display:grid; grid-template-columns:1fr 22px; gap:8px 14px; width:100%; background:transparent; border:0; text-align:left; padding:0; color:#e9f1fa; }
  .layer-surface button[aria-pressed="true"] { background:transparent; }
  .layer-surface button strong { font:500 19px/1.25 var(--sans); }
  .layer-surface button span { grid-column:1; font:11px var(--mono); color:#b7c7d9; }
  .layer-surface button b { grid-column:2; grid-row:1/3; align-self:center; color:var(--field); }
  .layer-section { display:flex; align-items:center; gap:7px; position:absolute; bottom:15px; left:62px; right:24px; }
  .layer-section i { display:block; width:8px; height:3px; border:1px solid #7faabb; }
  .layer-section span { flex:1; height:1px; background:linear-gradient(90deg,#7faabb55,transparent); }
  .ownership { display:grid; grid-template-rows:0fr; margin:0 0 0 30px; transition:grid-template-rows 750ms var(--ease-space),margin 750ms var(--ease-space); }
  .ownership-content { min-height:0; overflow:hidden; }
  .ownership p { font-size:12px; margin:0 0 12px; }
  .ownership ul { list-style:none; padding:0; margin:0; display:grid; grid-template-columns:repeat(auto-fit,minmax(165px,1fr)); gap:0 22px; }
  .ownership li { border-top:1px solid var(--line); padding:9px 0; }
  .ownership li>span { display:block; font:10px/1.7 var(--mono); color:#b9c6d7; margin-top:6px; overflow-wrap:anywhere; }
  .dependency-seam { position:relative; margin:14px 0 22px 26px; font:11px/1.6 var(--mono); max-width:52ch; padding-left:20px; }
  .dependency-seam>span { position:absolute; left:0; color:var(--field); }
  .runtime-layer:last-child .dependency-seam>span { display:none; }
  .ownership-focus { padding-top:65px; }
  .ownership-focus h3 { margin:12px 0; font-size:24px; }
  .ownership-focus p { font-size:13px; }
  .focus-bracket { display:flex; flex-direction:column; gap:14px; border-left:2px solid var(--field); padding:18px; margin:30px 0; background:linear-gradient(90deg,#17323777,transparent); }
  .focus-bracket span { color:#c7dddf; font-size:12px; }
  .drawing-note { border-top:1px solid var(--line); padding-top:20px; }
  .ownership-focus a { display:inline-block; padding:14px 0; color:var(--field); font-size:12px; }
  [data-precision="2"] .runtime-layer { margin-top:28px; width:calc(88% + var(--layer) * 3%); margin-left:auto; }
  [data-precision="2"] .layer-surface { transform:rotateZ(-2deg); padding-bottom:20px; border-right-color:transparent; }
  [data-precision="2"] .ownership { grid-template-rows:1fr; margin-top:16px; }
  [data-precision="2"] .layer-section { display:none; }
  [data-precision="2"] .runtime-layer.focused { transform:translateX(12px); }
  [data-precision="2"] .focused .layer-surface { border-color:#9ce4de; border-left-width:3px; }
  [data-precision="2"] .ownership-focus { position:sticky; top:160px; align-self:start; padding-top:15px; }
  .build-ingress { padding:30px 0; border-block:1px solid var(--line); display:grid; grid-template-columns:190px minmax(0,1fr); gap:16px 30px; }
  .build-ingress h3 { margin:9px 0; }
  .build-ingress ol { list-style:none; padding:0; margin:0; display:flex; flex-wrap:wrap; align-items:center; gap:12px; }
  .build-ingress li { font-size:12px; }
  .build-ingress li+li::before { content:'→'; margin-right:12px; color:var(--field); }
  .build-ingress p { grid-column:2; margin:0; font-size:12px; }
  .architecture-boundaries { display:grid; grid-template-columns:1fr 1fr; gap:60px; margin:50px 0; align-items:start; }
  .architecture-boundaries h3 { margin:10px 0 16px; }
  .architecture-boundaries p { font-size:13px; }
  .lifecycle-boundary { padding-left:22px; border-left:2px solid #eed6a1; }
  .research-detachment { padding:25px; border:1px dashed #af9cce88; background:linear-gradient(135deg,#37274522,transparent); }
  .research-detachment>small { color:#c8b2e2; }
  .metadata-boundary { margin-top:24px; padding-top:20px; border-top:1px solid #af9cce44; }
  @media(max-width:1000px) {
    .architecture-drawing { grid-template-columns:minmax(0,1fr) 190px; column-gap:25px; padding-left:40px; }
    .layer-surface { padding-inline:18px; }
    .layer-surface button strong { font-size:16px; }
  }
  @media(max-width:700px) {
    .concept-pipeline { grid-template-columns:1fr; }
    .concept-pipeline li { padding:20px 10px; }
    .concept-pipeline li+li::before { content:'↓'; top:-10px; left:10px; }
    .concept-pipeline strong { font-size:12px; }
    .architecture-drawing { display:flex; flex-direction:column; padding:0 0 30px 30px; }
    .direction-seam { width:22px; bottom:260px; }
    .runtime-strata { padding:0; perspective:none; }
    .runtime-layer { margin-top:12px; }
    .layer-surface { transform:none; padding:18px 12px 32px; }
    .layer-surface>header { gap:10px; }
    .layer-coordinate { font-size:10px; }
    .layer-surface button strong { font-size:15px; }
    .layer-surface button span { font-size:10px; }
    .ownership { margin-left:0; }
    .ownership ul { grid-template-columns:1fr; }
    .ownership-focus,[data-precision="2"] .ownership-focus { position:static; padding-top:28px; }
    .focus-bracket { margin:18px 0; }
    [data-precision="2"] .runtime-layer { width:100%; }
    [data-precision="2"] .runtime-layer.focused { transform:none; }
    [data-precision="2"] .layer-surface { transform:none; }
    .dependency-seam { margin-left:0; }
    .architecture-boundaries { grid-template-columns:1fr; gap:35px; margin:32px 0; }
    .build-ingress { grid-template-columns:1fr; }
    .build-ingress ol { flex-direction:column; align-items:flex-start; gap:15px; padding-left:18px; border-left:1px solid var(--line); }
    .build-ingress li+li::before { content:'↓'; display:block; margin-bottom:8px; }
    .build-ingress p { grid-column:1; }
    .research-detachment { margin-left:22px; padding:20px; }
  }
</style>
