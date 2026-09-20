<script lang="ts">
  import { base } from '$app/paths';
  import IntentPlan from '$lib/components/IntentPlan.svelte';
  import SourceLink from '$lib/components/SourceLink.svelte';
  import StatusBadge from '$lib/components/StatusBadge.svelte';
  import { runtimeData } from '$lib/generated/runtime-data';
</script>

<svelte:head>
  <title>Nexus Cerebri · Reason freely. Verify everything.</title>
  <meta
    name="description"
    content="The official Nexus Cerebri knowledge surface for deterministic temporal planning, CPIR, lifecycle safety and research boundaries."
  />
</svelte:head>

<section class="hero">
  <div class="hero-copy">
    <div class="hero-meta">
      <StatusBadge status="Foundation" />
      <span>Software {runtimeData.metadata.softwareVersion}</span>
      <span>Spec {runtimeData.metadata.specVersion}</span>
      <span>CPIR {runtimeData.metadata.cpirVersion}</span>
    </div>
    <p class="kicker">NEXUS CEREBRI / OPEN RESEARCH FOUNDATION</p>
    <h1>Reason freely.<br /><em>Verify everything.</em></h1>
    <p class="hero-lede">
      An application-independent temporal planning foundation where probabilistic interpretation can
      explore meaning, while deterministic layers preserve facts, constraints, permissions and execution safety.
    </p>
    <div class="hero-actions">
      <a class="primary-action" href={base + '/explore/'}>Enter the Atlas <span>→</span></a>
      <a class="secondary-action" href={base + '/docs/'}>Read canonical docs</a>
    </div>
    <SourceLink path="docs/architecture/specifications/master-v0.4.md" label="Master Specification 0.4" />
  </div>
  <div class="hero-field" aria-hidden="true">
    <div class="orbit orbit-a"></div><div class="orbit orbit-b"></div><div class="orbit orbit-c"></div>
    <div class="field-core"><span>N</span><small>proof-bound planning</small></div>
    <span class="field-node f1">time</span><span class="field-node f2">scope</span><span class="field-node f3">policy</span>
    <span class="field-node f4">search</span><span class="field-node f5">proof</span><span class="field-node f6">trace</span>
  </div>
</section>

<section class="metrics" aria-label="Repository authority">
  <article><small>SOFTWARE</small><strong>{runtimeData.metadata.softwareVersion}</strong><span>Cargo workspace authority</span></article>
  <article><small>SPECIFICATION</small><strong>{runtimeData.metadata.specVersion}</strong><span>Current Master baseline</span></article>
  <article><small>CPIR</small><strong>{runtimeData.metadata.cpirVersion}</strong><span>Internal serialized schema</span></article>
  <article><small>RUST</small><strong>{runtimeData.metadata.rustVersion}</strong><span>Pinned workspace language floor</span></article>
</section>

<IntentPlan />

<section class="home-grid">
  <article class="feature-card">
    <span class="truth-label" data-kind="REAL">REAL</span>
    <h2>Deterministic core output</h2>
    <p>The website build executes the Rust planner and temporal diagnostic examples. Displayed fixture results therefore come from the same core authority as the transports.</p>
    <a href={base + '/planning/'}>Inspect planning field →</a>
  </article>
  <article class="feature-card">
    <span class="truth-label" data-kind="EDUCATIONAL">EDUCATIONAL</span>
    <h2>Architecture you can read visually</h2>
    <p>Diagrams explain ownership and lifecycle boundaries without pretending that an animation is a runtime trace.</p>
    <a href={base + '/architecture/'}>See the constellation →</a>
  </article>
  <article class="feature-card">
    <span class="truth-label" data-kind="FUTURE CONCEPT">FUTURE CONCEPT</span>
    <h2>Research without product theater</h2>
    <p>Learning, neural planning, advanced repair and real provider execution stay visibly future until repository truth says otherwise.</p>
    <a href={base + '/roadmap/'}>Read roadmap strata →</a>
  </article>
</section>
