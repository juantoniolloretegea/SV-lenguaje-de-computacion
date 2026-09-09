// Caracterización del corte 26ebc9f1; no comprueba una reparación ni ejecuta R1.
// Uso: node sondar.mjs <directorio rust/target extraído> <directorio temporal>.
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import assert from 'node:assert/strict';

assert.equal(process.argv.length, 4, 'Se requieren los dos directorios indicados.');
const raiz = path.resolve(process.argv[2]);
const temporal = path.resolve(process.argv[3]);
fs.mkdirSync(temporal, { recursive: true });
const huella = bytes => createHash('sha256').update(bytes).digest('hex');
const identidades = {
  'release/sv-native': 'cb0db4112cea29ed01972f88a549b0add8bb43a261034f0e8f37cb04974b8e7a',
  'release/examples/assembly_probe': '5743942312962a351a152f106f49b89651e04c9ca6e14f59d49bc178638233e2',
  'wasm32-unknown-unknown/release/sv_wasm.wasm': '8ab9af22232c4dfe50a7bfd97cfd587ff0f881ef36ddbabf462f7140bbb2d728',
};
for (const [archivo, sha] of Object.entries(identidades)) {
  assert.equal(huella(fs.readFileSync(path.join(raiz, archivo))), sha, archivo);
}
const bytesCasos = fs.readFileSync(new URL('casos.json', import.meta.url));
const casos = JSON.parse(bytesCasos.toString('utf8'));
assert.equal(casos.length, 13);
const modulo = fs.readFileSync(path.join(raiz, 'wasm32-unknown-unknown/release/sv_wasm.wasm'));
const { instance } = await WebAssembly.instantiate(modulo, {});
const e = instance.exports;
const codificador = new TextEncoder();
const decodificador = new TextDecoder('utf-8', { fatal: true });
function escribir(nombre, bytes) {
  const ptr = e[nombre](bytes.length);
  new Uint8Array(e.memory.buffer, ptr, bytes.length).set(bytes);
}
function leer(empaquetado) {
  const ptr = Number(empaquetado & 0xffffffffn);
  const longitud = Number((empaquetado >> 32n) & 0x7fffffffn);
  return {
    error: Boolean(empaquetado & (1n << 63n)),
    texto: decodificador.decode(new Uint8Array(e.memory.buffer, ptr, longitud)),
  };
}
const resultados = [];
for (const caso of casos) {
  const argumentos = [];
  const unidades = [];
  caso.units.forEach((u, i) => {
    assert.ok(['en', 'es'].includes(u.profile));
    assert.equal(path.basename(u.file), u.file);
    const bytes = u.hex ? Buffer.from(u.hex, 'hex') : codificador.encode(u.source);
    const archivo = path.join(temporal, u.file);
    fs.writeFileSync(archivo, bytes);
    argumentos.push('--profile', u.profile, archivo);
    escribir(i ? 'sv_assembly_source_b_buffer' : 'sv_source_buffer', bytes);
    escribir(i ? 'sv_assembly_file_b_buffer' : 'sv_file_buffer', codificador.encode(u.file));
    unidades.push({ ...u, sha256: huella(bytes) });
  });
  const binario = caso.units.length === 1 ? 'release/sv-native' : 'release/examples/assembly_probe';
  const nativo = spawnSync(path.join(raiz, binario), argumentos, { encoding: 'utf8', timeout: 10000 });
  if (nativo.error) throw nativo.error;
  assert.equal(nativo.signal, null, caso.id);
  const perfiles = caso.units.map(u => u.profile === 'es' ? 1 : 0);
  const wasm = leer(caso.units.length === 1
    ? e.sv_compile_svp_json_profile(perfiles[0])
    : e.sv_compile_svp_assembly_json(...perfiles));
  assert.equal(nativo.status, caso.technical ? 2 : caso.expected === null ? 0 : 1, caso.id);
  assert.equal(wasm.error, caso.expected !== null, caso.id);
  if (caso.expected === null) {
    assert.equal(nativo.stderr, '', caso.id);
    assert.equal(nativo.stdout, wasm.texto + '\n', caso.id);
    assert.equal(JSON.parse(wasm.texto).ir_version, '0.3', caso.id);
  } else {
    assert.equal(nativo.stdout, '', caso.id);
    assert.equal(wasm.texto, caso.expected, caso.id);
    // El texto de sistema se custodia, pero no se exige idéntico entre sistemas.
    if (caso.technical) {
      assert.ok(nativo.stderr.startsWith('no se pudo leer '), caso.id);
    } else {
      assert.equal(nativo.stderr, 'SVP no admitido: ' + caso.expected + '\n', caso.id);
    }
  }
  resultados.push({ id: caso.id, unidades, nativo: {
    estado: nativo.status, salida: nativo.stdout, error: nativo.stderr,
  }, wasm });
}
process.stdout.write(JSON.stringify({
  corte: '26ebc9f139397b086ce630df358d6b0fb11d997b',
  artefacto: 10116699662,
  entorno: { node: process.version, plataforma: process.platform, arquitectura: process.arch, sistema: os.release() },
  binarios: identidades,
  casos_sha256: huella(bytesCasos),
  casos: resultados.length,
  ejecuciones: resultados.length * 2,
  caracterizacion_conforme: true,
  reparacion_diagnostica_verificada: false,
  navegador_real_ejecutado: false,
  compilacion_rust_local: false,
  resultados,
}, null, 2) + '\n');
