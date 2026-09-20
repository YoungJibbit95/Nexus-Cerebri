<script lang="ts">
  import Icon from './Icon.svelte';
  import type { LabView } from '../lib/contracts.ts';
  let { view, onselect, api, version }: { view: LabView; onselect: (view: LabView) => void; api: string; version: string } = $props();
  const views: LabView[] = ['Planner', 'Temporal', 'Trace', 'Semantics', 'Preferences', 'ML', 'Dataset'];
</script>

<aside class="sidebar">
  <a class="brand" href="#workspace" aria-label="Cerebri Lab workspace">
    <span class="brand-mark" aria-hidden="true">N</span>
    <span><strong>Cerebri<span class="brand-lab">Lab</span></strong><small>NEXUS / RESEARCH WORKSPACE</small></span>
  </a>
  <div class="sidebar-label">EXPLORE</div>
  <nav aria-label="Lab views">
    {#each views as item, index}
      <button class:active={view === item} aria-label={item} aria-current={view === item ? 'page' : undefined} onclick={() => onselect(item)}>
        <Icon name={item === 'Temporal' ? 'clock' : item.toLowerCase()} /><span>{item}</span>
        {#if item === 'ML' || item === 'Dataset'}<span class="nav-status">LATER</span>{:else}<span class="nav-number">0{index + 1}</span>{/if}
      </button>
    {/each}
  </nav>
  <div class="sidebar-note"><Icon name="shield" size={23} /><strong>Observe. Understand. Verify.</strong><p>Every proposal stays inspectable. Only the Rust core decides what is valid.</p><span class="mini-badge">INTERNAL LAB</span></div>
  <div class="sidebar-bottom"><span class="connection"><span class:online={api === 'Connected'} class="status-dot"></span>{api}</span><small>Rust core {version || '· local API'}</small><span class="workspace-name">NEXUS CEREBRI <span>↗</span></span></div>
</aside>
