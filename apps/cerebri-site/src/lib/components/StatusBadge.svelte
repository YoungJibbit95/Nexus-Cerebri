<script lang="ts">
  import type { StatusKind } from '$lib/site';
  let { status }: { status: StatusKind } = $props();
</script>

<span class="status-badge" data-status={status}><i aria-hidden="true"></i>{status}</span>
