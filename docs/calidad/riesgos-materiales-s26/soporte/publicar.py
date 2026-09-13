"""Publica sólo el alta documental y coteja los dos árboles.
Servicio: GitHub mediante HostedAppsClient/github_io preexistente.
No envía mensajes ni ejecuta ensayos; actualizaciones de rama sin force.
Ejecutar desde la raíz del checkout; argumento: directorio de comprobantes.
"""
from pathlib import Path
import sys
import hashlib
import json
import subprocess
from publicar_archivos import publicar

R=Path.cwd(); E=R/'docs/calidad/riesgos-materiales-s26'
C=Path(sys.argv[1]); C.mkdir(parents=True,exist_ok=True)
base='2fc90a08575484ce7c2af9f3fc544c2249c90c03'
assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==base
paths=['docs/calidad/Inventario-sv/sucesos/'+n for n in ('SUCESOS_SV.csv','SUCESOS_SV.md','HISTORIAL_SUCESOS_SV.csv')]
paths += ['docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.'+x for x in ('csv','md')]
paths += ['docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md','docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/ESTADO_WORKFLOW.json']
assert set(subprocess.check_output(['git','diff','--name-only'],text=True).splitlines())==set(paths)
files={p:str(R/p) for p in paths}
files.update({str(p.relative_to(R)):str(p) for p in E.rglob('*') if p.is_file() and '__pycache__' not in p.parts})
lab={}
for p,v in files.items():
    if 'REGISTRO_EVOLUCION_TECNICA_PROYECTO.' in p: continue
    q='laboratorio/tareas-watson/sucesos-sv/'+Path(p).name if '/Inventario-sv/sucesos/' in p else p.replace('docs/calidad/','laboratorio/tareas-watson/',1)
    lab[q]=v
b=publicar('juantoniolloretegea/SV-matematica-semantica-cuaternaria','lab/playground-sv-permanente',lab,'docs: alta S26 riesgos materiales y matriz previa; RETP-222',C/'laboratorio.json',expected_base='500b283729a8930f8c23033df3fe369d5370759b')
a=publicar('juantoniolloretegea/SV-lenguaje-de-computacion','main',files,'docs: S26 fallos a provocar y evidencia exigible; RETP-222',C/'lenguaje.json',expected_base=base)
for p,v in files.items():
    raw=Path(v).read_bytes(); sha=hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest()
    assert a['files'][p]==sha
    if 'REGISTRO_EVOLUCION_TECNICA_PROYECTO.' not in p:
        q='laboratorio/tareas-watson/sucesos-sv/'+Path(p).name if '/Inventario-sv/sucesos/' in p else p.replace('docs/calidad/','laboratorio/tareas-watson/',1)
        assert b['files'][q]==sha
assert a['verified'] and b['verified']
print(json.dumps({'lenguaje':a['commit'],'laboratorio':b['commit'],'archivos_publicos':len(files),'archivos_espejo':len(lab),'arboles_y_espejo':'conformes'},ensure_ascii=False))
