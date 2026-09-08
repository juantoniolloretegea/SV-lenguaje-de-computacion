import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {parseDocumentaryJson} from './documentary_json.mjs';

const parse = text => parseDocumentaryJson(Buffer.from(text, 'utf8'));
const rejected = (text, cause) => assert.throws(() => parse(text), error => error.message === cause);

test('Conserva valores fijados, orden y escalares Unicode', () => {
  const value = parse(String.raw` {"z":[0,9007199254740991,true,false,null],"a":"\u00f1\ud83d\ude00"} `);
  assert.equal(JSON.stringify(value), '{"z":[0,9007199254740991,true,false,null],"a":"ñ😀"}');
  assert.equal(JSON.stringify(parse(String.raw`"\b\f\n\r\t\u0000\/\\\""`)), String.raw`"\b\f\n\r\t\u0000/\\\""`);
});
test('Rechaza conversiones numéricas antes de perder información', () => {
  for (const token of ['1.0', '1e0', '-0', '9007199254740990.1', '01']) rejected(token, 'JSON_NUMERO_FORMA');
  rejected('9007199254740992', 'JSON_NUMERO_RANGO');
});
test('Rechaza claves repetidas e índices; preserva claves ordinarias', () => {
  rejected('{"a":0,"a":1}', 'JSON_CLAVE_REPETIDA');
  rejected(String.raw`{"a":0,"\u0061":1}`, 'JSON_CLAVE_REPETIDA');
  for (const key of ['0', '1', '4294967294']) rejected(`{"${key}":0}`, 'JSON_CLAVE_INDICE');
  const input = '{"4294967295":0,"01":1,"__proto__":null}';
  assert.equal(JSON.stringify(parse(input)), input);
});
test('Rechaza sustitutos Unicode aislados y UTF-8 inválido', () => {
  for (const text of [String.raw`"\ud800"`, String.raw`"\udc00"`, String.raw`"\ud800\u0041"`]) rejected(text, 'JSON_UNICODE');
  for (const bytes of [[34,255,34], [34,237,160,128,34], [34,195]]) {
    assert.throws(() => parseDocumentaryJson(Uint8Array.from(bytes)), error => error.message === 'JSON_UTF8');
  }
});
test('Rechaza sintaxis inválida sin reparación', () => {
  for (const text of ['', '[1,]', '{"a":0,}', 'null null', '[1 2]', '{"a" 0}', String.raw`"\x"`, '"\n"']) rejected(text, 'JSON_SINTAXIS');
});
test('Admite el testigo G/H y conserva sus identificadores y ocho pares', () => {
  const source = parseDocumentaryJson(readFileSync(new URL('./testigos-gh.json', import.meta.url)));
  assert.equal(source.schema, 'TESTIGOS-GH/1');
  assert.deepEqual(source.witnesses.map(w => w.id), ['GH-DOC-01','GH-DOC-02','GH-DOC-03','GH-DOC-04','GH-DOC-05','GH-DOC-06','GH-DOC-07','GH-DOC-08']);
  for (const witness of source.witnesses) assert.deepEqual(witness.domain.states.map(s => s.id), ['x','y']);
});
