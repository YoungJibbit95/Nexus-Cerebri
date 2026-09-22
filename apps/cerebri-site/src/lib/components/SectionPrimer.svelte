<script lang="ts">
  let { kind }: { kind: string } = $props();

  const primers: Record<string, {
    eyebrow: string; title: string; plain: string; steps: Array<[string,string,string]>; technical: string;
  }> = {
    cpir: {
      eyebrow: 'READ THE EVIDENCE BEFORE THE STRUCTURE',
      title: 'CPIR is the planning envelope, not “just more prompt context.”',
      plain: 'It makes the inputs to planning explicit: what is being requested, what knowledge is available, which scope is allowed, which policy applies, what capability exists and how much search is permitted.',
      steps: [
        ['01','What is being asked?','Request identity'],
        ['02','Which facts or sources count?','Knowledge + provenance'],
        ['03','Where does the allowed space end?','Scope boundary'],
        ['04','What rules and capabilities apply?','Policy + capability'],
        ['05','How much search is allowed?','Search budget']
      ],
      technical: 'Omitted, empty and populated scope filters retain different semantics. Epistemic knowledge state is distinct from processing state, and capability does not silently expand scope.'
    },
    planning: {
      eyebrow: 'HOW TO READ THE SEARCH FIELD',
      title: 'The planner searches a declared space and reports what it actually established.',
      plain: 'Think of the visualization as a bounded trajectory map: blocked regions remove options, grid positions are evaluated, feasible candidates remain, and one deterministic result is selected from the Rust output.',
      steps: [
        ['01','Declare the searchable window','Scope + grid'],
        ['02','Mark what is already unavailable','Known busy interval'],
        ['03','Evaluate discrete candidate positions','Candidate search'],
        ['04','Keep feasible results inspectable','Feasible set'],
        ['05','Select and assess the result','Outcome + assessment']
      ],
      technical: 'The browser only renders generated Rust results. PROVEN_OPTIMAL applies to the declared discrete grid after full traversal with a solution; candidate limits can reduce the assessment.'
    },
    time: {
      eyebrow: 'HOW TO READ TEMPORAL STATE',
      title: 'Time is typed because “looks like a timestamp” is not enough.',
      plain: 'Cerebri keeps instants, zones, half-open intervals and DST ambiguity explicit so planning does not silently guess what a local clock value means.',
      steps: [['01','An interval has exact boundary semantics','[start, end)'],['02','A local time can fail to exist','DST gap'],['03','A local time can map to two instants','DST fold']],
      technical: 'Free-time complements are only known under complete temporal coverage. Gap/fold policy is explicit rather than inferred by the UI.'
    },
    safety: {
      eyebrow: 'HOW TO READ AUTHORITY',
      title: 'A valid plan is not automatically a permitted action.',
      plain: 'Cerebri increases authority in explicit lifecycle stages. Each promotion carries stronger guarantees, and execution remains beyond a separate boundary.',
      steps: [['01','Plan','Create a proposal'],['02','Validate','Check domain validity'],['03','Authorize','Bind permission'],['04','Execute','Cross the provider boundary']],
      technical: 'Only AuthorizedActionPlan can enter execution. Validation ≠ permission and Planner ≠ Executor.'
    }
  };

  const primer = $derived(primers[kind]);
</script>

{#if primer}
  <section class="section-primer" data-primer={kind} aria-label={`${kind} reading guide`}>
    <div class="primer-copy">
      <span class="kicker">{primer.eyebrow}</span>
      <h2>{primer.title}</h2>
      <p>{primer.plain}</p>
      <div class="primer-technical depth-technical"><small>TECHNICAL ANCHOR</small><p>{primer.technical}</p></div>
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
