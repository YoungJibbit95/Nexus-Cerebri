<script lang="ts">
  import { planner, request, candidateIdentity, timeLabel, timePosition, durationPosition } from '$lib/planning-display';
  let { inspected = $bindable(planner.conflicts.rejections.at(-1)!.start) }: { inspected?: string } = $props();
  const rejection = $derived(planner.conflicts.rejections.find(entry => entry.start === inspected)!);
  const reason = $derived(rejection.reasons[0].HardConstraint);
  const blocker = $derived(request.context.objects.find(object => object.id === reason.evidence.blocking_objects[0])!);
  const busyRange = $derived(blocker.time.value.knowledge.data);
</script>

<section class="rejection-witness" aria-labelledby="rejection-title">
  <header><span class="kicker">01 / INSPECT A CONFLICT</span><h3 id="rejection-title">The red cut has a reason.</h3><p>The requested interval crosses supplied busy time. Inspect the rejection recorded by Rust.</p></header>
  <div class="witness-controls" role="group" aria-label="Rejected candidates">
    {#each planner.conflicts.rejections as entry}
      <button aria-pressed={entry.start === inspected} onclick={() => inspected = entry.start}>{candidateIdentity(entry.start)} · {timeLabel(entry.start)}</button>
    {/each}
  </div>
  <div class="witness-body">
    <div class="witness-geometry" aria-hidden="true">
      <div class="witness-row"><span>Known busy</span><b>{timeLabel(busyRange.start)}–{timeLabel(busyRange.end)} UTC</b><div class="witness-track"><i class="busy-interval" style={`left:${timePosition(busyRange.start)}%;width:${timePosition(busyRange.end) - timePosition(busyRange.start)}%`}></i></div></div>
      <div class="witness-row"><span>{candidateIdentity(inspected)} · proposed placement</span><b>{timeLabel(inspected)} · {request.duration.value.knowledge.data / 60} min</b><div class="witness-track"><i class="rejected-interval" style={`left:${timePosition(inspected)}%;width:${durationPosition}%`}><em></em></i></div></div>
      <div class="witness-axis"><span>{timeLabel(request.scope.time_range.start)}</span><span>{timeLabel(request.scope.time_range.end)} UTC</span></div>
    </div>
    <div class="witness-reason" aria-live="polite" aria-atomic="true"><span>RUST REJECTION / {candidateIdentity(inspected)}</span><strong>{reason.constraint.kind}</strong><p>{timeLabel(inspected)} UTC, {request.duration.value.knowledge.data / 60} minutes: <b>{reason.reason}</b> with <code>{blocker.id}</code> ({timeLabel(busyRange.start)}–{timeLabel(busyRange.end)} UTC).</p><p>A preference cannot override this rejection.</p></div>
  </div>
  <details><summary>Inspect the rejection record</summary><pre><code>{JSON.stringify(rejection, null, 2)}</code></pre></details>
</section>

<style>
  .rejection-witness{padding:2rem 0;border-block:1px solid var(--line);min-width:0}.rejection-witness h3{font-size:clamp(1.5rem,2.5vw,2rem);line-height:1.2;margin:.75rem 0}.rejection-witness p{font-size:1rem;color:var(--text-3);line-height:1.65;max-width:65ch}.witness-controls{display:flex;gap:.5rem;flex-wrap:wrap;margin:1.5rem 0}.witness-controls button{font:.8125rem var(--mono);min-height:44px;padding:.6rem 1rem;border:1px solid var(--line);background:var(--space-1);color:var(--text-2);cursor:pointer}.witness-controls button[aria-pressed=true]{border-color:var(--red);color:var(--red);background:rgba(255,135,149,.04)}
  .witness-body{display:grid;grid-template-columns:minmax(0,1.2fr) minmax(0,.8fr);gap:3rem;align-items:center}.witness-row{display:flex;justify-content:space-between;gap:.5rem;flex-wrap:wrap;font:.8125rem/1.6 var(--mono);color:var(--text-3);margin-top:1.25rem}.witness-row b{font-weight:400;color:var(--text-2)}.witness-track{position:relative;height:34px;width:100%;border-bottom:1px solid var(--line)}.witness-track>i{position:absolute;top:10px;height:8px;border:1px solid var(--red);transition:left 400ms var(--ease-resolve)}.busy-interval{background:repeating-linear-gradient(135deg,#ffa6b222 0 5px,transparent 5px 10px)}.rejected-interval{background:linear-gradient(90deg,#f69caa88,#f69caa11)}.rejected-interval em{position:absolute;left:15%;top:-8px;height:24px;border-left:2px solid var(--red);transform:rotate(25deg)}.witness-axis{display:flex;justify-content:space-between;font:.8125rem var(--mono);color:var(--text-3);margin-top:1rem}.witness-reason{border-left:2px solid var(--red);padding-left:1.5rem}.witness-reason>span{display:block;color:var(--red);font:.8125rem var(--mono)}.witness-reason>strong{display:block;font:1rem var(--mono);margin-top:1rem}.witness-reason code{color:var(--text-1)}.rejection-witness details{margin-top:1.5rem}.rejection-witness summary{cursor:pointer;padding:.8rem 0;font-size:.9375rem;min-height:44px}.rejection-witness pre{overflow:auto;font-size:.8125rem;padding:1rem;background:var(--space-1)}
  @media(max-width:760px){.witness-body{grid-template-columns:1fr;gap:2rem}.witness-controls button{flex:1 1 40%;padding:.6rem}.witness-row{font-size:.8125rem}.witness-reason{padding-left:1rem}}
  @media(prefers-reduced-motion:reduce){.witness-track>i{transition:none}}
</style>
