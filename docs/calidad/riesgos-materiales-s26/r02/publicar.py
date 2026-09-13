"""Publicar exclusivamente la recepción documental S26 R02 y su espejo.
Uso desde checkout: python -B <este_script> /directorio/comprobantes
Conector GitHub; blobs/árbol/commit; cotejo de base; update_ref sin force.
"""
from pathlib import Path
import json
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'soporte'))
from publicar_archivos import publicar

R = Path.cwd()
E = R / 'docs/calidad/riesgos-materiales-s26'
C = Path(sys.argv[1])
C.mkdir(parents=True, exist_ok=True)
report = json.loads((E / 'r02/VERIFICACION.json').read_text())
assert report['conforme'] and report['nuevos_contrastes_ejecutados'] == 0
names = list(report['archivos_editados']) + [
    'docs/calidad/riesgos-materiales-s26/RECEPCION_CONTRACTUAL_R02.md',
    'docs/calidad/riesgos-materiales-s26/r02/registrar.py',
    'docs/calidad/riesgos-materiales-s26/r02/publicar.py',
    'docs/calidad/riesgos-materiales-s26/r02/VERIFICACION.json']
files = {p: str(R / p) for p in names}
lab = {}
for p, v in files.items():
    if 'REGISTRO_EVOLUCION_TECNICA_PROYECTO.' in p:
        continue
    if '/Inventario-sv/sucesos/' in p:
        q = 'laboratorio/tareas-watson/sucesos-sv/' + Path(p).name
    elif p.startswith('docs/arquitectura/'):
        q = p.replace('docs/arquitectura/', 'laboratorio/tareas-watson/arquitectura/', 1)
    else:
        q = p.replace('docs/calidad/', 'laboratorio/tareas-watson/', 1)
    lab[q] = v
b = publicar('juantoniolloretegea/SV-matematica-semantica-cuaternaria',
             'lab/playground-sv-permanente', lab, 'docs: S26 recepción contractual de custodia y lectura R02',
             C / 'laboratorio.json', expected_base=report['corte_laboratorio'])
a = publicar('juantoniolloretegea/SV-lenguaje-de-computacion', 'main', files,
             'docs: S26 recepción contractual de custodia y lectura R02',
             C / 'publico.json', expected_base=report['corte_lenguaje'])
reverse = {v: p for p, v in lab.items()}
for p, v in files.items():
    if v in reverse:
        assert a['files'][p] == b['files'][reverse[v]], p
assert a['verified'] and b['verified']
print(json.dumps(dict(publico=a['commit'], laboratorio=b['commit'], espejo='verificado'), ensure_ascii=False))
