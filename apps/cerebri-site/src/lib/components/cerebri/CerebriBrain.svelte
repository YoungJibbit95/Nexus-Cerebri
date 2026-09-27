<script lang="ts">
  import { onMount } from 'svelte';
  import { brainModules, mix, ramp, focusStrength } from './brain-geometry';
  import { candidates, busy, durationPosition, scopeStart, scopeEnd, timeLabel } from '$lib/planning-display';

  let { progress = 0, activation = 1, mobile = false, overview = false, centered = false }: {
    progress?: number; activation?: number; mobile?: boolean; overview?: boolean; centered?: boolean;
  } = $props();
  const uid = $props.id();
  let svg: SVGSVGElement;
  let bounds = $state({ width: 1280, height: 800 });
  let outlines = $state<Record<string, { x: number; y: number }[]>>({});
  const explosion = $derived(ramp(progress, .12, .4));
  const handoff = $derived(ramp(progress, .83, 1));
  const width = $derived(mobile ? 500 : 1280);
  const height = $derived(mobile ? 780 : 800);
  const drawingScale = $derived(Math.min(bounds.width / width, bounds.height / height));
  const camera = $derived({
    x: mobile ? 250 : centered ? 640 : mix(895, 640, ramp(progress, .04, .25)),
    y: mobile ? 350 : 340,
    scale: mobile ? Math.min(mix(1.05, .75, explosion), (bounds.width - 36) / (mix(560, 820, explosion) * drawingScale)) : mix(1.12, .9, explosion)
  });
  const labelReveal = $derived(ramp(progress, .2, .38) * (1 - ramp(progress, .82, .9)));
  const labelPosition = (x: number, y: number) =>
    `--label-x:${bounds.width / 2 + (x - width / 2) * drawingScale}px;--label-y:${bounds.height / 2 + (y - height / 2) * drawingScale}px`;
  const focusActive = $derived(ramp(progress, .42, .44) * (1 - ramp(progress, .81, .84)));
  const geometry = (module: typeof brainModules[number], x: number, y: number) => {
    const angle = module.angle * explosion * Math.PI / 180;
    const dx = x - module.cx, dy = y - module.cy;
    return {
      x: camera.x + (module.cx + dx * Math.cos(angle) - dy * Math.sin(angle) + module.dx * explosion * (mobile ? .64 : 1)) * camera.scale,
      y: camera.y + (module.cy + dx * Math.sin(angle) + dy * Math.cos(angle) + module.dy * explosion * (mobile ? module.id === 'proposal' ? 1.4 : 1.7 : 1)) * camera.scale
    };
  };
  const segmentTransform = (module: typeof brainModules[number]) =>
    `translate(${camera.x} ${camera.y}) scale(${camera.scale}) translate(${module.dx * explosion * (mobile ? .64 : 1)} ${module.dy * explosion * (mobile ? module.id === 'proposal' ? 1.4 : 1.7 : 1)}) rotate(${module.angle * explosion} ${module.cx} ${module.cy})`;
  const plot = $derived(mobile
    ? { x: (width - bounds.width / drawingScale) / 2 + 50 / drawingScale, y: 125, w: (bounds.width - 65) / drawingScale, h: 500 }
    : { x: 130, y: 110, w: 1020, h: 495 });
  const projectedCandidate = (index: number) => {
    const c = candidates[index];
    return mobile
      ? { x: plot.x + (40 + index % 3 * (bounds.width - 150) / 2) / drawingScale, y: plot.y + c.position / 100 * plot.h, dx: 0, dy: durationPosition / 100 * plot.h }
      : { x: plot.x + c.position / 100 * plot.w, y: 166 + index * 42, dx: durationPosition / 100 * plot.w, dy: 0 };
  };
  const selectedIndex = candidates.findIndex(candidate => candidate.state === 'selected');
  const targetBox = (id: string) => {
    const proposal = projectedCandidate(selectedIndex);
    const targets: Record<string, number[]> = mobile ? {
      scope: [plot.x + 20 / drawingScale, 103, (bounds.width - 80) / drawingScale, 552],
      'known-state': [plot.x - 4, plot.y + (busy?.position ?? 0) / 100 * plot.h, 12, (busy?.width ?? 0) / 100 * plot.h],
      constraints: [plot.x + 20 / drawingScale, plot.y + (busy?.width ?? 0) / 100 * plot.h, (bounds.width - 80) / drawingScale, 2],
      candidates: [plot.x + 25 / drawingScale, 115, (bounds.width - 90) / drawingScale, 535], preferences: [442, 680, 9, 30],
      validity: [plot.x + 20 / drawingScale, 659, (bounds.width - 80) / drawingScale, 2],
      ordering: [(width - bounds.width / drawingScale) / 2 + 20 / drawingScale, 711 - 22 / drawingScale, 185 / drawingScale, 44 / drawingScale],
      proposal: [proposal.x - 6, proposal.y - 3, 12, proposal.dy + 6]
    } : {
      scope: [plot.x - 20, plot.y + 22, plot.w + 40, 495],
      'known-state': [plot.x + (busy?.position ?? 0) / 100 * plot.w, 145, (busy?.width ?? 0) / 100 * plot.w, 473],
      constraints: [plot.x + (busy?.width ?? 0) / 100 * plot.w, 145, 2, 473],
      candidates: [plot.x, 145, plot.w, 473], preferences: [1152, 678, 16, 2],
      validity: [plot.x - 20, 632, plot.w + 40, 2], ordering: [590 - 100 / drawingScale, 700 - 22 / drawingScale, 200 / drawingScale, 44 / drawingScale],
      proposal: [proposal.x - 4, proposal.y - 7, proposal.dx + 8, 14]
    };
    return targets[id];
  };
  // Sample the authored paths once. Interpolating their retained vertices makes
  // the shell itself become the scope/busy geometry, rather than crossfading charts.
  onMount(() => {
    const resize = new ResizeObserver(() => { bounds = { width: svg.clientWidth, height: svg.clientHeight }; });
    resize.observe(svg);
    const result: Record<string, { x: number; y: number }[]> = {};
    svg.querySelectorAll<SVGPathElement>('[data-outline]').forEach(path => {
      const length = path.getTotalLength();
      const points = Array.from({ length: 80 }, (_, i) => {
        const p = path.getPointAtLength(i / 80 * length);
        return { x: p.x, y: p.y };
      });
      const area = points.reduce((sum, p, i) => { const next = points[(i + 1) % points.length]; return sum + p.x * next.y - next.x * p.y; }, 0);
      if (area < 0) points.reverse();
      const minX = Math.min(...points.map(p => p.x)), minY = Math.min(...points.map(p => p.y));
      const spanX = Math.max(...points.map(p => p.x)) - minX, spanY = Math.max(...points.map(p => p.y)) - minY;
      const start = points.reduce((best, p, i) =>
        (p.x - minX) / spanX + (p.y - minY) / spanY < (points[best].x - minX) / spanX + (points[best].y - minY) / spanY ? i : best, 0);
      result[path.dataset.outline!] = [...points.slice(start), ...points.slice(0, start)];
    });
    outlines = result;
    return () => resize.disconnect();
  });
  const morphedOutline = (module: typeof brainModules[number]) => {
    const points = outlines[module.id];
    if (!points) return module.path;
    const [x, y, w, h] = targetBox(module.id);
    return points.map((point, index) => {
      const start = geometry(module, point.x, point.y);
      const edge = index / points.length * 4;
      const end = edge < 1 ? { x: x + edge * w, y }
        : edge < 2 ? { x: x + w, y: y + (edge - 1) * h }
        : edge < 3 ? { x: x + (3 - edge) * w, y: y + h }
        : { x, y: y + (4 - edge) * h };
      return `${index ? 'L' : 'M'}${mix(start.x, end.x, handoff)} ${mix(start.y, end.y, handoff)}`;
    }).join(' ') + ' Z';
  };
  const spine = $derived({
    x1: mix(camera.x, plot.x, handoff), y1: mix(camera.y - 268 * camera.scale, plot.y, handoff),
    x2: mix(camera.x, mobile ? plot.x : plot.x + plot.w, handoff),
    y2: mix(camera.y + 255 * camera.scale, mobile ? plot.y + plot.h : plot.y, handoff)
  });
</script>

<div class="cerebri-brain" class:mobile class:overview data-progress={progress.toFixed(4)} data-activation={activation >= 1 ? 'ready' : activation > .12 ? 'waking' : 'dormant'} style={`--activation:${activation};--svg-font:${13 / drawingScale}px`}>
  <svg bind:this={svg} viewBox={`0 0 ${width} ${height}`} role="img" aria-label="Cerebri: Scope, Known State, Hard Constraints, Preferences, Candidate Space, Validity, Deterministic Ordering, Proposal and a separate Authority Boundary">
    <defs>
      <linearGradient id={uid + '-skin'} x1="0" y1="0" x2=".8" y2="1">
        <stop stop-color="#244663" stop-opacity=".95"/><stop offset=".35" stop-color="#0e2339"/><stop offset="1" stop-color="#101329"/>
      </linearGradient>
      <linearGradient id={uid + '-light'}><stop stop-color="#60e7ff"/><stop offset=".55" stop-color="#4a8fff"/><stop offset="1" stop-color="#bd80ff"/></linearGradient>
      <radialGradient id={uid + '-well'}><stop stop-color="#087baf" stop-opacity=".18"/><stop offset=".6" stop-color="#164169" stop-opacity=".06"/><stop offset="1" stop-color="#061020" stop-opacity="0"/></radialGradient>
      <pattern id={uid + '-hatch'} width="12" height="12" patternUnits="userSpaceOnUse" patternTransform="rotate(30)"><path d="M0 0 V12" stroke="#ef88a8" stroke-opacity=".16"/></pattern>
    </defs>
    <g aria-hidden="true">
      <ellipse class="brain-atmosphere" cx={camera.x} cy={camera.y} rx={mobile ? 240 : 540} ry="380" fill={`url(#${uid}-well)`}/>
      <g class="engineering-orbits" opacity={(1 - handoff) * .25 * activation} transform={`translate(${camera.x} ${camera.y}) scale(${camera.scale})`}>
        <ellipse rx={mix(310, 520, explosion)} ry={mix(282, 315, explosion)} fill="none" stroke="#437290" stroke-dasharray="2 12"/>
        <path d="M-320 0 H-286 M286 0 H320 M0-300 V-280 M0 280 V300" stroke="#5c9eae"/>
      </g>
      {#each brainModules as module, index (module.id)}
        {@const point = geometry(module, module.cx, module.cy)}
        {@const strength = focusStrength(progress, module.id)}
        {@const visibility = 1 - focusActive * (1 - strength) * .58}
        <g class="brain-segment" data-module={module.id} style={`--module-color:${module.color}`} opacity={visibility}>
          <path class="segment-connector" d={`M${camera.x} ${camera.y} Q${mix(camera.x, point.x, .55)} ${camera.y} ${point.x} ${point.y}`} fill="none" stroke={module.color} stroke-dasharray="3 8" opacity={explosion * (1 - handoff) * .36}/>
          <g transform={segmentTransform(module)} opacity={1 - handoff}>
            <path class="segment-depth" d={module.path} transform="translate(0 12)" fill="#030810" stroke={module.color} stroke-opacity=".3"/>
            <path class="segment-rim" d={module.path} transform="translate(0 5)" fill="#091222" stroke={module.color} stroke-opacity=".24"/>
          </g>
          <path class="segment-skin" data-outline={module.id}
            d={handoff > 0 && outlines[module.id] ? morphedOutline(module) : module.path}
            transform={handoff > 0 && outlines[module.id] ? undefined : segmentTransform(module)}
            fill={handoff ? module.id === 'known-state' ? `url(#${uid}-hatch)` : module.id === 'proposal' ? '#22d7f31a' : '#091528' : `url(#${uid}-skin)`}
            fill-opacity={handoff ? module.id === 'known-state' || module.id === 'proposal' ? 1 : mix(1, .03, handoff) : 1}
            stroke={module.color} stroke-width={mix(1.1 + strength, module.id === 'scope' ? 1 : .65, handoff)}
            stroke-opacity={mix(.22 + activation * .55, module.id === 'candidates' || module.id === 'preferences' ? .1 : .6, handoff)}/>
          <g transform={segmentTransform(module)} opacity={(1 - handoff) * (.16 + activation * .8)}>
            {#each module.folds as fold, foldIndex}
              <path class="fold-shadow" d={fold} fill="none" stroke="#010710" stroke-width="5" transform="translate(0 3)"/>
              <path class="signal-path" data-signal={module.id} d={fold} pathLength="1" stroke={module.color} fill="none" stroke-width="1.8" stroke-linecap="round"
                stroke-dasharray="1" stroke-dashoffset={1 - ramp(activation, .08 + index * .06 + foldIndex * .03, .52 + index * .045)}/>
            {/each}
            {#if module.id === 'constraints'}
              <path d="M-174 156 L-153 142 M-161 169 L-140 155 M-148 182 L-127 168" fill="none" stroke={module.color} stroke-width="4"/>
            {:else if module.id === 'preferences'}
              <path d="M166-19 Q250-22 224 69 M161-11 Q232-5 218 56 M159 2 Q213 6 208 43" fill="none" stroke={module.color} stroke-opacity=".35"/>
            {:else if module.id === 'ordering'}
              {#each [-47, -20, 7, 34] as y}
                <path d={`M-22 ${y} L0 ${y + 12} L22 ${y}`} fill="none" stroke="#b1efff" stroke-opacity={.25 + activation * .5}/>
              {/each}
            {:else if module.id === 'validity'}
              <path d="M103 175 L118 189 L150 153" fill="none" stroke={module.color} stroke-width="3"/>
            {/if}
          </g>
        </g>
      {/each}
      <path class="temporal-spine" data-handoff="time-rail" d={`M${spine.x1} ${spine.y1} C${mix(spine.x1 - 8, spine.x1, handoff)} ${mix(spine.y1 + 180, spine.y1, handoff)} ${mix(spine.x2 + 8, spine.x2, handoff)} ${mix(spine.y2 - 180, spine.y2, handoff)} ${spine.x2} ${spine.y2}`}
        fill="none" stroke={`url(#${uid}-light)`} stroke-width="1.7" pathLength="1" stroke-dasharray="1" stroke-dashoffset={1 - ramp(activation, 0, .36)}/>
      {#each [0, 1, 2, 3] as tick}
        <g opacity={handoff} transform={`translate(${mobile ? plot.x : plot.x + tick / 3 * plot.w} ${mobile ? plot.y + tick / 3 * plot.h : plot.y})`}>
          <path d={mobile ? 'M-5 0 H5' : 'M0-5 V5'} stroke="#93b9d3"/>
          <text class="time-label" x={mobile ? -13 : 0} y={mobile ? 5 : -16} text-anchor={mobile ? 'end' : tick === 0 ? 'start' : tick === 3 ? 'end' : 'middle'}>{timeLabel(scopeStart + (scopeEnd - scopeStart) * tick / 3)}</text>
        </g>
      {/each}
      <g class="brain-candidates" data-handoff="candidate-field">
        {#each candidates as candidate, index (candidate.id)}
          {@const origin = geometry(brainModules[3], 66 + index % 4 * 31, -206 + Math.floor(index / 4) * 32)}
          {@const target = projectedCandidate(index)}
          {@const x = mix(origin.x, target.x, handoff)}
          {@const y = mix(origin.y, target.y, handoff)}
          {@const extent = ramp(progress, .9, .985)}
          {@const ink = progress < .935 ? '#a4f4ff' : candidate.state === 'rejected' ? '#f38dac' : candidate.state === 'selected' ? '#42d9ff' : '#66e9c0'}
          <g class="brain-candidate" data-candidate-id={candidate.id} data-start={candidate.start} data-state={candidate.state} transform={`translate(${x} ${y})`}>
            <path class="candidate-interval" d={`M0 0 L${target.dx * extent} ${target.dy * extent}`} stroke={ink} stroke-width={mobile ? 6 : 7} opacity=".48"/>
            <circle class="capsule-origin" r={mix(3, 4, handoff)} fill={ink} opacity={.25 + activation * .75}/>
            <circle class="capsule-end" cx={target.dx * extent} cy={target.dy * extent} r="4" fill="#061020" stroke={ink} opacity={extent}/>
            {#if candidate.state === 'rejected'}
              <path class="rejection-cut" d={mobile ? 'M-9 20 L9 13' : 'M17-11 L9 11'} stroke={ink} stroke-width="2" opacity={extent}/>
            {/if}
            <text class="candidate-identity" x={mobile ? 13 : -38} y={mobile ? 4 : 5} opacity={handoff}>{candidate.id}</text>
            <text class="candidate-time" x={mobile ? 13 : 0} y={mobile ? 25 : -15} opacity={handoff}>{candidate.label}</text>
          </g>
        {/each}
      </g>
      <g class="authority-boundary" data-module="authority" transform={`translate(${mix(camera.x + (mobile ? mix(263, 450, explosion) : mix(340, 620, explosion)) * camera.scale, mobile ? plot.x + (bounds.width - 75) / drawingScale : 1095, handoff)} ${mix(camera.y + 185 * camera.scale, mobile ? 710 : 700, handoff)})`} opacity={ramp(activation, .7, 1)}>
        <path d={`M-12 ${mix(-70, -18, handoff)} V${mix(70, 18, handoff)} M-3 ${mix(-70, -18, handoff)} V${mix(70, 18, handoff)}`} fill="none" stroke="#dcb779" stroke-width="2"/>
        <path d="M-78 0 H-40" stroke="#49ddec"/><circle cx="-40" r="3" fill="#061020" stroke="#49ddec"/>
      </g>
    </g>
  </svg>
  {#each brainModules as module}
    {@const point = geometry(module, module.cx, module.cy)}
    {@const strength = focusStrength(progress, module.id)}
    <span class="module-label" data-label={module.id} aria-hidden="true" style={`${labelPosition(point.x, point.y + (module.id === 'ordering' ? 136 : 101) * camera.scale)};--label-color:${module.color};opacity:${labelReveal * (1 - focusActive * (1 - strength) * .45)}`}>{module.label}</span>
  {/each}
  <span class="authority-label" aria-hidden="true" style={`${labelPosition(mix(camera.x + (mobile ? mix(263, 450, explosion) : mix(340, 620, explosion)) * camera.scale, mobile ? plot.x + (bounds.width - 75) / drawingScale : 1095, handoff), mix(camera.y + 278 * camera.scale, mobile ? 750 : 747, handoff))};opacity:${ramp(progress, .26, .4)}`}>Authority Boundary</span>
  {#if progress >= .98 && !overview}
    {@const box = targetBox('ordering')}
    <a class="comparator-entry" href="#candidate-comparison" style={labelPosition(box[0] + box[2] / 2, box[1] + box[3] / 2)}>Deterministic Ordering <span aria-hidden="true">→</span></a>
  {/if}
</div>

<style>
  .cerebri-brain{position:relative;width:100%;height:100%;isolation:isolate}
  svg{display:block;width:100%;height:100%;overflow:visible}
  .segment-skin,.segment-depth,.segment-rim,.fold-shadow,.signal-path,.segment-connector,.temporal-spine{vector-effect:non-scaling-stroke}
  .segment-skin{stroke-linejoin:round}
  .time-label,.candidate-identity,.candidate-time{font:var(--svg-font,13px) var(--mono);fill:#a8bed4}
  .module-label,.authority-label{position:absolute;left:0;top:0;transform:translate(var(--label-x),var(--label-y)) translate(-50%,-50%);font:.8125rem/1.35 var(--sans);color:var(--label-color,#dcb779);text-align:center;width:max-content;max-width:13rem;pointer-events:none;text-shadow:0 2px 8px #030711,0 0 5px #030711}
  .mobile .module-label{max-width:7.2rem;font-size:.8125rem}.authority-label{transform:translate(var(--label-x),var(--label-y)) translate(-100%,-50%)}.mobile .authority-label{max-width:6rem;font-size:.8125rem}
  .mobile [data-label="constraints"]{max-width:5rem}
  .comparator-entry{position:absolute;left:0;top:0;transform:translate(var(--label-x),var(--label-y)) translate(-50%,-50%);display:flex;align-items:center;justify-content:center;gap:.5rem;min-height:44px;padding:0 .6rem;color:#a4ebff;font:.8125rem var(--sans);text-decoration:none}
  .comparator-entry:hover{background:#0f2841}.comparator-entry:focus-visible{outline:2px solid var(--cyan);outline-offset:2px}
  @media(prefers-reduced-motion:reduce){.cerebri-brain *{animation:none!important;transition:none!important}}
</style>
