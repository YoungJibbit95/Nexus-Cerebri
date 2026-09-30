<script lang="ts">
  let { hasProposal = false, hasResult = false }: { hasProposal?: boolean; hasResult?: boolean } = $props();
</script>
<section class="authority-rail" aria-label="Planner authority boundary">
  <div class="authority-current"><small>LAB STOPS HERE</small><strong>{hasProposal ? 'Proposal' : hasResult ? 'No feasible proposal' : 'Awaiting proposal'}</strong><span>Planning only</span></div>
  <span class="authority-barrier" aria-hidden="true">╎</span>
  <div><small>REQUIRED NEXT</small><strong>Validation / authorization</strong><span>Current facts, policy, confirmation</span></div>
  <span aria-hidden="true">→</span>
  <div><small>NOT AVAILABLE IN LAB</small><strong>Execution</strong><span>Freshness and execution preflight</span></div>
  <p>Proposal ≠ execution. No calendar has been changed.</p>
</section>
