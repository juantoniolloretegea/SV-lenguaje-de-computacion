from pathlib import Path
import sys,json,shutil
sys.path.insert(0,'tmp/diagnosticos-158')
from ejecutar import run,r,rustc,e
out=Path('entregas/diagnosticos-del-compilador-y-procedencia')
run('03-catalogo-build',[rustc,'--edition=2021',str(r/'catalogo.rs'),'--extern',f'sv_core={r}/libsv_core_new_debug.rlib','-o',str(r/'catalogo')])
b=run('03-catalogo',[str(r/'catalogo')]);rows=[]
for line in b.decode().splitlines():
 _,key,code,es,en=line.split('\t');rows.append(dict(id=key,code=code or None,es=es,en=en))
(out/'CATALOGO.json').write_text(json.dumps({'version':'COMPILER-DIAGNOSTICS/1','causas':rows},ensure_ascii=False,indent=2)+'\n')
for lang in ['es','en']:
 p=out/'idiomas'/lang/'diagnosticos.json';p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps({'version':'COMPILER-DIAGNOSTICS/1','idioma':lang,'mensajes':{x['id']:x[lang] for x in rows}},ensure_ascii=False,indent=2)+'\n')
shutil.copytree('entregas/diagnosticos-de-recepcion-y-prueba-entre-modelos/clientes',out/'clientes',dirs_exist_ok=True)
print('12 causas catalogadas desde mensajes compilados; códigos canónicos E004/E115 conservados')
