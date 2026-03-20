# Registro de Evolución Técnica del Proyecto

## 1. Naturaleza y subordinación

Este registro tiene carácter operativo y subordinado.
No sustituye a la doctrina vigente del Sistema Vectorial SV ni al historial de commits.
Su función es conservar, por hitos, la trazabilidad técnica mínima necesaria para reconstruir la evolución del proyecto con criterios de calidad, evidencia y congruencia.

## 2. Objeto

Registrar cambios relevantes del desarrollo técnico que alteren el estado gobernable del proyecto.

## 3. Regla de inclusión

Debe registrarse un hito cuando cambie de forma relevante alguno de estos planos:

- apertura o cierre de parche;
- creación de paquete de trabajo con efecto estructural;
- cambio en validator, runner, catálogo de errores o suite;
- cambio en documentación pública de estado;
- cambio en especificación operativa ya cerrada;
- sorpresa técnica real;
- cierre de fase o apertura de nueva fase;
- decisión arquitectónica relevante.

## 4. Regla de exclusión

No deben registrarse:

- microcambios sin efecto estructural;
- correcciones cosméticas sin impacto de contrato;
- pasos mecánicos intermedios de publicación;
- reintentos materiales que no cambien el estado del proyecto;
- no deben registrarse anotaciones sin relevancia técnica, documental o de control.

## 5. Regla temporal

Toda entrada debe abrirse dentro de la ventana del hito correspondiente y quedar cerrada, como máximo, antes de:

- abrir el siguiente frente estructural, o
- declarar el hito definitivamente cerrado.

## 6. Revisión en puntos de control

El registro deberá revisarse de forma dura al menos en estos casos:

- cierre de parche estructural;
- sorpresa técnica real con impacto relevante;
- cambio de catálogo de errores;
- cierre de fase;
- paso a nueva fase.

## 7. Campos mínimos obligatorios

Toda entrada deberá incluir, como mínimo:

- ID
- Fecha
- Hora (Europe/Madrid)
- Tipo de hito
- Frente / parche / fase
- Resumen del cambio
- Motivo o argumento
- Base doctrinal o técnica invocada
- Artefactos afectados
- Evidencia
- Impacto
- Objeción adversarial considerada
- Decisión
- Estado

## 8. Tipos de hito recomendados

Se recomienda usar, de forma estable, alguno de estos tipos:

- ZIP
- APERTURA_PARCHE
- CIERRE_PARCHE
- CAMBIO_VALIDATOR
- CAMBIO_RUNNER
- CAMBIO_CATALOGO_ERRORES
- CAMBIO_DOCUMENTACION_PUBLICA
- SORPRESA_TECNICA
- CIERRE_FASE
- APERTURA_FASE
- DECISION_ARQUITECTONICA

## 9. Estados recomendados

- abierto
- cerrado
- pendiente_revision
- trasladado
- sustituido

## 10. Impacto recomendado

El impacto deberá expresarse con lenguaje breve, estable y técnico.
Se recomienda usar una o varias de estas etiquetas:

- adecuacion_funcional
- fiabilidad
- mantenibilidad
- seguridad
- compatibilidad
- trazabilidad
- gobierno_tecnico

## 11. Tabla maestra

| ID | Fecha | Hora (Europe/Madrid) | Tipo de hito | Frente/Fase | Resumen del cambio | Estado |
|---|---|---:|---|---|---|---|
| RETP-2026-001 | 13/03/2026 | 19:08:39 | CIERRE_PARCHE | PARCHE 2E-B / Fase IV | Se impone la alternancia constitutiva de `Trajectory`, se adapta la línea SEC-0 afectada, se añaden casos positivo y negativo de conformidad y se actualiza la suite a 17/17. | cerrado |
| RETP-2026-002 | 13/03/2026 | 20:12:56 | DECISION_ARQUITECTONICA | Gobierno técnico / Fase IV | Se fija la jerarquía operativa del proyecto, el control dual ligero/duro y la prioridad de sanear la concordancia IR ↔ catálogo de errores ↔ validator ↔ runner ↔ documentación pública antes de abrir nuevos frentes funcionales o backend Rust. | cerrado |
| RETP-2026-003 | 13/03/2026 | 23:40:35 | CAMBIO_DOCUMENTACION_PUBLICA | Microauditoría de concordancia / Fase IV | Se sinceran la cobertura declarada del validator, la descripción pública de emisión/cobertura de errores y la sincronización dual del registro técnico `.md`/`.csv`, sin abrir todavía un frente funcional sobre N4/Uso. | cerrado |
| RETP-2026-004 | 14/03/2026 | 18:36:25 | CAMBIO_DOCUMENTACION_PUBLICA | Beta / espejo doctrinal controlado / Fase IV | Se activa el espejo controlado entre `especificaciones/` y `beta/`, se alojan la matriz operativa y la publicación de frontera 0-1 en sus rutas diferenciadas y se actualizan los índices públicos del repositorio doctrinal y del repositorio del lenguaje sin alterar la jerarquía normativa. | cerrado |
| RETP-2026-005 | 14/03/2026 | 21:10:21 | CIERRE_PARCHE | MICROPARCHE B / declaración operativa de rutas / Fase IV | Se registra el cierre del microparche B en el repositorio del lenguaje: declaración explícita de Ruta A / Ruta B / Beta en la matriz operativa, en `beta/README.md` y en `README.md`, con precisión expresa de que la triada no obliga a sincronía conceptual completa con el repositorio doctrinal. | cerrado |
| RETP-2026-008 | 18/03/2026 | 16:31:19 | DECISION_ARQUITECTONICA | Continuidad del frente básico y habilitación del backend / Fase IV | Se declara públicamente que la pendencia material de los sentidos restantes del frente básico no bloquea el avance del lenguaje SV hacia backend, siempre que el núcleo preserve puertos de extensión previstos y se mantenga la subordinación doctrinal y técnica vigente. | cerrado |

| RETP-2026-018 | 19/03/2026 | 13:20:00 | CAMBIO_DOCUMENTACION_PUBLICA | Frente final del lenguaje SV / Bloque A — contrato diagnóstico / sincronización pública de suite y catálogo / Fase IV | Se sincronizan la documentación pública del catálogo y de la suite con el estado ya materializado de E007, E008, E009 y E507. | cerrado |
| RETP-2026-020 | 19/03/2026 | 14:05:00 | CAMBIO_DOCUMENTACION_PUBLICA | Frente final del lenguaje SV / Bloque A — contrato diagnóstico / sinceramiento de cobertura explícita de E001 / Fase IV | Se retira E001 de la lista de códigos cubiertos explícitamente por la suite y se documenta que conserva emisión observable sin caso adversarial explícito propio tras la extracción local hacia E507. | cerrado |

## 12. Entradas detalladas

### RETP-2026-001

- **Fecha:** 13/03/2026
- **Hora (Europe/Madrid):** 19:08:39
- **Tipo de hito:** CIERRE_PARCHE
- **Frente/Fase:** PARCHE 2E-B / Fase IV
- **Resumen del cambio:** Se incorpora validación mínima y explícita de la alternancia constitutiva en trayectorias; se adapta el caso adversarial afectado; se añaden nuevos casos de conformidad; se actualiza la documentación de la suite.
- **Motivo o argumento:** La alternancia de `Trajectory` constaba en la IR, pero no estaba impuesta todavía por el validator.
- **Base doctrinal o técnica invocada:** IR canónica v0.2; gramática superficial mínima v0.1; baseline SEC-0.
- **Artefactos afectados:** validator; runner de conformidad; casos de test; documentación de tests.
- **Evidencia:** verificación directa del parche; resultados internos comprobados; evidencias públicas de actualización de archivos.
- **Impacto:** adecuacion_funcional; trazabilidad; gobierno_tecnico.
- **Objeción adversarial considerada:** riesgo de reducir el parche a maquillaje de tests y riesgo de romper la línea adversarial previa.
- **Decisión:** cerrar el hueco mediante enforcement mínimo real y cobertura explícita de suite.
- **Estado:** cerrado

### RETP-2026-002

- **Fecha:** 13/03/2026
- **Hora (Europe/Madrid):** 20:12:56
- **Tipo de hito:** DECISION_ARQUITECTONICA
- **Frente/Fase:** Gobierno técnico / Fase IV
- **Resumen del cambio:** Se fija un marco de continuidad operativa por jerarquías, bloques cerrables, control dual y subordinación expresa de la capa operativa al repositorio doctrinal.
- **Motivo o argumento:** El proyecto ha alcanzado suficiente madurez técnica y metodológica como para requerir una capa mínima de gobierno técnico sin duplicación doctrinal ni burocratización.
- **Base doctrinal o técnica invocada:** repositorio doctrinal vigente; Guía práctica del conocimiento profundo y la crítica de la razón pura; consensos operativos cerrados del frente del lenguaje.
- **Artefactos afectados:** metodología de trabajo; control de cambios; auditoría; registro técnico; planificación de siguientes frentes.
- **Evidencia:** debate técnico consolidado y punto de restauración formal fijado.
- **Impacto:** trazabilidad; gobierno_tecnico; mantenibilidad.
- **Objeción adversarial considerada:** riesgo de burocratización y de creación de una segunda capa doctrinal paralela.
- **Decisión:** adoptar una capa operativa subordinada, ligera durante la marcha y dura sólo en puntos de control.
- **Estado:** cerrado

### RETP-2026-003

- **Fecha:** 13/03/2026
- **Hora (Europe/Madrid):** 23:40:35
- **Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA
- **Frente/Fase:** Microauditoría de concordancia / Fase IV
- **Resumen del cambio:** Se ajusta la documentación pública para distinguir entre validación implementada y concordancia fuerte total; se explicita la diferencia entre catálogo, emisión observable y cobertura de suite; y se sincroniza el registro dual `.md`/`.csv`.
- **Motivo o argumento:** La microauditoría ha constatado que el frontend está estable, pero la documentación pública sobreatribuía cobertura del validator y no reflejaba con precisión el estado observable del contrato diagnóstico ni la asimetría entre registro en prosa y registro tabular.
- **Base doctrinal o técnica invocada:** IR canónica v0.2; decisión C1C de regularización por Vía B; procedimiento de auditoría técnica del proyecto; suite y código fuente observables del frontend.
- **Artefactos afectados:** `src/svp_validator.py`; `src/README.md`; `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
- **Evidencia:** microauditoría cerrada de concordancia; contraste directo entre código fuente, suite observable y documentación pública; actualización sincronizada de los dos registros técnicos.
- **Impacto:** trazabilidad; gobierno_tecnico; mantenibilidad.
- **Objeción adversarial considerada:** riesgo de convertir una microauditoría en pseudoavance sin corrección funcional real y riesgo de petrificar provisionalmente la ausencia de validación fuerte en N4/Uso.
- **Decisión:** sincerar primero el estado público verificable y reservar para bloque posterior, separado y adversarial, cualquier cierre funcional sobre N4/Uso o convergencia fuerte total.
- **Estado:** cerrado

## 13. Regla de estilo

Este registro debe mantenerse con lenguaje formal, sobrio, técnico y revisable por terceros. Las entradas deben describir hechos, fundamento, evidencia, impacto y estado.

## 14. Vigencia

Este registro permanecerá vigente mientras el proyecto requiera trazabilidad técnica ligera y controlada de su evolución, sin perjuicio de futuras versiones más estructuradas si la complejidad del sistema lo exigiera.

### RETP-2026-004

- **Fecha:** 14/03/2026
- **Hora (Europe/Madrid):** 18:36:25
- **Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA
- **Frente/Fase:** Beta / espejo doctrinal controlado / Fase IV
- **Resumen del cambio:** Se activa el espejo controlado entre `especificaciones/` y `beta/`; se incorpora la matriz operativa al plano doctrinal `especificaciones/proposiciones/`; se aloja la publicación de frontera 0-1 en `especificaciones/laboratorio/` y `beta/Lab.SV/`; y se actualizan los README públicos e índices internos de ambos repositorios.
- **Motivo o argumento:** Era necesario incrustar en ambos repositorios dos publicaciones estratégicas de alta sensibilidad metodológica sin confundir rango doctrinal, sin contaminar la IR ni la semántica canónica y dejando explícita la vigilancia diferenciada de `beta/`.
- **Base doctrinal o técnica invocada:** repositorio doctrinal `SV-matematica-semantica`; especificación transversal subordinada sobre la U; matriz operativa del Clasificador C0–C5; publicación `Desde la terna (0, 1, U) hasta la nueva frontera (0, 1, U, 0-1)`; criterio interno de trazabilidad secuencial.
- **Artefactos afectados:** `SV-matematica-semantica/README.md`; `SV-matematica-semantica/especificaciones/README.md`; `SV-matematica-semantica/especificaciones/proposiciones/matriz_operativa_completa_clasificador_sv.md`; `SV-matematica-semantica/especificaciones/laboratorio/desde_la_terna_0_1_u_hasta_la_nueva_frontera_0_1_u_0_1.md`; `SV-lenguaje-de-computacion/README.md`; `SV-lenguaje-de-computacion/beta/README.md`; `SV-lenguaje-de-computacion/beta/Lab.SV/desde_la_terna_0_1_u_hasta_la_nueva_frontera_0_1_u_0_1.md`.
- **Evidencia:** subida secuencial validada por PDFs de confirmación; auditoría de ingestión de ZIP frescos de ambos repositorios; verificación de jerarquía sin `beta/` en el repositorio doctrinal y con `Lab.SV` como hija de `beta/` en el repositorio del lenguaje.
- **Impacto:** trazabilidad; gobierno_tecnico; mantenibilidad; compatibilidad.
- **Objeción adversarial considerada:** riesgo de que el espejo físico eleve indebidamente el rango doctrinal de piezas experimentales o de que `beta/` contamine el repositorio doctrinal.
- **Decisión:** mantener diferenciación estructural estricta: `especificaciones/{nucleo,proposiciones,laboratorio}` en el repositorio doctrinal y `beta/{C1_proposiciones,Lab.SV}` en el repositorio del lenguaje, con subordinación expresa en los README públicos.
- **Estado:** cerrado

### RETP-2026-005

- **Fecha:** 14/03/2026
- **Hora (Europe/Madrid):** 21:10:21
- **Tipo de hito:** CIERRE_PARCHE
- **Frente/Fase:** MICROPARCHE B / declaración operativa de rutas / Fase IV
- **Resumen del cambio:** Se registra como cierre autónomo del repositorio del lenguaje la declaración operativa de Ruta A, Ruta B y Beta en `beta/C1_proposiciones/matriz_operativa_completa_clasificador_sv.md`, `beta/README.md` y `README.md`, junto con la precisión pública de que esa triada no obliga a identidad funcional ni a sincronía conceptual completa con `SV-matematica-semantica`.
- **Motivo o argumento:** El microparche B constaba ya materialmente en los documentos públicos del repositorio del lenguaje, pero faltaba su trazabilidad explícita en el registro técnico propio gobernado por `docs/calidad/`.
- **Base doctrinal o técnica invocada:** jerarquía entre repositorios fijada en `README.md`; regla de subordinación de `beta/`; distinción entre espejo material e identidad funcional; criterio interno de trazabilidad secuencial.
- **Artefactos afectados:** `README.md`; `beta/README.md`; `beta/C1_proposiciones/matriz_operativa_completa_clasificador_sv.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
- **Evidencia:** ingestión de ZIP fresco del repositorio del lenguaje; contraste directo con el ZIP fresco del repositorio doctrinal; verificación de presencia material de la triada de rutas en los tres documentos públicos; actualización sincronizada del registro dual `.md`/`.csv`.
- **Impacto:** trazabilidad; gobierno_tecnico; mantenibilidad; compatibilidad.
- **Objeción adversarial considerada:** riesgo de registrar de forma redundante un cambio ya absorbido por el hito RETP-2026-004 o de deslizar por registro una equivalencia funcional impropia entre Beta del repositorio doctrinal y Beta del repositorio del lenguaje.
- **Decisión:** registrar el microparche B como cierre propio del repositorio del lenguaje y dejar expresamente fijado que la coincidencia material de la triada no impone sincronía conceptual completa entre repositorios de distinta misión.
- **Estado:** cerrado

- ### RETP-2026-006
- Fecha: 16/03/2026
- Hora (Europe/Madrid): 03:47:27
- Tipo de hito: DECISION_ARQUITECTONICA
- Frente/Fase: Gobierno técnico / decisión estratégica ternaria-Beta / Fase IV
- Resumen del cambio: Se fija la primacía activa de la célula ternaria SV, se declara la hibernación estratégica de la Beta como frente doctrinal activo y se establecen criterios de reactivación exclusivamente por contradicción local suficiente de la ternaria, dejando además publicada el acta operativa en `beta/ACTA_DECISION_TERNARIA_Y_HIBERNACION_BETA.md`.
- Motivo o argumento: La triple adversarial aplicada al corpus vigente (guía, doctrina fuerte de la U y Compilador) no acredita todavía insuficiencia material suficiente de la ternaria para abrir un frente cuaternario con rango doctrinal propio. Dada la restricción real de energía estratégica, se decide concentrar el esfuerzo vivo del proyecto en la vía ternaria y conservar Beta solo como laboratorio subsidiario e hibernado.
- Base doctrinal o técnica invocada: guía práctica del conocimiento profundo y la crítica de la razón pura; Origen doctrinal, definición y alcance de la U en el Sistema Vectorial SV; Compilador doctrinal y célula meta SV(9,3)-IA; acta estratégica publicada en `beta/ACTA_DECISION_TERNARIA_Y_HIBERNACION_BETA.md`.
- Artefactos afectados: `beta/ACTA_DECISION_TERNARIA_Y_HIBERNACION_BETA.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`; repositorio privado `SV-matematica-semantica-cuaternaria`.
- Evidencia: evidencia de publicación del acta en `beta/`; contraste adversarial del corpus publicado; decisión estratégica consolidada en acta y registro.
- Impacto: trazabilidad; gobierno_tecnico; mantenibilidad; compatibilidad.
- Objeción adversarial considerada: riesgo de congelar prematuramente una vía cuaternaria potencialmente fértil y riesgo simétrico de dispersar recursos vitales en dos frentes cuando el corpus vigente todavía sostiene la suficiencia operativa y conceptual de la ternaria.
- Decisión: mantener activa la vía ternaria como único camino de desarrollo inmediato; hibernar la Beta como frente doctrinal activo; reabrir Beta solo si aparece contradicción local suficiente, consistente y no resoluble limpiamente en `0/1/U`, sin estadística, inferencia ni minería de datos.
- Estado: cerrado


### RETP-2026-008

- **Fecha:** 18/03/2026
- **Hora (Europe/Madrid):** 16:31:19
- **Tipo de hito:** DECISION_ARQUITECTONICA
- **Frente/Fase:** Continuidad del frente básico y habilitación del backend / Fase IV
- **Resumen del cambio:** Se incorpora un informe público en `docs/arquitectura/` para declarar que la pendencia material de los sentidos restantes del frente básico no bloquea el avance del lenguaje SV hacia backend, siempre que el núcleo preserve puertos de extensión previstos y mantenga la subordinación doctrinal y técnica vigente.
- **Motivo o argumento:** La materialización ya observable de semántica, visión y motricidad permite afirmar una equivalencia estructural de clase entre los carriles ya abiertos y los sentidos restantes, de modo que éstos deben tratarse como extensiones subordinadas del frente básico y no como condición previa de refundación del núcleo.
- **Base doctrinal o técnica invocada:** programa de interfaces del Sistema Vectorial SV; IR canónica y bienformación del lenguaje SV — v0.2; Nota de arquitectura mínima del núcleo enganchable del lenguaje SV; README del repositorio `SV-lenguaje-de-computacion`.
- **Artefactos afectados:** `docs/arquitectura/INFORME_DE_CONTINUIDAD_DEL_FRENTE_BASICO_Y_HABILITACION_DEL_BACKEND_SV.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
- **Evidencia:** contraste público entre el programa de interfaces ya abierto en semántica, visión y motricidad, la IR v0.2 vigente, el criterio de núcleo enganchable ya fijado en `docs/arquitectura/` y el estado público del repositorio del lenguaje.
- **Impacto:** trazabilidad; gobierno_tecnico; compatibilidad; mantenibilidad.
- **Objeción adversarial considerada:** riesgo de petrificar prematuramente un núcleo incapaz de recibir futuros carriles perceptivos o de presentar el informe como cierre doctrinal superior cuando su naturaleza real es pública, técnica y subordinada.
- **Decisión:** declarar legítima la continuidad del backend bajo condición expresa de preservar puertos de extensión previstos y registrar el criterio como decisión arquitectónica pública en el repositorio del lenguaje.
- **Estado:** cerrado


### RETP-2026-009

- **Fecha:** 18/03/2026
- **Hora (Europe/Madrid):** 22:30:09
- **Tipo de hito:** DECISION_GOBIERNO_TECNICO
- **Frente/Fase:** Auditoría de repos frescos / fijación de sedes del ecosistema SV / Fase IV
- **Resumen del cambio:** Se fija `SV-matematica-semantica` como sede superior doctrinal y normativa del ecosistema y `SV-lenguaje-de-computacion` como sede central operativa y técnica, quedando además el pliego reconocido como ley interna de lectura y coherencia para la fase actual.
- **Motivo o argumento:** La auditoría directa de ambos repositorios muestra que la sede doctrinal ha alcanzado estabilidad suficiente de fase, mientras la fricción real del trabajo, la calidad, la validación y la deuda técnica se concentran en el repositorio del lenguaje, que resulta más apto como centro de gobierno operativo.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; auditoría cerrada de `SV-matematica-semantica`; auditoría cerrada de `SV-lenguaje-de-computacion`; jerarquía documental ya fijada; regla de no corrección silenciosa entre niveles.
- **Artefactos afectados:** `docs/calidad/ACTA_DE_FIJACION_DE_SEDES_Y_GOBIERNO_TECNICO_DEL_ECOSISTEMA_SV_2026_03_18.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`; `SV-matematica-semantica/documentos/registros/NOTA_DE_REMISION_SOBRE_SEDE_DOCTRINAL_Y_SEDE_OPERATIVA_DEL_ECOSISTEMA_SV_2026_03_18.md`.
- **Evidencia:** ingestión de ZIP frescos de ambos repositorios; verificación directa de jerarquía, estructura, suite y documentos de calidad; cierre adversarial de Fases 1, 2 y 3 del cruce entre sedes.
- **Impacto:** gobierno_tecnico; trazabilidad; mantenibilidad; coherencia_ecosistema.
- **Objeción adversarial considerada:** riesgo de convertir la sede operativa en autoridad doctrinal de facto o, en el extremo opuesto, de seguir tratando la sede doctrinal como taller técnico diario y contaminar su estabilidad de fase.
- **Decisión:** mantener arriba la sede doctrinal y abajo la sede operativa, impedir la reapertura ordinaria de esta distribución y permitir solo tres supuestos de reconsideración: revisión del pliego obligada por la realidad del sistema, requerimiento muy complejo del lenguaje o acumulación de elementos parciales con fuerza suficiente de reconsideración.
- **Estado:** cerrado


### RETP-2026-010

- **Fecha:** 18/03/2026
- **Hora (Europe/Madrid):** 23:45:11
- **Tipo de hito:** DECISION_GOBIERNO_TECNICO
- **Frente/Fase:** Gobierno operativo del lenguaje / materialización dual del pliego / constructor del manual SVP / Wishlist IRQ / Fase IV
- **Resumen del cambio:** Se materializa el pliego en doble sede interna, se crea el constructor del manual del lenguaje SVP con extensión `.svp` y se crea el Wishlist IRQ del ecosistema SV como mecanismo de recogida, aparcamiento y priorización controlada de deseos e ideas.
- **Motivo o argumento:** Era necesario evitar dependencia exclusiva de la URL canónica pública del pliego y, al mismo tiempo, ordenar dos frentes operativos del repositorio del lenguaje: la construcción progresiva del manual y la gestión disciplinada de la presión creativa lateral.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; fijación de sedes del ecosistema; frontera normativa del lenguaje SV; IR canónica v0.2; regla de segregación entre ruta activa y Beta.
- **Artefactos afectados:** `README.md`; `docs/README.md`; `docs/gobierno/PLIEGO_DE_CONDICIONES_DEL_SISTEMA_VECTORIAL_SV.md`; `docs/gobierno/README.md`; `docs/gobierno/MANIFIESTO_PLIEGO_Y_ASSETS.md`; `docs/gobierno/WISHLIST_IRQ_DEL_ECOSISTEMA_SV.md`; `docs/gobierno/WISHLIST_IRQ_DEL_ECOSISTEMA_SV.csv`; `docs/manual_svp/README.md`; `docs/manual_svp/CONSTRUCTOR_DEL_MANUAL_DEL_LENGUAJE_SVP.md`; `docs/manual_svp/INDICE_Y_ESTADO_DEL_MANUAL_DEL_LENGUAJE_SVP.csv`; `docs/manual_svp/PLANTILLA_DE_TRAMO_DEL_MANUAL_DEL_LENGUAJE_SVP.md`; `docs/manual_svp/PLANTILLA_DE_TRAMO_DEL_MANUAL_DEL_LENGUAJE_SVP.svp`; `docs/calidad/ACTA_DE_MATERIALIZACION_DEL_PLIEGO_Y_GOBIERNO_OPERATIVO_DEL_LENGUAJE_SV_2026_03_18.md`.
- **Evidencia:** construcción deliberada del paquete dual de repositorios; incorporación completa de assets del pliego; creación del constructor del manual; creación del Wishlist IRQ con CSV propio.
- **Impacto:** gobierno_tecnico; trazabilidad; mantenibilidad; autonomia_unidades; control_de_presion_creativa.
- **Objeción adversarial considerada:** riesgo de multiplicar documentos sin eje rector o de abrir un circuito de ideas laterales que compita con la ruta activa del lenguaje.
- **Decisión:** mantener el pliego como ley interna visible en ambas sedes, usar el constructor del manual como máquina de cierre progresivo del manual SVP y canalizar toda idea lateral por el Wishlist IRQ antes de convertirla en tarea o frente real.
- **Estado:** cerrado


### RETP-2026-011

- **Fecha:** 18/03/2026
- **Hora (Europe/Madrid):** 23:58:40
- **Tipo de hito:** DECISION_GOBIERNO_TECNICO
- **Frente/Fase:** Gobierno operativo del lenguaje / compatibilidad interlenguajes / lista de deseos IRQ / Fase IV
- **Resumen del cambio:** Se activa la política y protocolo de compatibilidad interlenguajes del Lenguaje SV, se incorpora la entrada `WIRQ-2026-001` como primer deseo sustantivo de la lista de deseos IRQ y se amplía el constructor del manual SVP con un tramo específico de compatibilidad y adopción.
- **Motivo o argumento:** Era necesario dejar ya ordenada la futura compatibilidad del lenguaje con Python, Rust, C, C++, Kotlin y la vigilancia de legado como Cobol para evitar deuda estructural y lluvia de ideas no gobernada.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; frontera normativa del lenguaje SV; IR canónica v0.2; constructor del manual SVP; lista de deseos IRQ del ecosistema SV.
- **Artefactos afectados:** `docs/gobierno/WISHLIST_IRQ_DEL_ECOSISTEMA_SV.md`; `docs/gobierno/WISHLIST_IRQ_DEL_ECOSISTEMA_SV.csv`; `docs/gobierno/POLITICA_Y_PROTOCOLO_DE_COMPATIBILIDAD_INTERLENGUAJES_DEL_LENGUAJE_SV.md`; `docs/manual_svp/CONSTRUCTOR_DEL_MANUAL_DEL_LENGUAJE_SVP.md`; `docs/manual_svp/INDICE_Y_ESTADO_DEL_MANUAL_DEL_LENGUAJE_SVP.csv`; `docs/manual_svp/TRAMO_10_COMPATIBILIDAD_INTERLENGUAJES_Y_ADOPCION_DEL_LENGUAJE_SV.md`; `docs/calidad/ACTA_DE_ACTIVACION_DE_POLITICA_DE_COMPATIBILIDAD_INTERLENGUAJES_Y_REGISTRO_CERO_DE_LISTA_DE_DESEOS_IRQ_2026_03_18.md`.
- **Evidencia:** construcción deliberada del bloque de compatibilidad; registro del deseo sustantivo inicial; ampliación del constructor del manual.
- **Impacto:** gobierno_tecnico; trazabilidad; adopcion_ecosistema; mantenibilidad; control_de_presion_creativa.
- **Objeción adversarial considerada:** riesgo de abrir demasiado pronto un frente de bindings concretos y riesgo simétrico de no dejar sembrada a tiempo la política de compatibilidad y adopción del lenguaje.
- **Decisión:** activar ya la política y el protocolo, mantener abierta la línea en lista de deseos IRQ con IRQ-2 y abrir un tramo específico del manual SVP, dejando las implementaciones concretas por lenguaje para fases posteriores gobernadas por evidencia.
- **Estado:** cerrado


### RETP-2026-012

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 06:39:08
- **Tipo de hito:** APERTURA_FASE
- **Frente/Fase:** Frente final del lenguaje SV / cierre operativo del lenguaje / Fase IV
- **Resumen del cambio:** Se abre formalmente el frente final del lenguaje SV, se fijan hoja de ruta, criterios de cierre, registro de deuda viva y tablero de bloques cerrables bajo gobierno técnico del repositorio operativo.
- **Motivo o argumento:** Una vez fijados pliego, sedes, publicación base, wishlist IRQ, política de compatibilidad y constructor del manual, el trabajo vivo del ecosistema queda concentrado en el cierre técnico del lenguaje y exige un marco operativo explícito.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; frontera normativa del lenguaje SV v0; IR canónica v0.2; nota de arquitectura mínima del núcleo enganchable; informe de continuidad del frente básico y habilitación del backend.
- **Artefactos afectados:** `docs/arquitectura/ACTA_DE_APERTURA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV_2026_03_19.md`; `docs/arquitectura/HOJA_DE_RUTA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`; `docs/arquitectura/CRITERIOS_DE_CIERRE_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`; `docs/arquitectura/README.md`; `docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`; `docs/calidad/TABLERO_DE_BLOQUES_CERRABLES_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.csv`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`; `docs/README.md`.
- **Evidencia:** cierre previo de auditoría de repos frescos, fijación de sedes del ecosistema, publicación del pliego y consolidación del gobierno operativo de fase.
- **Impacto:** gobierno_tecnico; trazabilidad; mantenibilidad; coherencia_ecosistema; preparacion_backend.
- **Objeción adversarial considerada:** riesgo de abrir un nuevo frente nominal sin reducir deuda real o de desplazar prematuramente el esfuerzo al backend soberano sin cerrar antes el núcleo semántico-diagnóstico.
- **Decisión:** abrir el frente exclusivamente para sanear y cerrar mejor el lenguaje vigente, prohibiendo que esta apertura se use como excusa para reabrir doctrina, multiplicar interfaces o adelantar backend sin ABI mínimo fijado.
- **Estado:** abierto

## RETP-2026-013 — Apertura de microauditoría del Bloque A — Contrato diagnóstico

- **Fecha:** 19/03/2026
- **Hora Europe/Madrid:** 07:20:00
- **Tipo de hito:** APERTURA_BLOQUE
- **Frente/Fase:** Frente final del lenguaje SV / Bloque A — Contrato diagnóstico / Fase IV
- **Resumen del cambio:** Se abre la microauditoría cerrada del Bloque A y se crean matriz de concordancia diagnóstica, guía de lectura y plantilla de dictamen de saneamiento.
- **Motivo/argumento:** La deuda viva principal del frente final sigue concentrándose en la concordancia entre IR, catálogo público, implementación efectiva, validator y suite.
- **Base doctrinal/técnica:** pliego de condiciones del Sistema Vectorial SV; Frontera normativa del lenguaje SV v0; IR canónica v0.2; hoja de ruta del frente final del lenguaje SV.
- **Artefactos afectados:** `docs/calidad/ACTA_DE_APERTURA_DE_MICROAUDITORIA_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO_2026_03_19.md`; `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`; `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`; `docs/calidad/DICTAMEN_DE_SANEAMIENTO_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO.md`; registros técnicos `.md/.csv`.
- **Evidencia:** paquete operativo de apertura del Bloque A.
- **Impacto:** `gobierno_tecnico`; `trazabilidad`; `coherencia_diagnostica`; `mantenibilidad`.
- **Objeción adversarial:** riesgo de abrir una microauditoría sin cerrar nada o de duplicar análisis ya realizados sin una matriz única de contraste.
- **Decisión:** abrir la microauditoría solo para producir matriz, clasificación y tratamiento recomendado del contrato diagnóstico vigente.
- **Estado:** abierto

### RETP-2026-014

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 10:07:25
- **Tipo de hito:** DECISION_GOBIERNO_TECNICO
- **Frente/Fase:** Gobierno operativo del lenguaje / normalización de redacción pública / ampliación de Wishlist IRQ / Fase IV
- **Resumen del cambio:** Se amplía la Wishlist IRQ del ecosistema SV con las entradas `WIRQ-2026-002` a `WIRQ-2026-006` y se normaliza la redacción pública de los documentos afectados para excluir referencias operativas internas.
- **Motivo o argumento:** Era necesario incorporar nuevas líneas de deseo estratégico del responsable del proyecto sin desplazar el Bloque A — Contrato diagnóstico y, al mismo tiempo, depurar la documentación pública para excluir referencias privadas o procedimentales impropias de un registro público.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; Wishlist IRQ del ecosistema SV; registro técnico vivo del frente final del lenguaje; criterio de bloques cerrables; regla de subordinación entre ruta activa, Wishlist y gobierno técnico.
- **Artefactos afectados:** `docs/gobierno/WISHLIST_IRQ_DEL_ECOSISTEMA_SV.md`; `docs/gobierno/WISHLIST_IRQ_DEL_ECOSISTEMA_SV.csv`; `docs/gobierno/README.md`; `docs/calidad/ACTA_DE_NORMALIZACION_DE_REDACCION_PUBLICA_Y_AMPLIACION_DE_WISHLIST_IRQ_2026_03_19.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
- **Evidencia:** lectura directa del estado público del repositorio operativo; reevaluación adversarial de las IRQ nuevas; revisión de redacción pública y compilación única de artefactos documentales.
- **Impacto:** gobierno_tecnico; trazabilidad; mantenibilidad; control_de_presion_creativa; descubrilidad.
- **Objeción adversarial considerada:** riesgo de mezclar mecánica interna con documentación pública y riesgo simétrico de absorber nuevas líneas de deseo sin un cauce público sobrio y revisable.
- **Decisión:** incorporar las nuevas entradas a la Wishlist IRQ con prioridad diferenciada, mantener su subordinación a la ruta activa del Bloque A y dejar normalizada la redacción pública de los documentos afectados.
- **Estado:** cerrado

### RETP-2026-015

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 12:40:45
- **Tipo de hito:** SANEAMIENTO_TECNICO
- **Frente/Fase:** Frente final del lenguaje SV / Bloque A — Contrato diagnóstico / saneamiento local E007 E008 E009 / Fase IV
- **Resumen del cambio:** Se materializa en el repositorio operativo un saneamiento local de la familia E007 E008 E009 mediante validación efectiva de conectores y tablas de admisibilidad y ampliación de la suite adversarial con casos inválidos específicos.
- **Motivo o argumento:** La microauditoría del Bloque A había identificado deuda real en la emisión observable de E007 E008 y E009. El repositorio ya muestra ahora una cobertura más sincera y verificable para esos tres códigos.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; IR canónica de bienformación v0.2; Bloque A — Contrato diagnóstico; criterio de saneamiento mínimo por bloque cerrable.
- **Artefactos afectados:** `src/svp_parser.py`; `src/svp_validator.py`; `tests/run_conformance.py`; `tests/conformance/invalid/conector_mapping_incompleto.svp`; `tests/conformance/invalid/conector_target_no_ternario.svp`; `tests/conformance/invalid/admissibility_table_incompleta.svp`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
- **Evidencia:** revisión directa de los PDFs de confirmación del repositorio público con presencia material del saneamiento local y de los tres casos inválidos añadidos.
- **Impacto:** coherencia_diagnostica; trazabilidad; mantenibilidad; cobertura_de_suite.
- **Objeción adversarial considerada:** riesgo de confundir materialización pública de archivos con ejecución remota de la suite. El registro declara la materialización visible del saneamiento y no afirma ejecución en GitHub no acreditada por evidencia aportada.
- **Decisión:** dar por materializado el saneamiento local E007 E008 E009 y dejar pendiente un cierre posterior del subtramo E001 sin mezclarlo en esta ronda.
- **Estado:** cerrado

### RETP-2026-016

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 12:40:46
- **Tipo de hito:** DECISION_GOBIERNO_TECNICO
- **Frente/Fase:** Gobierno operativo del lenguaje / ampliación de Wishlist IRQ / agentes especializados / Fase IV
- **Resumen del cambio:** Se incorpora la entrada `WIRQ-2026-007` a la Wishlist IRQ para recoger la generación de agentes especializados por el lenguaje SV con configuración precargada y remisión estructural al ecosistema `SVperitus`.
- **Motivo o argumento:** El deseo tiene alto valor estratégico de adopción y especialización experta, pero no debe desplazar el Bloque A ni abrir por entusiasmo una implementación prematura.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; Wishlist IRQ del ecosistema SV; criterio de control de fase; plano N4 ya visible en las declaraciones `domain`, `agent` y `query_spec` del lenguaje.
- **Artefactos afectados:** `docs/gobierno/WISHLIST_IRQ_DEL_ECOSISTEMA_SV.md`; `docs/gobierno/WISHLIST_IRQ_DEL_ECOSISTEMA_SV.csv`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
- **Evidencia:** revisión del estado público del Wishlist IRQ; lectura de la estructura superficial vigente del lenguaje; evaluación adversarial de la nueva propuesta de agentes especializados.
- **Impacto:** adopcion_ecosistema; descubrilidad; gobierno_tecnico; control_de_presion_creativa.
- **Objeción adversarial considerada:** riesgo de leer como compromiso de implementación inmediata una línea que en esta fase solo puede admitirse como deseo estratégico subordinado y coordinado con `SVperitus`.
- **Decisión:** clasificar `WIRQ-2026-007` como `IRQ-4`, con ruta `aparcado`, y reabrirla solo cuando exista mayor madurez del núcleo del lenguaje y delimitación formal del cruce con `SVperitus`.
- **Estado:** cerrado



### RETP-2026-017

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 13:00:59
- **Tipo de hito:** SANEAMIENTO_TECNICO
- **Frente/Fase:** Frente final del lenguaje SV / Bloque A — Contrato diagnóstico / saneamiento local E001 E507 / Fase IV
- **Resumen del cambio:** Se descomprime parcialmente la sobrecarga diagnóstica de `E001` al hacer que el caso adversarial de coerción implícita de `U` mediante `null` o `None` emita `E507`, con actualización sincronizada de parser, catálogo implementativo, suite y documentación pública asociada.
- **Motivo o argumento:** Tras el saneamiento local de `E007 E008 E009`, el siguiente subtramo legítimo del Bloque A era reducir la bolsa genérica de `E001` sin abrir todavía una refundición completa del parser. El caso `u_coercion.svp` ofrecía un cierre local real, verificable y de bajo radio de impacto.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; IR canónica de bienformación v0.2; Bloque A — Contrato diagnóstico; catálogo implementativo vigente; criterio de saneamiento mínimo por bloque cerrable.
- **Artefactos afectados:** `src/svp_errors.py`; `src/svp_parser.py`; `tests/run_conformance.py`; `tests/README.md`; `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
- **Evidencia:** verificación local del caso adversarial `u_coercion.svp` con emisión `E507` y ejecución completa de la suite de conformidad en el estado preparado para el paquete.
- **Impacto:** `coherencia_diagnostica`; `trazabilidad`; `mantenibilidad`; `cobertura_de_suite`.
- **Objeción adversarial considerada:** riesgo de presentar este ajuste como resolución total de la sobrecarga de `E001`. El parche no clausura todo `E001`; solo extrae de su bolsa genérica el subcaso de coerción implícita de `U`, dejando para bloque posterior el resto de errores sintácticos aún absorbidos por ese código.
- **Decisión:** materializar el saneamiento local `E001/E507` como bloque técnico autónomo y mantener pendiente una futura depuración más amplia del parse diagnóstico.
- **Estado:** cerrado


## RETP-2026-018

**Fecha:** 19/03/2026  
**Hora (Europe/Madrid):** 13:20:00  
**Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA  
**Frente/Fase:** Frente final del lenguaje SV / Bloque A — contrato diagnóstico / sincronización pública de suite y catálogo / Fase IV  
**Resumen del cambio:** Se sincronizan `tests/README.md` y `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md` con el estado ya materializado del saneamiento local `E007 E008 E009` y `E001/E507`, actualizando conteos, casos inválidos, emisión observable y cobertura explícita de suite.  
**Motivo o argumento:** Tras los saneamientos locales ya materializados, la documentación pública seguía reflejando un estado anterior de la suite y del catálogo observable, con infradeclaración de `E007`, `E008` y `E009` y con conteo desfasado en `tests/README.md`.  
**Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; Bloque A — Contrato diagnóstico; criterio de sincronización pública en el mismo bloque que altere el contrato observable; documentación pública vigente del catálogo y de la suite.  
**Artefactos afectados:** `tests/README.md`; `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.  
**Evidencia:** revisión directa del estado público ya materializado de `src/svp_validator.py`, `tests/run_conformance.py` y de los casos inválidos añadidos; contraste de la documentación pública con ese estado observable.  
**Impacto:** coherencia_diagnostica; trazabilidad; descubrilidad; mantenibilidad.  
**Objeción adversarial considerada:** riesgo de registrar como nuevo avance funcional lo que en realidad es una sincronización documental posterior. Este hito no declara saneamiento técnico adicional; solo corrige la documentación pública para que no sobreatribuya ni infradeclare el estado observable vigente.  
**Decisión:** actualizar la documentación pública de suite y catálogo en bloque mínimo separado, sin reabrir todavía una depuración más amplia del parse diagnóstico ni adelantar nuevos frentes funcionales.  
**Estado:** cerrado


Fecha: 19/03/2026
Hora (Europe/Madrid): 13:40:00
Tipo de hito: CAMBIO_RUNNER
Frente/Fase: Frente final del lenguaje SV / Bloque A — Contrato diagnóstico / ampliación de cobertura explícita E005 E010 / Fase IV
Resumen del cambio: Se amplía la suite de conformidad observable con los casos adversariales `duplicate_identifier.svp` e `invalid_role_literal.svp`, se actualiza `tests/run_conformance.py` y se sincronizan `tests/README.md` y `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md` con la nueva cobertura explícita de `E005` y `E010`.
Motivo o argumento: Tras los saneamientos locales previos, el siguiente cierre legítimo dentro de la familia E001–E010 era aumentar la cobertura explícita de códigos ya emitibles y de bajo riesgo, sin abrir todavía una refundición más amplia del parse diagnóstico.
Base doctrinal o técnica invocada: pliego de condiciones del Sistema Vectorial SV; Bloque A — Contrato diagnóstico; catálogo implementativo vigente; criterio de bloques cerrables; suite observable del frontend de referencia.
Artefactos afectados: `tests/run_conformance.py`; `tests/README.md`; `tests/conformance/invalid/duplicate_identifier.svp`; `tests/conformance/invalid/invalid_role_literal.svp`; `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
Evidencia: verificación local de la suite con 22 de 22 casos de conformidad superados y comprobación directa de que los dos nuevos adversariales manifiestan `E005` y `E010`, respectivamente.
Impacto: `coherencia_diagnostica`; `trazabilidad`; `cobertura_de_suite`; `mantenibilidad`.
Objeción adversarial considerada: riesgo de presentar una ampliación de cobertura como si fuera saneamiento funcional nuevo. El paquete no altera la semántica implementativa del frontend; hace explícita y verificable en la suite una parte más del contrato diagnóstico ya existente.
Decisión: ampliar de forma controlada la cobertura observable del Bloque A sobre `E005` y `E010`, mantener `E004` como deuda documentada de alcanzabilidad superficial y reservar para bloque posterior cualquier descompresión adicional de `E001`.
Estado: cerrado

Fecha: 19/03/2026
Hora (Europe/Madrid): 14:05:00
Tipo de hito: CAMBIO_DOCUMENTACION_PUBLICA
Frente/Fase: Frente final del lenguaje SV / Bloque A — Contrato diagnóstico / sinceramiento de cobertura explícita de E001 / Fase IV
Resumen del cambio: Se corrige `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md` para retirar `E001` de la lista de códigos cubiertos explícitamente por la suite y se deja documentado que, tras la extracción observacional de `u_coercion.svp` hacia `E507`, `E001` conserva sitio de emisión observable pero carece hoy de caso adversarial explícito propio en la suite.
Motivo o argumento: La ampliación previa de cobertura explícita sobre `E005` y `E010` dejó una sobreatribución residual en la documentación pública: `tests/run_conformance.py` ya no declara ningún caso inválido con código esperado `E001`, mientras que el catálogo público seguía listándolo como cubierto por la suite.
Base doctrinal o técnica invocada: pliego de condiciones del Sistema Vectorial SV; Bloque A — Contrato diagnóstico; catálogo implementativo vigente; criterio de sinceramiento público del estado observable; suite observable del frontend de referencia.
Artefactos afectados: `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
Evidencia: contraste directo entre `tests/run_conformance.py` y la sección de cobertura explícita de `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`, con ausencia observable de casos inválidos que esperen `E001` y persistencia de `E001` en el listado público de cobertura.
Impacto: `coherencia_diagnostica`; `trazabilidad`; `mantenibilidad`; `descubrilidad`.
Objeción adversarial considerada: riesgo de registrar como hito separado una mera corrección de redacción. En este caso sí procede porque la sobreatribución afecta al contrato diagnóstico público y altera la lectura verificable de la cobertura observable de la suite.
Decisión: sincerar la documentación pública del catálogo retirando `E001` de la cobertura explícita de suite, mantener `E001` como código con emisión observable directa y reservar para bloque posterior cualquier reintroducción de cobertura explícita mediante un adversarial propio.
Estado: cerrado


Fecha: 19/03/2026
Hora (Europe/Madrid): 14:25:00
Tipo de hito: CAMBIO_RUNNER
Frente/Fase: Frente final del lenguaje SV / Bloque A — Contrato diagnóstico / reintroducción de cobertura explícita de E001 / Fase IV
Resumen del cambio: Se añade el caso adversarial invalid_tri_literal.svp, se actualiza tests/run_conformance.py y se sincronizan tests/README.md y docs/referencia/ERRORES_CANONICOS_SV_v0_2.md para reintroducir cobertura explícita observable de E001 sin revertir la extracción del subcaso u_coercion.svp hacia E507.
Motivo o argumento: Tras el sinceramiento previo del catálogo público, E001 había quedado como código con emisión observable directa pero sin adversarial propio en la suite. La superficie actual sí permite construir un caso mínimo y limpio de literal ternario no reconocido, por lo que procede cerrar ese hueco verificable.
Base doctrinal o técnica invocada: pliego de condiciones del Sistema Vectorial SV; Bloque A — Contrato diagnóstico; catálogo implementativo vigente; suite observable del frontend de referencia; criterio de bloques cerrables.
Artefactos afectados: tests/run_conformance.py; tests/README.md; tests/conformance/invalid/invalid_tri_literal.svp; docs/referencia/ERRORES_CANONICOS_SV_v0_2.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
Evidencia: verificación local del nuevo caso adversarial con emisión E001 y ejecución completa de la suite de conformidad en el estado preparado para el paquete.
Impacto: coherencia_diagnostica; trazabilidad; cobertura_de_suite; mantenibilidad.
Objeción adversarial considerada: riesgo de reabrir la bolsa diagnóstica de E001 de forma indiscriminada. El paquete no deshace el saneamiento E001/E507; sólo reintroduce una cobertura explícita y limpia para el subcaso residual de literal ternario no reconocido.
Decisión: ampliar de forma controlada la cobertura observable de E001 mediante un adversarial mínimo específico y mantener separada cualquier depuración posterior de otros errores sintácticos todavía absorbidos por E001.
Estado: cerrado

| RETP-2026-023 | 19/03/2026 | 17:10:00 | CAMBIO_DOCUMENTACION_PUBLICA | Frente final del lenguaje SV / Bloque A — contrato diagnóstico / normalización fina de dictamen y matriz / Fase IV | Se normalizan los artefactos canónicos del Bloque A para eliminar restos de plantilla, corregir la cobertura consignada en matriz y cerrar una mención residual impropia de cocina interna en documentación pública. | cerrado |

Fecha: 19/03/2026
Hora: 17:10:00
Tipo de hito: CAMBIO_DOCUMENTACION_PUBLICA
Frente/Fase: Frente final del lenguaje SV / Bloque A — contrato diagnóstico / normalización fina de dictamen y matriz / Fase IV
RETP-2026-023 — Normalización fina de artefactos canónicos del Bloque A

Resumen del cambio: Se reescriben el dictamen del Bloque A y la matriz de concordancia para eliminar restos de plantilla, consolidar en una sola versión el estado técnico ya alcanzado por la familia E001–E010 y corregir la consignación de cobertura explícita y emisión observable en los códigos E005, E007, E008, E009, E010 y E507. Además, se elimina una mención residual impropia de cocina interna en el acta pública de fijación de sedes.

Motivo o argumento: Tras la sincronización anterior del barrido E001–E010, el repositorio público conservaba una duplicación de plantilla en el dictamen, una referencia residual a unidades internas en documentación pública y una matriz todavía no del todo congruente con la suite observable vigente.

Base doctrinal o técnica: Pliego de condiciones del Sistema Vectorial SV; Bloque A — Contrato diagnóstico; criterio de redacción pública sobria; suite observable del frontend; auditoría directa de repo fresco.

Artefactos afectados: docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md; docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv; docs/calidad/DICTAMEN_DE_SANEAMIENTO_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO.md; docs/calidad/ACTA_DE_FIJACION_DE_SEDES_Y_GOBIERNO_TECNICO_DEL_ECOSISTEMA_SV_2026_03_18.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.

Evidencia: contraste directo entre repo fresco, suite observable, matriz diagnóstica, dictamen y documentos públicos de calidad.

Impacto: coherencia_diagnostica; trazabilidad; mantenibilidad; redaccion_publica.

Objeción adversarial considerada: riesgo de considerar este bloque como simple retoque editorial. Procede como hito separado porque afecta a artefactos canónicos de gobierno técnico del Bloque A y corrige desalineaciones visibles entre documentación pública y estado real alcanzado.

Decisión: cerrar la limpieza fina de los artefactos canónicos del Bloque A antes de abrir la familia E101–E111 y dejar así el barrido E001–E010 en un estado documental plenamente estable para la fase actual.

Estado: cerrado


| RETP-2026-024 | 19/03/2026 | 17:35:00 | CAMBIO_RUNNER | Frente final del lenguaje SV / Bloque A — contrato diagnóstico / apertura de familia E101 E111 con cobertura explícita E101 E105 / Fase IV | Se amplía la suite observable con dos adversariales mínimos para E101 y E105 y se sincronizan tests, catálogo y matriz diagnóstica con esa cobertura explícita inicial de la familia E101–E111. | cerrado |

Fecha: 19/03/2026
Hora: 17:35:00
Tipo de hito: CAMBIO_RUNNER
Frente/Fase: Frente final del lenguaje SV / Bloque A — contrato diagnóstico / apertura de familia E101 E111 con cobertura explícita E101 E105 / Fase IV
RETP-2026-024 — Apertura controlada de la familia E101–E111 con cierres locales E101 y E105

Resumen del cambio: Se añaden los casos adversariales `cellstate_vector_length_mismatch.svp` y `bridge_position_fuera_de_rango.svp`, se actualizan `tests/run_conformance.py` y `tests/README.md`, y se sincronizan el catálogo público y la matriz diagnóstica para hacer explícita la cobertura observable de `E101` y `E105`.

Motivo o argumento: La matriz canónica vigente sitúa a `E101` y `E105` como códigos con emisión directa observable pero todavía sin cobertura explícita de suite. Eran, por tanto, los dos cierres locales de menor radio de impacto para abrir legítimamente la familia `E101–E111`.

Base doctrinal o técnica: Pliego de condiciones del Sistema Vectorial SV; Bloque A — Contrato diagnóstico; matriz diagnóstica canónica; catálogo implementativo vigente; suite observable del frontend de referencia; criterio de bloques cerrables.

Artefactos afectados: `tests/run_conformance.py`; `tests/README.md`; `tests/conformance/invalid/cellstate_vector_length_mismatch.svp`; `tests/conformance/invalid/bridge_position_fuera_de_rango.svp`; `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`; `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`; `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.

Evidencia: preparación del paquete técnico sobre estado fresco ya sincronizado; verificación local de la suite con incorporación de dos adversariales mínimos y actualización congruente de la documentación pública afectada.

Impacto: coherencia_diagnostica; trazabilidad; cobertura_de_suite; mantenibilidad.

Objeción adversarial considerada: riesgo de presentar como apertura de familia una refundición más amplia de la capa de estado. No procede: el bloque se limita a dos cierres locales ya emitibles y de bajo radio, dejando sin tocar por ahora los subtramos `E102`, `E104`, `E106` y el grupo `E107–E111`.

Decisión: abrir la familia `E101–E111` por sus dos casos ya directamente emitibles y más baratos de verificar, mantener el resto del subtramo bajo auditoría y reservar para la siguiente ronda el contraste fino `E102 / E104 / E106 / E111`.

Estado: cerrado


| RETP-2026-025 | 19/03/2026 | 18:20:00 | VERIFICABILIDAD_EXTERNA | Frente final del lenguaje SV / verificabilidad externa mínima / FrameComparison ejemplos y señalización histórica / Fase IV | Se refuerza la verificabilidad externa del repositorio mediante la normalización léxica de `FrameComparison`, la publicación de ejemplos canónicos mínimos y la creación de una carpeta pública de sondas adversariales documentadas. | cerrado |

Fecha: 19/03/2026
Hora: 18:20:00
Tipo de hito: VERIFICABILIDAD_EXTERNA
Frente/Fase: Frente final del lenguaje SV / verificabilidad externa mínima / FrameComparison ejemplos y señalización histórica / Fase IV
RETP-2026-025 — Cierre de verificabilidad externa mínima antes del contraste fino de E101–E111

Resumen del cambio: Se elimina la duplicidad silenciosa de clave en el lexer para `FrameComparison`, se ajusta el parser para aceptar el mismo literal superficial como constructor de `QueryContext` y como `query_type`, se publican dos ejemplos canónicos mínimos en `examples/` y se añade una pequeña capa pública de sondas adversariales documentadas en `tests/adversarial/documentados/`. Además, se refuerza la señalización histórica del documento de concordancia parcial de raíz.

Motivo o argumento: La lectura adversarial externa había detectado un riesgo de verificabilidad pública: una redundancia léxica objetivamente anómala en `FrameComparison`, ausencia de programas `.svp` en `examples/` y un documento histórico en raíz con nombre potencialmente sobreatributivo. Antes de seguir ampliando el barrido técnico de familias, convenía cerrar este bloque corto de credibilidad externa.

Base doctrinal o técnica: Pliego de condiciones del Sistema Vectorial SV; Bloque A — Contrato diagnóstico; criterio de verificabilidad externa mínima; suite pública vigente; análisis adversarial externo contrastado con repo fresco.

Artefactos afectados: `src/svp_lexer.py`; `src/svp_parser.py`; `README.md`; `src/README.md`; `examples/README.md`; `examples/celula_basica_con_evaluacion.svp`; `examples/consulta_framecomparison.svp`; `tests/adversarial/README.md`; `tests/adversarial/documentados/README.md`; `tests/adversarial/documentados/celula_aislada_con_resolucion_u.svp`; `tests/adversarial/documentados/composicion_serie_con_trayectoria.svp`; `tests/adversarial/documentados/agente_con_consulta_y_dominio.svp`; `CONCORDANCIA_CATALOGO_ERRORES_IR_v0_2_IMPLEMENTACION_PARCHE_1A.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.

Evidencia: contraste directo sobre repo fresco; verificación local de la suite de conformidad tras la normalización de `FrameComparison`; y lectura comparada entre materiales públicos canónicos y adversariales.

Impacto: verificabilidad_externa; trazabilidad; descubrilidad; redaccion_publica; mantenibilidad.

Objeción adversarial considerada: riesgo de mezclar ejemplos públicos con sondas adversariales y de publicar materiales híbridos que no fueran ni ejemplos canónicos ni piezas de contraste confesadas. Se evita con dos carriles separados: `examples/` para ejemplos mínimos y `tests/adversarial/documentados/` para sondas públicas de tensión técnica.

Decisión: cerrar primero un bloque corto de verificabilidad externa mínima y dejar para la siguiente ronda el contraste fino `E102 / E104 / E106 / E111`, ya sobre una superficie pública más limpia y mejor defendible frente a revisión externa.

Estado: cerrado


| RETP-2026-026 | 19/03/2026 | 19:20:00 | CAMBIO_DOCUMENTACION_PUBLICA | Frente final del lenguaje SV / Bloque A — contrato diagnóstico / contraste fino E102 E104 E106 E111 / Fase IV | Se ejecuta el contraste fino público de E102 E104 E106 y E111 mediante sondas adversariales documentadas y sincronización de matriz dictamen y registro. | cerrado |

Fecha: 19/03/2026
Hora: 19:20:00
Tipo de hito: CAMBIO_DOCUMENTACION_PUBLICA
Frente/Fase: Frente final del lenguaje SV / Bloque A — contrato diagnóstico / contraste fino E102 E104 E106 E111 / Fase IV
RETP-2026-026 — Contraste fino documentado de E102 E104 E106 y E111 antes de pasar a E201–E211

Resumen del cambio: Se amplía la carpeta pública `tests/adversarial/documentados/` con cuatro piezas de contraste fino sobre `E102`, `E104`, `E106` y `E111`, y se sincronizan la matriz canónica, el dictamen del Bloque A y el registro dual para reflejar el resultado de ese contraste sin abrir todavía cambios semánticos del frontend.

Motivo o argumento: Tras el cierre de verificabilidad externa mínima y el repo fresco ya estabilizado, el siguiente cuello de botella no estaba en más ejemplos ni en más microcierres de suite, sino en sincerar el subtramo residual `E102 / E104 / E106 / E111` antes de abandonar la familia `E101–E111`.

Base doctrinal o técnica: Pliego de condiciones del Sistema Vectorial SV; Bloque A — Contrato diagnóstico; matriz diagnóstica canónica; criterio de contraste fino por subtramos; repo fresco del frontend de referencia; capa pública de sondas adversariales documentadas.

Artefactos afectados: `tests/adversarial/documentados/README.md`; `tests/adversarial/documentados/e102_output_semantics_ausente_contraste.svp`; `tests/adversarial/documentados/e104_codominio_de_conector_invalido_contraste.svp`; `tests/adversarial/documentados/e106_relacion_semantica_ausente_contraste.svp`; `tests/adversarial/documentados/e111_codominio_sin_orden_total_contraste.md`; `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`; `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`; `docs/calidad/DICTAMEN_DE_SANEAMIENTO_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.

Evidencia: contraste directo sobre repo fresco y lectura comparada entre IR v0.2 catálogo público efectivo implementación validator y superficie adversarial documentada.

Impacto: coherencia_diagnostica; trazabilidad; verificabilidad_externa; redaccion_publica; mantenibilidad.

Objeción adversarial considerada: riesgo de presentar como cierre funcional lo que en realidad es un sinceramiento documental. No procede sobreatribuir: el bloque no afirma convergencia semántica nueva, sino que deja documentado con mayor precisión qué cae hoy en `E006` o `E008`, qué mantiene coincidencia formal y qué sigue sin sonda superficial propia.

Decisión: dar por ejecutado el contraste fino de `E102 / E104 / E106 / E111`, cerrar la familia `E101–E111` para la fase actual en términos de contraste y trasladar el siguiente barrido legítimo a `E201–E211`.

Estado: cerrado


| RETP-2026-027 | 19/03/2026 | 20:10:00 | CAMBIO_RUNNER | Frente final del lenguaje SV / Bloque A — contrato diagnóstico / apertura de familia E201 E211 con cobertura explícita E202 E204 E205 / Fase IV | Se amplía la suite observable con tres adversariales mínimos para E202 E204 y E205 y se sincronizan tests catálogo matriz dictamen y registro con esa cobertura explícita inicial de la familia E201–E211. | cerrado |
| RETP-2026-028 | 19/03/2026 | 23:10:34 | SORPRESA_TECNICA | Frente final del lenguaje SV / gobernanza de coherencia entre frame histórico, reapertura y consulta presente / Fase IV | Se registra una nota técnica de calidad para gobernar la tensión entre trayectoria histórica, retorno legítimo a `U`, cobertura/admisibilidad vigente y consulta presente, sin reabrir doctrina ni elevar aún el asunto a paper público. | cerrado |

Fecha: 19/03/2026
Hora: 20:10:00
Tipo de hito: CAMBIO_RUNNER
Frente/Fase: Frente final del lenguaje SV / Bloque A — contrato diagnóstico / apertura de familia E201 E211 con cobertura explícita E202 E204 E205 / Fase IV
RETP-2026-027 — Apertura controlada de la familia E201–E211 con cierres locales E202 E204 y E205

Resumen del cambio: Se añaden los casos adversariales `gate_input_no_evalresult.svp`, `query_context_opaco.svp` y `supervise_target_opaco.svp`, se actualizan `tests/run_conformance.py` y `tests/README.md`, y se sincronizan el catálogo público, la matriz diagnóstica, el dictamen del Bloque A y el registro técnico para hacer explícita la cobertura observable de `E202`, `E204` y `E205`.

Motivo o argumento: Dentro de la familia `E201–E211`, `E202`, `E204` y `E205` eran los subcasos de menor radio y emisión más directa en la superficie publicada. `E210` y `E211` ya estaban cubiertos, mientras que `E201`, `E203`, `E206`, `E207`, `E208` y `E209` quedan todavía condicionados por huecos de alcanzabilidad superficial o por emisiones no específicas del parser v0.1.

Base doctrinal o técnica: Pliego de condiciones del Sistema Vectorial SV; Bloque A — Contrato diagnóstico; matriz diagnóstica canónica; catálogo implementativo vigente; suite observable del frontend de referencia; criterio de bloques cerrables por familias.

Artefactos afectados: `tests/run_conformance.py`; `tests/README.md`; `tests/conformance/invalid/gate_input_no_evalresult.svp`; `tests/conformance/invalid/query_context_opaco.svp`; `tests/conformance/invalid/supervise_target_opaco.svp`; `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`; `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`; `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`; `docs/calidad/DICTAMEN_DE_SANEAMIENTO_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.

Evidencia: preparación del paquete técnico sobre repo fresco tras el cierre de `RETP-2026-026`; verificación local de la suite con incorporación de tres adversariales mínimos y actualización congruente de la documentación pública afectada.

Impacto: coherencia_diagnostica; trazabilidad; cobertura_de_suite; mantenibilidad.

Objeción adversarial considerada: riesgo de presentar como apertura de familia un cierre total que todavía no existe. No procede: el bloque se limita a tres emisiones directas y deja explícitamente como residuales `E201`, `E203`, `E206`, `E207`, `E208` y `E209`.

Decisión: abrir la familia `E201–E211` por sus tres casos ya directamente emitibles y más baratos de verificar, mantener el resto del subtramo bajo contraste fino documentado y reservar para la siguiente ronda la apertura de `E301–E304`.

Estado: cerrado

### RETP-2026-028

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 23:10:34
- **Tipo de hito:** SORPRESA_TECNICA
- **Frente/Fase:** Frente final del lenguaje SV / gobernanza de coherencia entre frame histórico, reapertura y consulta presente / Fase IV
- **Resumen del cambio:** Se registra una nota técnica de calidad para gobernar una tensión relevante entre el último frame históricamente acreditado, la reapertura legítima a `U`, la cobertura/admisibilidad vigente y la forma en que la consulta presente expresa el estado actual.
- **Motivo o argumento:** El frente final del lenguaje exige cierre honesto entre IR, validator, runner, capa de consulta y documentación pública. La tensión detectada no tiene estatuto de wishlist ni de papel doctrinal nuevo, pero sí de riesgo gobernable del frente.
- **Base doctrinal o técnica invocada:** sede doctrinal principal sobre horizonte, transducción, consulta y `U`; `docs/arquitectura/ACTA_DE_APERTURA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV_2026_03_19.md`; `IR_CANONICA_BIENFORMACION_SV_v0_2.md`; `spec/u_en_svp_semantica_formal_y_regimen_de_resolucion.md`.
- **Artefactos afectados:** `docs/calidad/NOTA_TECNICA_SOBRE_FRAME_HISTORICO_REAPERTURA_Y_CONSULTA_PRESENTE_2026_03_19.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`; `docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`.
- **Evidencia:** lectura adversarial comparada del repositorio doctrinal principal y del repositorio del lenguaje, con contraste expreso entre trayectoria, `U`, transducción y consulta.
- **Impacto:** trazabilidad; gobierno_tecnico; fiabilidad; mantenibilidad.
- **Objeción adversarial considerada:** riesgo de duplicar doctrina ya existente y de sobreactuar como crisis basal un problema ya parcialmente previsto por la sede doctrinal superior.
- **Decisión:** registrar la tensión como nota técnica subordinada de calidad, abrir deuda viva específica y reservar cualquier eventual publicación doctrinal nueva para un momento posterior y solo si la bajada operativa acreditara insuficiencia real del marco vigente.
- **Estado:** cerrado

