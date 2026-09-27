<script lang="ts">
  import { onMount, type Snippet } from 'svelte';
  import CandidateField from './CandidateField.svelte';
  import { planner, ranked, request, candidates, candidateIdentity, timeLabel } from '$lib/planning-display';
  let { children }: { children?: Snippet } = $props();
  let host: HTMLElement;
  let automatic = $state(0);
  let manual = $state<number | null>(null);
  let reducedMotion = $state(false);
  let pinned = $state(false);
  const scene = $derived(manual ?? automatic);
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
    let frame = 0;
    let active = false;
    let visible = false;
    const update = () => {
      const surface = host.querySelector<HTMLElement>('.observatory')!;
      const rect = host.getBoundingClientRect();
      const height = surface.offsetHeight;
      const top = Math.ceil(Math.max(...['.site-header', '.depth-control'].map(selector => document.querySelector(selector)?.getBoundingClientRect().bottom ?? 0)) + 20);
      reducedMotion = media.matches;
      pinned = !reducedMotion && height + top + 32 <= innerHeight;
      automatic = reducedMotion ? 6 : !pinned ? 0 : Math.min(6, Math.max(0, Math.floor((top - rect.top) / Math.max(rect.height - height, 1) * 7)));
      host.style.setProperty('--observatory-top', top + 'px');
      host.style.setProperty('--scene-height', height + 'px');
    };
    const schedule = () => { cancelAnimationFrame(frame); frame = requestAnimationFrame(update); };
    const activity = () => {
      const next = visible && !document.hidden;
      if (next !== active) {
        active = next;
        if (active) window.addEventListener('scroll', schedule, { passive: true });
        else window.removeEventListener('scroll', schedule);
      }
      if (active) schedule(); else cancelAnimationFrame(frame);
    };
    const observer = new IntersectionObserver(entries => { visible = entries[0].isIntersecting; activity(); }, { rootMargin: '100px' });
    const resize = new ResizeObserver(schedule);
    observer.observe(host);
    resize.observe(host.querySelector('.observatory')!);
    window.addEventListener('resize', schedule);
    document.addEventListener('visibilitychange', activity);
    media.addEventListener('change', schedule);
    update();
    return () => {
      observer.disconnect(); resize.disconnect(); cancelAnimationFrame(frame);
      window.removeEventListener('scroll', schedule); window.removeEventListener('resize', schedule);
      document.removeEventListener('visibilitychange', activity); media.removeEventListener('change', schedule);
    };
  });
</script>

<div class="observatory-track" class:static-view={!pinned} bind:this={host} id="planning-journey">
  <section class="observatory" class:calibrated={scene === 6} data-scene={scene} data-motion={reducedMotion ? 'reduced' : 'full'} aria-label="Appointment search: bounded search observatory">
    <div class="arrival-copy">
      {@render children?.()}
      <div class="scene-explanation" id="scene-explanation">
        <!-- Reserve the tallest explanation so a scene change cannot move its scroll boundaries. -->
        {#each scenes as step, index}
          <div class="scene-copy" class:current={scene === index} aria-hidden={scene !== index}>
            <span class="scene-number">0{index + 1} / {step.name}</span>
            <h2>{step.title}</h2>
            <p>{step.text}</p>
          </div>
        {/each}
      </div>
    </div>
    <div class="arrival-instrument">
      <div class="calibration-caption"><span>REAL / APPOINTMENT EXAMPLE</span><small>{request.duration.value.knowledge.data / 60} MIN · {candidates.length} POSITIONS</small></div>
      <CandidateField phase={scene} context="hero" />
      <div class="identity-handoff" class:resolved={scene === 6}><span>{candidateIdentity(ranked[0].start)}</span><i aria-hidden="true"></i><b>{timeLabel(ranked[0].start)} UTC</b><small>{scene < 6 ? 'CONCEPTUAL ASSEMBLY · ACTUAL CANDIDATE POSITIONS' : 'SAME OPTION · EXPLORE THE RESULT'}</small></div>
    </div>
    <div class="scene-controls" role="group" aria-label="Planning journey stages">
      {#each scenes as step, index}
        <button aria-pressed={scene === index} aria-controls="scene-explanation" onclick={() => manual = index}><span>0{index + 1}</span>{step.name}</button>
      {/each}
      {#if manual !== null}<button class="follow-scroll" onclick={() => manual = null}>Follow scroll</button>{/if}
    </div>
  </section>
</div>

<style>
  .observatory-track{position:relative;width:100%;height:calc(var(--scene-height,740px) + 760px);scroll-margin-top:9rem}
  .observatory{position:sticky;top:var(--observatory-top,145px);display:grid;grid-template-columns:minmax(0,.8fr) minmax(0,1.2fr);gap:2rem 3rem;width:100%;isolation:isolate}
  .arrival-copy{align-self:center;min-width:0}.arrival-instrument{min-width:0;align-self:center}
  .calibration-caption{display:flex;justify-content:space-between;flex-wrap:wrap;gap:10px;padding:0 6%;font:.75rem var(--mono);color:var(--text-3)}.calibration-caption span{color:var(--cyan)}.calibration-caption small{font:inherit}
  .scene-explanation{display:grid;border-top:1px solid var(--line-strong);padding-top:1.25rem;margin-top:1.75rem;min-height:12rem}.scene-copy{grid-area:1/1;visibility:hidden}.scene-copy.current{visibility:visible}.scene-number{font:.8125rem var(--mono);color:var(--cyan);text-transform:uppercase;letter-spacing:.05em}.scene-explanation h2{font-size:clamp(1.25rem,2vw,1.65rem);line-height:1.2;letter-spacing:-.025em;margin:1rem 0 .7rem}.scene-explanation p{font-size:1rem;line-height:1.65;color:var(--text-3);margin:0;max-width:48ch}
  .identity-handoff{display:flex;align-items:center;flex-wrap:wrap;gap:12px;margin:0 6%;font:.8125rem var(--mono);color:var(--text-3)}.identity-handoff>span{color:var(--cyan)}.identity-handoff i{flex:1;min-width:24px;border-top:1px solid var(--line-strong)}.identity-handoff b{font-weight:400;color:var(--text-1)}.identity-handoff small{width:100%;font:.6875rem/1.6 var(--mono);letter-spacing:.04em}
  .scene-controls{grid-column:1/-1;display:flex;gap:6px;flex-wrap:wrap;border-top:1px solid var(--line);padding-top:1rem}.scene-controls button{display:flex;align-items:center;gap:9px;background:none;border:1px solid transparent;border-bottom-color:var(--line);border-radius:0;min-height:44px;padding:10px 12px;color:var(--text-3);font:.8125rem var(--sans);cursor:pointer}.scene-controls button span{font:.6875rem var(--mono);color:var(--text-3)}.scene-controls button[aria-pressed=true]{border-color:var(--line-strong);color:var(--text-1);background:rgba(32,216,255,.055)}.scene-controls button[aria-pressed=true] span{color:var(--cyan)}.follow-scroll{margin-left:auto}
  .static-view{height:auto}.static-view .observatory{position:relative;top:auto}
  @media(max-width:1100px){.observatory{grid-template-columns:1fr;gap:2rem}.arrival-copy{display:grid;grid-template-columns:1fr 1fr;gap:2rem}.scene-explanation{margin:0;align-self:center}.arrival-instrument{width:min(800px,100%);margin:auto}}
  @media(max-width:700px){.arrival-copy{display:contents}.observatory :global(.hero-copy){grid-row:1}.scene-explanation{grid-row:4;margin:0;min-height:0}.scene-controls{gap:4px}.scene-controls button{padding:9px 7px}.calibration-caption,.identity-handoff{padding:0;margin-inline:0}.identity-handoff small{font-size:.75rem}}
  @media(max-width:1100px){.scene-controls{grid-row:2}.arrival-instrument{grid-row:3}}
</style>
