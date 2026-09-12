# Sucesos SV — estado vigente

[Reglas del registro](README.md) · [CSV](SUCESOS_SV.csv) · [Historial de actualizaciones](HISTORIAL_SUCESOS_SV.csv)

Fechas en UTC. Cada ficha corresponde a la última revisión del suceso; las revisiones anteriores se conservan en el historial.

| Suceso | Estado | Actividad | Responsable | Última actualización |
| --- | --- | --- | --- | --- |
| S0 | finalizado | Puesta en servicio del registro Sucesos SV | Watson / W-S0 | 2026-09-12T10:23:08Z |
| S1 | finalizado | Contraste C/I de cobertura y comprobación independiente | Watson / W-S0 | 2026-09-12T10:51:15Z |
| S2 | finalizado | Contraste causal de vigencia con consulta inequívoca | Watson / W-S0 | 2026-09-12T11:28:10Z |
| S3 | finalizado | Correspondencia de presentación y pérdida de negación | Watson / W-S0 | 2026-09-12T11:42:55Z |
| S4 | finalizado | Preparación de la prueba externa común del recorrido documental | Watson / W-S0 | 2026-09-12T12:03:04Z |
| S5 | finalizado | Recepción y evaluación de la prueba externa común | Watson / W-S0 | 2026-09-12T14:48:43Z |
| S6 | finalizado | Diseño y cualificación del segundo intento de trazabilidad íntegra | Watson / W-S0 | 2026-09-12T15:02:14Z |
| S7 | pendiente | Recepción del segundo intento de trazabilidad íntegra | Watson / W-S0 | 2026-09-12T15:02:14Z |

## S0 · Puesta en servicio del registro Sucesos SV

**Estado:** finalizado

**Alta:** 2026-09-12T10:21:21Z

**Inicio:** 2026-09-12T10:21:21Z

**Actualización:** 2026-09-12T10:23:08Z

**Fin:** 2026-09-12T10:23:08Z

**Responsable:** Watson / W-S0

**Alcance:** Registro incremental desde S0; tres estados; historial de actualizaciones; entrada obligatoria en Léame primero; continuidad de registros de calidad y espejo; retirada del archivo de soporte inicio.md de sucesos.

**Repositorios y ramas:** SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente

**Cortes de entrada:** Lenguaje be9e5e4d4041223223bd7d0dc38e1851414fd46a; laboratorio dd6d563ea7b75dd1d420f1fe542334b224a55886

**Dependencias:** —

**Resultado:** Registro puesto en servicio desde S0, con S1 pendiente, historial de cambios y entrada obligatoria en Léame primero. Retirado el archivo de soporte de la carpeta de sucesos.

**Verificación:** Publicaciones de apertura cotejadas: cuatro archivos del registro idénticos entre repositorios; Léame primero idéntico; historial RETP previo conservado; inicio.md de sucesos ausente; sin cambios ajenos al alcance.

**Evidencias:** [Referencia 1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/06da92137bcac75f3ae4b23ca059550fdf0a6a6c) · [Referencia 2](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/commit/c237e9d6dc06f481f75a3a874b3bfcf411a4ba32)

**Referencia de calidad:** [Referencia 1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-163)

**Siguiente acción:** Continuar S1 conforme a RETP-162 y actualizar su estado antes de iniciar.

**Observaciones:** Actividad documental. No se han ejecutado nuevos contrastes funcionales. W-S0 identifica de forma estable a la unidad responsable de esta apertura.


## S1 · Contraste C/I de cobertura y comprobación independiente

**Estado:** finalizado

**Alta:** 2026-09-12T10:21:21Z

**Inicio:** 2026-09-12T10:37:36Z

**Actualización:** 2026-09-12T10:51:15Z

**Fin:** 2026-09-12T10:51:15Z

**Responsable:** Watson / W-S0

**Alcance:** Integración de adenda IA y frame: selección completa frente a omisión de evidencia requerida con citas verdaderas; reutilización de custodia, entrega y lectura existentes.

**Repositorios y ramas:** SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente

**Cortes de entrada:** Lenguaje 428d294c42602bc116fa0550262c25730017492e; laboratorio 0d6b386121a24e915ce550dc2b52186867f6fdd4; cápsula RETP-152 cotejada

**Dependencias:** S0; contrato y criterios C/I de RETP-162

**Resultado:** Cobertura documental de dos piezas conforme en dos posiciones del banco. Ocho controles en seis ejecuciones; no cierre integral C/I ni A–L.

**Verificación:** 48 observaciones de ocho controles; cuatro cuerpos previos preservados; 29 capturas idénticas entre ejecuciones; dos sensibilidades detectadas y dos clientes forjados rechazados. Intentos instrumentales conservados.

**Evidencias:** [Referencia 1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores/s1-cobertura-independiente/README.md)

**Referencia de calidad:** [Referencia 1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-164)

**Siguiente acción:** S2: fijar contraste causal de vigencia con una misma consulta inequívoca y montaje explícito.

**Observaciones:** P3-04 devuelve PETICION_AMBIGUA y no consulta la política; no acredita revocación aplicable. Sin modelos externos ni reserva abierta. Candidata nativa de laboratorio; sin promoción productiva.


## S2 · Contraste causal de vigencia con consulta inequívoca

**Estado:** finalizado

**Alta:** 2026-09-12T10:51:15Z

**Inicio:** 2026-09-12T11:23:19Z

**Actualización:** 2026-09-12T11:28:10Z

**Fin:** 2026-09-12T11:28:10Z

**Responsable:** Watson / W-S0

**Alcance:** Continuar C/I: mismo encargo inequívoco con vigencia positiva y negativa; cobertura de evidencia, acceso independiente y diferencia de resolución bajo un montaje declarado.

**Repositorios y ramas:** SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente

**Cortes de entrada:** Lenguaje 111b4e8fc24481cc76be347e724d027e1c636973; laboratorio 35ea2ba7b397d7b8813be82ec8caeff4ff377bad

**Dependencias:** S1; criterios C/I de RETP-147 y relevo RETP-162

**Resultado:** Consulta idéntica bajo dos vigencias: DATO/8.40 frente a PERMISO_REVOCADO/null, con mismo significado y una llamada de política. Cobertura documental conforme en el par integrado.

**Verificación:** Doce controles en seis ejecuciones: 72 observaciones; 42 capturas idénticas por ejecución; diez cuerpos históricos conservados; dos sensibilidades detectadas; 23 invocaciones sin incidencias adicionales.

**Evidencias:** [Referencia 1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores/s2-vigencia-causal/README.md)

**Referencia de calidad:** [Referencia 1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-165-cierre-s2)

**Siguiente acción:** S3 pendiente: correspondencia de presentación y pérdida de negación, reutilizando RETP-149/150/152.

**Observaciones:** Causalidad de instantáneas sintéticas acreditada en banco nativo; no revocación profesional, cierre universal C/I o A–L, prueba externa ni P4/P5/P6. S1 y sus fuentes originales se conservan.


## S3 · Correspondencia de presentación y pérdida de negación

**Estado:** finalizado

**Alta:** 2026-09-12T11:28:10Z

**Inicio:** 2026-09-12T11:38:45Z

**Actualización:** 2026-09-12T11:42:55Z

**Fin:** 2026-09-12T11:42:55Z

**Responsable:** Watson / W-S0

**Alcance:** Criterio D de la integración adenda IA y frame: precisar el objeto efectivamente presentado y un testigo de pérdida de negación; reutilizar entrega y lectura existentes.

**Repositorios y ramas:** SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente

**Cortes de entrada:** Lenguaje 1430f64869b341ad0795588db425fb69e65e06cb; laboratorio 84a30056bc9707376b962b58fc6966ffee16b092

**Dependencias:** S2; criterio D de RETP-147/162; contratos de RETP-149/150/152

**Resultado:** Presentación íntegra escrita y recuperada; pérdida previa de negación rechazada sin crear archivo; alteración posterior detectada sobre los bytes recuperados, con referencia original intacta.

**Verificación:** Ocho controles en seis ejecuciones: 48 observaciones; 28 capturas idénticas por ejecución; dos sensibilidades detectadas; 22 invocaciones sin fallo inesperado de cualificación.

**Evidencias:** [Referencia 1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores/s3-presentacion-y-negacion/README.md)

**Referencia de calidad:** [Referencia 1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-166-cierre-s3)

**Siguiente acción:** S4 pendiente: preparar la prueba externa común del recorrido documental con instrucción, acceso y esperados fijados.

**Observaciones:** Alcance de archivo y proceso confiable; no pantalla, revisión humana, resistencia al host ni cierre integral D/C/I o A–L. Preparación anterior a la fijación conservada; fuentes S2 intactas.


## S4 · Preparación de la prueba externa común del recorrido documental

**Estado:** finalizado

**Alta:** 2026-09-12T11:42:55Z

**Inicio:** 2026-09-12T11:58:46Z

**Actualización:** 2026-09-12T12:03:04Z

**Fin:** 2026-09-12T12:03:04Z

**Responsable:** Watson / W-S0

**Alcance:** Un mismo encargo sobre el recorrido delimitado de lectura, cobertura y presentación; fuentes públicas accesibles, esperados previamente custodiados y formato de recepción comparable.

**Repositorios y ramas:** SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente; SVcustos-dataset: main

**Cortes de entrada:** Lenguaje d2fee48ce16cc5585d6d7d681c6ee0538246a113; laboratorio 30591fbf939627688c134d774029a0e9b365f497; SVcustos 5e0ed66c62fe753b6e94af500ec00f2674fb92e7

**Dependencias:** S1/S2/S3; RETP-147/162; condiciones de acceso y prueba externa del workflow

**Resultado:** Paquete autosuficiente publicado en SVcustos main; doce casos con oráculo previo custodiado, rúbrica de 100 puntos y plantillas de respuesta/medición.

**Verificación:** 12/12 resultados previos cotejados; 51 archivos de reproducción preservados; descarga sin credenciales HTTP 200 del documento y ZIP, con huellas y bytes coincidentes.

**Evidencias:** [Referencia 1](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/e1dcd9f5df30a1bbbfd446c7c4e5578d0a08a75c/pruebas-externas/s4-recorrido-documental/PRUEBA_COMUN.md) · [Referencia 2](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/e1dcd9f5df30a1bbbfd446c7c4e5578d0a08a75c/pruebas-externas/s4-recorrido-documental/RUBRICA.md)

**Referencia de calidad:** [Referencia 1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-167-cierre-s4)

**Siguiente acción:** S5 pendiente: recibir y evaluar respuestas originales, trazabilidad y mediciones por participante.

**Observaciones:** Oráculo publicado primero en custodia privada; compromiso público y acceso sin credenciales verificados. Lectura efectiva de cada participante pendiente; no se han enviado mensajes ni recibido respuestas externas.


## S5 · Recepción y evaluación de la prueba externa común

**Estado:** finalizado

**Alta:** 2026-09-12T12:03:04Z

**Inicio:** 2026-09-12T12:25:53Z

**Actualización:** 2026-09-12T14:48:43Z

**Fin:** 2026-09-12T14:48:43Z

**Responsable:** Watson / W-S0

**Alcance:** Confirmación de acceso y conservación de respuestas originales; cotejo de E01–E12 contra oráculo previo; puntuación de resultado y trazabilidad; mediciones separadas por participante e intento.

**Repositorios y ramas:** SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente; SVcustos-dataset: main

**Cortes de entrada:** SVcustos e1dcd9f5df30a1bbbfd446c7c4e5578d0a08a75c; S4 / RETP-167; oráculo y rúbrica comprometidos antes de recepción.

**Dependencias:** S4; respuesta efectiva por el canal del encargo; revisión humana de aceptación

**Resultado:** Recepción y cotejo inicial de cuatro respuestas completos. El primer instrumento no acredita trazabilidad íntegra: umbral parcial y ambigüedades de etiquetas. Se conserva como antecedente y no fundamenta por sí solo exclusión general.

**Verificación:** Cuatro originales y evaluaciones conservados; oráculo S4 intacto. Los reparos se mantienen como hallazgos de las entregas. No se convierte la puntuación parcial en acreditación de trazabilidad total.

**Evidencias:** [Referencia 1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores/s5-recepcion-externa/grok/README.md)

**Referencia de calidad:** [Referencia 1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-169-recepcion-grok)

**Siguiente acción:** S6: preparar y cualificar el segundo instrumento por instrucción del usuario; exigir cumplimiento íntegro antes de valorar la nueva entrega.

**Observaciones:** Finalización del alcance de recepción y balance del primer instrumento; no aceptación de los modelos ni conformidad global. La dirección encomienda al responsable el nuevo diseño y su validación. No se exige al usuario revisar diseños.


## S6 · Diseño y cualificación del segundo intento de trazabilidad íntegra

**Estado:** finalizado

**Alta:** 2026-09-12T14:48:43Z

**Inicio:** 2026-09-12T14:48:43Z

**Actualización:** 2026-09-12T15:02:14Z

**Fin:** 2026-09-12T15:02:14Z

**Responsable:** Watson / W-S0

**Alcance:** Contrato inequívoco de entrega documental; referencia previa; cotejo automático contra material fijado; controles positivos y negativos; paquete común público.

**Repositorios y ramas:** SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente; SVcustos-dataset: main

**Cortes de entrada:** Lenguaje 0820653780d362996768065b6b50ffff0859220b; laboratorio 9a23aa356b94b2e000c7f73b8d036394a0ef7657; SVcustos e1dcd9f5df30a1bbbfd446c7c4e5578d0a08a75c.

**Dependencias:** S5; instrucción del usuario de exigir trazabilidad completa y validar el instrumento antes de la nueva entrega.

**Resultado:** Segundo contrato inequívoco publicado con exigencia íntegra, referencia previa, cotejador y advertencia de exclusión/publicación. Instrumento cualificado para su alcance documental y listo para entrega.

**Verificación:** 482/482 controles previstos: 8 variantes válidas aceptadas; 472 defectos rechazados; 2 alteraciones del instrumento detectadas. Dos ejecuciones CLI con salidas 0/2. Referencia contrastada con oráculo previo; descarga pública sin credenciales HTTP 200 y bytes idénticos.

**Evidencias:** [Referencia 1](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/fccde9cf524a0d62b2dd1a2ee05170d6f4358074/pruebas-externas/s6-trazabilidad-total/PRUEBA_COMUN.md) · [Referencia 2](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/fccde9cf524a0d62b2dd1a2ee05170d6f4358074/pruebas-externas/s6-trazabilidad-total/CUALIFICACION.json) · [Referencia 3](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/fccde9cf524a0d62b2dd1a2ee05170d6f4358074/pruebas-externas/s6-trazabilidad-total/CUSTODIA_PREVIA.json)

**Referencia de calidad:** [Referencia 1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-170-cierre)

**Siguiente acción:** S7 pendiente: recepción del segundo intento de los cuatro participantes y aplicación íntegra del contrato fijado.

**Observaciones:** La validación del diseño fue realizada por Watson / W-S0. Cotejador Python de archivos y referencia; no autoridad semántica ni sustitución de Rust. Incidencia de preparación conservada. S4 y cuatro originales S5 intactos; contrato nuevo sin aplicación retrospectiva.


## S7 · Recepción del segundo intento de trazabilidad íntegra

**Estado:** pendiente

**Alta:** 2026-09-12T15:02:14Z

**Inicio:** —

**Actualización:** 2026-09-12T15:02:14Z

**Fin:** —

**Responsable:** Watson / W-S0

**Alcance:** Mismo documento fijado para DeepSeek, Claude, Qwen y Grok; preservación de respuestas originales; cotejo íntegro por obligación; comprobación de cualquier afirmación adicional de actividad; dictamen trazable.

**Repositorios y ramas:** SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente; SVcustos-dataset: main

**Cortes de entrada:** SVcustos fccde9cf524a0d62b2dd1a2ee05170d6f4358074; referencia privada e2ef439e22583dac0f9b1c4fe4b38ce4bc5e0576; S6 / RETP-170.

**Dependencias:** S6 cualificado; entrega íntegra y recepción efectiva de respuestas del segundo intento.

**Resultado:** Ninguna segunda respuesta recibida; sin dictámenes de participantes.

**Verificación:** Documento público descargado sin credenciales: HTTP 200; 29890 bytes y SHA-256 coincidentes.

**Evidencias:** [Referencia 1](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/fccde9cf524a0d62b2dd1a2ee05170d6f4358074/pruebas-externas/s6-trazabilidad-total/PRUEBA_COMUN.md)

**Referencia de calidad:** [Referencia 1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-170-cierre)

**Siguiente acción:** Entregar el mismo documento íntegro a los cuatro participantes; conservar cada devolución como segundo intento y cotejar contra la referencia previamente fijada.

**Observaciones:** Advertencia explícita de descarte general por la dirección del SV y posible difusión internacional del expediente. Sin revisión del diseño exigida al usuario. Ninguna medida ni actividad de participante se presume realizada.

