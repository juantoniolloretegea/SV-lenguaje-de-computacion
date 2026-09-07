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

function readCompileResult(exports, packed) {
  const result = unpackResult(packed);
  const bytes = new Uint8Array(exports.memory.buffer, result.ptr, result.len);
  const copy = bytes.slice();
  return { error: result.error, text: decoder.decode(copy), bytes: copy };
}

function compileCase(exports, source, fileName) {
  writeInternalBuffer(exports, "sv_source_buffer", source);
  writeInternalBuffer(exports, "sv_file_buffer", fileName);
  return readCompileResult(exports, exports.sv_compile_svp_json());
}

function compileProfileCase(exports, source, fileName, profileCode) {
  writeInternalBuffer(exports, "sv_source_buffer", source);
  writeInternalBuffer(exports, "sv_file_buffer", fileName);
  return readCompileResult(exports, exports.sv_compile_svp_json_profile(profileCode));
}

function compileUnits(exports, units) {
  const code = profile => ({ en: 0, es: 1 })[profile];
  if (units.length === 1) {
    return compileProfileCase(exports, units[0].source, units[0].file_name, code(units[0].profile));
  }
  if (units.length !== 2) throw new Error("testigo de ensamblaje inválido");
  writeInternalBuffer(exports, "sv_source_buffer", units[0].source);
  writeInternalBuffer(exports, "sv_file_buffer", units[0].file_name);
  writeInternalBuffer(exports, "sv_assembly_source_b_buffer", units[1].source);
  writeInternalBuffer(exports, "sv_assembly_file_b_buffer", units[1].file_name);
  return readCompileResult(exports, exports.sv_compile_svp_assembly_json(code(units[0].profile), code(units[1].profile)));
}

function equalBytes(left, right) {
  return left.length === right.length && left.every((value, index) => value === right[index]);
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
  if (manifest.schema !== "sv-r0-browser-parity-manifest-v3" || manifest.failures.length) {
    throw new Error("manifiesto inválido o con fallos previos");
  }
  if (!manifest.counts.valid || !manifest.counts.invalid ||
      manifest.cases.length !== manifest.counts.valid + manifest.counts.invalid) {
    throw new Error("corpus vacío o incompleto");
  }
  const wasmBytes = await wasmResponse.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(wasmBytes, {});
  const exports = instance.exports;

  const required = [
    "memory",
    "sv_source_buffer",
    "sv_file_buffer",
    "sv_compile_svp_json",
    "sv_compile_svp_json_profile",
    "sv_assembly_source_b_buffer",
    "sv_assembly_file_b_buffer",
    "sv_compile_svp_assembly_json",
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
      } else if (!equalBytes(result.bytes, encoder.encode(testCase.expected_payload))) {
        failures.push(`VALID ${testCase.name}: stdout WebAssembly != nativo`);
      } else {
        validOk += 1;
      }
    } else if (testCase.category !== "invalid") {
      failures.push(`categoría desconocida: ${testCase.category}`);
    } else if (!result.error) {
      failures.push(`INVALID ${testCase.name}: WebAssembly aceptó la entrada`);
    } else if (!equalBytes(result.bytes, encoder.encode(testCase.expected_diagnostic))) {
      failures.push(`INVALID ${testCase.name}: diagnóstico WASM != nativo`);
    } else {
      invalidOk += 1;
    }
  }

  let sensitivityOk = 0;
  if (!Array.isArray(manifest.sensitivity_cases) || manifest.sensitivity_cases.length !== 5) {
    throw new Error('faltan las cinco fuentes de sensibilidad');
  }
  for (const testCase of manifest.sensitivity_cases) {
    const result = compileCase(exports, testCase.source, testCase.file_name);
    if (result.error !== testCase.error || result.text !== testCase.expected_payload) {
      failures.push(`SENSITIVITY ${testCase.name}: estado o bytes distintos del nativo comprobado`);
    } else {
      sensitivityOk += 1;
    }
  }

  const closedDomainProbes = [
    {
      name: "DG-01-EN",
      profile: 0,
      source: "semantic_relation R { kind: ForeignRelation; }",
    },
    {
      name: "DG-02-EN",
      profile: 0,
      source: "pattern P { kind: ForeignPattern; arity: 1; }",
    },
    {
      name: "DG-03-EN",
      profile: 0,
      source: "semantic_relation R { kind: DeclaredRelation; } graph G { nodes: []; edges: []; relation: R; regime: ForeignRegime; }",
    },
    {
      name: "DG-01-ES",
      profile: 1,
      source: "relación_semántica R { clase: RelaciónExtranjera; }",
    },
    {
      name: "DG-02-ES",
      profile: 1,
      source: "patrón P { clase: PatrónExtranjero; aridad: 1; }",
    },
    {
      name: "DG-03-ES",
      profile: 1,
      source: "relación_semántica R { clase: RelaciónDeclarada; } grafo G { nodos: []; aristas: []; relación: R; régimen: RégimenExtranjero; }",
    },
  ];

  let closedDomainsOk = 0;
  for (const probe of closedDomainProbes) {
    const result = compileProfileCase(exports, probe.source, `${probe.name}.svp`, probe.profile);
    if (!result.error) {
      failures.push(`${probe.name}: WebAssembly aceptó un literal fuera de un dominio cerrado`);
    } else if (!result.text.includes("dominio cerrado")) {
      failures.push(`${probe.name}: rechazo sin acreditar la frontera de dominio cerrado: ${result.text}`);
    } else {
      closedDomainsOk += 1;
    }
  }

  if (validOk !== manifest.counts.valid || invalidOk !== manifest.counts.invalid) {
    failures.push("recuentos ejecutados distintos del manifiesto");
  }
  let bridgeOk = 0;
  if (!manifest.bridge_cases || manifest.bridge_cases.length !== 18) {
    failures.push("banco BridgeSet ausente o incompleto");
  } else {
    for (const probe of manifest.bridge_cases) {
      const result = compileUnits(exports, probe.units);
      if (result.error !== probe.error || result.text !== probe.expected_payload) {
        failures.push(`BridgeSet ${probe.name}: divergencia frente al nativo comprobado`);
      } else { bridgeOk++; }
    }
  }
  let horizonOk = 0;
  if (!manifest.horizon_cases || manifest.horizon_cases.length !== 20) {
    failures.push("banco Horizon ausente o incompleto");
  } else {
    for (const probe of manifest.horizon_cases) {
      const result = compileUnits(exports, probe.units);
      if (result.error !== probe.error || result.text !== probe.expected_payload) {
        failures.push(`Horizon ${probe.name}: divergencia frente al nativo comprobado`);
      } else { horizonOk++; }
    }
  }
  let domainOk = 0;
  if (!manifest.domain_cases || manifest.domain_cases.length !== 24) {
    failures.push("banco Domain ausente o incompleto");
  } else {
    for (const probe of manifest.domain_cases) {
      const result = compileUnits(exports, probe.units);
      if (result.error !== probe.error || result.text !== probe.expected_payload) {
        failures.push(`Domain ${probe.name}: divergencia frente al nativo comprobado`);
      } else { domainOk++; }
    }
  }
  let optionalOk = 0;
  if (!manifest.optional_cases || manifest.optional_cases.length !== 64) {
    failures.push("banco Campos opcionales ausente o incompleto");
  } else {
    for (const probe of manifest.optional_cases) {
      const result = compileUnits(exports, probe.units);
      if (result.error !== probe.error || result.text !== probe.expected_payload) {
        failures.push(`Campos opcionales ${probe.name}: divergencia frente al nativo comprobado`);
      } else { optionalOk++; }
    }
  }
  let ternarizerOk = 0;
  if (!manifest.ternarizer_cases || manifest.ternarizer_cases.length !== 40) {
    failures.push("banco K1-T ausente o incompleto");
  } else {
    for (const probe of manifest.ternarizer_cases) {
      const result = compileUnits(exports, probe.units);
      if (result.error !== probe.error || result.text !== probe.expected_payload) {
        failures.push(`K1-T ${probe.name}: divergencia frente al nativo comprobado`);
      } else { ternarizerOk++; }
    }
  }
  const summary = {
    source_head: manifest.source_head,
    base_head: manifest.base_head,
    versions,
    valid_ok: validOk,
    invalid_ok: invalidOk,
    closed_domains_ok: closedDomainsOk,
    sensitivity_ok: sensitivityOk,
    bridge_ok: bridgeOk,
    horizon_ok: horizonOk,
    domain_ok: domainOk,
    optional_ok: optionalOk,
    ternarizer_ok: ternarizerOk,
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
