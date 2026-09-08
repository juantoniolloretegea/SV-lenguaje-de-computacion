import fs from 'node:fs';
import {WASI} from 'node:wasi';
const wasi = new WASI({version:'preview1', args:[], env:{}, preopens:{}, returnOnExit:true});
const module = await WebAssembly.compile(fs.readFileSync(process.argv[2]));
const instance = await WebAssembly.instantiate(module, {wasi_snapshot_preview1:wasi.wasiImport});
process.exitCode = wasi.start(instance);
