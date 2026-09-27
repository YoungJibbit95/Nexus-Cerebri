<script lang="ts">
  import { onMount } from 'svelte';
  import CandidateField from './CandidateField.svelte';
  import { ranked, candidateIdentity, timeLabel } from '$lib/planning-display';
  let host: HTMLElement;
  let scene = $state(0);
  let reducedMotion = $state(false);
  let pinned = $state(true);
  const scenes = ['Possible options', 'Finding a time', 'Options → times', 'Search window', 'Start → appointment', 'Checking overlaps', 'The checked options'];
  onMount(() => {
    const media = matchMedia('(prefers-reduced-motion: reduce)');
    let frame = 0;
    const update = () => {
      reducedMotion = media.matches;
      const surface = host.querySelector<HTMLElement>('.observatory');
      const headerBottom = Math.max(...['.site-header', '.depth-control'].map(selector => document.querySelector(selector)?.getBoundingClientRect().bottom ?? 0));
      const top = Math.ceil(headerBottom + 20);
      host.style.setProperty('--observatory-top', top + 'px');
      pinned = (surface?.offsetHeight ?? 0) + top + 32 <= innerHeight;
      const rect = host.getBoundingClientRect();
      scene = reducedMotion || !pinned ? 6 : Math.min(6, Math.max(0, Math.floor((top - rect.top) / Math.max(rect.height - (surface?.offsetHeight ?? 0), 1) * 7)));
    };
    const schedule = () => { cancelAnimationFrame(frame); frame = requestAnimationFrame(update); };
    update();
    window.addEventListener('scroll', schedule, { passive: true });
    window.addEventListener('resize', schedule);
    media.addEventListener('change', schedule);
    const resize = new ResizeObserver(schedule);
    resize.observe(host.querySelector('.observatory')!);
    return () => { resize.disconnect(); cancelAnimationFrame(frame); window.removeEventListener('scroll', schedule); window.removeEventListener('resize', schedule); media.removeEventListener('change', schedule); };
  });
</script>

<div class="observatory-track" class:static-view={!pinned || reducedMotion} bind:this={host}>
  <div class="observatory" class:calibrated={scene === 6} data-scene={scene} data-motion={reducedMotion ? 'reduced' : 'full'} role="img"
    aria-label={"Appointment search. " + scenes[scene] + ". Rust checks eleven possible starts. Four overlap the existing appointment; seven pass. First in the result: " + candidateIdentity(ranked[0].start) + ", " + timeLabel(ranked[0].start) + " UTC. Nothing is booked."}>
    <div class="calibration-caption" aria-hidden="true"><span>0{scene + 1} / SEARCH STEP</span><strong>{scenes[scene]}</strong><small>SPACE → TIME → PLANNING</small></div>
    <CandidateField phase={scene} context="hero" />
    <div class="identity-handoff" aria-hidden="true" class:resolved={scene === 6}><span>{candidateIdentity(ranked[0].start)}</span><i></i><b>{timeLabel(ranked[0].start)} UTC</b><small>SAME OPTION · EXPLORE THE RESULT</small></div>
  </div>
</div>

<style>
  .observatory-track{position:relative;margin:3rem auto 0;width:min(1120px,100%);height:1250px}
  .observatory{position:sticky;top:var(--observatory-top,145px);width:100%;isolation:isolate;overflow:clip;padding:30px 10px}
  .calibration-caption{display:flex;align-items:baseline;gap:24px;padding:0 7%;font:.8125rem var(--mono);color:var(--text-3)}
  .calibration-caption strong{color:var(--text-1);font:400 clamp(18px,2.3vw,28px) var(--sans);letter-spacing:-.03em}.calibration-caption small{margin-left:auto;font-size:.8125rem}
  .identity-handoff{display:flex;align-items:center;flex-wrap:wrap;gap:14px;margin:0 7%;font:.8125rem var(--mono);opacity:.65;transition:opacity 700ms}
  .identity-handoff.resolved{opacity:1}.identity-handoff>span{color:var(--cyan)}.identity-handoff i{flex:1;min-width:24px;border-top:1px solid var(--line-strong);position:relative}.identity-handoff i::before{content:'';position:absolute;left:0;top:-4px;width:7px;height:7px;background:var(--cyan);border-radius:50%}.identity-handoff small{font-size:.8125rem;color:var(--text-3)}
  @media(max-width:760px){.observatory-track{height:1450px;margin-top:2rem}.observatory{padding-inline:0}.calibration-caption{gap:10px;flex-wrap:wrap}.calibration-caption small{display:none}.calibration-caption strong{width:100%}.identity-handoff small{width:100%}}
  .observatory-track.static-view{height:auto}.static-view .observatory{position:relative;top:auto}
  @media(prefers-reduced-motion:reduce){.observatory-track{height:auto}.observatory{position:relative;top:auto}.identity-handoff{transition:none}}
</style>
