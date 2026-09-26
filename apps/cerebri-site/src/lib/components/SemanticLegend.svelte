<script lang="ts">
  import SemanticMark from './SemanticMark.svelte';
  import type { SemanticLegendItem } from '$lib/visual-grammar';

  let {
    items,
    label = 'Semantic visual legend',
    compact = false
  }: {
    items: SemanticLegendItem[];
    label?: string;
    compact?: boolean;
  } = $props();
</script>

<div class:compact class="semantic-legend" role="list" aria-label={label}>
  {#each items as item}
    <span role="listitem"><SemanticMark kind={item.kind} label={item.label} detail={item.detail} compact={compact} /></span>
  {/each}
</div>
