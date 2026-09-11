import fs from 'node:fs';import path from 'node:path';import crypto from 'node:crypto';import z from 'node:zlib';import assert from 'node:assert/strict';import {spawnSync} from 'node:child_process';
const [rootArg,rustArg,priorArg,outArg]=process.argv.slice(2),root=path.resolve(rootArg),rust=path.resolve(rustArg),prior=path.resolve(priorArg),out=path.resolve(outArg),dir=path.dirname(new URL(import.meta.url).pathname);
if(fs.existsSync(out))throw Error('Salida existente');fs.mkdirSync(out,{recursive:true});
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const put=(p,b)=>fs.writeFileSync(path.join(out,p),Buffer.isBuffer(b)||typeof b==='string'?b:JSON.stringify(b,null,2)+'\n');
const old=JSON.parse(fs.readFileSync(path.join(prior,'RESULTADO.json'))),report={...old,casos:old.casos.filter(x=>x.id!=='C08'),procesos:[...old.procesos],antecedentes:['RESULTADO_INICIAL.json','RESULTADO_SEGUNDO_INTENTO.json'],rectificacion:'RECTIFICACION_ESPERADO_C08.md'};delete report.error;
const run=(id,cmd,args)=>{const r=spawnSync(cmd,args,{timeout:60000,maxBuffer:32*1024*1024});put(id+'.stdout',r.stdout??Buffer.alloc(0));put(id+'.stderr',r.stderr??Buffer.alloc(0));const rec={id,cmd,args,exit_code:r.status,signal:r.signal,error:r.error?String(r.error):null};put(id+'.json',rec);report.procesos.push(rec);return r;};
try{
 const p=path.join(root,'docs/calidad/tuberias-ia/ie004/recepcion-av');
 const expected=JSON.parse(fs.readFileSync(path.join(p,'CONTROLES_PUBLICOS.json'))).casos_v.find(x=>x.id==='V04');assert.equal(expected.causa,'TRUNCADO');
 const cap=JSON.parse(z.gunzipSync(Buffer.from(fs.readFileSync(path.join(p,'CAPTURAS_RECUPERABLES.json.gz.b64'),'utf8'),'base64')));
 const historical=Buffer.from(cap.archivos.find(x=>x.ruta==='nativo-release-V04-v.stdout').base64,'base64'),actual=fs.readFileSync(path.join(prior,'V04-v.stdout'));
 assert(actual.equals(historical));assert.equal(JSON.parse(actual).causa,expected.causa);assert.equal(JSON.parse(actual).estado,expected.estado);
 report.casos.push({id:'C08',conforme:true,estado:expected.estado,causa:expected.causa,anexo_identico:true,nueva_ejecucion:false});
 const src='#![allow(dead_code)]\n'+['nat','frame','frame_tests'].map(n=>'#[path = '+JSON.stringify(path.join(root,'rust/sv_core/src/'+n+'.rs'))+'] mod '+n+';').join('\n')+'\npub use nat::Nat;\n';put('arnes_frame.rs',src);
 const bin=path.join(out,'frame-tests.bin');const c=run('compilar-frame',rust,['--edition=2021','--test',path.join(out,'arnes_frame.rs'),'-o',bin]);assert.equal(c.status,0);
 const t=run('frame-existentes',bin,['frame_tests::','--test-threads=1']);assert.equal(t.status,0);assert.match(t.stdout.toString(),/16 passed; 0 failed/);
 report.casos.push({id:'C09',conforme:true,pruebas_frame_existentes:16,alcance:'Constructor y guardas de Frame; sin transición ni integración IE-004'});
 report.estado='CONTRASTE_COMPROBADO_TRAS_RECTIFICAR_OBSERVADOR';
}catch(e){report.estado='FALLO';report.error=String(e);process.exitCode=1;}finally{
 put('RESULTADO.json',report);const priorFiles=fs.readdirSync(prior).filter(x=>!x.endsWith('.bin')).map(ruta=>({root:prior,ruta}));const newFiles=fs.readdirSync(out).filter(x=>!x.endsWith('.bin')).map(ruta=>({root:out,ruta}));
 const archivos=[...priorFiles,...newFiles].map(x=>{const b=fs.readFileSync(path.join(x.root,x.ruta));return{ruta:(x.root===prior?'contraste-2/':'continuacion/')+x.ruta,bytes:b.length,sha256:hash(b),base64:b.toString('base64')};});
 fs.writeFileSync(path.join(dir,'EVIDENCIA_RECUPERABLE.json'),JSON.stringify({version:'IE004-TRAZABILIDAD-CONTRASTE/1',archivos},null,2)+'\n');fs.writeFileSync(path.join(dir,'RESULTADO_CONTRASTE.json'),JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify({estado:report.estado,casos:report.casos,error:report.error}));
}
