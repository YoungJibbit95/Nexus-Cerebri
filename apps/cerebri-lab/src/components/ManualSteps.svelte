<script lang="ts">
  let { labels, selected, onchange, name }: { labels: string[]; selected: number; onchange: (index: number) => void; name: string } = $props();
</script>
<nav class="manual-steps" aria-label={name}>
  {#each labels as label, index}<button aria-pressed={selected === index} class:active={selected === index} onclick={() => onchange(index)}><small>{String(index + 1).padStart(2, '0')}</small>{label}</button>{/each}
</nav>
<p class="fine-print">Choose a step to explore. The completed data remains available at every step.</p>
