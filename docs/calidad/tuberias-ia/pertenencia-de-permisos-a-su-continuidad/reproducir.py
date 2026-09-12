"""Reproduce el contraejemplo y la candidata; no constituye autoridad profesional."""
from pathlib import Path
import sys,json,base64,hashlib,subprocess
here=Path(__file__).resolve().parent
rust=Path(sys.argv[1]).resolve();work=Path(sys.argv[2]).resolve();work.mkdir(exist_ok=False)
for label,capsule in [('previo','PREVIO_FUENTES_CON_SONDA.json'),('candidata','CANDIDATA_FUENTES.json')]:
 for f in json.loads((here/capsule).read_text())['archivos']:
  rel=Path(f['ruta']);assert not rel.is_absolute() and '..' not in rel.parts
  b=base64.b64decode(f['base64']);assert hashlib.sha256(b).hexdigest()==f['sha256']
  p=work/label/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(b)
logs=[]
def run(name,args,expected=0,stdout_contains=(),stderr_contains=()):
 p=subprocess.run(list(map(str,args)),capture_output=True,timeout=60)
 row={'paso':name,'argv':list(map(str,args)),'codigo':p.returncode,'stdout':p.stdout.decode(),'stderr':p.stderr.decode()};logs.append(row);(work/'RESULTADOS.json').write_text(json.dumps(logs,ensure_ascii=False,indent=2)+'\n')
 assert p.returncode==expected,row
 for s in stdout_contains:assert s in row['stdout'],row
 for s in stderr_contains:assert s in row['stderr'],row
 print(name,'conforme al esperado',flush=True)
 return row
run('compilar previo',[rust,'--edition=2021','--test',work/'previo/rust/sv_core/src/lib.rs','-o',work/'previo-test'])
run('reproducir dos fallos',[work/'previo-test','pc0','--nocapture','--test-threads=1'],101,('1 passed; 2 failed','PC02 cruzado: aceptado=true mediado=true','PC03 cruzado: aceptado=true llamadas=1 eventos=2'))
for mode,flags in [('debug',['-C','opt-level=0','-C','overflow-checks=yes']),('release',['-C','opt-level=3','-C','overflow-checks=no'])]:
 exe=work/(mode+'-test')
 run('compilar '+mode,[rust,'--edition=2021','--test',work/'candidata/rust/sv_core/src/lib.rs',*flags,'-o',exe])
 run('regresion '+mode,[exe,'--nocapture','--test-threads=1'],stdout_contains=('217 passed; 0 failed','PC02 cruzado: aceptado=false mediado=false','PC03 cruzado: aceptado=false llamadas=0 eventos=0'))
for version in ['previo','candidata']:
 lib=work/('libsv_core_'+version+'.rlib');exe=work/('coste_'+version)
 run(version+' sin cfg(test)',[rust,'--edition=2021','--crate-type=lib','--crate-name=sv_core',work/version/'rust/sv_core/src/lib.rs','-C','opt-level=3','-o',lib])
 run(version+' cliente público',[rust,'--edition=2021',here/'conductores/coste.rs','--extern',f'sv_core={lib}','-o',exe])
 run(version+' tamaños',[exe])
run('rechazar mutación e inyección de prueba',[rust,'--edition=2021',here/'conductores/no_mutar.rs','--extern',f'sv_core={work}/libsv_core_candidata.rlib','-o',work/'no_mutar'],1,stderr_contains=('E0616','E0599'))
