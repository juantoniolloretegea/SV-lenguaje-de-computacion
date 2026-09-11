import fs from 'node:fs';import path from 'node:path';import os from 'node:os';import crypto from 'node:crypto';import zlib from 'node:zlib';import assert from 'node:assert/strict';import {spawnSync}from'node:child_process';import{performance}from'node:perf_hooks';
const dir=path.dirname(new URL(import.meta.url).pathname);const salida=process.env.IE004_SALIDA||path.join(dir,'resultados');const rustc=process.env.IE004_RUSTC||'rustc';
assert(!fs.existsSync(salida),'No sobrescribir ni repetir implícitamente');fs.mkdirSync(salida,{recursive:true});
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');const put=(p,s)=>fs.writeFileSync(path.join(salida,p),typeof s==='string'?s:JSON.stringify(s,null,2)+'\n');
const fijacion=JSON.parse(fs.readFileSync(path.join(dir,'FIJACION_PREVIA.json')));for(const f of fijacion.archivos){const b=fs.readFileSync(path.join(dir,f.ruta));assert.equal(b.length,f.bytes);assert.equal(hash(b),f.sha256);}
const cs=JSON.parse(fs.readFileSync(path.join(dir,'CASOS_PUBLICOS.json'))).casos;
function run(id,cmd,args,timeout){const t=performance.now(),inicio=new Date().toISOString();const r=spawnSync(cmd,args,{cwd:dir,encoding:'utf8',timeout,maxBuffer:32*1024*1024,killSignal:'SIGKILL'});const duracion_ms=performance.now()-t;put(id+'.stdout',r.stdout||'');put(id+'.stderr',r.stderr||'');const m={id,cmd,args,inicio,fin:new Date().toISOString(),duracion_ms,timeout_ms:timeout,exit_code:r.status,signal:r.signal,error:r.error?.message||null};put(id+'.json',m);return{...m,stdout:r.stdout||''};}
const v=run('rustc-version',rustc,['-vV'],10000);assert.equal(v.exit_code,0);assert.match(v.stdout,/^rustc 1\.98\.0 /);
put('ENTORNO.json',{inicio:new Date().toISOString(),rustc:v.stdout,node:process.version,plataforma:os.platform(),release:os.release(),arch:os.arch(),cpu:os.cpus()[0]?.model,cpus:os.cpus().length,memoria_host_bytes:os.totalmem(),reserva_leida:false,unidades:'Propias de MEMO/1; no comparables directamente con R10-SINTAXIS/1',tiempos:'Una muestra por configuración, incluye arranque y exportación del bosque. No benchmark ni latencia del modelo.'});
const resultados=[],capturas=[],binarios=[];let compMs=0,execMs=0;let forestBase=null;
for(const target of ['nativo','wasi'])for(const perfil of ['debug','release']){
 const id=target+'-'+perfil;const bin=path.join(salida,id+(target==='wasi'?'.wasm':'.bin'));const args=['--edition=2021','-C','opt-level='+(perfil==='debug'?0:3),'-C','overflow-checks='+(perfil==='debug'?'yes':'no'),'-C','panic=abort'];
 if(target==='wasi')args.push('--target','wasm32-wasip1','-C','link-arg=--max-memory=67108864');
 const c=run(id+'-compilar',rustc,[...args,'memo.rs','-o',bin],Math.min(120000,480000-compMs));compMs+=c.duracion_ms;
 if(c.exit_code!==0){resultados.push({id,estado:'COMPILACION_FALLIDA'});continue;}
 const b=fs.readFileSync(bin);binarios.push({id,bytes:b.length,sha256:hash(b)});
 const r=run(id+'-ejecutar',target==='wasi'?process.execPath:bin,target==='wasi'?[path.join(dir,'wasi.mjs'),bin]:[],Math.min(30000,300000-execMs));execMs+=r.duracion_ms;
 const gz=zlib.gzipSync(r.stdout);const nombre='captura-'+hash(gz)+'.jsonl.gz.b64';put(nombre,gz.toString('base64')+'\n');capturas.push({id,archivo:nombre,stdout_bytes:Buffer.byteLength(r.stdout),stdout_sha256:hash(r.stdout),gzip_bytes:gz.length,gzip_sha256:hash(gz)});
 let lineas=[],errorParse=null;for(const l of r.stdout.trim().split('\n').filter(Boolean)){try{lineas.push(JSON.parse(l));}catch(e){errorParse=String(e);break;}}
 const cabecera=lineas.shift();let equivalentes=true;
 const compactas=lineas.map(({nodos,familias,...resto})=>({...resto,...(nodos?{bosque_sha256:hash(JSON.stringify({nodos,familias}))}:{})}));
 const forest=lineas.map(({id,estado,unidades,categorias,raiz,nodos,familias})=>({id,estado,unidades,categorias,raiz,nodos,familias}));
 if(forestBase===null)forestBase=JSON.stringify(forest);else equivalentes=forestBase===JSON.stringify(forest);
 const comparaciones=cs.map(c=>{const halladas=compactas.filter(r=>r.id===c.id);const x=halladas[0];return{id:c.id,grupo:c.grupo,esperado:c.esperado,coincide:halladas.length===1&&x.estado===c.esperado,observado:x||null};});
 const informe={id,completada:r.exit_code===0&&!errorParse&&lineas.length===cs.length,compilacion_ms:c.duracion_ms,proceso_ms:r.duracion_ms,exit_code:r.exit_code,error_parse:errorParse,cabecera,bosque_y_trabajo_igual_primera:equivalentes,coincidencias:comparaciones.filter(c=>c.coincide).length,total:cs.length,max_unidades:Math.max(0,...compactas.map(x=>x.unidades||0)),max_capacidad_bytes:Math.max(0,...compactas.map(x=>x.capacidad_bytes||0)),max_pico_solicitado_bytes:Math.max(0,...compactas.map(x=>x.pico_solicitado_bytes||0)),comparaciones};
 resultados.push(informe);put('SALIDAS-'+id+'.json',informe);console.log(JSON.stringify({...informe,comparaciones:undefined}));
}
put('RESULTADO.json',{registro:'RETP-2026-132',instrumento:'IE004-R10-MEMO/1',reserva_leida:false,perfil_2_modificado:false,conformidad_calendario_2:false,corrector_semantico:false,compilacion_ms:compMs,ejecucion_ms:execMs,configuraciones:resultados.map(({comparaciones,...r})=>r),binarios,capturas});
const incluir=fs.readdirSync(salida).filter(f=>!f.endsWith('.bin')&&!f.endsWith('.wasm')&&!f.endsWith('-ejecutar.stdout'));
put('MANIFIESTO_RESULTADOS.json',{archivos:incluir.map(ruta=>{const b=fs.readFileSync(path.join(salida,ruta));return{ruta,bytes:b.length,sha256:hash(b)};}),capturas,nota:'stdout completos de las cuatro ejecuciones recuperables sin pérdida desde gzip/base64, ver recuperar.mjs. Binarios identificados en RESULTADO.json, reconstruibles desde fuente/flags; no se presupone bit a bit en otros paths/entornos. Manifiesto excluido de su propia lista.'});
