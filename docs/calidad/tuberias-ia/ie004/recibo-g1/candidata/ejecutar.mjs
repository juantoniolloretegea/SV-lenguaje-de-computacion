import fs from 'node:fs';import path from 'node:path';import crypto from 'node:crypto';import {spawnSync} from 'node:child_process';
import assert from 'node:assert/strict';
// node ejecutar.mjs RUSTC SALIDA_NUEVA
const [rustArg,outArg]=process.argv.slice(2);assert(rustArg&&outArg);
const rust=path.resolve(rustArg),out=path.resolve(outArg),dir=path.dirname(new URL(import.meta.url).pathname);
assert(!fs.existsSync(out));fs.mkdirSync(out,{recursive:true});
const sha=b=>crypto.createHash('sha256').update(b).digest('hex');
const originalCapsule=JSON.parse(fs.readFileSync(path.join(dir,'../CAPSULA_TESTIGOS_G1.json')));
fs.mkdirSync(path.join(dir,'fixtures'),{recursive:true});
for(const a of originalCapsule.artefactos){const b=Buffer.from(a.base64,'base64');assert.equal(sha(b),a.sha256);assert.equal(b.length,a.bytes);const p=path.join(dir,'fixtures',a.id+'.bin');if(fs.existsSync(p))assert(fs.readFileSync(p).equals(b));else fs.writeFileSync(p,b);}
function walk(p){return fs.readdirSync(p,{withFileTypes:true}).flatMap(f=>f.isDirectory()?walk(path.join(p,f.name)):[path.join(p,f.name)]);}
const files=[...walk(dir).filter(p=>/\.(rs|json|mjs|bin)$/.test(p)&&!p.includes('/evidencia/')), ...['semantica-a','recepcion-av'].flatMap(n=>walk(path.resolve(dir,'../..',n)).filter(p=>p.endsWith('.rs')))];
const sourceFiles=[...new Set(files)].sort().map(p=>{const b=fs.readFileSync(p);return {path:path.relative(dir,p),bytes:b.length,sha256:sha(b)}});
const manifest=Buffer.from(JSON.stringify({version:'IE004-G1-FIJACION/1',archivos:sourceFiles},null,2)+'\n');
fs.writeFileSync(path.join(out,'FIJACION_PREVIA.json'),manifest);
fs.copyFileSync(path.join(dir,'PERFIL_Y_PLAN.json'),path.join(out,'PERFIL_Y_PLAN.json'));
const result={registro:'RETP-2026-144',estado:'EN_CURSO',fuentes_sha256:sha(manifest),procesos:[],configuraciones:[],alcance:'Nativo intra-proceso y recuperación en memoria; supervisor/plazo inyectado; sin P4, G2/G3/G4 o reserve',reserva_leida:false,modelo_invocado:false};
function run(id,cmd,args,env={}){const r=spawnSync(cmd,args,{timeout:60000,maxBuffer:32*1024*1024,env:{...process.env,...env}});const stdout=r.stdout??Buffer.alloc(0),stderr=r.stderr??Buffer.alloc(0);fs.writeFileSync(path.join(out,id+'.stdout'),stdout);fs.writeFileSync(path.join(out,id+'.stderr'),stderr);const entry={id,cmd,args,exit_code:r.status,signal:r.signal,error:r.error?String(r.error):null,stdout_sha256:sha(stdout),stderr_sha256:sha(stderr)};fs.writeFileSync(path.join(out,id+'.json'),JSON.stringify(entry,null,2)+'\n');result.procesos.push(entry);return{...entry,stdout,stderr};}
try{
 const v=run('rustc-version',rust,['--version','--verbose']);assert.equal(v.exit_code,0);assert.match(v.stdout.toString(),/^rustc 1\.98\.0 /);
 for(const mode of ['debug','release']){
  const bin=path.join(out,'g1-'+mode+'.bin');const flags=mode==='release'?['-C','opt-level=3','-C','overflow-checks=no']:['-C','opt-level=0','-C','overflow-checks=yes'];
  const built=run('compilar-'+mode,rust,['--edition=2021','--test',path.join(dir,'lib.rs'),...flags,'-o',bin]);
  if(built.exit_code!==0){result.configuraciones.push({modo:mode,compila:false});continue;}
  const binhash=sha(fs.readFileSync(bin)),capture=path.join(out,'capturas-'+mode);fs.mkdirSync(capture);
  const tested=run('test-'+mode,bin,['--nocapture','--test-threads=1'],{G1_FUENTES_SHA256:sha(manifest),G1_BINARIO_SHA256:binhash,G1_CAPTURAS:capture});
  const ids=[...tested.stdout.toString().matchAll(/TESTIGO_G1:(G1-\d+):CONFORME_EN_ALCANCE/g)].map(m=>m[1]);
  result.configuraciones.push({modo:mode,compila:true,binario_sha256:binhash,exit_code:tested.exit_code,testigos:ids,tests_conformes:tested.exit_code===0&&new Set(ids).size===20});
 }
 // Frontera de uso público: sólo se permite leer; el cierre y el manejador
 // no se pueden fabricar desde otra crate. Compilación de testigo válido primero.
 const lib=path.join(out,'libg1.rlib');const built=run('compilar-biblioteca',rust,['--edition=2021','--crate-type=lib','--crate-name=g1',path.join(dir,'lib.rs'),'-o',lib]);assert.equal(built.exit_code,0);
 const good='extern crate g1; fn main(){let _=g1::Custodio::nuevo(g1::Realizacion{fuentes_sha256:[1;32],binario_sha256:[2;32]}).unwrap();}\n';
 const bad='extern crate g1; fn main(){let _=g1::Manejador{ambito:1,ordinal:1,slot:0};}\n';
 for(const [id,src,ok]of [['uso-publico',good,true],['forja-manejador',bad,false]]){const p=path.join(out,id+'.rs');fs.writeFileSync(p,src);const r=run(id,rust,['--edition=2021',p,'--extern','g1='+lib,'-o',path.join(out,id+'.bin')]);if(ok)assert.equal(r.exit_code,0);else{assert.notEqual(r.exit_code,0);assert.match(r.stderr.toString(),/private/);}}
 // Cotejo de paridad de capturas, sin comparar tiempos ni identidad del binario.
 let parity=true,count=0;for(const p of walk(path.join(out,'capturas-debug'))){const relative=path.relative(path.join(out,'capturas-debug'),p);const q=path.join(out,'capturas-release',relative);if(!fs.existsSync(q)||!fs.readFileSync(p).equals(fs.readFileSync(q)))parity=false;count++;}
 result.paridad_capturas={archivos:count,identicos:parity};
 for(const f of sourceFiles){const b=fs.readFileSync(path.resolve(dir,f.path));assert.equal(sha(b),f.sha256);assert.equal(b.length,f.bytes);}
 result.estado=result.configuraciones.every(c=>c.tests_conformes)&&parity?'CONFORME_EN_ALCANCE_INTRAPROCESO':'FALLO';
}catch(e){result.estado='FALLO';result.error=String(e);}finally{
 fs.writeFileSync(path.join(out,'RESULTADO.json'),JSON.stringify(result,null,2)+'\n');
 const artifacts=walk(out).filter(p=>!p.endsWith('.bin')&&!p.endsWith('.rlib')).sort().map(p=>{const b=fs.readFileSync(p);return {ruta:path.relative(out,p),bytes:b.length,sha256:sha(b),base64:b.toString('base64')}});
 fs.writeFileSync(path.join(out,'CAPSULA.json'),JSON.stringify({version:'IE004-G1-CAPTURAS/1',archivos:artifacts},null,2)+'\n');
 console.log(JSON.stringify(result));if(result.estado==='FALLO')process.exitCode=1;
}
