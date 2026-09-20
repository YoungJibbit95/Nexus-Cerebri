<script lang="ts">
  import { onMount } from 'svelte';

  type Depth = 'understand' | 'technical' | 'research';
  const options: Array<{ id: Depth; label: string; hint: string }> = [
    { id: 'understand', label: 'Understand', hint: 'Conceptual' },
    { id: 'technical', label: 'Technical', hint: 'Contracts' },
    { id: 'research', label: 'Research', hint: 'Boundaries' }
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
  {#each options as option}
    <button class:active={depth === option.id} aria-pressed={depth === option.id} onclick={() => apply(option.id)}>
      <span>{option.label}</span>
      <small>{option.hint}</small>
    </button>
  {/each}
</div>
