import assert from 'node:assert/strict';
import {test} from 'node:test';
import {loadCases, states, encode, decode, digest, representations, recover, restore,
  sideInformation, observe, certify, campaign} from './run.mjs';

const cases = loadCases();
const family = cases.families[0];
const domain = states(family);
const first = domain[0].packet;

test('la enumeración completa tiene la matriz comprometida', () => {
  assert.deepEqual(campaign().totals, {families: 6, states: 48, operations: 18, rows: 54,
    sufficient: 36, losses: 18, exact_recoveries: 288, lateral_recoveries: 144, technical_controls: 24});
});

test('rechaza dominio vacío, parcial y estado ajeno al producto constituido', () => {
  assert.throws(() => certify(family, []), /IncompleteOrForeignDomain/);
  assert.throws(() => certify(family, domain.slice(1)), /IncompleteOrForeignDomain/);
  const foreign = structuredClone(domain);
  foreign[0].packet.a.unit = 'UNIT-NO-CONSTITUIDA';
  assert.throws(() => certify(family, foreign), /IncompleteOrForeignDomain/);
});

test('el observado no puede convertirse automáticamente en el esperado', () => {
  assert.throws(() => certify(family, domain, representations, () => ['SALIDA-INCORRECTA']), /RecoveryMismatch/);
  const altered = structuredClone(family);
  altered.operations[0].expected[0][1] = 7; // era el texto "7"; no se permite coerción
  assert.throws(() => certify(altered), /RecoveryMismatch/);
});

test('rechaza frontera ampliada y operación hecha constante para esconder pérdida', () => {
  const frontier = structuredClone(family);
  frontier.operations[0].frontier = 2;
  assert.throws(() => certify(frontier), /FrontierMismatch/);
  const constant = structuredClone(family);
  constant.operations[0].expected[1] = constant.operations[0].expected[0];
  assert.throws(() => certify(constant), /IndistinctOperation/);
});

test('un hash o ID de fuente oculto altera H y anula el testigo declarado', () => {
  assert.throws(() => certify(family, domain, packet => {
    const reps = representations(packet);
    reps[1].source_sha256 = digest(encode(packet));
    return reps;
  }), /ReductionMismatch/);
});

test('la recuperación sólo con H reducido carece de las dependencias perdidas', () => {
  for (const f of cases.families) {
    const h = encode(representations(states(f)[0].packet)[2]);
    assert.throws(() => recover(h, f.operations[0]), /RepresentationInsufficient/);
    assert.throws(() => recover(h, f.operations[1]), /RepresentationInsufficient/);
    assert.deepEqual(recover(h, f.operations[2]), f.operations[2].expected[0]);
  }
});

test('S requiere versión, contenido y vínculo a la identidad P declarada', () => {
  const h = encode(representations(first)[2]);
  const s = sideInformation(first);
  const pin = digest(encode(first));
  assert.deepEqual(decode(restore(h, s, pin)), first);
  assert.throws(() => restore(h, s), /SideSourceMismatch/);
  assert.throws(() => restore(h, {...s, schema: 'S-FIF/2'}, pin), /SideVersionMismatch/);
  assert.throws(() => restore(h, {...s, a: domain[2].packet.a}, pin), /SideContentMismatch/);
  // Misma H, S auténtica de otro estado y digest internamente correcto: P detecta el intercambio.
  assert(encode(representations(domain[2].packet)[2]).equals(h));
  assert.throws(() => restore(h, sideInformation(domain[2].packet), pin), /SideSourceMismatch/);
});

test('bytes inválidos, homónimos, precisión y normalización no pasan silenciosamente', () => {
  for (const text of ['{"a":1,"a":2}', '{"x":9007199254740993}', '{"x":-0}', '{ "a": 1 }', 'null\n']) {
    assert.throws(() => decode(Buffer.from(text)), /WireEncodingMismatch/);
  }
  assert.throws(() => decode(Buffer.from([0xc3, 0x28])));
  assert.deepEqual(decode(encode({number: 1, string: '1', flag: true, missing: null, list: ['b', 'a']})),
    {number: 1, string: '1', flag: true, missing: null, list: ['b', 'a']});
});

test('captura fallida, no admisión, configuración y transporte quedan separados', () => {
  assert.deepEqual(observe(Buffer.from('{'), domain), {kind: 'CaptureFailure'});
  assert.deepEqual(observe(encode({...first, schema: 'FOREIGN/1'}), domain), {kind: 'NotAdmitted'});
  assert.throws(() => observe(encode(first), domain, 'CAPTURE-FIF/2'), /MissingOrWrongConfiguration/);
  assert.throws(() => recover(Buffer.from('{'), family.operations[0]));
  assert.equal(observe(encode(first), domain).kind, 'Ok');
});

test('mismo fichero o imagen no identifica derivación ni interpretación', () => {
  for (const f of [cases.families[1], cases.families[2]]) {
    const [x, , y] = states(f);
    assert.deepEqual(x.packet.fixed, y.packet.fixed);
    assert.notDeepEqual(recover(encode(x.packet), f.operations[0]), recover(encode(y.packet), f.operations[0]));
    assert(encode(representations(x.packet)[1]).equals(encode(representations(y.packet)[1])));
  }
});

test('orden no equivale a constancia de administración ni ubicación a episodio', () => {
  for (const f of [cases.families[3], cases.families[4]]) {
    const [x, , y] = states(f);
    assert.deepEqual(x.packet.context, y.packet.context);
    assert.notDeepEqual(recover(encode(x.packet), f.operations[0]), recover(encode(y.packet), f.operations[0]));
  }
});

test('historia conserva orden causal, anterioridad y procedencia aunque la vista coincida', () => {
  const f = cases.families[5];
  const packet = states(f)[0].packet;
  const reordered = structuredClone(packet);
  reordered.a.causal_order = reordered.fixed.received_order;
  assert.notDeepEqual(recover(encode(reordered), f.operations[0]), f.operations[0].expected[0]);
  assert.deepEqual(observe(encode(reordered), states(f)), {kind: 'NotAdmitted'});
  const [x, , y] = states(f);
  assert.equal(x.packet.fixed.current_state, y.packet.fixed.current_state);
  assert.notDeepEqual(recover(encode(x.packet), f.operations[0]), recover(encode(y.packet), f.operations[0]));
});
