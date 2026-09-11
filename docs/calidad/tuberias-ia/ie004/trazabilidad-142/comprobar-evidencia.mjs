import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import zlib from 'node:zlib';
import {spawnSync} from 'node:child_process';
import assert from 'node:assert/strict';

// Custodia y conducción de Rust existente. No decide semántica SV.
// node comprobar-evidencia.mjs CORTE_LOCAL RUSTC DIRECTORIO_NUEVO
const [sourceArg,rustArg,outArg]=process.argv.slice(2);
if(!sourceArg||!rustArg||!outArg)throw Error('Faltan corte local, rustc y salida nueva');
const root=path.resolve(sourceArg),rust=path.resolve(rustArg),out=path.resolve(outArg);
if(fs.existsSync(out))throw Error('Salida existente: no sobrescribir ni repetir implícitamente');
fs.mkdirSync(out,{recursive:true});
const dir=path.dirname(new URL(import.meta.url).pathname);
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const gitblob=b=>crypto.createHash('sha1').update(Buffer.from('blob '+b.length+'\0')).update(b).digest('hex');
const read=p=>fs.readFileSync(path.join(root,p));
const put=(p,b)=>fs.writeFileSync(path.join(out,p),Buffer.isBuffer(b)||typeof b==='string'?b:JSON.stringify(b,null,2)+'\n');
const report={registro:'RETP-2026-142',tipo:'CONTRASTE_ACOTADO',casos:[],procesos:[],reserva_leida:false,modelo_invocado:false,tuberia_integral_acreditada:false};
function check(id,fn){try{const detail=fn();report.casos.push({id,conforme:true,...detail});}catch(e){report.casos.push({id,conforme:false,error:String(e)});throw e;}}
function run(id,cmd,args,input=Buffer.alloc(0),timeout=10000){
  const result=spawnSync(cmd,args,{input,timeout,maxBuffer:32*1024*1024});
  const stdout=result.stdout??Buffer.alloc(0),stderr=result.stderr??Buffer.alloc(0);
  put(id+'.stdin',input);put(id+'.stdout',stdout);put(id+'.stderr',stderr);
  const rec={id,cmd,args,timeout_ms:timeout,exit_code:result.status,signal:result.signal,error:result.error?String(result.error):null,stdin_sha256:hash(input),stdout_sha256:hash(stdout),stderr_sha256:hash(stderr)};
  put(id+'.json',rec);report.procesos.push(rec);
  return {...rec,stdout,stderr};
}
try{
  const plan=fs.readFileSync(path.join(dir,'PLAN_CONTRASTE.json'));put('PLAN_FIJADO.json',plan);report.plan_sha256=hash(plan);
  const manifest=JSON.parse(fs.readFileSync(path.join(dir,'FUENTES_CORTE.json')));report.corte=manifest.commit;
  for(const f of manifest.archivos){const b=read(f.path);assert.equal(b.length,f.bytes);assert.equal(gitblob(b),f.blob);}
  report.fuentes_cotejadas=manifest.archivos.length;
  const prefix='docs/calidad/tuberias-ia/ie004/recepcion-av/';
  const metadata=JSON.parse(read(prefix+'ARCHIVO_CAPTURAS.json'));
  const stored=read(prefix+'CAPTURAS_RECUPERABLES.json.gz.b64');
  const compressed=Buffer.from(stored.toString(),'base64');const raw=zlib.gunzipSync(compressed);
  const capture=JSON.parse(raw),files=new Map();
  check('C01',()=>{
    for(const [name,b]of[['base64',stored],['gzip',compressed],['contenido',raw]]){assert.equal(b.length,metadata[name+'_bytes']);assert.equal(hash(b),metadata[name+'_sha256']);}
    assert.equal(capture.archivos.length,409);assert.equal(capture.archivos.length,metadata.numero_archivos);
    for(const f of capture.archivos){assert(!files.has(f.ruta));const b=Buffer.from(f.base64,'base64');assert.equal(b.length,f.bytes);assert.equal(hash(b),f.sha256);files.set(f.ruta,b);}
    return {archivos:files.size,capsula_sha256:hash(stored)};
  });
  const get=n=>{assert(files.has(n),n);return files.get(n);};const json=n=>JSON.parse(get(n));
  const old=json('RESULTADO.json');
  check('C02',()=>{
    let n=0;const states=[];
    for(const r of old.resultados)for(const lane of r.transporte){
      const a=get(r.id+'-'+lane.scenario+'-a.stdout'),t=get(r.id+'-'+lane.scenario+'-a.stderr');
      const b=get(lane.scenario+'-'+r.id+'-cuerpo.json');
      assert.equal(a.subarray(0,8).toString(),'SVAC0001');assert.equal(Number(a.readBigUInt64LE(8)),b.length);
      assert(a.subarray(16,a.length-32).equals(b));assert.equal(a.subarray(-32).toString('hex'),hash(b));
      // Corrección del observador: reconocer sólo el prefijo exacto de Node
      // presente en esta cápsula; conservar stderr completo y su huella.
      const text=t.toString();let prefijo='';
      if(r.id.startsWith('wasi-')){
        const lines=text.split('\n');
        assert.match(lines[0],/^\(node:\d+\) ExperimentalWarning: WASI is an experimental feature and might change at any time$/);
        assert.equal(lines[1],'(Use `node --trace-warnings ...` to show where the warning was created)');
        prefijo=lines[0]+'\n'+lines[1]+'\n';
      }
      const trace=JSON.parse(text.slice(prefijo.length));assert.equal(trace.cuerpo_sha256,hash(b));assert.equal(hash(b),lane.cuerpo_sha256);assert.equal(hash(t),lane.traza_a_sha256);
      const delivered=lane.events.findIndex(e=>e.nombre==='CUERPO_A_ENTREGADO'),opened=lane.events.findIndex(e=>e.nombre==='V_ABIERTA');assert(delivered>=0&&opened>delivered);
      const vrec=json(r.id+'-'+lane.scenario+'-v.json');
      if(lane.scenario==='V_SIN_EOF'){assert.equal(vrec.timeout,true);assert.equal(vrec.signal,'SIGKILL');assert.equal(lane.v.estado,'V_PLAZO_AGOTADO_POR_CONDUCTOR');}
      else{const annex=json(r.id+'-'+lane.scenario+'-v.stdout');assert.equal(annex.cuerpo_a_sha256,hash(b));assert.equal(annex.estado,lane.v.estado);}
      states.push({configuracion:r.id,escenario:lane.scenario,cuerpo_sha256:hash(b),stderr_completo_sha256:hash(t),prefijo_node:prefijo,traza_json_sha256:hash(Buffer.from(text.slice(prefijo.length))),estado_v:lane.v.estado});n++;
    }
    assert.equal(n,36);put('ENLACES_HISTORICOS.json',states);return {recorridos_historicos_cotejados:n,nuevas_ejecuciones_de_esos_recorridos:0};
  });
  const ver=run('rustc-version',rust,['-vV']);assert.equal(ver.exit_code,0);assert.match(ver.stdout.toString(),/^rustc 1\.98\.0 /);
  const bin=path.join(out,'receptor.bin');const compilation=run('compilar-receptor',rust,['--edition=2021','-C','opt-level=3','-C','overflow-checks=no','-C','panic=abort',path.join(root,prefix+'main.rs'),'-o',bin],undefined,60000);assert.equal(compilation.exit_code,0);
  report.binario_reconstruido_sha256=hash(fs.readFileSync(bin));report.identidad_binaria_con_historico_exigida=false;
  const wire=read(prefix+'publicos/a/A01.json');
  let ar,body;
  check('C03',()=>{ar=run('A01-a',bin,['a'],wire);assert.equal(ar.exit_code,0);assert(ar.stdout.equals(get('nativo-release-V02-a.stdout')));assert(ar.stderr.equals(get('nativo-release-V02-a.stderr')));assert.equal(JSON.parse(ar.stderr).original_transporte_sha256,hash(wire));return {marco_y_traza_identicos:true};});
  check('C04',()=>{const r=run('A01-validar',bin,['validar'],ar.stdout);assert.equal(r.exit_code,0);assert(r.stdout.equals(get('V02-nativo-release-cuerpo.json')));body=r.stdout;return {cuerpo_sha256:hash(body)};});
  check('C05',()=>{const r=run('marco-truncado',bin,['validar'],ar.stdout.subarray(0,-1));assert.equal(r.exit_code,2);assert.equal(r.stdout.length,0);assert.match(r.stderr.toString(),/SV_AV_ERROR:TRUNCADO/);return {rechazo:'TRUNCADO',sin_cuerpo:true};});
  check('C06',()=>{const b=Buffer.from(ar.stdout);const i=b.indexOf(Buffer.from('8.40'));assert(i>=16&&i<b.length-32);b[i]=57;const r=run('marco-alterado',bin,['validar'],b);assert.equal(r.exit_code,2);assert.equal(r.stdout.length,0);assert.match(r.stderr.toString(),/SV_AV_ERROR:CORRELACION_INVALIDA/);return {rechazo:'CORRELACION_INVALIDA',sin_cuerpo:true};});
  const spec=JSON.parse(read(prefix+'CONTROLES_PUBLICOS.json'));
  for(const [id,item,expected]of[['C07','V02','DERIVACION_SINTACTICA_COMPROBADA'],['C08','V04','V_RECHAZADA']])check(id,()=>{const r=run(item+'-v',bin,['v',wire.toString(),spec.lote_sha256,hash(body),'1000000'],read(prefix+'publicos/v/'+item+'.json'));assert.equal(r.exit_code,0);const annex=JSON.parse(r.stdout);assert.equal(annex.estado,expected);if(item==='V04')assert.equal(annex.causa,spec.casos_v.find(x=>x.id==='V04').causa);assert(r.stdout.equals(get('nativo-release-'+item+'-v.stdout')));return {estado:annex.estado,causa:annex.causa,anexo_identico:true};});
  const src='#![allow(dead_code)]\n'+['nat','frame','frame_tests'].map(n=>'#[path = '+JSON.stringify(path.join(root,'rust/sv_core/src/'+n+'.rs'))+'] mod '+n+';').join('\n')+'\npub use nat::Nat;\n';
  const harness=path.join(out,'arnes_frame.rs'),tests=path.join(out,'frame-tests.bin');put('arnes_frame.rs',src);
  const compiled=run('compilar-frame',rust,['--edition=2021','--test',harness,'-o',tests],undefined,60000);assert.equal(compiled.exit_code,0);
  check('C09',()=>{const r=run('frame-existentes',tests,['frame_tests::','--test-threads=1']);assert.equal(r.exit_code,0);assert.match(r.stdout.toString(),/16 passed; 0 failed/);return {pruebas_frame_existentes:16,alcance:'Constructor y guardas de Frame; sin transición ni integración IE-004'};});
  report.estado='CONTRASTE_CONFORME_EN_SU_ALCANCE';
}catch(e){report.estado='FALLO';report.error=String(e);process.exitCode=1;}finally{
  put('RESULTADO.json',report);
  const artifacts=fs.readdirSync(out).filter(n=>!n.endsWith('.bin')).map(ruta=>{const b=fs.readFileSync(path.join(out,ruta));return {ruta,bytes:b.length,sha256:hash(b),base64:b.toString('base64')};});
  fs.writeFileSync(path.join(dir,'EVIDENCIA_RECUPERABLE.json'),JSON.stringify({version:'IE004-TRAZABILIDAD-CONTRASTE/1',archivos:artifacts},null,2)+'\n');
  fs.writeFileSync(path.join(dir,'RESULTADO_CONTRASTE.json'),JSON.stringify(report,null,2)+'\n');
  console.log(JSON.stringify({estado:report.estado,casos:report.casos,error:report.error}));
}
