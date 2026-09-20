import { readFile, readdir, mkdir, writeFile, copyFile, rm } from 'node:fs/promises';
import { resolve, relative, dirname, sep } from 'node:path';
import MarkdownIt from 'markdown-it';
const root=process.cwd(), destination=resolve(root,'site');
if(dirname(destination)!==root || relative(root,destination)!=='site')throw new Error('unsafe generated output path');
await rm(destination,{recursive:true,force:true});
const version=(await readFile('Cargo.toml','utf8')).match(/\[workspace\.package\][\s\S]*?version = "([^"]+)"/)[1];
const md=new MarkdownIt({html:false,linkify:false,typographer:false});
const renderLink=md.renderer.rules.link_open || ((tokens,idx,options,env,self)=>self.renderToken(tokens,idx,options));
md.renderer.rules.link_open=(tokens,idx,options,env,self)=>{
 const href=tokens[idx].attrGet('href');
 if(href&&!/^[a-z]+:/i.test(href))tokens[idx].attrSet('href',href.replace(/\.md(?=#|$)/,'.html'));
 return renderLink(tokens,idx,options,env,self);
};
async function walk(dir){
 const result=[];
 for(const e of await readdir(dir,{withFileTypes:true})){
  if(['.git','.idea','target','node_modules','site','dist'].includes(e.name))continue;
  const p=resolve(dir,e.name);
  if(e.isDirectory())result.push(...await walk(p));else if(p.endsWith('.md'))result.push(p);
 }
 return result;
}
for(const path of await walk(root)){
 const rel=relative(root,path), output=resolve(destination,rel.replace(/\.md$/,'.html'));
 const depth=rel.split(sep).length-1, prefix='../'.repeat(depth);
 const content=md.render((await readFile(path,'utf8')).replace(/^<!-- doc:[^]*?-->\s*/,''));
 const lang=rel.includes('docs'+sep+'de'+sep)?'de':'en';
 const html='<!doctype html><html lang="'+lang+'"><meta charset="utf-8"><meta name="viewport" content="width=device-width"><title>Nexus Cerebri Foundation</title><style>body{font:17px/1.6 system-ui;max-width:960px;margin:2rem auto;padding:0 1rem;color:#182735;background:#fafcfd}pre{overflow:auto;padding:1rem;background:#eaf0f3}table{border-collapse:collapse}td,th{border:1px solid #aab8c1;padding:.5rem}a{color:#175e7a}nav{padding:1rem 0;border-bottom:1px solid}</style><nav><a href="'+prefix+'README.html">Nexus Cerebri</a> · <a href="'+prefix+'docs/de/README.html">Deutsch</a> · <a href="'+prefix+'docs/en/README.html">English</a> · '+version+' research</nav><main>'+content+'</main></html>';
 await mkdir(dirname(output),{recursive:true});await writeFile(output,html);
 if(rel==='README.md')await writeFile(resolve(destination,'index.html'),html);
}
await writeFile(resolve(destination,'.nojekyll'),'');
await mkdir(resolve(destination,'examples'),{recursive:true});
await copyFile(resolve(root,'examples/request.json'),resolve(destination,'examples/request.json'));
await copyFile(resolve(root,'examples/temporal-request.json'),resolve(destination,'examples/temporal-request.json'));
console.log('Static documentation built from repository Markdown in site/.');
