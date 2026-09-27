<script lang="ts">
  import type { Snippet } from 'svelte';
  import CerebriHeroScene from './cerebri/CerebriHeroScene.svelte';
  let { children }: { children?: Snippet } = $props();
</script>

<CerebriHeroScene {children} />
