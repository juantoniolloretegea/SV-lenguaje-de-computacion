# S26 R06 · T0701 · Recepción de mutación y concordancia de corte

**14 de septiembre de 2026. Unidad: Watson / W-S26. Resultado: 13/13 casos conformes en debug y 13/13 en release; ambas campañas con salida 0.**

Se ejecutó el [banco previo](README.md) después de publicar su precompromiso en el Lenguaje, [0665bc32137a76e3a922197ab648526d8c0349bf](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/0665bc32137a76e3a922197ab648526d8c0349bf), y en laboratorio, [aae666c74ac5db4af9a56ceb2b60a2ba5b7226fa](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/commit/aae666c74ac5db4af9a56ceb2b60a2ba5b7226fa). Antes de ejecutar se cotejaron ambos árboles completos con el auxiliar Rust R08 y se releyeron las ramas publicadas.

Los seis archivos del banco y las 107 piezas de entrada cotejadas permanecieron intactos durante ambas campañas. Los oráculos no se modificaron después de observar resultados. [EJECUCIONES.json](EJECUCIONES.json) conserva comandos, fechas, entorno, salidas y huellas de los ejecutables.

## Resultado material

| Casos | Observación recibida | Alcance |
|---|---|---|
| P01/P02 | Control válido; una escritura posterior a admit altera el mismo archivo, pero el consumidor recibe el descriptor A ya admitido y su recibo literal. | Ligadura local admisión–consumo pese a mutación del archivo. |
| P03/P04 | Muestra intermedia B y restitución A sobre el mismo objeto. El perfil mínimo conserva consumo A; el perfil de continuidad previamente solicitado termina NO_EJECUTADO, sin captura ni recibo. | No se degrada el perfil para aceptar una garantía que no se ofrece. |
| P05 | Tras leer 3072 bytes, se modifica el último byte pendiente. Se recibe esa alteración; después se restituye el archivo A. Guarda M01 y ausencia de consumidor. | Extremos del archivo iguales no convierten en válido el búfer alterado entre lecturas. |
| P06 | Tras el mismo prefijo, se modifica el primer byte ya leído; se observa B y se restituye A. El búfer recibido es A y obtiene recibo válido. | Contraejemplo a deducir inmutabilidad a partir de extremos iguales e incluso de un contenido recibido válido. |
| P07/P08 | La dependencia de estado B recibida antes de la copia se rechaza por I03; una escritura posterior a ReceivedBytes deja válida la copia A y conserva el archivo B como observación nueva. | La custodia se coteja con lo recibido; las copias no cambian por una escritura posterior. |
| P09/P10 | El conjunto explícito r2 es válido; una solicitud r2 frente a custodia r1 se rechaza por I02. | Igualdad de contenido compatible con ocurrencias/revisiones distintas, sin intercambiarlas. |
| P11/P12 | Captura r2 de bytes idénticos para admisión r1: D04, sin recibo aceptado; vínculo r1 con entrega r2 en solicitud: A01. | Concordancia de referentes en sus fronteras. P11 conserva una captura ya efectuada; no prueba bloqueo anterior a todo consumo. |
| P13 | Solicitud con estado alterado y su hash correcto, frente a custodia original: I03. | Una huella propuesta coherente no sustituye la expectativa autorizada. |

Todos los casos conservaron dev/inode del objeto observado en cada muestra aplicable. Las muestras binarias intermedias y finales se guardaron separadamente de la captura del consumidor. Los puntos de mutación fueron barreras síncronas en un hilo, sin dependencia de una temporización supuesta.

En P05 y P06 las lecturas inicial y final de geometría tienen SHA-256 b022979a6d586dee19788944eee5f7e84ec416b947d182cac3a7a1a80d1cc213. Las muestras intermedias son distintas y están conservadas. La conclusión depende de los bytes y de sus fronteras observadas; esa huella no se convierte en identidad de ocurrencia ni prueba de continuidad.

## Evidencia y reproducción

[EVIDENCIA.tar.gz](EVIDENCIA.tar.gz): 119784 bytes; SHA-256 0b7a963da949e824d0c898254827fb6cd2a9dc7ed7eda6fd247c7b376328ca11.

Contiene ambas campañas completas, sus resultado.json, aperturas, solicitudes efectivamente presentadas, muestras del objeto, geometría recibida, descriptor admitido, contenido consumido, rechazos, recepciones y observacion.json. Incluye stdout/stderr, logs de compilación y el manifiesto de archivos precomprometidos. Los cortes publicados identifican los árboles cotejados antes de ejecutar.

| Perfil | Inicio UTC | Casos conformes | Salida |
|---|---|---:|---:|
| debug | 2026-09-14T06:54:10Z | 13/13 | 0 |
| release, debug-assertions=false | 2026-09-14T06:54:38Z | 13/13 | 0 |

Rust/Cargo 1.98.0, compilación offline/locked; Linux 6.18.44 x86_64. El log conserva 25 advertencias del núcleo heredado; no hubo error de compilación. La edición administrativa y los cotejos de árboles utilizaron los auxiliares Rust existentes. No se ejecutó Python ni Java.

## Límites y relevo

La cualificación se limita a T07 bajo la custodia y el modelo local declarados. No acredita integridad física de RAM, resistencia al anfitrión, adquisición atómica de todas las piezas, continuidad del soporte, concurrencia hostil, supervivencia del observador, durabilidad, revocación distribuida ni efecto externo. P09 usa una revisión ya presente en el registro experimental; no demuestra procedencia de una adquisición externa por igualdad de bytes.

| Incremento recibido | Discriminadores en su alcance | Estado del conjunto |
|---|---|---|
| LOCAL01 | T01/T02/T05/T06 | Evidencia anterior conservada. |
| PROCESO03 | T03/T04/T08 | Evidencia anterior conservada; PROCESO01 impedido y PROCESO02 incompleto mantienen sus resultados propios. |
| T0701 | T07 | Recibido localmente en esta incorporación. |

Esta correspondencia no prueba la composición de las tres realizaciones. El siguiente paso es fijar y precomprometer un banco integrado que enlace apertura y referentes, recepción/admisión, modificación de fuente o dependencia, captura, informe y observación de terminación. Debe reutilizar las piezas ya contrastadas y precisar qué hechos sobreviven juntos a cada corte, qué rechazo precede al consumo y qué ausencia conserva la indeterminación del efecto.

No se repiten T0701 ni las campañas anteriores por cambiar de unidad o actualizar el registro. Una repetición requiere una modificación material o una obligación integrada identificada. Sucesos S26, revisión 20, enlaza esta recepción y su espejo; W-S26 se mantiene como responsable. RETP canónica no se modifica y no se crean ramas. Cero casos globales S26 cerrados; Bis y la secuencia hacia S24 permanecen abiertos.
