import { readFile, readdir, mkdir, writeFile, copyFile, rm } from 'node:fs/promises';
import { resolve, relative, dirname, sep, basename } from 'node:path';
import MarkdownIt from 'markdown-it';

const root=process.cwd(), destination=resolve(root,'site');
if(dirname(destination)!==root || relative(root,destination)!=='site') throw new Error('unsafe generated output path');
await rm(destination,{recursive:true,force:true});
const cargo=await readFile('Cargo.toml','utf8');
const version=cargo.match(/\[workspace\.package\][\s\S]*?version = "([^"]+)"/)?.[1] ?? 'development';
const master=await readFile('docs/architecture/specifications/master-v0.4.md','utf8').catch(()=> '');
const spec=master.match(/Specification version:\*\*\s*([0-9.]+)/)?.[1] ?? '0.4';
const md=new MarkdownIt({html:true,linkify:false,typographer:false});
const renderLink=md.renderer.rules.link_open || ((tokens,idx,options,env,self)=>self.renderToken(tokens,idx,options));
md.renderer.rules.link_open=(tokens,idx,options,env,self)=>{
 const href=tokens[idx].attrGet('href');
 if(href&&!/^(?:[a-z]+:|#)/i.test(href)) tokens[idx].attrSet('href',href.replace(/\.md(?=#|$)/,'.html'));
 return renderLink(tokens,idx,options,env,self);
};
async function walk(dir){
 const result=[];
 for(const e of await readdir(dir,{withFileTypes:true})){
  if(['.git','.idea','target','node_modules','site','dist'].includes(e.name)) continue;
  const p=resolve(dir,e.name);
  if(e.isDirectory()) result.push(...await walk(p)); else if(p.endsWith('.md')) result.push(p);
 }
 return result;
}
const files=await walk(root);
const docs=files.filter(p=>relative(root,p).startsWith('docs'+sep));
const css=await readFile('scripts/docs-theme.css','utf8');
const js=await readFile('scripts/docs-ui.js','utf8');
const esc=s=>s.replace(/[&<>"]/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;'}[c]));
const pretty=s=>s.replace(/\.md$/,'').replace(/README$/i,'Overview').replace(/[-_]/g,' ').replace(/\b\w/g,c=>c.toUpperCase());
const category=rel=>{
 const p=rel.split(sep);
 if(rel==='CHANGELOG.md') return 'Project';
 if(rel==='README.md') return 'Project';
 if(p[0]!=='docs') return 'Project';
 return pretty(p[1]||'Documentation');
};
const navItems=files.filter(p=>{
 const rel=relative(root,p);
 return rel==='CHANGELOG.md'||rel==='README.md'||rel==='docs/README.md'||rel.startsWith('docs'+sep);
}).map(p=>({rel:relative(root,p),title:pretty(basename(p)),group:category(relative(root,p))}));
function sidebar(prefix,current){
 const groups=new Map();
 for(const item of navItems){
   if(!groups.has(item.group)) groups.set(item.group,[]);
   groups.get(item.group).push(item);
 }
 return [...groups].map(([g,items])=>`<section class="nav-group"><h3>${esc(g)}</h3>${items.map(i=>{
   const href=prefix+i.rel.replace(/\.md$/,'.html').split(sep).join('/');
   const label=i.rel==='CHANGELOG.md'?'Changelog':i.rel==='README.md'?'Project Home':i.rel==='docs/README.md'?'Documentation Map':i.title;
   return `<a ${i.rel===current?'class="active"':''} href="${href}">${esc(label)}</a>`;
 }).join('')}</section>`).join('');
}
function shell({rel,title,content,home=false}){
 const depth=home?0:rel.split(sep).length-1, prefix='../'.repeat(depth);
 const source=rel==='README.md'?'README.md':rel;
 const changelog=prefix+'CHANGELOG.html';
 return `<!doctype html><html lang="${rel.includes('docs'+sep+'de'+sep)?'de':'en'}"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="theme-color" content="#090e18"><title>${esc(title)} · Nexus Cerebri</title><style>${css}</style></head>
 <body><div class="ambient"></div><aside class="sidebar"><a class="brand" href="${prefix}index.html"><span class="mark">N</span><span><strong>Nexus <em>Cerebri</em></strong><small>KNOWLEDGE SYSTEM</small></span></a><nav>${sidebar(prefix,rel)}</nav><div class="side-foot"><span class="dot"></span> Documentation live<small>Software ${esc(version)} · Spec ${esc(spec)}</small></div></aside>
 <div class="shell"><header class="topbar"><button class="menu" aria-label="Toggle navigation">☰</button><div class="crumb">NEXUS / <b>${esc(title.toUpperCase())}</b></div><div class="top-actions"><button class="search-button" data-open-search>⌘ Search</button><a href="${changelog}">Changelog</a><a href="https://github.com/YoungJibbit95/Nexus-Cerebri">GitHub ↗</a></div></header>
 <main>${content}</main><footer><span>Neural intuition. Symbolic verification.</span><span>Documentation generated from repository truth.</span></footer></div>
 <dialog id="search"><div class="search-head"><input autofocus placeholder="Search documentation…" aria-label="Search documentation"><button data-close-search>×</button></div><div class="search-results"></div></dialog><script>${js}</script></body></html>`;
}
const architecture=`<section class="flow neural-flow"><span>Input</span><i>→</i><span>CPIR</span><i>→</i><span>Planner</span><i>→</i><span>Validation</span><i>→</i><span>ActionPlan</span><i>→</i><span>Authorization</span><i>→</i><span>Executor</span></section>`;
const mindmap=`<div class="mindmap"><div class="mind-core">CEREBRI</div><div class="mind-node n1">Temporal</div><div class="mind-node n2">CPIR</div><div class="mind-node n3">Constraints</div><div class="mind-node n4">Validation</div><div class="mind-node n5">Execution</div><div class="mind-node n6">Research</div></div>`;
const matrix=`<div class="matrix"><div><b>Layer</b><b>Role</b><b>Trust</b></div><div><span>Interpretation</span><span>Meaning & inference</span><em>Probabilistic</em></div><div><span>Planning</span><span>Candidate search</span><em>Bounded</em></div><div><span>Validation</span><span>Constraint proof</span><em>Deterministic</em></div><div><span>Execution</span><span>Authorized mutation</span><em>Guarded</em></div></div>`;
const hero=`<section class="hero portal-hero"><div class="hero-copy"><span class="eyebrow">NEXUS CEREBRI / OPEN RESEARCH</span><h1>Temporal intelligence,<br><em>interlocked with proof.</em></h1><p data-i18n-en="An application-independent planning foundation where neural intuition meets symbolic verification. One living surface for architecture, research, implementation and change history." data-i18n-de="Eine anwendungsunabhängige Planungsgrundlage, in der neuronale Intuition auf symbolische Verifikation trifft. Eine lebende Oberfläche für Architektur, Forschung, Implementierung und Änderungshistorie.">An application-independent planning foundation where neural intuition meets symbolic verification. One living surface for architecture, research, implementation and change history.</p><div class="hero-actions"><a class="primary" href="docs/README.html">Explore docs →</a><a class="secondary" href="CHANGELOG.html">Changelog</a><button class="secondary language-toggle" data-language>DE</button></div></div><div class="logo-stage"><div class="logo-halo"></div><img src="assets/nexus-cerebri-logo.png" alt="Nexus Cerebri logo"></div></section>
<section class="stats"><article><small>SOFTWARE</small><strong>${esc(version)}</strong><span>Temporal Core research</span></article><article><small>SPECIFICATION</small><strong>${esc(spec)}</strong><span>Foundation baseline</span></article><article><small>CPIR</small><strong>0.1</strong><span>Internal schema</span></article><article><small>MODEL</small><strong>Hybrid</strong><span>intuition × verification</span></article></section>
<section class="interlock-grid"><article class="card architecture-card"><span class="eyebrow">SYSTEM FLOW</span><h2>Intuition proposes. Verification constrains.</h2><p>The architecture keeps semantic interpretation, bounded planning, deterministic validation and authorized execution visibly separate.</p>${architecture}${matrix}</article><article class="card mind-card"><span class="eyebrow">KNOWLEDGE MAP</span><h2>One interconnected system.</h2>${mindmap}</article></section>
<section class="visual-band"><div><span class="eyebrow">REASONING MODEL</span><h2>Two modes. One auditable path.</h2><p>Flexible interpretation can explore meaning and preference. Deterministic layers preserve hard boundaries, explicit knowledge states and execution safety.</p></div><div class="dual-graph"><div class="curve intuition"><span>Neural intuition</span></div><div class="curve proof"><span>Symbolic verification</span></div><div class="merge">Validated<br>plan</div></div></section>
<section class="grid"><article class="card"><span class="eyebrow">WIKI / DOCS</span><h2>Repository-native knowledge.</h2><p>Architecture decisions, standards, roadmap, progress logs and bilingual references are rendered directly from Markdown.</p><a href="docs/README.html">Open knowledge map →</a></article><article class="card"><span class="eyebrow">CHANGE STREAM</span><h2>Always-current history.</h2><p><code>CHANGELOG.md</code> is rendered on every deployment. No duplicated release notes and no stale portal copy.</p><a href="CHANGELOG.html">Read changelog →</a></article><article class="card"><span class="eyebrow">LANGUAGE</span><h2>English + Deutsch.</h2><p>Use the language switch for portal copy and the existing DE/EN documentation trees for source documentation.</p><div class="language-pills"><a href="docs/en/README.html">EN</a><a href="docs/de/README.html">DE</a></div></article></section>
<section class="section-title"><span class="eyebrow">EXPLORE THE SYSTEM</span><h2>Architecture, development and research — visually connected.</h2></section>
<section class="topic-grid"><a href="docs/architecture/specifications/master-v0.4.html"><b>01</b><strong>Master Specification</strong><span>Normative architecture baseline</span></a><a href="docs/development/roadmap/README.html"><b>02</b><strong>Roadmap</strong><span>Milestones and learning sequence</span></a><a href="docs/testing/testing-visualization-research-standard.html"><b>03</b><strong>Testing & Research</strong><span>Verification and evidence standards</span></a><a href="docs/architecture/decisions/ADR-0001-workspace.html"><b>04</b><strong>Architecture Decisions</strong><span>Auditable technical decisions</span></a></section>`;
await mkdir(destination,{recursive:true});
for(const path of files){
 const rel=relative(root,path);
 const out=resolve(destination,rel.replace(/\.md$/,'.html'));
 const raw=(await readFile(path,'utf8')).replace(/^<!-- doc:[^]*?-->\s*/,'');
 const first=raw.match(/^#\s+(.+)$/m)?.[1]?.replace(/[*`#]/g,'') ?? pretty(basename(path));
 const body=`<div class="doc-layout"><article class="document"><div class="doc-kicker">${esc(category(rel))}</div>${md.render(raw)}</article><aside class="toc"><strong>ON THIS PAGE</strong><div data-toc></div></aside></div>`;
 await mkdir(dirname(out),{recursive:true});
 await writeFile(out,shell({rel,title:first,content:body}));
}
await writeFile(resolve(destination,'index.html'),shell({rel:'README.md',title:'Home',content:hero,home:true}));
await writeFile(resolve(destination,'.nojekyll'),'');
await mkdir(resolve(destination,'assets'),{recursive:true});
await copyFile(resolve(root,'assets/nexus-cerebri-logo.png'),resolve(destination,'assets/nexus-cerebri-logo.png'));
for(const f of ['request.json','temporal-request.json']){await mkdir(resolve(destination,'examples'),{recursive:true});await copyFile(resolve(root,'examples',f),resolve(destination,'examples',f));}
const search=navItems.map(x=>({title:x.rel==='CHANGELOG.md'?'Changelog':x.title,path:x.rel.replace(/\.md$/,'.html').split(sep).join('/'),group:x.group}));
await writeFile(resolve(destination,'search-index.json'),JSON.stringify(search));
console.log(`Cerebri Portal built: ${files.length} Markdown sources -> site/`);
