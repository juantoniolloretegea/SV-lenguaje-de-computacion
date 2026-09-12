# S1 · Cobertura de evidencia y comprobación independiente

**RETP-164 · 12 de septiembre de 2026. S1 finalizado en alcance documental acotado.**

La candidata conserva el cuerpo original y exige las dos piezas documentales fijadas por el contrato del ensayo. Una selección con el caso íntegro pero sin su evidencia de vigencia se rechaza, aunque el cuerpo entregable permanezca intacto. La referencia se obtiene del enlace confiable antes de recibir la selección y no puede construirse con ésta.

Este resultado acredita una comprobación documental concreta. **No cierra íntegramente C/I ni el conjunto A–L.** P3-04 es una petición ambigua y su resolución previa no consulta la política; su vigencia negativa permite probar la omisión de evidencia, pero no demuestra el efecto causal de una revocación aplicable. S2 conserva ese siguiente contraste, pendiente, dentro de la integración de adenda IA y frame.

## Qué debía ocurrir y qué ocurrió

| Comprobación | Resultado |
| --- | --- |
| Ruta previa RETP-152 | Entrega cuerpos íntegros y permite recuperar el montaje; no recibe ni comprueba la selección adicional de evidencias. No se refuta su contrato previo. |
| CI01 · P3-04 con las dos piezas | Entrega documental completa; cuerpo y citas conservados. |
| CI02 · Caso íntegro sin vigencia | Rechazo `FaltaVigencia`; propuesta y pieza omitida recuperables; no entrega acreditada con cobertura. |
| CI03 · Referencia reducida | Rechazo `IdentidadMontaje` por el enlace existente. |
| CI04 · Cita de vigencia alterada | Rechazo `VigenciaDistinta`. |
| CI05 · Falta el caso | Rechazo `FaltaCaso`. |
| CI06 · Identidad ajena | Rechazo heredado `Identidad`. |
| CI07 · JSON válido con contenido alterado | Rechazo heredado `ContenidoDistinto`. |
| CI08 · P3-01 completo | Entrega documental válida; control frente al rechazo indiscriminado. |

Se ejecutaron los ocho controles en tres repeticiones de depuración y tres de optimización: **48 observaciones de ocho controles**, sobre dos posiciones de cobertura. El enlace recorre las cuatro primeras posiciones por su orden constituido; las dos intermedias no son preguntas nuevas. Los cuatro cuerpos coinciden con sus huellas esperadas anteriores. Las 29 capturas por ejecución de cualificación son idénticas entre las seis ejecuciones.

Los dos controles de sensibilidad fueron detectados por CI02: suprimir la obligación de vigencia y reemplazar la referencia independiente por la selección. Dos compilaciones de cliente externo, una por modo, rechazaron la construcción directa de la referencia con E0451.

## Realización y responsabilidad

La cápsula de RETP-152 conserva sus 71 archivos, incluida la custodia, entrega literal, presentación y lectura vinculada. Se incorpora únicamente [cobertura.rs](codigo/cobertura.rs), una candidata documental de banco con referencias prestadas y dos piezas fijas. No asigna significado clínico, resuelve vigencia ni crea permiso. Las entradas del montaje usadas son P3-01 y P3-04; sus intervalos y huellas están en [BANCO_FIJADO.json](BANCO_FIJADO.json).

`Referencia::desde` obtiene la consulta, el caso y la entrada de montaje del enlace ya fijado. `Referencia::comprobar` conserva la comprobación previa de lectura y exige ambas piezas sin reconstruir ni completar las ausentes. La vista de entrega sólo se obtiene tras esas comprobaciones. El registro de propuesta y rechazo pertenece al conductor del ensayo; la protección material frente a un conductor que omita esa vía no se acredita.

Los nombres de causas son locales a la candidata. Las causas heredadas mantienen su origen y ninguna se convierte en U. Su correspondencia alimenta el catálogo durante la integración; no se ejecuta una ampliación general de diagnósticos ni se consolida ahora ese catálogo.

## Intentos conservados y corrección del instrumento

1. La primera invocación instrumental no pudo iniciar Rust porque `/usr/bin/time` no estaba disponible. Se conservó el intento y se emplearon `subprocess`, `time.monotonic` y `resource.getrusage` únicamente para ejecutar y medir procesos. No son una realización de la semántica SV.
2. El primer intento de cualificación se detuvo en CI07: el espécimen había roto la sintaxis JSON y no ejercitaba el cambio de contenido previsto. La observación puntual produjo `Lectura(Presentacion(Sintaxis(Json)))`. Se conservan fuente, manifiesto, salida y propuesta iniciales. La [adenda del instrumento](ADENDA_INSTRUMENTO_CI07.md) fija la corrección antes de reanudar: alterar una letra dentro de una cadena válida, manteniendo `ContenidoDistinto` como esperado. La candidata de cobertura y las fuentes heredadas no cambiaron.

No se presenta el primer intento como cualificación conforme. La cualificación descrita en la tabla corresponde íntegramente al instrumento corregido. Se registraron 30 invocaciones dentro del presupuesto, una sin productor iniciado; 29 procesos sí se iniciaron. Los rechazos intencionales de clientes y mutantes se distinguen de errores instrumentales y funcionales.

## Evidencia y reproducción

- [Contrato previo](CONTRATO_S1.md), [fuentes cotejadas](FUENTES_COTEJADAS.json) y [candidata identificada](CANDIDATA_FIJADA.json).
- [Observación de la base](OBSERVACION_BASE.json), [resultado](RESULTADO.json) y [comandos con salidas](COMANDOS.json).
- [Fuentes heredadas íntegras](BASE_RETP152.json), [código del contraste](codigo/contraste.rs) y [esperados previos del instrumento](codigo/ESPERADO.tsv).
- [Capturas, propuestas, rechazos y medidas](EVIDENCIAS_S1.json): contenido literal en base64, tamaño y SHA-256 por artefacto.
- [Reproductor de la cualificación](reproducir.py), que extrae las fuentes y conserva nuevas salidas en un directorio distinto.

Entorno observado: Rust 1.98.0 (`88d9e12ae178fab0fb5cc050a94da85685d449ea`), LLVM 22.1.8, `x86_64-unknown-linux-gnu`. Los comandos identifican las opciones de depuración y optimización. El campo de identidad `LG1_FUENTES` recibió la huella de `BANCO_FIJADO.json`, que identifica la cápsula fuente y los intervalos; `LG1_BINARIO`, la huella del ejecutable de cada proceso. No se atribuye a una huella legitimidad del conocimiento.

Las medidas conservan pared y CPU de hijos por diferencia de `getrusage`. RSS por proceso no estaba disponible y se declara ausente. Compilación, ejecución y rechazos quedan individualizados; estas medidas no acreditan coste de IA ni viabilidad P5. El reproductor se entrega como ayuda reproducible con fuentes fijadas; no se le atribuye una ejecución adicional en este expediente, cuyo presupuesto ya está consumido.

## Relevo único y pendientes conservados

**S2, pendiente:** fijar un par de una misma consulta inequívoca con vigencia positiva y negativa del banco, con montaje sucesor explícito si fuese necesario, y comprobar la dependencia causal antes de atribuir cobertura a una revocación aplicable. Se conservarán originales, criterio previo, referencia independiente y la diferencia respecto de P3-04. No se modifica retrospectivamente este banco para presentar su resultado como ese contraste causal.

C e I reciben evidencia parcial de selección y referencia; su cierre integrado no se acredita. A, B y D–L conservan los criterios y carencias de la matriz RETP-162. La salida realmente presentada y la revisión humana mantienen su obligación propia. La recepción profesional sigue condicionada a su premisa y verificadores, sin bloquear toda lectura documental. La custodia humana P3 ya recibida se conserva; no se vuelve a solicitar.

La prueba externa común sigue pendiente. No se han solicitado rondas de modelos ni abierto la reserva P3. P4, P5, auditoría P6 y continuación de fila 9 mantienen sus condiciones. Se conserva el orden: integración 1+3, recogida de errores durante el recorrido y consolidación posterior del catálogo. No se promueve código productivo ni se acredita WASI, navegador, persistencia o aislamiento material por este expediente.
