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
const architecture=`<section class="flow"><span>Input</span><i>→</i><span>CPIR</span><i>→</i><span>Planner</span><i>→</i><span>Validation</span><i>→</i><span>ActionPlan</span><i>→</i><span>Authorization</span><i>→</i><span>Executor</span></section>`;
const hero=`<section class="hero"><div><span class="eyebrow">NEXUS CEREBRI / OPEN RESEARCH</span><h1>Make temporal reasoning<br><em>visible and verifiable.</em></h1><p>An application-independent planning foundation built around neural intuition + symbolic verification. Explore the architecture, implementation, decisions and current development state from one living knowledge surface.</p><div class="hero-actions"><a class="primary" href="docs/README.html">Explore documentation →</a><a class="secondary" href="CHANGELOG.html">Latest changes</a></div></div><div class="orb"><div class="ring r1"></div><div class="ring r2"></div><div class="core">N</div></div></section>
<section class="stats"><article><small>SOFTWARE</small><strong>${esc(version)}</strong><span>Temporal Core research</span></article><article><small>SPECIFICATION</small><strong>${esc(spec)}</strong><span>Foundation baseline</span></article><article><small>CPIR</small><strong>0.1</strong><span>Internal schema</span></article><article><small>PRINCIPLE</small><strong>Deterministic</strong><span>trusted lower layers</span></article></section>
<section class="grid"><article class="card wide"><span class="eyebrow">SYSTEM ARCHITECTURE</span><h2>Reason probabilistically. Verify symbolically.</h2><p>Cerebri separates interpretation, planning, deterministic validation, authorization and execution. The planner proposes; trusted layers decide what is valid and executable.</p>${architecture}</article><article class="card"><span class="eyebrow">KNOWLEDGE BASE</span><h2>Repository-native wiki</h2><p>Architecture decisions, standards, roadmap, progress logs and bilingual references are rendered directly from the repository Markdown.</p><a href="docs/README.html">Open knowledge map →</a></article><article class="card"><span class="eyebrow">CHANGE STREAM</span><h2>Always-current changelog</h2><p>The page is generated from <code>CHANGELOG.md</code> on every Pages deployment. No duplicated release notes.</p><a href="CHANGELOG.html">Read changelog →</a></article></section>
<section class="section-title"><span class="eyebrow">CURRENT DOCUMENTATION</span><h2>One surface for architecture, development and research.</h2></section>
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
for(const f of ['request.json','temporal-request.json']){await mkdir(resolve(destination,'examples'),{recursive:true});await copyFile(resolve(root,'examples',f),resolve(destination,'examples',f));}
const search=navItems.map(x=>({title:x.rel==='CHANGELOG.md'?'Changelog':x.title,path:x.rel.replace(/\.md$/,'.html').split(sep).join('/'),group:x.group}));
await writeFile(resolve(destination,'search-index.json'),JSON.stringify(search));
console.log(`Cerebri Portal built: ${files.length} Markdown sources -> site/`);
