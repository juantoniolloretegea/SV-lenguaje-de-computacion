import fs from 'node:fs';import path from 'node:path';import crypto from 'node:crypto';
const dir=path.dirname(new URL(import.meta.url).pathname),hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const files=[];function walk(root){for(const e of fs.readdirSync(path.join(dir,root),{withFileTypes:true})){const f=path.posix.join(root,e.name);if(e.isDirectory()){if(e.name==='publicos')walk(f);else if(root.startsWith('publicos'))walk(f);}else if(/\.(rs|md|json|mjs)$/.test(e.name)&&e.name!=='FIJACION_PREVIA.json'&&!e.name.startsWith('publicacion-'))files.push(f);}}
walk('');
const stat=ruta=>{const b=fs.readFileSync(path.join(dir,ruta));return{ruta,bytes:b.length,sha256:hash(b)}};
const inherited=['gramatica.rs','recursos.rs','sintaxis.rs','semantica.rs','servicio.rs'].map(f=>'../semantica-a/'+f);
const j={registro:'RETP-2026-136',instrumento:'IE004-RECEPCION-AV/1',perfil:'IE004-ES-P2/3-COSTE/1',fecha:new Date().toISOString(),ejecuciones_de_controles:0,extracciones_tabla_constante:1,matriz:['nativo-debug','nativo-release','wasi-debug','wasi-release'],rust:'1.98.0',archivos:files.sort().map(stat),fuentes_heredadas:inherited.map(stat),reserva_leida:false,python_ejecutado:false,parada:'Una matriz fijada; conservar todo fallo; un ciclo causal posterior como máximo y siempre con nueva fijación previa.'};
fs.writeFileSync(path.join(dir,'FIJACION_PREVIA.json'),JSON.stringify(j,null,2)+'\n');console.log(JSON.stringify({files:j.archivos.length,heredadas:j.fuentes_heredadas.length,sha256:hash(fs.readFileSync(path.join(dir,'FIJACION_PREVIA.json')))}));
