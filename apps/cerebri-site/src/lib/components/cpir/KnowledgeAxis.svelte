<script lang="ts">
  import { request } from '$lib/planning-display';
  let { precise = false }: { precise?: boolean } = $props();
</script>

<section class="knowledge-axis" aria-labelledby="knowledge-axis-title">
  <header><small>02 / CONTEXT</small><h3 id="knowledge-axis-title">Knowledge has two coordinates.</h3></header>
  <div class="axis-matrix">
    <span class="epistemic-axis">KNOWLEDGE ↑</span>
    <span class="processing-axis">PROCESSING →</span>
    <span class="unresolved-column">UNRESOLVED</span><span class="resolved-column">RESOLVED</span>
    <ol aria-label="Canonical temporal knowledge">
      {#each request.context.objects as object, index}
        <li data-object-id={object.id} data-processing={object.time.value.processing} data-knowledge={object.time.value.knowledge.state} class:known={object.time.value.knowledge.state === 'KNOWN'}>
          <span class="state-coordinate">{object.time.value.knowledge.state}</span>
          <span class="axis-point" aria-hidden="true">{object.time.value.knowledge.state === 'KNOWN' ? '●' : '◇'}</span>
          <div><strong>{object.id}</strong><small>{object.time.value.processing} / {object.time.value.knowledge.state}</small></div>
          <dl class:hidden-fields={!precise}>
            <div><dt><code>context.objects[{index}].time.value.processing</code></dt><dd>{object.time.value.processing}</dd></div>
            <div><dt><code>context.objects[{index}].time.value.knowledge.state</code></dt><dd>{object.time.value.knowledge.state}</dd></div>
            <div><dt><code>context.objects[{index}].time.provenance</code></dt><dd>{object.time.provenance}</dd></div>
          </dl>
        </li>
      {/each}
    </ol>
  </div>
  <p>MISSING is resolved absence of temporal knowledge. The empty UNRESOLVED column is an axis label, not another fixture object.</p>
</section>

<style>
  header { margin-bottom:26px; } h3 { margin-top:8px; }
  .axis-matrix { position:relative; padding:62px 0 25px 55px; border-bottom:1px solid var(--line); }
  .axis-matrix::before { content:''; position:absolute; top:25px; bottom:25px; left:55px; border-left:1px solid var(--line); }
  .epistemic-axis { position:absolute; left:0; top:90px; writing-mode:vertical-rl; transform:rotate(180deg); font:10px var(--mono); color:#b5c5d8; }
  .processing-axis { position:absolute; bottom:0; left:72px; font:10px var(--mono); color:#b5c5d8; }
  .unresolved-column,.resolved-column { position:absolute; top:18px; font:10px var(--mono); color:#b5c5d8; }
  .unresolved-column { left:66px; } .resolved-column { right:8px; color:#e2f2f5; }
  ol { list-style:none; padding:0; margin:0; display:flex; flex-direction:column-reverse; gap:26px; }
  li { position:relative; padding:18px 0 0 15px; border-top:1px dashed var(--line); }
  .state-coordinate { position:absolute; top:-10px; left:12px; background:#071326; padding:0 7px; color:#eed4a1; font:11px var(--mono); }
  .known .state-coordinate { color:#efafb7; }
  .axis-point { position:absolute; right:25px; top:-16px; color:#eed4a1; font-size:22px; }
  .known .axis-point { color:#efafb7; }
  strong { display:block; font:13px var(--mono); margin-bottom:6px; }
  li small { display:block; }
  dl { margin-top:16px!important; padding-left:10px; border-left:1px solid var(--line); }
  dl>div { margin:9px 0; } dt code { font-size:10px!important; } dd { font:11px/1.7 var(--mono); }
  .hidden-fields { display:none; }
  p { margin:25px 0 0; font-size:12px; }
</style>
