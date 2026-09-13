from pathlib import Path
import sys
sys.path.insert(0,'manifiesto-sv');from publicar_archivos import publicar
R=Path('manifiesto-sv/checkout');D=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis04-realizacion-i0205-v0_1'
files={str((D/p).relative_to(R)):str(D/p) for p in ['proyecto/src/lib.rs','ADENDA_R01_PREVIA.json','evidencias/lib-campana-01.rs']}
publicar('juantoniolloretegea/SV-matematica-semantica-cuaternaria','lab/playground-sv-permanente',{p.replace('docs/calidad/','laboratorio/tareas-watson/',1):v for p,v in files.items()},'fix: acotar lectura al saldo agregado; sonda previa R01','bis-realizacion/R01_LAB.json',expected_base='ce31385d525c2ed9548c33a10aea208cafc45046')
publicar('juantoniolloretegea/SV-lenguaje-de-computacion','main',files,'fix: acotar lectura al saldo agregado; sonda previa R01','bis-realizacion/R01_PUBLICO.json',expected_base='2bce84a27f81b92a78ba71237a83de1f67895444')
