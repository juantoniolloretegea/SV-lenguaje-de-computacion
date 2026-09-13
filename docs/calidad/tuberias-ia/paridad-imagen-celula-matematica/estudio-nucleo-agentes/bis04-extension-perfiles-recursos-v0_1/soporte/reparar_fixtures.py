# ES: Variantes nuevas ante colisión léxica; conserva íntegro el banco anterior.
# EN: New variants for a lexical collision; preserves the previous bank unchanged.
from pathlib import Path
import json,hashlib,tarfile,io,gzip,shutil,re
R=Path('manifiesto-sv/checkout');B=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes';E=B/'bis04-extension-perfiles-recursos-v0_1';W=Path('bis-extension/montaje-r01');shutil.copytree(Path('bis-extension/montaje/entradas'),W/'entradas');inp=W/'entradas'
def h(b):return hashlib.sha256(b).hexdigest()
def dump(p,x):p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
def archive(root,dest,prefix):
 m=io.BytesIO()
 with tarfile.open(fileobj=m,mode='w') as t:
  for p in sorted(root.rglob('*')):
   if p.is_file():
    b=p.read_bytes();info=tarfile.TarInfo(prefix+'/'+str(p.relative_to(root)));info.size=len(b);info.mode=0o644;t.addfile(info,io.BytesIO(b))
 dest.write_bytes(gzip.compress(m.getvalue(),mtime=0))
first=Path('bis-extension/reproduccion-01/ejecucion');archive(first,E/'EVIDENCIA_CAMPANA_01.tar.gz','campana-01');dump(E/'HUELLAS_CAMPANA_01.json',{str(p.relative_to(first)):h(p.read_bytes()) for p in sorted(first.rglob('*')) if p.is_file()})
assert json.loads((first/'resultados/RESUMEN.json').read_text())['fallos']==3
pairs=json.loads((inp/'PARIDAD_PREVIA.json').read_text());changes=[]
for p in sorted((inp/'paridad').glob('*.svp')):
 b=p.read_bytes();new=re.sub(rb'\bCelda\b',b'CeldaPrueba',b);assert new!=b
 dest=inp/'paridad-r01'/p.name;dest.parent.mkdir(exist_ok=True);dest.write_bytes(new);changes.append({'original':str(p.relative_to(inp)),'sha256_original':h(b),'variante':str(dest.relative_to(inp)),'sha256_variante':h(new),'cambio':'Identificador Celda → CeldaPrueba; dos ocurrencias, sin cambiar datos, orden, dimensiones ni operación.'});assert b.count(b'Celda')==2
for pair in pairs:
 if pair['id'] in ('PAR03','PAR04','PAR05'):
  pair['id']+='-R01';pair['a']=pair['a'].replace('paridad/','paridad-r01/');pair['b']=pair['b'].replace('paridad/','paridad-r01/')
dump(inp/'PARIDAD_PREVIA.json',pairs);dump(E/'PARIDAD_R01_PREVIA.json',pairs)
archive(inp,E/'ENTRADAS_R01.tar.gz','entradas');dump(E/'ENTRADAS_R01_MATERIALIZADAS.json',{str(p.relative_to(inp)):h(p.read_bytes()) for p in sorted(inp.rglob('*')) if p.is_file()})
dump(E/'ADENDA_R01_PREVIA.json',{'causa':'Los fixtures C12 usan Celda, palabra protegida de la unión de perfiles. Ambas compilaciones fallan antes de emitir IR. Defecto del estímulo; no evidencia de desigualdad semántica.','evidencia':'EVIDENCIA_CAMPANA_01.tar.gz','estado':'VARIANTES_NUEVAS_PENDIENTES_DE_EJECUCION','cambios':changes,'oraculos':'Cuatro equivalencias y una desigualdad siguen exigidas. PAR01/02 y los 26 casos integrados permanecen idénticos. PAR03/04/05 históricos conservan su fracaso; nuevas identidades -R01.','nucleo_y_admision':'Sin modificaciones','documentacion_futura':'Trasladar la colisión del identificador Celda al retorno de errores/documentación; no marcar como ejecutadas las 202 filas originales.'})
shutil.copyfile(E/'soporte/reproducir.py',E/'soporte/reproducir-campana-01.py')
p=E/'soporte/reproducir.py';s=p.read_text().replace("'ENTRADAS.tar.gz'","'ENTRADAS_R01.tar.gz'").replace("'ENTRADAS_MATERIALIZADAS.json'","'ENTRADAS_R01_MATERIALIZADAS.json'");p.write_text(s)
shutil.copyfile('bis-extension/reparar_fixtures.py',E/'soporte/reparar_fixtures.py')
dump(E/'PRECOMPROMISO_R01.json',{str(p.relative_to(E)):h(p.read_bytes()) for p in [E/'PARIDAD_R01_PREVIA.json',E/'ENTRADAS_R01.tar.gz',E/'ENTRADAS_R01_MATERIALIZADAS.json',E/'ADENDA_R01_PREVIA.json',E/'soporte/reproducir.py',E/'proyecto/src/main.rs',E/'proyecto/src/observer.rs']})
print('Ejecución fallida conservada; tres variantes R01 pendientes.')
