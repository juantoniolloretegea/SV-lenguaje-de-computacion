#!/usr/bin/env python3
# ES: Administración reproducible. Toda admisión y observación se ejecuta en Rust.
# EN: Reproducible administration. All admission and observation runs in Rust.
from pathlib import Path
import sys,tarfile,shutil,subprocess,json,hashlib,os,time
D=Path(__file__).resolve().parents[1]
if len(sys.argv)!=2:raise SystemExit('Uso / usage: python soporte/reproducir.py DIRECTORIO_NUEVO')
W=Path(sys.argv[1]).resolve();W.mkdir(parents=True,exist_ok=False)
for archive in ('sv_core-verificado.tar.gz','ENTRADAS.tar.gz'):
 with tarfile.open(D/archive) as t:t.extractall(W,filter='data')
for p,h in json.loads((D/'NUCLEO_FUENTES.json').read_text())['archivos'].items():assert hashlib.sha256((W/'sv_core'/p).read_bytes()).hexdigest()==h,p
for p,h in json.loads((D/'ENTRADAS_MATERIALIZADAS.json').read_text()).items():assert hashlib.sha256((W/'entradas'/p).read_bytes()).hexdigest()==h,p
shutil.copytree(D/'proyecto',W/'proyecto');out=W/'ejecucion';out.mkdir();commands=[]
def run(args,name):
 r=subprocess.run([str(x) for x in args],capture_output=True);(out/(name+'.stdout')).write_bytes(r.stdout);(out/(name+'.stderr')).write_bytes(r.stderr);commands.append({'orden':[str(x) for x in args],'exit_code':r.returncode});(out/'ORDENES.json').write_text(json.dumps(commands,indent=2)+'\n');return r
for tool in ('rustc','cargo'):assert run([tool,'--version'],tool).returncode==0
for operation in ('build','test'):
 args=['cargo',operation,'--locked','--offline','--manifest-path',W/'proyecto/Cargo.toml']
 if operation=='test':args.append('--lib')
 assert run(args,operation).returncode==0,operation
target=W/'proyecto/target/debug';deps=target/'deps';core=next(deps.glob('libsv_core-*.rlib'));checks=[]
for spec in json.loads((D/'sondas/ESPERADOS.json').read_text()):
 id=spec['id'];r=run(['rustc','--edition=2021','--crate-type=lib','--error-format=json','--extern','sv_bis_i0205='+str(target/'libsv_bis_i0205.rlib'),'--extern','sv_core='+str(core),'-L','dependency='+str(deps),D/'sondas'/f'{id}.rs','-o',out/(id+'.rlib')],id)
 diagnostics=[json.loads(l) for l in r.stderr.decode().splitlines() if l.startswith('{')];codes={x['code']['code'] for x in diagnostics if x.get('level')=='error' and x.get('code')};ok=(r.returncode==0) if spec['compila'] else (r.returncode!=0 and codes=={spec['codigo']});checks.append({'id':id,'conforme':ok,'codigos':sorted(codes)})
(out/'ENCAPSULACION.json').write_text(json.dumps(checks,indent=2)+'\n')
args=[str(target/'sv_bis_i0205'),str(W/'entradas'),str(out/'resultados')];start=time.monotonic_ns()
with (out/'campana.stdout').open('wb') as stdout,(out/'campana.stderr').open('wb') as stderr:
 p=subprocess.Popen(args,stdout=stdout,stderr=stderr);_,status,usage=os.wait4(p.pid,0);p.returncode=os.waitstatus_to_exitcode(status)
commands.append({'orden':args,'exit_code':p.returncode});(out/'ORDENES.json').write_text(json.dumps(commands,indent=2)+'\n')
(out/'RECURSOS.json').write_text(json.dumps({'elapsed_ns':time.monotonic_ns()-start,'max_rss_kib_linux':usage.ru_maxrss,'user_seconds':usage.ru_utime,'system_seconds':usage.ru_stime},indent=2)+'\n')
print((out/'campana.stdout').read_text());print('Evidencia / evidence:',out)
raise SystemExit(0 if p.returncode==0 and all(x['conforme'] for x in checks) else 1)
