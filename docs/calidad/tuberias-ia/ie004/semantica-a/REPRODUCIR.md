# Reproducir la matriz pública y recuperar sus capturas

**RETP-2026-135.** Descargar esta carpeta completa del commit de resultado. La fijación previa de fuentes está en los commits RETP-134 identificados en el acta. No descargar ni proporcionar a este banco archivos de la reserva.

La ejecución usa Rust 1.98.0 con targets x86_64-unknown-linux-gnu y wasm32-wasip1, y Node 24.19.0 como conductor. El archivo resultados-1/ENTORNO.json identifica el entorno observado. Node verifica huellas, invoca rustc, limita procesos, captura y comprime salida; las decisiones de petición y las aserciones funcionales se ejecutan en Rust.

Desde esta carpeta, con ese toolchain instalado:

```sh
IE004_RUSTC=/ruta/al/toolchain/bin/rustc IE004_SALIDA=/ruta/nueva/resultados node ejecutar.mjs
```

El directorio de salida debe ser nuevo. El conductor verifica FIJACION_PREVIA.json antes de compilar. No hace falta ejecutar preparar-casos.mjs: las fuentes de los casos ya están fijadas. Ese generador documenta la conversión de los fixtures públicos a Rust y no forma parte del servicio. BASE_PUBLICA.json corresponde a los literales artificiales de servicio.rs; cambiar uno sin el otro invalida este corte.

Flags: edition 2021, opt-level 0/3, overflow-checks yes/no y panic=abort. WASI fija un máximo de memoria lineal de 64 MiB en el módulo y no recibe preopens ni variables de entorno del conductor. Eso no demuestra la contención A/V, una política de host completa o una garantía de RSS. Plazo por compilación 120 s y por ejecución 30 s, salida capturada máxima 32 MiB. Los costes de compilación, ejecución y exportación se conservan separados.

Para recuperar la evidencia original sin volver a ejecutar pruebas:

```sh
node recuperar.mjs resultados-1 /ruta/nueva/capturas
```

recuperar.mjs comprueba tamaños y SHA-256 del gzip y de cada stdout descomprimido, antes de escribir. RESULTADO.json identifica los dos blobs comprimidos compartidos por las cuatro capturas. No hay pérdida de registros, aunque los casos compactos omitan el bosque y lo refieran por huella.

Cada JSONL contiene cabecera, 61 casos semánticos, 72 casos sintácticos y resumen. analisis_completo se refiere a la evaluación semántica de A: las regresiones puramente sintácticas lo mantienen false. Las entradas rechazadas antes de crear el motor no ofrecen un bosque; un análisis técnico incompleto no publica significados parciales como respuesta. La evidencia de un análisis incompleto, cuando existe, está marcada como tal.

Los enlaces entre nodos, familias y estados son índices locales de esa captura. La igualdad de significados usa la representación tipada; la evidencia original y sus offsets se preservan aparte. Las diez categorías de coste y las ocho clases de reserva están documentadas en SUCESORA_CALENDARIO_Y_CONTABILIDAD.md. Esta traza es evidencia ejecutable de reglas y estados, no una reconstrucción del pensamiento interno de un LLM.

Los 61 oráculos públicos fueron preparados por Watson, incluyen los 16 contrastes heredados de /2 y no son una evaluación ciega independiente. Las 72 regresiones tienen oráculo sintáctico. Un éxito en estos conjuntos no acredita la reserva, español abierto, seguridad clínica ni aporte de una IA externa.
