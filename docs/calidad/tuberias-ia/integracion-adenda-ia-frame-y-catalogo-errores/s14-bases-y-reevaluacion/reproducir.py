#!/usr/bin/env python3
"""Reproduce S14 G/J: orquestación y cotejo; decisiones en Rust/G1."""
from pathlib import Path
import argparse,base64,hashlib,json,os,resource,shutil,subprocess,time
p=argparse.ArgumentParser();p.add_argument('--rustc',required=True);p.add_argument('--salida',required=True);a=p.parse_args();D=Path(__file__).resolve().parent;W=Path(a.salida).resolve();sha=lambda b:hashlib.sha256(b).hexdigest()
if W.exists():raise SystemExit('La salida debe ser nueva: no se sobrescribe una campaña.')
W.mkdir(parents=True)
def js(p,x):p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
def require(ok,msg):
 if not ok:raise RuntimeError(msg)
f=json.loads((D/'FIJACION_PREVIA.json').read_text())
for n,h in f['archivos'].items():require(sha((D/n).read_bytes())==h,'Fijación distinta: '+n)
for x in json.loads((D/'FUENTES_S2.json').read_text())['archivos']:
 q=W/x['ruta'];require(q.resolve().is_relative_to(W),'Ruta fuera de salida');b=base64.b64decode(x['base64']);require(len(b)==x['bytes'] and sha(b)==x['sha256'],'Fuente alterada');q.parent.mkdir(parents=True,exist_ok=True);q.write_bytes(b)
shutil.copytree(D/'codigo',W/'codigo');shutil.copytree(D/'especimenes',W/'especimenes')
logs=[];digests=None;bins={};start=time.monotonic()
def run(label,cmd,code=0,env=None,contains=None):
 require(len(logs)<17,'Presupuesto excedido');r=dict(paso=label,argv=list(map(str,cmd)),esperado_exit=code)
 if env:r['entorno_fijado']={k:env[k] for k in ['LG1_FUENTES','LG1_BINARIO','S14_CAPTURAS']}
 before=resource.getrusage(resource.RUSAGE_CHILDREN);t=time.monotonic()
 try:
  v=subprocess.run(r['argv'],capture_output=True,env=env,timeout=60);r.update(exit_code=v.returncode,stdout=v.stdout.decode(errors='replace'),stderr=v.stderr.decode(errors='replace'))
 except Exception as e:
  r['error']=repr(e);logs.append(r);js(W/'COMANDOS.json',logs);raise
 after=resource.getrusage(resource.RUSAGE_CHILDREN);r.update(pared_s=time.monotonic()-t,cpu_usuario_s=after.ru_utime-before.ru_utime,cpu_sistema_s=after.ru_stime-before.ru_stime,rss_individual=None);logs.append(r);js(W/'COMANDOS.json',logs)
 require(v.returncode==code,'Salida inesperada: '+label)
 if contains:require(contains in r['stderr'],'Sensibilidad no detectada en caso fijado: '+label)
 print(label+' correcto',flush=True);return v.stdout
v=run('compilador',[a.rustc,'-vV']);require(b'release: 1.98.0\n' in v,'Versión distinta');require(sha(Path(a.rustc).read_bytes())=='3690cc576ede140504698405d5d8fa3826aaadbe71699c6c4ed0a565d6f493e2','Binario de compilador distinto')
js(W/'ENTORNO.json',dict(compilador=v.decode(),rustc_sha256=sha(Path(a.rustc).read_bytes()),sistema=os.uname().sysname,maquina=os.uname().machine))
def env(exe,cap):
 cap.mkdir();return os.environ|dict(LG1_FUENTES=sha((D/'FIJACION_PREVIA.json').read_bytes()),LG1_BINARIO=sha(exe.read_bytes()),S14_CAPTURAS=str(cap))
expected=(D/'codigo/ESPERADO.tsv').read_bytes()
try:
 for mode in ['debug','release']:
  d=W/mode;d.mkdir();flags=['--edition=2021','-C','opt-level='+('3' if mode=='release' else '0'),'-C','overflow-checks='+('no' if mode=='release' else 'yes')];g=d/'libg1.rlib';exe=d/'contraste'
  run(mode+' biblioteca',[a.rustc,*flags,'--crate-type=lib','--crate-name=g1',W/'fuentes-base/recibo-g1/candidata/lib.rs','-o',g])
  run(mode+' conductor',[a.rustc,*flags,W/'codigo/contraste.rs','--extern',f'g1={g}','-o',exe]);bins[mode]=dict(sha256=sha(exe.read_bytes()),bytes=exe.stat().st_size)
  for n in range(1,4):
   cap=d/('capturas-'+str(n));out=run(mode+' ejecución '+str(n),[exe],env=env(exe,cap));require(out==expected,'Salida de banco distinta')
   cur={p.name:sha(p.read_bytes()) for p in cap.iterdir()};require(digests is None or cur==digests,'Capturas no idénticas');digests=cur
 for mutant,case in [('sin_identidad_carga','GJ02'),('consumo_falso','GJ04'),('sin_base_original','GJ06')]:
  exe=W/'debug'/mutant;cap=W/'debug'/('capturas-'+mutant)
  run(mutant+' compilación',[a.rustc,'--edition=2021','-C','opt-level=0','-C','overflow-checks=yes','--cfg',mutant,W/'codigo/contraste.rs','--extern',f'g1={W}/debug/libg1.rlib','-o',exe]);bins[mutant]=dict(sha256=sha(exe.read_bytes()),bytes=exe.stat().st_size)
  run(mutant+' detección',[exe],code=1,env=env(exe,cap),contains='FALLO '+case)
 require(len(logs)==17,'Presupuesto incompleto');js(W/'RESULTADO.json',dict(version='S14-RESULTADO/1',conforme=True,casos=12,ejecuciones_normales=6,observaciones=72,invocaciones=len(logs),capturas_identicas_por_ejecucion=len(digests),sensibilidades={'sin_identidad_carga':'GJ02','consumo_falso':'GJ04','sin_base_original':'GJ06'},pared_campana_s=time.monotonic()-start,limite='Host confiable y custodia intraproceso. Sin promoción nuclear, historia durable, QueryResult nativo ni comportamiento de LLM.'))
except Exception as e:
 js(W/'INTERRUPCION.json',dict(error=repr(e),invocaciones=len(logs),pared_s=time.monotonic()-start,conforme=False));raise
finally:js(W/'BINARIOS.json',bins)
print('S14 conforme dentro de contrato y presupuesto fijados.',flush=True)
