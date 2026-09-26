<script lang="ts">
  import SemanticMark from './SemanticMark.svelte';
  import type { SemanticLegendItem } from '$lib/visual-grammar';

  let { items, label = 'Semantic visual legend', compact = false }: {
    items: SemanticLegendItem[]; label?: string; compact?: boolean;
  } = $props();
</script>

<div class:compact class="semantic-legend" role="list" aria-label={label}>
  {#each items as item}
    <span role="listitem"><SemanticMark kind={item.kind} label={item.label} detail={item.detail} compact={compact} /></span>
  {/each}
</div>

<style>
  .semantic-legend{display:flex;flex-wrap:wrap;gap:10px 18px;align-items:center;padding:13px 15px;border:1px solid rgba(84,147,222,.13);border-radius:14px;background:rgba(4,15,38,.56)}
  .semantic-legend>span{min-width:0}.compact{gap:7px 13px;padding:10px 12px}
  @media(max-width:520px){.semantic-legend{gap:7px 10px;padding:9px}}
</style>