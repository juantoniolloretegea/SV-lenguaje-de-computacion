import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {parseDocumentaryJson} from './json-estricto.mjs';
const here=path.dirname(new URL(import.meta.url).pathname);
const [bin,out,packetFile,deliveryFile,oracleFile,commitmentFile]=process.argv.slice(2);
if(!bin||!out)throw Error('USO: ejecutar.mjs bin salida [solicitudes entrega oraculo compromiso]');
fs.mkdirSync(out,{recursive:true});
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const save=(n,v)=>fs.writeFileSync(path.join(out,n),JSON.stringify(v,null,2)+'\n');
const keys=['operacion','objeto','parametro','momento','campo'];
const hayTime=fs.existsSync('/usr/bin/time');
const route=(parametro='IGG',momento='ACTUAL',campo='VALOR',objeto='CASO-A',operacion='LEER')=>({operacion,objeto,parametro,momento,campo});
const ctx=route();
const diag=estado=>JSON.stringify({estado,contenido:null,base:'K-IE004/1',politica:'P-IE004/1'})+'\n';
const data=(ruta,contenido)=>JSON.stringify({estado:'DATO_EXPERIMENTAL',contenido,objeto:ruta.objeto,parametro:ruta.parametro,momento:ruta.momento,campo:ruta.campo,fuente:'FICCION-IE004/1',alcance:'Registro artificial; no acredita concentración ni condición clínica.',base:'K-IE004/1',politica:'P-IE004/1'})+'\n';
const base=JSON.parse(fs.readFileSync(path.join(here,'base.json')));
function expected(r,d,live=true){if(d)return diag(d);if(r.operacion!=='LEER')return diag('OPERACION_DENEGADA');if(r.objeto!=='CASO-A'||r.parametro==='IGM')return diag('ACCESO_DENEGADO');if(!live)return diag('PERMISO_REVOCADO');const rec=base.registros.find(x=>x.objeto===r.objeto&&x.parametro===r.parametro&&x.momento===r.momento);if(!rec)throw Error('ORACULO_REFERENCIA');return data(r,rec[r.campo.toLowerCase()]);}
const u32=n=>{const b=Buffer.alloc(4);b.writeUInt32LE(n);return b;};
const str=s=>{const b=Buffer.from(s??'');return Buffer.concat([u32(b.length),b]);};
function wire(rows){return Buffer.concat([u32(rows.length),...rows.flatMap(x=>[str(x.pregunta),...keys.map(k=>str(x.contexto[k])),u32(x.vigente===false?0:1),str(x.diagnostico),...keys.map(k=>str(x.ruta?.[k])),...keys.map(k=>str(x.apoyos?.[k]))])]);}
let sequence=0;const medidas=[];
function run(dest,input,label,exit=0){
 const id=String(++sequence).padStart(4,'0'),timing=path.join(out,'medida-'+id+'.txt');
 const cmd=dest==='nativo'?[path.join(bin,'receptor')]:[process.execPath,path.join(here,'wasi.mjs'),path.join(bin,'receptor.wasm')];
 const start=process.hrtime.bigint();
 const comando=hayTime?['/usr/bin/time','-f','%e %U %S %M','-o',timing,...cmd]:cmd;
 const p=spawnSync(comando[0],comando.slice(1),{input,timeout:5000,maxBuffer:1048576,env:{PATH:process.env.PATH,LANG:'C.UTF-8'}});
 const ns=Number(process.hrtime.bigint()-start);const raw=fs.existsSync(timing)?fs.readFileSync(timing,'utf8').trim():'';
 medidas.push({id,destino:dest,etiqueta:label,entrada_sha256:hash(input),salida_sha256:hash(p.stdout??Buffer.alloc(0)),duracion_ns:ns,time:hayTime?raw:null,instrumento:hayTime?'GNU_TIME':'RELOJ_MONOTONICO_CPU_RSS_NO_OBSERVABLES',status:p.status,signal:p.signal,error:p.error?.message??null});
 assert.equal(p.error,undefined,label+' error');assert.equal(p.status,exit,label+' exit '+p.stderr?.toString());return p.stdout;
}
const supports=r=>Object.fromEntries(keys.map(k=>[k,'@contexto.'+k]));
const specimen=(r=route(),d=null,vigente=true)=>({pregunta:'Control de receptor',contexto:r??ctx,ruta:r,diagnostico:d,apoyos:r?supports(r):{},vigente});
const fixtures=[];
// Oráculos de control independientes de la función Rust, no derivados de su salida.
for(const [r,content] of [[route(),'8.40'],[route('IGG','ANTERIOR'),'7.10'],[route('IGA'),'1.25'],[route('IGG','ACTUAL','ESTADO'),'1'],[route('IGG','ANTERIOR','ESTADO'),'0'],[route('IGA','ACTUAL','ESTADO'),'U'],[route('IGG','ACTUAL','UNIDAD'),'unidad-simulada'],[route('IGG','ACTUAL','FUENTE'),'FICCION-IE004/1'],[route('IGG','ACTUAL','ALCANCE'),'Registro artificial; no acredita concentración ni condición clínica.']])fixtures.push({entrada:specimen(r),esperado:data(r,content)});
for(const [row,state] of [[specimen(route('IGM')),'ACCESO_DENEGADO'],[specimen(route('IGG','ACTUAL','VALOR','CASO-B')),'ACCESO_DENEGADO'],[specimen(route('IGG','ACTUAL','VALOR','CASO-A','ESCRIBIR')),'OPERACION_DENEGADA'],[specimen(route(),null,false),'PERMISO_REVOCADO'],[specimen(null,'CONTEXTO_INSUFICIENTE'),'CONTEXTO_INSUFICIENTE'],[specimen(null,'FUERA_DE_COBERTURA'),'FUERA_DE_COBERTURA'],[specimen(null,'OPERACION_NO_ADMITIDA'),'OPERACION_NO_ADMITIDA'],[specimen(route('OTRO')),'REFERENCIA_INVALIDA'],[{...specimen(),apoyos:{...supports(ctx),parametro:'no está en la pregunta'}},'APOYO_INVALIDO'],[{...specimen(),ruta:route('IGA')},'APOYO_INVALIDO'],[specimen(null,'U'),'PROPUESTA_INVALIDA']])fixtures.push({entrada:row,esperado:diag(state)});
save('oraculos-control.json',fixtures);
const control=wire(fixtures.map(x=>x.entrada)),want=Buffer.from(fixtures.map(x=>x.esperado).join(''));
const malformed=[{b:Buffer.alloc(0),s:'TRAMA_INCOMPLETA'},{b:u32(0),s:'CANTIDAD_INVALIDA'},{b:u32(257),s:'CANTIDAD_INVALIDA'},{b:Buffer.concat([control,Buffer.from([0])]),s:'TRAMA_SOBRANTE'},{b:Buffer.concat([u32(1),u32(2),Buffer.from([0xc3,0x28])]),s:'UTF8_INVALIDO'},{b:Buffer.alloc(262145),s:'TRAMA_LIMITE'}];
let result;
try{
 for(let i=0;i<10;i++)for(const dest of (i%2?['wasi','nativo']:['nativo','wasi']))assert.deepEqual(run(dest,control,'control-'+i),want);
 for(const x of malformed)for(const dest of ['nativo','wasi'])assert.equal(run(dest,x.b,x.s,2).toString(),diag(x.s));
 // Testigo del límite semántico: una cita literal no prueba que la ruta respete la negación.
 const semantic={pregunta:'No consulte IgG; consulte IgA.',contexto:ctx,ruta:ctx,apoyos:{...supports(ctx),parametro:'IgG'},diagnostico:null};
 const witness=[];
 for(const dest of ['nativo','wasi']){const b=run(dest,wire([semantic]),'sustitucion-semantica');assert.equal(b.toString(),data(ctx,'8.40'));assert.notEqual(b.toString(),data(route('IGA'),'1.25'));witness.push({destino:dest,salida:b.toString(),esperado_por_peticion:data(route('IGA'),'1.25')});}
 save('LIMITE-SEMANTICO.json',{estado:'BRECHA_CONFIRMADA_EN_RECEPTOR_EXPERIMENTAL',causa:'Tipo, permiso y cita literal válidos no prueban identificación semántica correcta.',entrada:semantic,observaciones:witness,promocion_al_sv:false});
 let capture=null;
 if(packetFile||deliveryFile||oracleFile||commitmentFile){
  if(![packetFile,deliveryFile,oracleFile,commitmentFile].every(Boolean))throw Error('RECEPCION_REQUIERE_CUATRO_ARCHIVOS');
  const pb=fs.readFileSync(packetFile),db=fs.readFileSync(deliveryFile),ob=fs.readFileSync(oracleFile),cb=fs.readFileSync(commitmentFile);
  if(db.length>262144)throw Error('ENTREGA_LIMITE');
  const p=parseDocumentaryJson(pb),d=parseDocumentaryJson(db),o=parseDocumentaryJson(ob),c=parseDocumentaryJson(cb);
  assert.equal(hash(ob),c.sha256);assert.equal(ob.length,c.bytes);assert.equal(hash(pb),c.solicitudes_sha256);assert.equal(hash(pb),o.packet_sha256);
  const exact=(x,ks)=>assert.deepEqual(Object.keys(x).sort(),[...ks].sort());
  exact(d,['version','modelo','sesion','configuracion','respuestas']);assert.equal(d.version,'IE-004/1');
  for(const key of ['modelo','sesion','configuracion'])assert.ok(typeof d[key]==='string'&&d[key].length>0&&d[key].length<=1024);
  assert.equal(d.respuestas.length,p.casos.length);assert.equal(o.expectativas.length,p.casos.length);
  const ids=new Set();const rows=[];const semanticResults=[];
  for(const [i,entry]of d.respuestas.entries()){
   exact(entry,['id','ruta','diagnostico','apoyos']);const source=p.casos.find(x=>x.id===entry.id),g=o.expectativas.find(x=>x.id===entry.id);assert.ok(source&&g&&!ids.has(entry.id));ids.add(entry.id);
   if(entry.ruta===null){assert.equal(typeof entry.diagnostico,'string');exact(entry.apoyos,[]);}else{exact(entry.ruta,keys);exact(entry.apoyos,keys);assert.equal(entry.diagnostico,null);for(const k of keys){assert.equal(typeof entry.ruta[k],'string');assert.equal(typeof entry.apoyos[k],'string');}}
   const same=JSON.stringify(entry.ruta&&keys.map(k=>entry.ruta[k]))===JSON.stringify(g.ruta&&keys.map(k=>g.ruta[k]))&&entry.diagnostico===g.diagnostico;
   rows.push({...source,...entry,vigente:g.permiso_vigente});semanticResults.push({id:entry.id,identificacion_correcta:same,esperado:expected(g.ruta,g.diagnostico,g.permiso_vigente)});
  }
  const input=wire(rows);const outputs={};for(const dest of ['nativo','wasi']){let first;for(let i=0;i<3;i++){const b=run(dest,input,'captura-'+i);if(first)assert.deepEqual(b,first);else first=b;}outputs[dest]=first;fs.writeFileSync(path.join(out,'captura-'+dest+'.jsonl'),first);}
  assert.deepEqual(outputs.nativo,outputs.wasi);const lines=outputs.nativo.toString().trimEnd().split('\n');assert.equal(lines.length,rows.length);
  const findings=rows.map((x,i)=>({...semanticResults[i],salida:lines[i]+'\n',respuesta_correcta:lines[i]+'\n'===semanticResults[i].esperado}));
  const byId=Object.fromEntries(findings.map(x=>[x.id,x]));const relations={};for(const [kind,groups]of Object.entries(o.clases)){relations[kind]=groups.map(ids=>({ids,conforme:kind==='contrastes'?new Set(ids.map(id=>byId[id].salida)).size===ids.length:new Set(ids.map(id=>byId[id].salida)).size===1}));}
  const conformes=findings.filter(x=>x.identificacion_correcta&&x.respuesta_correcta).length;
  capture={estado:conformes===rows.length?'CONFORME_EN_LOTE_NO_PROMOCION':'FALLOS_EN_LOTE',casos:rows.length,conformes,identificacion_correcta:findings.filter(x=>x.identificacion_correcta).length,respuesta_correcta:findings.filter(x=>x.respuesta_correcta).length,entrega_sha256:hash(db),solicitudes_sha256:hash(pb),oraculo_sha256:hash(ob),modelo_declarado:d.modelo,sesion_declarada:d.sesion,configuracion_declarada:d.configuracion,hallazgos:findings,relaciones:relations,paridad_bytes:true,repeticiones_receptor_por_destino:3};
  save('RECEPCION.json',capture);fs.copyFileSync(deliveryFile,path.join(out,'entrega-original.json'));fs.copyFileSync(packetFile,path.join(out,'solicitudes-originales.json'));fs.copyFileSync(oracleFile,path.join(out,'ORACULO-ABIERTO.json'));
 }
 result={estado:'RECEPTOR_COMPROBADO_BRECHA_SEMANTICA_CONFIRMADA',controles:fixtures.length,negativos_transporte:malformed.length,repeticiones_por_control_y_destino:10,paridad_bytes:true,frontera_operativa_sv:'NO_INTEGRADA',aislamiento_material:'NO_ACREDITADO',modelo_real_ejecutado_por_este_proceso:false,captura:!!capture,fuente_commit:process.env.FUENTE_COMMIT??'LOCAL',ejecutor_commit:process.env.EJECUTOR_COMMIT??'LOCAL',run:process.env.GITHUB_RUN_ID??'LOCAL',entorno:{node:process.version,platform:os.platform(),arch:os.arch(),cpu:os.cpus()[0]?.model,cpus:os.availableParallelism(),memoria:os.totalmem()},binarios:Object.fromEntries(['receptor','receptor.wasm'].map(f=>[f,{bytes:fs.statSync(path.join(bin,f)).size,sha256:hash(fs.readFileSync(path.join(bin,f)))}])),medidas:medidas.length};
 save('RESULTADO.json',result);console.log(JSON.stringify(result));
}finally{save('medidas.json',medidas);}
