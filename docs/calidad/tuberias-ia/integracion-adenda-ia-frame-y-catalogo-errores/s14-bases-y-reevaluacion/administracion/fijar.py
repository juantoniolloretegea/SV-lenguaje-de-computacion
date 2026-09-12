from pathlib import Path
from datetime import datetime,timezone
import json,hashlib,ast,shutil
R=Path(__file__).resolve().parent;D=R/'entrega';sha=lambda b:hashlib.sha256(b).hexdigest();J=lambda p,x:p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n')
ast.parse((D/'reproducir.py').read_text());assert not (D/'FIJACION_PREVIA.json').exists()
(D/'README.md').write_text('''# S14 — Bases documentales y recuperación/reevaluación

Apertura fijada antes de compilar. [Contrato](CONTRATO_S14.md), [procedencia](PROCEDENCIA.json), [fuentes anteriores](FUENTES_S2.json), [fijación](FIJACION_PREVIA.json), [código](codigo/bases.rs) y [reproductor](reproducir.py).

Doce casos y tres sensibilidades. Resultado pendiente de ejecución. La base exterior de vigencia se lee desde un archivo cuyo nombre se conserva; el productor G1 permanece intacto. Recuperación original y reevaluación llevan base e identidad propias. No se acredita QueryResult nativo, transición SV, producción profesional ni historia durable.

Reproducción en Linux x86_64, con Rust 1.98.0 y su binario fijado:

```sh
python3 reproducir.py --rustc /ruta/absoluta/rustc --salida /ruta/nueva/s14
```

El contrato fija máximo 17 invocaciones, sin reintentos ante fallo inesperado. No cambian núcleo, IR, paquetes externos ni rectificaciones de rumbo anteriores.
''')
J(D/'PREPARACION.json',{'version':'S14-PREPARACION/1','compilacion_previa':False,'ejecucion_funcional_previa':False,'incidencias':[],'ajuste_de_diseno':'S2 fija un único montaje: se preserva esa guarda y se reutiliza G1 directamente para la variación documental. No se reescriben sus fuentes.','limite':'Lectura de código y elaboración previas a la fijación; aún no se cualifica implementación nueva.'})
A=D/'administracion';A.mkdir(exist_ok=True)
for n in ['preparar.py','fijar.py','registrar.py','snapshot.py']:shutil.copyfile(R/n,A/n)
J(D/'FIJACION_PREVIA.json',dict(version='S14-FIJACION/1',fecha_utc=datetime.now(timezone.utc).isoformat(),archivos={str(p.relative_to(D)):sha(p.read_bytes()) for p in sorted(D.rglob('*')) if p.is_file()}))
print('Fijación previa:',sha((D/'FIJACION_PREVIA.json').read_bytes()))
