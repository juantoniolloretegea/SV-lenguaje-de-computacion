# ES: Materialización administrativa exacta; no interpreta ni valida SVP.
# EN: Exact administrative materialization; does not interpret or validate SVP.
from pathlib import Path
import json,hashlib,shutil,tarfile,sys
R=Path('manifiesto-sv/checkout');B=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes';D=B/'bis04-realizacion-i0205-v0_1';I=B/'integracion-c02-c05-v0_1';S=B/'bis03-sedes-i0205-v0_1';W=Path('bis-realizacion/montaje');W.mkdir(exist_ok=True)
def dump(p,x):p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
for p in (I,S):
 for name,h in json.loads((p/'MANIFIESTO.json').read_text()).items():assert hashlib.sha256((p/name).read_bytes()).hexdigest()==h
for name in ('lib.rs','json.rs','main.rs','observer.rs'):shutil.copyfile(Path('bis-realizacion')/name,D/'proyecto/src'/name)
shutil.copytree(D/'proyecto',W/'proyecto',dirs_exist_ok=True)
with tarfile.open(D/'sv_core-verificado.tar.gz') as t:t.extractall(W,filter='data')
for p,h in json.loads((D/'NUCLEO_FUENTES.json').read_text())['archivos'].items():assert hashlib.sha256((W/'sv_core'/p).read_bytes()).hexdigest()==h
inp=W/'entradas';inp.mkdir(exist_ok=True)
reg=json.loads((I/'REGISTRO_CONFIABLES.json').read_text()) if (I/'REGISTRO_CONFIABLES.json').exists() else json.loads((I/'REGISTRO_CONFIABLE.json').read_text());dump(inp/'registry.json',reg)
for k,name in [('constitucion','constitution.bin'),('convenio','convention.bin'),('transformaciones','transforms.bin'),('estado_matematico','canonical.bin')]:shutil.copyfile(I/reg[k]['archivo'],inp/name)
req=json.loads((I/'SOLICITUDES.json').read_text());ctx=json.loads((I/'CONTEXTOS_CONFIABLES.json').read_text());plans=json.loads((I/'PLANES_DE_INYECCION.json').read_text());ora=json.loads((I/'ORACULOS.json').read_text())
for q,c,p,o in zip(req,ctx,plans,ora):
 assert q['id']==c['id']==p['id']==o['id'];dest=inp/q['id'];dest.mkdir(exist_ok=True);r=q['solicitud']
 for name,x in [('request.json',r),('context.json',c),('plan.json',p),('oracle.json',o)]:dump(dest/name,x)
 for k,name in [('fuente','source.bin'),('estado_matematico','state.bin'),('geometria','geometry.bin')]:shutil.copyfile(I/r[k]['archivo'],dest/name)
 if r.get('soporte'):shutil.copyfile(I/r['soporte']['contenido']['archivo'],dest/'support.bin')
 if p.get('buffer_capturado'):shutil.copyfile(I/p['buffer_capturado']['archivo'],dest/'injection.bin')
 if o.get('contenido_entrega_esperado'):shutil.copyfile(I/o['contenido_entrega_esperado']['archivo'],dest/'expected.bin')
for c in json.loads((S/'BANCO_ADAPTADOR_PREVIO.json').read_text())['casos']:
 dest=inp/'instrumental'/c['id'];dump(dest/'expected.json',c['esperado']);shutil.copyfile(S/c['entrada']['archivo'],dest/'input.bin')
dump(D/'ENTRADAS_MATERIALIZADAS.json',{str(p.relative_to(inp)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(inp.rglob('*')) if p.is_file()})
print('Materialización íntegra. Rust no ejecutado por este script.')
