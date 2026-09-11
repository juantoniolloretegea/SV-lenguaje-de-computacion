import fs from 'node:fs';import path from 'node:path';import crypto from 'node:crypto';import assert from 'node:assert/strict';import {spawn} from 'node:child_process';
const dir=path.dirname(new URL(import.meta.url).pathname),out=path.join(dir,'resultados-1');
if(fs.existsSync(out))throw Error('La ronda ya existe: no sobrescribir ni repetir implícitamente');
const hash=b=>crypto.createHash('sha256').update(b).digest('hex'),write=(file,data)=>fs.writeFileSync(path.join(out,file),typeof data==='string'||Buffer.isBuffer(data)?data:JSON.stringify(data,null,2)+'\n');
const fix=JSON.parse(fs.readFileSync(path.join(dir,'FIJACION_PREVIA.json'))),spec=JSON.parse(fs.readFileSync(path.join(dir,'CONTROLES_PUBLICOS.json')));
for(const f of [...fix.archivos,...fix.fuentes_heredadas]){const b=fs.readFileSync(path.join(dir,f.ruta));assert.equal(b.length,f.bytes);assert.equal(hash(b),f.sha256);}
const rust=process.env.SV_AV_RUSTC;if(!rust)throw Error('Falta toolchain Rust fijada');
if(!process.env.SV_AV_CORTE_CALIDAD||!process.env.SV_AV_CORTE_LAB)throw Error('Faltan cortes de prepublicación');
fs.mkdirSync(out);write('CORTES.json',{registro:'RETP-2026-136',calidad:process.env.SV_AV_CORTE_CALIDAD,laboratorio:process.env.SV_AV_CORTE_LAB,reserva_leida:false,python_ejecutado:false});
async function run(id,cmd,args,input=Buffer.alloc(0),opts={}){
 const start=Date.now(),record={id,cmd,args:args.map((a,i)=>a.length>1000?`[argumento ${i}: ${Buffer.byteLength(a)} bytes, SHA-256 ${hash(Buffer.from(a))}]`:a),inicio:new Date(start).toISOString(),timeout_ms:opts.timeout??30000,demora_ms:opts.delay??0};
 const child=spawn(cmd,args,{cwd:dir,env:{...process.env},stdio:['pipe','pipe','pipe']});
 const stdout=[],stderr=[];let count=0,exceeded=false,timedOut=false,spawnError=null,inputAt=null,delayTimer;
 const timer=setTimeout(()=>{timedOut=true;child.kill('SIGKILL');},record.timeout_ms);
 child.stdout.on('data',b=>{count+=b.length;if(count>32*1024*1024){exceeded=true;child.kill('SIGKILL');}else stdout.push(b);});
 child.stderr.on('data',b=>{if(stderr.reduce((a,b)=>a+b.length,0)+b.length<1024*1024)stderr.push(b);else{exceeded=true;child.kill('SIGKILL');}});
 child.stdin.on('error',e=>{if(e.code!=='EPIPE'&&e.code!=='ERR_STREAM_DESTROYED')spawnError=String(e);});
 if(!opts.keepOpen){delayTimer=setTimeout(()=>{inputAt=new Date().toISOString();child.stdin.end(input);},opts.delay??0);}
 return await new Promise(resolve=>{
  child.on('error',e=>{spawnError=String(e);});
  child.on('close',(code,signal)=>{clearTimeout(timer);clearTimeout(delayTimer);const raw=Buffer.concat(stdout),err=Buffer.concat(stderr);Object.assign(record,{fin:new Date().toISOString(),duracion_ms:Date.now()-start,exit_code:code,signal,timeout:timedOut,salida_excedida:exceeded,error:spawnError,entrada_entregada:inputAt,stdin_bytes:input.length,stdout_bytes:raw.length,stdout_sha256:hash(raw),stderr_bytes:err.length});write(id+'.stdout',raw);write(id+'.stderr',err);write(id+'.json',record);resolve({record,stdout:raw,stderr:err});});
 });
}
const version=await run('rustc-version',rust,['-vV'],undefined,{timeout:10000});assert.equal(version.record.exit_code,0);assert.match(version.stdout.toString(),/^rustc 1\.98\.0 /);
write('ENTORNO.json',{rust:version.stdout.toString(),node:process.version,plataforma:process.platform,arquitectura:process.arch,wasi:{preopens:{},env:{},max_memory_bytes:67108864},nota:'Tiempos de campaña, no benchmark; RSS y CPU no instrumentados.'});
const results=[];
for(const target of ['nativo','wasi'])for(const profile of ['debug','release']){
 const id=target+'-'+profile,flags=['--edition=2021','-C','opt-level='+(profile==='debug'?0:3),'-C','overflow-checks='+(profile==='debug'?'yes':'no'),'-C','panic=abort'];
 if(target==='wasi')flags.push('--target','wasm32-wasip1','-C','link-arg=--max-memory=67108864');
 const binaries={};let failed=false;
 for(const kind of ['receptor','controles']){const bin=path.join(out,id+'-'+kind+(target==='wasi'?'.wasm':'.bin'));const r=await run(id+'-compilar-'+kind,rust,[...flags,...(kind==='controles'?['--cfg','controles_publicos']:[]),'main.rs','-o',bin],undefined,{timeout:120000});if(r.record.exit_code!==0){failed=true;break;}const bytes=fs.readFileSync(bin);binaries[kind]={path:bin,bytes:bytes.length,sha256:hash(bytes)};}
 if(failed){results.push({id,estado:'FALLO_COMPILACION'});continue;}
 const invoke=(tag,kind,args,input,opts)=>run(id+'-'+tag,target==='nativo'?binaries[kind].path:process.execPath,target==='nativo'?args:[path.join(dir,'wasi.mjs'),binaries[kind].path,...args],input,opts);
 const control=await invoke('control','controles',['control']);let logs=[],parseError=null;try{logs=control.stdout.toString().trim().split('\n').map(JSON.parse);}catch(e){parseError=String(e);}
 const summary=logs.at(-1);const failures=logs.filter(x=>x.conforme===false);const lanes=[];
 // Intervenciones de transporte fijadas: abrir V sólo después de validar y depositar el cuerpo A.
 for(const scenario of spec.escenarios_transporte){
  const events=[];const mark=nombre=>events.push({nombre,fecha:new Date().toISOString()});
  const a=spec.casos_a[0],aw=fs.readFileSync(path.join(dir,a.file));mark('A_RECIBIDA');
  const ar=await invoke(scenario+'-a','receptor',['a'],aw);
  const accepted=await invoke(scenario+'-validar','receptor',['validar'],ar.stdout);
  if(ar.record.exit_code!==0||accepted.record.exit_code!==0){lanes.push({scenario,conforme:false,causa:'A_NO_ENTREGABLE',events});continue;}
  assert(accepted.stdout.equals(ar.stdout.subarray(16,ar.stdout.length-32)));
  const bodyHash=hash(accepted.stdout);write(scenario+'-'+id+'-cuerpo.tmp',accepted.stdout);fs.renameSync(path.join(out,scenario+'-'+id+'-cuerpo.tmp'),path.join(out,scenario+'-'+id+'-cuerpo.json'));mark('CUERPO_A_ENTREGADO');
  const vId=scenario==='V02_DEMORADA'?'V02':scenario==='V_SIN_EOF'?'V01':scenario;
  const v=spec.casos_v.find(x=>x.id===vId);mark('V_ABIERTA');const vb=fs.readFileSync(path.join(dir,v.file));
  const vr=await invoke(scenario+'-v','receptor',['v',aw.toString('utf8'),spec.lote_sha256,bodyHash,String(v.cupo)],vb,scenario==='V_SIN_EOF'?{timeout:150,keepOpen:true}:scenario==='V02_DEMORADA'?{delay:50}:{});
  mark('V_FINALIZADA');let annex=null,ok;
  if(scenario==='V_SIN_EOF'){ok=vr.record.timeout&&vr.record.signal==='SIGKILL'&&vr.stdout.length===0;annex={estado:'V_PLAZO_AGOTADO_POR_CONDUCTOR',cuerpo_a_sha256:bodyHash,plazo_inyectado_ms:150,nota:'Control funcional; no acreditación de contención P4.'};write(scenario+'-'+id+'-anexo.json',annex);}
  else{try{annex=JSON.parse(vr.stdout);ok=vr.record.exit_code===0&&annex.estado===v.estado&&annex.causa===v.causa&&annex.cuerpo_a_sha256===bodyHash;}catch{ok=false;}}
  const deposited=fs.readFileSync(path.join(out,scenario+'-'+id+'-cuerpo.json'));ok=ok&&hash(deposited)===bodyHash&&events.findIndex(e=>e.nombre==='CUERPO_A_ENTREGADO')<events.findIndex(e=>e.nombre==='V_ABIERTA');
  lanes.push({scenario,conforme:ok,events,cuerpo_sha256:bodyHash,traza_a_sha256:hash(ar.stderr),v:annex,demora_real_ms:vr.record.entrada_entregada?Date.parse(vr.record.entrada_entregada)-Date.parse(vr.record.inicio):null});
 }
 const baseBody=lanes[0]?.cuerpo_sha256;const sameBody=lanes.every(x=>x.cuerpo_sha256===baseBody);
 results.push({id,binaries,control:control.record,resumen:summary,parseError,fallos:failures,transporte:lanes,cuerpos_a_identicos:sameBody,estado:control.record.exit_code===0&&!parseError&&summary?.fallos===0&&lanes.every(x=>x.conforme)&&sameBody?'CONFORME_EN_ALCANCE':'FALLO'});
 write('RESULTADO_PARCIAL.json',{resultados:results});console.log(JSON.stringify({id,estado:results.at(-1).estado,resumen:summary,fallos:failures.map(x=>({grupo:x.grupo,id:x.id,anexo:x.anexo})),transporte:lanes.map(x=>({id:x.scenario,ok:x.conforme}))}));
}
write('RESULTADO.json',{registro_preparacion:'RETP-2026-136',instrumento:'IE004-RECEPCION-AV/1',resultados:results,reserva_leida:false,python_ejecutado:false,seguridad_material:'NO_ACREDITADA',memoria_fisica:'NO_MEDIDA',nota:'Una matriz fijada; tres reproducciones internas por control, una observación por escenario de transporte y configuración. No son muestras de un proveedor.'});
const listed=fs.readdirSync(out).filter(f=>!f.endsWith('.bin')&&!f.endsWith('.wasm'));write('MANIFIESTO_RESULTADOS.json',{archivos:listed.map(ruta=>{const b=fs.readFileSync(path.join(out,ruta));return{ruta,bytes:b.length,sha256:hash(b)}}),binarios:results.flatMap(x=>Object.entries(x.binaries||{}).map(([tipo,b])=>({id:x.id,tipo,bytes:b.bytes,sha256:b.sha256}))),nota:'Binarios identificados y reconstruibles. Todos los stdout/stderr y el cuerpo A son recuperables.'});
if(results.some(x=>x.estado!=='CONFORME_EN_ALCANCE'))process.exitCode=1;
