import { readFile, readdir, access } from 'node:fs/promises';
import { resolve, dirname, relative, sep } from 'node:path';
import { execFileSync } from 'node:child_process';
const root = process.cwd();
const excluded = new Set(['.git','.idea','target','node_modules','site','dist']);
async function files(dir) {
 const out=[];
 for(const entry of await readdir(dir,{withFileTypes:true})) {
  if(excluded.has(entry.name))continue;
  const path=resolve(dir,entry.name);
  if(entry.isDirectory())out.push(...await files(path));else out.push(path);
 }
 return out;
}
const paths = await files(root);
for(const path of paths.filter(p=>p.endsWith('.md'))) {
 const source=await readFile(path,'utf8');
 const prose=source.replace(/\x60\x60\x60[^]*?\x60\x60\x60/g,'');
 for(const match of prose.matchAll(/\[[^\]]*\]\(([^)]+)\)/g)) {
  const link=match[1].split('#')[0];
  if(!link||/^[a-z]+:/i.test(link))continue;
  await access(resolve(dirname(path),decodeURIComponent(link)));
 }
}
const de=paths.filter(p=>p.includes(sep+'docs'+sep+'de'+sep)&&p.endsWith('.md'));
const en=paths.filter(p=>p.includes(sep+'docs'+sep+'en'+sep)&&p.endsWith('.md'));
if(de.length!==en.length)throw new Error('DE/EN page counts differ');
for(const path of de) {
 const pair=path.replace(sep+'de'+sep,sep+'en'+sep);
 await access(pair);
 if(!path.endsWith(sep+'README.md')){
  for(const [p,lang] of [[path,'de'],[pair,'en']]) {
   if(!(await readFile(p,'utf8')).includes('lang: '+lang))throw new Error('missing language metadata: '+p);
  }
 }
}
const metadata=JSON.parse(execFileSync('cargo',['metadata','--no-deps','--format-version','1'],{encoding:'utf8'}));
const version=(await readFile('Cargo.toml','utf8')).match(/\[workspace\.package\][\s\S]*?version = "([^"]+)"/)[1];
const packages=new Map(metadata.packages.map(p=>[p.name,p]));
const allowed={
 'cerebri-types':[], 'cerebri-temporal':[],
 'cerebri-constraints':['cerebri-types','cerebri-temporal'],
 'cerebri-semantics':['cerebri-types','cerebri-temporal'],
 'cerebri-preferences':['cerebri-types','cerebri-temporal'],
 'cerebri-ml':['cerebri-types','cerebri-temporal'],
 'cerebri-planner':['cerebri-types','cerebri-temporal','cerebri-constraints','cerebri-semantics','cerebri-preferences'],
 'cerebri-integrations':['cerebri-types','cerebri-temporal','cerebri-planner','cerebri-constraints'],
 'cerebri-core':['cerebri-types','cerebri-planner','cerebri-temporal'],
 'cerebri-api':['cerebri-core'],
 'cerebri-node':['cerebri-core']
};
for(const p of packages.values()){
 if(p.version!==version)throw new Error('workspace version mismatch');
 for(const d of p.dependencies){
  if(d.path?.replaceAll('\\','/').includes('/research/'))throw new Error('production dependency on research');
  if(packages.has(d.name)&&!allowed[p.name]?.includes(d.name))throw new Error('layer violation: '+p.name+' -> '+d.name);
 }
 if(p.name==='cerebri-core'&&p.dependencies.some(d=>['axum','tokio','sqlx','rusqlite','reqwest'].includes(d.name)))throw new Error('core I/O dependency');
}
const readme=await readFile('README.md','utf8');
const changelog=await readFile('CHANGELOG.md','utf8');
if(!changelog.includes('**Software target:** `'+version+'`'))throw new Error('CHANGELOG software target mismatch');
for(const text of [version,'0.4','0.1'])if(!readme.includes(text))throw new Error('README version missing');
if(JSON.parse(await readFile('apps/cerebri-lab/package.json','utf8')).version!==version)throw new Error('Lab version mismatch');
const sitePackage=JSON.parse(await readFile('apps/cerebri-site/package.json','utf8'));
if(sitePackage.version!==version)throw new Error('Official site version mismatch');
const siteSource=await readFile('apps/cerebri-site/src/lib/site.ts','utf8');
for(const section of ['explore','cpir','planning','time','safety','architecture','lab','roadmap','developers']) {
 if(!siteSource.includes(section+':'))throw new Error('missing official site surface: '+section);
}
const siteConfig=await readFile('apps/cerebri-site/svelte.config.js','utf8');
if(!siteConfig.includes('@sveltejs/adapter-static')||!siteConfig.includes('/Nexus-Cerebri'))throw new Error('official site static/base-path configuration missing');
const siteCss=await readFile('apps/cerebri-site/src/app.css','utf8');
if(/fonts\.googleapis\.com|@import\s+url\(/i.test(siteCss))throw new Error('official site must not depend on runtime font imports');
for(const retired of ['scripts/build-docs.mjs','scripts/docs-theme.css','scripts/docs-ui.js']) {
 if(paths.some(p=>relative(root,p).replaceAll('\\','/')===retired))throw new Error('retired docs portal file remains: '+retired);
}
const fixture=JSON.parse(await readFile('examples/request.json','utf8'));
if(fixture.schema_version.major!==0||fixture.schema_version.minor!==1)throw new Error('CPIR fixture version mismatch');
const master=await readFile('docs/architecture/specifications/master-v0.4.md','utf8');
if(master.includes('validated ActionPlan'))throw new Error('stale execution invariant');
for(const path of paths) {
 const name=relative(root,path).replaceAll('\\','/');
 if(/^(datasets\/(personal|private)|models\/artifacts)\//.test(name))throw new Error('private/generated data in source tree');
 if(!/\.(md|rs|json|mjs|js|ts|svelte|css|toml|yml|html)$/.test(path))continue;
 const text=await readFile(path,'utf8');
 if(/-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----/.test(text) ||
   /(?:ghp|github_pat)_[A-Za-z0-9_]{30,}/.test(text))throw new Error('possible credential: '+name);
}
console.log('Repository checks passed: links, DE/EN counterparts, versions, dependency boundaries, official site truth/configuration and credential patterns.');
