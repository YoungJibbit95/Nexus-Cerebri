<script lang="ts">
  import type { SemanticKind } from '$lib/visual-grammar';

  let {
    kind,
    label,
    detail = '',
    compact = false
  }: {
    kind: SemanticKind;
    label: string;
    detail?: string;
    compact?: boolean;
  } = $props();

  const codes: Record<SemanticKind, string> = {
    fact: 'F',
    constraint: '!',
    preference: 'P',
    scope: 'S',
    candidate: 'C',
    violation: '×',
    result: 'R',
    authority: 'A',
    research: '↗'
  };
</script>

<span class:compact class="semantic-mark" data-semantic-kind={kind}>
  <span class="semantic-glyph" aria-hidden="true"><i></i><b>{codes[kind]}</b></span>
  <span class="semantic-mark-copy">
    <strong>{label}</strong>
    {#if detail}<small>{detail}</small>{/if}
  </span>
</span>
