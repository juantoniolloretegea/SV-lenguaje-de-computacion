# ES: Ejecuta herramientas nativas y conserva sus resultados, sin aceptar SVP en Python.
# EN: Runs native tools and preserves results; Python does not accept SVP.
from pathlib import Path
import subprocess,json,hashlib,shutil,datetime
D=Path('manifiesto-sv/checkout/docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis04-realizacion-i0205-v0_1');W=Path('bis-realizacion/montaje');out=Path('bis-realizacion/campana-01');out.mkdir(exist_ok=False)
for p,h in json.loads((D/'PRECOMPROMISO.json').read_text()).items():assert hashlib.sha256((D/p).read_bytes()).hexdigest()==h
pub=json.loads(Path('bis-realizacion/PREVIO_PUBLICO.json').read_text());lab=json.loads(Path('bis-realizacion/PREVIO_LAB.json').read_text());assert pub['verified'] and lab['verified']
commands=[]
def run(args,name):
 begin=datetime.datetime.now(datetime.timezone.utc).isoformat();res=subprocess.run(args,capture_output=True);(out/(name+'.stdout')).write_bytes(res.stdout);(out/(name+'.stderr')).write_bytes(res.stderr);commands.append({'orden':args,'inicio_utc':begin,'exit_code':res.returncode,'stdout':name+'.stdout','stderr':name+'.stderr'});(out/'ORDENES.json').write_text(json.dumps(commands,indent=2)+'\n');return res
for tool in ('rustc','cargo'):assert run([tool,'--version'],tool).returncode==0
assert run(['cargo','build','--locked','--offline','--manifest-path',str(W/'proyecto/Cargo.toml')],'build').returncode==0
assert run(['cargo','test','--locked','--offline','--lib','--manifest-path',str(W/'proyecto/Cargo.toml')],'construction').returncode==0
target=W/'proyecto/target/debug';deps=target/'deps';core=next(deps.glob('libsv_core-*.rlib'))
results=[]
for spec in json.loads((D/'sondas/ESPERADOS.json').read_text()):
 id=spec['id'];r=run(['rustc','--edition=2021','--crate-type=lib','--error-format=json','--extern','sv_bis_i0205='+str(target/'libsv_bis_i0205.rlib'),'--extern','sv_core='+str(core),'-L','dependency='+str(deps),str(D/'sondas'/f'{id}.rs'),'-o',str(out/(id+'.rlib'))],id)
 diagnostics=[json.loads(l) for l in r.stderr.decode().splitlines() if l.startswith('{')];codes={x['code']['code'] for x in diagnostics if x.get('level')=='error' and x.get('code')};ok=(r.returncode==0) if spec['compila'] else (r.returncode!=0 and codes=={spec['codigo']});results.append({'id':id,'conforme':ok,'exit_code':r.returncode,'codigos':sorted(codes)})
(out/'ENCAPSULACION.json').write_text(json.dumps(results,indent=2)+'\n')
r=run(['/usr/bin/time','-v','-o',str(out/'RECURSOS.txt'),str(target/'sv_bis_i0205'),str(W/'entradas'),str(out/'resultados')],'campana')
(out/'CORTE.json').write_text(json.dumps({'precompromiso_publico':pub['commit'],'precompromiso_laboratorio':lab['commit'],'ejecutable_sha256':hashlib.sha256((target/'sv_bis_i0205').read_bytes()).hexdigest(),'sondas_conformes':sum(x['conforme'] for x in results),'exit_campana':r.returncode},indent=2)+'\n')
print(json.dumps(results,indent=2));print(r.stdout.decode());print(r.stderr.decode());print('Campaña exit:',r.returncode)
