<script lang="ts">
  import { onMount } from 'svelte';
  import { runtimeData } from '$lib/generated/runtime-data';

  const planner = runtimeData.plannerResult as any;
  const request = runtimeData.request as any;
  const scopeStart = new Date(request.scope.time_range.start).getTime();
  const scopeEnd = new Date(request.scope.time_range.end).getTime();
  const scopeMs = Math.max(scopeEnd - scopeStart, 1);
  const durationMs = Number(request.duration.value.knowledge.data ?? 0) * 1000;
  const granularityMs = Number(request.granularity ?? 0) * 1000;
  const evaluated = Number(planner.search_space?.evaluated ?? 0);
  const rejectedStarts = new Set((planner.conflicts?.rejections ?? []).map((entry: any) => new Date(entry.start).getTime()));
  const validStarts = new Set((planner.candidates ?? []).map((candidate: any) => new Date(candidate.start).getTime()));
  const winnerRange = planner.candidates?.[0]?.proposed?.placements?.[0]?.range;
  const selectedStart = winnerRange ? new Date(winnerRange.start).getTime() : null;
  const busyObject = (request.context?.objects ?? []).find((object: any) => object.time?.value?.knowledge?.state === 'KNOWN');
  const busyRange = busyObject?.time?.value?.knowledge?.data;
  const busyStart = busyRange ? new Date(busyRange.start).getTime() : null;
  const busyEnd = busyRange ? new Date(busyRange.end).getTime() : null;
  const spanDesktop = (durationMs / scopeMs) * 84;
  const spanMobile = (durationMs / scopeMs) * 78;
  const scatterY = [30, 65, 38, 74, 26, 58, 42, 72, 34, 62, 46];
  const scatterOffset = [-2.8, 2.2, -1.6, 3.1, -2.1, 1.4, 2.4, -1.1, 1.8, -2.5, .8];
  const formatTime = (instant: number) => new Date(instant).toLocaleTimeString('en-GB', { hour: '2-digit', minute: '2-digit', timeZone: 'UTC' });

  const points = Array.from({ length: evaluated }, (_, index) => {
    const instant = scopeStart + index * granularityMs;
    const ratio = Math.max(0, Math.min(1, (instant - scopeStart) / scopeMs));
    const state = instant === selectedStart ? 'selected' : rejectedStarts.has(instant) ? 'rejected' : validStarts.has(instant) ? 'valid' : 'unseen';
    return {
      instant,
      label: formatTime(instant),
      state,
      spaceX: 8 + ratio * 84 + (scatterOffset[index] ?? 0),
      spaceY: scatterY[index] ?? 50,
      railX: 8 + ratio * 84,
      railY: 10 + ratio * 78
    };
  });

  const busyGeometry = busyStart !== null && busyEnd !== null
    ? {
        left: 8 + ((busyStart - scopeStart) / scopeMs) * 84,
        width: ((busyEnd - busyStart) / scopeMs) * 84,
        top: 10 + ((busyStart - scopeStart) / scopeMs) * 78,
        height: ((busyEnd - busyStart) / scopeMs) * 78
      }
    : null;

  let host: HTMLElement;
  let scene = $state(0);
  let reducedMotion = $state(false);

  const sceneName = $derived([
    'atmospheric space',
    'observatory aperture',
    'temporal coordinate structure',
    'bounded search space',
    'candidate intervals',
    'validated candidate field',
    'selected proposal'
  ][scene] ?? 'selected proposal');

  onMount(() => {
    const media = window.matchMedia('(prefers-reduced-motion: reduce)');
    let active = true;
    let frame = 0;
    const section = host.closest('.hero') as HTMLElement | null;

    const updatePreference = () => {
      reducedMotion = media.matches;
      if (reducedMotion) scene = 6;
    };

    const updateScene = () => {
      if (!active || reducedMotion || !section) return;
      cancelAnimationFrame(frame);
      frame = requestAnimationFrame(() => {
        const rect = section.getBoundingClientRect();
        const travel = Math.max(rect.height - window.innerHeight * .3, 1);
        const progress = Math.max(0, Math.min(1, -rect.top / travel));
        scene = Math.min(6, Math.floor(progress * 7));
      });
    };

    const observer = new IntersectionObserver(([entry]) => {
      active = entry.isIntersecting;
      if (active) updateScene();
    }, { rootMargin: '25% 0px 25% 0px' });

    updatePreference();
    observer.observe(section ?? host);
    window.addEventListener('scroll', updateScene, { passive: true });
    window.addEventListener('resize', updateScene, { passive: true });
    media.addEventListener('change', updatePreference);
    updateScene();

    return () => {
      cancelAnimationFrame(frame);
      observer.disconnect();
      window.removeEventListener('scroll', updateScene);
      window.removeEventListener('resize', updateScene);
      media.removeEventListener('change', updatePreference);
    };
  });
</script>

<div
  bind:this={host}
  class="observatory"
  class:calibrated={scene >= 2}
  class:bounded={scene >= 3}
  class:intervals={scene >= 4}
  class:validated={scene >= 5}
  class:proposed={scene >= 6}
  data-motion={reducedMotion ? 'reduced' : 'full'}
  role="img"
  aria-label={"Bounded search observatory. Current scene: " + sceneName + ". Eleven repository-derived start positions resolve into a bounded time grid; four overlap known busy time, seven remain valid, and the first ordered candidate becomes a proposal without crossing the authority boundary."}
>
  <div class="world" aria-hidden="true">
    <span class="world-plane"></span>
    <span class="world-arc arc-a"></span>
    <span class="world-arc arc-b"></span>
    <span class="world-light"></span>
  </div>

  <div class="aperture" aria-hidden="true">
    <span></span><i></i><b></b>
  </div>

  <div class="scope-frame" aria-hidden="true">
    <small>BOUNDED SCOPE</small>
    <span class="scope-start">{formatTime(scopeStart)}</span>
    <span class="scope-end">{formatTime(scopeEnd)}</span>
  </div>

  <div class="time-rail" aria-hidden="true">
    <i></i>
    <span class="tick t0"></span><span class="tick t1"></span><span class="tick t2"></span><span class="tick t3"></span>
    <small class="l0">{formatTime(scopeStart)}</small>
    <small class="l1">{formatTime(scopeStart + scopeMs / 3)}</small>
    <small class="l2">{formatTime(scopeStart + scopeMs * 2 / 3)}</small>
    <small class="l3">{formatTime(scopeEnd)}</small>
  </div>

  {#if busyGeometry}
    <div
      class="busy-interval"
      aria-hidden="true"
      style={"--busy-left:" + busyGeometry.left + "%;--busy-width:" + busyGeometry.width + "%;--busy-top:" + busyGeometry.top + "%;--busy-height:" + busyGeometry.height + "%"}
    >
      <span>KNOWN BUSY</span>
    </div>
  {/if}

  <div class="candidate-field" aria-hidden="true">
    {#each points as point, index}
      <div
        class="candidate-object"
        class:rejected={point.state === 'rejected'}
        class:valid={point.state === 'valid'}
        class:selected={point.state === 'selected'}
        style={"--space-x:" + point.spaceX + "%;--space-y:" + point.spaceY + "%;--rail-x:" + point.railX + "%;--rail-y:" + point.railY + "%;--span-x:" + spanDesktop + "%;--span-y:" + spanMobile + "%;--delay:" + index * 24 + "ms"}
      >
        <i class="constraint-cut"></i>
        <b class="evidence-thread"></b>
        <small>{point.state === 'selected' ? 'PROPOSAL' : point.state.toUpperCase()}</small>
      </div>
    {/each}
  </div>

  <div class="proposal-rail" aria-hidden="true"><span>PROPOSAL RAIL</span><i></i></div>
  <div class="authority-gate" aria-hidden="true"><span>AUTHORITY</span><i></i><b>NOT CROSSED</b></div>
  <div class="observatory-readout" aria-hidden="true">
    <span>SPACE</span><i>→</i><span>TIME</span><i>→</i><span>VALIDITY</span><i>→</i><strong>PROPOSAL</strong>
  </div>
</div>

<style>
  .observatory{position:absolute;z-index:2;left:50%;bottom:18px;width:min(1120px,92vw);height:430px;transform:translateX(-50%);pointer-events:none;isolation:isolate;--instrument:#79eaff;--muted:#6e86a7;--line:rgba(89,164,226,.28);--valid:#60e6b8;--reject:#ff8795}
  .world{position:absolute;inset:0;overflow:hidden;opacity:.92;transition:opacity 700ms ease}.world-plane{position:absolute;left:8%;right:8%;top:54%;height:1px;background:linear-gradient(90deg,transparent,rgba(76,136,217,.24),transparent);transform:perspective(700px) rotateX(62deg) scaleX(.86)}.world-arc{position:absolute;border:1px solid rgba(75,137,207,.12);border-radius:50%}.arc-a{width:76%;height:54%;left:12%;top:18%;transform:rotate(-8deg)}.arc-b{width:58%;height:82%;left:21%;top:2%;border-style:dashed;border-color:rgba(122,92,214,.13);transform:rotate(21deg)}.world-light{position:absolute;width:48%;height:56%;left:26%;top:20%;border-radius:50%;background:radial-gradient(circle,rgba(32,216,255,.09),rgba(63,72,211,.04) 40%,transparent 72%);filter:blur(24px)}
  .aperture{position:absolute;inset:7% 5%;border:1px solid rgba(91,169,226,.08);border-radius:46% 54% 50% 50% / 54% 43% 57% 46%;opacity:.18;transition:opacity 650ms ease,border-color 650ms ease,transform 800ms cubic-bezier(.16,1,.3,1);transform:scale(.94)}.aperture span,.aperture i,.aperture b{position:absolute;display:block}.aperture span{inset:7%;border:1px solid rgba(69,183,230,.08);border-radius:inherit}.aperture i{left:50%;top:-6%;bottom:-6%;width:1px;background:linear-gradient(transparent,rgba(61,178,225,.12),transparent)}.aperture b{top:50%;left:-4%;right:-4%;height:1px;background:linear-gradient(90deg,transparent,rgba(91,151,226,.1),transparent)}
  .calibrated .aperture{opacity:.8;border-color:rgba(79,193,235,.2);transform:scale(1)}.calibrated .world{opacity:.45}
  .scope-frame{position:absolute;left:6.5%;right:6.5%;top:43%;height:112px;border:1px solid transparent;border-radius:18px;opacity:0;transition:opacity 380ms ease,border-color 380ms ease;background:linear-gradient(90deg,rgba(32,216,255,.018),transparent 30%,transparent 70%,rgba(117,75,255,.018))}.scope-frame small{position:absolute;left:12px;top:9px;color:#7188a6;font:700 10px var(--mono);letter-spacing:.1em}.scope-start,.scope-end{position:absolute;bottom:8px;color:#6a82a3;font:700 10px var(--mono)}.scope-start{left:10px}.scope-end{right:10px}.bounded .scope-frame{opacity:1;border-color:rgba(73,165,216,.22)}
  .time-rail{position:absolute;left:8%;right:8%;top:58%;height:54px;opacity:0;transition:opacity 350ms ease,transform 650ms cubic-bezier(.16,1,.3,1);transform:translateY(12px)}.time-rail>i{position:absolute;left:0;right:0;top:0;height:1px;background:linear-gradient(90deg,rgba(32,216,255,.3),rgba(126,149,231,.44),rgba(117,75,255,.24))}.tick{position:absolute;top:-5px;width:1px;height:11px;background:#7695b9}.t0{left:0}.t1{left:33.333%}.t2{left:66.666%}.t3{right:0}.time-rail small{position:absolute;top:11px;color:#6e85a4;font:700 10px var(--mono)}.l0{left:0}.l1{left:33.333%;transform:translateX(-50%)}.l2{left:66.666%;transform:translateX(-50%)}.l3{right:0}.calibrated .time-rail{opacity:1;transform:none}
  .busy-interval{position:absolute;left:var(--busy-left);width:var(--busy-width);top:48%;height:48px;border-left:1px solid rgba(255,135,149,.66);border-right:1px solid rgba(255,135,149,.45);background:linear-gradient(90deg,rgba(255,135,149,.14),rgba(255,135,149,.06));opacity:0;transition:opacity 380ms ease}.busy-interval span{position:absolute;left:8px;top:7px;color:#d58b96;font:700 9px var(--mono);letter-spacing:.08em}.bounded .busy-interval{opacity:1}
  .candidate-field{position:absolute;inset:0}.candidate-object{position:absolute;left:var(--space-x);top:var(--space-y);width:7px;height:7px;border-radius:50%;background:#a7dfff;box-shadow:0 0 14px rgba(98,205,255,.55);transition:left 720ms cubic-bezier(.16,1,.3,1) var(--delay),top 720ms cubic-bezier(.16,1,.3,1) var(--delay),width 420ms cubic-bezier(.16,1,.3,1),height 420ms cubic-bezier(.16,1,.3,1),transform 520ms cubic-bezier(.16,1,.3,1),background 260ms ease,border-color 260ms ease,opacity 260ms ease}.calibrated .candidate-object{left:var(--rail-x);top:58%}.intervals .candidate-object{width:var(--span-x);height:14px;margin-top:-7px;border:1px solid rgba(104,187,226,.58);border-radius:999px;background:linear-gradient(90deg,rgba(110,211,239,.8),rgba(67,122,195,.3));box-shadow:0 0 18px rgba(32,216,255,.08)}.intervals .candidate-object::before,.intervals .candidate-object::after{content:"";position:absolute;top:50%;width:8px;height:8px;border-radius:50%;transform:translateY(-50%)}.intervals .candidate-object::before{left:-4px;background:#baf6ff;box-shadow:0 0 8px rgba(93,222,255,.58)}.intervals .candidate-object::after{right:-4px;border:1px solid #9fdcf2;background:#07152f}.candidate-object small{position:absolute;left:50%;top:21px;transform:translateX(-50%);color:#7e95b1;font:700 8px var(--mono);letter-spacing:.05em;opacity:0;white-space:nowrap}.validated .candidate-object small{opacity:1}.validated .candidate-object.rejected{border-color:rgba(255,135,149,.52);background:linear-gradient(90deg,rgba(255,135,149,.52),rgba(117,46,72,.22));opacity:.62}.validated .candidate-object.valid{border-color:rgba(96,230,184,.46);background:linear-gradient(90deg,rgba(96,230,184,.58),rgba(44,111,95,.2))}.validated .candidate-object.selected{border-color:rgba(32,216,255,.82);background:linear-gradient(90deg,#8af2ff,rgba(32,216,255,.42));box-shadow:0 0 0 3px rgba(32,216,255,.06),0 0 22px rgba(32,216,255,.14)}.validated .candidate-object.rejected small{color:#d68d98}.validated .candidate-object.valid small{color:#76bba3}.validated .candidate-object.selected small{color:#78e9ff}
  .constraint-cut{position:absolute;right:20%;top:-5px;width:1px;height:23px;background:var(--reject);transform:rotate(28deg);opacity:0;box-shadow:0 0 8px rgba(255,135,149,.28)}.evidence-thread{position:absolute;right:20%;top:-34px;width:1px;height:28px;background:linear-gradient(var(--reject),transparent);opacity:0}.validated .rejected .constraint-cut,.validated .rejected .evidence-thread{opacity:1}
  .proposal-rail{position:absolute;left:30%;right:19%;top:35%;height:32px;opacity:0;transition:opacity 320ms ease}.proposal-rail i{position:absolute;left:0;right:0;top:17px;height:1px;background:linear-gradient(90deg,rgba(32,216,255,.36),rgba(32,216,255,.08))}.proposal-rail span{position:absolute;left:0;top:0;color:#6e8eaa;font:700 9px var(--mono);letter-spacing:.08em}.proposed .proposal-rail{opacity:1}.proposed .candidate-object.selected{transform:translateY(-98px)}
  .authority-gate{position:absolute;right:12%;top:28%;width:58px;height:116px;opacity:0;transition:opacity 320ms ease}.authority-gate i{position:absolute;left:50%;top:18px;bottom:20px;width:2px;background:linear-gradient(rgba(255,199,102,.16),rgba(255,199,102,.78),rgba(255,199,102,.16));box-shadow:0 0 12px rgba(255,199,102,.16)}.authority-gate span{position:absolute;left:50%;top:0;transform:translateX(-50%);color:#c8aa72;font:700 9px var(--mono);letter-spacing:.08em}.authority-gate b{position:absolute;left:50%;bottom:0;transform:translateX(-50%);color:#927e5b;font:700 8px var(--mono);white-space:nowrap}.proposed .authority-gate{opacity:1}
  .observatory-readout{position:absolute;left:50%;bottom:2%;transform:translateX(-50%);display:flex;align-items:center;gap:10px;color:#607895;font:700 9px var(--mono);letter-spacing:.08em;opacity:.72}.observatory-readout i{font-style:normal;color:#365c80}.observatory-readout strong{color:#85ddeb;font-weight:750}
  @media (max-width:900px){.observatory{width:96vw;height:390px;bottom:5px}.authority-gate{right:7%}.scope-frame{left:4%;right:4%}.time-rail{left:7%;right:7%}.observatory-readout{display:none}}
  @media (max-width:620px){:global(.hero){min-height:1160px}.observatory{height:520px;width:100%;bottom:-10px}.world-arc{opacity:.5}.aperture{inset:5% 9%}.scope-frame{left:44%;right:auto;top:8%;width:86px;height:79%;border-radius:16px}.scope-frame small{left:50%;top:8px;transform:translateX(-50%);white-space:nowrap}.scope-start,.scope-end{display:none}.time-rail{left:50%;right:auto;top:10%;width:52px;height:78%;transform:translateX(-50%)}.time-rail>i{left:0;right:auto;top:0;bottom:0;width:1px;height:auto}.tick{left:-5px!important;right:auto!important;top:auto;width:11px;height:1px}.t0{top:0}.t1{top:33.333%}.t2{top:66.666%}.t3{bottom:0}.time-rail small{left:10px!important;right:auto!important;transform:none!important;top:auto}.l0{top:-5px}.l1{top:calc(33.333% - 5px)}.l2{top:calc(66.666% - 5px)}.l3{bottom:-5px}.busy-interval{left:calc(50% - 26px);width:52px;top:var(--busy-top);height:var(--busy-height);border-left:1px solid rgba(255,135,149,.55);border-right:1px solid rgba(255,135,149,.55);border-top:0}.busy-interval span{writing-mode:vertical-rl;left:auto;right:5px;top:8px}.calibrated .candidate-object{left:50%;top:var(--rail-y);transform:translateX(-50%)}.intervals .candidate-object{width:14px;height:var(--span-y);margin-top:0;margin-left:-7px;border-radius:999px;background:linear-gradient(180deg,rgba(110,211,239,.8),rgba(67,122,195,.3))}.intervals .candidate-object::before,.intervals .candidate-object::after{left:50%;right:auto;top:auto;transform:translateX(-50%)}.intervals .candidate-object::before{top:-4px}.intervals .candidate-object::after{bottom:-4px}.candidate-object small{display:none}.constraint-cut{right:auto;left:-5px;top:22%;width:23px;height:1px;transform:rotate(-28deg)}.evidence-thread{right:auto;left:-30px;top:22%;width:25px;height:1px;background:linear-gradient(90deg,transparent,var(--reject))}.proposal-rail{left:58%;right:8%;top:35%;height:120px}.proposal-rail i{left:0;right:auto;top:0;bottom:0;width:1px;height:auto}.proposal-rail span{left:8px;top:0;writing-mode:vertical-rl}.proposed .candidate-object.selected{transform:translate(88px,0)}.authority-gate{right:3%;top:29%;height:158px}.observatory-readout{display:none}}
  @media (prefers-reduced-motion:reduce){.world-arc,.candidate-object,.aperture,.scope-frame,.time-rail,.busy-interval,.proposal-rail,.authority-gate{transition:none!important;animation:none!important}}
</style>