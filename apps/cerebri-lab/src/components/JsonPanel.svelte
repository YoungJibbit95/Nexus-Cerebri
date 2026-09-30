<script lang="ts">
  import { getContext } from 'svelte';
  import { DEPTH_CONTEXT } from '../lib/depth.ts';
  import type { ExplanationMode } from '../lib/contracts.ts';
  let { title = 'Source data', value, open = false }: { title?: string; value: unknown; open?: boolean } = $props();
  const depth = getContext<() => ExplanationMode>(DEPTH_CONTEXT);
  const mode = $derived(depth?.() ?? 'Technical');
</script>
{#if mode !== 'Simple'}{#key mode}<details class="json-panel" open={open || mode === 'Research'}><summary>{title}<span>JSON</span></summary><pre>{JSON.stringify(value, null, 2)}</pre></details>{/key}{/if}
