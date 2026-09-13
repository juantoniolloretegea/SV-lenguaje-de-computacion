from pathlib import Path
import sys,json,hashlib
sys.path.insert(0,'manifiesto-sv');from publicar_archivos import publicar
R=Path('manifiesto-sv/checkout');E=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis04-extension-perfiles-recursos-v0_1'
for p,h in json.loads((E/'PRECOMPROMISO_R01.json').read_text()).items():assert hashlib.sha256((E/p).read_bytes()).hexdigest()==h
files={str(p.relative_to(R)):str(p) for p in sorted(E.rglob('*')) if p.is_file()};lab={p.replace('docs/calidad/','laboratorio/tareas-watson/',1):v for p,v in files.items()}
publicar('juantoniolloretegea/SV-matematica-semantica-cuaternaria','lab/playground-sv-permanente',lab,'test: conservar fallo lexico C12 y fijar variantes de paridad R01','bis-extension/R01_LAB.json',expected_base=json.loads(Path('bis-extension/PREVIO_LAB.json').read_text())['commit'])
publicar('juantoniolloretegea/SV-lenguaje-de-computacion','main',files,'test: variantes R01 ante identificador protegido Celda; sin alterar oraculos','bis-extension/R01_PUBLICO.json',expected_base=json.loads(Path('bis-extension/PREVIO_PUBLICO.json').read_text())['commit'])
