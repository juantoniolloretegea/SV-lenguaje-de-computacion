import fs from 'node:fs';import path from 'node:path';import crypto from 'node:crypto';import zlib from 'node:zlib';import assert from 'node:assert/strict';
const dir=path.dirname(new URL(import.meta.url).pathname),dest=process.argv[2];if(!dest)throw Error('Indicar un directorio nuevo de extracción');if(fs.existsSync(dest))throw Error('No sobrescribir una extracción existente');
const hash=b=>crypto.createHash('sha256').update(b).digest('hex'),m=JSON.parse(fs.readFileSync(path.join(dir,'ARCHIVO_CAPTURAS.json')));
const b64=fs.readFileSync(path.join(dir,m.archivo));assert.equal(hash(b64),m.base64_sha256);assert.equal(b64.length,m.base64_bytes);
const gz=Buffer.from(b64.toString().trim(),'base64');assert.equal(hash(gz),m.gzip_sha256);assert.equal(gz.length,m.gzip_bytes);
const raw=zlib.gunzipSync(gz,{maxOutputLength:64*1024*1024});assert.equal(hash(raw),m.contenido_sha256);assert.equal(raw.length,m.contenido_bytes);
const data=JSON.parse(raw);assert.equal(data.version,'IE004-AV-CAPTURAS/1');assert.equal(data.archivos.length,m.numero_archivos);
const names=new Set();for(const f of data.archivos){assert(/^[A-Za-z0-9_.-]+$/.test(f.ruta)&&f.ruta!=='.'&&f.ruta!=='..'&&!names.has(f.ruta));names.add(f.ruta);const b=Buffer.from(f.base64,'base64');assert.equal(b.length,f.bytes);assert.equal(hash(b),f.sha256);}
fs.mkdirSync(dest,{recursive:true});for(const f of data.archivos)fs.writeFileSync(path.join(dest,f.ruta),Buffer.from(f.base64,'base64'),{flag:'wx'});
console.log(JSON.stringify({archivos_recuperados:data.archivos.length,bytes:raw.length,ejecuciones_del_receptor:0}));
