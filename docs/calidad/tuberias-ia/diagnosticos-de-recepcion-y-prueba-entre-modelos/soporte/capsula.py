from pathlib import Path
import json,base64,hashlib,difflib
r=Path('tmp/continuacion-157');out=Path('entregas/diagnosticos-de-recepcion-y-prueba-entre-modelos')
files=[];patch=[];base=Path('tmp/admision-154/candidata')
for p in sorted((r/'candidata').rglob('*')):
 if not p.is_file():continue
 rel=str(p.relative_to(r/'candidata'));b=p.read_bytes();files.append(dict(ruta=rel,sha256=hashlib.sha256(b).hexdigest(),base64=base64.b64encode(b).decode()))
 before=(base/rel).read_text() if (base/rel).exists() else ''
 if b.decode()!=before:patch.extend(difflib.unified_diff(before.splitlines(True),b.decode().splitlines(True),fromfile='a/'+rel,tofile='b/'+rel))
(out/'CANDIDATA_FUENTES.json').write_text(json.dumps({'version':'RETP-157','base':'RETP-154','archivos':files},ensure_ascii=False,indent=2)+'\n')
(out/'CAMBIO_INCREMENTAL.patch').write_text(''.join(patch))
print(len(files),'fuentes conservadas')
