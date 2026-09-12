from pathlib import Path
from datetime import datetime,timezone
import json,hashlib,ast,shutil
R=Path(__file__).resolve().parent;D=R/'entrega';J=lambda p,x:p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n');sha=lambda b:hashlib.sha256(b).hexdigest()
for p in [D/'reproducir.py',R/'registrar.py',R/'publicar.py']:ast.parse(p.read_text())
assert not (D/'FIJACION_PREVIA.json').exists()
(D/'README.md').write_text('''# S15 — Causas de recepción y no admisión

Apertura anterior a compilación. [Contrato](CONTRATO_S15.md), [código](codigo/receptor.rs), [conductor](codigo/contraste.rs), [esperados](codigo/ESPERADO.tsv), [procedencia](PROCEDENCIA.json), [rectores](RECTORES.json), [cápsula anterior](FUENTES_S2.json) y [fijación](FIJACION_PREVIA.json).

Resultado pendiente. Catorce casos de laboratorio conectados a cobertura S2 intacta, seis ejecuciones y tres sensibilidades; máximo 21 invocaciones. No constituye integración con un proveedor real ni promoción nuclear.

Reproducción Linux x86_64 con Rust 1.98.0 fijado:

```sh
python3 reproducir.py --rustc /ruta/absoluta/rustc --salida /ruta/nueva/s15
```

Esta apertura conservará sus bytes. El acta de resultado, si se produce, se incorporará como documento posterior.
''')
J(D/'PREPARACION.json',dict(version='S15-PREPARACION/1',compilacion_previa=False,ejecucion_funcional_previa=False,incidencias=[],alcance='Lectura de fuentes, diseño y análisis sintáctico Python; ninguna cualificación funcional de la implementación nueva aún.'))
A=D/'administracion';A.mkdir(exist_ok=True)
for n in ['preparar.py','configurar.py','fijar.py','registrar.py','publicar.py','snapshot.py']:shutil.copyfile(R/n,A/n)
J(D/'FIJACION_PREVIA.json',dict(version='S15-FIJACION/1',fecha_utc=datetime.now(timezone.utc).isoformat(),archivos={str(p.relative_to(D)):sha(p.read_bytes()) for p in sorted(D.rglob('*')) if p.is_file()}))
print('Fijación previa',sha((D/'FIJACION_PREVIA.json').read_bytes()))
