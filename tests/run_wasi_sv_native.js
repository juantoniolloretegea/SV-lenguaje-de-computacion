#!/usr/bin/env node
"use strict";

// Host mínimo para ejecutar el mismo `sv-native` compilado a wasm32-wasip1.
// No interpreta SVP ni contiene semántica del Lenguaje: sólo proporciona
// argumentos, un directorio preabierto y las importaciones WASI preview1.

const fs = require("node:fs");
const path = require("node:path");
const { WASI } = require("node:wasi");

async function main() {
  if (process.argv.length !== 4) {
    console.error("uso: node tests/run_wasi_sv_native.js <sv-native.wasm> <archivo.svp>");
    process.exitCode = 2;
    return;
  }

  const wasmPath = path.resolve(process.argv[2]);
  const sourcePath = path.resolve(process.argv[3]);
  const hostDir = path.dirname(sourcePath);
  const guestDir = "/sv-input";
  const guestPath = `${guestDir}/${path.basename(sourcePath)}`;

  const wasi = new WASI({
    version: "preview1",
    args: ["sv-native", guestPath],
    env: {},
    preopens: {
      [guestDir]: hostDir,
    },
    returnOnExit: true,
  });

  const bytes = fs.readFileSync(wasmPath);
  const module = await WebAssembly.compile(bytes);
  const instance = await WebAssembly.instantiate(module, {
    wasi_snapshot_preview1: wasi.wasiImport,
  });

  const code = wasi.start(instance);
  if (Number.isInteger(code)) {
    process.exitCode = code;
  }
}

main().catch((error) => {
  console.error(`fallo del host WASI: ${error && error.stack ? error.stack : error}`);
  process.exitCode = 2;
});
