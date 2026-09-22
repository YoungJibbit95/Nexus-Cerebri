<script lang="ts">
  import { base } from '$app/paths';
  import SourceLink from '$lib/components/SourceLink.svelte';
  import StatusBadge from '$lib/components/StatusBadge.svelte';
  import SystemVisual from '$lib/components/SystemVisual.svelte';
  import SectionPrimer from '$lib/components/SectionPrimer.svelte';
  import VisualizationFrame from '$lib/components/VisualizationFrame.svelte';
  import { runtimeData } from '$lib/generated/runtime-data';

  let { data } = $props();
  const section = $derived(data.section);
  const sectionNumber = $derived(String(['explore','cpir','planning','time','safety','architecture','lab','roadmap','developers'].indexOf(data.key) + 1).padStart(2, '0'));
  const textAlternatives: Record<string, string> = {
    atlas: 'A central Cerebri node connects to Temporal, CPIR, Constraints, Planner and Validation. A visually distinct Research node sits outside the implemented core.',
    cpir: 'Connected evidence layers summarize request identity, knowledge and provenance, scope, policy, capability and search budget. Scope is visually bounded and capability remains separate.',
    planning: 'A three-hour search field shows a known busy block, deterministic grid positions, evaluated candidates and the selected candidate. A proof strip states outcome and assessment from Rust output.',
    time: 'Three time paths explain half-open intervals, a DST gap and a DST fold, with raw temporal diagnostic output available in a disclosure.',
    safety: 'A one-way promotion pipeline progresses from PlanningRequest through ProposedPlan, ValidatedPlan, ActionPlan and AuthorizedActionPlan to ExecutionResult while keeping planner and executor boundaries distinct.',
    architecture: 'Layered planes show types and temporal foundations, semantic and constraint layers, planner, core facade and thin transports. A separate research node is not part of production dependency flow.',
    roadmap: 'Four strata show implemented Foundation and Temporal Core, deterministic planner verification and later provider, repair, learning and neural work as a separate research horizon.',
    developers: 'A transport boundary lists validate, plan and temporal REST routes and explicitly shows that execute is absent, followed by a directional adapter-to-core-to-domain flow.',
    lab: 'Three panes show planner inspection, temporal diagnostics and an inactive reserved ML and dataset surface, separated from the official site.'
  };
</script>

<svelte:head>
  <title>{section.eyebrow} · Nexus Cerebri</title>
  <meta name="description" content={section.summary} />
</svelte:head>

<section class="page-hero" data-section={data.key}>
  <div class="page-nebula" aria-hidden="true"></div>
  <div class="page-planet" aria-hidden="true"><span></span><i></i></div>
  <div class="page-orbit" aria-hidden="true"><i></i><b></b><span></span></div>
  <div class="page-hero-copy">
    <div class="hero-meta"><StatusBadge status={section.status} /><span>{section.truth}</span><span>FIELD {sectionNumber}</span></div>
    <p class="kicker">NEXUS CEREBRI / {section.eyebrow}</p>
    <h1>{section.title}</h1>
    <p class="page-summary">{section.summary}</p>
    <div class="page-source-row"><SourceLink path={section.sourcePath} label={section.sourceLabel} /></div>
  </div>
  <aside class="principle-list" aria-label="Key boundaries">
    <header><small>BOUNDARY CONDITIONS</small><span>{sectionNumber} / 03</span></header>
    {#each section.bullets as bullet, index}
      <div><span>{String(index + 1).padStart(2, '0')}</span><p>{bullet}</p><i aria-hidden="true"></i></div>
    {/each}
  </aside>
  <div class="depth-technical depth-panel">
    <small>TECHNICAL DEPTH / CONTRACTS</small>
    <p>{section.technical}</p>
  </div>
  <div class="depth-research depth-panel research">
    <small>RESEARCH DEPTH / BOUNDARY</small>
    <p>{section.research}</p>
  </div>
  <div class="page-hero-index" aria-hidden="true"><span>{sectionNumber}</span><small>NEXUS FIELD</small></div>
</section>

<SectionPrimer kind={data.key} />

<section class="section-visual-wrap" data-section={data.key}>
  <div class="section-transition-orbit" aria-hidden="true"><span></span><i></i><b></b></div>
  <VisualizationFrame
    kind={section.truth}
    label={section.eyebrow}
    caption={section.truth === 'REAL'
      ? 'Built from current repository facts or Rust-generated fixture output.'
      : section.truth === 'EDUCATIONAL'
        ? 'An explanatory diagram of documented architecture; not a live runtime trace.'
        : 'A future-facing concept that is not implemented.'}
    textAlternative={textAlternatives[section.visual]}
  >
    <SystemVisual visual={section.visual} />
  </VisualizationFrame>
</section>

{#if data.key === 'lab'}
  <section class="callout">
    <div class="callout-planet" aria-hidden="true"></div>
    <div class="callout-index">BOUNDARY / LAB</div>
    <div>
      <span class="kicker">SEPARATE DEVELOPER SURFACE</span>
      <h2>Lab stays independent.</h2>
      <p>The official site explains the Lab boundary but does not absorb its API-driven inspection tools.</p>
    </div>
    <a class="secondary-action" href={base + '/docs/project/'}><span>Project entry points</span><b>→</b></a>
  </section>
{:else if data.key === 'planning'}
  <section class="callout">
    <div class="callout-planet" aria-hidden="true"></div>
    <div class="callout-index">CORE / RESULT</div>
    <div>
      <span class="kicker">CURRENT CORE RESULT</span>
      <h2>{(runtimeData.plannerResult as any).assessment ?? 'Structured planner assessment'}</h2>
      <p>The value above is generated by running the Rust core example during the website build.</p>
    </div>
    <a class="secondary-action" href={base + '/docs/architecture/decisions/ADR-0006-search-transports/'}><span>Read ADR-0006</span><b>→</b></a>
  </section>
{/if}
