<script lang="ts">
  import { base } from '$app/paths';
  let { data } = $props();

  const groups = [
    ['Architecture', 'architecture/', 'Specifications, decisions and system boundaries'],
    ['Development', 'development/', 'Roadmaps, implementation notes and contributor guidance'],
    ['Testing', 'testing/', 'Verification strategy, fixtures and quality gates'],
    ['English reference', 'en/', 'Reference material in English'],
    ['Deutsch reference', 'de/', 'Referenzmaterial auf Deutsch'],
    ['Archive', 'archive/', 'Historical material kept for traceability']
  ] as const;

  const groupDocs = (prefix: string) => data.docs.filter((doc: any) => doc.sourcePath.startsWith('docs/' + prefix));
  const rootDocs = $derived(data.docs.filter((doc: any) => !doc.sourcePath.startsWith('docs/')));
</script>

<svelte:head>
  <title>Canonical documentation · Nexus Cerebri</title>
  <meta name="description" content="Repository-native Nexus Cerebri documentation rendered from canonical Markdown." />
</svelte:head>

<section class="docs-hero">
  <div class="docs-orbit" aria-hidden="true"><i></i><span></span><b></b></div>
  <span class="truth-label" data-kind="REAL">REAL</span>
  <p class="kicker">NEXUS CEREBRI / REPOSITORY-NATIVE KNOWLEDGE</p>
  <h1>Canonical docs.<br /><em>One source of truth.</em></h1>
  <p>These pages are generated from the Markdown already reviewed in the repository. The website adds navigation and presentation, not an alternate specification corpus.</p>
</section>

<section class="doc-groups">
  <article class="doc-group doc-group-primary">
    <header><div><small>START HERE</small><h2>Project entry points</h2><p>High-signal repository documents for understanding Cerebri before following narrower references.</p></div><span>{rootDocs.length} docs</span></header>
    {#each rootDocs as doc}
      <a href={base + '/docs/' + doc.slug + '/'}><span>{doc.title}</span><small>{doc.sourcePath}</small><b aria-hidden="true">→</b></a>
    {/each}
  </article>
  {#each groups as [label, prefix, description]}
    <article class="doc-group">
      <header><div><small>COLLECTION</small><h2>{label}</h2><p>{description}</p></div><span>{groupDocs(prefix).length} docs</span></header>
      {#each groupDocs(prefix) as doc}
        <a href={base + '/docs/' + doc.slug + '/'}><span>{doc.title}</span><small>{doc.sourcePath}</small><b aria-hidden="true">→</b></a>
      {/each}
    </article>
  {/each}
</section>
