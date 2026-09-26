<div class="space-environment" aria-hidden="true">
  <div class="space-void"></div>
  <div class="atmosphere atmosphere-far"></div>
  <div class="atmosphere atmosphere-mid"></div>
  <div class="structural-field field-a"></div>
  <div class="structural-field field-b"></div>
  <div class="spatial-light light-a"></div>
  <div class="spatial-light light-b"></div>
</div>

<style>
  .space-environment{position:fixed;inset:0;z-index:0;pointer-events:none;overflow:hidden;background:#020511}
  .space-void{position:absolute;inset:0;background:radial-gradient(ellipse at 50% -8%,rgba(29,87,170,.12),transparent 42%),linear-gradient(180deg,rgba(2,5,17,.2),rgba(2,5,17,.82))}
  .atmosphere{position:absolute;inset:-8%;background-repeat:repeat;will-change:transform}
  .atmosphere-far{opacity:.52;background-image:radial-gradient(circle at 8% 12%,rgba(255,255,255,.72) 0 1px,transparent 1.5px),radial-gradient(circle at 24% 48%,rgba(104,207,255,.48) 0 1px,transparent 1.5px),radial-gradient(circle at 43% 20%,rgba(255,255,255,.58) 0 1px,transparent 1.5px),radial-gradient(circle at 67% 63%,rgba(164,145,255,.48) 0 1px,transparent 1.5px),radial-gradient(circle at 82% 26%,rgba(255,255,255,.62) 0 1px,transparent 1.5px),radial-gradient(circle at 93% 82%,rgba(113,214,255,.44) 0 1px,transparent 1.5px);background-size:840px 720px;animation:farDrift 70s linear infinite}
  .atmosphere-mid{opacity:.24;background-image:radial-gradient(circle at 18% 70%,rgba(255,255,255,.9) 0 1.3px,transparent 2px),radial-gradient(circle at 54% 35%,rgba(61,218,255,.72) 0 1.2px,transparent 2px),radial-gradient(circle at 78% 76%,rgba(177,151,255,.7) 0 1.2px,transparent 2px);background-size:1180px 930px;animation:midDrift 94s linear infinite reverse}
  .structural-field{position:absolute;border:1px solid rgba(77,135,202,.045);border-radius:50%}.field-a{width:980px;height:330px;right:-420px;top:18vh;transform:rotate(-17deg)}.field-b{width:1120px;height:390px;left:-570px;bottom:4vh;transform:rotate(21deg);border-style:dashed;border-color:rgba(117,75,255,.04)}
  .spatial-light{position:absolute;border-radius:50%;filter:blur(70px);opacity:.2;will-change:transform}.light-a{width:60vw;height:38vw;left:-17vw;top:4vh;background:radial-gradient(circle,rgba(32,216,255,.28),rgba(20,139,255,.07) 45%,transparent 72%);animation:lightDriftA 38s ease-in-out infinite alternate}.light-b{width:56vw;height:44vw;right:-19vw;bottom:-8vw;background:radial-gradient(circle,rgba(117,75,255,.23),rgba(161,71,255,.055) 48%,transparent 72%);animation:lightDriftB 46s ease-in-out infinite alternate}
  @keyframes farDrift{to{transform:translate3d(-24px,15px,0)}}@keyframes midDrift{to{transform:translate3d(18px,-12px,0)}}@keyframes lightDriftA{from{transform:translate3d(-1vw,-1vh,0) scale(.98)}to{transform:translate3d(2vw,2vh,0) scale(1.04)}}@keyframes lightDriftB{from{transform:translate3d(1vw,1vh,0) scale(1.02)}to{transform:translate3d(-2vw,-1vh,0) scale(.97)}}
  @media(max-width:620px){.atmosphere-mid{display:none}.spatial-light{filter:blur(48px);opacity:.15}.structural-field{opacity:.55}}
  @media(prefers-reduced-motion:reduce){.atmosphere,.spatial-light{animation:none!important;transform:none!important}}
</style>