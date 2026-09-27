<script lang="ts">
  import './cerebri-explainer.css';
  import { base } from '$app/paths';
</script>

<section class="cerebri-explainer cosmic-section" aria-labelledby="cerebri-explainer-title">

  <div class="explainer-copy">
    <span class="section-index">HOW PLANNING WORKS</span>
    <p class="kicker">THE INPUT COMES FIRST</p>
    <h2 id="cerebri-explainer-title">What does the<br /><em>planner need?</em></h2>
    <p class="explainer-lede">
      Suppose you need 30 minutes between 09:00 and 12:00, and 09:00–10:00 is already occupied.
      Cerebri needs those details in separate data fields, along with the rules it must follow.
      This input format is called CPIR. The current planner reads these fields, not the sentence above.
    </p>

    <div class="principle-stack" aria-label="Three current planner principles">
      <article>
        <span>01</span>
        <div><strong>Record the known appointments</strong><p>Existing appointments tell the planner which times are occupied. Missing availability information stays unknown.</p></div>
      </article>
      <article>
        <span>02</span>
        <div><strong>Rule out conflicts</strong><p>A possible appointment must fit inside the requested window and pass the required checks, including the check for overlaps.</p></div>
      </article>
      <article>
        <span>03</span>
        <div><strong>Compare the remaining options</strong><p>Preferences and fixed comparison rules put valid options in order. The same input gives the same result: this is deterministic planning.</p></div>
      </article>
    </div>

    <div class="explainer-reading depth-technical">
      <small>TECHNICAL READING</small>
      <p>CPIR records time, scope, facts, rules, preferences, their sources and planning permissions separately. Rust validates the input, checks possible placements and compares valid ones by preferred-start distance, number of changes, time shifted, start time and object ID.</p>
    </div>
    <div class="explainer-reading depth-research research">
      <small>RESEARCH BOUNDARY</small>
      <p>A later learning component could help interpret requests or compare valid options. None runs in the current planner. Required rules and permission checks would still apply.</p>
    </div>
  </div>

  <div class="explainer-stage" aria-label="From appointment details to a checked proposal">
    <div class="stage-caption stage-caption-left"><small>APPOINTMENT DETAILS</small><span>existing appointments · time window · rules · wishes</span></div>
    <div class="interpretation-cloud" aria-hidden="true">
      <i class="cloud-node c1"></i><i class="cloud-node c2"></i><i class="cloud-node c3"></i><i class="cloud-node c4"></i>
      <span class="cloud-path p1"></span><span class="cloud-path p2"></span><span class="cloud-path p3"></span>
    </div>

    <div class="stage-annotation annotation-a"><b>1</b><span><strong>The input is structured.</strong><small>Duration, occupied times and rules arrive in separate fields.</small></span></div>

    <div class="boundary-shell">
      <span class="boundary-label bl-time">TIME</span>
      <span class="boundary-label bl-scope">SCOPE</span>
      <span class="boundary-label bl-policy">POLICY</span>
      <span class="boundary-label bl-knowledge">FACTS</span>
      <span class="boundary-label bl-constraints">HARD RULES</span>
    </div>

    <div class="stage-annotation annotation-b"><b>2</b><span><strong>Conflicting times are rejected.</strong><small>A preferred time must still pass the overlap and other required checks.</small></span></div>

    <div class="deterministic-core">
      <div class="core-title"><small>DETERMINISTIC CORE</small><strong>Generate → Validate → Order</strong></div>
      <div class="core-candidates" aria-hidden="true"><span></span><span></span><span class="selected"></span><span></span></div>
      <div class="core-proof"><b>✓</b><span>Result and search coverage</span></div>
    </div>

    <div class="stage-annotation annotation-c"><b>3</b><span><strong>The remaining options are compared.</strong><small>The result says which option comes first and how much of the search was completed.</small></span></div>

    <div class="authority-rail">
      <small>SEPARATE CHECKS BEFORE EXECUTION</small>
      <div class="authority-steps">
        <span><i></i><b>Proposed</b></span><span><i></i><b>Validated</b></span><span><i></i><b>Authorized</b></span><span><i></i><b>Executed</b></span>
      </div>
      <p class="technical-only">PlanningRequest → ProposedPlan → ValidatedPlan → ActionPlan → AuthorizedActionPlan → ExecutionResult</p>
    </div>

  </div>

  <div class="explainer-panels" aria-label="Cerebri planner concepts">
    <article><span class="mini-cue cue-time" aria-hidden="true"><i></i></span><div><small>KNOWN APPOINTMENTS</small><strong>Which times are occupied?</strong><p>The supplied appointment from 09:00 to 10:00 blocks every option that overlaps it.</p></div></article>
    <article><span class="mini-cue cue-evidence" aria-hidden="true"><i></i><i></i><i></i></span><div><small>REQUIRED RULES</small><strong>Which options must be rejected?</strong><p>The new appointment must fit in the time window and avoid conflicts. Wishes cannot override those rules.</p></div></article>
    <article><span class="mini-cue cue-search" aria-hidden="true"><i></i></span><div><small>PREFERENCES</small><strong>Which valid option comes first?</strong><p>A preferred start can influence the order. Fixed comparison rules resolve any remaining ties.</p></div></article>
    <article><span class="mini-cue cue-authority" aria-hidden="true"><i></i><b></b></span><div><small>EXECUTION</small><strong>Has anything been booked?</strong><p>No. A proposal still needs full validation and authorization before an application may execute it.</p></div></article>
  </div>

  <div class="explainer-next">
    <span>Next: see which of the possible start times remain.</span>
    <a href={base + '/planning/'}>Inspect planning <b>→</b></a>
  </div>
</section>
