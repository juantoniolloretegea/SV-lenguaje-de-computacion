// Comparación de los binarios custodiados de ES27/2 y ES27/3 en un mismo host.
// Uso: node comparar-rendimiento.mjs directorio_artefacto_2 directorio_artefacto_3 salida.json
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import crypto from 'node:crypto';
import {spawnSync} from 'node:child_process';
const [anterior,actual,destino]=process.argv.slice(2);
if(!anterior||!actual||!destino)throw Error('ARGUMENTOS');
const sha=b=>crypto.createHash('sha256').update(b).digest('hex');
const u32=n=>{const b=Buffer.alloc(4);b.writeUInt32LE(n);return b;};
const directorios={anterior,actual}, resumenes={};
for(const [v,d] of Object.entries(directorios)){
 const r=JSON.parse(fs.readFileSync(path.join(d,'resultado/RESULTADO.json')));resumenes[v]=r;
 for(const a of r.artefactos){const f=path.join(d,'bin',a.nombre),b=fs.readFileSync(f);if(b.length!==a.bytes||sha(b)!==a.sha256)throw Error('BINARIO_DISTINTO');if(a.nombre==='consulta27')fs.chmodSync(f,0o755);}
}
const oraculos=JSON.parse(fs.readFileSync(path.join(anterior,'resultado/oraculos.json'))).casos.slice(0,26);
const cuerpos=[1,50].map(repeticiones=>{
 const casos=Array.from({length:repeticiones},()=>oraculos).flat();
 const entrada=Buffer.concat([u32(casos.length),...casos.flatMap(c=>{const q=Buffer.from(c.entrada_base64,'base64');return [u32(q.length),q,u32(0)];})]);
 return {repeticiones,consultas:casos.length,entrada,esperado:casos.map(c=>c.esperado).join('')};
});
const medidas=[];
for(const carga of cuerpos)for(const motor of ['nativo','wasi']){
 // Dos pares de calentamiento y veinte pares AB/BA alternos.
 for(let par=-2;par<20;par++)for(const version of (par%2===0?['anterior','actual']:['actual','anterior'])){
  const d=directorios[version],cmd=motor==='nativo'?path.resolve(d,'bin/consulta27'):process.execPath;
  const args=motor==='nativo'?[]:['--no-warnings',path.resolve(d,'fuentes/wasi.mjs'),path.resolve(d,'bin/consulta27.wasm')];
  const inicio=process.hrtime.bigint();
  const r=spawnSync(cmd,args,{input:carga.entrada,timeout:10000,maxBuffer:4*1024*1024,env:{PATH:process.env.PATH,LANG:'C.UTF-8'}});
  const ns=Number(process.hrtime.bigint()-inicio);
  if(r.error||r.signal||r.status!==0)throw Error('EJECUCION '+String(r.error??r.signal??r.status)+' '+r.stderr);
  const esperado=version==='actual'?carga.esperado.replaceAll('G-ES27/2','G-ES27/3'):carga.esperado;
  if(!r.stdout.equals(Buffer.from(esperado)))throw Error('SALIDA_DISTINTA');
  if(par>=0)medidas.push({consultas:carga.consultas,motor,par,version,duracion_ns:ns,entrada_sha256:sha(carga.entrada),salida_sha256:sha(r.stdout)});
 }
}
const mediana=xs=>{const s=[...xs].sort((a,b)=>a-b),m=s.length/2;return (s[Math.floor(m)]+s[Math.ceil(m)-1])/2;};
const comparaciones=[];
for(const carga of cuerpos)for(const motor of ['nativo','wasi']){
 const m=medidas.filter(x=>x.consultas===carga.consultas&&x.motor===motor);
 const antes=mediana(m.filter(x=>x.version==='anterior').map(x=>x.duracion_ns));
 const despues=mediana(m.filter(x=>x.version==='actual').map(x=>x.duracion_ns));
 const cocientes=Array.from({length:20},(_,par)=>m.find(x=>x.par===par&&x.version==='actual').duracion_ns/m.find(x=>x.par===par&&x.version==='anterior').duracion_ns);
 comparaciones.push({consultas:carga.consultas,motor,mediana_anterior_ns:antes,mediana_actual_ns:despues,cambio_mediana_porcentaje:100*(despues/antes-1),mediana_cociente_pareado:mediana(cocientes),min_cociente_pareado:Math.min(...cocientes),max_cociente_pareado:Math.max(...cocientes)});
}
const resultado={version:1,alcance:'Comparación local de procesos completos; no medición aislada del parser ni umbral productivo.',entorno:{node:process.version,plataforma:process.platform,arquitectura:process.arch,cpu:os.cpus()[0]?.model,cpus:os.cpus().length,memoria_total:os.totalmem()},fuentes:Object.fromEntries(Object.entries(resumenes).map(([v,r])=>[v,{fuente_commit:r.fuente_commit,entrada_commit:r.entrada_commit,run_id:r.run_id,artefactos:r.artefactos}])),protocolo:{oraculos:26,pares_por_carga_y_motor:20,calentamientos:2,orden:'AB/BA alternos',cargas:[26,1300],control_salida:'Literal, salvo versión de gramática declarada en cada ejecutable'},comparaciones,medidas};
fs.writeFileSync(destino,JSON.stringify(resultado,null,2)+'\n');
console.log(JSON.stringify({comparaciones,muestras:medidas.length}));
