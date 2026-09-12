# Segundo intento · Trazabilidad íntegra

**Encargo listo para entrega.** Abra [PRUEBA_COMUN.md](PRUEBA_COMUN.md): contiene el contrato, el aviso de admisión y exclusión, todas las fuentes y reglas, los doce casos y la plantilla. Es idéntico para DeepSeek, Claude, Qwen y Grok y puede adjuntarse completo sin acceso al laboratorio.

Se exige el 100 % de las obligaciones de cada caso. El incumplimiento no se compensa con una puntuación media. El criterio de descarte general y la posible publicación internacional del expediente, incluidos Europa y Estados Unidos, constan antes de la participación. El dictamen identificará la evidencia y distinguirá el criterio de la dirección de los hechos demostrados.

## Cualificación realizada

[CUALIFICACION.json](CUALIFICACION.json) documenta 482 controles: ocho variantes válidas aceptadas, 472 entregas defectuosas rechazadas y dos alteraciones del instrumento detectadas como ERROR_INSTRUMENTO. También se ejecutó la interfaz del cotejador con una referencia válida y una causa incorrecta, con códigos de salida 0 y 2 respectivamente. Los tiempos de esas dos ejecuciones son medidas del receptor, no de participantes.

Se comprobó omisión y alteración de cada campo, cada fuente y cada regla, pérdida de límites, casos ausentes y duplicados, identidad, negación, permiso, límite de lectura, tipos, claves JSON duplicadas, codificación inválida, límites de tamaño y pertenencia al banco. Se aceptan órdenes de claves y colecciones distintos, escapes Unicode equivalentes y formato estructural permitido. La correspondencia de decisión y causa se contrastó con los doce resultados previos S4, derivados de los artefactos Rust S2/S3.

Una incidencia de preparación del guion de cualificación —un literal bytes con una tilde— se corrigió antes de ejecutar controles y se conserva en la custodia. No modificó el contrato, la referencia ni el verificador; no fue una respuesta de modelo ni un control de cualificación ejecutado.

## Material y comprobación

- [Contrato](CONTRATO.md), [banco](BANCO.json) y [plantilla de entrega](PLANTILLA_RESPUESTA.json).
- [Compromiso previo de huellas](COMPROMISO_PREVIO.json) y [manifiesto del paquete](MANIFIESTO.json).
- [Cotejador reproducible](cotejar_entrega.py). Requiere Python 3 y biblioteca estándar; compara archivos contra referencia fijada, sin ejecutar ni redefinir la semántica del SV.
- [Plantilla opcional de actividad](PLANTILLA_ACTIVIDAD.json). Las operaciones adicionales necesitan registros adjuntos para acreditarse; una declaración o ruta local no equivale a un registro recibido.

El evaluador usa la referencia privada custodiada antes de publicar el encargo:

```sh
python3 cotejar_entrega.py --respuesta RESPUESTA.json --banco BANCO.json --referencia REFERENCIA_PREVIA.json --compromiso COMPROMISO_PREVIO.json --salida DICTAMEN.json
```

Salida 0: CONFORME_DOCUMENTAL. Salida 2: NO_CONFORME con rutas de incumplimiento. Salida 3: ERROR_INSTRUMENTO; se suspende su uso y no se imputa al participante. Para la vía documental no se necesita ejecutar esta orden ni acceder a la referencia privada.

La referencia, procedencia, controles completos y salidas de cualificación se custodian en el laboratorio. El compromiso permite comprobar que no cambian después de recibir respuestas. La primera campaña y sus originales se conservan como antecedentes; este nuevo contrato no se aplica retrospectivamente.

La cualificación acredita este instrumento documental frente a los controles publicados. La trazabilidad de operaciones adicionales se revisa con sus registros. No se atribuye al cotejador conocimiento de procesos internos, control del host ni certificación universal de un modelo. No hay todavía respuestas del segundo intento.


[Constancia de custodia previa](CUSTODIA_PREVIA.json): commit `e2ef439e22583dac0f9b1c4fe4b38ce4bc5e0576` en la rama existente del laboratorio, verificado antes de la publicación de este paquete.
