from pathlib import Path
import hashlib, json, shutil
from datetime import datetime, timezone

# ES: Conserva los archivos de inicio y añade una activación idempotente.
# EN: Preserve startup files and add idempotent activation.
base = Path(__file__).resolve().parent
target = Path('/opt/sv-rust-1.98.0/env.sh')
shutil.copyfile(base / 'env.sh', target)
line = '[ ! -r /opt/sv-rust-1.98.0/env.sh ] || . /opt/sv-rust-1.98.0/env.sh\n'
result = []
for name in ['.profile', '.bashrc']:
    p = Path.home() / name
    old = p.read_bytes() if p.exists() else b''
    # .bashrc must activate before its noninteractive early return.
    new = old if line.encode() in old else (line.encode() + old if name == '.bashrc' else old + b'\n' + line.encode())
    p.write_bytes(new)
    result.append({'archivo_inicio': name, 'antes_sha256': hashlib.sha256(old).hexdigest(), 'despues_sha256': hashlib.sha256(new).hexdigest(), 'cambio': old != new})
(base / 'CONFIGURACION.json').write_text(json.dumps({'fecha_utc': datetime.now(timezone.utc).isoformat(), 'activador': str(target), 'variables': ['SV_RUST_PREFIX', 'PATH'], 'archivos': result}, ensure_ascii=False, indent=2) + '\n')
