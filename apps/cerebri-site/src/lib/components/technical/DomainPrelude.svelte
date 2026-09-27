<script lang="ts">
  import { request, timeLabel } from '$lib/planning-display';
  let { kind }: { kind: string } = $props();
</script>

<div class="domain-prelude" class:envelope={kind === 'cpir'} aria-hidden="true">
  {#if kind === 'cpir'}
    <div class="prelude-envelope"><span>CPIR / INPUT</span><div class="prelude-range"><b>{timeLabel(request.scope.time_range.start)}</b><i></i><b>{timeLabel(request.scope.time_range.end)}</b></div><code>scope.time_range</code><div class="prelude-fold"><i></i><i></i><i></i></div></div>
  {:else}
    {#each [0,1,2,3,4] as layer}<div class="prelude-layer" style={'--layer:' + layer}><i></i><i></i><i></i><span></span></div>{/each}
  {/if}
</div>

<style>
  .domain-prelude { position:absolute; right:5%; top:75px; width:420px; height:290px; pointer-events:none; opacity:.45; }
  .prelude-envelope { position:absolute; inset:35px 0 0; padding:32px; border:1px solid #8edbdd55; border-top:2px solid #8edbdd; transform:rotate(-8deg) skewY(2deg); background:linear-gradient(145deg,#123e4f66,transparent); }
  .prelude-envelope>span { color:#acdada; font:10px var(--mono); }
  .prelude-envelope code { color:#acdada; font:12px var(--mono); }
  .prelude-range { display:flex; gap:12px; align-items:center; margin:20px 0 10px; font:16px var(--mono); color:#bee9e8; }
  .prelude-range i { flex:1; height:1px; background:#94d4d6; border-inline:3px solid #a4dadd; }
  .prelude-fold { padding:20px 0 0 30px; display:flex; gap:15px; }
  .prelude-fold i { width:45px; height:25px; border-left:1px solid #94d4d6; border-bottom:1px solid #94d4d644; }
  .prelude-layer { position:absolute; left:calc(var(--layer) * 10px); top:calc(var(--layer) * 39px); width:340px; height:110px; padding:30px; transform:rotate(-13deg) skewX(24deg); border:1px solid #85c8d066; background:linear-gradient(100deg,#10334599,#071a3044); display:flex; gap:12px; }
  .prelude-layer i { width:40px; height:18px; border:1px solid #aacbdb55; }
  .prelude-layer span { flex:1; border-top:1px solid #aacbdb55; margin-top:9px; }
  @media(max-width:760px) { .domain-prelude { width:260px; height:190px; right:-25px; top:75px; opacity:.28; } .prelude-envelope { padding:20px; } .prelude-layer { width:210px; height:75px; top:calc(var(--layer) * 25px); padding:20px; } }
</style>
