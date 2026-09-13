#!/usr/bin/env python3
# ES: Administración de archivos y procesos; admisión y comparación exclusivamente Rust.
# EN: File/process administration; admission and comparison exclusively in Rust.
from pathlib import Path
import sys,tarfile,shutil,subprocess,json,hashlib,os,time
D=Path(__file__).resolve().parents[1]
if len(sys.argv)!=2:raise SystemExit('Uso / usage: python soporte/reproducir.py DIRECTORIO_NUEVO')
W=Path(sys.argv[1]).resolve();W.mkdir(parents=True,exist_ok=False)
for archive in ('sv_core-verificado.tar.gz','ADMISION_VERIFICADA.tar.gz','ENTRADAS.tar.gz'):
 with tarfile.open(D/archive) as t:t.extractall(W,filter='data')
for root,name in [('sv_core','NUCLEO_FUENTES.json'),('sv_bis_i0205','ADMISION_FUENTES.json'),('entradas','ENTRADAS_MATERIALIZADAS.json')]:
 manifest=json.loads((D/name).read_text());manifest=manifest['archivos'] if root=='sv_core' else manifest
 for p,h in manifest.items():assert hashlib.sha256((W/root/p).read_bytes()).hexdigest()==h,p
shutil.copytree(D/'proyecto',W/'proyecto');out=W/'ejecucion';out.mkdir();commands=[]
def run(args,name):
 r=subprocess.run([str(x) for x in args],capture_output=True);(out/(name+'.stdout')).write_bytes(r.stdout);(out/(name+'.stderr')).write_bytes(r.stderr);commands.append({'orden':[str(x) for x in args],'exit_code':r.returncode});(out/'ORDENES.json').write_text(json.dumps(commands,indent=2)+'\n');return r
for tool in ('rustc','cargo'):assert run([tool,'--version'],tool).returncode==0
assert run(['cargo','build','--locked','--offline','--manifest-path',W/'proyecto/Cargo.toml'],'build').returncode==0,'cargo build; inspect build.stderr'
binary=W/'proyecto/target/debug/sv_bis_extension';args=[str(binary),str(W/'entradas'),str(out/'resultados')];start=time.monotonic_ns()
with (out/'campana.stdout').open('wb') as stdout,(out/'campana.stderr').open('wb') as stderr:
 p=subprocess.Popen(args,stdout=stdout,stderr=stderr);_,status,usage=os.wait4(p.pid,0);p.returncode=os.waitstatus_to_exitcode(status)
commands.append({'orden':args,'exit_code':p.returncode});(out/'ORDENES.json').write_text(json.dumps(commands,indent=2)+'\n')
(out/'RECURSOS.json').write_text(json.dumps({'elapsed_ns':time.monotonic_ns()-start,'max_rss_kib_linux':usage.ru_maxrss,'user_seconds':usage.ru_utime,'system_seconds':usage.ru_stime,'ejecutable_sha256':hashlib.sha256(binary.read_bytes()).hexdigest()},indent=2)+'\n')
print((out/'campana.stdout').read_text());print('Evidencia / evidence:',out)
raise SystemExit(p.returncode)
