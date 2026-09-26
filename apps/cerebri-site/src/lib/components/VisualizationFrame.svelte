<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { TruthKind } from '$lib/site';

  let { kind, label, caption, textAlternative, children }: {
    kind: TruthKind;
    label: string;
    caption: string;
    textAlternative: string;
    children: Snippet;
  } = $props();
</script>

<figure class="visual-frame" data-truth={kind} aria-label={label}>
  <p class="visual-alt-static">{textAlternative}</p>
  <div class="visual-space" aria-hidden="true"><i></i><i></i><i></i><span></span></div>
  <div class="visual-heading">
    <div><span class="truth-label" data-kind={kind}>{kind}</span><strong>{label}</strong></div>
    <span class="visual-coordinate" aria-hidden="true">NEXUS CEREBRI / SYSTEM VIEW</span>
  </div>
  <div class="visual-body">{@render children()}</div>
  <figcaption class="visual-foot">
    <details class="visual-alt"><summary>Text alternative</summary><p>{textAlternative}</p></details>
    <span class="visual-caption">{caption}</span>
  </figcaption>
</figure>

<style>
  .visual-alt-static{position:absolute!important;width:1px!important;height:1px!important;padding:0!important;margin:-1px!important;overflow:hidden!important;clip:rect(0,0,0,0)!important;white-space:nowrap!important;border:0!important}
</style>