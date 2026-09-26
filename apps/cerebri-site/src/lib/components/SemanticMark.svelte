<script lang="ts">
  import type { SemanticKind } from '$lib/visual-grammar';

  let { kind, label, detail = '', compact = false }: {
    kind: SemanticKind; label: string; detail?: string; compact?: boolean;
  } = $props();

  const codes: Record<SemanticKind, string> = {
    fact: 'F', constraint: '!', preference: 'P', scope: 'S', candidate: 'C',
    violation: '×', result: 'R', authority: 'A', research: '↗'
  };
</script>

<span class:compact class="semantic-mark" data-semantic-kind={kind}>
  <span class="semantic-glyph" aria-hidden="true"><i></i><b>{codes[kind]}</b></span>
  <span class="semantic-mark-copy">
    <strong>{label}</strong>
    {#if detail}<small>{detail}</small>{/if}
  </span>
</span>

<style>
  .semantic-mark{--semantic:#72a9db;--semantic-soft:rgba(114,169,219,.13);display:inline-flex;align-items:center;gap:10px;min-width:0;color:#c9d9ea}
  .semantic-glyph{position:relative;flex:0 0 34px;width:34px;height:34px;display:grid;place-items:center}
  .semantic-glyph::before{content:"";position:absolute;inset:3px;border:1px solid color-mix(in srgb,var(--semantic) 62%,transparent);background:var(--semantic-soft);box-shadow:inset 0 0 16px color-mix(in srgb,var(--semantic) 10%,transparent),0 0 18px color-mix(in srgb,var(--semantic) 8%,transparent)}
  .semantic-glyph i{position:absolute;inset:0;opacity:.35;border:1px solid color-mix(in srgb,var(--semantic) 35%,transparent);border-radius:50%}
  .semantic-glyph b{position:relative;z-index:1;color:var(--semantic);font:800 11px/1 var(--mono)}
  .semantic-mark-copy{min-width:0;display:grid;gap:2px}.semantic-mark-copy strong{color:#dce8f5;font-size:12px;line-height:1.22}.semantic-mark-copy small{color:#7189a7;font-size:10px;line-height:1.28}
  .compact{gap:7px}.compact .semantic-glyph{flex-basis:28px;width:28px;height:28px}.compact .semantic-mark-copy strong{font-size:11px}.compact .semantic-mark-copy small{font-size:9px}
  [data-semantic-kind="fact"]{--semantic:#42e8e0;--semantic-soft:rgba(66,232,224,.11)}[data-semantic-kind="fact"] .semantic-glyph::before{border-radius:50%}
  [data-semantic-kind="constraint"]{--semantic:#ff8795;--semantic-soft:rgba(255,135,149,.11)}[data-semantic-kind="constraint"] .semantic-glyph::before{clip-path:polygon(30% 0,70% 0,100% 30%,100% 70%,70% 100%,30% 100%,0 70%,0 30%)}
  [data-semantic-kind="preference"]{--semantic:#b9a0ff;--semantic-soft:rgba(185,160,255,.11)}[data-semantic-kind="preference"] .semantic-glyph::before{inset:7px;transform:rotate(45deg)}
  [data-semantic-kind="scope"]{--semantic:#4aa5ff;--semantic-soft:rgba(74,165,255,.09)}[data-semantic-kind="scope"] .semantic-glyph::before{inset:4px 7px;border-width:1px 0;border-radius:0;background:linear-gradient(90deg,var(--semantic) 0 1px,transparent 1px calc(100% - 1px),var(--semantic) calc(100% - 1px))}
  [data-semantic-kind="candidate"]{--semantic:#6f9fce;--semantic-soft:rgba(111,159,206,.1)}[data-semantic-kind="candidate"] .semantic-glyph::before{inset:7px 2px;border-radius:999px}
  [data-semantic-kind="violation"]{--semantic:#ff8795;--semantic-soft:rgba(255,135,149,.07)}[data-semantic-kind="violation"] .semantic-glyph::before{border-radius:50%;border-style:dashed}
  [data-semantic-kind="result"]{--semantic:#20d8ff;--semantic-soft:rgba(32,216,255,.14)}[data-semantic-kind="result"] .semantic-glyph::before{inset:5px;border-radius:50%;box-shadow:0 0 0 3px rgba(32,216,255,.06),0 0 22px rgba(32,216,255,.18)}
  [data-semantic-kind="authority"]{--semantic:#60e6b8;--semantic-soft:rgba(96,230,184,.1)}[data-semantic-kind="authority"] .semantic-glyph::before{clip-path:polygon(25% 4%,75% 4%,96% 50%,75% 96%,25% 96%,4% 50%)}
  [data-semantic-kind="research"]{--semantic:#a147ff;--semantic-soft:rgba(161,71,255,.09)}[data-semantic-kind="research"] .semantic-glyph::before{border-radius:50%;border-style:dashed}
  @media(prefers-reduced-motion:reduce){.semantic-mark{transition:none!important;animation:none!important}}
</style>