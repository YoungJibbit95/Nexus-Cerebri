<script lang="ts">
  import { base } from '$app/paths';
</script>

<section class="cerebri-explainer cosmic-section" aria-labelledby="cerebri-explainer-title">
  <div class="explainer-transition" aria-hidden="true"><i></i><span></span><b></b></div>

  <div class="explainer-copy">
    <span class="section-index">ACT 02 / WHAT CEREBRI DOES</span>
    <p class="kicker">STRUCTURED PROBLEM → BOUNDED PLAN</p>
    <h2 id="cerebri-explainer-title">Make the problem explicit.<br /><em>Then search it.</em></h2>
    <p class="explainer-lede">
      The current planner receives structured planning state: time, known objects, scope, hard rules,
      preferences, policy and capability. It searches only the declared space, rejects invalid candidates,
      orders the valid ones deterministically and returns a proposal with an honest search assessment.
    </p>

    <div class="principle-stack" aria-label="Three current planner principles">
      <article>
        <span>01</span>
        <div><strong>Represent what is known</strong><p>Facts, temporal state, scope and provenance stay explicit instead of being hidden inside a guess.</p></div>
      </article>
      <article>
        <span>02</span>
        <div><strong>Reject what breaks hard rules</strong><p>Known busy state and hard constraints remove candidates before ranking begins.</p></div>
      </article>
      <article>
        <span>03</span>
        <div><strong>Order valid candidates deterministically</strong><p>Preferences and fixed tie-breaks compare candidates without turning planning into execution authority.</p></div>
      </article>
    </div>

    <div class="explainer-reading depth-technical">
      <small>TECHNICAL READING</small>
      <p>CPIR carries structured state into typed Rust validation and bounded search. The current total ordering key uses preferred-start distance, mutation count, shift seconds, start and object ID.</p>
    </div>
    <div class="explainer-reading depth-research research">
      <small>RESEARCH BOUNDARY</small>
      <p>Learned or neural interpretation may become an advisory layer later. It is not part of the current planner runtime and would not replace deterministic validity, permission or execution checks.</p>
    </div>
  </div>

  <div class="explainer-stage" aria-label="How structured planning state becomes a bounded deterministic result">
    <div class="stage-caption stage-caption-left"><small>STRUCTURED PLANNING CONTEXT</small><span>known state · scope · rules · preferences</span></div>
    <div class="interpretation-cloud" aria-hidden="true">
      <i class="cloud-node c1"></i><i class="cloud-node c2"></i><i class="cloud-node c3"></i><i class="cloud-node c4"></i>
      <span class="cloud-path p1"></span><span class="cloud-path p2"></span><span class="cloud-path p3"></span>
    </div>

    <div class="boundary-shell">
      <span class="boundary-label bl-time">TIME</span>
      <span class="boundary-label bl-scope">SCOPE</span>
      <span class="boundary-label bl-policy">POLICY</span>
      <span class="boundary-label bl-knowledge">FACTS</span>
      <span class="boundary-label bl-constraints">HARD RULES</span>
      <div class="boundary-ring ring-a"></div><div class="boundary-ring ring-b"></div>
    </div>

    <div class="deterministic-core">
      <span class="core-pulse" aria-hidden="true"></span>
      <div class="core-title"><small>DETERMINISTIC CORE</small><strong>Generate → Validate → Order</strong></div>
      <div class="core-grid" aria-hidden="true"><i></i><i></i><i></i><i></i><i></i><i></i></div>
      <div class="core-candidates" aria-hidden="true"><span></span><span></span><span class="selected"></span><span></span></div>
      <div class="core-proof"><b>✓</b><span>bounded search assessment</span></div>
    </div>

    <div class="authority-rail">
      <small>AUTHORITY INCREASES EXPLICITLY</small>
      <div class="authority-steps">
        <span><i></i><b>Proposed</b></span><span><i></i><b>Validated</b></span><span><i></i><b>Authorized</b></span><span><i></i><b>Executed</b></span>
      </div>
      <p class="technical-only">PlanningRequest → ProposedPlan → ValidatedPlan → ActionPlan → AuthorizedActionPlan → ExecutionResult</p>
    </div>

    <div class="stage-annotation annotation-a"><b>1</b><span><strong>The input is structured.</strong><small>The current planner consumes CPIR, not free-form language.</small></span></div>
    <div class="stage-annotation annotation-b"><b>2</b><span><strong>Hard rules remove candidates.</strong><small>Invalid positions never become valid merely because they would be preferable.</small></span></div>
    <div class="stage-annotation annotation-c"><b>3</b><span><strong>Valid candidates are ordered.</strong><small>The result and its bounded assessment remain inspectable.</small></span></div>
  </div>

  <div class="explainer-panels" aria-label="Cerebri planner concepts">
    <article><span class="mini-cue cue-time" aria-hidden="true"><i></i></span><div><small>KNOWN STATE</small><strong>What is already true?</strong><p>Typed time, known objects and supplied facts define the state the planner must respect.</p></div></article>
    <article><span class="mini-cue cue-evidence" aria-hidden="true"><i></i><i></i><i></i></span><div><small>HARD VALIDITY</small><strong>What may not be broken?</strong><p>Scope and hard constraints decide whether a candidate is admissible before preferences matter.</p></div></article>
    <article><span class="mini-cue cue-search" aria-hidden="true"><i></i></span><div><small>ORDERING</small><strong>How are valid candidates compared?</strong><p>Deterministic ranking observations and tie-breaks produce one total candidate order.</p></div></article>
    <article><span class="mini-cue cue-authority" aria-hidden="true"><i></i><b></b></span><div><small>LIFECYCLE</small><strong>What does a proposal authorize?</strong><p>Nothing by itself: validation, authorization and execution remain separate downstream states.</p></div></article>
  </div>

  <div class="explainer-next">
    <span>Next: follow the real CPIR 0.2 fixture through its candidate grid.</span>
    <a href={base + '/planning/'}>Inspect planning <b>→</b></a>
  </div>
</section>
