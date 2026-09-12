#!/usr/bin/env python3
"""Reproduce el contraste S3 acotado en Linux, con Rust 1.98.0."""
from pathlib import Path
import argparse,base64,hashlib,json,os,resource,shutil,subprocess,time
p=argparse.ArgumentParser();p.add_argument('--rustc',default='rustc');p.add_argument('--salida',required=True);a=p.parse_args()
root=Path(__file__).resolve().parent;dest=Path(a.salida).resolve();sha=lambda b:hashlib.sha256(b).hexdigest()
if dest.exists():raise SystemExit('El destino de reproducción debe ser nuevo.')
dest.mkdir(parents=True)
def js(p,x):p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
fixed=json.loads((root/'FIJACION_PREVIA.json').read_text())
for name,h in fixed['archivos'].items():assert sha((root/name).read_bytes())==h,name
bank=json.loads((root/'BANCO_FIJADO.json').read_text());assert sha((root/'FUENTES_S2.json').read_bytes())==bank['capsula_s2_sha256']
for x in json.loads((root/'FUENTES_S2.json').read_text())['archivos']:
 p=dest/x['ruta'];assert p.resolve().is_relative_to(dest);b=base64.b64decode(x['base64']);assert len(b)==x['bytes'] and sha(b)==x['sha256'];p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(b)
assert sha((root/'ESPECIMENES.json').read_bytes())==bank['especimenes_sha256'];(dest/'especimenes').mkdir()
for name,x in json.loads((root/'ESPECIMENES.json').read_text()).items():
 b=base64.b64decode(x['base64']);assert len(b)==x['bytes'] and sha(b)==x['sha256'];(dest/'especimenes'/name).write_bytes(b)
shutil.copytree(root/'codigo',dest/'codigo');shutil.copytree(root/'sensibilidad',dest/'sensibilidad');shutil.copy2(root/'DECLARACION_VIGENCIA.json',dest/'DECLARACION_VIGENCIA.json')
logs=[];captures={};expected=(root/'codigo/ESPERADO.tsv').read_bytes();start=time.monotonic()
def run(label,cmd,code=0,env=None,contains=None):
 assert len(logs)<22;record=dict(paso=label,argv=list(map(str,cmd)),esperado_exit=code);before=resource.getrusage(resource.RUSAGE_CHILDREN);t=time.monotonic()
 if env:record['entorno_fijado']={k:env[k] for k in ['LG1_FUENTES','LG1_BINARIO','S3_CAPTURAS']}
 try:
  v=subprocess.run(record['argv'],capture_output=True,timeout=60,env=env);record.update(exit_code=v.returncode,stdout=v.stdout.decode(errors='replace'),stderr=v.stderr.decode(errors='replace'))
 except Exception as e:
  record['error']=repr(e);logs.append(record);js(dest/'COMANDOS.json',logs);raise
 after=resource.getrusage(resource.RUSAGE_CHILDREN);record.update(pared_s=time.monotonic()-t,cpu_usuario_s=after.ru_utime-before.ru_utime,cpu_sistema_s=after.ru_stime-before.ru_stime,rss_proceso=None);logs.append(record);js(dest/'COMANDOS.json',logs)
 assert v.returncode==code,record
 if contains:assert contains in record['stderr'],record
 print(label,'correcto',flush=True);return v.stdout
v=run('compilador',[a.rustc,'-vV']);assert b'release: 1.98.0\n' in v
js(dest/'ENTORNO.json',dict(version=v.decode(),compilador_sha256=sha(Path(a.rustc).read_bytes()) if Path(a.rustc).is_file() else None,sistema=os.uname().sysname,maquina=os.uname().machine))
def env(exe,cap):
 cap.mkdir();return os.environ|{'LG1_FUENTES':sha((root/'FIJACION_PREVIA.json').read_bytes()),'LG1_BINARIO':sha(exe.read_bytes()),'S3_CAPTURAS':str(cap)}
def build(d,src):
 d.mkdir();flags=['--edition=2021','-C','opt-level='+('3' if d.name=='release' else '0'),'-C','overflow-checks='+('no' if d.name=='release' else 'yes')];g=d/'libg1.rlib';l=d/'liblote.rlib';c=d/'libcobertura.rlib'
 run(d.name+' g1',[a.rustc,*flags,'--crate-type=lib','--crate-name=g1',src/'recibo-g1/candidata/lib.rs','-o',g])
 run(d.name+' lote',[a.rustc,*flags,'--crate-type=lib','--crate-name=lote',src/'lote-g1/lib.rs','--extern',f'g1={g}','-o',l])
 deps=['--extern',f'g1={g}','--extern',f'lote={l}','-L',f'dependency={d}']
 run(d.name+' cobertura',[a.rustc,*flags,'--crate-type=lib','--crate-name=cobertura',dest/'cobertura/cobertura.rs',*deps,'-o',c]);deps+=['--extern',f'cobertura={c}']
 exe=d/'contraste';run(d.name+' conductor',[a.rustc,*flags,dest/'codigo/contraste.rs',*deps,'-o',exe]);return exe,flags,deps
for mode in ['debug','release']:
 exe,_,_=build(dest/mode,dest/'fuentes-base')
 for n in range(1,4):
  cap=dest/mode/('capturas-'+str(n));out=run(mode+' ejecución '+str(n),[exe],env=env(exe,cap));assert out==expected
  actual={p.name:sha(p.read_bytes()) for p in cap.iterdir()}
  if captures:assert actual==captures
  else:captures=actual
mut=dest/'fuentes-sin-comparacion';shutil.copytree(dest/'fuentes-base',mut);shutil.copy2(dest/'sensibilidad/presentacion-sin-comparacion.rs',mut/'recibo-g1/candidata/presentacion.rs')
exe,_,_=build(dest/'sin-comparacion',mut);run('contenido desactivado detectado',[exe],code=1,env=env(exe,dest/'sin-comparacion/capturas'),contains='FALLO D03')
d=dest/'debug';shutil.copy2(dest/'sensibilidad/contraste-observador-circular.rs',dest/'codigo/contraste-circular.rs');flags=['--edition=2021','-C','opt-level=0','-C','overflow-checks=yes'];deps=['--extern',f'g1={d}/libg1.rlib','--extern',f'lote={d}/liblote.rlib','--extern',f'cobertura={d}/libcobertura.rlib','-L',f'dependency={d}'];exe=d/'observador-circular'
run('observador circular compilación',[a.rustc,*flags,dest/'codigo/contraste-circular.rs',*deps,'-o',exe]);run('observador circular detectado',[exe],code=1,env=env(exe,d/'capturas-circular'),contains='FALLO D04')
assert len(logs)==22
js(dest/'RESULTADO.json',dict(version='S3-RESULTADO/1',conforme=True,controles=8,ejecuciones=6,observaciones=48,capturas_identicas_por_ejecucion=len(captures),invocaciones=22,sensibilidades={'comparacion desactivada':'D03','observacion circular':'D04'},pared_campana_s=time.monotonic()-start,limite='Archivo nativo y proceso confiable; no pantalla, revisión humana ni imposición frente al host.'))
print('S3 conforme en alcance fijado; presupuesto cerrado.',flush=True)
