
// Continuidad y registro: conservar prefijos históricos exactos y modificar sólo S22.
const B=load("B"),D=B+"/bis-c12",F=load("c12files"),HEAD=load("head"),LAB=load("labhead"),L=load("L");
const now=new Date().toISOString();
const csvparse=s=>{
 const out=[];let row=[],v="",q=false;
 for(let i=0;i<s.length;i++){const c=s[i];
  if(q){if(c==='"'){if(s[i+1]==='"'){v+='"';i++;}else q=false;}else v+=c;}
  else if(c==='"')q=true;
  else if(c===','){row.push(v);v="";}
  else if(c==='\n'||c==='\r'){if(c==='\r'&&s[i+1]==='\n')i++;row.push(v);out.push(row);row=[];v="";}
  else v+=c;
 }if(q)throw Error("CSV incompleto");if(v||row.length){row.push(v);out.push(row);}return out;
};
const csvrow=(r,nl="\n")=>r.map(v=>/[",\r\n]/.test(v)?'"'+v.replaceAll('"','""')+'"':v).join(",")+nl;
const E="docs/calidad/Inventario-sv/sucesos/";
const oldrows=csvparse(load(E+"SUCESOS_SV.csv")),fields=oldrows[0];
const rows=oldrows.slice(1).map(r=>Object.fromEntries(fields.map((k,i)=>[k,r[i]])));
const row=rows.find(r=>r.id==="S22");if(!row)throw Error("S22 ausente");
const next="Consolidar cobertura y pendientes C01–C12 en BIS-02; preparar decisiones de sede BIS-03 y realizar y probar lo justificado en BIS-04.";
Object.assign(row,{
fecha_actualizacion_utc:now,cortes_de_entrada:"Lenguaje "+HEAD+"; laboratorio "+LAB,
resultado:"BIS-C12 preparado: perfiles ES/EN, documentación bilingüe y vías de construcción; veinte escenarios, seis positivos y catorce negativos, once entradas literales. Paridad canónica y procedencia separadas.",
verificacion:"Lectura estática de API y construcción. Cotejo documental JavaScript y GitHub tras desconexión del entorno local; no se ejecutó preparador Python ni nuevos ensayos Rust. Cero escenarios C12 ejecutados. Dos originales ejecutados y veintidós pendientes.",
evidencias:"https://github.com/"+load("repo")+"/blob/main/"+D+"/README.md",
referencia_calidad:"https://github.com/"+load("repo")+"/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-215",
siguiente_accion:next,
observaciones:"C12 es la última familia numerada; BIS-02 sigue abierto. Sin cambio de Rust, semántica, IR, dominio o catálogo. S24 pendiente. Sincronización local pendiente por desconexión; publicación y cotejo por conector GitHub."
});
F[E+"SUCESOS_SV.csv"]=csvrow(fields)+rows.map(r=>csvrow(fields.map(k=>r[k]))).join("");
let md="# Sucesos SV · Estado vigente\n\n[Reglas](README.md) · [CSV](SUCESOS_SV.csv) · [Historial](HISTORIAL_SUCESOS_SV.csv)\n\nFechas UTC. Las revisiones previas permanecen en el historial.\n\n";
for(const r of rows)md+="## "+r.id+" · "+r.actividad+"\n\n"+fields.filter(k=>!["id","actividad"].includes(k)).map(k=>"**"+k+":** "+(r[k]||"—")).join("\n\n")+"\n\n";
F[E+"SUCESOS_SV.md"]=md;
const h=csvparse(load(E+"HISTORIAL_SUCESOS_SV.csv"));const idcol=h[0].indexOf("id");const rev=1+Math.max(...h.slice(1).filter(r=>r[idcol]==="S22").map(r=>Number(r[0])));
F[E+"HISTORIAL_SUCESOS_SV.csv"]=load(E+"HISTORIAL_SUCESOS_SV.csv")+csvrow([String(rev),...fields.map(k=>row[k])]);
const R="docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO";
if(load(R+".csv").includes("RETP-2026-215"))throw Error("RETP duplicado");
F[R+".csv"]=load(R+".csv")+csvrow(["RETP-2026-215",now.slice(0,10),"","PREPARACION_CONTRACTUAL","S22",row.resultado,"Instrucción expresa del autor; continuidad del workflow","Pilares; perfiles SVP; documentación ES/EN; API Rust","C12; Sucesos; historial; Léame primero; estado",row.verificacion,"Contrato y entradas previas",row.observaciones,next,row.estado],"\r\n");
F[R+".md"]=load(R+".md")+"\n\n<a id=\"retp-215\"></a>\n\n### RETP-2026-215 · S22 · BIS-C12: perfiles, documentación y construcción\n\n"+now+". "+row.resultado+" "+row.verificacion+" [Referencia](tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis-c12/README.md). "+next+" "+row.observaciones+"\n";
const state=JSON.parse(load(B+"/ESTADO_WORKFLOW.json"));
Object.assign(state,{ultimo_informe:"bis-c12/README.md",ultimo_retp:"RETP-2026-215",siguiente_accion:next,ampliacion:"BIS-C12: veinte escenarios y once entradas literales; cero ejecutados. Cotejo auxiliar por JavaScript/GitHub tras desconexión local.",preparacion_bis_c12:{contrato:"bis-c12/CONTRATO_PERFILES_DOCUMENTACION_CONSTRUCCION_v0_1.md",banco:"bis-c12/BANCO_PREVIO_v0_1.json",variantes_especificadas:20,variantes_ejecutadas:0,entradas_literales:11,corte_revision:HEAD,alcance:"Paridad, documentación y construcción especificadas; ensayos propios pendientes",sincronizacion_local:"pendiente por desconexión del entorno"}});
F[B+"/ESTADO_WORKFLOW.json"]=JSON.stringify(state,null,2)+"\n";
F[B+"/README.md"]=load(B+"/README.md").replace("## Trabajo vigente: S22 · RETP-2026-214","## Trabajo vigente: S22 · RETP-2026-215").replace("Continúa BIS-C12: perfiles ES/EN, documentación y construcción;","[BIS-C12](bis-c12/README.md) añade veinte escenarios de perfiles, documentación y construcción, once entradas literales, cero ejecutados. Sigue consolidar cobertura y pendientes C01–C12;");
let leame=load(L).replace("**Trabajo vigente: S22 / RETP-2026-214.","**Trabajo vigente: S22 / RETP-2026-215.");
const a=leame.indexOf("**Siguiente objeto material:**"),z=leame.indexOf("\n\n",a);
if(a<0||z<0)throw Error("Marcador ausente");
F[L]=leame.slice(0,a)+"**Siguiente objeto material:** consolidar cobertura y pendientes C01–C12 en BIS-02 y preparar sedes BIS-03. [BIS-C12](../paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis-c12/README.md) distingue perfil fuente, documentación bilingüe y construcción: veinte escenarios, once entradas literales, cero ejecutados. Igualdad semántica y procedencia se cotejan separadamente. Los constructores internos no validan por sí solos; las entradas públicas inspeccionadas aplican guardas. Dos escenarios originales ejecutados y veintidós pendientes. Completar familias documentales no cierra BIS-02 ni el workflow. S24 mantiene GUI diferida. Cotejo documental por JavaScript/GitHub tras desconexión local; sincronización del checkout pendiente."+leame.slice(z);
store("c12files",F);store("csvparsecode",csvparse.toString());store("c12time",now);
