from pathlib import Path
import sys,subprocess,json,hashlib
sys.path.insert(0,'manifiesto-sv');from publicar_archivos import publicar
R=Path('manifiesto-sv/checkout');D=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis03-sedes-i0205-v0_1'
for p,h in json.loads((D/'MANIFIESTO.json').read_text()).items():assert hashlib.sha256((D/p).read_bytes()).hexdigest()==h,p
files={str(p.relative_to(R)):str(p) for p in sorted(D.rglob('*')) if p.is_file()}
for p in subprocess.check_output(['git','diff','--name-only'],cwd=R,text=True).splitlines():
 assert p.startswith('docs/calidad/');files[p]=str(R/p)
lab={}
for p,v in files.items():
 if 'REGISTRO_EVOLUCION_TECNICA_PROYECTO.' in p:continue
 q=('laboratorio/tareas-watson/sucesos-sv/'+Path(p).name) if '/Inventario-sv/sucesos/' in p else p.replace('docs/calidad/','laboratorio/tareas-watson/',1);lab[q]=v
print('Archivos público:',len(files),'; laboratorio:',len(lab),flush=True)
publicar('juantoniolloretegea/SV-matematica-semantica-cuaternaria','lab/playground-sv-permanente',lab,'docs: S22 sedes e interfaz C02-C05; RETP-219','bis-sedes/PUBLICACION_LAB.json',expected_base='e1779bdfc53ab815e7b3dc2c24914c3ecdafc3a2')
publicar('juantoniolloretegea/SV-lenguaje-de-computacion','main',files,'docs: decision de sedes y banco instrumental I0205; RETP-219','bis-sedes/PUBLICACION_PUBLICO.json',expected_base='3c4f5f87e2e052daacb25be4a4dfe7c1db0d946d')
