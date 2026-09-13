from pathlib import Path
import sys,json,hashlib
sys.path.insert(0,'manifiesto-sv');from publicar_archivos import publicar
R=Path('manifiesto-sv/checkout');D=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis04-realizacion-i0205-v0_1'
for p,h in json.loads((D/'PRECOMPROMISO.json').read_text()).items():assert hashlib.sha256((D/p).read_bytes()).hexdigest()==h
files={str(p.relative_to(R)):str(p) for p in sorted(D.rglob('*')) if p.is_file()}
lab={p.replace('docs/calidad/','laboratorio/tareas-watson/',1):v for p,v in files.items()}
publicar('juantoniolloretegea/SV-matematica-semantica-cuaternaria','lab/playground-sv-permanente',lab,'test: precompromiso fuentes y sondas de realizacion I0205','bis-realizacion/PREVIO_LAB.json',expected_base='4705ba49c917cb525eeeba4b39f16686afe1e255')
publicar('juantoniolloretegea/SV-lenguaje-de-computacion','main',files,'test: precompromiso fuentes y sondas de realizacion I0205','bis-realizacion/PREVIO_PUBLICO.json',expected_base='2067310b30e5aee76120444f0166b01390a04970')
