<script lang="ts">
  import { base } from '$app/paths';
</script>

<section class="cerebri-explainer cosmic-section" aria-labelledby="cerebri-explainer-title">
  <div class="explainer-transition" aria-hidden="true"><i></i><span></span><b></b></div>

  <div class="explainer-copy">
    <span class="section-index">ACT 02 / WHAT CEREBRI IS</span>
    <p class="kicker">INTERPRETATION → BOUNDED ACTION</p>
    <h2 id="cerebri-explainer-title">Reason broadly.<br /><em>Act inside proof.</em></h2>
    <p class="explainer-lede">
      Cerebri separates understanding a request from making a binding plan. It can explore meaning and ambiguity,
      but a concrete result only becomes authoritative after time, scope, policy, evidence and lifecycle boundaries are explicit.
    </p>

    <div class="principle-stack" aria-label="Three core principles">
      <article>
        <span>01</span>
        <div><strong>Interpret broadly</strong><p>Meaning can stay flexible while context and possible intent are explored.</p></div>
      </article>
      <article>
        <span>02</span>
        <div><strong>Plan deterministically</strong><p>Declared constraints create a bounded search space with inspectable candidate evaluation.</p></div>
      </article>
      <article>
        <span>03</span>
        <div><strong>Commit with explicit authority</strong><p>Proposal, validation, authorization and execution are separate lifecycle states.</p></div>
      </article>
    </div>

    <div class="explainer-reading depth-technical">
      <small>TECHNICAL READING</small>
      <p>CPIR carries explicit planning evidence into typed temporal and constraint-aware Rust planning. Validation narrows state; it does not grant execution permission.</p>
    </div>
    <div class="explainer-reading depth-research research">
      <small>RESEARCH BOUNDARY</small>
      <p>Future learned or neural interpretation may sit outside this verified core, but it does not replace the deterministic authority boundary.</p>
    </div>
  </div>

  <div class="explainer-stage" aria-label="How Cerebri narrows interpretation into bounded action">
    <div class="stage-caption stage-caption-left"><small>OPEN INTERPRETATION SPACE</small><span>context · ambiguity · possible intent</span></div>
    <div class="interpretation-cloud" aria-hidden="true">
      <i class="cloud-node c1"></i><i class="cloud-node c2"></i><i class="cloud-node c3"></i><i class="cloud-node c4"></i>
      <span class="cloud-path p1"></span><span class="cloud-path p2"></span><span class="cloud-path p3"></span>
    </div>

    <div class="boundary-shell">
      <span class="boundary-label bl-time">TIME</span>
      <span class="boundary-label bl-scope">SCOPE</span>
      <span class="boundary-label bl-policy">POLICY</span>
      <span class="boundary-label bl-knowledge">KNOWLEDGE</span>
      <span class="boundary-label bl-constraints">CONSTRAINTS</span>
      <div class="boundary-ring ring-a"></div><div class="boundary-ring ring-b"></div>
    </div>

    <div class="deterministic-core">
      <span class="core-pulse" aria-hidden="true"></span>
      <div class="core-title"><small>DETERMINISTIC CORE</small><strong>Search → Evaluate → Prove</strong></div>
      <div class="core-grid" aria-hidden="true"><i></i><i></i><i></i><i></i><i></i><i></i></div>
      <div class="core-candidates" aria-hidden="true"><span></span><span></span><span class="selected"></span><span></span></div>
      <div class="core-proof"><b>✓</b><span>inspectable result</span></div>
    </div>

    <div class="authority-rail">
      <small>AUTHORITY INCREASES EXPLICITLY</small>
      <div class="authority-steps">
        <span><i></i><b>Proposed</b></span><span><i></i><b>Validated</b></span><span><i></i><b>Authorized</b></span><span><i></i><b>Executed</b></span>
      </div>
      <p class="technical-only">PlanningRequest → ProposedPlan → ValidatedPlan → ActionPlan → AuthorizedActionPlan → ExecutionResult</p>
    </div>

    <div class="stage-annotation annotation-a"><b>1</b><span><strong>Interpretation may be uncertain.</strong><small>Uncertainty is not silently promoted into action.</small></span></div>
    <div class="stage-annotation annotation-b"><b>2</b><span><strong>Boundaries become explicit.</strong><small>Scope and time limit what may be considered.</small></span></div>
    <div class="stage-annotation annotation-c"><b>3</b><span><strong>The core resolves a bounded result.</strong><small>Search and proof remain inspectable.</small></span></div>
  </div>

  <div class="explainer-panels" aria-label="Cerebri explainer points">
    <article><span class="mini-cue cue-time" aria-hidden="true"><i></i></span><div><small>TIME + SCOPE</small><strong>Where can the system act?</strong><p>Typed time and explicit scope turn vague possibility into a bounded domain.</p></div></article>
    <article><span class="mini-cue cue-evidence" aria-hidden="true"><i></i><i></i><i></i></span><div><small>CPIR / EVIDENCE</small><strong>What facts and rules count?</strong><p>Knowledge, provenance, policy, capability and budget remain separately inspectable.</p></div></article>
    <article><span class="mini-cue cue-search" aria-hidden="true"><i></i></span><div><small>PLANNING</small><strong>Which candidate survives?</strong><p>The planner evaluates a declared search space instead of improvising hidden actions.</p></div></article>
    <article><span class="mini-cue cue-authority" aria-hidden="true"><i></i><b></b></span><div><small>LIFECYCLE</small><strong>When does a result gain authority?</strong><p>Validation and permission remain distinct, and execution sits beyond its own boundary.</p></div></article>
  </div>

  <div class="explainer-next">
    <span>Next: see the same idea using the canonical synthetic request.</span>
    <a href={base + '/cpir/'}>Inspect CPIR <b>→</b></a>
  </div>
</section>
