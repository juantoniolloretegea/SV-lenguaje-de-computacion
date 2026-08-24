"use strict";

// Arnés de navegador de R0.
// JavaScript sólo transporta bytes, invoca exports WebAssembly y compara el
// observable con el oráculo de prueba. No interpreta sintaxis ni semántica SVP.

const ERROR_FLAG = 1n << 63n;
const PTR_MASK = 0xffff_ffffn;
const LEN_MASK = 0x7fff_ffffn;
const encoder = new TextEncoder();
const decoder = new TextDecoder("utf-8", { fatal: true });

function unpackResult(packed) {
  const value = BigInt(packed);
  return {
    error: (value & ERROR_FLAG) !== 0n,
    ptr: Number(value & PTR_MASK),
    len: Number((value >> 32n) & LEN_MASK),
  };
}

function writeInternalBuffer(exports, exportName, text) {
  const bytes = encoder.encode(text);
  const ptr = exports[exportName](bytes.length);
  new Uint8Array(exports.memory.buffer, ptr, bytes.length).set(bytes);
}

function compileCase(exports, source, fileName) {
  writeInternalBuffer(exports, "sv_source_buffer", source);
  writeInternalBuffer(exports, "sv_file_buffer", fileName);

  const result = unpackResult(exports.sv_compile_svp_json());
  const bytes = new Uint8Array(exports.memory.buffer, result.ptr, result.len);
  const text = decoder.decode(bytes.slice());
  return { error: result.error, text };
}

async function main() {
  const [manifestResponse, wasmResponse] = await Promise.all([
    fetch("/artifacts/r0-browser/manifest.json", { cache: "no-store" }),
    fetch("/artifacts/r0-browser/sv_wasm.wasm", { cache: "no-store" }),
  ]);
  if (!manifestResponse.ok || !wasmResponse.ok) {
    throw new Error("no se pudieron cargar los artefactos de paridad");
  }

  const manifest = await manifestResponse.json();
  const wasmBytes = await wasmResponse.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(wasmBytes, {});
  const exports = instance.exports;

  const required = [
    "memory",
    "sv_source_buffer",
    "sv_file_buffer",
    "sv_compile_svp_json",
  ];
  for (const name of required) {
    if (!(name in exports)) {
      throw new Error(`export WebAssembly ausente: ${name}`);
    }
  }

  const versions = {
    grammar: `${exports.sv_grammar_version_major()}.${exports.sv_grammar_version_minor()}`,
    ir: `${exports.sv_ir_version_major()}.${exports.sv_ir_version_minor()}`,
    serializer: `${exports.sv_serializer_version_major()}.${exports.sv_serializer_version_minor()}.${exports.sv_serializer_version_patch()}`,
  };
  if (versions.grammar !== "0.2" || versions.ir !== "0.3" || versions.serializer !== "0.1.0") {
    throw new Error(`versiones WebAssembly inesperadas: ${JSON.stringify(versions)}`);
  }

  const failures = [];
  let validOk = 0;
  let invalidOk = 0;

  for (const testCase of manifest.cases) {
    const result = compileCase(exports, testCase.source, testCase.file_name);
    if (testCase.category === "valid") {
      if (result.error) {
        failures.push(`VALID ${testCase.name}: WebAssembly rechazó: ${result.text}`);
      } else if (result.text !== testCase.expected_stdout.trimEnd()) {
        failures.push(`VALID ${testCase.name}: stdout WebAssembly != nativo`);
      } else {
        validOk += 1;
      }
    } else if (!result.error) {
      failures.push(`INVALID ${testCase.name}: WebAssembly aceptó la entrada`);
    } else {
      invalidOk += 1;
    }
  }

  const summary = {
    source_head: manifest.source_head,
    base_head: manifest.base_head,
    versions,
    valid_ok: validOk,
    invalid_ok: invalidOk,
    failures,
  };

  document.body.dataset.status = failures.length === 0 ? "pass" : "fail";
  document.getElementById("result").textContent = JSON.stringify(summary);
  document.title = failures.length === 0 ? "SV R0 browser parity PASS" : "SV R0 browser parity FAIL";
}

main().catch((error) => {
  document.body.dataset.status = "fail";
  document.getElementById("result").textContent = JSON.stringify({
    failures: [String(error && error.stack ? error.stack : error)],
  });
  document.title = "SV R0 browser parity FAIL";
});
