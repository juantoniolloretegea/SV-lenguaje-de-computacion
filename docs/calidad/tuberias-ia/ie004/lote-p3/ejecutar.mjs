import fs from 'node:fs';import path from 'node:path';import crypto from 'node:crypto';import zlib from 'node:zlib';import assert from 'node:assert/strict';import {spawn} from 'node:child_process';
const dir=path.dirname(new URL(import.meta.url).pathname),out=path.join(dir,'resultados-1'),av=path.join(dir,'../recepcion-av');
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
if(fs.existsSync(out))throw Error('La campaña ya existe; se conserva sin sobrescribir');
const fix=JSON.parse(fs.readFileSync(path.join(dir,'FIJACION_PREVIA.json'))),spec=JSON.parse(fs.readFileSync(path.join(dir,'CONTROLES_PUBLICOS.json'))),orig=JSON.parse(fs.readFileSync(path.join(dir,'PROCEDENCIA_Y_ESPERADOS.json')));
for(const f of fix.archivos){const b=fs.readFileSync(path.join(dir,f.ruta));assert.equal(b.length,f.bytes);assert.equal(hash(b),f.sha256);}
const zip=Buffer.from(fs.readFileSync(path.join(dir,'ENTRADAS_PUBLICAS.json.gz.b64'),'utf8').trim(),'base64');assert.equal(hash(zip),spec.paquete.gzip_sha256);
const raw=zlib.gunzipSync(zip,{maxOutputLength:spec.paquete.bytes});assert.equal(hash(raw),spec.paquete.sha256);const inputs=new Map(JSON.parse(raw).map(x=>[x.id,Buffer.from(x.base64,'base64')]));
for(const s of spec.casos){assert.equal(inputs.get(s.id).length,s.bytes);assert.equal(hash(inputs.get(s.id)),s.sha256);}
const rust=process.env.SV_LOTE_RUSTC;if(!rust||!process.env.SV_LOTE_CORTE_CALIDAD||!process.env.SV_LOTE_CORTE_LAB)throw Error('Faltan toolchain/cortes fijados');
fs.mkdirSync(out);function save(p,x){fs.writeFileSync(path.join(out,p),Buffer.isBuffer(x)||typeof x==='string'?x:JSON.stringify(x,null,2)+'\n');}
save('CORTES.json',{preparacion:'RETP-2026-138',calidad:process.env.SV_LOTE_CORTE_CALIDAD,laboratorio:process.env.SV_LOTE_CORTE_LAB,fijacion_sha256:hash(fs.readFileSync(path.join(dir,'FIJACION_PREVIA.json'))),reserva_abierta:false});
async function run(id,cmd,args,input=Buffer.alloc(0),timeout=30000,maxOut=2*1024*1024+48){
 const start=Date.now(),c=spawn(cmd,args,{cwd:dir,stdio:['pipe','pipe','pipe'],env:{...process.env}}),a=[],e=[];let na=0,ne=0,limited=false,timed=false,err=null;
 const timer=setTimeout(()=>{timed=true;c.kill('SIGKILL')},timeout);
 c.stdout.on('data',b=>{na+=b.length;if(na>maxOut){limited=true;c.kill('SIGKILL')}else a.push(b)});c.stderr.on('data',b=>{ne+=b.length;if(ne>262144){limited=true;c.kill('SIGKILL')}else e.push(b)});
 c.stdin.on('error',x=>{if(!['EPIPE','ERR_STREAM_DESTROYED'].includes(x.code))err=String(x)});c.on('error',x=>{err=String(x)});c.stdin.end(input);
 return await new Promise(resolve=>c.on('close',(code,signal)=>{clearTimeout(timer);const stdout=Buffer.concat(a),stderr=Buffer.concat(e);const record={id,cmd,args,inicio:new Date(start).toISOString(),fin:new Date().toISOString(),duracion_ms:Date.now()-start,exit_code:code,signal,timeout:timed,salida_excedida:limited,error:err,stdin_bytes:input.length,stdin_sha256:hash(input),stdout_bytes:stdout.length,stdout_sha256:hash(stdout),stderr_bytes:stderr.length,stderr_sha256:hash(stderr)};save(id+'.stdout',stdout);save(id+'.stderr',stderr);save(id+'.json',record);resolve({record,stdout,stderr});}));
}
function frame(raw,magic,max){assert(raw.length>=48);assert.equal(raw.subarray(0,8).toString(),magic);const n=raw.readBigUInt64LE(8);assert(n<=BigInt(max));assert.equal(raw.length,Number(n)+48);const body=raw.subarray(16,-32);assert.equal(raw.subarray(-32).toString('hex'),hash(body));return body;}
function trace(raw){const xs=raw.toString().split('\n').filter(x=>x.startsWith('{')).map(JSON.parse);assert.equal(xs.length,1);return xs[0];}
const version=await run('rustc-version',rust,['-vV']);assert.equal(version.record.exit_code,0);assert.match(version.stdout.toString(),/^rustc 1\.98\.0 /);
save('ENTORNO.json',{rust:version.stdout.toString(),node:process.version,plataforma:process.platform,arquitectura:process.arch,wasi:{env:{},preopens:{},max_memory:67108864},instrumentacion:'Tiempo de proceso y contadores. Sin RSS, pila, CPU o evaluación P4/P5.'});
const results=[];
for(const target of ['nativo','wasi'])for(const profile of ['debug','release']){
 const id=target+'-'+profile,flags=['--edition=2021','-C','opt-level='+(profile==='debug'?0:3),'-C','overflow-checks='+(profile==='debug'?'yes':'no'),'-C','panic=abort'];if(target==='wasi')flags.push('--target','wasm32-wasip1','-C','link-arg=--max-memory=67108864');
 const bins={},tests=[],flows=[];let compilation=true;
 for(const [name,source]of [['adaptador','main.rs'],['receptor-a','../recepcion-av/main.rs']]){const file=path.join(out,id+'-'+name+(target==='wasi'?'.wasm':'.bin'));const r=await run(id+'-compilar-'+name,rust,[...flags,source,'-o',file],undefined,120000);if(r.record.exit_code!==0){compilation=false;break;}const bytes=fs.readFileSync(file);bins[name]={file,bytes:bytes.length,sha256:hash(bytes)};}
 if(!compilation){results.push({id,estado:'FALLO_COMPILACION',bins});continue;}
 const invoke=(tag,name,args,input,max)=>run(id+'-'+tag,target==='nativo'?bins[name].file:process.execPath,target==='nativo'?args:[path.join(av,'wasi.mjs'),bins[name].file,...args],input,30000,max);
 const seen=new Map();
 for(let rep=1;rep<=3;rep++)for(const s of spec.casos){
  const b=inputs.get(s.id),r=await invoke(s.id+'-r'+rep,'adaptador',[s.huella_argumento],b);let good=r.record.exit_code===s.exit_code&&!r.record.timeout&&!r.record.error&&!r.record.salida_excedida,error=null,t=null,extracted=null;
  try{if(s.exit_code===0){const body=frame(r.stdout,'SVLT0001',2097152),parsed=JSON.parse(body);t=trace(r.stderr);const original=JSON.parse(b);assert.equal(parsed.version,'IE004-LOTE-EXTRAIDO/1');assert.equal(parsed.solicitudes_sha256,s.sha256);assert.equal(parsed.solicitudes.length,24);assert.equal(t.casos.length,24);
    for(let i=0;i<24;i++){const a=parsed.solicitudes[i],src=original.casos[i],tr=t.casos[i];assert.deepEqual(a,{version:'IE004-A-SOLICITUD/1',...src});assert.equal(tr.id,src.id);assert.equal(tr.pregunta_sha256,hash(Buffer.from(src.pregunta)));const segment=b.subarray(tr.inicio,tr.fin);assert.deepEqual(JSON.parse(segment),src);assert.equal(tr.original_caso_sha256,hash(segment));assert.equal(tr.solicitud_sha256,hash(Buffer.from(JSON.stringify(a))));}
    assert(t.trabajo_lote<=24000000&&t.reserva_acumulada_bytes<=16777216);assert.equal(t.a_iniciada,false);assert.equal(t.v_abierta,false);assert.equal(t.auxiliar_abierto,false);extracted=parsed;
   }else{assert.equal(r.stdout.length,0);assert(r.stderr.toString().includes('SV_LOTE_ERROR:'+s.causa));if(s.id!=='L31')t=trace(r.stderr);if(s.id==='L05')assert.equal(t.recibidos,2097153);}
   if(seen.has(s.id)){assert.equal(hash(r.stdout),seen.get(s.id).stdout);if(t)assert.deepEqual(t,seen.get(s.id).trace);}else seen.set(s.id,{stdout:hash(r.stdout),trace:t});
  }catch(e){good=false;error=String(e);}
  tests.push({id:s.id,repeticion:rep,conforme:good,error,stdout_sha256:hash(r.stdout),traza:t});
  if(s.pipeline&&good){
   // Entrada A únicamente: no se abren ni extraen auxiliares, propuestas o el oráculo.
   // La verificación del manifiesto anterior es custodia, fuera de este recorrido.
   for(let i=0;i<24;i++){const req=extracted.solicitudes[i],wire=Buffer.from(JSON.stringify(req));save(id+'-r'+rep+'-'+req.id+'-solicitud.json',wire);const a=await invoke('r'+rep+'-'+req.id+'-a','receptor-a',['a'],wire,4144);let ok=a.record.exit_code===0&&!a.record.error&&!a.record.timeout,error=null,body=null,at=null;
    try{body=frame(a.stdout,'SVAC0001',4096);assert.equal(hash(body),orig.casos[i].cuerpo_esperado_sha256);assert.deepEqual(JSON.parse(body),orig.casos[i].cuerpo_esperado);at=trace(a.stderr);assert.equal(at.original,req.pregunta);assert.equal(at.id,req.id);assert.equal(at.original_transporte_sha256,hash(wire));assert.equal(at.cuerpo_sha256,hash(body));assert.equal(at.iniciado_a,true);save(id+'-r'+rep+'-'+req.id+'-cuerpo.json',body);}catch(e){ok=false;error=String(e)}
    flows.push({id:req.id,repeticion:rep,conforme:ok,error,cuerpo_sha256:body?hash(body):null,fuente:orig.casos[i].antecedente,unidades_a:at?.unidades_a,original_conservado:ok,v_abierta:false,auxiliar_abierto:false});
   }
  }
 }
 results.push({id,bins,controles:tests,recorrido:flows,estado:tests.length===108&&tests.every(x=>x.conforme)&&flows.length===72&&flows.every(x=>x.conforme)?'CONFORME_EN_ALCANCE':'FALLO'});
 save('RESULTADO_PARCIAL.json',{resultados:results});console.log(JSON.stringify({id,estado:results.at(-1).estado,controles:tests.filter(x=>x.conforme).length,total:tests.length,recorrido:flows.filter(x=>x.conforme).length,fallos:tests.filter(x=>!x.conforme).map(x=>({id:x.id,rep:x.repeticion,error:x.error}))}));
}
const parity=[];for(const s of spec.casos){const vals=results.map(r=>r.controles?.find(x=>x.id===s.id&&x.repeticion===1));parity.push({id:s.id,conforme:vals.every(Boolean)&&vals.every(v=>v.stdout_sha256===vals[0].stdout_sha256)});}
save('RESULTADO.json',{instrumento:'IE004-LOTE-ADAPTADOR/1',preparacion:'RETP-2026-138',resultados:results,paridad:parity,estado:results.length===4&&results.every(r=>r.estado==='CONFORME_EN_ALCANCE')&&parity.every(x=>x.conforme)?'CONFORME_EN_ALCANCE_PUBLICO':'FALLO',reserva_leida:false,captura_modelo:false,compatibilidad_p3_integral:'NO_ACREDITADA',aislamiento_p4:'NO_ACREDITADO',rss:'NO_MEDIDA'});
const files=fs.readdirSync(out).filter(x=>!x.endsWith('.bin')&&!x.endsWith('.wasm'));save('MANIFIESTO_RESULTADOS.json',{archivos:files.map(ruta=>{const b=fs.readFileSync(path.join(out,ruta));return{ruta,bytes:b.length,sha256:hash(b)}}),binarios:results.flatMap(r=>Object.entries(r.bins||{}).map(([tipo,b])=>({id:r.id,tipo,...b}))),nota:'Todos los stdout/stderr, metadatos, solicitudes extraídas y cuerpos; binarios reconstruibles desde fuentes fijadas.'});
if(results.some(r=>r.estado!=='CONFORME_EN_ALCANCE')||parity.some(x=>!x.conforme))process.exitCode=1;
