from pathlib import Path
import subprocess,json,hashlib
r=Path('tmp/enlace-153').resolve();o=Path('entregas/pertenencia-de-permisos-a-su-continuidad').resolve();rust=Path('tmp/presentacion-150/toolchain/installed/bin/rustc').resolve();logs=[]
for name,args in [('compilar',[rust,'--edition=2021','--test',r/'base/src/lib.rs','-o',r/'base-test']),('contrastar',[r/'base-test','pc0','--nocapture','--test-threads=1'])]:
 p=subprocess.run(list(map(str,args)),capture_output=True,timeout=60);logs.append({'paso':name,'argv':list(map(str,args)),'codigo':p.returncode,'stdout':p.stdout.decode(),'stderr':p.stderr.decode()});(o/'ANTECEDENTE.json').write_text(json.dumps(logs,ensure_ascii=False,indent=2)+'\n');print(name,p.returncode,p.stdout.decode()[-2300:],p.stderr.decode()[-700:],flush=True)
 if name=='compilar':assert p.returncode==0
