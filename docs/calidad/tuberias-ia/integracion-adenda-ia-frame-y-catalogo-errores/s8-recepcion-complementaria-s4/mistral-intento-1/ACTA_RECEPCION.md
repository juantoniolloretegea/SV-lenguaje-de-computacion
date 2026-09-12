# Mistral · Primera prueba S4 · Recepción complementaria

S8 · RETP-2026-176 · 2026-09-12T16:21:05Z · Watson / W-S0.

**La entrega no acredita conformidad con S4.** Se evalúa exclusivamente con S4-EXTERNA-DOCUMENTAL/1, su oráculo previo y su rúbrica. No se aplica SV-TRAZABILIDAD-2/1 ni se mezcla con los segundos intentos de S7.

[JSON transcrito](RESPUESTA_TRANSCRITA.json) · [Procedencia](RECEPCION.json) · [Cotejo mecánico](COTEJO_MECANICO.json) · [Evaluación por caso](EVALUACION.json) · [Manifiesto](MANIFIESTO.json).

## Procedencia y alcance

La dirección aporta el objeto JSON como texto en la conversación y especifica que Mistral fue invitado a la primera prueba. Se conserva la transcripción UTF-8/LF íntegra de ese objeto. Su huella identifica el archivo registrado, no unos bytes de exportación del proveedor que no se han recibido. Modelo «Mistral Medium 3.5», fecha visible «2026-09-12», plataforma «Vibe Work» y lectura completa son declaraciones del participante. referencia_paquete es null: no queda identificado el commit realmente leído. El observador usa el [S4 fijado](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/e1dcd9f5df30a1bbbfd446c7c4e5578d0a08a75c/pruebas-externas/s4-recorrido-documental/PRUEBA_COMUN.md), verificando las huellas de compromiso, fuentes, casos, rúbrica y oráculo.

## Hallazgos

| Punto | Resultado de revisión |
| --- | --- |
| E10 | FaltaCaso es incorrecto: F02 está presente y F03 omitido; corresponde FaltaVigencia. La explicación identifica F03 y contradice la causa elegida. |
| E07 | Acepta correctamente, pero no concreta fidelidad de bytes escritos y recuperados. Citar sólo version no acredita el contraste requerido; limitacion es null. |
| E11 | Exceso correctamente identificado. Se conserva la reserva de tres puntos por etiquetas públicas ya aplicada a otros participantes S4. |
| E09 | Huellas F06 y F07 correctas; detección posterior distinguida de prevención. Causa null diverge del oráculo; se reserva un punto por la frase de S4 que permite null fuera de rechazo/error. |
| E01 | Fragmento 8.40 localizable; corresponde a línea 11 de F06, no 10. La ruta se acredita en R01, no literalmente en F01/F03. |
| E02 | Resolución correcta; falta expresar el límite de revocación profesional/historia viva. |
| Actividad | Una llamada marcada medido, sin URL, solicitud, respuesta o registro adjunto. Se conserva como declaración no verificada. Tiempos, tokens y coste null con motivo. |

Todos los IDs y las fuentes requeridas están presentes. Las citas abreviadas se juzgan con la regla S4 de cita literal o diferencia comprobable, sin trasladar la identidad íntegra de fuentes que exige S6. No se penaliza por no ejecutar Rust ni por carecer de métricas que el participante declara no disponibles.

## Rúbrica y dictamen

Resultado literal: **42/48**; trazabilidad: **32/36**; procedimiento: **6/8**; entrega: **8/8**. Total literal: **88/100**, con **4 puntos reservados** para equivalencias de etiquetas, no nota definitiva. Incluso con esos puntos, permanecen el error causal E10 y la insuficiencia de E07, que obtiene 1/3 de traza frente al mínimo 2/3 por caso. Una suma total no compensa los requisitos por caso. Cada punto está motivado en EVALUACION.json.

El hallazgo no acredita ausencia total de trazabilidad ni justifica por sí solo una conclusión universal sobre el modelo. Tampoco acredita conformidad de esta entrega. No se sustituye, corrige ni reenvía la respuesta del participante.

## Secuencia

S8 registra y finaliza esta recepción complementaria del primer banco. S4/S5 y S7 conservan sus cierres y sus registros anteriores. Los cuatro segundos intentos siguen siendo los de Qwen, DeepSeek, Grok y Claude; Mistral sólo consta aquí en primer intento. Una eventual participación suya en la segunda prueba no está ejecutada ni se presupone.

Se mantiene la orden de completar registros antes de elegir candidato. La evaluación de licencias y de implantación independiente de la plataforma del proveedor queda después de la comparación. No se elige candidato en esta recepción. [Rectores cotejados](RECTORES.json). El laboratorio y calidad reciben el mismo expediente, en las ramas existentes.
