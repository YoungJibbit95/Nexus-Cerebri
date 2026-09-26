<script lang="ts">
  import { base } from '$app/paths';
  import IntentPlan from '$lib/components/IntentPlan.svelte';
  import BoundedSearchObservatory from '$lib/components/BoundedSearchObservatory.svelte';
  import CerebriExplainer from '$lib/components/CerebriExplainer.svelte';
  import SourceLink from '$lib/components/SourceLink.svelte';
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
  <title>Nexus Cerebri · Deterministic temporal planning</title>
  <meta
    name="description"
    content="The official Nexus Cerebri knowledge surface for deterministic temporal planning, CPIR, lifecycle safety and research boundaries."
  />
</svelte:head>

<section class="hero cosmic-hero" use:pointerField>
  <div class="hero-nebula hero-nebula-a" aria-hidden="true"></div>
  <div class="hero-nebula hero-nebula-b" aria-hidden="true"></div>
  <div class="hero-copy">
    <div class="hero-badge"><span class="badge-orbit" aria-hidden="true"></span><b>NEXUS CEREBRI</b><span>DETERMINISTIC PLANNING FOUNDATION</span><i aria-hidden="true"></i></div>
    <h1><span>Bound the problem.</span><em>Verify the plan.</em></h1>
    <p class="hero-lede">
      Nexus Cerebri turns structured temporal context into a bounded search. Known state and hard rules reject invalid
      candidates, deterministic ordering compares what remains, and a proposal never becomes execution authority by itself.
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

  <BoundedSearchObservatory />

  <div class="hero-scroll-cue" aria-hidden="true"><span>FOLLOW THE SEARCH</span><i></i><b></b></div>
</section>

<div class="home-story">
  <div class="story-spine" aria-hidden="true"><span></span><i></i><b></b></div>

<section class="metrics" aria-label="Repository authority">
  <div class="metrics-intro">
    <span class="kicker">CURRENT REPOSITORY STATE</span>
    <h2>The system you can inspect.<br /><em>Today.</em></h2>
  </div>
  <div class="metric-orbit" aria-hidden="true"><i></i><span></span></div>
  <article><small>SOFTWARE</small><strong>{runtimeData.metadata.softwareVersion}</strong><span>{runtimeData.metadata.releaseStatus} · qualification in progress</span></article>
  <article><small>SPECIFICATION</small><strong>{runtimeData.metadata.specVersion}</strong><span>Current Master baseline</span></article>
  <article><small>CPIR</small><strong>{runtimeData.metadata.cpirVersion}</strong><span>Structured planning schema</span></article>
  <article><small>RUST</small><strong>{runtimeData.metadata.rustVersion}</strong><span>Pinned workspace language floor</span></article>
  <SourceLink path="docs/architecture/specifications/master-v0.4.md" label="Master Specification 0.4" />
</section>

<CerebriExplainer />
<IntentPlan />
</div>

<section class="domain-journey cosmic-section" aria-labelledby="journey-title">
  <div class="section-copy journey-copy">
    <span class="section-index">ACT 04–09 / CEREBRI UNIVERSE</span>
    <span class="kicker">ONE SYSTEM · EXPLICIT BOUNDARIES</span>
    <h2 id="journey-title">Follow the system.<br /><em>Keep authority explicit.</em></h2>
    <p>Move from typed time and CPIR through bounded search, lifecycle safety, architecture and future research.</p>
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
    <p class="kicker">CURRENT · EXPLANATORY · FUTURE</p>
    <h2>Current behavior stays separate from future research.</h2>
    <p>Repository-backed outputs, explanatory diagrams and future concepts are labeled separately so their authority is never interchangeable.</p>
  </div>
  <article class="feature-card real-surface">
    <div class="feature-orb" aria-hidden="true"></div><span class="truth-label" data-kind="REAL">REAL</span>
    <h3>Deterministic core output</h3>
    <p>The website build executes Rust planner and temporal diagnostic examples. Displayed fixture results come from the same core authority as the transports.</p>
    <a href={base + '/planning/'}>Inspect planning field <span>→</span></a>
  </article>
  <article class="feature-card educational-surface">
    <div class="feature-orb" aria-hidden="true"></div><span class="truth-label" data-kind="EDUCATIONAL">EDUCATIONAL</span>
    <h3>Architecture shown as explanation</h3>
    <p>Diagrams explain ownership and lifecycle boundaries without presenting educational motion as runtime behavior.</p>
    <a href={base + '/architecture/'}>See the constellation <span>→</span></a>
  </article>
  <article class="feature-card future-surface">
    <div class="feature-orb" aria-hidden="true"></div><span class="truth-label" data-kind="FUTURE CONCEPT">FUTURE CONCEPT</span>
    <h3>Research stays visibly future</h3>
    <p>Learning, neural planning, advanced repair and production provider execution remain future work until implementation exists.</p>
    <a href={base + '/roadmap/'}>Read roadmap strata <span>→</span></a>
  </article>
</section>

<section class="home-finale cosmic-section">
  <div class="finale-planet" aria-hidden="true"><span></span><i></i></div>
  <div class="finale-copy">
    <span class="kicker">ACT 10 / ENTER THE SYSTEM</span>
    <h2>Inspect what exists.<br /><em>See what comes later.</em></h2>
    <p>Start with the system map, inspect the real planning fixture, or go straight to the repository documentation.</p>
    <div class="hero-actions">
      <a class="primary-action" href={base + '/explore/'}><span>Enter the Atlas</span><b>↗</b></a>
      <a class="secondary-action" href={base + '/docs/'}><span>Canonical Docs</span><b>→</b></a>
    </div>
  </div>
</section>
