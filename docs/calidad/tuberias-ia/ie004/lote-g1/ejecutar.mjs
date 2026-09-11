import fs from 'node:fs';import path from 'node:path';import crypto from 'node:crypto';import assert from 'node:assert/strict';import {spawnSync} from 'node:child_process';
const [rustArg,outArg]=process.argv.slice(2);assert(rustArg&&outArg);const rust=path.resolve(rustArg),out=path.resolve(outArg),dir=path.dirname(new URL(import.meta.url).pathname),parent=path.dirname(dir);
assert(!fs.existsSync(out),'No sobrescribir campaña');fs.mkdirSync(out,{recursive:true});
const sha=b=>crypto.createHash('sha256').update(b).digest('hex');
const read=p=>fs.readFileSync(path.join(dir,p));const save=(p,x)=>fs.writeFileSync(path.join(out,p),typeof x==='string'||Buffer.isBuffer(x)?x:JSON.stringify(x,null,2)+'\n');
const plan=JSON.parse(read('PLAN_FIJADO.json')),lote=read('../lote-p3/LOTE_SINTETICO_PUBLICO.json'),mont=read('../lote-p3/MONTAJES_PUBLICOS.json'),esperados=read('../lote-p3/PROCEDENCIA_Y_ESPERADOS.json');
assert.equal(sha(lote),plan.entrada_fijada.lote_sha256);assert.equal(sha(mont),plan.entrada_fijada.montaje_sha256);assert.equal(sha(esperados),plan.entrada_fijada.esperados_sha256);
const paths=['lib.rs','tests.rs','PLAN_FIJADO.json','ejecutar.mjs','../lote-p3/adaptador.rs','../lote-p3/LOTE_SINTETICO_PUBLICO.json','../lote-p3/MONTAJES_PUBLICOS.json','../lote-p3/PROCEDENCIA_Y_ESPERADOS.json',...['semantica-a','recepcion-av'].flatMap(n=>fs.readdirSync(path.join(parent,n)).filter(p=>p.endsWith('.rs')).map(p=>'../'+n+'/'+p)),...['lib.rs','av.rs','json_vista.rs'].map(p=>'../recibo-g1/candidata/'+p)].sort();
const manifest=Buffer.from(JSON.stringify({version:'IE004-LOTE-G1-FIJACION/1',archivos:paths.map(p=>{const b=read(p);return{path:p,bytes:b.length,sha256:sha(b)}})},null,2)+'\n');save('FIJACION_PREVIA.json',manifest);save('PLAN_FIJADO.json',read('PLAN_FIJADO.json'));
const result={registro:'RETP-2026-145',estado:'EN_CURSO',fuentes_sha256:sha(manifest),procesos:[],configuraciones:[],reserva_leida:false,modelo_invocado:false,alcance:'Enlace público nativo intraproceso; no inscripción/Frame ni persistencia durable'};
function run(id,cmd,args,env={}){const r=spawnSync(cmd,args,{timeout:60000,maxBuffer:8*1024*1024,env:{...process.env,...env}});const a=r.stdout??Buffer.alloc(0),b=r.stderr??Buffer.alloc(0);save(id+'.stdout',a);save(id+'.stderr',b);const x={id,cmd,args,exit_code:r.status,signal:r.signal,error:r.error?String(r.error):null,stdout_bytes:a.length,stdout_sha256:sha(a),stderr_bytes:b.length,stderr_sha256:sha(b)};save(id+'.json',x);result.procesos.push(x);return{...x,stdout:a};}
try{
 const v=run('rustc-version',rust,['-vV']);assert.equal(v.exit_code,0);assert.match(v.stdout.toString(),/^rustc 1\.98\.0 /);
 const original=JSON.parse(lote),montajes=JSON.parse(mont),oracle=JSON.parse(esperados);
 for(const mode of ['debug','release']){
  const flags=['-C','opt-level='+(mode==='debug'?0:3),'-C','overflow-checks='+(mode==='debug'?'yes':'no')];const lib=path.join(out,'libg1-'+mode+'.rlib'),bin=path.join(out,'lote-g1-'+mode+'.bin');
  const g=run('compilar-g1-'+mode,rust,['--edition=2021','--crate-type=lib','--crate-name=g1',path.join(parent,'recibo-g1/candidata/lib.rs'),...flags,'-o',lib]);
  const r=g.exit_code===0?run('compilar-enlace-'+mode,rust,['--edition=2021','--test',path.join(dir,'lib.rs'),'--extern','g1='+lib,...flags,'-o',bin]):null;
  if(!r||r.exit_code!==0){result.configuraciones.push({modo:mode,compila:false});continue;}
  const capture=path.join(out,'capturas-'+mode);fs.mkdirSync(capture);const binhash=sha(fs.readFileSync(bin));save('REALIZACION-'+mode+'.json',{fuentes_sha256:sha(manifest),binario_sha256:binhash,g1_rlib_sha256:sha(fs.readFileSync(lib))});
  const t=run('test-'+mode,bin,['--nocapture','--test-threads=1'],{LG1_FUENTES:sha(manifest),LG1_BINARIO:binhash,LG1_CAPTURAS:capture});
  const ids=[...t.stdout.toString().matchAll(/TESTIGO_LOTE_G1:(LG1-\d+):CONFORME/g)].map(m=>m[1]);const entry={modo:mode,compila:true,binario_sha256:binhash,exit_code:t.exit_code,testigos:ids,recorrido:[],conforme:false};
  try{
   const get=n=>fs.readFileSync(path.join(capture,n));assert(get('lote-original.json').equals(lote));assert(get('montaje-original.json').equals(mont));const index=JSON.parse(get('INDICE.json'));assert.equal(index.lote_sha256,sha(lote));assert.equal(index.montaje_sha256,sha(mont));assert.equal(index.casos.length,24);
   for(let i=0;i<24;i++){
    const id='P3-'+String(i+1).padStart(2,'0'),row=index.casos[i],req=get(id+'.solicitud.json'),body=get(id+'.cuerpo.json'),frame=get(id+'.marco'),trace=JSON.parse(get(id+'.traza'));
    assert.equal(row.id,id);assert.equal(row.posicion,i);assert.equal(row.vigente,montajes.casos[i].vigente);assert.equal(row.v,'NO_SOLICITADA');assert.equal(row.solicitud_sha256,sha(req));assert.equal(row.cuerpo_sha256,sha(body));
    assert.deepEqual(JSON.parse(req),{version:'IE004-A-SOLICITUD/1',...original.casos[i]});assert.deepEqual(JSON.parse(lote.subarray(row.inicio,row.fin)),original.casos[i]);assert.equal(sha(body),oracle.casos[i].cuerpo_esperado_sha256);assert.deepEqual(JSON.parse(body),oracle.casos[i].cuerpo_esperado);
    assert.equal(frame.subarray(0,8).toString(),'SVAC0001');assert.equal(Number(frame.readBigUInt64LE(8)),body.length);assert.equal(frame.length,body.length+48);assert(frame.subarray(16,-32).equals(body));assert.equal(frame.subarray(-32).toString('hex'),sha(body));
    assert.equal(trace.id,id);assert.equal(trace.original,original.casos[i].pregunta);assert.equal(trace.original_transporte_sha256,sha(req));assert.equal(trace.cuerpo_sha256,sha(body));
    entry.recorrido.push({id,conforme:true,cuerpo_sha256:sha(body),vigente:row.vigente,ambito:row.ambito,ordinal:row.ordinal});
   }
   assert.equal(new Set(index.casos.map(x=>x.ambito+':'+x.ordinal)).size,24);assert(index.g1_reserva_agregada<=plan.recursos.reservas_g1_agregadas_bytes);entry.cuentas=index;
   const total=fs.readdirSync(capture).reduce((n,p)=>n+fs.statSync(path.join(capture,p)).size,0);assert(total<=plan.recursos.capturas_ensayo_bytes);entry.capturas_bytes=total;entry.conforme=t.exit_code===0&&new Set(ids).size===12;
  }catch(e){entry.error_cotejo=String(e);}
  result.configuraciones.push(entry);save('RESULTADO_PARCIAL.json',result);
 }
 if(result.configuraciones.length===2&&result.configuraciones.every(x=>x.conforme)){
  const a=path.join(out,'capturas-debug'),b=path.join(out,'capturas-release'),files=fs.readdirSync(a).sort();assert.deepEqual(files,fs.readdirSync(b).sort());for(const p of files)assert(fs.readFileSync(path.join(a,p)).equals(fs.readFileSync(path.join(b,p))),p);result.paridad={archivos:files.length,identicos:true};result.estado='CONFORME_ENLACE_PUBLICO_INTRAPROCESO';
 }else result.estado='FALLO';
 for(const f of JSON.parse(manifest).archivos){const b=read(f.path);assert.equal(sha(b),f.sha256);assert.equal(b.length,f.bytes);}result.fuentes_fijadas_intactas=true;
}catch(e){result.estado='FALLO_OBSERVADOR_O_EJECUCION';result.error=String(e);}
save('RESULTADO.json',result);
function walk(p){return fs.readdirSync(p,{withFileTypes:true}).flatMap(f=>f.isDirectory()?walk(path.join(p,f.name)):[path.join(p,f.name)]);}
const files=walk(out).filter(p=>!p.endsWith('.bin')&&!p.endsWith('.rlib'));save('CAPSULA.json',{version:'IE004-LOTE-G1-CAPTURAS/1',archivos:files.map(p=>{const b=fs.readFileSync(p);return{ruta:path.relative(out,p),bytes:b.length,sha256:sha(b),base64:b.toString('base64')}})});
console.log(JSON.stringify({estado:result.estado,configuraciones:result.configuraciones.map(x=>({modo:x.modo,compila:x.compila,testigos:x.testigos?.length,recorrido:x.recorrido?.length,conforme:x.conforme,error:x.error_cotejo})),paridad:result.paridad}));if(result.estado!=='CONFORME_ENLACE_PUBLICO_INTRAPROCESO')process.exitCode=1;
