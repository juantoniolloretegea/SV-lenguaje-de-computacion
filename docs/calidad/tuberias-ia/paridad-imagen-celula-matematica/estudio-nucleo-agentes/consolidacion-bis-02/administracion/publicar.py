from pathlib import Path
import sys,subprocess,json,hashlib
sys.path.insert(0,'manifiesto-sv');from publicar_archivos import publicar
R=Path('manifiesto-sv/checkout');D=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/consolidacion-bis-02'
m=json.loads((D/'MANIFIESTO_PREVIO.json').read_text())
for p,h in m['archivos_sha256'].items():assert hashlib.sha256((D/p).read_bytes()).hexdigest()==h,p
files={str(p.relative_to(R)):str(p) for p in sorted(D.rglob('*')) if p.is_file()}
for p in subprocess.check_output(['git','diff','--name-only'],cwd=R,text=True).splitlines():
 assert p.startswith('docs/calidad/');files[p]=str(R/p)
lab={}
for p,v in files.items():
 if 'REGISTRO_EVOLUCION_TECNICA_PROYECTO.' in p:continue
 q=('laboratorio/tareas-watson/sucesos-sv/'+Path(p).name) if '/Inventario-sv/sucesos/' in p else p.replace('docs/calidad/','laboratorio/tareas-watson/',1);lab[q]=v
print('Archivos a publicar:',len(files),'; espejo:',len(lab),flush=True)
publicar('juantoniolloretegea/SV-matematica-semantica-cuaternaria','lab/playground-sv-permanente',lab,'docs: BIS-02 consolidación C01-C12; RETP-216','bis-consolidacion-02/PUBLICACION_LAB.json',expected_base='5dc375e7d7af3e1110bb57a9b4e7449eb817dac6')
publicar('juantoniolloretegea/SV-lenguaje-de-computacion','main',files,'docs: BIS-02 cobertura y pendientes; RETP-216','bis-consolidacion-02/PUBLICACION_PUBLICO.json',expected_base='4fc7a2ceb2ece6d5d69d12e3216eda48e68f6136')
