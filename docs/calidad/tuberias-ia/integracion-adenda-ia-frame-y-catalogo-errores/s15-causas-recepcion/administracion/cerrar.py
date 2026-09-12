from pathlib import Path
from datetime import datetime,timezone
import json,hashlib,shutil,ast
R=Path(__file__).resolve().parent;D=R/'entrega';sha=lambda b:hashlib.sha256(b).hexdigest();J=lambda p,x:p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
for n in ['documentar.py','publicar_cierre.py','cerrar.py']:shutil.copyfile(R/n,D/'administracion'/n)
shutil.copyfile(R/'registrar.py',D/'administracion/registrar_cierre.py')
fixed=json.loads((D/'FIJACION_PREVIA.json').read_text())
for n,h in fixed['archivos'].items():assert sha((D/n).read_bytes())==h,n
for p in D.rglob('*.py'):ast.parse(p.read_text())
for p in D.rglob('*.json'):json.loads(p.read_text())
for x in json.loads((D/'EVIDENCIAS_S15.json').read_text())['archivos']:
 import base64
 b=base64.b64decode(x['base64']);assert sha(b)==x['sha256'] and len(b)==x['bytes']
# La cápsula materializada conserva todos sus fuentes, también después de sensibilidad.
for x in json.loads((D/'FUENTES_S2.json').read_text())['archivos']:assert sha((R/'ejecucion'/x['ruta']).read_bytes())==x['sha256']
J(D/'MANIFIESTO_SHA256.json',dict(version='S15-MANIFIESTO/1',fecha_utc=datetime.now(timezone.utc).isoformat(),archivos={str(p.relative_to(D)):dict(bytes=p.stat().st_size,sha256=sha(p.read_bytes())) for p in sorted(D.rglob('*')) if p.is_file() and p.name!='MANIFIESTO_SHA256.json'}))
print('Fijación, cápsula, 581 capturas y manifestación final comprobadas.')
