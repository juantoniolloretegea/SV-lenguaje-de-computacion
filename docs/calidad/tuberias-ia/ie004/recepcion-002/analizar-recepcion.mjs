import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import crypto from 'node:crypto';

// Lectura de resultados ya recibidos. No modifica propuestas, oráculos o receptor.
const [actual, anterior] = process.argv.slice(2);
assert.ok(actual && anterior, 'USO: node analizar-recepcion.mjs recepcion-002 recepcion-001');
const read = (dir, name) => JSON.parse(fs.readFileSync(path.join(dir, name), 'utf8'));
const save = (name, x) => fs.writeFileSync(path.join(actual, name), JSON.stringify(x, null, 2) + '\n');
const hash = b => crypto.createHash('sha256').update(b).digest('hex');
const r = read(actual, 'RECEPCION.json');
const old = read(anterior, 'RECEPCION.json');
const oracle = read(actual, 'ORACULO-ABIERTO.json');
const packet = read(actual, 'solicitudes-originales.json');
const oldPacket = read(anterior, 'solicitudes-originales.json');
const delivery = read(actual, 'entrega-original.json');
const oldDelivery = read(anterior, 'entrega-original.json');
const good = x => x.identificacion_correcta && x.respuesta_correcta;
const byId = Object.fromEntries(r.hallazgos.map(x => [x.id, x]));
const oldById = Object.fromEntries(old.hallazgos.map(x => [x.id, x]));
const fields = ['operacion', 'objeto', 'parametro', 'momento', 'campo'];
const signature = x => [x.ruta && fields.map(k => x.ruta[k]), x.diagnostico];
const relations = Object.fromEntries(Object.entries(r.relaciones).map(([kind, groups]) => [kind,
  groups.map(g => ({
    ids: g.ids, relacion_bruta: g.conforme,
    miembros_correctos: g.ids.every(id => good(byId[id])),
    conforme_funcional: g.conforme && g.ids.every(id => good(byId[id]))
  }))
]));
const repeated = oracle.repetidos_de_001.map(pair => {
  const source = packet.casos.find(x => x.id === pair.confirmacion);
  const previous = oldPacket.casos.find(x => x.id === pair.anterior);
  const originalCase = x => [x.pregunta, fields.map(k => x.contexto[k]), x.nota_externa];
  assert.deepEqual(originalCase(source), originalCase(previous), 'Repetido alterado: ' + pair.confirmacion);
  const a = oldDelivery.respuestas.find(x => x.id === pair.anterior);
  const b = delivery.respuestas.find(x => x.id === pair.confirmacion);
  const f1 = oldById[pair.anterior], f2 = byId[pair.confirmacion];
  return {
    ...pair, pregunta_contexto_nota_identicos: true,
    ruta_y_diagnostico_iguales: JSON.stringify(signature(a)) === JSON.stringify(signature(b)),
    cuerpo_igual: f1.salida === f2.salida,
    correcto_en_001: good(f1), correcto_en_002: good(f2),
    conformidad_funcional_en_ambas: good(f1) && good(f2),
    salida_001: JSON.parse(f1.salida), salida_002: JSON.parse(f2.salida)
  };
});
const counts = pred => r.hallazgos.filter(pred);
const positives = counts(x => JSON.parse(x.esperado).estado === 'DATO_EXPERIMENTAL');
const negatives = counts(x => JSON.parse(x.esperado).estado !== 'DATO_EXPERIMENTAL');
save('LECTURA-FUNCIONAL.json', {
  version: 'IE-004/1', entrega: '002', criterio: oracle.criterio_relacional,
  identificaciones_correctas: r.identificacion_correcta, respuestas_correctas: r.respuesta_correcta,
  total: r.casos, consultas_con_dato: {total: positives.length, correctas: positives.filter(good).length},
  diagnosticos_y_denegaciones: {total: negatives.length, correctos: negatives.filter(good).length},
  fallos: counts(x => !good(x)).map(x => x.id), relaciones: relations,
  estado: 'CANDIDATA_INSUFICIENTE_EN_CONFIRMACION_AISLAMIENTO_NO_VERDE'
});
save('COMPARACION-001-002.json', {
  criterio: oracle.criterio_entre_sesiones,
  alcance: 'Ocho entradas repetidas, no equivalencia del corpus completo ni aislamiento contextual demostrado.',
  sesion_001: old.sesion_declarada, sesion_002: r.sesion_declarada,
  independencia_contextual: 'NO_ACREDITADA: nueva conversación declarada, memoria de proyecto visible y relato del usuario sin traza íntegra de contexto.',
  resumen: {
    repetidos: repeated.length,
    rutas_y_diagnosticos_iguales: repeated.filter(x => x.ruta_y_diagnostico_iguales).length,
    cuerpos_iguales: repeated.filter(x => x.cuerpo_igual).length,
    correctos_en_ambas: repeated.filter(x => x.conformidad_funcional_en_ambas).length,
    correctos_en_001: repeated.filter(x => x.correcto_en_001).length,
    correctos_en_002: repeated.filter(x => x.correcto_en_002).length
  }, casos: repeated
});
const measures = read(actual, 'medidas.json');
const median = a => {const sorted = [...a].sort((x,y) => x-y); return sorted[Math.floor(sorted.length/2)];};
save('MEDICION.json', {
  objeto: 'Lote de 24 propuestas depositadas, tres reproducciones por destino con arranque de proceso.',
  destinos: Object.fromEntries(['nativo', 'wasi'].map(dest => {
    const m = measures.filter(x => x.destino === dest && x.etiqueta.startsWith('captura-'));
    assert.equal(m.length, 3);
    const ns = m.map(x => x.duracion_ns);
    const output = fs.readFileSync(path.join(actual, 'captura-' + dest + '.jsonl'));
    return [dest, {muestras_ms: ns.map(n => n/1e6), mediana_ms: median(ns)/1e6,
      minimo_ms: Math.min(...ns)/1e6, maximo_ms: Math.max(...ns)/1e6,
      bytes_salida: output.length, sha256_salida: hash(output),
      cpu_rss: m.every(x => x.time === null) ? 'NO_OBSERVABLES' : 'VER_MEDIDAS_ORIGINALES'}];
  })),
  procesos: measures.length, inferencia_modelo: 'NO_MEDIDA',
  limite: 'Tres muestras de proceso en anfitrión compartido no aíslan el coste del receptor ni prueban regresión. No comparación de rendimiento con ES27 o con el lote distinto 001.'
});
console.log(JSON.stringify({funcional: read(actual, 'LECTURA-FUNCIONAL.json'), repetidos: read(actual, 'COMPARACION-001-002.json').resumen, medicion: read(actual, 'MEDICION.json')}));
