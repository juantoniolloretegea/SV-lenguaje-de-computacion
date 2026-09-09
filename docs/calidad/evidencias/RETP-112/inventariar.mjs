// Localizador documental del corte declarado en fuentes.json.
// No analiza SVP, no asigna doctrina a cadenas y no participa en producción.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, '../../../../');
const manifest = JSON.parse(fs.readFileSync(path.join(here, 'fuentes.json'), 'utf8'));
const sha = bytes => crypto.createHash('sha256').update(bytes).digest('hex');

// Conserva longitud y saltos de línea al ocultar comentarios y literales.
// No expande macros, no evalúa cfg y no construye un grafo de llamadas.
function mask(source, language) {
  const chars = source.split('');
  const blank = (a, b) => { for (let j = a; j < b; j++) if (chars[j] !== '\n') chars[j] = ' '; };
  for (let i = 0; i < source.length;) {
    const start = i;
    if (source.startsWith('//', i)) {
      i = source.indexOf('\n', i);
      if (i < 0) i = source.length;
      blank(start, i);
    } else if (source.startsWith('/*', i)) {
      i += 2;
      let depth = 1;
      while (i < source.length && depth) {
        if (source.startsWith('/*', i)) { depth++; i += 2; }
        else if (source.startsWith('*/', i)) { depth--; i += 2; }
        else i++;
      }
      if (depth) throw new Error('Comentario sin cierre en el localizador');
      blank(start, i);
    } else {
      const raw = language === 'rust' && !/[A-Za-z0-9_]/.test(source[i - 1] ?? '')
        ? /^(?:br|r)(#*)"/.exec(source.slice(i)) : null;
      if (raw) {
        const end = source.indexOf('"' + raw[1], i + raw[0].length);
        if (end < 0) throw new Error('Literal raw sin cierre');
        i = end + 1 + raw[1].length;
        blank(start, i);
      } else if (source[i] === '"' || (language === 'javascript' && "'`".includes(source[i]))) {
        const quote = source[i++];
        while (i < source.length) {
          if (source[i] === '\\') { i += 2; continue; }
          if (source[i++] === quote) break;
        }
        blank(start, i);
      } else if (language === 'rust' && source[i] === "'") {
        // Distingue el literal carácter de una vida Rust como 'a.
        const char = /^'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^'\\\n])'/u.exec(source.slice(i));
        if (char) { i += char[0].length; blank(start, i); }
        else i++;
      } else i++;
    }
  }
  return chars.join('');
}

function endCall(masked, start) {
  let depth = 0;
  for (let i = masked.indexOf('(', start); i < masked.length; i++) {
    if (masked[i] === '(') depth++;
    if (masked[i] === ')' && --depth === 0) return i + 1;
    // Sólo los paréntesis alteran depth.
    if (masked[i] !== ')' && masked[i] !== '(') continue;
  }
  throw new Error('Llamada sin cierre en el localizador');
}

const records = [];
for (const file of manifest.fuentes) {
  const bytes = fs.readFileSync(path.join(root, file.ruta));
  if (sha(bytes) !== file.sha256) throw new Error(`FUENTE_DISTINTA: ${file.ruta}`);
  const source = new TextDecoder('utf-8', { fatal: true }).decode(bytes);
  const masked = mask(source, file.lenguaje);
  const pattern = /\bErr\s*\(|\.map_err\s*\(|\.ok_or_else\s*\(|\beprintln!\s*\(|\bpacked_result\s*\(|\bthrow\s+new\s+Error\s*\(|\.(?:unwrap|expect)\s*\(/g;
  for (const hit of masked.matchAll(pattern)) {
    const start = hit.index;
    const line = source.slice(0, start).split('\n').length;
    if (!file.rangos.some(([a, b]) => a <= line && line <= b)) continue;
    const end = endCall(masked, start);
    const expression = source.slice(start, end);
    if (/^Err/.test(hit[0]) && /^\s*=>/.test(masked.slice(end))) continue;
    if (/^packed_result/.test(hit[0]) && !/true\s*\)$/.test(masked.slice(start, end))) continue;
    const category = /unwrap|expect/.test(hit[0]) ? 'asercion_interna'
      : /map_err/.test(hit[0]) ? 'adaptacion'
      : /eprintln|packed_result/.test(hit[0]) ? 'salida'
      : 'emision';
    const typed = /\b(FrontendError|FrameClosureViolation|UnsafeUResolution|InvalidAdmissibilitySpec)::([A-Za-z_]+)/.exec(expression);
    const literalCodes = [...new Set(expression.match(/\bE[0-9]{3}\b/g) ?? [])];
    records.push({
      id: `LOC-${String(records.length + 1).padStart(3, '0')}`,
      ruta: file.ruta,
      linea: line,
      linea_final: source.slice(0, end).split('\n').length,
      bytes: [Buffer.byteLength(source.slice(0, start)), Buffer.byteLength(source.slice(0, end))],
      clase: category,
      familia: file.familia,
      variante_escrita: typed ? `${typed[1]}::${typed[2]}` : null,
      codigos_literales: literalCodes,
      codigo_de_tipo_revisado: typed ? ({ FrameClosureViolation: 'E308', UnsafeUResolution: 'E305', InvalidAdmissibilitySpec: 'E110' }[typed[1]] ?? null) : null,
      constante_de_modulo_revisada: file.familia === 'datos_transicion' ? 'E406' : null,
      alcance: 'Localización estática; alcanzabilidad individual no acreditada por este recuento',
      expresion: expression,
      sha256_expresion: sha(Buffer.from(expression)),
    });
  }
}

const byFile = manifest.fuentes.map(file => ({ ruta: file.ruta, puntos: records.filter(r => r.ruta === file.ruta).length }));
const classes = Object.fromEntries(['emision', 'adaptacion', 'salida', 'asercion_interna'].map(c => [c, records.filter(r => r.clase === c).length]));
const result = {
  version: 'inventario-documental/1', corte: manifest.corte,
  total_puntos: records.length, por_clase: classes, por_archivo: byFile,
  cobertura_dinamica_acreditada: false, localizacion_implementada: false,
  advertencia: 'LOC identifica una ubicación de esta versión del inventario, no un código diagnóstico ni una causa constituida. Las adaptaciones y salidas pueden corresponder al mismo error. El lector es léxico: no prueba exhaustividad semántica, ni expansión, ni accesibilidad individual.',
  puntos: records,
};
process.stdout.write(JSON.stringify(result, null, 2) + '\n');
