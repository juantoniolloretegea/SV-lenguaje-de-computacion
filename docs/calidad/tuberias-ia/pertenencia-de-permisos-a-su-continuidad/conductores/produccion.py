from pathlib import Path
import subprocess,json,hashlib
r=Path('tmp/enlace-153').resolve();o=Path('entregas/pertenencia-de-permisos-a-su-continuidad').resolve();rust=Path('tmp/presentacion-150/toolchain/installed/bin/rustc').resolve();logs=[];sizes={}
def run(name,args,code=0,errors=()):
 p=subprocess.run(list(map(str,args)),capture_output=True,timeout=60);row={'paso':name,'argv':list(map(str,args)),'codigo':p.returncode,'stdout':p.stdout.decode(),'stderr':p.stderr.decode()};logs.append(row);(o/'FRONTERA_PUBLICA_Y_COSTE.json').write_text(json.dumps({'procesos':logs,'tamanos':sizes},ensure_ascii=False,indent=2)+'\n');assert p.returncode==code,row
 for e in errors:assert e in row['stderr'],row
 print(name,'OK',flush=True);return row
for version in ['previo','candidata']:
 lib=r/('libsv_core_'+version+'.rlib');exe=r/('coste_'+version)
 run(version+' lib sin cfg(test)',[rust,'--edition=2021','--crate-type=lib','--crate-name=sv_core',r/version/'rust/sv_core/src/lib.rs','-C','opt-level=3','-o',lib])
 run(version+' cliente público',[rust,'--edition=2021',r/'coste.rs','--extern',f'sv_core={lib}','-o',exe])
 sizes[version]=json.loads(run(version+' tamaños y continuidad vacía',[exe])['stdout'])
run('mutación de pertenencia y acceso a inyección de pruebas rechazados',[rust,'--edition=2021',r/'no_mutar.rs','--extern',f'sv_core={r}/libsv_core_candidata.rlib','-o',r/'no_mutar'],1,('E0616','E0599'))
(o/'FRONTERA_PUBLICA_Y_COSTE.json').write_text(json.dumps({'procesos':logs,'tamanos':sizes},ensure_ascii=False,indent=2)+'\n')
print(sizes,flush=True)
