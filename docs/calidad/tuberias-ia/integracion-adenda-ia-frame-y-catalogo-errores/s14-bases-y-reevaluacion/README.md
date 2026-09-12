# S14 — Bases documentales y recuperación/reevaluación

Apertura fijada antes de compilar. [Contrato](CONTRATO_S14.md), [procedencia](PROCEDENCIA.json), [fuentes anteriores](FUENTES_S2.json), [fijación](FIJACION_PREVIA.json), [código](codigo/bases.rs) y [reproductor](reproducir.py).

Doce casos y tres sensibilidades. Resultado pendiente de ejecución. La base exterior de vigencia se lee desde un archivo cuyo nombre se conserva; el productor G1 permanece intacto. Recuperación original y reevaluación llevan base e identidad propias. No se acredita QueryResult nativo, transición SV, producción profesional ni historia durable.

Reproducción en Linux x86_64, con Rust 1.98.0 y su binario fijado:

```sh
python3 reproducir.py --rustc /ruta/absoluta/rustc --salida /ruta/nueva/s14
```

El contrato fija máximo 17 invocaciones, sin reintentos ante fallo inesperado. No cambian núcleo, IR, paquetes externos ni rectificaciones de rumbo anteriores.
