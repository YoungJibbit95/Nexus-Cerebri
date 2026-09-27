<script lang="ts">
  import './core-assembly.css';
  import { candidates, busy, durationPosition, scopeStart, scopeEnd, timeLabel } from '$lib/planning-display';
  let { phase = 6, context = 'workbench', inspected }: { phase?: number; context?: 'hero' | 'workbench'; inspected?: string } = $props();
  const depths = [-70, 35, -35, 90, 15, -90, 55, -15, 80, -45, 20];
  const heights = [28, 67, 38, 77, 22, 58, 42, 74, 32, 63, 47];
</script>

<div class="candidate-coordinate" data-phase={phase} data-context={context} aria-hidden="true">
  <div class="coordinate-volume">
    {#if context === 'hero'}
      <div class="core-assembly">
        {#each ['TIME WINDOW', 'KNOWN STATE', 'POSSIBLE PLACEMENTS'] as layer, index}<div class="core-plate" style={`--plate:${index}`}><span>{layer}</span><i></i></div>{/each}
      </div>
    {/if}
    <div class="calibration-plane"><i></i><i></i><i></i></div>
    <div class="optical-aperture"><i></i><b></b></div>
    <div class="coordinate-scope"><span>BOUNDED SCOPE</span><b>{timeLabel(scopeStart)}–{timeLabel(scopeEnd)} UTC</b></div>
    <div class="coordinate-axis">
      {#each [0, 1, 2, 3] as index}
        <span style={`--tick:${index / 3 * 100}%`}><i></i><small>{timeLabel(scopeStart + (scopeEnd - scopeStart) * index / 3)}</small></span>
      {/each}
    </div>
    {#if busy}<div class="coordinate-busy" style={`--busy:${busy.position}%;--busy-span:${busy.width}%`}><span>KNOWN BUSY</span></div>{/if}
    <ol class="candidate-lanes">
      {#each candidates as candidate, index (candidate.id)}
        <li class:inspected={inspected === candidate.start} data-state={candidate.state} data-candidate-id={candidate.id} data-start={candidate.start}
          style={`--position:${candidate.position}%;--duration:${durationPosition}%;--lane:${index};--scatter:${heights[index] ?? 50}%;--depth:${depths[index] ?? 0}px;--delay:${index * 25}ms;--mobile-lane:${index % 3};--narrow-lane:${index % 2};--core-x:${27 + index % 4 * 13}%;--core-y:${34 + Math.floor(index / 4) * 14}%`}>
          <span class="candidate-identity">{candidate.id}</span>
          <span class="candidate-capsule"><i class="capsule-origin"></i><i class="capsule-end"></i><b class="incision"></b><em class="evidence-anchor"></em></span>
          <span class="candidate-coordinate-label">{candidate.label}</span>
          <small class="candidate-state">{candidate.state === 'selected' ? 'FIRST IN RUST ORDER' : candidate.state}</small>
        </li>
      {/each}
    </ol>
    <div class="traversal-contour"><i></i><i></i><span>BOUNDED GRID · {candidates.length} POSITIONS</span></div>
  </div>
  {#if busy}<span class="mobile-busy-key">KNOWN BUSY</span>{/if}
</div>

<style>
  .mobile-busy-key{display:none}
  .inspected .candidate-capsule{outline:1px solid var(--red);outline-offset:4px}.inspected .candidate-identity{color:var(--red);font-weight:700}
  .candidate-coordinate{container-type:inline-size;height:35rem;position:relative;isolation:isolate;perspective:1000px;--ink:var(--cyan);--settle:cubic-bezier(.18,.8,.2,1)}
  .coordinate-volume{position:absolute;inset:2rem 8% 3rem;transform-style:preserve-3d;transform:rotateX(48deg) rotateZ(-9deg);transition:transform 1100ms var(--settle)}
  .calibration-plane{position:absolute;inset:10% -3%;border:1px solid var(--line);transform:translateZ(-75px);transition:transform 1000ms var(--settle),opacity 800ms;opacity:.3}
  .calibration-plane i{position:absolute;left:0;right:0;top:25%;border-top:1px solid var(--line);transform:skewY(-7deg);transition:transform 1000ms}
  .calibration-plane i:nth-child(2){top:50%}.calibration-plane i:nth-child(3){top:75%}
  .optical-aperture{position:absolute;inset:-1% -2%;border:1px solid var(--line-strong);border-radius:50%;transform:translateZ(35px) rotateZ(12deg);box-shadow:inset 0 0 0 14px rgba(32,216,255,.015);transition:transform 1100ms var(--settle),border-radius 1100ms var(--settle),opacity 600ms;opacity:.4}
  .optical-aperture i,.optical-aperture b{position:absolute;inset:12px;border:1px solid var(--line);border-radius:inherit}.optical-aperture b{inset:-12px;border-style:dashed;opacity:.4}
  .coordinate-scope{position:absolute;inset:0;border-inline:1px solid var(--line-strong);opacity:0;transition:opacity 500ms 200ms}
  .coordinate-scope span,.coordinate-scope b{position:absolute;top:-18px;font:.8125rem var(--mono);letter-spacing:.09em;color:var(--text-3);font-weight:400}.coordinate-scope b{right:0;letter-spacing:0}
  .coordinate-axis{position:absolute;left:0;right:0;top:50%;height:1px;background:var(--line-strong);transform:rotate(-13deg);transition:top 1000ms var(--settle),transform 1000ms var(--settle)}
  .coordinate-axis>span{position:absolute;left:var(--tick);top:0;transform:translateY(-30px) rotate(13deg);transition:transform 950ms var(--settle);opacity:.3}
  .coordinate-axis i{height:12px;border-left:1px solid var(--text-3);position:absolute;top:-5px}
  .coordinate-axis small{position:absolute;top:14px;font:.8125rem var(--mono);color:var(--text-3);white-space:nowrap;transform:translateX(-50%)}
  .coordinate-axis>span:first-child small{transform:none}.coordinate-axis>span:last-child small{transform:translateX(-100%)}
  .coordinate-busy{position:absolute;left:var(--busy);width:var(--busy-span);top:60px;bottom:20px;border-inline:1px solid rgba(255,135,149,.4);background:repeating-linear-gradient(125deg,transparent 0 12px,rgba(255,135,149,.035) 12px 13px);opacity:0;transition:opacity 450ms 250ms}
  .coordinate-busy span{position:absolute;top:auto;bottom:.5rem;left:.5rem;font:.8125rem var(--mono);color:var(--red);writing-mode:vertical-rl}
  .candidate-lanes{position:absolute;inset:0;list-style:none;padding:0;margin:0;transform-style:preserve-3d}
  .candidate-lanes li{position:absolute;left:var(--position);top:var(--scatter);width:var(--duration);height:12px;transform:translateZ(var(--depth));transition:top 1000ms var(--settle) var(--delay),left 1000ms var(--settle),transform 1000ms var(--settle) var(--delay),color 450ms 800ms;color:var(--cyan)}
  .candidate-capsule{display:block;position:absolute;inset:0;width:0;height:10px;border-radius:0 8px 8px 0;background:linear-gradient(90deg,currentColor,transparent);transition:width 750ms var(--settle) var(--delay),background 450ms,opacity 450ms;opacity:.85}
  .capsule-origin,.capsule-end{position:absolute;top:50%;width:8px;height:8px;border-radius:50%;transform:translate(-50%,-50%);background:currentColor;box-shadow:0 0 16px color-mix(in srgb,currentColor 22%,transparent)}
  .capsule-origin{left:0}.capsule-end{left:100%;background:var(--space-1);border:1px solid currentColor;opacity:0;transition:opacity 300ms 450ms}
  .candidate-identity,.candidate-coordinate-label,.candidate-state{position:absolute;font:.8125rem var(--mono);opacity:0;transition:opacity 500ms 650ms;white-space:nowrap}
  .candidate-identity{left:-3rem;top:-.15rem;color:var(--text-3)}.candidate-coordinate-label{top:-1.5rem;left:0;color:var(--text-3)}.candidate-state{top:17px;left:0;font-size:.8125rem;letter-spacing:.04em}
  .incision{position:absolute;left:8px;top:-9px;height:28px;border-left:2px solid var(--red);transform:rotate(25deg) scaleY(0);transition:transform 350ms 500ms}
  .evidence-anchor{position:absolute;left:13px;top:-22px;height:13px;border-left:1px solid var(--red);transform:scaleY(0);transform-origin:bottom;transition:transform 350ms 700ms}.evidence-anchor::before{content:'';position:absolute;top:0;left:-3px;width:5px;height:5px;border:1px solid var(--red)}
  .traversal-contour{position:absolute;inset:-30px -4%;pointer-events:none;opacity:0;transition:opacity 600ms 900ms}.traversal-contour i{position:absolute;width:24px;height:24px;border:1px solid var(--cerebri)}.traversal-contour i:first-child{top:0;left:0;border-right:0;border-bottom:0}.traversal-contour i:nth-child(2){right:0;bottom:0;border-left:0;border-top:0}.traversal-contour>span{position:absolute;bottom:0;left:4%;font:.8125rem var(--mono);letter-spacing:.06em;color:var(--text-3)}
  [data-phase='1'] .coordinate-volume{transform:rotateX(25deg) rotateZ(-4deg)}
  [data-phase='1'] .optical-aperture{opacity:.8;transform:translateZ(15px) rotateZ(0)}
  [data-phase]:where(:not([data-phase='0']):not([data-phase='1'])) .coordinate-volume{transform:none}
  [data-phase]:where(:not([data-phase='0']):not([data-phase='1'])) .candidate-lanes li{top:50%;transform:none}
  [data-phase]:where(:not([data-phase='0']):not([data-phase='1'])) .coordinate-axis{transform:none}
  [data-phase]:where(:not([data-phase='0']):not([data-phase='1'])) .coordinate-axis>span{transform:none;opacity:1}
  [data-phase]:where(:not([data-phase='0']):not([data-phase='1'])) .calibration-plane{transform:none;opacity:.12}
  [data-phase]:where(:not([data-phase='0']):not([data-phase='1'])) .calibration-plane i{transform:none}
  [data-phase]:where(:not([data-phase='0']):not([data-phase='1'])) .optical-aperture{transform:none;border-radius:2px;opacity:.15}
  [data-phase='3'] .coordinate-scope,[data-phase='4'] .coordinate-scope,[data-phase='5'] .coordinate-scope,[data-phase='6'] .coordinate-scope{opacity:1}
  [data-phase='4'] .candidate-lanes li,[data-phase='5'] .candidate-lanes li,[data-phase='6'] .candidate-lanes li{top:calc(5rem + var(--lane) * 2.35rem)}
  [data-phase='4'] .coordinate-axis,[data-phase='5'] .coordinate-axis,[data-phase='6'] .coordinate-axis{top:28px}
  [data-phase='4'] .candidate-capsule,[data-phase='5'] .candidate-capsule,[data-phase='6'] .candidate-capsule{width:100%;border-block:1px solid color-mix(in srgb,currentColor 55%,transparent)}
  [data-phase='4'] .capsule-end,[data-phase='5'] .capsule-end,[data-phase='6'] .capsule-end,[data-phase='4'] .candidate-identity,[data-phase='5'] .candidate-identity,[data-phase='6'] .candidate-identity{opacity:1}
  [data-phase='5'] .coordinate-busy,[data-phase='6'] .coordinate-busy{opacity:1}
  [data-phase='5'] li[data-state='rejected'],[data-phase='6'] li[data-state='rejected']{color:var(--red)}
  [data-phase='5'] li[data-state='valid'],[data-phase='6'] li[data-state='valid']{color:var(--green)}
  [data-phase='5'] li[data-state='rejected'] :is(.incision,.evidence-anchor),[data-phase='6'] li[data-state='rejected'] :is(.incision,.evidence-anchor){transform:rotate(25deg) scaleY(1)}
  [data-phase='6'] :is(.traversal-contour,.candidate-coordinate-label){opacity:1}
  [data-phase='6'] li[data-state='selected'] .candidate-capsule{box-shadow:0 0 0 4px rgba(32,216,255,.08);opacity:1}
  [data-phase='6']{perspective:none}[data-phase='6'] .coordinate-volume,[data-phase='6'] .candidate-lanes{transform-style:flat}
  li[data-state='rejected'] .candidate-coordinate-label{left:26px}
  [data-phase='6'] li[data-state='selected'] .candidate-state{opacity:1;left:calc(100% + 1rem);top:-.15rem}
  @media(max-width:760px){
    .candidate-coordinate{height:46rem}.coordinate-volume{inset:2.25rem 4% 4.5rem max(18%,3.5rem)}.mobile-busy-key{display:block;position:absolute;bottom:.5rem;left:max(18%,3.5rem);font:.8125rem var(--mono);color:var(--red)}.mobile-busy-key::before{content:"";display:inline-block;width:1rem;margin-right:.5rem;border-top:1px solid currentColor}
    .coordinate-axis{left:0;right:auto;width:1px;height:86%;top:6%!important;transform:rotate(-14deg);transform-origin:top;background:var(--line-strong)}
    .coordinate-axis>span{left:0;top:var(--tick)}.coordinate-axis i{width:10px;height:1px;border:0;border-top:1px solid var(--text-3);left:-5px;top:0}.coordinate-axis small{left:-3rem;top:-.4rem;transform:none!important;font-size:.8125rem}
    .coordinate-scope{inset:0 36%;border-inline:1px solid var(--line)}.coordinate-scope span{top:-22px;left:50%;transform:translateX(-50%);white-space:nowrap;font-size:.8125rem}.coordinate-scope b{display:none}
    .coordinate-busy{left:0;width:10px;top:calc(6% + var(--busy) * .86);height:calc(var(--busy-span) * .86);bottom:auto}.coordinate-busy span{display:none}
    [data-phase]:where(:not([data-phase='0']):not([data-phase='1'])) .candidate-lanes li{left:50%;top:calc(6% + var(--position) * .86);width:10px;height:calc(var(--duration) * .86)}
    [data-phase='4'] .candidate-lanes li,[data-phase='5'] .candidate-lanes li,[data-phase='6'] .candidate-lanes li{left:calc(13% + var(--mobile-lane) * 31%)}
    .candidate-capsule{height:0;width:8px!important;background:linear-gradient(currentColor,transparent);transition:height 750ms var(--settle) var(--delay)}
    [data-phase='4'] .candidate-capsule,[data-phase='5'] .candidate-capsule,[data-phase='6'] .candidate-capsule{height:100%;border-block:0;border-inline:1px solid currentColor}
    .capsule-origin{left:50%;top:0}.capsule-end{left:50%;top:100%}.candidate-identity{top:-1.3rem;left:-.4rem;font-size:.8125rem}.candidate-coordinate-label,li[data-state='rejected'] .candidate-coordinate-label{left:1rem;top:1.25rem;font-size:.8125rem;writing-mode:vertical-rl}.candidate-state{display:none}
    .incision{left:-7px;top:8px;height:1px;width:24px;border:0;border-top:2px solid var(--red)}.evidence-anchor{top:13px;left:-21px;height:1px;width:14px;border:0;border-top:1px solid var(--red)}
  }
  @container(max-width:14rem){
    [data-phase='4'] .candidate-lanes li,[data-phase='5'] .candidate-lanes li,[data-phase='6'] .candidate-lanes li{left:calc(18% + var(--narrow-lane) * 52%)}
  }
  @media(prefers-reduced-motion:reduce){.candidate-coordinate{perspective:none}.coordinate-volume,.candidate-lanes{transform-style:flat}.candidate-coordinate *,.candidate-coordinate *::before{transition:none!important;animation:none!important}}
</style>
