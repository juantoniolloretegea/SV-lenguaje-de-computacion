import fs from 'node:fs';
import { WASI } from 'node:wasi';
const wasi = new WASI({version:'preview1', args:['receptor-ie004'], env:{}, preopens:{}, returnOnExit:true});
const modulo = await WebAssembly.compile(fs.readFileSync(process.argv[2]));
const instancia = await WebAssembly.instantiate(modulo, {wasi_snapshot_preview1:wasi.wasiImport});
process.exitCode = wasi.start(instancia);
