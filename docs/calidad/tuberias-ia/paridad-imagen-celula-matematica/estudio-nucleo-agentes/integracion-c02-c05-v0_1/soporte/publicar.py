from pathlib import Path
import sys,subprocess,json,hashlib
sys.path.insert(0,'manifiesto-sv');from publicar_archivos import publicar
R=Path('manifiesto-sv/checkout');D=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/integracion-c02-c05-v0_1'
for p,h in json.loads((D/'MANIFIESTO.json').read_text()).items():assert hashlib.sha256((D/p).read_bytes()).hexdigest()==h,p
files={str(p.relative_to(R)):str(p) for p in sorted(D.rglob('*')) if p.is_file()}
for p in subprocess.check_output(['git','diff','--name-only'],cwd=R,text=True).splitlines():
 assert p.startswith('docs/calidad/');files[p]=str(R/p)
lab={}
for p,v in files.items():
 if 'REGISTRO_EVOLUCION_TECNICA_PROYECTO.' in p:continue
 q=('laboratorio/tareas-watson/sucesos-sv/'+Path(p).name) if '/Inventario-sv/sucesos/' in p else p.replace('docs/calidad/','laboratorio/tareas-watson/',1);lab[q]=v
print('Archivos público:',len(files),'; laboratorio:',len(lab),flush=True)
publicar('juantoniolloretegea/SV-matematica-semantica-cuaternaria','lab/playground-sv-permanente',lab,'docs: S22 compromiso integrado C02-C05; RETP-218','bis-integracion/PUBLICACION_LAB.json',expected_base='3678ae6c2881d74b2d46da99c33775d6a2979108')
publicar('juantoniolloretegea/SV-lenguaje-de-computacion','main',files,'docs: contrato y 26 casos integrados C02-C05; RETP-218','bis-integracion/PUBLICACION_PUBLICO.json',expected_base='18963fa275d2a29633e164d9a2ea5fe1c4f11216')
