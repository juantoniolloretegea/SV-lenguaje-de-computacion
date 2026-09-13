from pathlib import Path
import sys,subprocess,json,hashlib
sys.path.insert(0,'manifiesto-sv');from publicar_archivos import publicar
R=Path('manifiesto-sv/checkout');D=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis-c11'
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
publicar('juantoniolloretegea/SV-matematica-semantica-cuaternaria','lab/playground-sv-permanente',lab,'docs: BIS-C11 presupuestos y recursos; RETP-214','bis-c11/PUBLICACION_LAB.json',expected_base='9da7f6aba6ff72c9a73cd12d24acda4e8e5cc659')
publicar('juantoniolloretegea/SV-lenguaje-de-computacion','main',files,'docs: BIS-C11 contrato y veinte escenarios previos; RETP-214','bis-c11/PUBLICACION_PUBLICO.json',expected_base='b22d03cf369bbe9cab902846d061d5de46f76397')
