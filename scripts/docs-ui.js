const q=s=>document.querySelector(s);const slug=s=>s.toLowerCase().trim().replace(/[^\w\s-]/g,'').replace(/\s+/g,'-');
const article=q('.document');if(article){const box=q('[data-toc]');article.querySelectorAll('h2,h3').forEach(h=>{h.id=h.id||slug(h.textContent);const a=document.createElement('a');a.href='#'+h.id;a.textContent=h.textContent;if(h.tagName==='H3')a.style.paddingLeft='10px';box?.append(a)})}
q('.menu')?.addEventListener('click',()=>q('.sidebar')?.classList.toggle('open'));
const d=q('#search'),input=d?.querySelector('input'),results=d?.querySelector('.search-results');let index=[];
fetch((location.pathname.includes('/docs/')?'../../'.repeat(Math.max(1,location.pathname.split('/docs/')[1].split('/').length-1)):'')+'search-index.json').then(r=>r.json()).then(x=>index=x).catch(()=>{});
document.querySelectorAll('[data-open-search]').forEach(b=>b.addEventListener('click',()=>d?.showModal()));q('[data-close-search]')?.addEventListener('click',()=>d?.close());
input?.addEventListener('input',()=>{const term=input.value.toLowerCase();results.innerHTML=index.filter(x=>!term||(`${x.title} ${x.group} ${x.path}`).toLowerCase().includes(term)).slice(0,20).map(x=>`<a href="${location.pathname.includes('/docs/')?'../../'.repeat(Math.max(1,location.pathname.split('/docs/')[1].split('/').length-1)):''}${x.path}"><strong>${x.title}</strong><small>${x.group} · ${x.path}</small></a>`).join('')});
window.addEventListener('keydown',e=>{if((e.metaKey||e.ctrlKey)&&e.key==='k'){e.preventDefault();d?.showModal();input?.focus()}if(e.key==='Escape'&&d?.open)d.close()});
let lang=localStorage.getItem('cerebri-lang')||'en';
function applyLang(){document.documentElement.dataset.language=lang;document.querySelectorAll('[data-i18n-en]').forEach(el=>el.textContent=el.dataset['i18n'+(lang==='de'?'De':'En')]);document.querySelectorAll('[data-language]').forEach(b=>b.textContent=lang==='en'?'DE':'EN')}
document.querySelectorAll('[data-language]').forEach(b=>b.addEventListener('click',()=>{lang=lang==='en'?'de':'en';localStorage.setItem('cerebri-lang',lang);applyLang()}));applyLang();
