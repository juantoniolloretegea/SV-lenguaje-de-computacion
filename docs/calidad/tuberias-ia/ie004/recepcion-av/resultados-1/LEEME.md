# Lectura de los resultados

En Git se ofrecen directamente RESULTADO.json, ENTORNO.json, CORTES.json y MANIFIESTO_RESULTADOS.json. Los archivos originales enumerados por el manifiesto, incluidos stdout/stderr y marcos binarios, se conservan sin pérdida dentro de `../CAPTURAS_RECUPERABLES.json.gz.b64`.

Use `../recuperar-resultados.mjs` con un directorio nuevo para recuperarlos. El archivo contiene la ronda original completa antes de incorporar esta nota de navegación. No ejecuta nuevamente el receptor ni depende de un artefacto de Actions con caducidad.
