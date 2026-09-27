<script lang="ts">
  import { timeLabel } from '$lib/planning-display';
  let { label, path, start, end, precision = 0, kind = 'scope' }: {
    label: string; path: string; start: string; end: string; precision?: number; kind?: 'scope' | 'fact' | 'candidate';
  } = $props();
</script>

<div class="range-field" data-field={path} data-precision={precision} data-range-kind={kind}>
  <div class="range-heading"><strong>{label}</strong><code>{path}</code></div>
  <div class="range-geometry">
    <span class="range-connector" aria-hidden="true"></span>
    <dl class="range-endpoints">
      {#each [{ name: 'start', value: start }, { name: 'end', value: end }] as endpoint (endpoint.name)}
        <div class={'range-endpoint ' + endpoint.name} data-field={path + '.' + endpoint.name} data-value={endpoint.value}>
          <i aria-hidden="true"></i>
          <dt><span class="sr-only">{path}.</span><code>{endpoint.name}</code></dt>
          <dd><span class="range-time">{timeLabel(endpoint.value)} <small>UTC</small></span><time datetime={endpoint.value}>{endpoint.value}</time></dd>
        </div>
      {/each}
    </dl>
  </div>
</div>

<style>
  .sr-only{position:absolute;width:1px;height:1px;overflow:hidden;clip-path:inset(50%);white-space:nowrap}
</style>
