<script lang="ts">
  import { onMount, type Snippet } from 'svelte';
  import CerebriBrain from './CerebriBrain.svelte';
  import { sceneIndex, sceneStops, clamp, ramp } from './brain-geometry';
  import { planner, ranked, request, candidates, candidateIdentity, timeLabel } from '$lib/planning-display';
  let { children }: { children?: Snippet } = $props();
  let host: HTMLElement;
  let surface: HTMLElement;
  let automatic = $state(0);
  let manual = $state<number | null>(null);
  let reducedMotion = $state(false);
  let mobile = $state(false);
  let narrow = $state(false);
  let pinned = $state(true);
  let activation = $state(0);
  const progress = $derived(manual === null ? automatic : sceneStops[manual]);
  const scene = $derived(manual ?? sceneIndex(progress));
  const knownRange = request.context.objects.find(object => object.id === 'busy')!.time.value.knowledge.data;
  const scenes = [
    { name: 'The instrument', title: 'A possibility has a place.', text: `${candidates.length} positions form one compact assembly. Open it to see the time, state and rules that give each position meaning.` },
    { name: 'Open the structure', title: 'Separate the layers. Keep the objects.', text: 'The assembly opens along a shared axis. These are conceptual layers of the example, not a diagram of Rust modules.' },
    { name: 'Time', title: 'Give possibility a coordinate.', text: `Each point settles onto a possible start time, ${request.granularity / 60} minutes apart. Duration, occupied times and rules arrive in separate data fields called CPIR; the current planner does not read a sentence.` },
    { name: 'Scope & state', title: 'The search has a boundary.', text: `The time window is ${timeLabel(request.scope.time_range.start)}–${timeLabel(request.scope.time_range.end)} UTC. The supplied appointment occupies ${timeLabel(knownRange.start)}–${timeLabel(knownRange.end)}. Scope limits the search; it does not grant permission.` },
    { name: 'Candidates', title: 'A start becomes an interval.', text: `The new appointment needs ${request.duration.value.knowledge.data / 60} minutes. Each point extends into that duration. The last possible start is ${timeLabel(candidates.at(-1)!.start)}, so the whole appointment fits before ${timeLabel(request.scope.time_range.end)}.` },
    { name: 'Validity', title: 'The conflict becomes visible.', text: `${planner.conflicts.rejections.length} intervals cross known busy time. Their cuts retain the evidence of rejection. The other ${ranked.length} pass these checks; preference never makes a conflict valid.` },
    { name: 'Inspect the result', title: 'Keep the valid possibilities.', text: `${ranked.length} candidates remain. ${candidateIdentity(ranked[0].start)} starts at ${timeLabel(ranked[0].start)} UTC and is first in the Rust-produced order. Below, inspect why it comes first and where its authority ends.` }
  ];

  onMount(() => {
    const media = matchMedia('(prefers-reduced-motion: reduce)');
    let frame = 0, activationFrame = 0, active = false, visible = false, disposed = false;
    let trackStart = 0, distance = 1, activationTime = 0, lastActivationTime = 0;
    const readProgress = () => {
      automatic = reducedMotion ? 1 : !pinned ? 0 : clamp((scrollY - trackStart) / distance);
      frame = 0;
    };
    const schedule = () => { if (!frame) frame = requestAnimationFrame(readProgress); };
    const measure = () => {
      if (disposed) return;
      reducedMotion = media.matches;
      mobile = innerWidth <= 700;
      narrow = innerWidth <= 1100;
      const top = Math.ceil(Math.max(...['.site-header', '.depth-control'].map(selector => document.querySelector(selector)?.getBoundingClientRect().bottom ?? 0)) + 16);
      host.style.setProperty('--observatory-top', top + 'px');
      const caption = surface.querySelector<HTMLElement>('.scene-explanation')!.offsetHeight;
      const controls = surface.querySelector<HTMLElement>('.scene-controls')!.offsetHeight;
      const identity = surface.querySelector<HTMLElement>('.identity-handoff')!.offsetHeight;
      const available = innerHeight - top - caption - controls - identity - 70;
      pinned = !reducedMotion && available >= 260;
      host.style.setProperty('--brain-height', (pinned ? Math.min(available, mobile ? 650 : 800) : mobile ? 520 : 680) + 'px');
      host.style.setProperty('--scroll-distance', (mobile ? 4.5 : 5.5) * innerHeight + 'px');
      requestAnimationFrame(() => {
        if (disposed) return;
        trackStart = host.getBoundingClientRect().top + scrollY - top;
        distance = Math.max(1, host.offsetHeight - surface.offsetHeight);
        readProgress();
      });
      if (reducedMotion) { activation = 1; cancelAnimationFrame(activationFrame); }
    };
    const awaken = (now: number) => {
      if (!active || media.matches || activation >= 1) { lastActivationTime = 0; return; }
      if (lastActivationTime) activationTime += Math.min(now - lastActivationTime, 50);
      lastActivationTime = now;
      activation = clamp(activationTime / 2200);
      if (activation < 1) activationFrame = requestAnimationFrame(awaken);
    };
    const activity = () => {
      const next = visible && !document.hidden;
      if (next !== active) {
        active = next;
        if (active) { window.addEventListener('scroll', schedule, { passive: true }); schedule(); activationFrame = requestAnimationFrame(awaken); }
        else { window.removeEventListener('scroll', schedule); cancelAnimationFrame(frame); frame = 0; cancelAnimationFrame(activationFrame); lastActivationTime = 0; }
      }
    };
    const observer = new IntersectionObserver(entries => { visible = entries[0].isIntersecting; activity(); }, { rootMargin: '100px' });
    const resize = new ResizeObserver(measure);
    observer.observe(host);
    resize.observe(surface.querySelector('.scene-explanation')!);
    resize.observe(surface.querySelector('.scene-controls')!);
    resize.observe(surface.querySelector('.identity-handoff')!);
    window.addEventListener('resize', measure);
    document.addEventListener('visibilitychange', activity);
    media.addEventListener('change', measure);
    document.fonts.ready.then(measure);
    measure();
    return () => {
      disposed = true; observer.disconnect(); resize.disconnect(); cancelAnimationFrame(frame); cancelAnimationFrame(activationFrame);
      window.removeEventListener('scroll', schedule); window.removeEventListener('resize', measure);
      document.removeEventListener('visibilitychange', activity); media.removeEventListener('change', measure);
    };
  });
</script>

<div class="cerebri-hero" class:flow-introduction={reducedMotion || !pinned || narrow}>
  <div class="hero-introduction" class:departed={!reducedMotion && pinned && !narrow && progress >= .12} style:opacity={!reducedMotion && pinned && !narrow ? 1 - ramp(progress, .02, .12) : 1}>{@render children?.()}</div>
  {#if reducedMotion}
    <div class="reduced-brain-overview" aria-hidden="true">
      <CerebriBrain progress={0} activation={1} {mobile} overview centered />
      <CerebriBrain progress={.4} activation={1} {mobile} overview centered />
    </div>
  {/if}
  <div class="observatory-track" class:static-view={!pinned} bind:this={host} id="planning-journey">
    <section class="observatory" class:calibrated={scene === 6} data-scene={scene} data-progress={progress.toFixed(4)} data-motion={reducedMotion ? 'reduced' : 'full'} aria-label="Appointment search: bounded search observatory" bind:this={surface}>
      <div class="arrival-instrument">
        <CerebriBrain {progress} {activation} {mobile} centered={narrow || reducedMotion || !pinned}/>
        <div class="calibration-caption"><span>REAL / APPOINTMENT EXAMPLE</span><small>{request.duration.value.knowledge.data / 60} MIN · {candidates.length} POSITIONS</small></div>
        <div class="identity-handoff" class:resolved={scene === 6}><span>{candidateIdentity(ranked[0].start)}</span><i aria-hidden="true"></i><b>{timeLabel(ranked[0].start)} UTC</b><span class="identity-legend"><small class:hidden={scene === 6}>CONCEPTUAL ASSEMBLY · ACTUAL CANDIDATE POSITIONS</small><small class:hidden={scene < 6}>SAME OPTION · EXPLORE THE RESULT</small></span></div>
      </div>
      <div class="scene-explanation" id="scene-explanation">
        <!-- Reserve the tallest explanation so copy edits cannot move scroll boundaries. -->
        {#each scenes as step, index}
          <div class="scene-copy" class:current={scene === index} aria-hidden={scene !== index}>
            <div><span class="scene-number">0{index + 1} / {step.name}</span><h2>{step.title}</h2></div>
            <p>{step.text}</p>
          </div>
        {/each}
      </div>
      <div class="scene-controls" role="group" aria-label="Planning journey stages">
        {#each scenes as step, index}
          <button aria-pressed={scene === index} aria-controls="scene-explanation" onclick={() => manual = index}><span>0{index + 1}</span>{step.name}</button>
        {/each}
        {#if manual !== null}<button class="follow-scroll" onclick={() => manual = null}>Follow scroll</button>{/if}
      </div>
      <div class="scroll-measure" aria-hidden="true"><i style={`transform:scaleX(${progress})`}></i></div>
    </section>
  </div>
</div>

<style>
  .cerebri-hero{position:relative;width:100%;--brain-height:clamp(260px,calc(100svh - 414px),800px)}
  .hero-introduction{position:absolute;top:3rem;left:0;width:34%;z-index:2}
  .hero-introduction.departed{visibility:hidden;pointer-events:none}
  .flow-introduction .hero-introduction{position:relative;top:auto;width:min(780px,100%);margin-bottom:2rem}
  .observatory-track{position:relative;height:calc(var(--brain-height,680px) + 250px + var(--scroll-distance,550vh));scroll-margin-top:9rem}
  .observatory{position:sticky;top:var(--observatory-top,130px);width:100%;isolation:isolate}
  .arrival-instrument{position:relative}
  .arrival-instrument :global(.cerebri-brain){height:var(--brain-height,680px)}
  .calibration-caption{position:absolute;inset:0 0 auto auto;display:flex;gap:1rem;justify-content:space-between;font:.75rem var(--mono);color:var(--text-3);width:52%;pointer-events:none}
  .calibration-caption span{color:var(--cyan)}.calibration-caption small{font:inherit}
  .identity-handoff{position:relative;display:flex;align-items:center;gap:12px;padding:.65rem 0;font:.8125rem var(--mono);color:var(--text-3)}
  .identity-handoff>span{color:var(--cyan)}.identity-handoff i{width:3rem;border-top:1px solid var(--line-strong)}.identity-handoff b{font-weight:400;color:var(--text-1)}.identity-handoff small{font:.75rem/1.6 var(--mono);letter-spacing:.035em;color:var(--text-3);grid-area:1/1}
  .identity-legend{display:grid;margin-left:auto}.identity-legend .hidden{visibility:hidden}
  .scene-explanation{display:grid;border-top:1px solid var(--line-strong);padding-block:1.2rem;min-height:8rem;margin-top:1rem}
  .scene-copy{grid-area:1/1;visibility:hidden;display:grid;grid-template-columns:minmax(0,.8fr) minmax(0,1.2fr);gap:3rem;align-items:center}
  .scene-copy.current{visibility:visible}
  .scene-number{font:.8125rem var(--mono);color:var(--cyan);text-transform:uppercase;letter-spacing:.05em}
  .scene-explanation h2{font-size:clamp(1.25rem,2vw,1.65rem);line-height:1.2;letter-spacing:-.025em;margin:.7rem 0 0}
  .scene-explanation p{font-size:1rem;line-height:1.6;color:var(--text-3);margin:0;max-width:65ch}
  .scene-controls{display:flex;gap:6px;overflow-x:auto;scrollbar-width:thin;max-width:100%;border-top:1px solid var(--line);padding-top:.5rem}
  .scene-controls button{display:flex;align-items:center;gap:9px;flex-shrink:0;background:none;border:1px solid transparent;border-bottom-color:var(--line);border-radius:0;min-height:44px;padding:10px 12px;color:var(--text-3);font:.8125rem var(--sans);cursor:pointer}
  .scene-controls button span{font:.6875rem var(--mono);color:var(--text-3)}
  .scene-controls button[aria-pressed=true]{border-color:var(--line-strong);color:var(--text-1);background:rgba(32,216,255,.055)}.scene-controls button[aria-pressed=true] span{color:var(--cyan)}
  .follow-scroll{margin-left:auto}
  .scroll-measure{height:1px;background:var(--line);margin-top:.6rem}.scroll-measure i{display:block;height:100%;background:linear-gradient(90deg,var(--cyan),var(--violet));transform-origin:left}
  .static-view{height:auto}.static-view .observatory{position:relative;top:auto}
  .reduced-brain-overview{display:grid;grid-template-columns:1fr 1fr;padding-top:0}
  .reduced-brain-overview :global(.cerebri-brain){height:500px}
  @media(max-width:1100px){.hero-introduction{position:relative;width:min(780px,100%);top:auto;margin-bottom:2rem}.calibration-caption{width:100%}.reduced-brain-overview{padding-top:0}}
  @media(max-width:700px){
    .hero-introduction{margin-bottom:2rem}.scene-copy{grid-template-columns:1fr;gap:.8rem}.scene-explanation{min-height:0;padding-block:1rem}.scene-explanation h2{font-size:1.25rem}.scene-explanation p{font-size:.9375rem;line-height:1.5}
    .calibration-caption{font-size:.75rem;gap:.5rem}.identity-handoff small{font-size:.75rem;max-width:none;text-align:left}.identity-handoff{font-size:.8125rem;gap:6px;flex-wrap:wrap}.identity-handoff i{width:1.2rem}.identity-legend{flex-basis:100%}
    .scene-controls button{padding:9px}.reduced-brain-overview{grid-template-columns:1fr}.reduced-brain-overview :global(.cerebri-brain){height:520px}
  }
</style>

