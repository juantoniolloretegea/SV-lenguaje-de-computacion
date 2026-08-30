"use strict";

const ERROR_FLAG = 1n << 63n;
const PTR_MASK = 0xffff_ffffn;
const LEN_MASK = 0x7fff_ffffn;
const MAX_SOURCE_BYTES = 1024 * 1024;
const encoder = new TextEncoder();
const fatalDecoder = new TextDecoder("utf-8", { fatal: true });

const I18N = {
  es: {
    laboratory: "LENGUAJE SV",
    title: "Lenguaje de Computación SV",
    subtitle: "SVP-ES + SVP-EN · programación bilingüe y ensamblaje multifuente en navegador.",
    repository: "Repositorio",
    diagnostics: "Diagnósticos",
    security: "Seguridad",
    features: "Características",
    history: "Historial Beta",
    interfaceLanguage: "Interfaz",
    sourceLanguage: "Perfil fuente",
    activeProfile: "Perfil fuente del código",
    profileExplanation: "El perfil fuente es explícito. Cambiarlo no traduce ni modifica el contenido del editor.",
    sourceCode: "Código fuente",
    editor: "Editor SVP",
    fileName: "Archivo",
    download: "Descargar .svp",
    selectedProfile: "Perfil fuente",
    run: "Ejecutar",
    clearOutput: "Limpiar resultado",
    execution: "Ejecución local",
    result: "Resultado",
    notExecuted: "Sin ejecutar",
    library: "Biblioteca",
    examples: "Ejemplos",
    examplesHelp: "Los ejemplos se cargan sólo cuando usted lo solicita. El idioma de la interfaz y el perfil fuente del código son independientes.",
    architectureTitle: "Una sola identidad computacional",
    architectureText: "SVP-ES y SVP-EN se canonicalizan antes del análisis sintáctico y convergen en el mismo analizador, IR y semántica. Los identificadores, cadenas, comentarios y datos del usuario no se traducen.",
    wasmSize: "Tamaño WebAssembly",
    projection: "Proyección",
    betaVersion: "Corte estable",
    assemblyTitle: "Ensamblaje multifuente",
    assemblyIntro: "Dos unidades .svp mantienen fronteras y perfiles fuente independientes. Se analizan bajo su perfil, convergen en representación canónica y se validan conjuntamente en una única IR. No se concatenan textos ni tokens entre archivos.",
    unitA: "Unidad A",
    unitB: "Unidad B",
    chooseFile: "Subir .svp",
    profile: "Perfil fuente",
    source: "Fuente",
    bytes: "bytes",
    loadIndependentDemo: "Demostración ES+EN independiente",
    loadLinkedDemo: "Demostración enlazada ES→EN",
    assemble: "Compilar y ensamblar",
    clearAssembly: "Limpiar ensamblaje",
    assemblyResult: "Resultado del ensamblaje",
    individualCheck: "Comprobación aislada",
    assemblyCheck: "Validación conjunta",
    waiting: "Pendiente",
    admitted: "Admitido",
    rejected: "No admitido",
    assemblyAdmitted: "Ensamblaje admitido",
    assemblyRejected: "Ensamblaje no admitido",
    sourceTooLarge: "La unidad supera el límite de seguridad de 1 MiB de esta interfaz.",
    invalidUtf8: "El archivo no es UTF-8 válido.",
    fileLoaded: "Archivo cargado",
    demoLoaded: "Demostración cargada",
    wasmLoading: "Comprobando identidad y cargando WebAssembly…",
    wasmReady: "WebAssembly: identidad, tamaño y exportaciones obligatorias verificados.",
    wasmError: "No se pudo inicializar WebAssembly",
    sourceHashMismatch: "El SHA-256 devuelto por la compilación no coincide con los bytes actuales del editor.",
    outputCleared: "Resultado limpiado.",
    downloadReady: "Fuente descargada sin traducción ni transformación.",
    uiChanged: "Idioma de interfaz cambiado; código y perfiles fuente permanecen inalterados.",
    profileChanged: "Perfil fuente cambiado; el código permanece inalterado.",
    loadedExample: "Ejemplo cargado",
    requiredProfile: "Perfil fuente activo",
    loadExample: "Cargar en el editor",
    footerText: "La interfaz sólo transporta bytes y presenta resultados. La compilación, el ensamblaje canónico y la validación global se ejecutan localmente en WebAssembly.",
    descriptionCell: "Declaración mínima de codominio, semántica y especificación de celda.",
    descriptionGate: "Compuerta con tabla de admisibilidad y operación de evaluación.",
    descriptionResolve: "Resolución constituida y consulta de campos de la operación.",
    descriptionCompose: "Composición de estructuras bajo el régimen de la gramática vigente.",
    descriptionQuery: "Consultas y contextos tipados reconocidos por el Lenguaje.",
    descriptionTrajectory: "Trayectoria y alternancia válida en el corpus de conformidad."
  },
  en: {
    laboratory: "SV LANGUAGE",
    title: "SV Computing Language",
    subtitle: "SVP-ES + SVP-EN · bilingual programming and multi-source assembly in the browser.",
    repository: "Repository",
    diagnostics: "Diagnostics",
    security: "Security",
    features: "Features",
    history: "Beta history",
    interfaceLanguage: "Interface",
    sourceLanguage: "Source profile",
    activeProfile: "Code source profile",
    profileExplanation: "The source profile is explicit. Changing it never translates or modifies the editor contents.",
    sourceCode: "Source code",
    editor: "SVP editor",
    fileName: "File",
    download: "Download .svp",
    selectedProfile: "Source profile",
    run: "Run",
    clearOutput: "Clear result",
    execution: "Local execution",
    result: "Result",
    notExecuted: "Not executed",
    library: "Library",
    examples: "Examples",
    examplesHelp: "Examples are loaded only when requested. Interface language and source profile remain independent.",
    architectureTitle: "One computational identity",
    architectureText: "SVP-ES and SVP-EN are canonicalized before parsing and converge on the same parser, IR and semantics. User identifiers, strings, comments and data are not translated.",
    wasmSize: "WebAssembly size",
    projection: "Projection",
    betaVersion: "Stable cut",
    assemblyTitle: "Multi-source assembly",
    assemblyIntro: "Two .svp units retain independent boundaries and source profiles. They are analyzed under their own profile, converge on canonical representation and are validated together in one IR. Text or token streams are never concatenated across files.",
    unitA: "Unit A",
    unitB: "Unit B",
    chooseFile: "Upload .svp",
    profile: "Source profile",
    source: "Source",
    bytes: "bytes",
    loadIndependentDemo: "Independent ES+EN demonstration",
    loadLinkedDemo: "Linked ES→EN demonstration",
    assemble: "Compile and assemble",
    clearAssembly: "Clear assembly",
    assemblyResult: "Assembly result",
    individualCheck: "Standalone check",
    assemblyCheck: "Joint validation",
    waiting: "Pending",
    admitted: "Admitted",
    rejected: "Not admitted",
    assemblyAdmitted: "Assembly admitted",
    assemblyRejected: "Assembly not admitted",
    sourceTooLarge: "The unit exceeds this interface's 1 MiB security limit.",
    invalidUtf8: "The file is not valid UTF-8.",
    fileLoaded: "File loaded",
    demoLoaded: "Demonstration loaded",
    wasmLoading: "Checking identity and loading WebAssembly…",
    wasmReady: "WebAssembly: identity, size and required exports verified.",
    wasmError: "WebAssembly could not be initialized",
    sourceHashMismatch: "The SHA-256 returned by compilation does not match the current editor bytes.",
    outputCleared: "Result cleared.",
    downloadReady: "Source downloaded without translation or transformation.",
    uiChanged: "Interface language changed; source and source profiles remain unchanged.",
    profileChanged: "Source profile changed; source remains unchanged.",
    loadedExample: "Example loaded",
    requiredProfile: "Active source profile",
    loadExample: "Load into editor",
    footerText: "The interface only transports bytes and presents results. Compilation, canonical assembly and global validation run locally in WebAssembly.",
    descriptionCell: "Minimal codomain, output-semantics and cell-specification declaration.",
    descriptionGate: "Gate with admissibility table and evaluation operation.",
    descriptionResolve: "Constituted resolution and access to operation fields.",
    descriptionCompose: "Composition of structures under the current grammar regime.",
    descriptionQuery: "Typed queries and contexts recognized by the Language.",
    descriptionTrajectory: "Trajectory and valid alternation from the conformance corpus."
  }
};

const state = {
  ui: "es",
  profile: "es",
  examples: [],
  wasm: null,
  build: null,
  hashSequence: 0,
  assembly: {
    a: { profile: "es", source: "", fileName: "unidad-a.svp" },
    b: { profile: "en", source: "", fileName: "unit-b.svp" }
  }
};

const el = {};
const t = key => I18N[state.ui][key] ?? I18N.es[key] ?? key;
const profileCode = p => p === "es" ? 1 : 0;
const profileShort = p => p === "es" ? "SVP-ES" : "SVP-EN";
const profileLong = p => p === "es"
  ? (state.ui === "es" ? "ESPAÑOL · SVP-ES" : "SPANISH · SVP-ES")
  : (state.ui === "es" ? "INGLÉS · SVP-EN" : "ENGLISH · SVP-EN");

function bytesToHex(bytes) {
  return [...bytes].map(b => b.toString(16).padStart(2, "0")).join("");
}

async function sha256Bytes(bytes) {
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return bytesToHex(new Uint8Array(digest));
}

async function sha256Text(text) {
  return sha256Bytes(encoder.encode(text));
}

function unpackResult(packed) {
  const value = BigInt(packed);
  return {
    error: (value & ERROR_FLAG) !== 0n,
    ptr: Number(value & PTR_MASK),
    len: Number((value >> 32n) & LEN_MASK)
  };
}

function readPacked(exports, packed) {
  const r = unpackResult(packed);
  const bytes = new Uint8Array(exports.memory.buffer, r.ptr, r.len).slice();
  return { error: r.error, raw: fatalDecoder.decode(bytes) };
}

function writeBuffer(exports, exportName, text) {
  const bytes = encoder.encode(text);
  if (bytes.length > MAX_SOURCE_BYTES && exportName.includes("source")) {
    throw new Error(t("sourceTooLarge"));
  }
  const ptr = exports[exportName](bytes.length);
  new Uint8Array(exports.memory.buffer, ptr, bytes.length).set(bytes);
}

function decodeBase64(text) {
  const compact = text.replace(/\s+/g, "");
  const binary = atob(compact);
  const out = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i += 1) out[i] = binary.charCodeAt(i);
  return out;
}

async function gunzip(bytes) {
  if (!("DecompressionStream" in globalThis)) {
    throw new Error("DecompressionStream no disponible en este navegador");
  }
  const stream = new Blob([bytes]).stream().pipeThrough(new DecompressionStream("gzip"));
  return new Uint8Array(await new Response(stream).arrayBuffer());
}

async function initializeWasm() {
  if (state.wasm) return state.wasm;
  setEditorStatus(t("wasmLoading"));
  if (!state.build) await loadBuildInfo();
  const response = await fetch("sv_wasm_b2.wasm.gz.b64", { cache: "no-store" });
  if (!response.ok) throw new Error(`HTTP ${response.status}`);
  const compressed = decodeBase64(await response.text());
  const wasmBytes = await gunzip(compressed);
  const hash = await sha256Bytes(wasmBytes);
  if (hash !== state.build.wasm_sha256) {
    throw new Error(`SHA-256 WebAssembly inesperado: ${hash}`);
  }
  if (wasmBytes.byteLength !== state.build.wasm_bytes) {
    throw new Error(`tamaño WebAssembly inesperado: ${wasmBytes.byteLength}`);
  }
  const { instance } = await WebAssembly.instantiate(wasmBytes, {});
  const exports = instance.exports;
  const required = [
    "memory", "sv_source_buffer", "sv_file_buffer", "sv_compile_svp_json_profile",
    "sv_assembly_source_b_buffer", "sv_assembly_file_b_buffer", "sv_compile_svp_assembly_json",
    "sv_grammar_version_major", "sv_grammar_version_minor", "sv_ir_version_major",
    "sv_ir_version_minor", "sv_serializer_version_major", "sv_serializer_version_minor",
    "sv_serializer_version_patch"
  ];
  for (const name of required) {
    if (!(name in exports)) throw new Error(`export WebAssembly ausente: ${name}`);
  }
  state.wasm = exports;
  if (el.buildGrammar) el.buildGrammar.textContent = `${exports.sv_grammar_version_major()}.${exports.sv_grammar_version_minor()}`;
  if (el.buildIr) el.buildIr.textContent = `${exports.sv_ir_version_major()}.${exports.sv_ir_version_minor()}`;
  if (el.buildProjection) el.buildProjection.textContent = `${exports.sv_serializer_version_major()}.${exports.sv_serializer_version_minor()}.${exports.sv_serializer_version_patch()}`;
  setEditorStatus(t("wasmReady"));
  return exports;
}

function setEditorStatus(message) {
  if (el.editorStatus) el.editorStatus.textContent = message;
}

function setStateBadge(node, kind, text) {
  node.dataset.state = kind;
  node.textContent = text;
}

function applyTranslations() {
  document.documentElement.lang = state.ui;
  document.body.dataset.uiLang = state.ui;
  for (const node of document.querySelectorAll("[data-i18n]")) node.textContent = t(node.dataset.i18n);
  for (const button of document.querySelectorAll("[data-ui-choice]")) {
    button.setAttribute("aria-pressed", String(button.dataset.uiChoice === state.ui));
  }
  el.profileLabel.textContent = profileLong(state.profile);
  el.editorProfileChip.textContent = profileShort(state.profile);
  el.examplesProfileChip.textContent = profileShort(state.profile);
  el.securityLink.href = state.ui === "es" ? "seguridad-b2.html" : "security-b2.html";
  el.diagnosticsLink.href = state.ui === "es" ? "diagnosticos.html" : "diagnostics.html";
  updateAssemblyLabels();
  renderExamples();
}

function setUiLanguage(lang) {
  if (!Object.hasOwn(I18N, lang) || lang === state.ui) return;
  const mainSource = el.editor.value;
  const a = el.assemblySourceA.value;
  const b = el.assemblySourceB.value;
  const profiles = [state.profile, state.assembly.a.profile, state.assembly.b.profile];
  state.ui = lang;
  applyTranslations();
  if (el.editor.value !== mainSource || el.assemblySourceA.value !== a || el.assemblySourceB.value !== b ||
      profiles[0] !== state.profile || profiles[1] !== state.assembly.a.profile || profiles[2] !== state.assembly.b.profile) {
    throw new Error("la interfaz no puede modificar fuentes ni perfiles");
  }
  setEditorStatus(t("uiChanged"));
}

function setMainProfile(profile) {
  if (!['es','en'].includes(profile) || profile === state.profile) return;
  const sourceBefore = el.editor.value;
  state.profile = profile;
  document.body.dataset.sourceProfile = profile;
  for (const button of document.querySelectorAll("[data-profile-choice]")) {
    button.setAttribute("aria-pressed", String(button.dataset.profileChoice === profile));
  }
  el.profileLabel.textContent = profileLong(profile);
  el.editorProfileChip.textContent = profileShort(profile);
  el.examplesProfileChip.textContent = profileShort(profile);
  el.sourceProfileCode.textContent = profile;
  renderExamples();
  if (el.editor.value !== sourceBefore) throw new Error("cambiar el perfil no puede modificar la fuente");
  setStateBadge(el.resultState, "idle", t("notExecuted"));
  setEditorStatus(t("profileChanged"));
}

function renderExamples() {
  if (!el.examplesList) return;
  el.examplesList.replaceChildren();
  for (const example of state.examples) {
    const card = document.createElement("article");
    card.className = "example-card";
    const h3 = document.createElement("h3");
    h3.textContent = state.ui === "es" ? example.title_es : example.title_en;
    const p = document.createElement("p");
    p.textContent = `${t(example.description_key)} · ${t("requiredProfile")}: ${profileShort(state.profile)}`;
    const button = document.createElement("button");
    button.type = "button";
    button.className = "secondary";
    button.textContent = t("loadExample");
    button.addEventListener("click", () => {
      el.editor.value = state.profile === "es" ? example.source_es : example.source_en;
      el.fileName.value = example.file_name;
      scheduleMainHash();
      setStateBadge(el.resultState, "idle", t("notExecuted"));
      setEditorStatus(`${t("loadedExample")}: ${state.ui === "es" ? example.title_es : example.title_en}.`);
    });
    card.append(h3, p, button);
    el.examplesList.append(card);
  }
}

function scheduleMainHash() {
  const seq = ++state.hashSequence;
  const source = el.editor.value;
  setTimeout(async () => {
    const hash = await sha256Text(source);
    if (seq === state.hashSequence) el.sourceSha.textContent = hash;
  }, 70);
}

async function compileText(source, fileName, profile) {
  const exports = await initializeWasm();
  const bytes = encoder.encode(source);
  if (bytes.length > MAX_SOURCE_BYTES) throw new Error(t("sourceTooLarge"));
  writeBuffer(exports, "sv_source_buffer", source);
  writeBuffer(exports, "sv_file_buffer", fileName);
  return readPacked(exports, exports.sv_compile_svp_json_profile(profileCode(profile)));
}

async function runSource() {
  setStateBadge(el.resultState, "running", t("run"));
  try {
    const source = el.editor.value;
    const fileName = el.fileName.value.trim() || "programa.svp";
    const result = await compileText(source, fileName, state.profile);
    let formatted = result.raw;
    let returnedHash = null;
    try {
      const parsed = JSON.parse(result.raw);
      returnedHash = parsed.source_sha256 ?? null;
      formatted = JSON.stringify(parsed, null, 2);
    } catch (_) {}
    el.output.textContent = formatted;
    const localHash = await sha256Text(source);
    el.sourceSha.textContent = localHash;
    if (returnedHash && returnedHash !== localHash) setEditorStatus(t("sourceHashMismatch"));
    else setEditorStatus(`${result.error ? t("rejected") : t("admitted")} · ${profileLong(state.profile)}.`);
    setStateBadge(el.resultState, result.error ? "fail" : "pass", result.error ? t("rejected") : t("admitted"));
  } catch (error) {
    el.output.textContent = String(error);
    setStateBadge(el.resultState, "fail", t("rejected"));
    setEditorStatus(`${t("wasmError")}: ${String(error)}`);
  }
}

function clearOutput() {
  el.output.textContent = "";
  setStateBadge(el.resultState, "idle", t("notExecuted"));
  setEditorStatus(t("outputCleared"));
}

function downloadSource() {
  const source = el.editor.value;
  const requested = el.fileName.value.trim() || "programa.svp";
  const fileName = requested.endsWith(".svp") ? requested : `${requested}.svp`;
  const blob = new Blob([source], { type: "text/plain;charset=utf-8" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = fileName;
  document.body.append(a);
  a.click();
  a.remove();
  URL.revokeObjectURL(url);
  setEditorStatus(t("downloadReady"));
}

function setAssemblyProfile(slot, profile) {
  if (!['es','en'].includes(profile)) return;
  state.assembly[slot].profile = profile;
  const card = slot === 'a' ? el.assemblyCardA : el.assemblyCardB;
  card.dataset.profile = profile;
  for (const button of card.querySelectorAll("[data-assembly-profile]")) {
    button.setAttribute("aria-pressed", String(button.dataset.assemblyProfile === profile));
  }
  updateAssemblyLabels();
}

function updateAssemblyLabels() {
  el.assemblyProfileA.textContent = profileShort(state.assembly.a.profile);
  el.assemblyProfileB.textContent = profileShort(state.assembly.b.profile);
  el.assemblyPair.textContent = `${profileShort(state.assembly.a.profile)} + ${profileShort(state.assembly.b.profile)} → IR 0.3`;
}

async function readUploadedFile(file) {
  if (file.size > MAX_SOURCE_BYTES) throw new Error(t("sourceTooLarge"));
  const bytes = new Uint8Array(await file.arrayBuffer());
  let source;
  try { source = fatalDecoder.decode(bytes); }
  catch (_) { throw new Error(t("invalidUtf8")); }
  return { source, hash: await sha256Bytes(bytes) };
}

async function loadAssemblyFile(slot, file) {
  if (!file) return;
  const data = await readUploadedFile(file);
  const sourceEl = slot === 'a' ? el.assemblySourceA : el.assemblySourceB;
  const nameEl = slot === 'a' ? el.assemblyNameA : el.assemblyNameB;
  const hashEl = slot === 'a' ? el.assemblyHashA : el.assemblyHashB;
  sourceEl.value = data.source;
  nameEl.value = file.name || (slot === 'a' ? 'unidad-a.svp' : 'unidad-b.svp');
  hashEl.textContent = data.hash;
  state.assembly[slot].source = data.source;
  state.assembly[slot].fileName = nameEl.value;
  setAssemblyStatus(`${t("fileLoaded")}: ${nameEl.value}`);
}

const DEMO = {
  aEs: `codominio K3 = { APTO, NO_APTO, INDETERMINADO };\nsemántica_de_salida Klin {\n  APTO -> "favorable";\n  NO_APTO -> "desfavorable";\n  INDETERMINADO -> "sin cierre";\n}\n`,
  aEn: `codomain K3B = { APTO, NO_APTO, INDETERMINADO };\noutput_semantics KlinB {\n  APTO -> "favorable";\n  NO_APTO -> "desfavorable";\n  INDETERMINADO -> "sin cierre";\n}\ncellspec C2 { b: 3; codomain: K3B; semantics: KlinB; role: Base; }\ncellstate S2 { spec: C2; vector: [Zero, One, U, Zero, Zero, One, U, Zero,One]; }\nlet E2 = evaluate(S2);\n`,
  bEnLinked: `cellspec C1 {\n  b: 3;\n  codomain: K3;\n  semantics: Klin;\n  role: Base;\n}\ncellstate S1 {\n  spec: C1;\n  vector: [Zero, One, U, Zero, Zero, One, U, Zero, One];\n}\nlet E1 = evaluate(S1);\n`,
  bEnIndependent: `codomain K4 = { SI, NO, DESCONOCIDO };\noutput_semantics Kout {\n  SI -> "sí";\n  NO -> "no";\n  DESCONOCIDO -> "sin cierre";\n}\ncellspec C4 { b: 3; codomain: K4; semantics: Kout; role: Base; }\ncellstate S4 { spec: C4; vector: [Zero, One, U, Zero, One, U, Zero, One, U]; }\nlet E4 = evaluate(S4);\n`
};

async function loadDemo(kind) {
  setAssemblyProfile('a', 'es');
  setAssemblyProfile('b', 'en');
  el.assemblySourceA.value = DEMO.aEs;
  el.assemblySourceB.value = kind === 'linked' ? DEMO.bEnLinked : DEMO.bEnIndependent;
  el.assemblyNameA.value = "base_es.svp";
  el.assemblyNameB.value = kind === 'linked' ? "consumidor_en.svp" : "unidad_en.svp";
  await refreshAssemblyHashes();
  setAssemblyStatus(`${t("demoLoaded")}: ${kind === 'linked' ? 'ES→EN' : 'ES+EN'}.`);
}

async function refreshAssemblyHashes() {
  const a = el.assemblySourceA.value;
  const b = el.assemblySourceB.value;
  el.assemblyHashA.textContent = await sha256Text(a);
  el.assemblyHashB.textContent = await sha256Text(b);
}

function setAssemblyStatus(message) {
  el.assemblyStatus.textContent = message;
}

async function runStandalone(slot) {
  const source = slot === 'a' ? el.assemblySourceA.value : el.assemblySourceB.value;
  const name = (slot === 'a' ? el.assemblyNameA.value : el.assemblyNameB.value).trim() || `${slot}.svp`;
  const profile = state.assembly[slot].profile;
  const result = await compileText(source, name, profile);
  const badge = slot === 'a' ? el.assemblyStandaloneA : el.assemblyStandaloneB;
  setStateBadge(badge, result.error ? "fail" : "pass", result.error ? t("rejected") : t("admitted"));
  return result;
}

async function runAssembly() {
  setStateBadge(el.assemblyJoint, "running", t("assemblyCheck"));
  el.assemblyOutput.textContent = "";
  try {
    const exports = await initializeWasm();
    const aSource = el.assemblySourceA.value;
    const bSource = el.assemblySourceB.value;
    const aName = el.assemblyNameA.value.trim() || "unidad-a.svp";
    const bName = el.assemblyNameB.value.trim() || "unidad-b.svp";
    if (encoder.encode(aSource).length > MAX_SOURCE_BYTES || encoder.encode(bSource).length > MAX_SOURCE_BYTES) {
      throw new Error(t("sourceTooLarge"));
    }

    await Promise.allSettled([runStandalone('a'), runStandalone('b')]);

    writeBuffer(exports, "sv_source_buffer", aSource);
    writeBuffer(exports, "sv_file_buffer", aName);
    writeBuffer(exports, "sv_assembly_source_b_buffer", bSource);
    writeBuffer(exports, "sv_assembly_file_b_buffer", bName);
    const combined = readPacked(exports, exports.sv_compile_svp_assembly_json(
      profileCode(state.assembly.a.profile), profileCode(state.assembly.b.profile)
    ));

    const aHash = await sha256Text(aSource);
    const bHash = await sha256Text(bSource);
    el.assemblyHashA.textContent = aHash;
    el.assemblyHashB.textContent = bHash;

    let payload = combined.raw;
    try {
      const parsed = JSON.parse(combined.raw);
      payload = JSON.stringify({
        beta: "B2",
        units: [
          { file: aName, profile: state.assembly.a.profile, source_sha256: aHash },
          { file: bName, profile: state.assembly.b.profile, source_sha256: bHash }
        ],
        assembly: parsed
      }, null, 2);
    } catch (_) {}
    el.assemblyOutput.textContent = payload;
    setStateBadge(el.assemblyJoint, combined.error ? "fail" : "pass", combined.error ? t("assemblyRejected") : t("assemblyAdmitted"));
    setAssemblyStatus(`${combined.error ? t("assemblyRejected") : t("assemblyAdmitted")} · ${profileShort(state.assembly.a.profile)} + ${profileShort(state.assembly.b.profile)}.`);
  } catch (error) {
    el.assemblyOutput.textContent = String(error);
    setStateBadge(el.assemblyJoint, "fail", t("assemblyRejected"));
    setAssemblyStatus(String(error));
  }
}

function clearAssembly() {
  for (const node of [el.assemblySourceA, el.assemblySourceB]) node.value = "";
  for (const node of [el.assemblyHashA, el.assemblyHashB]) node.textContent = "—";
  el.assemblyOutput.textContent = "";
  setStateBadge(el.assemblyStandaloneA, "idle", t("waiting"));
  setStateBadge(el.assemblyStandaloneB, "idle", t("waiting"));
  setStateBadge(el.assemblyJoint, "idle", t("waiting"));
  setAssemblyStatus("");
}

async function loadBuildInfo() {
  const response = await fetch("build-info.json", { cache: "no-store" });
  if (!response.ok) throw new Error(`build-info HTTP ${response.status}`);
  state.build = await response.json();
  if (el.buildSourceCommit) el.buildSourceCommit.textContent = state.build.stable_realization_commit ?? state.build.source_commit ?? "—";
  if (el.buildWasmSha) el.buildWasmSha.textContent = state.build.wasm_sha256 ?? "—";
  if (el.buildWasmBytes) el.buildWasmBytes.textContent = state.build.wasm_bytes != null ? `${state.build.wasm_bytes} bytes` : "—";
  if (el.buildBeta) el.buildBeta.textContent = state.build.beta ?? "B2";
}

async function loadExamples() {
  const response = await fetch("examples.json", { cache: "no-store" });
  if (!response.ok) throw new Error(`examples HTTP ${response.status}`);
  state.examples = await response.json();
  renderExamples();
  if (state.examples.length && !el.editor.value) {
    const first = state.examples[0];
    el.editor.value = first.source_es;
    el.fileName.value = first.file_name;
    scheduleMainHash();
  }
}

function bind() {
  Object.assign(el, {
    editor: document.getElementById("source-editor"),
    fileName: document.getElementById("file-name"),
    sourceSha: document.getElementById("source-sha"),
    sourceProfileCode: document.getElementById("source-profile-code"),
    editorStatus: document.getElementById("editor-status"),
    profileLabel: document.getElementById("profile-label"),
    editorProfileChip: document.getElementById("editor-profile-chip"),
    examplesProfileChip: document.getElementById("examples-profile-chip"),
    examplesList: document.getElementById("examples-list"),
    resultState: document.getElementById("result-state"),
    output: document.getElementById("result-output"),
    buildSourceCommit: document.getElementById("build-source-commit"),
    buildWasmSha: document.getElementById("build-wasm-sha"),
    buildWasmBytes: document.getElementById("build-wasm-bytes"),
    buildGrammar: document.getElementById("build-grammar"),
    buildIr: document.getElementById("build-ir"),
    buildProjection: document.getElementById("build-projection"),
    buildBeta: document.getElementById("build-beta"),
    securityLink: document.getElementById("security-link"),
    diagnosticsLink: document.getElementById("diagnostics-link"),
    assemblyCardA: document.getElementById("assembly-card-a"),
    assemblyCardB: document.getElementById("assembly-card-b"),
    assemblySourceA: document.getElementById("assembly-source-a"),
    assemblySourceB: document.getElementById("assembly-source-b"),
    assemblyNameA: document.getElementById("assembly-name-a"),
    assemblyNameB: document.getElementById("assembly-name-b"),
    assemblyHashA: document.getElementById("assembly-hash-a"),
    assemblyHashB: document.getElementById("assembly-hash-b"),
    assemblyProfileA: document.getElementById("assembly-profile-a"),
    assemblyProfileB: document.getElementById("assembly-profile-b"),
    assemblyPair: document.getElementById("assembly-pair"),
    assemblyStandaloneA: document.getElementById("assembly-standalone-a"),
    assemblyStandaloneB: document.getElementById("assembly-standalone-b"),
    assemblyJoint: document.getElementById("assembly-joint"),
    assemblyOutput: document.getElementById("assembly-output"),
    assemblyStatus: document.getElementById("assembly-status")
  });

  for (const button of document.querySelectorAll("[data-ui-choice]")) button.addEventListener("click", () => setUiLanguage(button.dataset.uiChoice));
  for (const button of document.querySelectorAll("[data-profile-choice]")) button.addEventListener("click", () => setMainProfile(button.dataset.profileChoice));
  for (const card of [el.assemblyCardA, el.assemblyCardB]) {
    const slot = card.dataset.slot;
    for (const button of card.querySelectorAll("[data-assembly-profile]")) button.addEventListener("click", () => setAssemblyProfile(slot, button.dataset.assemblyProfile));
  }
  document.getElementById("run-source").addEventListener("click", runSource);
  document.getElementById("clear-output").addEventListener("click", clearOutput);
  document.getElementById("download-source").addEventListener("click", downloadSource);
  document.getElementById("assembly-file-a").addEventListener("change", event => loadAssemblyFile('a', event.target.files?.[0]).catch(error => setAssemblyStatus(String(error))));
  document.getElementById("assembly-file-b").addEventListener("change", event => loadAssemblyFile('b', event.target.files?.[0]).catch(error => setAssemblyStatus(String(error))));
  document.getElementById("load-demo-independent").addEventListener("click", () => loadDemo('independent'));
  document.getElementById("load-demo-linked").addEventListener("click", () => loadDemo('linked'));
  document.getElementById("run-assembly").addEventListener("click", runAssembly);
  document.getElementById("clear-assembly").addEventListener("click", clearAssembly);
  el.editor.addEventListener("input", scheduleMainHash);
  el.assemblySourceA.addEventListener("input", () => { state.assembly.a.source = el.assemblySourceA.value; refreshAssemblyHashes(); });
  el.assemblySourceB.addEventListener("input", () => { state.assembly.b.source = el.assemblySourceB.value; refreshAssemblyHashes(); });
}

async function main() {
  bind();
  applyTranslations();
  await Promise.all([loadBuildInfo(), loadExamples()]);
  await initializeWasm();
  await loadDemo('independent');
  scheduleMainHash();
}

main().catch(error => {
  if (el.output) el.output.textContent = String(error);
  if (el.resultState) setStateBadge(el.resultState, "fail", t("rejected"));
  if (el.editorStatus) setEditorStatus(`${t("wasmError")}: ${String(error)}`);
});
