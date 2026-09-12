#!/usr/bin/env python3
"""S18: custodia y orquestación; producción y comprobación funcional en Rust/G1."""
from pathlib import Path
import argparse,base64,hashlib,json,os,resource,shutil,subprocess,time
p=argparse.ArgumentParser();p.add_argument('--rustc',required=True);p.add_argument('--salida',required=True);a=p.parse_args();D=Path(__file__).resolve().parent;W=Path(a.salida).resolve();sha=lambda b:hashlib.sha256(b).hexdigest()
if W.exists():raise SystemExit('Salida ya existente: no sobrescribir campaña ni reintentar.')
W.mkdir(parents=True)
def js(p,x):p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
def require(ok,msg):
 if not ok:raise RuntimeError(msg)
f=json.loads((D/'FIJACION_PREVIA.json').read_text())
for n,h in f['archivos'].items():require(sha((D/n).read_bytes())==h,'Fijación distinta: '+n)
for x in json.loads((D/'FUENTES_S2.json').read_text())['archivos']:
 q=W/x['ruta'];require(q.resolve().is_relative_to(W),'Ruta fuera de salida');b=base64.b64decode(x['base64'],validate=True);require(len(b)==x['bytes'] and sha(b)==x['sha256'],'Fuente alterada');q.parent.mkdir(parents=True,exist_ok=True);q.write_bytes(b)
shutil.copytree(D/'codigo',W/'codigo');shutil.copytree(D/'esperados',W/'esperados')
logs=[];digests=None;bins={};start=time.monotonic()
def limits():
 resource.setrlimit(resource.RLIMIT_AS,(2*1024**3,2*1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_FSIZE,(32*1024**2,32*1024**2))
def run(label,cmd,code=0,env=None,contains=None):
 require(len(logs)<29,'Presupuesto excedido');r=dict(paso=label,argv=list(map(str,cmd)),esperado_exit=code)
 if env:r['entorno_fijado']={k:env[k] for k in ['LG1_FUENTES','LG1_BINARIO','S18_CAPTURAS']}
 before=resource.getrusage(resource.RUSAGE_CHILDREN);t=time.monotonic()
 try:
  v=subprocess.run(r['argv'],capture_output=True,env=env,timeout=60,preexec_fn=limits);r.update(exit_code=v.returncode,stdout=v.stdout.decode(errors='replace'),stderr=v.stderr.decode(errors='replace'),stdout_base64=base64.b64encode(v.stdout).decode(),stderr_base64=base64.b64encode(v.stderr).decode(),stdout_sha256=sha(v.stdout),stderr_sha256=sha(v.stderr))
 except Exception as e:
  r['error']=repr(e);logs.append(r);js(W/'COMANDOS.json',logs);raise
 after=resource.getrusage(resource.RUSAGE_CHILDREN);r.update(pared_s=time.monotonic()-t,cpu_usuario_s=after.ru_utime-before.ru_utime,cpu_sistema_s=after.ru_stime-before.ru_stime,rss_individual=None,rss_max_hijos_hasta_paso_kib=after.ru_maxrss);logs.append(r);js(W/'COMANDOS.json',logs)
 require(v.returncode==code,'Salida inesperada: '+label)
 if contains:require(contains in r['stderr'],'Testigo esperado ausente: '+label)
 print(label+' correcto',flush=True);return v.stdout
expected=(D/'codigo/ESPERADO.tsv').read_bytes()
def environment(exe,cap):cap.mkdir();return os.environ|dict(LG1_FUENTES=sha((D/'FIJACION_PREVIA.json').read_bytes()),LG1_BINARIO=sha(exe.read_bytes()),S18_CAPTURAS=str(cap))
def savebin(k,p):bins[k]=dict(sha256=sha(p.read_bytes()),bytes=p.stat().st_size)
try:
 v=run('compilador',[a.rustc,'-vV']);require(b'release: 1.98.0\n' in v,'Versión distinta');require(sha(Path(a.rustc).read_bytes())=='3690cc576ede140504698405d5d8fa3826aaadbe71699c6c4ed0a565d6f493e2','Binario de compilador distinto');js(W/'ENTORNO.json',dict(compilador=v.decode(),rustc_sha256=sha(Path(a.rustc).read_bytes()),sistema=os.uname().sysname,maquina=os.uname().machine,limites_por_invocacion=dict(pared_s=60,cpu_s=60,espacio_virtual_bytes=2*1024**3,archivo_bytes=32*1024**2),rss='ru_maxrss de hijos acumulado hasta cada paso; no RSS individual de cada invocación.'))
 for mode in ['debug','release']:
  d=W/mode;d.mkdir();flags=['--edition=2021','-C','opt-level='+('3' if mode=='release' else '0'),'-C','overflow-checks='+('no' if mode=='release' else 'yes')];g=d/'libg1.rlib';lib=d/'librecorrido.rlib';exe=d/'contraste'
  run(mode+' g1',[a.rustc,*flags,'--crate-type=lib','--crate-name=g1',W/'fuentes-base/recibo-g1/candidata/lib.rs','-o',g]);deps=['--extern',f'g1={g}','-L',f'dependency={d}']
  run(mode+' recorrido',[a.rustc,*flags,'--crate-type=lib','--crate-name=recorrido',W/'codigo/recorrido.rs',*deps,'-o',lib]);deps+=['--extern',f'recorrido={lib}']
  run(mode+' conductor',[a.rustc,*flags,W/'codigo/contraste.rs',*deps,'-o',exe]);savebin(mode,exe)
  for n in range(1,4):
   cap=d/('capturas-'+str(n));out=run(mode+' ejecución '+str(n),[exe],env=environment(exe,cap));require(out==expected,'Salida del banco distinta');cur={p.name:sha(p.read_bytes()) for p in cap.iterdir()};require(digests is None or cur==digests,'Capturas no idénticas entre ejecuciones');digests=cur
 for mutant,case in [('omitir_contexto','I04'),('forzar_consumo_positivo','I14'),('omitir_base_citada','I09'),('admitir_negativa','I12')]:
  d=W/mutant;d.mkdir();lib=d/'librecorrido.rlib';exe=d/'contraste';deps=['--extern',f'g1={W}/debug/libg1.rlib','-L',f'dependency={W}/debug'];flags=['--edition=2021','-C','opt-level=0','-C','overflow-checks=yes']
  run(mutant+' compilación biblioteca',[a.rustc,*flags,'--cfg',mutant,'--crate-type=lib','--crate-name=recorrido',W/'codigo/recorrido.rs',*deps,'-o',lib]);deps+=['--extern',f'recorrido={lib}']
  run(mutant+' compilación conductor',[a.rustc,*flags,W/'codigo/contraste.rs',*deps,'-o',exe]);savebin(mutant,exe)
  run(mutant+' detección',[exe],code=1,env=environment(exe,d/'capturas'),contains='FALLO '+case)
 deps=['--extern',f'g1={W}/debug/libg1.rlib','--extern',f'recorrido={W}/debug/librecorrido.rlib','-L',f'dependency={W}/debug'];client=W/'debug/cliente_valido'
 run('cliente válido compilación',[a.rustc,'--edition=2021',W/'codigo/cliente_valido.rs',*deps,'-o',client]);require(run('cliente válido ejecución',[client])==b'CLIENTE_VALIDO\n','Cliente válido distinto')
 for n in ['fabricar_referencia','fabricar_entrega']:
  run(n+' rechazado',[a.rustc,'--edition=2021',W/'codigo'/f'{n}.rs',*deps,'-o',W/'debug'/n],code=1,contains='error[E0451]')
 require(len(logs)==29,'Presupuesto incompleto');js(W/'RESULTADO.json',dict(version='S18-RESULTADO/1',conforme=True,casos=24,ejecuciones_normales=6,observaciones=144,invocaciones=len(logs),capturas_identicas_por_ejecucion=len(digests),sensibilidades={'omitir_contexto':'I04','forzar_consumo_positivo':'I14','omitir_base_citada':'I09','admitir_negativa':'I12'},clientes_validos=1,fabricaciones_rechazadas=2,pared_campana_s=time.monotonic()-start,limite='Recorrido documental sintético con host confiable; sin proveedor, red real, autoridad profesional, efecto R1, persistencia tras caída ni promoción nuclear.'))
except Exception as e:
 js(W/'INTERRUPCION.json',dict(error=repr(e),invocaciones=len(logs),pared_s=time.monotonic()-start,conforme=False));raise
finally:js(W/'BINARIOS.json',bins)
print('S18 conforme en el banco fijado; no amplía su alcance.',flush=True)
