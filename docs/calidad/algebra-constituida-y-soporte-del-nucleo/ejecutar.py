"""Caracterización del compilador existente. No implementa semántica SV.
Uso: python ejecutar.py RUTA_RUSTC RUTA_SV_CORE DIRECTORIO_RESULTADOS
"""
from pathlib import Path
import sys, subprocess, json, hashlib
here=Path(__file__).resolve().parent
rustc=Path(sys.argv[1]).resolve(); core=Path(sys.argv[2]).resolve(); dest=Path(sys.argv[3]).resolve(); dest.mkdir(parents=True,exist_ok=True)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
commands=[]
def run(args):
 r=subprocess.run([str(a) for a in args],capture_output=True,text=True)
 commands.append(dict(argv=[str(a) for a in args],codigo=r.returncode,stdout=r.stdout,stderr=r.stderr))
 return r
version=run([rustc,'-Vv']);assert version.returncode==0
lib=dest/'libsv_core.rlib'; exe=dest/'sonda'
assert run([rustc,'--edition=2021','--crate-name','sv_core','--crate-type=rlib',core/'src/lib.rs','-o',lib]).returncode==0
assert run([rustc,'--edition=2021',here/'sonda.rs','--extern',f'sv_core={lib}','-o',exe]).returncode==0
plan=json.loads((here/'PLAN_FIJADO.json').read_text()); results=[]
for case in plan:
 p=here/'casos'/(case['caso']+'.svp'); r=run([exe,p]); ok=r.returncode==0
 (dest/(case['caso']+'.stdout')).write_text(r.stdout)
 (dest/(case['caso']+'.stderr')).write_text(r.stderr)
 row={**case,'codigo_observado':r.returncode,'coincide_caracterizacion':ok==case['aceptacion_estructural_esperada'] and (ok or (r.returncode==2 and r.stderr==case['stderr_esperado'])),'entrada_sha256':sha(p)}
 if ok:
  row['operaciones_emitidas']=json.loads(r.stdout).get('operations',[])
 results.append(row)
(dest/'RESULTADOS.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n')
(dest/'COMANDOS.json').write_text(json.dumps(commands,ensure_ascii=False,indent=2)+'\n')
(dest/'REALIZACION.json').write_text(json.dumps({'rustc':str(rustc),'rustc_sha256':sha(rustc),'version':version.stdout,'sv_core_sha256':{str(p.relative_to(core)):sha(p) for p in sorted((core/'src').glob('*.rs'))},'rlib_sha256':sha(lib),'sonda_sha256':sha(exe),'plan_sha256':sha(here/'PLAN_FIJADO.json')},ensure_ascii=False,indent=2)+'\n')
print(json.dumps({'casos':len(results),'coincidencias':sum(r['coincide_caracterizacion'] for r in results),'procesos':len(commands)}))
for r in results:
 if not r['coincide_caracterizacion']:print(r['caso'],(dest/(r['caso']+'.stderr')).read_text())
sys.exit(0 if all(r['coincide_caracterizacion'] for r in results) else 1)
