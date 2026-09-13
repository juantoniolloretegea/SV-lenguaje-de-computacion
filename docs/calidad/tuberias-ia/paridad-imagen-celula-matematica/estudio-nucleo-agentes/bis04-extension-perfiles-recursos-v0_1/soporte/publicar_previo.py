from pathlib import Path
import sys,json,hashlib
sys.path.insert(0,'manifiesto-sv');from publicar_archivos import publicar
R=Path('manifiesto-sv/checkout');E=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis04-extension-perfiles-recursos-v0_1'
for p,h in json.loads((E/'PRECOMPROMISO.json').read_text()).items():assert hashlib.sha256((E/p).read_bytes()).hexdigest()==h
files={str(p.relative_to(R)):str(p) for p in sorted(E.rglob('*')) if p.is_file()};lab={p.replace('docs/calidad/','laboratorio/tareas-watson/',1):v for p,v in files.items()}
publicar('juantoniolloretegea/SV-matematica-semantica-cuaternaria','lab/playground-sv-permanente',lab,'test: banco previo ES/EN, cuotas y captura; extension RETP-220','bis-extension/PREVIO_LAB.json',expected_base='ed5a78fb667401b052e915db38bf17465fe3b2b2')
publicar('juantoniolloretegea/SV-lenguaje-de-computacion','main',files,'test: precompromiso de extension nativa perfiles recursos captura','bis-extension/PREVIO_PUBLICO.json',expected_base='aa1229788f0a6625be43c74e29c38023b8193a4c')
