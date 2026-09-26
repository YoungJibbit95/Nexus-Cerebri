<script lang="ts">
  import { base } from '$app/paths';
  import IntentPlan from '$lib/components/IntentPlan.svelte';
  import CerebriExplainer from '$lib/components/CerebriExplainer.svelte';
  import SourceLink from '$lib/components/SourceLink.svelte';
  import StatusBadge from '$lib/components/StatusBadge.svelte';
  import { runtimeData } from '$lib/generated/runtime-data';

  function pointerField(node: HTMLElement) {
    let frame = 0;
    const move = (event: PointerEvent) => {
      if (event.pointerType === 'touch' || window.matchMedia('(prefers-reduced-motion: reduce)').matches) return;
      const rect = node.getBoundingClientRect();
      const x = ((event.clientX - rect.left) / rect.width - 0.5) * 2;
      const y = ((event.clientY - rect.top) / rect.height - 0.5) * 2;
      cancelAnimationFrame(frame);
      frame = requestAnimationFrame(() => {
        node.style.setProperty('--pointer-x', x.toFixed(3));
        node.style.setProperty('--pointer-y', y.toFixed(3));
      });
    };
    const reset = () => {
      cancelAnimationFrame(frame);
      frame = requestAnimationFrame(() => {
        node.style.setProperty('--pointer-x', '0');
        node.style.setProperty('--pointer-y', '0');
      });
    };
    node.addEventListener('pointermove', move);
    node.addEventListener('pointerleave', reset);
    return { destroy() { cancelAnimationFrame(frame); node.removeEventListener('pointermove', move); node.removeEventListener('pointerleave', reset); } };
  }
</script>

<svelte:head>
  <title>Nexus Cerebri · Reason freely. Verify everything.</title>
  <meta
    name="description"
    content="The official Nexus Cerebri knowledge surface for deterministic temporal planning, CPIR, lifecycle safety and research boundaries."
  />
</svelte:head>

<section class="hero cosmic-hero" use:pointerField>
  <div class="hero-nebula hero-nebula-a" aria-hidden="true"></div>
  <div class="hero-nebula hero-nebula-b" aria-hidden="true"></div>
  <div class="hero-constellation" aria-hidden="true">
    <svg viewBox="0 0 900 560" preserveAspectRatio="none">
      <path d="M90 380 C210 260 290 318 390 214 S610 118 785 205" />
      <path d="M250 94 C350 175 440 180 512 300 S650 430 822 396" />
      <circle cx="90" cy="380" r="4"/><circle cx="390" cy="214" r="4"/><circle cx="785" cy="205" r="4"/>
      <circle cx="250" cy="94" r="3"/><circle cx="512" cy="300" r="5"/><circle cx="822" cy="396" r="3"/>
    </svg>
  </div>

  <div class="hero-space-object hero-planet" aria-hidden="true"><span></span><i></i></div>
  <div class="hero-space-object hero-moon" aria-hidden="true"></div>
  <div class="hero-space-object hero-satellite" aria-hidden="true"><i></i><span></span><b></b></div>
  <div class="hero-space-object hero-probe" aria-hidden="true"><i></i><span></span></div>

  <div class="hero-copy">
    <div class="hero-badge"><span class="badge-orbit" aria-hidden="true"></span><b>NEXUS CEREBRI</b><span>DETERMINISTIC PLANNING FOUNDATION</span><i aria-hidden="true"></i></div>
    <h1><span>Reason freely.</span><em>Verify everything.</em></h1>
    <p class="hero-lede">
      Cerebri separates flexible interpretation from binding action. Explicit evidence, time, scope and constraints
      become deterministic, inspectable plans — without treating a model's guess as permission.
    </p>
    <div class="hero-actions">
      <a class="primary-action" href={base + '/explore/'}><span>Explore Cerebri</span><b aria-hidden="true">↗</b></a>
      <a class="secondary-action" href={base + '/docs/'}><span>Read the Specification</span><b aria-hidden="true">→</b></a>
    </div>
    <div class="hero-release-row" role="group" aria-label="Release status">
      <span>Published release: {runtimeData.metadata.publishedRelease}</span>
      <span>Unreleased · qualification in progress</span>
      <span>CPIR {runtimeData.metadata.cpirVersion}</span>
    </div>
  </div>

  <div class="hero-field orbital-intelligence" aria-hidden="true">
    <div class="orbital-halo halo-one" aria-hidden="true"></div>
    <div class="orbital-halo halo-two" aria-hidden="true"></div>
    <div class="orbital-halo halo-three" aria-hidden="true"></div>
    <svg class="field-links" viewBox="0 0 700 700" role="presentation">
      <defs>
        <linearGradient id="orbital-trace" x1="0" y1="0" x2="1" y2="1">
          <stop offset="0" stop-color="#20d8ff" stop-opacity="0"/>
          <stop offset=".44" stop-color="#20d8ff" stop-opacity=".8"/>
          <stop offset=".7" stop-color="#9b8cff" stop-opacity=".72"/>
          <stop offset="1" stop-color="#a147ff" stop-opacity="0"/>
        </linearGradient>
      </defs>
      <ellipse cx="350" cy="350" rx="240" ry="124" transform="rotate(-14 350 350)" />
      <ellipse cx="350" cy="350" rx="265" ry="170" transform="rotate(38 350 350)" />
      <ellipse cx="350" cy="350" rx="195" ry="282" transform="rotate(12 350 350)" />
      <path class="trace trace-a" d="M96 340 C210 205 300 204 350 350 C420 500 532 530 626 388" />
      <path class="trace trace-b" d="M160 180 C275 285 298 402 350 350 C435 266 510 215 584 198" />
    </svg>
    <div class="field-core">
      <span class="core-glow" aria-hidden="true"></span>
      <img src={base + '/nexus-cerebri-logo.png'} alt="" />
      <b>CEREBRI CORE</b>
      <small>proof-bound planning</small>
      <i aria-hidden="true"></i>
    </div>
    <div class="field-node f1"><i></i><span>TIME</span><small>typed domain</small></div>
    <div class="field-node f2"><i></i><span>SCOPE</span><small>orbital envelope</small></div>
    <div class="field-node f3"><i></i><span>POLICY</span><small>declared rules</small></div>
    <div class="field-node f4"><i></i><span>SEARCH</span><small>trajectory grid</small></div>
    <div class="field-node f5"><i></i><span>PROOF</span><small>locked result</small></div>
    <div class="field-node f6"><i></i><span>TRACE</span><small>evidence path</small></div>
    <span class="field-readout">ORBITAL INTELLIGENCE SYSTEM · AUTHORITY FLOWS DIRECTIONALLY</span>
  </div>

  <div class="hero-scroll-cue" aria-hidden="true"><span>ENTER THE FIELD</span><i></i><b></b></div>
</section>

<div class="home-story">
  <div class="story-spine" aria-hidden="true"><span></span><i></i><b></b></div>

<section class="metrics" aria-label="Repository authority">
  <div class="metrics-intro">
    <span class="kicker">LIVE REPOSITORY TRUTH</span>
    <h2>Built from what exists.<br /><em>Not what demos well.</em></h2>
  </div>
  <div class="metric-orbit" aria-hidden="true"><i></i><span></span></div>
  <article><small>SOFTWARE</small><strong>{runtimeData.metadata.softwareVersion}</strong><span>{runtimeData.metadata.releaseStatus} · qualification in progress</span></article>
  <article><small>SPECIFICATION</small><strong>{runtimeData.metadata.specVersion}</strong><span>Current Master baseline</span></article>
  <article><small>CPIR</small><strong>{runtimeData.metadata.cpirVersion}</strong><span>Internal serialized schema</span></article>
  <article><small>RUST</small><strong>{runtimeData.metadata.rustVersion}</strong><span>Pinned workspace language floor</span></article>
  <SourceLink path="docs/architecture/specifications/master-v0.4.md" label="Master Specification 0.4" />
</section>

<CerebriExplainer />
<IntentPlan />
</div>

<section class="domain-journey cosmic-section" aria-labelledby="journey-title">
  <div class="section-copy journey-copy">
    <span class="section-index">ACT 04–09 / CEREBRI UNIVERSE</span>
    <span class="kicker">ONE SYSTEM · DISTINCT DOMAINS</span>
    <h2 id="journey-title">Travel the system.<br /><em>Keep the boundaries.</em></h2>
    <p>Each domain gets its own visual atmosphere while remaining part of the same Nexus deep-space language.</p>
  </div>

  <div class="journey-grid">
    <a class="journey-card time-card" href={base + '/time/'}>
      <span class="journey-orbit" aria-hidden="true"><i></i><b></b></span><small>01 / TEMPORAL</small><h3>Time is a domain.</h3><p>UTC instants, explicit zones, half-open intervals, DST diagnostics and bounded recurrence.</p><strong>Open Temporal Lens <b>→</b></strong>
    </a>
    <a class="journey-card cpir-card" href={base + '/cpir/'}>
      <span class="journey-constellation" aria-hidden="true"><i></i><i></i><i></i><b></b></span><small>02 / EVIDENCE</small><h3>Knowledge has shape.</h3><p>CPIR keeps scope, provenance, policy, capability and search budget visibly separate.</p><strong>Explore CPIR <b>→</b></strong>
    </a>
    <a class="journey-card planning-card" href={base + '/planning/'}>
      <span class="journey-trajectory" aria-hidden="true"><i></i><b></b></span><small>03 / SEARCH</small><h3>Search. Constrain. Verify.</h3><p>Bounded single-event grid search ranks feasible candidates deterministically.</p><strong>Inspect Planning <b>→</b></strong>
    </a>
    <a class="journey-card safety-card" href={base + '/safety/'}>
      <span class="journey-lock" aria-hidden="true"><i></i><b></b></span><small>04 / PROOF</small><h3>Promotion is earned.</h3><p>Planning, validation, authorization and execution stay separated by stronger states.</p><strong>Follow Proof Chain <b>→</b></strong>
    </a>
    <a class="journey-card architecture-card" href={base + '/architecture/'}>
      <span class="journey-system" aria-hidden="true"><i></i><i></i><i></i><b></b></span><small>05 / AUTHORITY</small><h3>Architecture with direction.</h3><p>Foundations feed domain semantics and thin transports without moving authority outward.</p><strong>See Architecture <b>→</b></strong>
    </a>
    <a class="journey-card roadmap-card future" href={base + '/roadmap/'}>
      <span class="journey-nebula" aria-hidden="true"></span><small>06 / HORIZON</small><h3>Research stays distant.</h3><p>Providers, learning and neural layers remain later milestones until repository truth changes.</p><strong>View Roadmap <b>→</b></strong>
    </a>
  </div>
</section>

<section class="home-grid cosmic-section" aria-label="Truth surfaces">
  <div class="home-grid-intro">
    <p class="kicker">THREE TRUTH ATMOSPHERES</p>
    <h2>Beautiful does not mean ambiguous.</h2>
    <p>Implemented facts, explanatory models and future research share one visual universe without sharing one level of authority.</p>
  </div>
  <article class="feature-card real-surface">
    <div class="feature-orb" aria-hidden="true"></div><span class="truth-label" data-kind="REAL">REAL</span>
    <h3>Deterministic core output</h3>
    <p>The website build executes Rust planner and temporal diagnostic examples. Displayed fixture results come from the same core authority as the transports.</p>
    <a href={base + '/planning/'}>Inspect planning field <span>→</span></a>
  </article>
  <article class="feature-card educational-surface">
    <div class="feature-orb" aria-hidden="true"></div><span class="truth-label" data-kind="EDUCATIONAL">EDUCATIONAL</span>
    <h3>Architecture you can read visually</h3>
    <p>Diagrams explain ownership and lifecycle boundaries without pretending that an animation is a runtime trace.</p>
    <a href={base + '/architecture/'}>See the constellation <span>→</span></a>
  </article>
  <article class="feature-card future-surface">
    <div class="feature-orb" aria-hidden="true"></div><span class="truth-label" data-kind="FUTURE CONCEPT">FUTURE CONCEPT</span>
    <h3>Research without product theater</h3>
    <p>Learning, neural planning, advanced repair and real provider execution stay visibly future until repository truth says otherwise.</p>
    <a href={base + '/roadmap/'}>Read roadmap strata <span>→</span></a>
  </article>
</section>

<section class="home-finale cosmic-section">
  <div class="finale-planet" aria-hidden="true"><span></span><i></i></div>
  <div class="finale-copy">
    <span class="kicker">ACT 10 / ENTER THE SYSTEM</span>
    <h2>Explore the known.<br /><em>Keep the unknown visible.</em></h2>
    <p>Start with the Atlas for the system map, or go straight to the canonical repository documentation.</p>
    <div class="hero-actions">
      <a class="primary-action" href={base + '/explore/'}><span>Enter the Atlas</span><b>↗</b></a>
      <a class="secondary-action" href={base + '/docs/'}><span>Canonical Docs</span><b>→</b></a>
    </div>
  </div>
</section>
