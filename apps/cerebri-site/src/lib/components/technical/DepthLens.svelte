<script lang="ts">
  import { onMount } from 'svelte';

  let { value = $bindable(0), architecture = false }: { value?: number; architecture?: boolean } = $props();
  let reduced = $state(false);
  let local = $state(false);
  const stages = $derived(architecture
    ? [{ value: 0, label: 'Compressed' }, { value: 2, label: 'Exploded ownership' }]
    : [{ value: 0, label: 'Concept' }, { value: 1, label: 'Exact fields' }, { value: 2, label: 'Structured evidence' }]);

  function followDepth() {
    local = false;
    value = reduced || ['technical', 'research'].includes(document.documentElement.dataset.depth ?? '') ? 2 : 0;
  }
  onMount(() => {
    const media = matchMedia('(prefers-reduced-motion: reduce)');
    const sync = () => { reduced = media.matches; followDepth(); };
    const observer = new MutationObserver(followDepth);
    observer.observe(document.documentElement, { attributes: true, attributeFilter: ['data-depth'] });
    media.addEventListener('change', sync);
    sync();
    return () => { observer.disconnect(); media.removeEventListener('change', sync); };
  });
</script>

<div class="depth-lens">
  <div class="lens-caption"><span aria-hidden="true">⌜</span><div><small>TECHNICAL DEPTH LENS</small><p>{reduced ? 'Direct correspondence · reduced motion' : local ? 'Local inspection · same evidence' : 'Following explanation depth'}</p></div></div>
  <div class="lens-controls" role="group" aria-label={architecture ? 'Architecture precision' : 'Evidence precision'}>
    {#each stages as stage}
      <button aria-pressed={value === stage.value} disabled={reduced && stage.value !== 2} onclick={() => { local = true; value = stage.value; }}>{stage.label}</button>
    {/each}
    <button class="lens-reset" onclick={followDepth}>Follow depth</button>
  </div>
</div>
