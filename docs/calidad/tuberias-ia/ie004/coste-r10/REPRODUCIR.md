# Reproducción del ensayo de coste R10

**RETP-2026-133.** Fuentes de IE004-R10-MEMO/1 fijadas en RETP-132 antes de ejecutar. Se conserva una ejecución de cada configuración; reproducir produce evidencia nueva y no debe sobrescribir ésta.

## Identidad y resultados

Fuentes previas en Lenguaje: commit `638eba02f55f528c94871548858ec967d0e9ef2a`. Espejo previo: `1d6f1183c9604817852844544e1855c8051e2f2e`. `FIJACION_PREVIA.json` contiene bytes y SHA-256 de las fuentes, corpus y lanzador. `resultados/MANIFIESTO_RESULTADOS.json` identifica el conjunto de evidencia textual del corpus, incluido su entorno.

Motor Rust 1.98.0, host x86_64-unknown-linux-gnu, target adicional wasm32-wasip1. Node v24.19.0 fue únicamente lanzador, host WASI y procesador de evidencia; no ejecutó gramática ni semántica. Los argumentos exactos figuran en cada registro `*-compilar.json`. Los ejecutables están identificados por huella en los resultados; no se incluyen en este depósito. Su reconstrucción en otro path o entorno no implica identidad binaria bit a bit.

## Recuperar las salidas originales sin ejecutar el motor

Desde esta carpeta, con un destino nuevo:

```sh
node recuperar.mjs resultados /tmp/ie004-capturas-recuperadas-133
```

Se verifican SHA-256 y longitud tanto del gzip como de los bytes descomprimidos, con un máximo de 32 MiB por descompresión. Se reconstruyen las cuatro salidas completas, incluidos todos los nodos y familias. Hay dos objetos gzip/base64 porque debug y release emitieron idénticos bytes dentro de cada destino. Los resúmenes por caso no sustituyen esos originales.

## Reejecutar el corpus público, si se decide reproducir

Con Rust 1.98.0 y ambos targets ya instalados, desde una copia íntegra de esta carpeta:

```sh
IE004_RUSTC=rustc IE004_SALIDA=/tmp/ie004-reproduccion-corpus-133 node ejecutar.mjs
```

El destino debe ser nuevo. El lanzador coteja la fijación, compila debug/release nativo y WASI, y ejecuta cada binario una vez. Compilación: 120 s por proceso y 480 s acumulados; ejecución: 30 s por proceso y 300 s acumulados; salida capturada: máximo 32 MiB. WASI tiene máximo enlazado de 64 MiB y no recibe preopens ni variables de entorno. Estos topes del instrumento no acreditan aislamiento P4.

Las medidas de tiempo abarcan arranque y exportación del bosque; son una muestra por configuración. No son tiempo por consulta, p95, garantía de tiempo real, latencia de proveedor ni comparación de velocidad con la sonda anterior, que exportaba otro volumen de información. Los MiB contabilizados no equivalen a RSS.

## Reproducir los cuatro controles estructurales

Usar otra copia de la carpeta que conserve las fuentes, `resultados/RESULTADO.json` y sus capturas comprimidas, pero que no contenga el directorio `controles-resultados`. Conservar intacta la copia de evidencia publicada. Ejecutar una vez:

```sh
IE004_RUSTC=rustc node ejecutar-controles.mjs
```

Ese lanzador comprueba la huella de `memo.rs`, renombra sólo su main y añade `controles.rs`; no vuelve a ejecutar las 72 preguntas. Crea `controles-compilados.rs`, compila los cuatro destinos/configuraciones con 120 s máximos por compilación y 30 s por ejecución. Verifica además la recuperabilidad de las capturas anteriores sin ejecutar el corpus. La fijación de estos controles fue local, después de observar 72/72 y antes de sus propias ejecuciones; no se presenta como prepublicación remota ni como prueba inédita.

`controles-resultados/RESULTADO_CONTROLES.json` y sus logs conservan lo observado. `MANIFIESTO_CIERRE.json` identifica los archivos añadidos al cierre, sin incluirse a sí mismo. La reserva P3 no interviene en ninguna de estas operaciones.
