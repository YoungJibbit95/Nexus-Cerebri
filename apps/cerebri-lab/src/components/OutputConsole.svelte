<script lang="ts">
  import Icon from './Icon.svelte';
  import type { ConsoleEntry } from '../lib/contracts.ts';
  let { entries, onclear }: { entries: ConsoleEntry[]; onclear: () => void } = $props();
  let tab = $state<'planner' | 'model'>('planner');
</script>
<section class="panel console-panel" aria-labelledby="console-title">
  <div class="console-heading"><h2 id="console-title"><Icon name="terminal" size={18} />Output console</h2><div class="console-tabs" role="group" aria-label="Console source"><button class:active={tab === 'planner'} onclick={() => tab = 'planner'} aria-pressed={tab === 'planner'}>Core & Lab <span>{entries.length}</span></button><button class:active={tab === 'model'} onclick={() => tab = 'model'} aria-pressed={tab === 'model'}>Model <span>inactive</span></button></div><button class="quiet" onclick={onclear} disabled={!entries.length} aria-label="Clear console">Clear</button></div>
  <div class="console-body">
    {#if tab === 'planner'}
      {#each entries as entry (entry.id)}<div class="log-entry" class:error={entry.level === 'error'}><time datetime={entry.at}>{entry.at.slice(11, 23)}Z</time><span class="log-source">{entry.source}</span><div><p>{entry.message}</p>{#if entry.data !== undefined}<details><summary>Inspect structured output</summary><pre>{JSON.stringify(entry.data, null, 2)}</pre></details>{/if}</div><span class="log-level">{entry.level}</span></div>{:else}<p class="console-empty">Console is empty. Run or validate a request to capture its actual response.</p>{/each}
    {:else}<div class="model-idle"><Icon name="ml" size={25} /><div><strong>Model output is not connected</strong><p>Reserved for future versioned inference traces. No model is loaded and no inference is running.</p></div><span class="badge subtle">DEFERRED</span></div>{/if}
  </div>
  <div class="console-footer"><span><span class="status-dot"></span>Session memory · latest 40 entries</span><span>UTC timestamps / structured JSON</span></div>
</section>
