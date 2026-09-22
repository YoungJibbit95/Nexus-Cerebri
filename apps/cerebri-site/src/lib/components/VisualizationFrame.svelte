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

<figure class="visual-frame" data-truth={kind}>
  <div class="visual-space" aria-hidden="true"><i></i><i></i><i></i><span></span></div>
  <div class="visual-heading">
    <div><span class="truth-label" data-kind={kind}>{kind}</span><strong>{label}</strong></div>
    <span class="visual-coordinate" aria-hidden="true">NEXUS CEREBRI / SYSTEM VIEW</span>
  </div>
  <div class="visual-body">{@render children()}</div>
  <div class="visual-foot">
    <details class="visual-alt"><summary>Text alternative</summary><p>{textAlternative}</p></details>
    <figcaption>{caption}</figcaption>
  </div>
</figure>
