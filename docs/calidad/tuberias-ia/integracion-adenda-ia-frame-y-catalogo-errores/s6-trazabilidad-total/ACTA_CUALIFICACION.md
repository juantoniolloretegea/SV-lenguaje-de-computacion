# S6 · Cualificación del segundo intento de trazabilidad

RETP-2026-170 · 2026-09-12T15:02:14Z · Responsable: Watson / W-S0.

**S6 finalizado. S7 pendiente de recepción.** [Encargo común inmutable](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/fccde9cf524a0d62b2dd1a2ee05170d6f4358074/pruebas-externas/s6-trazabilidad-total/PRUEBA_COMUN.md), cualificado y publicado sin solicitar al usuario revisión de diseño. El requisito de admisión es el cumplimiento íntegro, sin compensaciones por nota media.

## Evidencia de diseño y liberación

Se conservaron los doce supuestos conocidos y sus fuentes. Las causas y decisiones se vincularon al oráculo previo S4, cotejado originalmente con capturas Rust S2/S3. Las etiquetas antes ambiguas están definidas de antemano: rechazo documental RECHAZAR; exceso RECHAZAR_LECTURA / Limite; ausencia ERROR_LECTURA / Io(NotFound). CasoDistinto y VigenciaDistinta comparan piezas distintas completas.

El contrato exige el enunciado íntegro, todas las fuentes y reglas especificadas con texto exacto, fundamento del catálogo, decisión, causa, contenido y llamadas cuando proceden, consecuencia y límites. No hay evaluación de estilo ni penalizaciones introducidas tras la devolución. El orden JSON y las colecciones equivalentes se aceptan; las citas preservan los bytes UTF-8 tras decodificar.

Se ejecutaron 482 controles, todos con el resultado esperado: ocho entregas válidas aceptadas; 472 defectos rechazados; dos alteraciones del instrumento clasificadas ERROR_INSTRUMENTO. Se comprobó la CLI real con una referencia correcta y una causa E05 incorrecta. [Informe público](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/fccde9cf524a0d62b2dd1a2ee05170d6f4358074/pruebas-externas/s6-trazabilidad-total/CUALIFICACION.json). Los controles, sus entradas y salidas completas se custodian junto a la referencia privada, commit e2ef439e22583dac0f9b1c4fe4b38ce4bc5e0576, anterior a la publicación. [Constancia](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/fccde9cf524a0d62b2dd1a2ee05170d6f4358074/pruebas-externas/s6-trazabilidad-total/CUSTODIA_PREVIA.json).

La incidencia de preparación de un literal bytes con tilde se corrigió antes de ejecutar la cualificación; se conserva íntegra y no se atribuye a un participante ni a un control ejecutado. El cotejador en Python sólo compara archivos y referencia; no ejecuta el dominio ni redefine la semántica de Rust.

El documento público fue descargado por GET sin Authorization ni cookies: HTTP 200; 29.890 bytes; SHA-256 ad5a03b70f376a037082c44aa2d98e10dcb56c621864136300f5015de30ed1f7. [Comprobación de acceso](ACCESO_PUBLICO.json). Se puede entregar el mismo archivo como adjunto.

## Criterio y alcance

El aviso de exclusión se incorpora al encargo antes de participar: para el SV y su dirección, la falta de trazabilidad exigida fundamenta consideración de opacidad, falta de auditabilidad y confianza, y descarte general para toda función. El expediente y los resultados podrán difundirse internacionalmente, incluidos Europa y Estados Unidos. El informe identificará incumplimiento y evidencia; distinguirá la decisión de la dirección de las capacidades o valoraciones universales que este ensayo no demuestra.

La traza documental de la respuesta es verificable por el cotejador. Las afirmaciones adicionales de ejecución y medidas requieren registros adjuntos y revisión propia; una declaración no se convierte en un hecho verificado. No añadir actividad propia es compatible con la vía documental; no exige ejecución o recálculo de huellas. Un incidente de transporte probado o un defecto del instrumento no se atribuye al modelo.

El primer instrumento queda como antecedente: las cuatro recepciones y puntuaciones literales se conservan; su umbral parcial no acredita trazabilidad total y no se aplica retrospectivamente este contrato. No se declara exclusión de un participante antes de recibir y comprobar su segunda entrega. No se ha enviado el encargo por herramientas de mensajería ni se han recibido aún esas respuestas.

## Continuidad

S7 preservará originales e intentos, aplicará el contrato fijado y registrará la evidencia de los dictámenes. Si se detecta un nuevo defecto del instrumento, se detiene su uso y se registra el fallo de diseño, sin encadenar campañas para conseguir un aprobado. Se mantienen integración 1+3 y registro continuo de causas; consolidación del catálogo al final. Mismas ramas; sin promoción de núcleo, host o cierre A–L.
