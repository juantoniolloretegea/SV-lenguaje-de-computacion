
// Cotejo auxiliar documental: no sustituye ensayos Rust.
const F=load("c12files"),B=load("B"),D=B+"/bis-c12",assert=(x,m)=>{if(!x)throw Error(m);};
const parse=eval("("+load("csvparsecode")+")");
assert(JSON.stringify(parse('a,b\r\n"x,y","z""q"\r\n'))===JSON.stringify([["a","b"],["x,y",'z"q']]),"autocontrol CSV");
const bank=JSON.parse(F[D+"/BANCO_PREVIO_v0_1.json"]),cases=bank.casos;
assert(cases.length===20&&bank.variantes===20&&bank.ejecutadas===0,"conteo");
assert(new Set(cases.map(x=>x.id)).size===20,"ids");
assert(cases.filter(x=>x.clase==="positivo").length===6&&cases.filter(x=>x.clase==="negativo").length===14,"clases");
const ent=load("c12entries");
assert(Object.keys(ent).length===11,"entradas");
for(const c of cases){assert(c.estado==="ESPECIFICADO_NO_EJECUTADO"&&c.resultado_observado===null,"observado");for(const p of c.archivos)assert(F[D+"/"+p]!==undefined,"referencia "+p);}
const literals=s=>[...s.matchAll(/"([^"]*)"/g)].map(m=>m[1]);
assert(JSON.stringify(literals(ent["base.en.svp"]))===JSON.stringify(literals(ent["base.es.svp"])),"cadenas");
assert(ent["dato_distinto.es.svp"]===ent["base.es.svp"].replace("codomain codominio: alfa","codominio: alfa"),"mutante datos");
assert(ent["orden_distinto.es.svp"]===ent["base.es.svp"].replace("[Cero,Uno,U,","[Uno,Cero,U,"),"mutante orden");
assert(ent["perfil_mezclado.es.svp"]===ent["base.es.svp"].replace("sea Evaluacion","let Evaluacion"),"mutante perfil");
assert(ent["dimension_invalida.en.svp"]===ent["base.en.svp"].replace("b: 4","b: 2"),"mutante dimensión");
const o=JSON.parse(F[D+"/ORACULO_CANONICO.json"]);assert(o.anclas_independientes.vector.length===16&&o.anclas_independientes.b**2===16,"oráculo");
assert(JSON.stringify(literals(ent["base.en.svp"]))===JSON.stringify(o.anclas_independientes.textos),"anclas texto");
const E="docs/calidad/Inventario-sv/sucesos/",R="docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO";
for(const p of [E+"HISTORIAL_SUCESOS_SV.csv",R+".csv",R+".md"])assert(F[p].startsWith(load(p)),"prefijo "+p);
const old=parse(load(E+"SUCESOS_SV.csv")),neo=parse(F[E+"SUCESOS_SV.csv"]);
assert(JSON.stringify(old.filter(x=>x[0]!=="S22"))===JSON.stringify(neo.filter(x=>x[0]!=="S22")),"otros sucesos");
const s24=neo.find(r=>r[0]==="S24"),head=neo[0];assert(s24[1]==="pendiente"&&s24[head.indexOf("fecha_inicio_utc")]===""&&s24[head.indexOf("fecha_fin_utc")]==="","GUI");
const oldstate=JSON.parse(load(B+"/ESTADO_WORKFLOW.json")),state=JSON.parse(F[B+"/ESTADO_WORKFLOW.json"]);
assert(state.escenarios_ejecutados===2&&state.escenarios_pendientes===22&&state.ultimo_retp==="RETP-2026-215","estado");
assert(JSON.stringify(state.etapas)===JSON.stringify(oldstate.etapas),"etapas");
for(const k of Object.keys(oldstate).filter(k=>k.startsWith("preparacion_bis_")))assert(JSON.stringify(state[k])===JSON.stringify(oldstate[k]),"familia previa");
assert(F[B+"/README.md"].includes("[BIS-C12](bis-c12/README.md)")&&F[load("L")].includes("RETP-2026-215"),"relevo");
const before=load("repotree").tree.filter(x=>x.type==="blob");const existing=Object.keys(F).filter(p=>before.some(x=>x.path===p));
assert(existing.length===8&&existing.every(p=>!p.includes("/bis-c")),"alcance archivos existentes");
assert(Object.keys(F).every(p=>p.startsWith("docs/calidad/")),"alcance documental");
for(const [p,t]of Object.entries(F)){
 if(p.endsWith(".json"))JSON.parse(t);
 if(p.startsWith(D+"/")&&p.endsWith(".md"))for(const m of t.matchAll(/\]\(([^)]+)\)/g)){
  const q=m[1];if(!q.startsWith("http"))assert(F[p.slice(0,p.lastIndexOf("/")+1)+q]!==undefined||q==="VERIFICACION_DOCUMENTAL.json","enlace "+q);
 }
}
F[D+"/VERIFICACION_DOCUMENTAL.json"]=JSON.stringify({estatuto:"Cotejo auxiliar JavaScript de documentos; no pruebas Rust ni modelos",corte:load("head"),comprobaciones:["20 casos únicos, 6 positivos y 14 negativos; todos sin observado","11 entradas y referencias presentes; cambios discriminantes literales cotejados","Cadenas ES/EN iguales; anclas de 16 posiciones declaradas","Prefijos históricos conservados; sólo S22 cambia; S24 pendiente","Ocho documentos de continuidad existentes modificados; fuentes Rust y familias previas intactas","JSON y enlaces internos cotejados"],ensayos_c12_ejecutados:0,limites:["No compilación Rust ni interpretación funcional SVP","No revisión automática de significado de la prosa","No actividad de IA ensayada","Entorno local desconectado; sincronización pendiente"],publicacion:"Verificación posterior del árbol, blobs y ref mediante publicar.js"},null,2)+"\n";
store("c12files",F);text("Cotejo documental conforme: 20 escenarios, 11 entradas, 0 ensayos funcionales C12.");
