from pathlib import Path
import sys,json,hashlib,subprocess
sys.path.insert(0,'manifiesto-sv');from publicar_archivos import publicar
R=Path('manifiesto-sv/checkout');E=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis04-extension-perfiles-recursos-v0_1'
for p,h in json.loads((E/'MANIFIESTO.json').read_text()).items():assert hashlib.sha256((E/p).read_bytes()).hexdigest()==h
files={str(p.relative_to(R)):str(p) for p in sorted(E.rglob('*')) if p.is_file()}
for p in subprocess.check_output(['git','diff','--name-only'],cwd=R,text=True).splitlines():assert p.startswith('docs/calidad/');files[p]=str(R/p)
lab={}
for p,v in files.items():
 if 'REGISTRO_EVOLUCION_TECNICA_PROYECTO.' in p:continue
 q=('laboratorio/tareas-watson/sucesos-sv/'+Path(p).name) if '/Inventario-sv/sucesos/' in p else p.replace('docs/calidad/','laboratorio/tareas-watson/',1);lab[q]=v
print('Archivos públicos:',len(files),'; laboratorio:',len(lab),flush=True)
publicar('juantoniolloretegea/SV-matematica-semantica-cuaternaria','lab/playground-sv-permanente',lab,'test: extension nativa ES EN cuotas captura y evidencia R01; RETP-221','bis-extension/PUBLICACION_LAB.json',expected_base=json.loads(Path('bis-extension/R01_LAB.json').read_text())['commit'])
publicar('juantoniolloretegea/SV-lenguaje-de-computacion','main',files,'test: extension Rust perfiles recursos captura; RETP-221','bis-extension/PUBLICACION_PUBLICO.json',expected_base=json.loads(Path('bis-extension/R01_PUBLICO.json').read_text())['commit'])
