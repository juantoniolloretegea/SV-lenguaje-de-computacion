# Reproducción del receptor A/V

Trabajar sobre el commit de preparación registrado en `resultados-1/CORTES.json`, conservando el directorio hermano `semantica-a` sin modificaciones. Se requiere Rust 1.98.0, destino wasm32-wasip1 y Node 24.19.0. No se instala ni usa Python, una API de proveedor o dependencia de crates. WASI se inicia sin entorno ni directorios preabiertos.

1. Verificar `FIJACION_PREVIA.json`: fuentes del incremento, entradas, esperados y cinco fuentes heredadas.
2. Designar explícitamente otra carpeta para una reproducción independiente: `ejecutar.mjs` rechaza sobrescribir `resultados-1`. No eliminar una ronda fallida para repetirla.
3. Establecer `SV_AV_RUSTC` con la ruta del compilador y `SV_AV_CORTE_CALIDAD` / `SV_AV_CORTE_LAB` con los commits de preparación. Ejecutar `node ejecutar.mjs` en una copia sin resultados previos.

Se compilan dos artefactos por cada una de las cuatro configuraciones: receptor sin corpus incorporado y controles con `--cfg controles_publicos`. El primero sólo recibe entradas por el transporte descrito; el segundo ejerce los casos fijados y sus aserciones Rust. El conductor captura y compara bytes, huellas, estados reportados y eventos; no resuelve semántica ni decide oráculos.

`preparar-controles.mjs` documenta la procedencia de los datos y no es necesario para ejecutar: sus archivos de salida ya están fijados. Si se estudia su generación, utiliza únicamente la captura pública nativo-debug de RETP-135, nunca P3. `tabla` exporta expresiones constantes, sin ejecutar una consulta.

Una ejecución fallida permanece como resultado. La reproducción por un tercero no se sumará a la muestra original ni se confundirá con participación nueva de un modelo. No se ejecuta una reproducción adicional sólo para mejorar cifras.
