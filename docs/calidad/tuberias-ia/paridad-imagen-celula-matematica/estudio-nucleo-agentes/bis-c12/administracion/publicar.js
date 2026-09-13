
// Publicación y verificación directa, sin entorno local ni force push.
// Requiere F=c12files y cortes/árboles recuperados mediante github_fetch.
function unwrap(r){if(r.isError)throw Error(JSON.stringify(r));return r.structuredContent.result||r.structuredContent;}
async function fetchJSON(url){return JSON.parse(unwrap(await tools.mcp__codex_apps__github_fetch({url})).content);}
const B=load("B"),D=B+"/bis-c12",F=load("c12files");
for(const key of ["lab","repo"]){
 const repo=load(key),branch=key==="lab"?"lab/playground-sv-permanente":"main",base=load(key==="lab"?"labhead":"head");
 const cp=load("c12publish-"+key)||{blobs:{},base};
 const url="https://api.github.com/repos/"+repo;
 if(cp.done){const h=await fetchJSON(url+"/git/ref/heads/"+branch);if(h.object.sha!==cp.commit)throw Error("Cabecera posterior distinta");text({repo,commit:cp.commit,verified:true});continue;}
 const head=await fetchJSON(url+"/git/ref/heads/"+branch);
 if(head.object.sha!==base)throw Error("Corte remoto cambió: "+repo);
 const files={};
 for(const [p,s]of Object.entries(F)){
  if(key==="lab"&&p.includes("REGISTRO_EVOLUCION_TECNICA_PROYECTO."))continue;
  const q=key==="repo"?p:p.includes("/Inventario-sv/sucesos/")?"laboratorio/tareas-watson/sucesos-sv/"+p.split("/").pop():p.replace("docs/calidad/","laboratorio/tareas-watson/");
  files[q]=s;
 }
 const before=load(key+"tree").tree.filter(x=>x.type==="blob");
 for(const [p,s]of Object.entries(files)){
  const prior=before.find(x=>x.path===p);if(prior&&prior.mode!=="100644")throw Error("Modo inesperado");
  if(!cp.blobs[p]){const b=unwrap(await tools.mcp__codex_apps__github_create_blob({repository_full_name:repo,content:s,encoding:"utf-8"}));if(!b.sha)throw Error("Sin SHA");cp.blobs[p]=b.sha;store("c12publish-"+key,cp);}
 }
 if(!cp.tree){
  const t=unwrap(await tools.mcp__codex_apps__github_create_tree({repository_full_name:repo,base_tree_sha:load(key+"commitbase").tree.sha,tree_elements:Object.entries(cp.blobs).map(([path,sha])=>({path,sha,mode:"100644",type:"blob"}))}));
  if(!t.sha)throw Error("Árbol sin SHA");cp.tree=t.sha;store("c12publish-"+key,cp);
 }
 const actual=await fetchJSON(url+"/git/trees/"+cp.tree+"?recursive=1");
 if(actual.truncated)throw Error("Árbol truncado");
 const after=actual.tree.filter(x=>x.type==="blob"),map=Object.fromEntries(after.map(x=>[x.path,x]));
 for(const x of before)if(!Object.hasOwn(files,x.path)){if(!map[x.path]||map[x.path].sha!==x.sha||map[x.path].mode!==x.mode)throw Error("Cambio ajeno "+x.path);}
 for(const [p,h]of Object.entries(cp.blobs))if(map[p]?.sha!==h)throw Error("Blob distinto "+p);
 if(after.some(x=>!before.some(b=>b.path===x.path)&&!Object.hasOwn(files,x.path)))throw Error("Archivo ajeno añadido");
 if(!cp.commit){const c=unwrap(await tools.mcp__codex_apps__github_create_commit({repository_full_name:repo,parent_sha:base,tree_sha:cp.tree,message:"docs: BIS-C12 perfiles, documentación y construcción; RETP-215"}));if(!c.sha)throw Error("Commit sin SHA");cp.commit=c.sha;store("c12publish-"+key,cp);}
 // Cotejar literalmente cada archivo preparado contra su blob creado.
 for(const [p,h]of Object.entries(cp.blobs)){
  const got=unwrap(await tools.mcp__codex_apps__github_fetch({url:"https://github.com/"+repo+"/blob/"+cp.commit+"/"+p})).content;
  if(got!==files[p])throw Error("Contenido distinto "+p);
 }
 const current=await fetchJSON(url+"/git/ref/heads/"+branch);if(current.object.sha!==base)throw Error("Cabecera cambió antes de publicar");
 unwrap(await tools.mcp__codex_apps__github_update_ref({repository_full_name:repo,branch_name:branch,sha:cp.commit,force:false}));
 const final=await fetchJSON(url+"/git/ref/heads/"+branch);if(final.object.sha!==cp.commit)throw Error("Cabecera no confirmada");
 const commit=await fetchJSON(url+"/git/commits/"+cp.commit);if(commit.tree.sha!==cp.tree||commit.parents[0].sha!==base)throw Error("Commit no concordante");
 cp.done=true;cp.count=Object.keys(files).length;store("c12publish-"+key,cp);text({repo,commit:cp.commit,files:cp.count,verified:true});
}
const a=load("c12publish-repo"),b=load("c12publish-lab");
for(const [p,h]of Object.entries(a.blobs)){
 if(p.includes("REGISTRO_EVOLUCION_TECNICA_PROYECTO."))continue;
 const q=p.includes("/Inventario-sv/sucesos/")?"laboratorio/tareas-watson/sucesos-sv/"+p.split("/").pop():p.replace("docs/calidad/","laboratorio/tareas-watson/");
 if(b.blobs[q]!==h)throw Error("Espejo diferente "+p);
}
text("Publicación, contenido literal, continuidad de árboles y espejo verificados. Checkout local pendiente de sincronizar.");
