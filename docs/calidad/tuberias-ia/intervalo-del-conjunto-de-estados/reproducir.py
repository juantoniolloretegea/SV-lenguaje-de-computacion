"""Reproduce la candidata experimental en un directorio nuevo, sin red."""
from pathlib import Path
import sys, json, hashlib, base64, shutil, subprocess
here=Path(__file__).resolve().parent
rust=Path(sys.argv[1]).resolve();work=Path(sys.argv[2]).resolve()
work.mkdir(parents=True,exist_ok=False)
manifest=json.loads((here/'MANIFIESTO.json').read_text())
for item in manifest['archivos']:
 rel=Path(item['ruta']);assert not rel.is_absolute() and '..' not in rel.parts
 data=(here/rel).read_bytes();assert hashlib.sha256(data).hexdigest()==item['sha256'],str(rel)
 if rel.parts[0] in ['evidencia','complemento','recepcion_portatil']:continue
 p=work/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(data)
for capsule,folder in [('BASE_FUENTES.json','base'),('CANDIDATA_FUENTES.json','candidata')]:
 for item in json.loads((work/capsule).read_text())['archivos']:
  rel=Path(item['ruta']);assert not rel.is_absolute() and '..' not in rel.parts
  b=base64.b64decode(item['base64'],validate=True);assert hashlib.sha256(b).hexdigest()==item['sha256']
  p=work/folder/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(b)
subprocess.run([sys.executable,str(work/'ejecutar.py'),str(rust)],check=True)
subprocess.run([sys.executable,str(work/'complemento.py'),str(rust)],check=True)
print('REPRODUCCIÓN COMPLETA EN ALCANCE NATIVO')
