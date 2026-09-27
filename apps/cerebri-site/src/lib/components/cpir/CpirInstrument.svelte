<script lang="ts">
  import { base } from '$app/paths';
  import { runtimeData } from '$lib/generated/runtime-data';
  import { request } from '$lib/planning-display';
  import SourceLink from '../SourceLink.svelte';
  import DepthLens from '../technical/DepthLens.svelte';
  import RangeField from '../technical/RangeField.svelte';
  import KnowledgeAxis from './KnowledgeAxis.svelte';
  import PlannerResult from './PlannerResult.svelte';
  import '../technical/technical.css';
  let precision = $state(0);
  const busy = request.context.objects[1].time.value.knowledge.data;
  const capability = request.planning_capability;
</script>

<div class="technical-instrument cpir-instrument" id="evidence-instrument" data-precision={precision}>
  <DepthLens bind:value={precision} />
  <section class="evidence-envelope" aria-labelledby="evidence-title" data-schema="request">
    <header class="instrument-heading"><div><small>CPIR INPUT / {runtimeData.metadata.cpirVersion}</small><h2 id="evidence-title">Evidence envelope</h2></div><span>ONE REQUEST · DISTINCT BOUNDARIES</span></header>
    <dl class="request-coordinate" aria-label="Request identity">
      <div><dt>Request <code>request_id</code></dt><dd>{request.request_id}</dd></div>
      <div><dt>Operation <code>operation</code></dt><dd>{request.operation}</dd></div>
      <div><dt>Targets <code>target_ids</code></dt><dd>{JSON.stringify(request.target_ids)}</dd></div>
    </dl>
    <ul class="evidence-strata" aria-label="Selected CPIR request fields">
      <li class="scope-stratum" id="scope-field">
        <small>01 / BOUNDED STATE</small>
        <RangeField label="Scope boundary" path="scope.time_range" start={request.scope.time_range.start} end={request.scope.time_range.end} {precision} />
        <div class="busy-fold"><RangeField label="Known busy interval" path="context.objects[1].time.value.knowledge.data" start={busy.start} end={busy.end} {precision} kind="fact" /></div>
        <dl class="temporal-measures">
          <div><dt>Duration <code>duration.value.knowledge.data</code></dt><dd>{request.duration.value.knowledge.data} s</dd></div>
          <div><dt>Grid <code>granularity</code></dt><dd>{request.granularity} s</dd></div>
          <div><dt>Mutation bound <code>scope.max_mutations</code></dt><dd>{request.scope.max_mutations}</dd></div>
        </dl>
      </li>
      <li class="knowledge-stratum"><KnowledgeAxis precise={precision > 0} /></li>
      <li class="empty-stratum">
        <small>03 / HARD CONSTRAINTS</small><h3>Constraint cut</h3>
        <dl><div><dt><code>constraints</code></dt><dd><span class="empty-symbol" aria-hidden="true">[ ]</span>{JSON.stringify(request.constraints)} · empty array</dd></div></dl>
        <p>The region exists; this fixture declares no additional constraints.</p>
      </li>
      <li class="empty-stratum preference-stratum">
        <small>04 / SOFT PREFERENCES</small><h3>Preference field</h3>
        <dl><div><dt><code>preferences.preferences</code></dt><dd><span class="empty-symbol" aria-hidden="true">∅</span>{JSON.stringify(request.preferences.preferences)} · empty array</dd></div></dl>
        <p>The preference set is present, with no entries in this request.</p>
      </li>
      <li class="policy-stratum">
        <small>05 / DECLARATIVE RULE PLANE</small><h3>Policy</h3><code>policy.snapshot</code>
        <dl class="field-detail">
          <div><dt><code>policy.policy_set_id</code></dt><dd>{request.policy.policy_set_id}</dd></div>
          <div><dt><code>policy.snapshot.mutation.allowed_actions</code></dt><dd>{request.policy.snapshot.mutation.allowed_actions.join(' · ')}</dd></div>
          <div><dt><code>policy.snapshot.confirmation.all_mutations</code></dt><dd>{String(request.policy.snapshot.confirmation.all_mutations)}</dd></div>
          <div><dt><code>policy.snapshot.allow_uncertain_duration</code></dt><dd>{String(request.policy.snapshot.allow_uncertain_duration)}</dd></div>
        </dl>
      </li>
      <li class="capability-stratum">
        <small>06 / PROPOSAL OPERATION SET</small><h3>Planning capability</h3><code>planning_capability</code>
        <div class="action-aperture"><span aria-hidden="true">⌜</span><strong>{capability.mutations[0].kind}</strong><span aria-hidden="true">⌟</span></div>
        <dl class="field-detail">
          <div><dt><code>planning_capability.mutations[0].object_id</code></dt><dd>{capability.mutations[0].object_id}</dd></div>
          <div><dt><code>planning_capability.read</code> / <code>planning_capability.plan</code></dt><dd>{String(capability.read)} / {String(capability.plan)}</dd></div>
        </dl>
        <p>Policy rules and proposal capability are separate. Neither is execution authority.</p>
      </li>
      <li class="budget-stratum">
        <div><small>07 / FINITE SEARCH BOUND</small><h3>Declared budget</h3><p>Limits, not observed resource usage.</p></div>
        <dl class="budget-rail">
          {#each Object.entries(request.budget) as [key, value]}
            <div data-field={'budget.' + key}><dt><code>budget.{key}</code></dt><dd>{value}<span aria-hidden="true"></span></dd></div>
          {/each}
        </dl>
      </li>
    </ul>
    <div class="input-source"><SourceLink path="examples/request.json" label="Canonical CPIR request" /><span>Selected exact paths · full request below</span></div>
    <details><summary>CPIR input · raw data</summary><textarea readonly rows="12" aria-label="CPIR request JSON" value={JSON.stringify(request, null, 2)}></textarea></details>
  </section>
  <div class="planner-aperture" role="region" aria-label="Planner boundary separates CPIR input from planner result">
    <span>CPIR INPUT</span><i aria-hidden="true"></i><div><strong>PLANNER BOUNDARY</strong><code>cerebri-core::plan → cerebri-planner</code><small>Compile · validate input · bounded search</small></div><i aria-hidden="true"></i><span>PLANNER RESULT</span>
  </div>
  <PlannerResult {precision} />
  <a class="runtime-link" href={base + '/architecture/#runtime-planner'}><span>CPIR types + snapshot compilation → owning layer</span><code>cerebri-planner →</code></a>
</div>

<style>
  .evidence-envelope { position:relative; padding:36px; border:1px solid var(--line); border-top:3px solid #82dbd9; background:linear-gradient(135deg,#09203588,transparent 55%); }
  .evidence-envelope::after { content:''; position:absolute; top:-9px; right:20px; width:65px; height:16px; background:#071326; border-block:1px solid #82dbd9; transform:skew(-30deg); }
  .request-coordinate { display:flex; flex-wrap:wrap; gap:24px 55px; padding:18px 0 25px; border-block:1px solid var(--line); }
  .request-coordinate dt { font-size:12px; color:var(--muted); }
  .request-coordinate code { display:block; font-size:10px; }
  .request-coordinate dd { margin-top:6px; font:13px/1.7 var(--mono); }
  .evidence-strata { padding:0; margin:0; list-style:none; display:grid; grid-template-columns:1.15fr 1fr; column-gap:46px; }
  .evidence-strata>li { min-width:0; padding:34px 0; border-bottom:1px solid var(--line); }
  .scope-stratum>small { display:block; margin-bottom:12px; }
  .busy-fold { margin:15px 0 0 28px; padding:20px 0 0 20px; border-left:1px solid #efb5b333; }
  .temporal-measures { display:grid; grid-template-columns:1fr 1fr; gap:14px; margin-top:15px!important; }
  .temporal-measures dt { color:var(--muted); font-size:11px; } .temporal-measures code { display:block; font-size:10px; }
  .temporal-measures dd { font:16px var(--mono); margin-top:6px; }
  .empty-stratum h3,.policy-stratum h3,.capability-stratum h3,.budget-stratum h3 { margin:8px 0 12px; }
  .empty-stratum dd { display:flex; align-items:center; gap:16px; font:12px var(--mono); }
  .empty-symbol { font:36px/1.5 var(--mono); color:#efafb7; }
  .preference-stratum .empty-symbol { color:#cebef1; }
  .empty-stratum p,.capability-stratum p { font-size:12px; max-width:38ch; }
  .policy-stratum { padding-left:22px!important; border-left:3px double #eed6a1; }
  .policy-stratum .field-detail { margin-top:15px; border:0; padding:0; }
  .action-aperture { display:flex; gap:15px; align-items:center; padding:15px 0; color:#acdbed; }
  .action-aperture span { font:40px var(--mono); } .action-aperture strong { font:14px var(--mono); }
  .budget-stratum { grid-column:1/-1; display:grid; grid-template-columns:190px minmax(0,1fr); gap:28px; align-items:center; }
  .budget-stratum p { font-size:12px; }
  .budget-rail { display:grid; grid-template-columns:repeat(4,minmax(0,1fr)); gap:18px; }
  .budget-rail code { font-size:10px; }
  .budget-rail dd { font:30px/1.6 var(--mono); position:relative; padding-bottom:10px; }
  .budget-rail dd span { display:block; height:12px; border-inline:2px solid #aed4ec; border-bottom:1px solid #aed4ec; background:repeating-linear-gradient(90deg,transparent 0 calc(25% - 1px),#aed4ec55 calc(25% - 1px) 25%); }
  .input-source { display:flex; flex-wrap:wrap; align-items:center; justify-content:space-between; gap:12px; padding:24px 0; }
  .input-source>span { color:var(--muted); font-size:11px; }
  .planner-aperture { display:flex; justify-content:center; align-items:center; gap:24px; padding:64px 0; border-bottom:1px solid var(--line); }
  .planner-aperture>span { font:11px var(--mono); color:#c4d4e5; }
  .planner-aperture i { width:55px; height:1px; background:#9de8df; position:relative; }
  .planner-aperture i::after { content:'›'; position:absolute; right:-1px; top:-12px; color:#9de8df; font-size:20px; }
  .planner-aperture>div { padding:18px; border-inline:2px solid #9de8df; text-align:center; }
  .planner-aperture strong,.planner-aperture small { display:block; }
  .planner-aperture strong { font:12px var(--mono); color:#a5eee8; margin-bottom:7px; }
  [data-precision="2"] .evidence-strata { column-gap:38px; }
  [data-precision="2"] .evidence-strata>li { border-left:1px solid var(--line); padding-left:20px; }
  @media(max-width:900px) {
    .evidence-envelope { padding:24px; }
    .evidence-strata { column-gap:26px; }
    .budget-stratum { grid-template-columns:1fr; }
    .planner-aperture { gap:12px; }
    .planner-aperture i { width:20px; }
  }
  @media(max-width:700px) {
    .evidence-envelope { padding:24px 16px; border-right:0; border-left:1px solid var(--line); }
    .evidence-strata { grid-template-columns:1fr; }
    .evidence-strata>li { padding-left:16px; border-left:1px solid var(--line); }
    .knowledge-stratum { grid-row:2; }
    .request-coordinate { gap:14px; flex-direction:column; }
    .request-coordinate>div { display:grid; grid-template-columns:95px minmax(0,1fr); }
    .request-coordinate dd { margin:0; font-size:11px; }
    .busy-fold { margin-left:0; padding-left:12px; }
    .temporal-measures { grid-template-columns:1fr; }
    .budget-rail { grid-template-columns:1fr 1fr; gap:22px 15px; }
    .planner-aperture { flex-direction:column; padding:38px 0; }
    .planner-aperture i { width:1px; height:25px; }
    .planner-aperture i::after { content:'↓'; top:8px; right:-7px; }
  }
</style>
