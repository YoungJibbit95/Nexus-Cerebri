<script lang="ts">
  import type { Snippet } from 'svelte';
  let { title, open = false, children }: { title: string; open?: boolean; children: Snippet } = $props();
</script>
<details class="inspector-drawer" {open}><summary>{title}<span>INSPECT ↓</span></summary><div class="inspector-content">{@render children()}</div></details>
