// Lanzamiento, captura y comparación. La gramática sólo se ejecuta en Rust.
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { performance } from 'node:perf_hooks';
const dir = path.dirname(new URL(import.meta.url).pathname);
const rustc = process.env.IE004_RUSTC || 'rustc';
const salida = process.env.IE004_SALIDA || path.join(dir, 'resultados');
if (fs.existsSync(salida)) throw new Error('La salida ya existe: no sobrescribir evidencia ni repetir implícitamente.');
fs.mkdirSync(salida, { recursive: true });
const hash = b => crypto.createHash('sha256').update(b).digest('hex');
const guardar = (p, x) => fs.writeFileSync(path.join(salida, p), typeof x === 'string' ? x : JSON.stringify(x, null, 2) + '\n');
const corpus = JSON.parse(fs.readFileSync(path.join(dir, 'CASOS_PUBLICOS.json'), 'utf8'));
const huellasFijadas = {
  'sonda.rs': 'cd5aff63e5ee979bcecbc843eab4ef5bad3b76459e1ed5845d10b294c6a39f04',
  'casos.rs': '5e27c00b1688b465cc3690942abcb81885ca5c0eb82e860f707b1b9063523fd7',
  'CASOS_PUBLICOS.json': '94bf8e9fe270dfb62a993dad0b566b3f8a5bcc6b4b1fee1b239f3da8bdbdfa39'
};
for (const [p, h] of Object.entries(huellasFijadas)) assert.equal(hash(fs.readFileSync(path.join(dir, p))), h);
assert.equal(corpus.casos.length, 72);
function proceso(nombre, comando, args, limite) {
  const inicio = new Date().toISOString();
  const t = performance.now();
  const r = spawnSync(comando, args, { cwd: dir, encoding: 'utf8', timeout: limite, killSignal: 'SIGKILL', maxBuffer: 8 * 1024 * 1024 });
  const ms = performance.now() - t;
  guardar(nombre + '.stdout', r.stdout || '');
  guardar(nombre + '.stderr', r.stderr || '');
  const registro = { nombre, comando, args, inicio, fin: new Date().toISOString(), duracion_ms: ms, limite_ms: limite, exit_code: r.status, signal: r.signal, error: r.error?.message || null };
  guardar(nombre + '.json', registro);
  return { ...registro, stdout: r.stdout || '' };
}
const version = proceso('rustc-version', rustc, ['-vV'], 10000);
assert.equal(version.exit_code, 0);
assert.match(version.stdout, /^rustc 1\.98\.0 /);
guardar('ENTORNO.json', {
  fecha: new Date().toISOString(), rustc: version.stdout,
  sistema: { plataforma: os.platform(), release: os.release(), arquitectura: os.arch(), cpu_modelo: os.cpus()[0]?.model, cpus_logicas: os.cpus().length, memoria_host_bytes: os.totalmem(), node: process.version },
  huellas_fuentes: huellasFijadas,
  cortes_preparacion: { lenguaje: 'f63d97c2d10fac93d471f731e763f20f904e0904', laboratorio: '7fd99bda2d21930fc1910d63437980be4dcdb453' },
  reserva_leida: false,
  limites: { compilacion_total_ms: 480000, ejecucion_total_ms: 300000, proceso_prueba_ms: 30000, reintentos: 0 },
  alcance_tiempos: 'Una ejecución local por configuración. Incluye arranque y salida; WASI incluye carga/compilación del módulo. No mide latencia del modelo, percentiles, tiempo real ni rendimiento del corrector completo.'
});
let compilacionMs = 0, ejecucionMs = 0;
const corridas = [];
for (const destino of ['nativo', 'wasi']) for (const perfil of ['debug', 'release']) {
  const id = destino + '-' + perfil;
  const binario = path.join(salida, id + (destino === 'wasi' ? '.wasm' : '.bin'));
  const flags = ['--edition=2021', '-C', 'opt-level=' + (perfil === 'debug' ? '0' : '3'), '-C', 'overflow-checks=' + (perfil === 'debug' ? 'yes' : 'no'), '-C', 'panic=abort'];
  if (destino === 'wasi') flags.push('--target', 'wasm32-wasip1', '-C', 'link-arg=--max-memory=67108864');
  const compilacion = proceso(id + '-compilar', rustc, [...flags, 'sonda.rs', '-o', binario], Math.min(120000, 480000 - compilacionMs));
  compilacionMs += compilacion.duracion_ms;
  if (compilacion.exit_code !== 0 || compilacionMs >= 480000) {
    corridas.push({ id, estado: 'COMPILACION_FALLIDA', compilacion });
    break;
  }
  const bin = fs.readFileSync(binario);
  const comando = destino === 'wasi' ? process.execPath : binario;
  const args = destino === 'wasi' ? [path.join(dir, 'wasi.mjs'), binario] : [];
  const ejecucion = proceso(id + '-ejecutar', comando, args, Math.min(30000, 300000 - ejecucionMs));
  ejecucionMs += ejecucion.duracion_ms;
  const lineas = ejecucion.stdout.trim().split('\n').filter(Boolean).map(s => JSON.parse(s));
  const cabecera = lineas.shift();
  const comparaciones = corpus.casos.map(c => {
    const rs = lineas.filter(r => r.id === c.id);
    return { id: c.id, grupo: c.grupo, esperado: c.esperado, observado: rs.length === 1 ? rs[0].estado : 'FALTA_O_DUPLICADO', coincide: rs.length === 1 && rs[0].estado === c.esperado, evidencia: rs[0] || null };
  });
  corridas.push({ id, estado: ejecucion.exit_code === 0 && lineas.length === 72 ? 'EJECUTADA' : 'EJECUCION_INCOMPLETA', compilacion_ms: compilacion.duracion_ms, ejecucion_ms: ejecucion.duracion_ms, binario: { nombre: path.basename(binario), bytes: bin.length, sha256: hash(bin) }, cabecera, coincidencias: comparaciones.filter(r => r.coincide).length, total: comparaciones.length, comparaciones });
  console.log(JSON.stringify({ id, compilacion_ms: compilacion.duracion_ms, ejecucion_ms: ejecucion.duracion_ms, cabecera, total: comparaciones.length, fallos: comparaciones.filter(r => !r.coincide).map(({ id, observado, evidencia }) => ({ id, observado, intentos: evidencia?.intentos })) }));
}
const completos = corridas.filter(c => c.estado === 'EJECUTADA');
const firma = c => JSON.stringify(c.comparaciones.map(r => r.evidencia));
const mismaSalidaCasos = completos.length === 4 && completos.every(c => firma(c) === firma(completos[0]));
guardar('RESULTADO.json', { instrumento: 'IE004-R10-SINTAXIS/1', registro: 'RETP-2026-131', reserva_leida: false, corrector_completo: false, compilacion_total_ms: compilacionMs, ejecucion_total_ms: ejecucionMs, coincidencia_entre_cuatro_configuraciones: mismaSalidaCasos, corridas, conclusion: completos.length === 4 && completos.some(c => c.coincidencias !== c.total) ? 'CALENDARIO_INSTRUMENTADO_INSUFICIENTE_PARA_EL_CORPUS_PUBLICO; CAPTURA_RESERVADA_NO_HABILITADA' : 'REVISAR_RESULTADOS_SIN_INFERIR_CUALIFICACION_SEMANTICA' });
const manifestados = fs.readdirSync(salida).filter(p => !p.endsWith('.bin') && !p.endsWith('.wasm')).map(p => {
  const b = fs.readFileSync(path.join(salida, p)); return { ruta: p, bytes: b.length, sha256: hash(b) };
});
guardar('MANIFIESTO_RESULTADOS.json', { archivos: manifestados, binarios: corridas.flatMap(c => c.binario ? [c.binario] : []), nota: 'Binarios reconstruibles con fuentes, rustc y flags registrados; se conservan localmente. Este manifiesto se excluye de su propia lista.' });
console.log(JSON.stringify({ mismaSalidaCasos, compilacionMs, ejecucionMs }));
