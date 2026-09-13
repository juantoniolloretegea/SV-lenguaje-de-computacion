from pathlib import Path
import sys,subprocess,json,hashlib
sys.path.insert(0,'manifiesto-sv');from publicar_archivos import publicar
R=Path('manifiesto-sv/checkout');D=R/'docs/calidad/tuberias-ia/recuperacion-rust-s25'
m=json.loads((D/'MANIFIESTO.json').read_text())
for p,h in m.items():assert hashlib.sha256((D/p).read_bytes()).hexdigest()==h,p
files={str(p.relative_to(R)):str(p) for p in sorted(D.rglob('*')) if p.is_file()}
for p in subprocess.check_output(['git','diff','--name-only'],cwd=R,text=True).splitlines():
 assert p.startswith('docs/calidad/');files[p]=str(R/p)
lab={}
for p,v in files.items():
 if 'REGISTRO_EVOLUCION_TECNICA_PROYECTO.' in p:continue
 q=('laboratorio/tareas-watson/sucesos-sv/'+Path(p).name) if '/Inventario-sv/sucesos/' in p else p.replace('docs/calidad/','laboratorio/tareas-watson/',1);lab[q]=v
print('Archivos a publicar:',len(files),'; espejo:',len(lab),flush=True)
publicar('juantoniolloretegea/SV-matematica-semantica-cuaternaria','lab/playground-sv-permanente',lab,'docs: S25 Rust 1.98.0 y continuidad nativa; RETP-217','rust-env-s25/PUBLICACION_LAB.json',expected_base='389ecbaf34a2274455d81da85bc46bbab675816d')
publicar('juantoniolloretegea/SV-lenguaje-de-computacion','main',files,'docs: S25 entorno Rust y repetición C01; RETP-217','rust-env-s25/PUBLICACION_PUBLICO.json',expected_base='52fc506a836821b7285834bdc0adf93e9ddc5680')
