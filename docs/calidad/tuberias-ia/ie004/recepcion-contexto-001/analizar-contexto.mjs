import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import crypto from 'node:crypto';

// Derivación documental sobre dos capturas recibidas; no invoca al modelo
// ni modifica la interpretación, el receptor o las expectativas congeladas.
const [actual, anterior] = process.argv.slice(2);
assert.ok(actual && anterior, 'USO: node analizar-contexto.mjs recepcion-contexto recepcion-002');
const bytes = (dir, name) => fs.readFileSync(path.join(dir, name));
const read = (dir, name) => JSON.parse(bytes(dir, name));
const save = (name, value) => fs.writeFileSync(path.join(actual, name), JSON.stringify(value, null, 2) + '\n');
const hash = b => crypto.createHash('sha256').update(b).digest('hex');
const fields = ['operacion', 'objeto', 'parametro', 'momento', 'campo'];
const good = x => x.identificacion_correcta && x.respuesta_correcta;
const signature = x => [x.ruta && fields.map(k => x.ruta[k]), x.diagnostico];
const equal = (a, b) => JSON.stringify(a) === JSON.stringify(b);
const r = read(actual, 'RECEPCION.json');
const old = read(anterior, 'RECEPCION.json');
const p = read(actual, 'solicitudes-originales.json');
const o = read(actual, 'ORACULO-ABIERTO.json');
const d = read(actual, 'entrega-original.json');
const od = read(anterior, 'entrega-original.json');
assert.deepEqual(bytes(actual, 'solicitudes-originales.json'), bytes(anterior, 'solicitudes-originales.json'));
assert.deepEqual(bytes(actual, 'ORACULO-ABIERTO.json'), bytes(anterior, 'ORACULO-ABIERTO.json'));
assert.equal(hash(bytes(actual, 'entrega-original.json')), r.entrega_sha256);
assert.equal(hash(bytes(anterior, 'entrega-original.json')), old.entrega_sha256);
assert.equal(hash(bytes(actual, 'solicitudes-originales.json')), r.solicitudes_sha256);
assert.equal(hash(bytes(actual, 'ORACULO-ABIERTO.json')), r.oraculo_sha256);
const index = rows => { const m = new Map(rows.map(x => [x.id, x])); assert.equal(m.size, rows.length); return m; };
const cur = index(r.hallazgos), prev = index(old.hallazgos), dc = index(d.respuestas), dp = index(od.respuestas);
const ids = p.casos.map(x => x.id);
for (const m of [cur, prev, dc, dp]) assert.deepEqual([...m.keys()].sort(), [...ids].sort());
for (const [dir, rec] of [[actual, r], [anterior, old]]) {
  assert.deepEqual(bytes(dir, 'captura-nativo.jsonl'), bytes(dir, 'captura-wasi.jsonl'));
  assert.equal(rec.hallazgos.map(x => x.salida).join(''), bytes(dir, 'captura-nativo.jsonl').toString());
  assert.equal(rec.conformes, rec.hallazgos.filter(good).length);
  for (const x of rec.hallazgos) assert.equal(x.respuesta_correcta, x.salida === x.esperado);
}
const relations = Object.fromEntries(Object.entries(o.clases).map(([kind, groups]) => [kind, groups.map(group => {
  const n = new Set(group.map(id => cur.get(id).salida)).size;
  const relation = kind === 'contrastes' ? n === group.length : n === 1;
  const correct = group.every(id => good(cur.get(id)));
  return {ids: group, relacion_bruta: relation, miembros_correctos: correct, conforme_funcional: relation && correct};
})]));
const positives = r.hallazgos.filter(x => JSON.parse(x.esperado).estado === 'DATO_EXPERIMENTAL');
const negatives = r.hallazgos.filter(x => JSON.parse(x.esperado).estado !== 'DATO_EXPERIMENTAL');
const failures = r.hallazgos.filter(x => !good(x));
save('LECTURA-FUNCIONAL.json', {
  version: 'IE-004/1', captura: 'CONTEXTO-001', comparada_con: '002@927f24d57701dda579ff822da7a7fdaf1eeed69e',
  criterio: o.criterio_relacional,
  total: r.casos, identificaciones_correctas: r.identificacion_correcta, respuestas_correctas: r.respuesta_correcta,
  conformes_conjuntos: r.conformes,
  consultas_con_dato: {total: positives.length, correctas: positives.filter(good).length},
  diagnosticos_y_denegaciones: {total: negatives.length, correctos: negatives.filter(good).length},
  datos_indebidos_en_captura: r.hallazgos.filter(x => JSON.parse(x.salida).estado === 'DATO_EXPERIMENTAL' && !good(x)).length,
  fallos: failures.map(x => x.id), relaciones: relations,
  estado_funcional: failures.length ? 'FALLO' : 'CONFORME_EN_LOTE',
  aislamiento: 'NO_VERDE: el testigo independiente de sustitucion semantica sigue siendo aceptado por el receptor.'
});
const cases = ids.map(id => {
  const a = dp.get(id), b = dc.get(id), f1 = prev.get(id), f2 = cur.get(id);
  assert.equal(f1.esperado, f2.esperado, 'Expectativa cambiada: ' + id);
  const changes = fields.filter(k => (a.apoyos[k] ?? null) !== (b.apoyos[k] ?? null));
  return {
    id, pregunta_contexto_nota_identicos: true, expectativa_identica: true,
    ruta_y_diagnostico_iguales: equal(signature(a), signature(b)),
    apoyos_iguales: changes.length === 0,
    cambios_apoyos: Object.fromEntries(changes.map(k => [k, {anterior: a.apoyos[k] ?? null, actual: b.apoyos[k] ?? null}])),
    cuerpo_igual: f1.salida === f2.salida,
    correcto_en_002: good(f1), correcto_en_contexto: good(f2),
    conformidad_funcional_en_ambas: good(f1) && good(f2)
  };
});
save('COMPARACION-CONTEXTO.json', {
  objeto: 'Contraste de 24 entradas identicas; corpus y expectativas ya conocidos, no validacion inedita.',
  original_002: {commit: '927f24d57701dda579ff822da7a7fdaf1eeed69e', sha256: old.entrega_sha256, modelo: od.modelo, sesion: od.sesion, configuracion: od.configuracion},
  contexto_001: {commit_entrega: '40c70301c520cc79f2a400424dfc184be31b2d78', commit_informe: 'cd67d2854c756cb073d02cd1440a57815b109ed9', sha256: r.entrega_sha256, modelo: d.modelo, sesion: d.sesion, configuracion: d.configuracion},
  atribucion_contextual: 'Fuera del proyecto confirmado por Juan Antonio; no controla temperatura, semilla, memoria global ni otras instrucciones. No demuestra una causa interna del modelo.',
  resumen: {
    entradas_identicas: cases.length,
    rutas_y_diagnosticos_iguales: cases.filter(x => x.ruta_y_diagnostico_iguales).length,
    apoyos_iguales: cases.filter(x => x.apoyos_iguales).length,
    cuerpos_iguales: cases.filter(x => x.cuerpo_igual).length,
    correctos_en_ambas: cases.filter(x => x.conformidad_funcional_en_ambas).length,
    corregidos: cases.filter(x => !x.correcto_en_002 && x.correcto_en_contexto).map(x => x.id),
    regresiones: cases.filter(x => x.correcto_en_002 && !x.correcto_en_contexto).map(x => x.id),
    fallos_persistentes: cases.filter(x => !x.correcto_en_002 && !x.correcto_en_contexto).map(x => x.id)
  }, casos: cases
});
const measures = read(actual, 'medidas.json');
const median = a => { const b = [...a].sort((x,y) => x-y); const i = Math.floor(b.length/2); return b.length % 2 ? b[i] : (b[i-1]+b[i])/2; };
save('MEDICION.json', {
  objeto: 'Lote de 24 propuestas depositadas; incluye arranque de proceso, no inferencia del modelo.',
  procesos: measures.length,
  destinos: Object.fromEntries(['nativo', 'wasi'].map(dest => {
    const rows = measures.filter(x => x.destino === dest && x.etiqueta.startsWith('captura-'));
    assert.equal(rows.length, r.repeticiones_receptor_por_destino);
    const output = bytes(actual, 'captura-' + dest + '.jsonl');
    for (const x of rows) { assert.equal(x.status, 0); assert.equal(x.salida_sha256, hash(output)); }
    const ms = rows.map(x => x.duracion_ns / 1e6);
    return [dest, {muestras_ms: ms, mediana_ms: median(ms), minimo_ms: Math.min(...ms), maximo_ms: Math.max(...ms),
      bytes_salida: output.length, sha256_salida: hash(output),
      cpu_rss: rows.every(x => x.time === null) ? 'NO_OBSERVABLES' : 'VER_MEDIDAS_ORIGINALES'}];
  })),
  inferencia_modelo: 'NO_MEDIDA',
  limite: 'Tres muestras en anfitrion compartido; no caracterizacion P5, no prueba de regresion ni de viabilidad productiva.'
});
console.log(JSON.stringify({funcional: read(actual, 'LECTURA-FUNCIONAL.json'), comparacion: read(actual, 'COMPARACION-CONTEXTO.json').resumen, medicion: read(actual, 'MEDICION.json')}));
