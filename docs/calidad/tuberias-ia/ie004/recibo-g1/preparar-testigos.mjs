import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';

// Custodia documental. No implementa ni juzga un receptor G1.
// node preparar-testigos.mjs EVIDENCIA_RECUPERABLE_RETP142.json SALIDA_NUEVA
const [source, output] = process.argv.slice(2);
assert(source && output, 'Faltan cápsula de origen y directorio nuevo');
assert(!fs.existsSync(output), 'Conservar salidas previas: se exige directorio nuevo');
const sha = b => crypto.createHash('sha256').update(b).digest('hex');
const raw = fs.readFileSync(source);
const expected = 'eafdc98ad4174d76bca87ac14c18fc9836d4175aef8c4f5001b295916b66496b';
assert.equal(sha(raw), expected, 'Cápsula distinta del corte fijado');
const capsule = JSON.parse(raw), originals = new Map();
for (const f of capsule.archivos) {
  assert(!originals.has(f.ruta));
  const b = Buffer.from(f.base64, 'base64');
  assert.equal(b.length, f.bytes); assert.equal(sha(b), f.sha256);
  originals.set(f.ruta, b);
}
const artifacts = [];
function add(id, bytes, provenance) {
  assert(!artifacts.some(a => a.id === id));
  artifacts.push({id, bytes:bytes.length, sha256:sha(bytes), base64:bytes.toString('base64'), procedencia:provenance});
  return bytes;
}
function take(id, name) {
  const ruta = 'contraste-2/' + name;
  assert(originals.has(ruta));
  return add(id, originals.get(ruta), {tipo:'ORIGINAL_COTEJADO_RETP142', ruta});
}
const input = take('solicitud-a01', 'A01-a.stdin');
const frame = take('marco-a01', 'A01-a.stdout');
const trace = take('traza-a01', 'A01-a.stderr');
const body = take('cuerpo-a01', 'A01-validar.stdout');
take('propuesta-v02', 'V02-v.stdin');
const v02 = take('anexo-v02', 'V02-v.stdout');
take('propuesta-v04', 'V04-v.stdin');
take('anexo-v04', 'V04-v.stdout');
const t = JSON.parse(trace), b = JSON.parse(body);
assert.equal(t.original_transporte_sha256, sha(input));
assert.equal(t.cuerpo_sha256, sha(body));
assert.equal(frame.subarray(0,8).toString(), 'SVAC0001');
assert(frame.subarray(16,-32).equals(body));
assert.equal(Number(frame.readBigUInt64LE(8)), body.length);
assert.equal(frame.subarray(-32).toString('hex'), sha(body));
assert.deepEqual(t.resolucion, b.resolucion);

const alteredInput = add('solicitud-espacio', Buffer.concat([input,Buffer.from(' ')]), {tipo:'MUTACION_PREPARADA',original:'solicitud-a01',cambio:'Añadir un espacio ASCII después del JSON'});
assert.deepEqual(JSON.parse(alteredInput), JSON.parse(input));
assert.notEqual(sha(alteredInput), t.original_transporte_sha256);
const lastBrace = trace.lastIndexOf(125); assert(lastBrace > 0);
const truncated = add('traza-truncada', trace.subarray(0,lastBrace), {tipo:'MUTACION_PREPARADA',original:'traza-a01',cambio:'Suprimir llave final y sufijo'});
assert.throws(() => JSON.parse(truncated));
const alteredTrace = JSON.parse(trace); alteredTrace.resolucion.contenido='9.40';
add('traza-discordante', Buffer.from(JSON.stringify(alteredTrace)+'\n'), {tipo:'MUTACION_PREPARADA',original:'traza-a01',cambio:'Contenido de resolución 8.40 a 9.40; hashes declarados intactos'});
assert.equal(alteredTrace.cuerpo_sha256, sha(body));
assert.notDeepEqual(alteredTrace.resolucion,b.resolucion);
const alteredFrame=Buffer.from(frame), offset=alteredFrame.indexOf(Buffer.from('8.40'));
assert(offset>=16 && offset<frame.length-32);alteredFrame[offset]=57;
add('marco-alterado',alteredFrame,{tipo:'MUTACION_PREPARADA',original:'marco-a01',cambio:'Un byte de 8.40 a 9.40 sin renovar hash'});
assert.notEqual(sha(alteredFrame.subarray(16,-32)), alteredFrame.subarray(-32).toString('hex'));
const alteredAnnex=JSON.parse(v02);alteredAnnex.cuerpo_a_sha256='0'.repeat(64);
add('anexo-otro-cuerpo',Buffer.from(JSON.stringify(alteredAnnex)+'\n'),{tipo:'MUTACION_PREPARADA',original:'anexo-v02',cambio:'Hash de cuerpo sustituido por 64 ceros'});
const prefix=add('solicitud-prefijo',input.subarray(0,16),{tipo:'MUTACION_PREPARADA',original:'solicitud-a01',cambio:'Conservar únicamente 16 bytes; corte del observador, no límite constitutivo'});
assert(prefix.length<input.length);

const base={entrada:'solicitud-a01',marco:'marco-a01',traza:'traza-a01',cuerpo:'cuerpo-a01'};
const definitions=[
 ['G1-01','apertura A propia',base,'recuperación completa','RECIBO_A_COMPLETO'],
 ['G1-02','apertura A propia',{...base,traza:null},'pérdida de traza','INTENTO_INCOMPLETO'],
 ['G1-03','apertura A propia',{...base,traza:'traza-truncada'},'traza incompleta','INTENTO_INCOMPLETO'],
 ['G1-04','apertura A propia',{...base,entrada:'solicitud-espacio'},'usar traza antigua con otro original','RECHAZO_CORRELACION'],
 ['G1-05','apertura A propia',{...base,traza:'traza-discordante'},'discordancia de resolución','RECHAZO_DISCORDANCIA'],
 ['G1-06','apertura A propia',{...base,marco:'marco-alterado'},'alteración de cuerpo sin hash','RECHAZO_INTEGRIDAD'],
 ['G1-07','apertura A propia',base,'inyectar fallo al recuperar traza tras capturarla','INTENTO_INCOMPLETO'],
 ['G1-08','A ya finalizada',base,'repetir finalización con la misma identidad','RECHAZO_DUPLICADO'],
 ['G1-09','dos aperturas distintas',base,'mismos bytes entregados con el manejador ajeno','RECHAZO_PERTENENCIA'],
 ['G1-10','A completo',{propuesta:'propuesta-v02',anexo:'anexo-v02'},'anexado propio posterior','ANEXO_RECUPERABLE'],
 ['G1-11','A completo',{propuesta:'propuesta-v04',anexo:'anexo-v04'},'propuesta malformada capturada entera; rechazo V íntegro','ANEXO_RECUPERABLE_DE_RECHAZO'],
 ['G1-12','A completo y V prevista',{},'mantener canal V sin EOF e inyectar interrupción por plazo','OBSERVACION_V_INCOMPLETA'],
 ['G1-13','A completo',{propuesta:'propuesta-v02',anexo:'anexo-otro-cuerpo'},'anexo discordante','RECHAZO_CORRELACION'],
 ['G1-14','V ya finalizada',{propuesta:'propuesta-v02',anexo:'anexo-v02'},'repetir anexado terminal','RECHAZO_DUPLICADO'],
 ['G1-15','A aún incompleto',{propuesta:'propuesta-v02',anexo:'anexo-v02'},'anexar antes de cierre A','RECHAZO_ESTADO'],
 ['G1-16','A completo',{},'conductor declara V no solicitada','REGISTRO_V_NO_SOLICITADA'],
 ['G1-17','apertura con vigencia revocada',base,'entregar cuerpo con vigente true','RECHAZO_MONTAJE'],
 ['G1-18','apertura con captura interrumpida',{entrada:'solicitud-prefijo'},'declarar prefijo completo=false y causa de límite; no fingir hash del total','INTENTO_INCOMPLETO']
];
const cases=definitions.map(([id,estado_previo,artefactos,accion,esperado])=>({id,estado_previo,artefactos,accion,esperado,ejecutado_con_receptor_g1:false,requiere_arnes_nativo:true,preservar_cierre_a:estado_previo.startsWith('A completo')||estado_previo==='V ya finalizada'}));
assert.equal(new Set(cases.map(c=>c.id)).size,18);
for(const c of cases)for(const ref of Object.values(c.artefactos))if(ref!==null)assert(artifacts.some(a=>a.id===ref));
assert.equal(JSON.parse(originals.get('contraste-2/V04-v.stdout')).causa,'TRUNCADO');
const result={registro:'RETP-2026-143',estado:'TESTIGOS_PREPARADOS_NO_EJECUTADOS_CON_RECEPTOR_G1',fuente_commit:'282e020f8bff2b68abd9a2e9138fa6b467debaf8',capsula_origen_sha256:sha(raw),archivos_origen_cotejados:originals.size,artefactos_preparados:artifacts.length,casos_especificados:cases.length,comprobaciones:'Procedencia, integridad, relaciones A01 y discriminación material de mutaciones. No veredicto G1.',script_sha256:sha(fs.readFileSync(new URL(import.meta.url))),modelo_invocado:false,reserva_leida:false};
fs.mkdirSync(output,{recursive:true});
for(const [name,data]of Object.entries({'CAPSULA_TESTIGOS_G1.json':{version:'IE004-G1-TESTIGOS/1',artefactos:artifacts},'TESTIGOS_G1.json':{version:'IE004-G1-CASOS/1',contrato:'G1/1',casos:cases},'PREPARACION_TESTIGOS.json':result}))fs.writeFileSync(path.join(output,name),JSON.stringify(data,null,2)+'\n');
console.log(JSON.stringify(result));
