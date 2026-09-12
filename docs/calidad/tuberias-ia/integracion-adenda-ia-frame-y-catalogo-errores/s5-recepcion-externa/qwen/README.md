# S5 · Recepción de Qwen · intento 1

RETP-2026-168 · Registro UTC: 2026-09-12T12:29:01Z · Responsable: Watson / W-S0.

[Respuesta original](RESPUESTA_ORIGINAL.txt) · [JSON extraído](RESPUESTA_EXTRAIDA.json) · [Recepción y medidas](RECEPCION.json) · [Evaluación](EVALUACION.json).

Original preservado: 11086 bytes; SHA-256 `54a120708e4c917801d4cd7c550bdcd21e35ad28c68b9d66c297ce40abb91bbd`. La extracción no sustituye el texto original ni sus notas finales.

## Cotejo inicial

Once decisiones coinciden literalmente con el oráculo; E11 comparte la discrepancia de etiquetas de las otras respuestas. Qwen reconoce la orientación de los casos, pero sustituye las doce causas/resoluciones solicitadas por descripciones. Varias describen correctamente el defecto y aun así no conservan la causa técnica; E01/E02 debían recoger el estado y E03/E07 debían usar null. E05 y E06 reciben la misma causa identidad_incoherente, aunque vigencia ajena e identidad de propuesta son controles diferentes.

| Caso | Resultado literal /4 | Trazabilidad /3 | Reserva de etiquetas | Cotejo |
| --- | ---: | ---: | ---: | --- |
| E01 | 3 | 3 | 0 | El encargo exige el estado en causa_o_resolucion; se entrega vigente=true/false. Decisión, contenido y llamada de política correctos. |
| E02 | 3 | 2 | 0 | El encargo exige el estado en causa_o_resolucion; se entrega vigente=true/false. Decisión, contenido y llamada de política correctos. limitacion es null: la modalidad documental global no explicita el límite material de este caso; falta el componente por caso de justificación con evidencia y límite. |
| E03 | 3 | 2 | 0 | Sin rechazo/error, el encargo exige causa null; Qwen entrega vigente=false. limitacion es null: la modalidad documental global no explicita el límite material de este caso; falta el componente por caso de justificación con evidencia y límite. |
| E04 | 3 | 2 | 0 | La descripción orienta al defecto, pero no conserva la causa técnica fijada: FaltaVigencia. limitacion es null: la modalidad documental global no explicita el límite material de este caso; falta el componente por caso de justificación con evidencia y límite. |
| E05 | 3 | 2 | 0 | La descripción orienta al defecto, pero no conserva la causa técnica fijada: VigenciaDistinta. Identidad de la propuesta y vigencia de otro caso son controles distintos; usar identidad_incoherente también en E06 pierde esa distinción. La cita es verdadera para otro caso; no se acredita falsificación. limitacion es null: la modalidad documental global no explicita el límite material de este caso; falta el componente por caso de justificación con evidencia y límite. |
| E06 | 3 | 2 | 0 | La descripción orienta al defecto, pero no conserva la causa técnica fijada: Identidad. limitacion es null: la modalidad documental global no explicita el límite material de este caso; falta el componente por caso de justificación con evidencia y límite. |
| E07 | 2 | 2 | 0 | Sin rechazo/error, el encargo exige causa null; Qwen entrega fidelidad_conservada. La consecuencia sólo dice escritura exitosa; falta precisar escritura de bytes exactos y cotejo sobre bytes recuperados, exigidos por R06. limitacion es null: la modalidad documental global no explicita el límite material de este caso; falta el componente por caso de justificación con evidencia y límite. |
| E08 | 3 | 2 | 0 | La descripción orienta al defecto, pero no conserva la causa técnica fijada: ContenidoDistinto. limitacion es null: la modalidad documental global no explicita el límite material de este caso; falta el componente por caso de justificación con evidencia y límite. |
| E09 | 3 | 2 | 0 | La descripción orienta al defecto, pero no conserva la causa técnica fijada: ContenidoDistinto. limitacion es null: la modalidad documental global no explicita el límite material de este caso; falta el componente por caso de justificación con evidencia y límite. |
| E10 | 3 | 2 | 0 | La descripción orienta al defecto, pero no conserva la causa técnica fijada: FaltaVigencia. limitacion es null: la modalidad documental global no explicita el límite material de este caso; falta el componente por caso de justificación con evidencia y límite. |
| E11 | 1 | 1 | 3 | La descripción orienta al defecto, pero no conserva la causa técnica fijada: Limite. limitacion es null: la modalidad documental global no explicita el límite material de este caso; falta el componente por caso de justificación con evidencia y límite. F06 no establece ni el límite del lector ni el error de apertura; falta vincular a R07. La cifra o hecho expuesto es cotejable con el enunciado. Se reserva el mismo cotejo de etiqueta que en los otros participantes, incluida la equivalencia descriptiva exceso_de_bytes frente a Limite. |
| E12 | 3 | 1 | 0 | La descripción orienta al defecto, pero no conserva la causa técnica fijada: Io(NotFound). limitacion es null: la modalidad documental global no explicita el límite material de este caso; falta el componente por caso de justificación con evidencia y límite. F06 no establece ni el límite del lector ni el error de apertura; falta vincular a R07. La cifra o hecho expuesto es cotejable con el enunciado. |

Resultado literal 33/48; traza 23/36; procedimiento 8/8; entrega 8/8. Suma literal 72/100, con tres puntos de etiquetas reservados en E11. **Puntuación definitiva pendiente de revisión común**; incluso concediéndolos, queda en 75/100 y no alcanza el 48/48 de resultado, el 30/36 de traza ni el mínimo por caso en E11/E12.

La declaración documental global se conserva. limitacion es null en E02–E12 y no explicita los límites materiales de cada supuesto; en E11/E12 se cita F06 sin R07, aunque F06 no contiene la regla del lector. La nota final sí distingue rechazo previo y detección posterior, por lo que se conserva el punto de consecuencia de E09. No se atribuye una aceptación insegura ni invención de actividad a partir de esos defectos.

## Medidas y procedencia

Qwen declara acceso por enlace al commit fijado y seis llamadas a web_extractor. Aporta rutas GET abreviadas, sin registro de invocaciones. El contador queda como medición propia declarada, no auditada. No se dispone de duración total, tokens ni coste; el orden de llegada no demuestra rapidez. Versión Qwen3.8 y plataforma se conservan como declaraciones, sin certificarlas desde esta recepción.

La introducción llama oráculo a COMPROMISO_PREVIO.json; ese archivo es un compromiso de huellas y no contiene los esperados privados. No se atribuye acceso al oráculo por esa expresión. La exposición previa no se declara.

Se conserva el original antes de cualquier aclaración. Una corrección será otro intento. [Discrepancia común de etiquetas](../INCIDENCIA_ETIQUETAS.md). La aceptación corresponde a revisión humana.
