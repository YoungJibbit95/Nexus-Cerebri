<script lang="ts">
  let { title = 'Source data', value, open = false }: { title?: string; value: unknown; open?: boolean } = $props();
</script>
<details class="json-panel" {open}><summary>{title}<span>JSON</span></summary><pre>{JSON.stringify(value, null, 2)}</pre></details>
