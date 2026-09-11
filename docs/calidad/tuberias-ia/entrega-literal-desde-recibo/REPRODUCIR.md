# Reproducción de RETP-149

La cápsula `FUENTES_REPRODUCIBLES.json` contiene la candidata completa de ensayo y sus dependencias públicas fijadas, incluidas las pruebas anteriores. No se aplica automáticamente sobre el núcleo productivo. `CAMBIO_CANDIDATO.patch` muestra las seis piezas nuevas o modificadas respecto del corte indicado en `FUENTES.json`.

Requiere Python 3 y `rustc 1.98.0` nativo x86_64 Linux con su biblioteca estándar. La procedencia del compilador usado está en `COMPILADOR.json`; las descargas oficiales se cotejaron con las huellas del manifiesto. Python extrae archivos, lanza procesos Rust y coteja resultados; no interpreta semántica SV.

Desde esta carpeta, extraiga las fuentes en una carpeta nueva:

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

Después:

```bash
python ejecutar.py /ruta/al/rustc fuentes-extraidas resultado-nuevo
```

El directorio de resultados debe ser nuevo. El conductor fija las huellas de las fuentes antes de compilar y las vuelve a cotejar al terminar. Se detiene ante un resultado no esperado. No repara fuentes ni cambia esperados. Conserva salidas y códigos de proceso incluso en caso de fallo. Una reproducción independiente es un ensayo nuevo: no reemplaza la evidencia original conservada.

La evidencia de esta entrega contiene **una matriz ejecutada, 31 procesos, cero correcciones causales**. Los dos fallos de compilación de clientes externos son negativos esperados: forja de campos privados y uso de una vista después de destruir su custodio.
