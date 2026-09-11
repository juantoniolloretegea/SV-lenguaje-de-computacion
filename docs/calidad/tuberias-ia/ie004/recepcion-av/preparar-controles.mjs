// Transformación de evidencia pública anterior y fijación de controles de transporte.
// No ejecuta el receptor, no interpreta preguntas nuevas y no consulta la reserva.
import fs from 'node:fs';import path from 'node:path';import crypto from 'node:crypto';
const dir=path.dirname(new URL(import.meta.url).pathname),old=path.resolve(dir,'../semantica-a');
const evidence=process.env.SV_AV_CAPTURA_ANTERIOR;
if(!evidence)throw Error('Indicar captura pública original de RETP-135');
const records=fs.readFileSync(evidence,'utf8').trim().split('\n').map(JSON.parse).filter(x=>x.fase==='semantica');
const cases=JSON.parse(fs.readFileSync(path.join(old,'CASOS_SEMANTICOS_PUBLICOS.json'))).casos;
const table=JSON.parse(fs.readFileSync(path.join(dir,'TABLA_EXPRESIONES.json')));
const hash=x=>crypto.createHash('sha256').update(x).digest('hex');
const cfg={fuente_sha256:'e5073224e257bf43e837274b867325a38171363a7521921a25714e3498c7c290',perfil:'IE004-ES-P2/3-COSTE/1'};
const names=[{1:'LEER',2:'ESCRIBIR'},{1:'CASO-A',2:'CASO-B'},{1:'IGG',2:'IGA',4:'IGM'},{1:'ACTUAL',2:'ANTERIOR'},{1:'VALOR',2:'UNIDAD',4:'ESTADO',8:'FUENTE',16:'ALCANCE'}],keys=['operacion','objeto','parametro','momento','campo'];
fs.mkdirSync(path.join(dir,'publicos','a'),{recursive:true});fs.mkdirSync(path.join(dir,'publicos','v'),{recursive:true});
const aa=[],vv=[];
function wire(id,q,ctx=[1,1,1,1,1]){return {version:'IE004-A-SOLICITUD/1',id:'PUBLICO-'+id,pregunta:q,contexto:Object.fromEntries(keys.map((k,i)=>[k,ctx[i]===0?null:names[i][ctx[i]]]))};}
function addA(id,b,esperado,extra={}){const bytes=Buffer.isBuffer(b)?b:Buffer.from(typeof b==='string'?b:JSON.stringify(b));const file='publicos/a/'+id+'.json';fs.writeFileSync(path.join(dir,file),bytes);aa.push({id,file,bytes:bytes.length,sha256:hash(bytes),esperado,...extra});}
for(const[id,source]of[['A01','S34'],['A02','S35'],['A03','C03'],['A04','S01'],['A05','S26'],['A06','S20'],['A07','S45'],['A08','C01']]){const c=cases.find(x=>x.id===source);addA(id,wire(id,c.pregunta,c.contexto),c.esperado,{fuente:source,vigente:c.vigente});}
const reject={estado:'A_TRANSPORTE_RECHAZADO',literal:null,ruta:null,llamadas_politica:0};
const primary=JSON.parse(fs.readFileSync(path.join(dir,aa[0].file)));
addA('A09',wire('A09','a'.repeat(8193)),reject,{causa:'BYTES_LIMITE'});
addA('A10',Buffer.from([0xff,0xfe]),reject,{causa:'UTF8_INVALIDO'});
addA('A11',JSON.stringify(primary).slice(0,-1),reject,{causa:'TRUNCADO'});
addA('A12',JSON.stringify(primary).replace('{','{"id":"PUBLICO-DUP",'),reject,{causa:'CLAVE_DUPLICADA'});
addA('A13',{...primary,propuesta:{ruta:'IGM'}},reject,{causa:'ESQUEMA_INVALIDO'});
addA('A14',{...primary,contexto:{...primary.contexto,parametro:'HEMOGLOBINA'}},reject,{causa:'ESQUEMA_INVALIDO'});
addA('A15',JSON.stringify(primary).replace('"parametro":"IGG"','"parametro":18446744073709551615'),reject,{causa:'ESQUEMA_INVALIDO'});
addA('A16',JSON.stringify(primary).replace('"parametro":"IGG"','"parametro":18446744073709551616'),reject,{causa:'ENTERO_O_DIMENSION_INVALIDA'});
addA('A17',JSON.stringify(primary).replace('IgG','\\u0049\\u0067\\u0047'),aa[0].esperado,{relacion:'mismos_bytes_decodificados_A01'});
addA('A18',JSON.stringify(primary).replace('IgG','\\ud800'),reject,{causa:'JSON_INVALIDO'});
const base=Buffer.from(JSON.stringify(primary));addA('A19',Buffer.concat([base,Buffer.alloc(65536-base.length,32)]),aa[0].esperado,{frontera:'65536_bytes_transporte'});
addA('A20',Buffer.concat([base,Buffer.alloc(65537-base.length,32)]),reject,{causa:'BYTES_LIMITE',observados:65537});
addA('A21',JSON.stringify(primary)+'{}',reject,{causa:'JSON_INVALIDO'});
addA('A22',wire('A22',cases.find(x=>x.id==='S26').pregunta+' actual'),{estado:'A_TOKENS_LIMITE',literal:null,ruta:null,llamadas_politica:0});
addA('A23',wire('A23','La IgG.',[0,0,0,0,0]),{estado:'CONTEXTO_INSUFICIENTE',literal:null,ruta:null,llamadas_politica:0});
const lot={version:'IE004-A-LOTE-PUBLICO/1',casos:aa.map(x=>({id:x.id,archivo:x.file,bytes:x.bytes,sha256:x.sha256}))};
const lotRaw=JSON.stringify(lot,null,2)+'\n';fs.writeFileSync(path.join(dir,'publicos','LOTE_PUBLICO.json'),lotRaw);const lotHash=hash(lotRaw);
function certificate(source,id){
 const r=records.find(x=>x.id===source),bs=r.bosque_sintactico,visited=new Map(),nodes=[];
 const root=bs.nodos.findIndex(n=>n[0]===table.reglas[0]&&n[1]===0&&n[2]===r.tokens.length);if(root<0)throw Error('Raíz pública ausente');
 const boundary=k=>k===0?0:k===r.tokens.length?Buffer.byteLength(r.original):r.tokens[k][0];
 function take(n){if(visited.has(n))return visited.get(n);const x=bs.nodos[n],f=bs.familias[x[3]],op=table.expresiones[x[0]],hs=[f[1],f[2]].filter(x=>x!==4294967295).map(take),index=nodes.length;
   nodes.push({regla:op.regla,expresion:x[0],alternativa:op.tipo==='ALTERNATIVA'||op.tipo==='REPETICION'?f[0]+1:0,inicio:boundary(x[1]),fin:boundary(x[2]),hijos:hs});visited.set(n,index);return index;}
 const raiz=take(root);return {version:'IE004-P3-V/1',id:'PUBLICO-'+id,solicitudes_sha256:lotHash,certificado:{version:'IE004-DERIVACION-EXP/1',perfil:cfg.perfil,fuente_fijada_sha256:cfg.fuente_sha256,pregunta_sha256:hash(Buffer.from(r.original)),raiz,nodos:nodes}};
}
const valid=certificate('S34','A01'),accent=certificate('S45','A07');
function addV(id,b,estado,causa=null,extra={}){const bytes=Buffer.isBuffer(b)?b:Buffer.from(typeof b==='string'?b:JSON.stringify(b));const file='publicos/v/'+id+'.json';fs.writeFileSync(path.join(dir,file),bytes);vv.push({id,file,bytes:bytes.length,sha256:hash(bytes),estado,causa,original:'A01',cupo:1000000,...extra});}
const good='DERIVACION_SINTACTICA_COMPROBADA';addV('V01','', 'V_AUSENTE');addV('V02',valid,good);
const vr=Buffer.from(JSON.stringify(valid));addV('V03',Buffer.concat([vr,Buffer.alloc(262144-vr.length,32)]),good);
addV('V04','{"version":','V_RECHAZADA','TRUNCADO');addV('V05',Buffer.concat([vr,Buffer.alloc(262145-vr.length,32)]),'V_RECHAZADA','BYTES_LIMITE',{observados:262145});
addV('V06',valid,'V_RECHAZADA','PRESUPUESTO_AGOTADO',{cupo:1});
for(const[id,mutate,causa]of[
 ['V07',v=>v.id='PUBLICO-A02','CORRELACION_INVALIDA'],['V08',v=>v.solicitudes_sha256='0'.repeat(64),'CORRELACION_INVALIDA'],
 ['V09',v=>v.certificado.pregunta_sha256='0'.repeat(64),'CORRELACION_INVALIDA'],['V10',v=>v.certificado.perfil='IE004-ES-P2/2','CORRELACION_INVALIDA'],
 ['V11',v=>{const i=v.certificado.nodos.findIndex(n=>n.hijos.length);v.certificado.nodos[i].hijos[0]=i;},'CERTIFICADO_INVALIDO'],
 ['V12',v=>v.certificado.nodos[0].inicio=1,'CERTIFICADO_INVALIDO'],['V13',v=>v.certificado.nodos[0].expresion=999,'CERTIFICADO_INVALIDO'],
 ['V14',v=>v.certificado.nodos[0].regla=25,'CERTIFICADO_INVALIDO'],['V15',v=>v.certificado.nodos.find(n=>n.alternativa).alternativa=3,'CERTIFICADO_INVALIDO'],
 ['V16',v=>v.certificado.nodos.find(n=>n.hijos.length===2).hijos.reverse(),'CERTIFICADO_INVALIDO'],
 ['V17',v=>v.certificado.nodos.push(structuredClone(v.certificado.nodos[0])),'CERTIFICADO_INVALIDO'],
 ['V18',v=>{while(v.certificado.nodos.length<513)v.certificado.nodos.push(structuredClone(v.certificado.nodos[0]));},'MEMORIA_LIMITE'],
 ['V19',v=>v.certificado.nodos[0].hijos=[0,0,0],'MEMORIA_LIMITE'],
 ['V20',v=>v.certificado.nodos[0].fin='18446744073709551616','ESQUEMA_INVALIDO'],
]){const v=structuredClone(valid);mutate(v);addV(id,v,'V_RECHAZADA',causa);}
addV('V21','['.repeat(98)+'0'+']'.repeat(98),'V_RECHAZADA','PROFUNDIDAD_LIMITE');
addV('V22',JSON.stringify(valid).replace('{','{"id":"PUBLICO-DUP",'),'V_RECHAZADA','CLAVE_DUPLICADA');
addV('V23',Buffer.from([0xff]),'V_RECHAZADA','UTF8_INVALIDO');
addV('V24',accent,good,null,{original:'A07'});
const badAccent=structuredClone(accent);badAccent.certificado.nodos[0].inicio=2;addV('V25',badAccent,'V_RECHAZADA','CERTIFICADO_INVALIDO',{original:'A07'});
const spec={registro:'RETP-2026-136',version:'IE004-AV-CONTROLES/1',lote_sha256:lotHash,fuente_captura_anterior_sha256:hash(fs.readFileSync(evidence)),casos_a:aa,casos_v:vv,repeticiones_rust:3,escenarios_transporte:['V01','V02','V03','V04','V05','V06','V08','V02_DEMORADA','V_SIN_EOF'],relaciones:[['A01','A02'],['A01','A17'],['A01','A19']],memoria_fisica:'NO_MEDIDA',reserva_leida:false};
fs.writeFileSync(path.join(dir,'CONTROLES_PUBLICOS.json'),JSON.stringify(spec,null,2)+'\n');
let rust='struct CasoA {id:&\'static str,bytes:&\'static[u8],estado:&\'static str,literal:Option<&\'static str>,ruta:Option<[u8;5]>,llamadas:u64,vigente:bool,causa:Option<&\'static str>,observados:Option<u64>}\nconst CASOS_A:&[CasoA]=&[\n';
const str=x=>JSON.stringify(x),opt=x=>x==null?'None':'Some('+str(x)+')';
for(const a of aa)rust+=`CasoA{id:${str(a.id)},bytes:include_bytes!(${str(a.file)}),estado:${str(a.esperado.estado)},literal:${opt(a.esperado.literal)},ruta:${a.esperado.ruta?'Some(['+a.esperado.ruta.join(',')+'])':'None'},llamadas:${a.esperado.llamadas_politica},vigente:${a.vigente!==false},causa:${opt(a.causa)},observados:${a.observados?'Some('+a.observados+')':'None'}},\n`;
rust+='];\nstruct CasoV{id:&\'static str,bytes:&\'static[u8],original:usize,estado:&\'static str,causa:Option<&\'static str>,cupo:u64,observados:Option<u64>}\nconst CASOS_V:&[CasoV]=&[\n';
for(const v of vv)rust+=`CasoV{id:${str(v.id)},bytes:include_bytes!(${str(v.file)}),original:${aa.findIndex(a=>a.id===v.original)},estado:${str(v.estado)},causa:${opt(v.causa)},cupo:${v.cupo},observados:${v.observados?'Some('+v.observados+')':'None'}},\n`;
rust+='];\nconst LOTE_SHA:&str="'+lotHash+'";\n';fs.writeFileSync(path.join(dir,'casos_fijados.rs'),rust);
console.log(JSON.stringify({controlesA:aa.length,controlesV:vv.length,lote:lotHash,derivacionPublicaNodos:valid.certificado.nodos.length}));
