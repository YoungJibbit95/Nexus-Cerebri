<script lang="ts">
  import { base } from '$app/paths';
  let { data } = $props();

  const groups = [
    ['Architecture', 'architecture/'],
    ['Development', 'development/'],
    ['Testing', 'testing/'],
    ['English reference', 'en/'],
    ['Deutsch reference', 'de/'],
    ['Archive', 'archive/']
  ] as const;

  const groupDocs = (prefix: string) => data.docs.filter((doc: any) => doc.sourcePath.startsWith('docs/' + prefix));
  const rootDocs = $derived(data.docs.filter((doc: any) => !doc.sourcePath.startsWith('docs/')));
</script>

<svelte:head>
  <title>Canonical documentation · Nexus Cerebri</title>
  <meta name="description" content="Repository-native Nexus Cerebri documentation rendered from canonical Markdown." />
</svelte:head>

<section class="docs-hero">
  <span class="truth-label" data-kind="REAL">REAL</span>
  <p class="kicker">REPOSITORY-NATIVE KNOWLEDGE</p>
  <h1>Canonical docs, rendered without a second truth.</h1>
  <p>These pages are generated from the Markdown already reviewed in the repository. The website adds navigation and presentation, not an alternate specification corpus.</p>
</section>

<section class="doc-groups">
  <article class="doc-group">
    <h2>Project entry points</h2>
    {#each rootDocs as doc}
      <a href={base + '/docs/' + doc.slug + '/'}><span>{doc.title}</span><small>{doc.sourcePath}</small></a>
    {/each}
  </article>
  {#each groups as [label, prefix]}
    <article class="doc-group">
      <h2>{label}</h2>
      {#each groupDocs(prefix) as doc}
        <a href={base + '/docs/' + doc.slug + '/'}><span>{doc.title}</span><small>{doc.sourcePath}</small></a>
      {/each}
    </article>
  {/each}
</section>
