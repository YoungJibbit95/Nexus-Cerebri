<script lang="ts">
  import '../app.css';
  import { base } from '$app/paths';
  import { onNavigate } from '$app/navigation';
  import { page } from '$app/state';
  import { onMount } from 'svelte';
  import DepthControl from '$lib/components/DepthControl.svelte';
  import SpaceEnvironment from '$lib/components/SpaceEnvironment.svelte';
  import { primaryNav } from '$lib/site';

  let { children } = $props();
  let menuOpen = $state(false);
  let scrolled = $state(false);
  const href = (path: string) => base + path;
  const active = (path: string) => path === '/' ? page.url.pathname === base + '/' : page.url.pathname.startsWith(base + path);

  onMount(() => {
    const update = () => (scrolled = window.scrollY > 24);
    update();
    window.addEventListener('scroll', update, { passive: true });
    return () => window.removeEventListener('scroll', update);
  });

  onNavigate((navigation) => {
    const doc = document as Document & { startViewTransition?: (cb: () => Promise<void>) => unknown };
    if (!doc.startViewTransition || window.matchMedia('(prefers-reduced-motion: reduce)').matches) return;
    return new Promise<void>((resolve) => {
      doc.startViewTransition?.(async () => {
        resolve();
        await navigation.complete;
      });
    });
  });
</script>

<a class="skip-link" href="#main-content">Skip to content</a>
<SpaceEnvironment />

<header class:scrolled class="site-header">
  <a class="brand" href={href('/')} aria-label="Nexus Cerebri home">
    <span class="brand-mark"><img src={href('/nexus-cerebri-logo.png')} alt="" /></span>
    <span class="brand-copy"><strong><span>Nexus</span> Cerebri</strong><small>DETERMINISTIC PLANNING FOUNDATION</small></span>
  </a>
  <button
    class="nav-toggle"
    aria-expanded={menuOpen}
    aria-controls="primary-navigation"
    onclick={() => (menuOpen = !menuOpen)}
  ><span>Navigation</span><i aria-hidden="true"></i></button>
  <nav id="primary-navigation" class:open={menuOpen} aria-label="Primary">
    {#each primaryNav as item}
      <a class:active={active(item.href)} aria-current={active(item.href) ? 'page' : undefined} href={href(item.href)} onclick={() => (menuOpen = false)}>
        {item.label}
      </a>
    {/each}
    <a class:active={active('/lab/')} class="lab-link" aria-current={active('/lab/') ? 'page' : undefined} href={href('/lab/')} onclick={() => (menuOpen = false)}>Lab</a>
  </nav>
  <DepthControl />
</header>

<main id="main-content">
  {@render children()}
</main>

<footer class="site-footer">
  <div class="footer-orbit" aria-hidden="true"><span></span><i></i><b></b></div>
  <div class="footer-statement">
    <small>NEXUS CEREBRI / VERIFIED FOUNDATION</small>
    <strong>Neural intuition. Symbolic verification.</strong>
    <span>Facts, constraints, permissions and proofs remain explicit.</span>
  </div>
  <div class="footer-links">
    <a href={href('/docs/project/')}>Project <span>↗</span></a>
    <a href={href('/docs/changelog/')}>Changelog <span>↗</span></a>
    <a href="https://github.com/YoungJibbit95/Nexus-Cerebri">GitHub <span>↗</span></a>
  </div>
</footer>
