# Reproducir el contraste acotado RETP-142

El expediente distingue la recuperación de evidencia histórica, la reejecución de un caso público A/V y las pruebas aisladas de Frame. No acredita la tubería integral ni ejecuta modelos o la reserva P3.

## Entradas

- Checkout local del Lenguaje en `8880a099b0b9549aaa66c2c5d93e0b9c247834ff`. `FUENTES_CORTE.json` contiene los 50 paths, tamaños y blobs Git exigidos. Usar ese corte, no el último main: los registros y la navegación pueden haber cambiado.
- Node con módulos estándar; Rust `1.98.0`. El resultado conservado identifica la versión exacta y las opciones utilizadas. La reproducción descrita es nativa; no constituye una nueva campaña de cuatro configuraciones.
- Copia de `comprobar-evidencia.mjs`, `PLAN_CONTRASTE.json` y `FUENTES_CORTE.json` en una carpeta instrumental nueva. Conservar junto a ella `RECTIFICACION_ESPERADO_C08.md` e `INCIDENCIA_OBSERVADOR.md` para interpretar la sucesión.

## Ejecución

Desde esa copia instrumental, sustituir las rutas de ejemplo por rutas absolutas:

```sh
node comprobar-evidencia.mjs /ruta/al/corte /ruta/a/rustc /ruta/a/salida-nueva
```

La salida no debe existir. El script escribe además `RESULTADO_CONTRASTE.json` y `EVIDENCIA_RECUPERABLE.json` junto a sí mismo: por eso debe ejecutarse desde la copia nueva, para conservar intactos los resultados publicados.

Primero coteja las fuentes y recupera la cápsula RETP-137. Verifica sus 409 archivos y los 36 recorridos ya capturados. Después compila las fuentes A/V originales, reproduce A01 y los controles de marco y V indicados en el plan, y ejecuta las 16 pruebas existentes de Frame en un arnés aislado. Conserva stdin, stdout, stderr y metadatos de los procesos; no publica los binarios como evidencia portable.

## Lectura de los resultados conservados

El plan original conserva un esperado incorrecto de mi observador en C08. La rectificación documentada fija `TRUNCADO`, que ya figuraba en el control y la captura históricos. No se cambió Rust ni el oráculo histórico.

El resultado publicado se construyó con C01–C07 del segundo intento, la comprobación offline de C08 contra los bytes conservados y la primera ejecución de C09 en la continuación. El script completo corregido se entrega para reproducir ese contraste; no se afirma una tercera ejecución completa de él. `completar-contraste.mjs` conserva el procedimiento efectivamente usado para la continuación.

Los dos fallos instrumentales permanecen en `RESULTADO_INICIAL.json`, `RESULTADO_SEGUNDO_INTENTO.json`, sus cápsulas y las versiones anteriores del observador. No son fallos de Rust. La mezcla de avisos Node y JSON en stderr de WASI permanece documentada: se reconoció el prefijo observado sin borrar ni modificar los bytes capturados.

Cada entrada de las cápsulas de evidencia contiene contenido base64, tamaño y SHA-256. Se puede recuperar cada archivo decodificando su contenido y cotejando esos dos atributos antes de usarlo. Las rutas originales de los procesos son procedencia del instrumental; no deben ejecutarse automáticamente al recuperar una cápsula.

Los tiempos, los paths de compilación y los hashes de los binarios pueden diferir en otra máquina. Los cotejos de cuerpo, traza, anexos y rechazo son los descritos por el expediente. Ningún resultado de este procedimiento convierte la evidencia del banco en inscripción de dominio o en recuperación durable del núcleo.
