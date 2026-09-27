<script lang="ts">
  import '$lib/creative-fidelity.css';
  import { instrumentEnvironment } from '$lib/instrument-environment';
  import { ranked, candidateIdentity, timeLabel } from '$lib/planning-display';
  import { base } from '$app/paths';
  import IntentPlan from '$lib/components/IntentPlan.svelte';
  import BoundedSearchObservatory from '$lib/components/BoundedSearchObservatory.svelte';
  import CerebriExplainer from '$lib/components/CerebriExplainer.svelte';
  import MathematicalInspection from '$lib/components/math/MathematicalInspection.svelte';
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
  <title>Nexus Cerebri · Find a time and understand the result</title>
  <meta
    name="description"
    content="See how Nexus Cerebri checks possible appointment times, rules out conflicts and returns a proposal you can inspect. Explore the project in English or German."
  />
</svelte:head>

<div class="instrument-story" use:instrumentEnvironment>
<section class="hero cosmic-hero" data-world="space" use:pointerField>
  <div class="hero-nebula hero-nebula-a" aria-hidden="true"></div>
  <div class="hero-nebula hero-nebula-b" aria-hidden="true"></div>
  <div class="hero-copy">
    <div class="hero-badge"><span class="badge-orbit" aria-hidden="true"></span><b>NEXUS CEREBRI</b><span>OPEN-SOURCE PLANNING PROJECT</span><i aria-hidden="true"></i></div>
    <h1><span>Find a time.</span><em>Understand the choice.</em></h1>
    <p class="hero-lede">
      Cerebri is a learning project exploring how software can make plans and explain its results.
      Its current example is an appointment: given a duration, a time window and rules as structured data,
      the planner checks possible times and returns a proposal. It does not book the appointment.
    </p>
    <div class="hero-actions">
      <a class="primary-action" href={base + '/explore/'}><span>Explore Cerebri</span><b aria-hidden="true">↗</b></a>
      <a class="secondary-action" href={base + '/docs/de/introduction/'} lang="de" hreflang="de"><span>Auf Deutsch lesen</span><b aria-hidden="true">→</b></a>
    </div>
    <div class="hero-release-row" role="group" aria-label="Release status">
      <span>Published release: {runtimeData.metadata.publishedRelease}</span>
      <span>In development · release checks in progress</span>
      <span>Input format: CPIR {runtimeData.metadata.cpirVersion}</span>
    </div>
  </div>

  <BoundedSearchObservatory />

  <div class="hero-scroll-cue" aria-hidden="true"><span>SEE HOW IT WORKS</span><i></i><b></b></div>
</section>

<div class="home-story">
  <div class="coordinate-relay" aria-hidden="true"><span><i></i>{candidateIdentity(ranked[0].start)} · {timeLabel(ranked[0].start)}</span></div>
  <div class="story-spine" aria-hidden="true"><span></span><i></i><b></b></div>

<section class="metrics" aria-label="Repository authority">
  <div class="metrics-intro">
    <span class="kicker">PROJECT STATUS</span>
    <h2>Available to explore.<br /><em>Still in development.</em></h2>
  </div>
  <div class="metric-orbit" aria-hidden="true"><i></i><span></span></div>
  <article><small>SOFTWARE</small><strong>{runtimeData.metadata.softwareVersion}</strong><span>{runtimeData.metadata.releaseStatus} · release checks in progress</span></article>
  <article><small>SPECIFICATION</small><strong>{runtimeData.metadata.specVersion}</strong><span>Architecture document version</span></article>
  <article><small>CPIR</small><strong>{runtimeData.metadata.cpirVersion}</strong><span>Planning input format version</span></article>
  <article><small>RUST</small><strong>{runtimeData.metadata.rustVersion}</strong><span>Minimum Rust version for this workspace</span></article>
  <SourceLink path="docs/architecture/specifications/master-v0.4.md" label="Master Specification 0.4" />
</section>

<CerebriExplainer />
<IntentPlan />
<MathematicalInspection />
</div>

<section data-world="horizon" class="domain-journey cosmic-section" aria-labelledby="journey-title">
  <div class="section-copy journey-copy">
    <span class="section-index">EXPLORE THE DETAILS</span>
    <span class="kicker">TIME · INPUT · PLANNING · EXECUTION</span>
    <h2 id="journey-title">What would you<br /><em>like to explore?</em></h2>
    <p>Follow a topic from the example to its data format, rules and implementation.</p>
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
      <span class="journey-nebula" aria-hidden="true"></span><small>06 / HORIZON</small><h3>Where could learning help?</h3><p>Explore how the learning project could grow from fixed planning rules to tested, learned advice.</p><strong>View Roadmap <b>→</b></strong>
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
    <h3>Learning is the next question</h3>
    <p>Could a model help interpret a request or compare valid times? These are future experiments. The current planner does not train a model or learn from choices.</p>
    <a href={base + '/docs/en/research/'}>Explore the research questions <span>→</span></a>
  </article>
</section>

<section class="home-finale cosmic-section">
  <div class="finale-planet" aria-hidden="true"><span></span><i></i></div>
  <div class="finale-copy">
    <span class="kicker">THE PROJECT’S BEGINNING</span>
    <h2>Why I started<br /><em>Nexus Cerebri.</em></h2>
    <p>I wanted to understand how neural networks learn. The <a href="https://www.nature.com/articles/323533a0">1986 backpropagation paper by David E. Rumelhart, Geoffrey E. Hinton and Ronald J. Williams</a> helped draw me into the mathematics. Following each operation turned equations I found intimidating into steps I could understand.</p>
    <p>Cerebri became a way to keep learning by building and testing a real system. Planning gave me a concrete problem: what is already known, which rules must hold, and why does one option come before another? Scheduling is the first example; the longer-term aim is planning software for different applications.</p>
    <p>The planner shown here follows fixed rules. Later, I want to explore how learned methods could help it find and compare options. The <a href={base + '/docs/en/research/'}>learning and research path</a> explains those questions and the checks that would still apply.</p>
    <div class="hero-actions">
      <a class="primary-action" href={base + '/docs/en/introduction/'}><span>Read the story and example</span><b>↗</b></a>
      <a class="secondary-action" href={base + '/docs/de/introduction/'} lang="de" hreflang="de"><span>Auf Deutsch lesen</span><b>→</b></a>
    </div>
  </div>
</section>

</div>
