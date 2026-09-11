# Reproducir IE004-R10-SINTAXIS/1

Esta ejecución no necesita originales reservados, red, Python ni dependencias Cargo. Requiere Rust 1.98.0 con targets x86_64-unknown-linux-gnu y wasm32-wasip1, y Node 24 con WASI preview1. La preparación del entorno puede requerir red. La comprobación publicada se hizo en Linux x86_64; otro host deberá declarar su diferencia.

Desde este directorio, use un directorio nuevo para su evidencia. El lanzador se niega a sobrescribir una salida existente:

```sh
rustup toolchain install 1.98.0 --profile minimal --target wasm32-wasip1
IE004_RUSTC="$(rustup which rustc --toolchain 1.98.0)" \
IE004_SALIDA=/tmp/ie004-r10-mi-reproduccion \
node ejecutar.mjs
```

La fuente `sonda.rs`, `casos.rs` y `CASOS_PUBLICOS.json` tienen huellas preestablecidas que el lanzador verifica antes de compilar. El driver `ejecutar.mjs` y el ejecutor `wasi.mjs` se incorporan con la evidencia de RETP-131; no ejecutan la gramática. Cada configuración se compila una vez y se ejecuta una vez. No ajuste las huellas ni los esperados para hacer pasar una implementación distinta: debe conservar su propia identidad de revisión.

Se guardan stdout, stderr, comandos, flags, códigos de salida, límites, horas y duración de cada proceso. También se generan `RESULTADO.json`, `ENTORNO.json` y `MANIFIESTO_RESULTADOS.json`. Los binarios quedan en ese directorio; sus tamaños y SHA-256 están en el manifiesto. En git se publican fuentes y evidencias textuales, no los binarios. La misma compilación en otro directorio puede producir bytes de binario diferentes por metadatos/rutas: esta prueba exige comparar la salida por caso y declarar las diferencias de entorno, no afirma compilación bit a bit universal.

La ejecución original produjo 69/72 coincidencias en cada configuración. Se espera conservar como hallazgos `PRESUPUESTO_AGOTADO` en C02, CP-T-127 y CP-T-128; los esperados del corpus siguen siendo DERIVA. Las 21 aserciones de primitivas terminaron correctamente. El código de salida 0 sólo significa que el instrumento terminó; consulte las comparaciones funcionales. No es prueba semántica del corrector P3.

No se repite para buscar un resultado favorable, no se amplían los cupos y no se abre la reserva. Las duraciones de una muestra no constituyen un benchmark ni una medida de latencia de IA. El máximo de memoria lineal del módulo no equivale al consumo de todo el motor WASI ni a una prueba de aislamiento del host.
