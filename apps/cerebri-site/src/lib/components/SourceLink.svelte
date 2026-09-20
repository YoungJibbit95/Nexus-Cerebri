<script lang="ts">
  let { path, label }: { path: string; label: string } = $props();
  const href = $derived('https://github.com/YoungJibbit95/Nexus-Cerebri/blob/main/' + path);
</script>

<a class="source-link" href={href} target="_blank" rel="noreferrer">
  <span>Source</span><strong>{label}</strong><span aria-hidden="true">↗</span>
</a>
