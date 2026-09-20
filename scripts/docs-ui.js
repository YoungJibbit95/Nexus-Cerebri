const $=(s,r=document)=>r.querySelector(s), $$=(s,r=document)=>[...r.querySelectorAll(s)];
$('.menu')?.addEventListener('click',()=>$('.sidebar')?.classList.toggle('open'));
const article=$('.document'), toc=$('.toc-links'); if(article&&toc){$$('h2,h3',article).forEach(h=>{const id=h.id||h.textContent.toLowerCase().replace(/[^\w]+/g,'-');h.id=id;const a=document.createElement('a');a.href='#'+id;a.textContent=h.textContent;if(h.tagName==='H3')a.style.paddingLeft='9px';toc.append(a)})}
const dialog=$('#search'), input=$('#search input'), results=$('.results');let idx=[];
fetch(document.body.dataset.root+'search-index.json').then(r=>r.json()).then(x=>idx=x).catch(()=>{});
$$('[data-search]').forEach(b=>b.addEventListener('click',()=>{dialog.showModal();input.focus()}));$('[data-close]')?.addEventListener('click',()=>dialog.close());
input?.addEventListener('input',()=>{const q=input.value.toLowerCase();results.innerHTML=idx.filter(x=>!q||(x.title+' '+x.group+' '+x.path).toLowerCase().includes(q)).slice(0,18).map(x=>`<a href="${document.body.dataset.root}${x.path}"><strong>${x.title}</strong><small>${x.group} · ${x.path}</small></a>`).join('')});
window.addEventListener('keydown',e=>{if((e.ctrlKey||e.metaKey)&&e.key==='k'){e.preventDefault();dialog.showModal();input.focus()}});
