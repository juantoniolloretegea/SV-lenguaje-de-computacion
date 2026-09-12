# S5 · Recepción y evaluación de la prueba externa

RETP-2026-168 · Registro UTC: 2026-09-12T12:25:53Z · Responsable: Watson / W-S0.

**S5 en ejecución.** Recibidos los archivos de DeepSeek, Claude y Qwen. Sus cotejos iniciales quedan documentados. Grok dispone de acceso según el usuario; su encargo está preparado y pendiente de envío. La recepción de estos archivos no equivale a aceptación ni cierra S5.

| Participante | Evidencia recibida | Resultado del cotejo inicial | Continuidad |
| --- | --- | --- | --- |
| [DeepSeek](deepseek/README.md) | Texto original, JSON extraído y declaración de lectura documental | E10: FaltaCaso en lugar de FaltaVigencia; rechazo conservado. Trazabilidad 35/36. | Revisión de etiquetas y eventual aclaración, conservando intento 1. |
| [Claude](claude/README.md) | JSON original, referencias y actividad declarada | E05: afirma conversión en DATO que el caso no demuestra. Trazabilidad 35/36. | Cotejo documental con exposición previa alta declarada; fuera de clasificación comparativa. |
| [Qwen](qwen/README.md) | Texto original, JSON extraído y declaración documental | Causas técnicas sustituidas por descripciones; traza 23/36, insuficiente en E11/E12. | Aclaraciones y revisión común; original preservado. |
| [Grok](ENCARGO_GROK.md) | Sin respuesta | Acceso comunicado por el usuario; encargo no enviado | Entregar el mismo documento fijado y registrar envío/recepción. |

[Recepciones estructuradas](RECEPCIONES.json) · [Cotejo independiente de bytes y huellas](COTEJO_RECEPCION.json) · [Discrepancia común de etiquetas](INCIDENCIA_ETIQUETAS.md).

## Evaluación y límites

Se ha aplicado la [rúbrica previamente fijada](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/e1dcd9f5df30a1bbbfd446c7c4e5578d0a08a75c/pruebas-externas/s4-recorrido-documental/RUBRICA.md) contra el oráculo comprometido antes de publicar S4. Las fichas separan resultado literal, evidencia, consecuencia, traza, procedimiento y entrega. La discrepancia entre etiquetas públicas y oráculo deja pendiente la puntuación definitiva; no oculta los defectos materiales señalados. No se ha acreditado ninguna incidencia crítica de las enumeradas por la rúbrica.

Las tres respuestas aportan referencias y explicaciones cotejables; la suficiencia por caso se detalla en sus fichas. Eso no acredita ejecución propia de Rust, que las tres niegan. El receptor ha recalculado los datos citados; la ejecución de herramientas que cada participante declara requiere sus registros para ser auditada. Un valor coincidente no prueba por sí solo quién lo calculó.

La duración total no está medida. El orden comunicado por el usuario es DeepSeek y después Claude, y posteriormente Qwen; no permite una clasificación de rapidez ni esfuerzo. La ausencia de exposición previa de otros participantes no se deduce de la declaración de Claude.

## Secuencia

S4 permanece finalizado como preparación del [encargo común inmutable](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/e1dcd9f5df30a1bbbfd446c7c4e5578d0a08a75c/pruebas-externas/s4-recorrido-documental/PRUEBA_COMUN.md). S5 recibe, conserva y evalúa los originales con el mismo banco. Siguen pendientes el encargo efectivo a Grok y la revisión común de etiquetas y aceptación. Las correcciones serán intentos adicionales. Continúan la integración 1+3, el registro de causas durante el trabajo y la consolidación posterior del catálogo. No se abre reserva P3 ni se da por cerrado el alcance integral C/I/D o A–L.

Los originales, recibos y evaluaciones se conservan en laboratorio y en su espejo de Calidad. La prueba de SVcustos conserva el commit e1dcd9f5df30a1bbbfd446c7c4e5578d0a08a75c. Mismas ramas existentes.

## Corte de recepción

Lenguaje: aba9f97839f2a16e0d6d83da0a959897c69c8742, main. Laboratorio: dfc71934c1887de869a925b93c79cc3f7c7984df, lab/playground-sv-permanente. Paquete de SVcustos: e1dcd9f5df30a1bbbfd446c7c4e5578d0a08a75c, main.

AGENTS.md y las piezas rectoras de pilares (05/09), perfiles y ensamblaje (06/09), y transición desde OP-IMM-001 con sus adendas, consultadas en esta secuencia, se han cotejado sin cambios con el corte del Lenguaje. Esta recepción no modifica semántica, contratos ni pruebas funcionales. [Huellas de las piezas rectoras](RECTORES_COTEJADOS.json).
