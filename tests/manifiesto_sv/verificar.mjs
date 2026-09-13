// ES: Oráculo de bytes contra el acta canónica; no interpreta su doctrina.
// EN: Byte oracle against the canonical act; it does not interpret its doctrine.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { WASI } from 'node:wasi';

const root = path.resolve(import.meta.dirname, '../..');
const native = path.resolve(process.argv[2] ?? 'rust/target/release');
const wasiDir = path.resolve(process.argv[3] ?? 'rust/target/wasm32-wasip1/release');
const wasm = path.resolve(process.argv[4] ?? 'rust/target/wasm32-unknown-unknown/release/sv_wasm.wasm');
const canonical = fs.readFileSync(path.join(root, 'docs/calidad/ACTA_TECNICA_DE_PROHIBICION_ABSOLUTA_DE_USO_BELICO_Y_PRIMACIA_DE_SUPERVIVENCIA_HUMANA_EN_EL_SV_2026_03_26.md'));
const sha = b => createHash('sha256').update(b).digest('hex');
assert.equal(sha(canonical), '31ae77a963718ac72bbb74a4aac4fd06d788117167162835e0652f9fb2a60056');
assert.equal(canonical.length, 19134);
assert.deepEqual(fs.readFileSync(path.join(root, 'rust/sv_core/assets/manifiesto-sv.md')), canonical);
const temp = fs.mkdtempSync(path.join(os.tmpdir(), 'sv-manifiesto-'));
const checks = [];
const same = b => assert.deepEqual(b, canonical);
try {
  // ES: El ejecutable se mueve fuera del repositorio y recibe un señuelo ajeno.
  // EN: The executable moves outside the repository and receives an unrelated decoy.
  fs.writeFileSync(path.join(temp, 'manifiesto-sv.md'), 'NO ES EL ACTA / NOT THE ACT');
  for (const [binary, args] of [['manifiesto-sv', []], ['sv-native', ['manifiesto-sv']]]) {
    const copy = path.join(temp, binary);
    fs.copyFileSync(path.join(native, binary), copy);
    fs.chmodSync(copy, 0o700);
    const result = spawnSync(copy, args, { cwd: temp, env: { MANIFIESTO_SV: 'señuelo', LANG: 'C' } });
    assert.equal(result.status, 0); assert.equal(result.stderr.length, 0); same(result.stdout);
    checks.push(`${binary}: texto íntegro fuera del repositorio`);
    for (const extra of ['extra', '--profile', '--help']) {
      const bad = spawnSync(copy, [...args, extra], { cwd: temp });
      assert.equal(bad.status, 2); assert.equal(bad.stdout.length, 0);
    }
    checks.push(`${binary}: rechazo de tres argumentos incompatibles`);
    if (process.platform === 'linux') {
      const fd = fs.openSync('/dev/full', 'w');
      try { assert.equal(spawnSync(copy, args, { stdio: ['ignore', fd, 'pipe'] }).status, 2); }
      finally { fs.closeSync(fd); }
      checks.push(`${binary}: fallo de stdout separado de éxito`);
    }
    // ES: WASI sin preopens ni red, con stdout dirigido a un descriptor del host.
    // EN: WASI without preopens or network, with stdout sent to a host descriptor.
    for (const suffix of [[], ['extra']]) {
      const file = path.join(temp, `${binary}-${suffix.length}.out`);
      const fd = fs.openSync(file, 'w');
      const err = fs.openSync(path.join(temp, 'stderr'), 'w');
      try {
        const wasi = new WASI({ version: 'preview1', args: [binary, ...args, ...suffix], env: {}, preopens: {}, returnOnExit: true, stdout: fd, stderr: err });
        const module = await WebAssembly.compile(fs.readFileSync(path.join(wasiDir, `${binary}.wasm`)));
        const instance = await WebAssembly.instantiate(module, { wasi_snapshot_preview1: wasi.wasiImport });
        const status = wasi.start(instance);
        assert.equal(status, suffix.length ? 2 : 0);
      } finally { fs.closeSync(fd); fs.closeSync(err); }
      const output = fs.readFileSync(file);
      if (suffix.length) assert.equal(output.length, 0); else same(output);
    }
    checks.push(`${binary}: WASI sin acceso a archivos, positivo y negativo`);
  }
  const { instance } = await WebAssembly.instantiate(fs.readFileSync(wasm), {});
  for (let i = 0; i < 2; i++) {
    const packed = instance.exports.sv_manifiesto_sv();
    assert.equal(packed >> 63n, 0n);
    const ptr = Number(packed & 0xffffffffn), len = Number((packed >> 32n) & 0x7fffffffn);
    same(Buffer.from(new Uint8Array(instance.exports.memory.buffer, ptr, len)));
  }
  checks.push('adaptador WASM: dos consultas íntegras por ABI');
  const altered = Buffer.from(canonical); altered[0] ^= 1;
  assert.throws(() => same(altered));
  assert.throws(() => same(canonical.subarray(0, -1)));
  checks.push('sensibilidad: byte alterado y truncamiento detectados');
  console.log(JSON.stringify({ resultado: 'CONFORME', bytes: canonical.length, sha256: sha(canonical), comprobaciones: checks, limite: 'Disponibilidad e identidad documental; no prueba de bloqueo de usos prohibidos ni confianza frente a host comprometido.' }, null, 2));
} finally { fs.rmSync(temp, { recursive: true, force: true }); }
