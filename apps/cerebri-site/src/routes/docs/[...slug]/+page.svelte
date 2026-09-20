<script lang="ts">
  import { base } from '$app/paths';
  let { data } = $props();
  const html = $derived(data.html.replaceAll('__BASE__', base));
</script>

<svelte:head>
  <title>{data.title} · Nexus Cerebri Docs</title>
</svelte:head>

<div class="document-shell" lang={data.language}>
  <aside class="document-meta">
    <span class="truth-label" data-kind="REAL">REAL</span>
    <p class="kicker">CANONICAL MARKDOWN</p>
    <strong>{data.title}</strong>
    <span>{data.sourcePath}</span>
    {#if data.status}<span>{data.status}</span>{/if}
    <a href={data.sourceUrl} target="_blank" rel="noreferrer">Open source ↗</a>
    <a href={base + '/docs/'}>← Documentation map</a>
  </aside>
  <article class="document">
    {@html html}
  </article>
</div>
