"""Publicación acotada S26 R05, laboratorio y espejo; sin force.
Uso desde checkout: publicar.py previo|resultado /directorio/comprobantes
"""
from pathlib import Path
import sys,json,hashlib
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'soporte'))
from publicar_archivos import publicar
R=Path.cwd();E=R/'docs/calidad/riesgos-materiales-s26';stage=sys.argv[1];C=Path(sys.argv[2]);C.mkdir(parents=True,exist_ok=True)
assert stage in ('previo','resultado')
files={str(p.relative_to(R)):str(p) for p in (E/'r05').rglob('*') if p.is_file() and '__pycache__' not in p.parts}
for n in ['Inventario-sv/sucesos/SUCESOS_SV.csv','Inventario-sv/sucesos/SUCESOS_SV.md','Inventario-sv/sucesos/HISTORIAL_SUCESOS_SV.csv','REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv','REGISTRO_EVOLUCION_TECNICA_PROYECTO.md','tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md','tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/MANIFIESTO.json','tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/ESTADO_WORKFLOW.json']:
    files['docs/calidad/'+n]=str(R/'docs/calidad'/n)
files['docs/calidad/riesgos-materiales-s26/README.md']=str(E/'README.md')
lab={}
for p,v in files.items():
    if 'REGISTRO_EVOLUCION_TECNICA_PROYECTO.' in p:continue
    q='laboratorio/tareas-watson/sucesos-sv/'+Path(p).name if '/Inventario-sv/sucesos/' in p else p.replace('docs/calidad/','laboratorio/tareas-watson/',1)
    lab[q]=v
if stage=='previo':pb='32410645663ea198516803afb88cf5195b584676';lb='58de66b3dfba33a934da535ab341d66b3ca274c9'
else:
    pb=json.loads((C/'previo-publico.json').read_text())['commit'];lb=json.loads((C/'previo-laboratorio.json').read_text())['commit']
b=publicar('juantoniolloretegea/SV-matematica-semantica-cuaternaria','lab/playground-sv-permanente',lab,'docs: S26 sondas custodia lectura R05 '+stage,C/(stage+'-laboratorio.json'),expected_base=lb)
a=publicar('juantoniolloretegea/SV-lenguaje-de-computacion','main',files,'docs: S26 sondas custodia lectura R05 '+stage,C/(stage+'-publico.json'),expected_base=pb)
for p,v in files.items():
    raw=Path(v).read_bytes();sha=hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest();assert a['files'][p]==sha
    if 'REGISTRO_EVOLUCION_TECNICA_PROYECTO.' in p:continue
    q='laboratorio/tareas-watson/sucesos-sv/'+Path(p).name if '/Inventario-sv/sucesos/' in p else p.replace('docs/calidad/','laboratorio/tareas-watson/',1)
    assert b['files'][q]==sha
assert a['verified'] and b['verified']
print(json.dumps({'fase':stage,'publico':a['commit'],'laboratorio':b['commit'],'espejo':'verificado'},ensure_ascii=False))
