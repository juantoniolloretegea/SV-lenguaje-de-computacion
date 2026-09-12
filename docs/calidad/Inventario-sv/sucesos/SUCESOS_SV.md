# Sucesos SV — estado vigente

[Reglas del registro](README.md) · [CSV](SUCESOS_SV.csv) · [Historial de actualizaciones](HISTORIAL_SUCESOS_SV.csv)

Fechas en UTC. Cada ficha corresponde a la última revisión del suceso; las revisiones anteriores se conservan en el historial.

| Suceso | Estado | Actividad | Responsable | Última actualización |
| --- | --- | --- | --- | --- |
| S0 | finalizado | Puesta en servicio del registro Sucesos SV | Watson / W-S0 | 2026-09-12T10:23:08Z |
| S1 | finalizado | Contraste C/I de cobertura y comprobación independiente | Watson / W-S0 | 2026-09-12T10:51:15Z |
| S2 | finalizado | Contraste causal de vigencia con consulta inequívoca | Watson / W-S0 | 2026-09-12T11:28:10Z |
| S3 | en ejecución | Correspondencia de presentación y pérdida de negación | Watson / W-S0 | 2026-09-12T11:38:45Z |

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

**Estado:** en ejecución

**Alta:** 2026-09-12T11:28:10Z

**Inicio:** 2026-09-12T11:38:45Z

**Actualización:** 2026-09-12T11:38:45Z

**Fin:** —

**Responsable:** Watson / W-S0

**Alcance:** Criterio D de la integración adenda IA y frame: precisar el objeto efectivamente presentado y un testigo de pérdida de negación; reutilizar entrega y lectura existentes.

**Repositorios y ramas:** SV-lenguaje-de-computacion: main; SV-matematica-semantica-cuaternaria: lab/playground-sv-permanente

**Cortes de entrada:** Lenguaje 1430f64869b341ad0795588db425fb69e65e06cb; laboratorio 84a30056bc9707376b962b58fc6966ffee16b092

**Dependencias:** S2; criterio D de RETP-147/162; contratos de RETP-149/150/152

**Resultado:** Alcance, ocho controles, especímenes y esperados fijados antes de la cualificación.

**Verificación:** Fuentes rectoras cotejadas; 73 fuentes S2 reutilizadas intactas; diferencia frente a RETP-150/152 delimitada; preparación previa conservada.

**Evidencias:** [Referencia 1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores/s3-presentacion-y-negacion/CONTRATO_S3.md)

**Referencia de calidad:** [Referencia 1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-166)

**Siguiente acción:** Ejecutar el contraste integrado de cobertura, escritura y recuperación del archivo, con dos sensibilidades.

**Observaciones:** Pérdida previa debe impedir escritura; alteración posterior debe detectarse al recuperar. Sin pantalla, revisión humana, host adverso ni reserva externa.

