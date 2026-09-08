// Subconjunto documental RETP-101, anterior a la conversión numérica de JavaScript.
// Lector externo independiente del generador Rust; no importa sus resultados.
export function parseDocumentaryJson(bytes) {
  let text;
  try { text = new TextDecoder('utf-8', {fatal: true, ignoreBOM: true}).decode(bytes); }
  catch { throw Error('JSON_UTF8'); }
  let at = 0;
  const fail = code => { throw Error(code); };
  const space = () => { while (/[ \t\r\n]/.test(text[at] ?? '\0')) at++; };
  const take = c => { if (text[at++] !== c) fail('JSON_SINTAXIS'); };
  function string() {
    const start = at;
    take('"');
    while (at < text.length) {
      const c = text[at++];
      if (c === '\\') { at++; continue; }
      if (c !== '"') continue;
      let value;
      try { value = JSON.parse(text.slice(start, at)); }
      catch { fail('JSON_SINTAXIS'); }
      // JSON.parse admite sustitutos aislados: el contrato documental los prohíbe.
      for (let i = 0; i < value.length; i++) {
        const unit = value.charCodeAt(i);
        if (unit >= 0xd800 && unit <= 0xdbff) {
          const low = value.charCodeAt(++i);
          if (!(low >= 0xdc00 && low <= 0xdfff)) fail('JSON_UNICODE');
        } else if (unit >= 0xdc00 && unit <= 0xdfff) fail('JSON_UNICODE');
      }
      return value;
    }
    fail('JSON_SINTAXIS');
  }
  function number() {
    const start = at;
    while (/[-+.eE0-9]/.test(text[at] ?? '\0')) at++;
    const token = text.slice(start, at);
    if (!/^(0|[1-9][0-9]*)$/.test(token)) fail('JSON_NUMERO_FORMA');
    if (BigInt(token) > 9007199254740991n) fail('JSON_NUMERO_RANGO');
    return Number(token); // La forma y el rango ya están comprobados sin redondeo.
  }
  function value() {
    space();
    if (text[at] === '"') return string();
    if (/[-0-9]/.test(text[at] ?? '\0')) return number();
    for (const [word, result] of [['true', true], ['false', false], ['null', null]]) {
      if (text.startsWith(word, at)) { at += word.length; return result; }
    }
    if (text[at] === '[') {
      at++; space();
      const items = [];
      if (text[at] === ']') { at++; return items; }
      for (;;) {
        items.push(value()); space();
        if (text[at] === ']') { at++; return items; }
        take(',');
      }
    }
    if (text[at] === '{') {
      at++; space();
      const result = Object.create(null);
      const keys = new Set();
      if (text[at] === '}') { at++; return result; }
      for (;;) {
        space();
        const key = string();
        if (keys.has(key)) fail('JSON_CLAVE_REPETIDA');
        keys.add(key);
        if (/^(0|[1-9][0-9]*)$/.test(key) && BigInt(key) <= 4294967294n) fail('JSON_CLAVE_INDICE');
        space(); take(':'); result[key] = value(); space();
        if (text[at] === '}') { at++; return result; }
        take(',');
      }
    }
    fail('JSON_SINTAXIS');
  }
  const result = value();
  space();
  if (at !== text.length) fail('JSON_SINTAXIS');
  return result;
}
