# Reproducción de RETP-150

Requiere Python 3 como conductor de procesos y `rustc 1.98.0` nativo x86_64 Linux con su biblioteca estándar. Python no evalúa la semántica del SV. El directorio de resultados debe ser nuevo.

Extraiga la cápsula de fuentes desde esta carpeta:

```python
from pathlib import Path, PurePosixPath
import json, base64, hashlib
root = Path('fuentes-extraidas')
root.mkdir(exist_ok=False)
for item in json.loads(Path('FUENTES_REPRODUCIBLES.json').read_text())['archivos']:
    relative = PurePosixPath(item['ruta'])
    assert not relative.is_absolute() and '..' not in relative.parts
    data = base64.b64decode(item['base64'], validate=True)
    assert len(data) == item['bytes']
    assert hashlib.sha256(data).hexdigest() == item['sha256']
    target = root / relative
    target.parent.mkdir(parents=True, exist_ok=True)
    with target.open('xb') as stream:
        stream.write(data)
```

Ejecute:

```bash
python ejecutar.py /ruta/al/rustc fuentes-extraidas resultado-nuevo
```

El conductor fija las fuentes antes de compilar y las coteja al terminar. Genera las propuestas públicas de formato, prueba Rust en debug/release y conserva salidas, códigos y huellas. Se detiene ante un resultado no esperado; no corrige el programa ni ajusta los esperados. La evidencia original contiene una matriz de 47 procesos, con cuatro rechazos de compilación previstos y cero correcciones causales.

`CAMBIO_CANDIDATO.patch` se compara con la candidata RETP-149, no con el núcleo productivo. No se aplica automáticamente. `FUENTES_REPRODUCIBLES.json` contiene todas las dependencias necesarias del ensayo. Las nuevas pruebas genéricas del reconocedor son testigos sintácticos, no estados constituidos de dominio.
