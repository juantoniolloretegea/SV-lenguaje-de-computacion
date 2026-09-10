# Reproducción del puesto IE-004

El paquete `ie004/fuentes` permite reproducir el receptor y sus controles desde este repositorio público. No requiere acceso al laboratorio privado, API de IA ni paquetes npm o crates externos. Se requiere Rust 1.98.0 con destino wasm32-wasip1, Node con WASI preview1 y un sistema capaz de ejecutar el binario nativo generado.

Desde la carpeta `docs/calidad/tuberias-ia/ie004/fuentes` de una copia de este repositorio:

```bash
mkdir -p ../bin ../resultado
rustc +1.98.0 --edition=2021 -O -C panic=abort --crate-name receptor_ie004 receptor.rs -o ../bin/receptor
rustc +1.98.0 --edition=2021 -O -C panic=abort --target wasm32-wasip1 -C link-arg=--max-memory=67108864 --crate-name receptor_ie004 receptor.rs -o ../bin/receptor.wasm
node ejecutar.mjs ../bin ../resultado
```

La ruta nativa y la WASI deben producir los mismos bytes y cumplir los oráculos de control. El resultado conserva además la brecha semántica demostrada: el testigo conocido pasa las comprobaciones del receptor con una referencia equivocada y el observador lo detecta. Un exit 0 del conductor significa que la caracterización se reprodujo; no que la candidata sea apta ni segura para el SV.

En Linux con `/usr/bin/time` se conservan sus medidas de tiempo, CPU y RSS. Si esa herramienta no está disponible, el conductor ejecuta directamente y registra únicamente tiempo monotónico de proceso; CPU y RSS quedan explícitamente no observables. No se simulan ni estiman esas magnitudes. Las ejecuciones locales pueden variar de entorno respecto de GitHub Actions.

Tras el depósito de Grok y la apertura del compromiso, se publicarán en el mismo expediente los cuatro archivos exactos de recepción: solicitudes, entrega, oráculo salado y compromiso. Se ejecutan así, desde la carpeta de fuentes y pasando sus rutas reales:

```bash
node ejecutar.mjs ../bin ../recepcion RUTA_SOLICITUDES RUTA_ENTREGA RUTA_ORACULO RUTA_COMPROMISO
```

El conductor verifica SHA y tamaño del oráculo y SHA de las solicitudes antes de evaluar la entrega. La expectativa de identificación procede del oráculo previo; la expectativa del literal procede de `base.json`, por separado de la implementación Rust. `RECEPCION.json` distingue identificación, cuerpo, relaciones y paridad. Los originales se conservan sin reserializar como prueba del depósito. Una impugnación del oráculo se tramita por sucesión.

El oráculo de la participación no se incluye mientras el participante trabaja. Esta restricción temporal no afecta a los controles públicos del puesto; el paquete de auditoría del resultado se completa después de congelar la captura. No se anuncia una auditoría ejecutada por Claude.
