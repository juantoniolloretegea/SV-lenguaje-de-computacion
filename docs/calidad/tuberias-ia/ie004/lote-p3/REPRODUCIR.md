# Reproducción del adaptador de lote

Use la carpeta `ie004/lote-p3` junto con las fuentes vecinas `recepcion-av` y `semantica-a` del mismo commit. Rust 1.98.0 con el destino wasm32-wasip1 y Node con WASI preview1 deben estar ya disponibles. No se instala ninguna dependencia ni se solicita una captura de modelo.

`FIJACION_PREVIA.json` identifica fuentes, datos públicos y esperados anteriores al ensayo. `ENTRADAS_PUBLICAS.json.gz.b64` conserva los bytes originales de los 36 controles, incluidos UTF-8 inválido, truncados y sobrelongitudes. Su tamaño y SHA se verifican antes y después de descomprimir. `preparar.mjs` documenta su construcción desde artefactos públicos; no hace falta ejecutarlo para reproducir.

Con `SV_LOTE_RUSTC` apuntando al rustc 1.98.0 disponible y `SV_LOTE_CORTE_CALIDAD`/`SV_LOTE_CORTE_LAB` identificando el depósito previo, ejecute `node ejecutar.mjs`. El conductor rechaza una carpeta `resultados-1` ya existente: cada reproducción externa se hace en una copia nueva, sin borrar la evidencia anterior. Compila adaptador y receptor A nativo/WASI debug/release, ejecuta la única matriz fijada y conserva stdout/stderr, argumentos, tiempos, solicitudes extraídas y cuerpos. Los ocho binarios quedan identificados y son reconstruibles.

No se depende de GitHub Actions. Los JSON originales de la reserva, el auxiliar y las propuestas no se leen. Las comprobaciones de custodia de fuentes/fixtures preceden y están separadas del recorrido A. El WASI heredado usa entorno vacío y ningún directorio preabierto. No se atribuye contención frente a host comprometido.

El JSON de resultados separa 36 controles repetidos tres veces por configuración y 24 solicitudes públicas recorridas tres veces. No son casos inéditos, observaciones independientes de modelos ni un benchmark. Las trazas son evidencia completa de los procesos capturados; tiempos/cotas no acreditan RSS ni P4/P5. Un resultado desfavorable se conserva y no se convierte en permiso para regenerar esperados.
