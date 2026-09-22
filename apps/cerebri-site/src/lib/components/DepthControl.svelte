<script lang="ts">
  import { onMount } from 'svelte';

  type Depth = 'understand' | 'technical' | 'research';
  const options: Array<{ id: Depth; label: string; hint: string; index: string }> = [
    { id: 'understand', label: 'Understand', hint: 'concept', index: '01' },
    { id: 'technical', label: 'Technical', hint: 'contracts', index: '02' },
    { id: 'research', label: 'Research', hint: 'boundary', index: '03' }
  ];
  let depth: Depth = 'understand';

  function apply(next: Depth) {
    depth = next;
    document.documentElement.dataset.depth = next;
    localStorage.setItem('cerebri-depth', next);
  }

  onMount(() => {
    const saved = localStorage.getItem('cerebri-depth');
    if (saved === 'understand' || saved === 'technical' || saved === 'research') apply(saved);
  });
</script>

<div class="depth-control" role="group" aria-label="Explanation depth">
  <span class="depth-label" aria-hidden="true">DEPTH</span>
  {#each options as option}
    <button class:active={depth === option.id} aria-pressed={depth === option.id} onclick={() => apply(option.id)}>
      <small>{option.index}</small>
      <span>{option.label}</span>
      <em>{option.hint}</em>
    </button>
  {/each}
</div>
