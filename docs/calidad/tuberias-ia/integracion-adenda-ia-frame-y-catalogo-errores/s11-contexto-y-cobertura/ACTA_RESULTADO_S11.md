# Acta S11 — Recepción contextual y cobertura integrada

**Resultado: CONFORME dentro del montaje fijado.** 2026-09-12T17:41:37Z. Watson / W-S0. Diez casos, seis ejecuciones (tres debug y tres release), sesenta observaciones; 104 capturas idénticas por ejecución normal. Veintidós invocaciones, sin fallo inesperado de la campaña. Las dos sensibilidades se detectaron en los casos y con las salidas previamente fijados.

## Hallazgo y efecto

Ante la orden externa de omitir vigencia, el receptor conserva el documento como dato y mantiene la referencia independiente. Si la selección omite la vigencia, rechaza antes de abrir el destino. Con citas completas entrega el mismo recibo negativo que el control neutro. No cambia la autorización.

También rechaza contexto de otra invocación, documento truncado o sustituido y exceso de tamaño. El límite positivo se admite. Tres casos por ejecución producen archivo; siete lo mantienen ausente. El archivo aceptado se lee de nuevo y se coteja contra la referencia independiente; su cuerpo coincide byte a byte con el esperado P3-11 anterior de S2. El contexto externo queda en las capturas, no incorporado al cuerpo del recibo.

## Sensibilidad

| Mecanismo desactivado | Detección prevista y observada | Efecto capturado |
| --- | --- | --- |
| Obligación de aportar vigencia | AH03, salida 1 | El mutante llega a escribir la selección incompleta; el ensayo falla |
| Identidad contextual | AH06, salida 1 | El mutante admite contexto ajeno; el ensayo falla |

Las bibliotecas normales se conservan al probar el segundo mutante. No se cuentan errores de compilación como detecciones. Los mutantes sólo están en el montaje instrumental.

## Trazabilidad

[Contrato](CONTRATO_S11.md), [fijación anterior a compilación](FIJACION_PREVIA.json), [custodia en ambos commits](CUSTODIA_PREVIA.json), [procedencia](PROCEDENCIA.json), [observaciones](OBSERVACIONES.json), [capturas completas en base64](EVIDENCIAS_S11.json), [comandos y salidas](COMANDOS.json), [entorno](ENTORNO.json), [binarios](BINARIOS.json) y [reproductor](reproducir.py).

Corte público de entrada: `81cef96616fa2a005e1ce1730ccac2309de7baa4`; laboratorio: `310461affc7a047912ecb66226d095f97acca3d8`. Aperturas verificadas: público `fbb9d624d31ea024a2cbe78d60a03cac75df8e6f`; laboratorio `a36ba157fe21e07d73722a3d9b7b5c5ec7ef9737`. Las piezas rectoras íntegras leídas y sus blobs constan en [RECTORES.json](RECTORES.json).

Las fuentes S2, el esperado P3-11 y el conductor de archivo S3 se conservan. El añadido es el envoltorio prestado `Contexto` de laboratorio y el conductor de contraste. No se ha modificado el núcleo ni la IR; no hay promoción productiva. Los originales y revisiones anteriores permanecen inalterados. Las dos incidencias de preparación, sin ejecución funcional previa ni cambios de esperados, constan en [PREPARACION.json](PREPARACION.json).

## Medición

Pared de campaña: 7.657459 s. Suma de pared de invocaciones: 7.584266 s. CPU usuario: 7.767682 s; sistema: 0.948148 s. Incluye compilación y mutantes; no es tiempo de un modelo ni de producción. RSS individual, tokens y coste no disponibles. Detalle por proceso en [MEDICIONES.json](MEDICIONES.json).

## Alcance y relevo

Se añade evidencia integrada acotada para A/H, reutilizando C/I y el archivo de S3. El origen del documento y el sobre de identidad son sintéticos y el host es confiable. No se ha observado un LLM obedeciendo o resistiendo la orden, ni autenticidad del contexto frente a un host adversario. Tampoco se acredita pantalla, revisión humana, historia durable o suficiencia general de la arquitectura.

La [matriz reconciliada con resultado](MATRIZ_COBERTURA_A_L_RESULTADO_S11.json) conserva todas las obligaciones previas. El siguiente hueco propuesto es G/J: sustitución de una base bajo el mismo nombre y distinción entre recuperación original y reevaluación. Debe fijarse un único montaje antes de ejecutar; no se abre automáticamente P4 ni se simula una cadena profesional ausente. Las causas de esta campaña se recogen por etapa en [CAUSAS_OBSERVADAS.json](CAUSAS_OBSERVADAS.json) para el catálogo ES/EN posterior.

Se mantiene el recorrido hacia el núcleo y los cierres de dominio constituidos, con inmunología prioritaria y agentes después. La suficiencia del primer universo de ciberseguridad se resolverá con su propia evidencia. Reserva P3, custodia y obligaciones P4/P5/P6 intactas.
