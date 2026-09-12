from pathlib import Path
import subprocess,json,hashlib,base64
r=Path('tmp/enlace-153').resolve();o=Path('entregas/pertenencia-de-permisos-a-su-continuidad').resolve();rust=Path('tmp/presentacion-150/toolchain/installed/bin/rustc').resolve();logs=[];sha=lambda b:hashlib.sha256(b).hexdigest()
caps={'archivos':[{'ruta':str(p.relative_to(r/'candidata')),'sha256':sha(p.read_bytes()),'base64':base64.b64encode(p.read_bytes()).decode()}for p in sorted((r/'candidata').rglob('*'))if p.is_file()]};(o/'CANDIDATA_FUENTES.json').write_text(json.dumps(caps,indent=2)+'\n')
for mode,flags in [('debug',['-C','opt-level=0','-C','overflow-checks=yes']),('release',['-C','opt-level=3','-C','overflow-checks=no'])]:
 for name,args in [('compilar',[rust,'--edition=2021','--test',r/'candidata/rust/sv_core/src/lib.rs',*flags,'-o',r/(mode+'-test')]),('regresion',[r/(mode+'-test'),'--nocapture','--test-threads=1'])]:
  p=subprocess.run(list(map(str,args)),capture_output=True,timeout=60);logs.append({'modo':mode,'paso':name,'argv':list(map(str,args)),'codigo':p.returncode,'stdout':p.stdout.decode(),'stderr':p.stderr.decode()});(o/'CANDIDATA_RESULTADOS.json').write_text(json.dumps(logs,ensure_ascii=False,indent=2)+'\n');print(mode,name,p.returncode,p.stdout.decode()[-900:],p.stderr.decode()[-300:],flush=True);assert p.returncode==0
