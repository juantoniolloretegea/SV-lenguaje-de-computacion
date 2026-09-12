#!/usr/bin/env python3
"""Cualificación nativa acotada de S2 desde fuentes y esperados fijados."""
from pathlib import Path
import argparse,base64,hashlib,json,os,resource,shutil,subprocess,time
p=argparse.ArgumentParser();p.add_argument('--rustc',default='rustc');p.add_argument('--salida',required=True);args=p.parse_args()
root=Path(__file__).resolve().parent;dest=Path(args.salida).resolve()
if dest.exists():raise SystemExit('La carpeta de salida debe ser nueva.')
dest.mkdir(parents=True);src=dest/'fuentes-base';src.mkdir();sha=lambda b:hashlib.sha256(b).hexdigest()
def js(p,x):p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
freeze=json.loads((root/'FIJACION_PREVIA.json').read_text())
for name,h in freeze['archivos'].items():assert sha((root/name).read_bytes())==h,name
bank=json.loads((root/'BANCO_FIJADO.json').read_text());base=json.loads((root/'BASE_RETP152.json').read_text())
assert sha((root/'BASE_RETP152.json').read_bytes())==bank['capsula_base_sha256']
for item in base['archivos']:
 target=src/item['ruta'];assert target.resolve().is_relative_to(src)
 data=base64.b64decode(item['base64']);assert sha(data)==item['sha256'] and len(data)==item['bytes'];target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(data)
for item in bank['cambios_sucesor']:
 target=src/item['ruta'];assert sha(target.read_bytes())==item['anterior_sha256'];data=(root/'sucesores'/item['ruta']).read_bytes();assert sha(data)==item['sucesor_sha256'];target.write_bytes(data)
for folder in ['codigo','sensibilidad']:shutil.copytree(root/folder,dest/folder)
(dest/'esperados').mkdir()
for name,item in json.loads((root/'ESPERADOS.json').read_text()).items():
 b=base64.b64decode(item['base64']);assert sha(b)==bank['esperados'][name];(dest/'esperados'/name).write_bytes(b)
shutil.copy2(root/'MONTAJE_ANTERIOR.json',dest/'MONTAJE_ANTERIOR.json')
expected=(root/'codigo/ESPERADO.tsv').read_text();logs=[];captures={};start=time.time()
def run(label,cmd,code=0,env=None,contains=None):
 assert len(logs)<23,'Presupuesto agotado';before=resource.getrusage(resource.RUSAGE_CHILDREN);t=time.monotonic();record=dict(paso=label,argv=list(map(str,cmd)),esperado_exit=code)
 if env:record['entorno_fijado']={k:env[k] for k in ['LG1_FUENTES','LG1_BINARIO','S2_CAPTURAS']}
 try:
  result=subprocess.run(record['argv'],capture_output=True,timeout=60,env=env);record.update(exit_code=result.returncode,stdout=result.stdout.decode(errors='replace'),stderr=result.stderr.decode(errors='replace'))
 except Exception as e:
  record.update(error=repr(e));logs.append(record);js(dest/'COMANDOS.json',logs);raise
 after=resource.getrusage(resource.RUSAGE_CHILDREN);record.update(pared_s=time.monotonic()-t,cpu_usuario_s=after.ru_utime-before.ru_utime,cpu_sistema_s=after.ru_stime-before.ru_stime,rss_proceso=None)
 logs.append(record);js(dest/'COMANDOS.json',logs)
 assert result.returncode==code,record
 if contains:assert contains in record['stderr'],record
 print(label,'correcto',flush=True);return result.stdout
version=run('compilador',[args.rustc,'-vV']);assert b'release: 1.98.0\n' in version
js(dest/'ENTORNO.json',dict(rustc_sha256=sha(Path(args.rustc).read_bytes()) if Path(args.rustc).is_file() else None,version=version.decode(),plataforma=os.uname().sysname,maquina=os.uname().machine))
def environment(exe,d):
 d.mkdir();return os.environ|{'LG1_FUENTES':sha((root/'FIJACION_PREVIA.json').read_bytes()),'LG1_BINARIO':sha(exe.read_bytes()),'S2_CAPTURAS':str(d)}
def build(d,base,coverage):
 d.mkdir();flags=['--edition=2021','-C','opt-level='+('3' if d.name=='release' else '0'),'-C','overflow-checks='+('no' if d.name=='release' else 'yes')]
 g=d/'libg1.rlib';l=d/'liblote.rlib';c=d/'libcobertura.rlib';deps=['--extern',f'g1={g}','--extern',f'lote={l}','-L',f'dependency={d}']
 run(d.name+' g1',[args.rustc,*flags,'--crate-type=lib','--crate-name=g1',base/'recibo-g1/candidata/lib.rs','-o',g])
 run(d.name+' lote',[args.rustc,*flags,'--crate-type=lib','--crate-name=lote',base/'lote-g1/lib.rs','--extern',f'g1={g}','-o',l])
 run(d.name+' cobertura',[args.rustc,*flags,'--crate-type=lib','--crate-name=cobertura',coverage,*deps,'-o',c])
 exe=d/'contraste';run(d.name+' contraste',[args.rustc,*flags,dest/'codigo/contraste.rs',*deps,'--extern',f'cobertura={c}','-o',exe]);return exe,flags,deps
for mode in ['debug','release']:
 d=dest/mode;exe,flags,deps=build(d,src,dest/'codigo/cobertura.rs')
 for n in range(1,4):
  cap=d/('capturas-'+str(n));out=run(mode+' ejecución '+str(n),[exe],env=environment(exe,cap));assert out.decode()==expected
  observed={p.name:sha(p.read_bytes()) for p in cap.iterdir()}
  if captures:assert observed==captures
  else:captures=observed
# Mutante limitado a suprimir obligación documental; bibliotecas previas intactas.
d=dest/'debug';flags=['--edition=2021','-C','opt-level=0','-C','overflow-checks=yes'];deps=['--extern',f'g1={d}/libg1.rlib','--extern',f'lote={d}/liblote.rlib','-L',f'dependency={d}'];lib=d/'libsin_obligacion.rlib';exe=d/'sin-obligacion'
run('sin-obligacion biblioteca',[args.rustc,*flags,'--crate-type=lib','--crate-name=cobertura',dest/'sensibilidad/sin-obligacion/cobertura.rs',*deps,'-o',lib])
run('sin-obligacion contraste',[args.rustc,*flags,dest/'codigo/contraste.rs',*deps,'--extern',f'cobertura={lib}','-o',exe])
run('sin-obligacion detección',[exe],code=1,env=environment(exe,d/'capturas-sin-obligacion'),contains='FALLO CI02')
mut=dest/'fuentes-sin-revocacion';shutil.copytree(src,mut);shutil.copy2(dest/'sensibilidad/sin-revocacion/servicio.rs',mut/'semantica-a/servicio.rs')
d=dest/'sin-revocacion';exe,_,_=build(d,mut,dest/'codigo/cobertura.rs')
run('sin-revocacion detección',[exe],code=1,env=environment(exe,d/'capturas'),contains='FALLO CA02')
assert len(logs)==23
# Relación documental con cuerpos esperados; no resolución semántica en Python.
positive=(dest/'debug/capturas-1/P3-01.cuerpo').read_bytes();negative=(dest/'debug/capturas-1/P3-11.cuerpo').read_bytes()
assert positive==(dest/'esperados/P3-01.cuerpo').read_bytes() and negative==(dest/'esperados/P3-11.cuerpo').read_bytes()
js(dest/'RESULTADO.json',dict(version='S2-RESULTADO/1',conforme=True,controles_por_ejecucion=12,ejecuciones=6,observaciones=72,capturas_identicas_por_ejecucion=len(captures),invocaciones=len(logs),sensibilidades={'sin-obligacion':'CI02','sin-revocacion':'CA02'},pared_total_s=time.time()-start,sha256_cuerpo_positivo=sha(positive),sha256_cuerpo_negativo=sha(negative),limite='Causalidad y cobertura del banco sintético nativo; no revocación profesional ni cierre integral C/I o A–L.'))
print('S2 conforme en alcance fijado; presupuesto cerrado.',flush=True)
