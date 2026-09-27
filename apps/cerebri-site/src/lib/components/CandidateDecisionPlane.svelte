<script lang="ts">
  import { onMount } from 'svelte';
  import { ranked, candidateIdentity, timeLabel, keyFields, decisiveIndex } from '$lib/planning-display';
  let { flowStage, reducedMotion }: { flowStage: number; reducedMotion: boolean } = $props();
  let manual = $state<number | null>(null);
  let inspected = $state(decisiveIndex);
  let mounted = $state(false);
  const mode = $derived(manual ?? (flowStage < 1 ? 0 : flowStage < 2 ? 1 : 2));
  const terminal = Math.max(0, decisiveIndex);
  onMount(() => { mounted = true; });
  $effect(() => {
    if (!mounted) return;
    if (reducedMotion || mode !== 1) { inspected = terminal; return; }
    inspected = 0;
    const timers = Array.from({ length: terminal }, (_, index) => setTimeout(() => inspected = index + 1, (index + 1) * 650));
    return () => timers.forEach(clearTimeout);
  });
</script>

<section class="deterministic-comparator" aria-label="Deterministic comparator" data-mode={mode} data-inspected={inspected} data-motion={reducedMotion ? 'reduced' : 'full'}>
  <header><small>DETERMINISTIC COMPARATOR</small><h3>First two Rust-ranked candidates</h3><p>The browser does not rank candidates. It reads the Rust-produced ordering keys and highlights the first field where the already ordered candidates differ.</p></header>
  <div class="representation-controls" role="group" aria-label="Candidate representation">
    {#each ['Candidate capsules', 'Inspect comparison', 'Resolve proposal'] as label, index}<button aria-pressed={mode === index} onclick={() => manual = index}>{label}</button>{/each}
    {#if manual !== null}<button class="follow-scroll" onclick={() => manual = null}>Follow scroll</button>{/if}
  </div>
  <div class="decision-surface">
    <div class="decision-scale" aria-hidden="true">
      {#each keyFields as field, index}<div class="comparator-row" class:decisive={index === decisiveIndex} class:examined={index <= inspected} class:later={index > terminal} data-key-field={field.key}><span>0{index + 1}</span><b>{field.label}</b><small>{index === decisiveIndex ? 'FIRST DIFFERENCE' : index < terminal ? 'EQUAL' : 'NOT NEEDED'}</small></div>{/each}
    </div>
    <div class="decision-aperture" aria-hidden="true" style={`--dimension:${inspected}`}><i></i><i></i></div>
    {#each ranked.slice(0, 2) as candidate, index (candidate.start)}
      <div class="decision-record" class:winner={index === 0} data-candidate-id={candidateIdentity(candidate.start)} style={`--row:${index}`}>
        <div class="persistent-candidate"><i class="identity-endpoint"></i><span>{candidateIdentity(candidate.start)}</span><b>{timeLabel(candidate.start)} UTC</b><i class="interval-end"></i></div>
        <dl class="record-dimensions">
          {#each keyFields as field, dimension}<div class:active={dimension === inspected} class:decisive={dimension === decisiveIndex} class:later={dimension > terminal}><dt>{field.label}</dt><dd>{field.render(candidate.ordering_key[field.key])}</dd></div>{/each}
        </dl>
        <small class="representation-label">{mode === 2 && index === 0 ? 'SELECTED PROPOSAL' : 'VALID CANDIDATE'}</small>
      </div>
    {/each}
    <div class="proposal-destination" aria-hidden="true"><span>PROPOSAL RAIL</span><i></i><b>AIR GAP</b></div>
    <div class="authority-plane" aria-hidden="true"><i></i><span>AUTHORITY</span><small>SEPARATE STATE</small></div>
  </div>
  <p class="comparator-resolution">First differing key: <strong>{decisiveIndex >= 0 ? keyFields[decisiveIndex].label : 'none'}</strong>. The current no-preference fixture resolves at the start-time tie-break after earlier key fields remain equal.</p>
  <p class="identity-equivalent">{candidateIdentity(ranked[0].start)} · {timeLabel(ranked[0].start)} UTC remains the same candidate in the field, comparison and proposal. Planning stops before authority.</p>
</section>

<style>
  .deterministic-comparator{position:relative;padding:36px 0 20px;border-block:1px solid var(--line);--columns:5;--decision-cyan:var(--cyan)}
  header{max-width:700px}header>small{font:.8125rem var(--mono);color:var(--cyan);letter-spacing:.08em}h3{font-size:clamp(22px,3vw,34px);margin:12px 0;letter-spacing:-.035em}header p,.comparator-resolution,.identity-equivalent{color:var(--text-3);font-size:.9375rem;line-height:1.7}
  .representation-controls{display:flex;flex-wrap:wrap;gap:8px;margin:24px 0 36px}.representation-controls button{background:none;border:0;border-bottom:1px solid var(--line);color:var(--text-3);padding:12px 10px;min-height:3.5rem;font-size:.8125rem;cursor:pointer}.representation-controls button[aria-pressed=true]{color:var(--text-1);border-color:var(--cyan)}.follow-scroll{margin-left:auto}
  .decision-surface{height:26rem;position:relative;border-bottom:1px solid var(--line);isolation:isolate}
  .decision-scale{display:grid;grid-template-columns:repeat(5,minmax(0,1fr));margin-left:22%;transition:opacity 350ms;opacity:0}
  .comparator-row{padding:0 12px 20px;display:grid;gap:8px;min-width:0;position:relative;border-top:1px solid var(--line)}.comparator-row>span{font:.8125rem var(--mono);color:var(--text-3);padding-top:8px}.comparator-row b{font-size:.8125rem;font-weight:500;overflow-wrap:anywhere}.comparator-row small{font:.8125rem var(--mono);color:var(--text-3);letter-spacing:.02em}.comparator-row.decisive small{color:var(--cyan)}.comparator-row.later{opacity:.4}
  .decision-aperture{position:absolute;left:calc(22% + var(--dimension) * 15.6%);width:15.6%;top:0;height:18rem;background:linear-gradient(180deg,rgba(32,216,255,.06),transparent);opacity:0;transition:left 500ms var(--ease-resolve),opacity 300ms}
  .decision-aperture i{position:absolute;inset:0;border-inline:1px solid var(--line-strong)}.decision-aperture i+i{inset:8px -5px;border-inline:0;border-block:1px solid var(--cyan);opacity:.5}
  .decision-record{position:absolute;left:0;right:0;top:calc(8rem + var(--row) * 5.5rem);height:4rem;transition:top 950ms var(--ease-resolve),opacity 450ms;display:flex;align-items:center}
  .persistent-candidate{position:absolute;left:0;width:20%;height:2.5rem;border-block:1px solid var(--line-strong);display:flex;align-items:center;gap:10px;padding:0 12px;color:var(--cyan);background:linear-gradient(90deg,rgba(32,216,255,.12),transparent);transition:width 750ms var(--ease-resolve),left 950ms var(--ease-resolve),height 700ms;border-radius:0 15px 15px 0}
  .persistent-candidate>span{font:.8125rem var(--mono)}.persistent-candidate>b{font:.8125rem var(--mono);color:var(--text-1);white-space:nowrap}.identity-endpoint,.interval-end{position:absolute;left:0;top:50%;width:8px;height:8px;background:var(--cyan);border-radius:50%;transform:translate(-50%,-50%)}.interval-end{left:100%;background:var(--space-1);border:1px solid var(--cyan)}
  .record-dimensions{position:absolute;left:22%;right:0;display:grid;grid-template-columns:repeat(5,minmax(0,1fr));margin:0;transform-origin:left;transition:transform 750ms var(--ease-resolve),opacity 300ms 200ms;transform:scaleX(.05);opacity:0}
  .record-dimensions>div{min-width:0;padding:15px 12px;border-bottom:1px solid var(--line);transition:background 300ms,color 300ms}.record-dimensions dt{position:absolute;width:1px;height:1px;overflow:hidden;clip-path:inset(50%)}.record-dimensions dd{margin:0;font:.8125rem var(--mono);overflow-wrap:anywhere}.record-dimensions .active{color:var(--text-1);background:rgba(32,216,255,.07)}.record-dimensions .later{color:var(--text-4)}
  .representation-label{position:absolute;left:0;top:3.2rem;font:.8125rem var(--mono);color:var(--text-3);transition:opacity 350ms}
  .proposal-destination{position:absolute;left:0;right:26%;top:21rem;height:2.5rem;border-bottom:1px solid var(--line-strong);opacity:.3;transition:opacity 400ms}
  .proposal-destination span{position:absolute;top:3rem;font:.8125rem var(--mono);color:var(--text-3)}.proposal-destination b{position:absolute;right:-3.5rem;top:1.6rem;width:3rem;font:.8125rem var(--mono);color:var(--amber);text-align:center}
  .authority-plane{position:absolute;right:0;bottom:0;width:21%;height:6rem;border-left:2px solid var(--amber);background:repeating-linear-gradient(110deg,transparent 0 12px,rgba(255,199,102,.035) 12px 13px);padding:12px;transform:skewY(-4deg);color:var(--amber)}.authority-plane span,.authority-plane small{display:block;font:.8125rem/1.8 var(--mono)}.authority-plane small{color:var(--text-3);font-size:.8125rem}
  [data-mode='0'] .decision-surface{height:20rem}[data-mode='0'] .decision-record{top:calc(2rem + var(--row) * 5.5rem)}[data-mode='2'] .decision-surface{height:18rem}[data-mode='2'] .proposal-destination{top:13rem}
  [data-mode='0'] .persistent-candidate{width:30%;left:calc(var(--row) * 8%)}
  [data-mode='1'] .record-dimensions{transform:none;opacity:1}[data-mode='1'] .decision-scale,[data-mode='1'] .decision-aperture{opacity:1}
  [data-mode='2'] .decision-record.winner{top:12.25rem}[data-mode='2'] .winner .persistent-candidate{left:40%;width:30%;transition-delay:200ms}[data-mode='2'] .winner .representation-label{left:40%;transition:left 950ms var(--ease-resolve) 200ms}
  [data-mode='2'] .decision-record:not(.winner){opacity:.4;top:3rem}[data-mode='2'] .proposal-destination{opacity:1}
  .comparator-resolution strong{color:var(--cyan)}.identity-equivalent{border-left:1px solid var(--cyan);padding-left:14px}
  @media(max-width:1024px){
    .decision-surface{height:46rem}[data-mode='0'] .decision-surface{height:22rem}[data-mode='2'] .decision-surface{height:20rem}[data-mode='2'] .proposal-destination{top:auto}[data-mode='2'] .decision-record:not(.winner){top:0}.decision-scale{display:none}.decision-aperture{left:0!important;top:calc(5.5rem + var(--dimension) * 5.5rem);height:5.5rem;width:100%;transition:top 500ms var(--ease-resolve)}
    .decision-record{top:0;left:calc(var(--row) * 52%);width:48%;right:auto;height:34rem;align-items:flex-start;transition:top 850ms var(--ease-resolve),left 850ms var(--ease-resolve)}
    .persistent-candidate,[data-mode='0'] .persistent-candidate{width:100%;left:0;height:3.5rem;flex-wrap:wrap;gap:0;padding:.3rem .4rem}.persistent-candidate>b{width:100%;font-size:.8125rem;white-space:normal;overflow-wrap:anywhere}.record-dimensions{top:5.5rem;left:0;right:0;grid-template-columns:1fr;transform:translateY(-20px);gap:0;transition:transform 650ms var(--ease-resolve),opacity 350ms}
    .record-dimensions>div{height:5.5rem;padding:10px 8px}.record-dimensions dt{overflow-wrap:anywhere;position:static;clip-path:none;width:auto;height:auto;font-size:.8125rem;color:var(--text-3);margin-bottom:8px}.record-dimensions dd{font-size:.8125rem}.representation-label{top:4rem;font-size:.8125rem}
    .record-dimensions .decisive.active{position:relative}.record-dimensions .decisive.active::after{content:'FIRST DIFFERENCE';position:absolute;right:8px;bottom:3px;font:.8125rem var(--mono);color:var(--cyan)}
    [data-mode='0'] .decision-record{top:2rem}[data-mode='0'] .decision-record:not(.winner){top:8rem}
    [data-mode='2'] .decision-record.winner{top:10rem;left:4%;width:54%}[data-mode='2'] .winner .persistent-candidate{width:100%;left:0}[data-mode='2'] .winner .representation-label{left:0;transition:none}
    .proposal-destination{top:auto;bottom:3.5rem;right:38%}.authority-plane{width:24%;height:8rem;padding:.75rem .5rem}.authority-plane span{font-size:.8125rem;overflow-wrap:anywhere}.authority-plane small{font-size:.8125rem;overflow-wrap:anywhere}.proposal-destination b{display:none}
    .representation-controls{gap:4px}.representation-controls button{padding-inline:6px;font-size:.8125rem}
  }
  @media(prefers-reduced-motion:reduce){.deterministic-comparator *{transition:none!important}.decision-aperture{display:none}.record-dimensions{transform:none!important}[data-mode='1'] .comparator-row.examined{border-top-color:var(--cyan)}}
</style>
