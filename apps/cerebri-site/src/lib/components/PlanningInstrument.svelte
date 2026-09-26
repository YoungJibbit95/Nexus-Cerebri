<script lang="ts">
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
  const exhausted = Boolean(planner.search_space?.exhausted);
  const formatTime = (instant: number) => new Date(instant).toLocaleTimeString('en-GB', { hour: '2-digit', minute: '2-digit', timeZone: 'UTC' });
  const durationPct = (durationMs / scopeMs) * 100;

  const candidates = Array.from({ length: evaluated }, (_, index) => {
    const instant = scopeStart + index * granularityMs;
    const ratio = Math.max(0, Math.min(1, (instant - scopeStart) / scopeMs));
    const state = instant === selectedStart ? 'selected' : rejectedStarts.has(instant) ? 'rejected' : validStarts.has(instant) ? 'valid' : 'unseen';
    return {
      instant,
      label: formatTime(instant),
      state,
      startPct: ratio * 100,
      mobileTop: 8 + ratio * 84,
      mobileHeight: (durationMs / scopeMs) * 84
    };
  });

  const busy = busyStart !== null && busyEnd !== null
    ? {
        start: ((busyStart - scopeStart) / scopeMs) * 100,
        width: ((busyEnd - busyStart) / scopeMs) * 100,
        mobileTop: 8 + ((busyStart - scopeStart) / scopeMs) * 84,
        mobileHeight: ((busyEnd - busyStart) / scopeMs) * 84
      }
    : null;

  const selectedLabel = selectedStart !== null ? formatTime(selectedStart) : '—';
</script>

<div class="planning-instrument" data-exhausted={exhausted}>
  <div class="instrument-meta" aria-hidden="true">
    <span class="candidate-summary">{validStarts.size} valid / {rejectedStarts.size} rejected</span>
    <span><i></i> HARD CONSTRAINT / KNOWN BUSY</span>
    <span><b></b> VALID CANDIDATE</span>
    <span><em></em> SELECTED PROPOSAL</span>
    <span class="preference-none">PREFERENCE FIELD / NONE IN THIS FIXTURE</span>
  </div>

  <div class="desktop-plane" aria-hidden="true">
    <div class="verification-contour"><span>{exhausted ? 'VERIFIED BOUNDED TRAVERSAL' : 'OPEN TRAVERSAL'}</span></div>
    <div class="desktop-coordinate">
      {#if busy}
        <div class="busy-column" style={"--busy-left:" + busy.start + "%;--busy-width:" + busy.width + "%"}><span>KNOWN BUSY</span></div>
      {/if}
      <div class="axis">
        <span class="axis-line"></span>
        <i style="left:0%"></i><i style="left:33.333%"></i><i style="left:66.666%"></i><i style="left:100%"></i>
        <small style="left:0%">{formatTime(scopeStart)}</small>
        <small style="left:33.333%">{formatTime(scopeStart + scopeMs / 3)}</small>
        <small style="left:66.666%">{formatTime(scopeStart + scopeMs * 2 / 3)}</small>
        <small style="left:100%">{formatTime(scopeEnd)}</small>
      </div>
      <ol class="candidate-lanes">
        {#each candidates as candidate, index}
          <li data-state={candidate.state}>
            <span class="candidate-index">C{String(index + 1).padStart(2, '0')}</span>
            <span
              class="candidate-capsule"
              style={"--candidate-left:" + candidate.startPct + "%;--candidate-width:" + durationPct + "%"}
            >
              <i></i><b></b><em></em>
            </span>
            <span class="candidate-time">{candidate.label}</span>
            <small>{candidate.state === 'selected' ? 'proposal' : candidate.state}</small>
          </li>
        {/each}
      </ol>
    </div>
  </div>

  <div class="mobile-plane" aria-hidden="true">
    <div class="mobile-scope"><span>09:00</span><span>12:00</span></div>
    <div class="mobile-rail"></div>
    {#if busy}
      <div class="mobile-busy" style={"--busy-top:" + busy.mobileTop + "%;--busy-height:" + busy.mobileHeight + "%"}><span>KNOWN BUSY</span></div>
    {/if}
    {#each candidates as candidate, index}
      <div
        class="mobile-candidate"
        class:rejected={candidate.state === 'rejected'}
        class:valid={candidate.state === 'valid'}
        class:selected={candidate.state === 'selected'}
        class:alternate={index % 2 === 1}
        style={"--candidate-top:" + candidate.mobileTop + "%;--candidate-height:" + candidate.mobileHeight + "%"}
      >
        <i></i><b></b><span>{candidate.label}</span><small>{candidate.state === 'selected' ? 'proposal' : candidate.state}</small>
      </div>
    {/each}
    <div class="mobile-verification"><span>{exhausted ? 'BOUNDED GRID EXHAUSTED' : 'TRAVERSAL OPEN'}</span></div>
  </div>

  <ol class="sr-candidates" aria-label="Evaluated candidate intervals">
    {#each candidates as candidate}
      <li>
        Candidate starting {candidate.label} UTC for {request.duration.value.knowledge.data / 60} minutes.
        {candidate.state === 'rejected'
          ? 'Rejected because it overlaps known busy time.'
          : candidate.state === 'selected'
            ? 'Valid and selected as the first proposal after deterministic ordering.'
            : candidate.state === 'valid'
              ? 'Valid candidate.'
              : 'Not classified in the generated result.'}
      </li>
    {/each}
  </ol>

  <div class="proposal-boundary" aria-label={"Selected proposal at " + selectedLabel + " UTC stops before a separate authority gate."}>
    <div class="proposal-object"><span></span><strong>{selectedLabel} UTC</strong><small>SELECTED PROPOSAL</small></div>
    <div class="proposal-trace" aria-hidden="true"><i></i></div>
    <div class="authority-gate">
      <span>AUTHORITY GATE</span><i aria-hidden="true"></i><small>SEPARATE STATE</small>
    </div>
    <div class="execution-side"><strong>EXECUTION</strong><small>not implied by planning</small></div>
  </div>
</div>

<style>
  .planning-instrument{position:relative;display:grid;gap:24px;padding-top:6px;--constraint:#ff8795;--valid:#60e6b8;--selected:#20d8ff;--line:rgba(91,151,216,.2)}
  .instrument-meta{display:flex;align-items:center;flex-wrap:wrap;gap:10px 20px;color:#758aa7;font:700 9px var(--mono);letter-spacing:.06em}.candidate-summary{color:#b6c9de;font-size:10px}.instrument-meta span{display:inline-flex;align-items:center;gap:7px}.instrument-meta i,.instrument-meta b,.instrument-meta em{width:10px;height:10px;display:inline-block}.instrument-meta i{border-left:2px solid var(--constraint);transform:skewX(-18deg)}.instrument-meta b{border-radius:50%;background:var(--valid);box-shadow:0 0 8px rgba(96,230,184,.18)}.instrument-meta em{border:1px solid var(--selected);border-radius:50%;box-shadow:inset 0 0 0 2px rgba(32,216,255,.08)}.preference-none{margin-left:auto;color:#607895}
  .desktop-plane{position:relative;min-height:500px;padding:70px 78px 30px 70px;border:1px solid rgba(70,136,204,.16);border-radius:20px;background:linear-gradient(180deg,rgba(4,15,39,.76),rgba(3,10,29,.62));overflow:hidden}
  .desktop-plane::before{content:"";position:absolute;inset:0;background:linear-gradient(90deg,transparent 0 32.9%,rgba(255,135,149,.025) 33% 33.5%,transparent 33.6% 100%);pointer-events:none}
  .verification-contour{position:absolute;inset:18px;border:1px solid rgba(66,232,224,.16);border-radius:16px;pointer-events:none}.verification-contour::before,.verification-contour::after{content:"";position:absolute;width:18px;height:18px}.verification-contour::before{left:-1px;top:-1px;border-left:2px solid rgba(66,232,224,.45);border-top:2px solid rgba(66,232,224,.45);border-radius:4px 0 0}.verification-contour::after{right:-1px;bottom:-1px;border-right:2px solid rgba(66,232,224,.45);border-bottom:2px solid rgba(66,232,224,.45);border-radius:0 0 4px}.verification-contour span{position:absolute;right:12px;top:8px;color:#6d9d98;font:700 9px var(--mono);letter-spacing:.08em}
  .desktop-coordinate{position:relative;margin-top:30px}.busy-column{position:absolute;left:var(--busy-left);top:-6px;width:var(--busy-width);bottom:0;border-left:1px solid rgba(255,135,149,.5);border-right:1px solid rgba(255,135,149,.28);background:linear-gradient(90deg,rgba(255,135,149,.11),rgba(255,135,149,.035));pointer-events:none}.busy-column span{position:absolute;left:8px;top:24px;color:#c47e88;font:700 9px var(--mono);letter-spacing:.08em;writing-mode:vertical-rl}
  .axis{position:absolute;left:0;right:0;top:-50px;height:34px}.axis-line{position:absolute;left:0;right:0;top:13px;height:1px;background:linear-gradient(90deg,rgba(32,216,255,.36),rgba(93,135,202,.3),rgba(117,75,255,.24))}.axis i{position:absolute;top:8px;width:1px;height:11px;background:#7890ad}.axis small{position:absolute;top:22px;transform:translateX(-50%);color:#657c9b;font:700 9px var(--mono)}.axis small:first-of-type{transform:none}.axis small:last-of-type{transform:translateX(-100%)}
  .candidate-lanes{position:relative;z-index:2;list-style:none;margin:0;padding:0}.candidate-lanes li{position:relative;height:34px;border-top:1px solid rgba(68,118,177,.075)}.candidate-lanes li:last-child{border-bottom:1px solid rgba(68,118,177,.075)}.candidate-index{position:absolute;right:calc(100% + 12px);top:9px;color:#536d8d;font:700 9px var(--mono)}.candidate-capsule{position:absolute;left:var(--candidate-left);width:var(--candidate-width);top:11px;height:12px;border:1px solid rgba(97,181,218,.44);border-radius:999px;background:linear-gradient(90deg,rgba(94,190,222,.58),rgba(52,103,164,.22))}.candidate-capsule::before,.candidate-capsule::after{content:"";position:absolute;top:50%;width:7px;height:7px;border-radius:50%;transform:translateY(-50%)}.candidate-capsule::before{left:-4px;background:#b9f4ff;box-shadow:0 0 7px rgba(71,207,255,.48)}.candidate-capsule::after{right:-4px;border:1px solid #91c9df;background:#06142d}.candidate-capsule i{position:absolute;right:20%;top:-5px;width:1px;height:20px;transform:rotate(28deg);background:transparent}.candidate-capsule b{position:absolute;right:20%;top:-10px;width:1px;height:9px;background:transparent}.candidate-capsule em{position:absolute;inset:-4px;border:1px solid transparent;border-radius:999px}.candidate-time{position:absolute;left:calc(var(--candidate-left) + 4px);top:8px;color:#93a8c3;font:700 9px var(--mono);transform:translateY(-120%)}.candidate-lanes li>small{position:absolute;right:-58px;top:9px;color:#6b819d;font:700 9px var(--mono);text-transform:uppercase}
  .candidate-lanes li[data-state="rejected"] .candidate-capsule{border-color:rgba(255,135,149,.42);background:linear-gradient(90deg,rgba(255,135,149,.46),rgba(102,43,67,.2))}.candidate-lanes li[data-state="rejected"] .candidate-capsule i{background:var(--constraint);box-shadow:0 0 7px rgba(255,135,149,.18)}.candidate-lanes li[data-state="rejected"] .candidate-capsule b{background:linear-gradient(var(--constraint),transparent)}.candidate-lanes li[data-state="rejected"]>small{color:#b97783}.candidate-lanes li[data-state="valid"] .candidate-capsule{border-color:rgba(96,230,184,.38);background:linear-gradient(90deg,rgba(96,230,184,.48),rgba(39,104,88,.16))}.candidate-lanes li[data-state="valid"]>small{color:#6fae99}.candidate-lanes li[data-state="selected"] .candidate-capsule{border-color:rgba(32,216,255,.72);background:linear-gradient(90deg,rgba(126,239,255,.82),rgba(32,216,255,.28));box-shadow:0 0 18px rgba(32,216,255,.08)}.candidate-lanes li[data-state="selected"] .candidate-capsule em{border-color:rgba(32,216,255,.2)}.candidate-lanes li[data-state="selected"]>small{color:#6fd9ed}
  .mobile-plane{display:none}
  .sr-candidates{position:absolute!important;width:1px!important;height:1px!important;padding:0!important;margin:-1px!important;overflow:hidden!important;clip:rect(0,0,0,0)!important;white-space:nowrap!important;border:0!important}
  .proposal-boundary{display:grid;grid-template-columns:minmax(160px,1fr) minmax(80px,.7fr) 86px minmax(120px,.8fr);align-items:center;gap:16px;min-height:112px;padding:18px 20px;border:1px solid rgba(82,140,207,.16);border-radius:18px;background:rgba(4,14,36,.56)}.proposal-object{position:relative;min-height:58px;padding:11px 14px 10px 52px;border:1px solid rgba(32,216,255,.32);border-radius:14px;background:linear-gradient(90deg,rgba(32,216,255,.08),rgba(15,37,71,.2))}.proposal-object>span{position:absolute;left:15px;top:25px;width:25px;height:9px;border:1px solid rgba(32,216,255,.65);border-radius:999px}.proposal-object>span::before{content:"";position:absolute;left:-3px;top:2px;width:5px;height:5px;border-radius:50%;background:#8ff2ff}.proposal-object strong{display:block;color:#dffaff;font:750 13px var(--mono)}.proposal-object small{display:block;margin-top:5px;color:#6aa9bb;font:700 9px var(--mono);letter-spacing:.08em}.proposal-trace{height:1px;background:linear-gradient(90deg,rgba(32,216,255,.3),rgba(32,216,255,.04));position:relative}.proposal-trace i{position:absolute;right:0;top:-3px;width:7px;height:7px;border-radius:50%;border:1px solid rgba(32,216,255,.55);background:#08172f}.authority-gate{position:relative;height:76px;text-align:center}.authority-gate>i{position:absolute;left:50%;top:18px;bottom:18px;width:2px;background:linear-gradient(rgba(255,199,102,.12),rgba(255,199,102,.72),rgba(255,199,102,.12));box-shadow:0 0 12px rgba(255,199,102,.12)}.authority-gate>span,.authority-gate>small{position:absolute;left:50%;transform:translateX(-50%);white-space:nowrap;font:700 8px var(--mono);letter-spacing:.07em}.authority-gate>span{top:0;color:#b99b67}.authority-gate>small{bottom:0;color:#796a50}.execution-side{opacity:.52;padding-left:4px}.execution-side strong{display:block;color:#a9b5c5;font:700 11px var(--mono);letter-spacing:.08em}.execution-side small{display:block;margin-top:5px;color:#697b94;font-size:11px}
  @media(max-width:760px){.instrument-meta{gap:8px 14px}.preference-none{margin-left:0}.desktop-plane{display:none}.mobile-plane{display:block;position:relative;height:690px;border:1px solid rgba(70,136,204,.16);border-radius:20px;background:linear-gradient(180deg,rgba(4,15,39,.76),rgba(3,10,29,.62));overflow:hidden}.mobile-scope{position:absolute;left:50%;top:4%;bottom:4%;width:68px;transform:translateX(-50%);border:1px solid rgba(66,232,224,.16);border-radius:16px}.mobile-scope span{position:absolute;left:50%;transform:translateX(-50%);color:#647d9d;font:700 9px var(--mono)}.mobile-scope span:first-child{top:8px}.mobile-scope span:last-child{bottom:8px}.mobile-rail{position:absolute;left:50%;top:8%;bottom:8%;width:1px;background:linear-gradient(rgba(32,216,255,.34),rgba(113,106,222,.28));transform:translateX(-50%)}.mobile-busy{position:absolute;left:calc(50% - 28px);top:var(--busy-top);height:var(--busy-height);width:56px;background:rgba(255,135,149,.1);border-top:1px solid rgba(255,135,149,.42);border-bottom:1px solid rgba(255,135,149,.3)}.mobile-busy span{position:absolute;right:5px;top:6px;writing-mode:vertical-rl;color:#ba7782;font:700 8px var(--mono)}.mobile-candidate{position:absolute;top:var(--candidate-top);height:var(--candidate-height);width:72px;left:calc(50% - 92px);border:1px solid rgba(94,180,217,.36);border-radius:999px;background:linear-gradient(180deg,rgba(86,185,220,.55),rgba(49,98,157,.18))}.mobile-candidate.alternate{left:calc(50% + 20px)}.mobile-candidate::before,.mobile-candidate::after{content:"";position:absolute;left:50%;width:7px;height:7px;border-radius:50%;transform:translateX(-50%)}.mobile-candidate::before{top:-4px;background:#b8f4ff}.mobile-candidate::after{bottom:-4px;border:1px solid #93ccdf;background:#07142d}.mobile-candidate span{position:absolute;left:50%;top:50%;transform:translate(-50%,-50%) rotate(-90deg);color:#b0c2d6;font:700 8px var(--mono);white-space:nowrap}.mobile-candidate small{position:absolute;left:50%;bottom:5px;transform:translateX(-50%);color:#6d819b;font:700 7px var(--mono);text-transform:uppercase}.mobile-candidate.rejected{border-color:rgba(255,135,149,.42);background:linear-gradient(180deg,rgba(255,135,149,.45),rgba(99,43,66,.18))}.mobile-candidate.rejected i{position:absolute;left:-5px;top:24%;width:82px;height:1px;background:var(--constraint);transform:rotate(-24deg);box-shadow:0 0 7px rgba(255,135,149,.16)}.mobile-candidate.valid{border-color:rgba(96,230,184,.4);background:linear-gradient(180deg,rgba(96,230,184,.48),rgba(37,99,84,.17))}.mobile-candidate.selected{left:calc(50% + 42px);border-color:rgba(32,216,255,.76);background:linear-gradient(180deg,rgba(126,239,255,.8),rgba(32,216,255,.24));box-shadow:0 0 17px rgba(32,216,255,.08)}.mobile-verification{position:absolute;inset:14px;border:1px solid rgba(66,232,224,.11);border-radius:14px;pointer-events:none}.mobile-verification span{position:absolute;right:8px;bottom:7px;color:#65948f;font:700 8px var(--mono);letter-spacing:.07em}.proposal-boundary{grid-template-columns:1fr 52px 72px;gap:10px}.execution-side{display:none}.proposal-object{padding-left:46px}.proposal-trace{min-width:32px}}
  @media(max-width:380px){.mobile-candidate{width:62px;left:calc(50% - 82px)}.mobile-candidate.alternate{left:calc(50% + 20px)}.mobile-candidate.selected{left:calc(50% + 34px)}.proposal-boundary{padding:14px}.proposal-object strong{font-size:11px}}
  @media(prefers-reduced-motion:reduce){.planning-instrument *{scroll-behavior:auto!important}}
</style>