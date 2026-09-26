<script lang="ts">
  let { kind }: { kind: string } = $props();

  const primers: Record<string, {
    eyebrow: string; title: string; plain: string; steps: Array<[string,string,string]>; technical: string;
  }> = {
    cpir: {
      eyebrow: 'STRUCTURED PLANNING INPUT',
      title: 'CPIR keeps the evidence for planning explicit.',
      plain: 'It records what is being requested, what knowledge is available, which scope is in play, which policy applies, what capability exists and how much search is permitted.',
      steps: [
        ['01','Identify the requested operation','Request identity'],
        ['02','Carry known state and provenance','Knowledge + provenance'],
        ['03','Bound what may be considered','Scope boundary'],
        ['04','Keep rules and capability separate','Policy + capability'],
        ['05','Declare the search budget','Search budget']
      ],
      technical: 'Omitted, empty and populated scope filters retain different semantics. Epistemic knowledge state is distinct from processing state, and capability does not silently expand scope.'
    },
    planning: {
      eyebrow: 'BOUNDED DETERMINISTIC SEARCH',
      title: 'The planner searches a declared grid and reports what it actually established.',
      plain: 'Known busy state and hard rules remove invalid positions. The remaining candidates are ordered deterministically, and the search assessment says whether the declared grid was exhausted.',
      steps: [
        ['01','Declare the searchable window','Scope + grid'],
        ['02','Compile known blocking state','Context snapshot'],
        ['03','Evaluate discrete positions','Candidate search'],
        ['04','Reject invalid candidates','Hard validity'],
        ['05','Order valid candidates','Result + assessment']
      ],
      technical: 'The browser renders generated Rust results. PROVEN_OPTIMAL applies only to the declared discrete grid after full traversal with a solution; candidate limits reduce the assessment to BEST_FOUND.'
    },
    time: {
      eyebrow: 'TYPED TEMPORAL STATE',
      title: 'A timestamp is not enough to describe time safely.',
      plain: 'Cerebri keeps instants, zones, half-open intervals and DST ambiguity explicit so planning does not silently guess what a local clock value means.',
      steps: [['01','Intervals have exact boundary semantics','[start, end)'],['02','A local time can fail to exist','DST gap'],['03','A local time can map to two instants','DST fold']],
      technical: 'Free-time complements are only known under complete temporal coverage. Gap/fold policy is explicit rather than inferred by the UI.'
    },
    safety: {
      eyebrow: 'EXPLICIT AUTHORITY',
      title: 'A valid plan is not automatically a permitted action.',
      plain: 'Cerebri increases authority in explicit lifecycle stages. Each promotion carries stronger guarantees, and execution remains behind a separate boundary.',
      steps: [['01','Plan','Create a proposal'],['02','Validate','Check domain validity'],['03','Authorize','Bind permission'],['04','Execute','Cross the adapter boundary']],
      technical: 'Only AuthorizedActionPlan can enter execution. Validation ≠ permission and Planner ≠ Executor.'
    }
  };

  const primer = $derived(primers[kind]);
</script>

{#if primer}
  <section class="section-primer" data-primer={kind} aria-label={kind}>
    <div class="primer-copy">
      <span class="kicker">{primer.eyebrow}</span>
      <h2>{primer.title}</h2>
      <p>{primer.plain}</p>
      <div class="primer-technical depth-technical"><small>TECHNICAL DETAIL</small><p>{primer.technical}</p></div>
    </div>
    <div class="primer-steps">
      {#each primer.steps as step, index}
        <article style={`--step:${index}`}>
          <span>{step[0]}</span><div><strong>{step[1]}</strong><small>{step[2]}</small></div><i aria-hidden="true"></i>
        </article>
      {/each}
    </div>
    <div class="primer-path" aria-hidden="true"><span></span><i></i><b></b></div>
  </section>
{/if}
