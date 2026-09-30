<script lang="ts">
  import type { ExplanationMode } from '../lib/contracts.ts';
  import { depths } from '../lib/depth.ts';
  let { mode, onchange }: { mode: ExplanationMode; onchange: (mode: ExplanationMode) => void } = $props();
</script>
<div class="depth-control" role="group" aria-label="Explanation depth">
  <span class="depth-caption">ONE RESULT<br /><strong>THREE DEPTHS</strong></span>
  {#each depths as depth}
    <button class:active={mode === depth.mode} aria-pressed={mode === depth.mode} onclick={() => onchange(depth.mode)}>
      <small>{depth.number}</small><span><strong>{depth.label}</strong><em>{depth.hint}</em></span>
    </button>
  {/each}
</div>
