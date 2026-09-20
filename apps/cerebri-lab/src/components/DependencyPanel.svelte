<script lang="ts">
  import JsonPanel from './JsonPanel.svelte';
  import type { DependencyGraph } from '../lib/contracts.ts';
  let { graph }: { graph: DependencyGraph } = $props();
  const nodes = $derived(graph.nodes.slice(0, 24));
  // Display geometry only: the core supplies all edges, cycles and ordering.
  const positions = $derived(new Map(nodes.map((id, index) => [id, { x: 115 + index % 4 * 210, y: 45 + Math.floor(index / 4) * 95 }])));
  const height = $derived(Math.max(100, Math.ceil(nodes.length / 4) * 95));
  const visibleEdges = $derived(graph.edges.filter((edge) => positions.has(edge.predecessor) && positions.has(edge.dependent)).slice(0, 64));
</script>

<section class="panel inspector-panel dependency-panel" aria-labelledby="dependency-title">
  <div class="panel-heading"><div><span class="eyebrow">CORE / DEPENDENCY GRAPH</span><h2 id="dependency-title">Explicit ordering relationships</h2></div><span class="badge subtle">{graph.nodes.length} nodes · {graph.edges.length} edges</span></div>
  <p class="panel-description">An arrow means predecessor.end ≤ dependent.start. Graph order is supplied by the core; it grants no scheduling or execution permission.</p>
  {#if nodes.length}
    <div class="dependency-scroll"><svg class="dependency-graph" viewBox={`0 0 860 ${height}`} role="img" aria-label="Dependency graph. Each directed edge is also listed in the table below.">
      <defs><marker id="dependency-arrow" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse"><path d="M 0 0 L 10 5 L 0 10 z" /></marker></defs>
      {#each visibleEdges as edge}
        {@const from = positions.get(edge.predecessor)!}
        {@const to = positions.get(edge.dependent)!}
        <path class="dependency-edge" d={edge.predecessor === edge.dependent ? `M ${from.x - 25} ${from.y - 15} C ${from.x - 85} ${from.y - 50}, ${from.x + 85} ${from.y - 50}, ${from.x + 25} ${from.y - 15}` : `M ${from.x} ${from.y + 17} Q ${(from.x + to.x) / 2} ${Math.max(from.y, to.y) + 57}, ${to.x} ${to.y + 17}`} marker-end="url(#dependency-arrow)" />
      {/each}
      {#each nodes as id}{@const position = positions.get(id)!}<g><title>{id}</title><rect x={position.x - 90} y={position.y - 17} width="180" height="34" rx="10" /><text x={position.x} y={position.y + 4} text-anchor="middle">{id.length > 23 ? `${id.slice(0, 20)}…` : id}</text></g>{/each}
    </svg></div>
  {/if}
  <h3 class="subheading">Core topological order</h3>
  {#if graph.order.length}<ol class="dependency-order">{#each graph.order as id}<li><code>{id}</code></li>{/each}</ol>
  {:else}<p class="fine-print">{graph.issues.length ? 'No complete order: the core reports graph issues.' : 'No objects in this graph.'}</p>{/if}
  {#if graph.issues.length}
    <h3 class="subheading">Graph diagnostics</h3><ul class="dependency-issues">{#each graph.issues as issue}<li>{#if issue === 'InputLimit'}Input limit exceeded.{:else if 'MissingReference' in issue}Missing reference: <code>{issue.MissingReference.predecessor} → {issue.MissingReference.dependent}</code>{:else}Cycle members: <code>{issue.Cycle.members.join(', ')}</code><br />Internal edges: {issue.Cycle.edges.map((edge) => `${edge.predecessor} → ${edge.dependent}`).join('; ')}{/if}</li>{/each}</ul>
  {/if}
  {#if graph.edges.length}<div class="table-wrap"><table><thead><tr><th>Predecessor</th><th>Dependent</th><th>Required relation</th></tr></thead><tbody>{#each graph.edges.slice(0, 64) as edge}<tr><td>{edge.predecessor}</td><td>{edge.dependent}</td><td>End ≤ start</td></tr>{/each}</tbody></table></div>{/if}
  <p class="fine-print">Diagram: first 24 nodes and 64 visible edges. Table: first 64 edges. The structured graph contains every node, edge and diagnostic.</p>
  <JsonPanel title="Dependency graph / complete evidence" value={graph} />
</section>
