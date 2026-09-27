<script lang="ts">
  import { onMount } from 'svelte';
  import { base } from '$app/paths';
  import { ranked, candidateIdentity } from '$lib/planning-display';
  import { runtimeLayers } from '$lib/architecture-ownership';
  import RangeField from './technical/RangeField.svelte';
  import './technical/technical.css';
  const placement = ranked[0].proposed.placements[0];
  const identity = candidateIdentity(ranked[0].start);
  const layers = runtimeLayers.filter(layer => ['core', 'planner', 'foundations'].includes(layer.id));
  let precision = $state(0);
  let reduced = $state(false);
  onMount(() => {
    const media = matchMedia('(prefers-reduced-motion: reduce)');
    const update = () => { reduced = media.matches; if (reduced) precision = 2; };
    update(); media.addEventListener('change', update);
    return () => media.removeEventListener('change', update);
  });
</script>

<section class="representation-descent" id="structured-descent" data-world="technical" data-motion={reduced ? 'reduced' : 'full'} aria-labelledby="descent-title">
  <header><span class="kicker">THE SAME PLAN / A MORE EXACT VIEW</span><h2 id="descent-title">The geometry has a type.</h2><p>The interval you followed has two endpoints and an object identity. Open its structure: the values remain the same.</p></header>
  <div class="descent-controls" role="group" aria-label="Placement representation">
    {#each ['Interval', 'Named fields', 'Exact record'] as label, index}<button aria-pressed={precision === index} onclick={() => precision = index}>{label}</button>{/each}
  </div>
  <div class="descent-record" data-precision={precision}>
    <div class="descent-identity"><span>{identity}</span><strong>{placement.object_id}</strong><small>PLACEMENT IN ProposedPlan</small></div>
    <RangeField label="The selected interval" path="placements[0].range" start={placement.range.start} end={placement.range.end} {precision} kind="candidate" />
    <div class="descent-notation"><code>[start, end)</code><p>Start included. End excluded. The representation is more explicit; the plan has gained no execution authority.</p></div>
  </div>
  <div class="software-descent">
    <div><span class="kicker">FROM REPRESENTATION TO SOFTWARE</span><h3>Responsibility moves inward.</h3><p>CPIR describes the planning request. The planner returns typed candidate records. The website displays those records; Rust owns their meaning.</p><a href={base + '/cpir/'}>Inspect the structured request <span aria-hidden="true">→</span></a></div>
    <ol aria-label="Selected Rust ownership layers">
      {#each layers as layer}<li><small>{layer.role}</small><strong>{layer.owners.map(owner => owner.name).join(' + ')}</strong><p>{layer.seam}</p></li>{/each}
    </ol>
  </div>
  <footer><p>These are selected ownership layers, not the complete dependency graph. The assembled opening instrument was a conceptual view of this example.</p><a href={base + '/architecture/#runtime-planner'}>Continue into the architecture <span aria-hidden="true">→</span></a></footer>
</section>

<style>
  .descent-record{--field:var(--cyan)}.descent-record :global(dl),.descent-record :global(dd){margin:0}.descent-record :global([data-precision='2'] .range-geometry){height:auto}.descent-record :global([data-precision='2'] .range-endpoints){display:grid;gap:1.5rem}.descent-record :global([data-precision='2'] .range-endpoint){position:relative;top:auto}.descent-record :global([data-precision='2'] .range-connector){bottom:1rem;height:auto}
  @media(min-width:761px){.descent-record :global(.range-field){grid-column:2;grid-row:1 / span 2}.descent-record .descent-notation{grid-column:1;grid-row:2;display:block}.descent-record .descent-notation p{margin-top:.75rem}}
  .representation-descent{position:relative;max-width:1280px;margin:0 auto;padding:4rem 2rem;border-top:1px solid var(--line);scroll-margin-top:10rem}.representation-descent h2{font-size:clamp(2rem,4vw,3.5rem);line-height:1.1;letter-spacing:-.045em;margin:1rem 0}.representation-descent p{font-size:1rem;line-height:1.7;color:var(--text-3);max-width:64ch}.descent-controls{display:flex;flex-wrap:wrap;gap:.5rem;margin:2rem 0}.descent-controls button{border:1px solid var(--line);padding:.75rem 1rem;background:none;color:var(--text-2);font-size:.875rem;min-height:44px;cursor:pointer}.descent-controls button[aria-pressed=true]{border-color:var(--cyan);color:var(--text-1);background:rgba(32,216,255,.05)}
  .descent-record{display:grid;grid-template-columns:minmax(0,.65fr) minmax(0,1.35fr);gap:1.5rem 3rem;padding:2rem 0;border-block:1px solid var(--line-strong)}.descent-identity{display:grid;align-content:start;gap:.75rem}.descent-identity>span{color:var(--cyan);font:1rem var(--mono)}.descent-identity strong{font:1.5rem var(--mono);overflow-wrap:anywhere}.descent-identity small{font:.8125rem/1.6 var(--mono);color:var(--text-3)}.descent-notation{grid-column:2;display:flex;align-items:start;gap:1.5rem}.descent-notation>code{font:1rem/1.7 var(--mono);white-space:nowrap;color:var(--cyan)}.descent-notation p{margin:0;font-size:.9375rem}.software-descent{display:grid;grid-template-columns:minmax(0,1fr) minmax(0,1fr);gap:3rem;padding-top:3rem}.software-descent h3{font-size:1.75rem;line-height:1.2;margin:1rem 0}.software-descent ol{padding:0;margin:0;list-style:none}.software-descent li{padding:0 0 1.5rem 1.5rem;border-left:1px solid var(--line-strong)}.software-descent li:last-child{padding-bottom:0}.software-descent li small{font:.8125rem var(--mono);color:var(--text-3)}.software-descent li strong{display:block;font:.9375rem/1.6 var(--mono);margin-top:.5rem;overflow-wrap:anywhere}.software-descent li p{font-size:.9375rem;margin:.5rem 0 0}.representation-descent a{display:inline-flex;align-items:center;gap:1rem;min-height:44px;font-size:1rem;color:var(--cyan)}.representation-descent footer{display:flex;justify-content:space-between;align-items:center;gap:2rem;margin-top:2.5rem;border-top:1px solid var(--line);padding-top:1.5rem}.representation-descent footer p{max-width:60ch;font-size:.875rem}.representation-descent footer a{flex-shrink:0}
  .descent-record :global(.range-endpoint dt){font-size:.8125rem}.descent-record :global(.range-endpoint time){font-size:.875rem}.descent-record :global(.range-heading>code){font-size:.8125rem;overflow-wrap:anywhere}.descent-record :global(.range-geometry){min-height:166px}
  @media(max-width:760px){.representation-descent{padding:3rem 1rem}.descent-record,.software-descent{grid-template-columns:1fr;gap:2rem}.descent-notation{grid-column:auto;flex-direction:column;gap:.75rem}.representation-descent footer{display:block}.descent-controls button{flex:1;padding:.75rem .5rem}.descent-controls{gap:.35rem}}
</style>
