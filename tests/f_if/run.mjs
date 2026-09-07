// Observador de paquetes documentales F-IF. No analiza SV ni ejecuta operaciones de su IR.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {resolve} from 'node:path';

export const encode = value => Buffer.from(JSON.stringify(value), 'utf8');
export const pretty = value => JSON.stringify(value, null, 2) + '\n';
export const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const copy = value => structuredClone(value);
const equal = (a, b) => encode(a).equals(encode(b));
const fields = (value, names) => assert.deepEqual(Object.keys(value), names);
const location = name => new URL(name, import.meta.url);

// Acepta únicamente la codificación declarada: no normaliza números, homónimos o texto.
export function decode(bytes) {
  const text = new TextDecoder('utf-8', {fatal: true}).decode(bytes);
  const value = JSON.parse(text);
  assert(encode(value).equals(Buffer.from(bytes)), 'WireEncodingMismatch');
  return value;
}

export function loadCases() {
  const bytes = readFileSync(location('cases.json'));
  const value = JSON.parse(bytes.toString('utf8'));
  assert(Buffer.from(pretty(value)).equals(bytes), 'FixtureEncodingMismatch');
  return value;
}

export function states(family) {
  const result = [];
  for (const c of [0, 1]) for (const a of [0, 1]) for (const b of [0, 1]) {
    result.push({
      id: `${family.id}-${c}${a}${b}`,
      point: {context: c, a, b},
      packet: {
        schema: `${family.id}/packet/1`,
        context: copy(family.axes.context[c]),
        a: copy(family.axes.a[a]),
        b: copy(family.axes.b[b]),
        fixed: copy(family.fixed),
      },
    });
  }
  return result;
}

export function observe(bytes, domain, config = 'CAPTURE-FIF/1') {
  assert.equal(config, 'CAPTURE-FIF/1', 'MissingOrWrongConfiguration');
  let packet;
  try { packet = decode(bytes); } catch { return {kind: 'CaptureFailure'}; }
  if (!domain.some(state => equal(packet, state.packet))) return {kind: 'NotAdmitted'};
  return {kind: 'Ok', packet};
}

// F0 es el paquete; r0 elimina a; r1 elimina b. Se preserva el orden de todo lo demás.
export function reduce(packet, step) {
  assert([0, 1].includes(step));
  const result = copy(packet);
  const key = step === 0 ? 'a' : 'b';
  assert(Object.hasOwn(result, key), `MissingReductionField:${key}`);
  delete result[key];
  return result;
}

export function representations(packet) {
  const f0 = copy(packet);
  const f1 = reduce(f0, 0);
  return [f0, f1, reduce(f1, 1)];
}

export function recover(wire, operation) {
  const representation = decode(wire);
  return operation.paths.map(path => {
    let value = representation;
    for (const key of path.split('.')) {
      assert(value !== null && typeof value === 'object' && Object.hasOwn(value, key),
        `RepresentationInsufficient:${path}`);
      value = value[key];
    }
    return copy(value);
  });
}

export function sideInformation(packet) {
  return {schema: 'S-FIF/1', source_sha256: digest(encode(packet)), a: copy(packet.a), b: copy(packet.b)};
}

export function restore(wire, side, trustedSourceDigest) {
  const h = decode(wire);
  fields(h, ['schema', 'context', 'fixed']);
  fields(side, ['schema', 'source_sha256', 'a', 'b']);
  assert.equal(side.schema, 'S-FIF/1', 'SideVersionMismatch');
  assert.equal(side.source_sha256, trustedSourceDigest, 'SideSourceMismatch');
  const packet = {schema: h.schema, context: h.context, a: copy(side.a), b: copy(side.b), fixed: h.fixed};
  assert.equal(digest(encode(packet)), side.source_sha256, 'SideContentMismatch');
  return encode(packet);
}

export function validateFamily(family, domain) {
  fields(family.axes, ['context', 'a', 'b']);
  for (const alternatives of Object.values(family.axes)) {
    assert.equal(alternatives.length, 2);
    assert(!equal(alternatives[0], alternatives[1]), 'IndistinctAxis');
  }
  // Comparación contra el producto declarado completo; una lista filtrada no lo certifica.
  assert.deepEqual(domain, states(family), 'IncompleteOrForeignDomain');
  assert.equal(new Set(domain.map(s => digest(encode(s.packet)))).size, 8);
  assert.equal(family.operations.length, 3);
  assert.deepEqual(family.operations.map(op => op.axis), ['a', 'b', 'context']);
  for (const op of family.operations) {
    assert.equal(op.expected.length, 2);
    assert(!equal(op.expected[0], op.expected[1]), 'IndistinctOperation');
    assert(op.paths.length > 0 && op.paths.every(p => p.startsWith(op.axis + '.')));
    assert([0, 1, 2].includes(op.frontier));
  }
}

export function certify(family, domain = states(family), project = representations, recovery = recover) {
  validateFamily(family, domain);
  const reps = domain.map(s => project(s.packet));
  for (let i = 0; i < domain.length; i++) {
    assert(equal(reps[i][0], domain[i].packet), 'F0Mismatch');
    assert.equal(reps[i].length, 3);
    for (const step of [0, 1]) assert(equal(reps[i][step + 1], reduce(reps[i][step], step)), 'ReductionMismatch');
    assert.equal(observe(encode(domain[i].packet), domain).kind, 'Ok');
    for (const rep of reps[i]) assert(equal(decode(encode(rep)), rep), 'TransportMismatch');
  }
  const matrix = [];
  const certificates = [];
  let exactChecks = 0;
  let lateralChecks = 0;
  for (const op of family.operations) {
    const expected = domain.map(s => op.expected[s.point[op.axis]]);
    const rows = [];
    for (const level of [0, 1, 2]) {
      let witness = null;
      for (let i = 0; i < domain.length; i++) for (let j = i + 1; j < domain.length; j++) {
        const hi = encode(reps[i][level]);
        const hj = encode(reps[j][level]);
        if (hi.equals(hj) && !equal(expected[i], expected[j]) && witness === null) {
          witness = {x: domain[i].id, y: domain[j].id, transmitted_utf8: hi.toString('utf8'),
            transmitted_sha256: digest(hi), q_x: expected[i], q_y: expected[j]};
        }
      }
      if (witness === null) {
        // No basta no encontrar colisión: se ejecuta q sobre H sin punto, ID de caso ni oráculo.
        for (let i = 0; i < domain.length; i++) {
          assert(equal(recovery(encode(reps[i][level]), op), expected[i]), `RecoveryMismatch:${op.id}:${level}`);
          exactChecks++;
        }
      } else {
        // En esta cadena la distinción perdida es además una dependencia ausente de q.
        for (let i = 0; i < domain.length; i++) assert.throws(() => recover(encode(reps[i][level]), op), /RepresentationInsufficient:/);
      }
      const row = {family: family.id, operation: op.id, level,
        judgment: witness === null ? 'SUFICIENTE_EN_X_FINITO' : 'PERDIDA_DEMOSTRADA_EN_X_FINITO',
        tested_states: domain.length, recovery: witness === null ? op.paths : null, witness};
      rows.push(row);
      matrix.push(row);
    }
    const accepted = rows.filter(row => row.witness === null).map(row => row.level);
    assert.deepEqual(accepted, Array.from({length: op.frontier + 1}, (_, i) => i), 'FrontierMismatch');
    certificates.push({operation: op.id, frontier: op.frontier, accepted_levels: accepted,
      recovery: op.paths, boundary_level: op.frontier < 2 ? op.frontier + 1 : null});
    for (let i = 0; i < domain.length; i++) {
      const combined = restore(encode(reps[i][2]), sideInformation(domain[i].packet), digest(encode(domain[i].packet)));
      assert(equal(recovery(combined, op), expected[i]), 'LateralRecoveryMismatch');
      lateralChecks++;
    }
  }
  return {matrix, certificates, exactChecks, lateralChecks};
}

export function campaign(cases = loadCases()) {
  assert.equal(cases.schema, 'F-IF/1');
  assert.equal(cases.contract, 'F-SV/0.1-candidata');
  assert.deepEqual(cases.families.map(f => f.id), Array.from({length: 6}, (_, i) => `IF-IMM-0${i + 1}`));
  assert.equal(new Set(cases.families.flatMap(f => f.operations.map(op => op.id))).size, 18);
  const families = [];
  for (const f of cases.families) {
    const domain = states(f);
    const result = certify(f, domain);
    const first = domain[0].packet;
    assert.deepEqual(observe(Buffer.from('{'), domain), {kind: 'CaptureFailure'});
    assert.deepEqual(observe(encode({...first, foreign: true}), domain), {kind: 'NotAdmitted'});
    assert.throws(() => observe(encode(first), domain, null), /MissingOrWrongConfiguration/);
    assert.throws(() => recover(Buffer.from('{'), f.operations[0]));
    families.push({id: f.id,
      sources: domain.map(s => ({id: s.id, point: s.point, bytes: encode(s.packet).length, sha256: digest(encode(s.packet))})),
      technical_controls: ['CaptureFailure', 'NotAdmitted', 'MissingOrWrongConfiguration', 'TransportFailure'],
      ...result});
  }
  const matrix = families.flatMap(f => f.matrix);
  return {schema: 'F-IF-EVIDENCE/1', base: cases.base,
    inputs: ['cases.json', 'run.mjs'].map(name => ({path: `tests/f_if/${name}`, sha256: digest(readFileSync(location(name)))})),
    totals: {families: 6, states: 48, operations: 18, rows: matrix.length,
      sufficient: matrix.filter(row => row.witness === null).length,
      losses: matrix.filter(row => row.witness !== null).length,
      exact_recoveries: families.reduce((n, f) => n + f.exactChecks, 0),
      lateral_recoveries: families.reduce((n, f) => n + f.lateralChecks, 0), technical_controls: 24}, families};
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const report = campaign();
  assert.equal(pretty(report), readFileSync(location('evidence.json'), 'utf8'), 'EvidenceDrift: revisar el cambio, no actualizar esperados automáticamente');
  console.log(JSON.stringify({result: 'PASS', ...report.totals, evidence_sha256: digest(Buffer.from(pretty(report)))}));
}
