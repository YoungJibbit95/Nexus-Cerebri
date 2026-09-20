<script lang="ts">
  import '../app.css';
  import { base } from '$app/paths';
  import DepthControl from '$lib/components/DepthControl.svelte';
  import { primaryNav } from '$lib/site';

  let { children } = $props();
  let menuOpen = $state(false);
  const href = (path: string) => base + path;
</script>

<a class="skip-link" href="#main-content">Skip to content</a>

<header class="site-header">
  <a class="brand" href={href('/')} aria-label="Nexus Cerebri home">
    <img src={href('/nexus-cerebri-logo.png')} alt="" />
    <span><strong>Nexus Cerebri</strong><small>COMPUTATIONAL COORDINATE FIELD</small></span>
  </a>
  <button
    class="nav-toggle"
    aria-expanded={menuOpen}
    aria-controls="primary-navigation"
    onclick={() => (menuOpen = !menuOpen)}
  >Menu</button>
  <nav id="primary-navigation" class:open={menuOpen} aria-label="Primary">
    {#each primaryNav as item}
      <a href={href(item.href)} onclick={() => (menuOpen = false)}>{item.label}</a>
    {/each}
    <a class="lab-link" href={href('/lab/')} onclick={() => (menuOpen = false)}>Lab</a>
  </nav>
  <DepthControl />
</header>

<main id="main-content">
  {@render children()}
</main>

<footer class="site-footer">
  <div>
    <strong>Neural intuition. Symbolic verification.</strong>
    <span>Facts, constraints, permissions and proofs remain explicit.</span>
  </div>
  <div class="footer-links">
    <a href={href('/docs/project/')}>Project</a>
    <a href={href('/docs/changelog/')}>Changelog</a>
    <a href="https://github.com/YoungJibbit95/Nexus-Cerebri">GitHub</a>
  </div>
</footer>
