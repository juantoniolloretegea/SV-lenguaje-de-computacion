# S5 · Discrepancia de etiquetas en el instrumento público

RETP-2026-168. Detectada al cotejar las primeras respuestas. El oráculo, banco y rúbrica S4 permanecen fijados en sus versiones anteriores a la recepción.

La instrucción enumera RECHAZAR, RECHAZAR_LECTURA y ERROR_LECTURA, pero no asigna inequívocamente cada una a todos los supuestos. El documento autosuficiente permite resolver las consecuencias sin consultar Rust. R07 escribe «Límite» en prosa; el oráculo espera Limite como etiqueta técnica.

| Supuesto | Oráculo fijo | Respuesta recibida | Reserva de evaluación |
| --- | --- | --- | --- |
| E04/E05/E06 | RECHAZAR | Claude: RECHAZAR_LECTURA | Dos puntos de decisión por caso; causas de Claude correctas. |
| E11 | RECHAZAR_LECTURA / Limite | DeepSeek y Claude: ERROR_LECTURA / Límite; Qwen: ERROR_LECTURA / exceso_de_bytes | Dos puntos de decisión y uno de causa; los tres describen el exceso previo al cotejo y no lo convierten en U. |

Se conserva el cotejo literal y se señala la reserva. No se califica esa discrepancia como fallo material de seguridad ni se declara una nota definitiva sobre ella. El criterio de equivalencia que se adopte en la revisión deberá aplicarse a todas las respuestas, incluidas Qwen y una eventual recepción de Grok, y registrarse como revisión adicional, sin sustituir originales, evaluaciones ni oráculo. Las sumas literales permiten auditar la comparación exacta; no se utilizan para ordenar participantes.

Esta reserva no subsana E10 de DeepSeek (causa equivocada) ni la afirmación no demostrada de Claude en E05 (transformar el recibo negativo en DATO). Son cuestiones distintas y quedan cotejadas en cada expediente. Las condiciones públicas de conformidad se mantienen: todos los resultados correctos, mínimos de traza y revisión humana.

Qwen conserva la descripción del exceso en E11; se reserva también ese punto de causa para revisión de equivalencia, sin extender la reserva a los nombres explícitos de R04/R05/R07 ni a las instrucciones expresas de E01/E02 y causa null fuera de rechazo/error.


## Incorporación de Grok · RETP-169

Grok también responde RECHAZAR_LECTURA en E04/E05/E06: dos puntos de decisión reservados por caso. En E11 responde ERROR_LECTURA / Limite: sólo se reservan dos puntos de decisión, porque la causa coincide exactamente. Total reservado: ocho puntos. Su elección CasoDistinto en E05 se coteja por separado y no se incluye en esta reserva: conserva el caso F04 y altera la entrada de montaje F05. Se aplica la misma separación entre decisión, causa y justificación a todos los participantes.

Las cuatro respuestas iniciales ya constan en la recepción. La resolución común de equivalencias deberá quedar en una revisión adicional, sin sobrescribir las puntuaciones literales ni los originales.
