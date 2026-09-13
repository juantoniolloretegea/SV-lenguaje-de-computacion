from pathlib import Path
import sys,json,hashlib,subprocess
sys.path.insert(0,'manifiesto-sv');from publicar_archivos import publicar
R=Path('manifiesto-sv/checkout');D=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis04-realizacion-i0205-v0_1'
for p,h in json.loads((D/'MANIFIESTO.json').read_text()).items():assert hashlib.sha256((D/p).read_bytes()).hexdigest()==h
files={str(p.relative_to(R)):str(p) for p in sorted(D.rglob('*')) if p.is_file()}
for p in subprocess.check_output(['git','diff','--name-only'],cwd=R,text=True).splitlines():assert p.startswith('docs/calidad/');files[p]=str(R/p)
lab={}
for p,v in files.items():
 if 'REGISTRO_EVOLUCION_TECNICA_PROYECTO.' in p:continue
 q=('laboratorio/tareas-watson/sucesos-sv/'+Path(p).name) if '/Inventario-sv/sucesos/' in p else p.replace('docs/calidad/','laboratorio/tareas-watson/',1);lab[q]=v
print('Publicación final:',len(files),'archivos públicos;',len(lab),'en laboratorio.',flush=True)
publicar('juantoniolloretegea/SV-matematica-semantica-cuaternaria','lab/playground-sv-permanente',lab,'feat: realizacion nativa C02-C05 y evidencia de dos campanas; RETP-220','bis-realizacion/PUBLICACION_LAB.json',expected_base='6ece56f9684f6b4a1045d954066301d64ac96129')
publicar('juantoniolloretegea/SV-lenguaje-de-computacion','main',files,'feat: realizacion Rust I0205, encapsulacion y observador; RETP-220','bis-realizacion/PUBLICACION_PUBLICO.json',expected_base='d9cea830eacc3a0f808ba88725a514867c57e643')
