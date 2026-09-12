#!/usr/bin/env python3
"""Reproduce la cualificación nativa de S1 desde sus fuentes fijadas."""
from pathlib import Path
import argparse,base64,hashlib,json,os,shutil,subprocess
p=argparse.ArgumentParser();p.add_argument('--rustc',default='rustc');p.add_argument('--salida',required=True);args=p.parse_args()
root=Path(__file__).resolve().parent;dest=Path(args.salida).resolve()
if dest.exists():raise SystemExit('La carpeta de salida debe ser nueva.')
dest.mkdir(parents=True);src=dest/'fuentes-base';src.mkdir()
sha=lambda b:hashlib.sha256(b).hexdigest()
base=json.loads((root/'BASE_RETP152.json').read_text());bank=json.loads((root/'BANCO_FIJADO.json').read_text())
assert sha((root/'BASE_RETP152.json').read_bytes())==bank['capsula_base_sha256']
for item in base['archivos']:
 target=src/item['ruta'];assert target.resolve().is_relative_to(src)
 data=base64.b64decode(item['base64']);assert len(data)==item['bytes'] and sha(data)==item['sha256']
 target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(data)
shutil.copytree(root/'codigo',dest/'codigo');shutil.copytree(root/'sensibilidad',dest/'sensibilidad')
expected=(root/'codigo/ESPERADO.tsv').read_text()
archive=json.loads((root/'EVIDENCIAS_S1.json').read_text());prefix='capturas/cualificacion-debug-1/'
captures={x['ruta'][len(prefix):]:x for x in archive['archivos'] if x['ruta'].startswith(prefix)}
logs=[]
def run(label,cmd,code=0,env=None,contains=None):
 result=subprocess.run(list(map(str,cmd)),capture_output=True,timeout=60,env=env)
 record=dict(paso=label,argv=list(map(str,cmd)),exit_code=result.returncode,stdout=result.stdout.decode(errors='replace'),stderr=result.stderr.decode(errors='replace'))
 logs.append(record);(dest/'COMANDOS_REPRODUCCION.json').write_text(json.dumps(logs,ensure_ascii=False,indent=2)+'\n')
 assert result.returncode==code,record
 if contains:assert contains in record['stderr'],record
 print(label,'correcto',flush=True);return record
version=run('compilador',[args.rustc,'-vV']);assert 'release: 1.98.0\n' in version['stdout'],'La evidencia original corresponde a Rust 1.98.0.'
def environment(exe,d):
 d.mkdir();return os.environ|{'LG1_FUENTES':sha((root/'BANCO_FIJADO.json').read_bytes()),'LG1_BINARIO':sha(exe.read_bytes()),'S1_CAPTURAS':str(d)}
for mode,opt,overflow in [('debug','0','yes'),('release','3','no')]:
 d=dest/mode;d.mkdir();flags=['--edition=2021','-C','opt-level='+opt,'-C','overflow-checks='+overflow]
 g=d/'libg1.rlib';l=d/'liblote.rlib';c=d/'libcobertura.rlib'
 deps=['--extern',f'g1={g}','--extern',f'lote={l}','-L',f'dependency={d}']
 run(mode+' g1',[args.rustc,*flags,'--crate-type=lib','--crate-name=g1',src/'recibo-g1/candidata/lib.rs','-o',g])
 run(mode+' lote',[args.rustc,*flags,'--crate-type=lib','--crate-name=lote',src/'lote-g1/lib.rs','--extern',f'g1={g}','-o',l])
 run(mode+' cobertura',[args.rustc,*flags,'--crate-type=lib','--crate-name=cobertura',dest/'codigo/cobertura.rs',*deps,'-o',c])
 exe=d/'contraste';full=deps+['--extern',f'cobertura={c}']
 run(mode+' contraste',[args.rustc,*flags,dest/'codigo/contraste.rs',*full,'-o',exe])
 for n in range(1,4):
  cap=d/('capturas-'+str(n));observed=run(mode+' ejecución '+str(n),[exe],env=environment(exe,cap));assert observed['stdout']==expected
  assert {x.name for x in cap.iterdir()}==set(captures)
  for name,item in captures.items():assert sha((cap/name).read_bytes())==item['sha256'],name
 run(mode+' referencia no construible',[args.rustc,*flags,dest/'codigo/forjar.rs',*full,'-o',d/'forjar'],code=1,contains='E0451')
 if mode=='debug':
  for name in ['sin-obligacion','referencia-circular']:
   lib=d/('lib'+name.replace('-','_')+'.rlib');mutante=d/name
   run(name+' biblioteca',[args.rustc,*flags,'--crate-type=lib','--crate-name=cobertura',dest/'sensibilidad'/name/'cobertura.rs',*deps,'-o',lib])
   run(name+' contraste',[args.rustc,*flags,dest/'codigo/contraste.rs',*deps,'--extern',f'cobertura={lib}','-o',mutante])
   run(name+' detección',[mutante],code=1,env=environment(mutante,d/('capturas-'+name)),contains='FALLO CI02')
print('Cualificación reproducida en alcance nativo; ocho controles, seis ejecuciones y dos sensibilidades.')
