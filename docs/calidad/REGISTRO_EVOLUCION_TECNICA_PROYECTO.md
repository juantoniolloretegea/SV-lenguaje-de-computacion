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
| RETP-2026-000 | 13/03/2026 | 21:02 | REGISTRO_INICIAL | — | Registro inicial de comienzo | cerrado |
| RETP-2026-001 | 13/03/2026 | 19:08:39 | CIERRE_PARCHE | PARCHE 2E-B / Fase IV | Se impone la alternancia constitutiva de Trajectory / se adapta la línea SEC-0 afectada / se añaden casos positivo y negativo de conformidad y se actualiza la suite a 17/17. | cerrado |
| RETP-2026-002 | 13/03/2026 | 20:12:56 | DECISION_ARQUITECTONICA | Gobierno técnico / Fase IV | Se fija la jerarquía operativa del proyecto / el control dual ligero/duro y la prioridad de sanear la concordancia IR ↔ catálogo de errores ↔ validator ↔ runner ↔ documentación pública antes de abrir nuevos frentes funcionales o backend Rust. | cerrado |
| RETP-2026-003 | 14/03/2026 | 23:40:35 | CAMBIO_DOCUMENTACION_PUBLICA | Microauditoría de concordancia / Fase IV | Se ajusta la documentación pública para distinguir entre validación implementada y concordancia fuerte total; se explicita la diferencia entre catálogo / emisión observable y cobertura de suite; y se sincroniza el registro dual .md/.csv. | cerrado |
| RETP-2026-004 | 14/03/2026 | 18:36:25 | CAMBIO_DOCUMENTACION_PUBLICA | Beta / espejo doctrinal controlado / Fase IV | Se activa el espejo controlado entre especificaciones/ y beta/ / se alojan la matriz operativa y la publicación de frontera 0-1 en sus rutas diferenciadas y se actualizan los índices públicos del repositorio doctrinal y del repositorio del lenguaje sin alterar la jerarquía normativa. | cerrado |
| RETP-2026-005 | 14/03/2026 | 21:10:21 | CIERRE_PARCHE | MICROPARCHE B / declaración operativa de rutas / Fase IV | Se registra el cierre del microparche B en el repositorio del lenguaje: declaración explícita de Ruta A / Ruta B / Beta en la matriz operativa / en beta/README.md y en README.md / con precisión expresa de que la triada no obliga a sincronía conceptual completa con el repositorio doctrinal. | cerrado |
| RETP-2026-006 | 16/03/2026 | 03:47:27 | DECISION_ARQUITECTONICA | Gobierno técnico / decisión estratégica ternaria-Beta / Fase IV | Se fija la primacía activa de la célula ternaria SV / se declara la hibernación estratégica de la Beta como frente doctrinal activo y se establecen criterios de reactivación exclusivamente por contradicción local suficiente de la ternaria / dejando además publicada el acta operativa en beta/ACTA_DECISION_TERNARIA_Y_HIBERNACION_BETA.md. | cerrado |
| RETP-2026-007 | 16/03/2026 | 04:50:23 | DECISION_ARQUITECTONICA | Gobierno técnico / control de escalado a Ruta Beta / Fase IV | Se fija en el registro técnico que toda unidad encargada que concluya motivadamente que procede abrir la Ruta Beta deberá elevar antes un informe al núcleo de coordinación Beta / que valorará si autoriza o no el salto. | cerrado |
| RETP-2026-008 | 18/03/2026 | 16:31:19 | DECISION_ARQUITECTONICA | Continuidad del frente básico y habilitación del backend / Fase IV | Se incorpora un informe público en docs/arquitectura/ para declarar que la pendencia material de los sentidos restantes del frente básico no bloquea el avance del lenguaje SV hacia backend / siempre que el núcleo preserve puertos de extensión previstos y mantenga la subordinación doctrinal y técnica vigente. | cerrado |
| RETP-2026-009 | 18/03/2026 | 22:30:09 | DECISION_GOBIERNO_TECNICO | Auditoría de repos frescos / fijación de sedes del ecosistema SV / Fase IV | Se fija SV-matematica-semantica como sede superior doctrinal y normativa del ecosistema y SV-lenguaje-de-computacion como sede central operativa y técnica / quedando además el pliego reconocido como ley interna de lectura y coherencia para la fase actual. | cerrado |
| RETP-2026-010 | 18/03/2026 | 23:45:11 | DECISION_GOBIERNO_TECNICO | Gobierno operativo del lenguaje / pliego doctrinal enlazado / manual SVP local / Wishlist IRQ / Fase IV | Se fijan el bloque local de gobierno operativo / el constructor del manual del lenguaje SVP con extensión .svp y el Wishlist IRQ del ecosistema SV / manteniendo la consulta pública consolidada del pliego en la sede doctrinal superior. | cerrado |
| RETP-2026-011 | 18/03/2026 | 23:58:40 | DECISION_GOBIERNO_TECNICO | Gobierno operativo del lenguaje / compatibilidad interlenguajes / lista de deseos IRQ / Fase IV | Se activa la política y protocolo de compatibilidad interlenguajes del Lenguaje SV / se incorpora la entrada WIRQ-2026-001 como primer deseo sustantivo de la lista de deseos IRQ y se amplía el constructor del manual SVP con un tramo específico de compatibilidad y adopción. | cerrado |
| RETP-2026-012 | 19/03/2026 | 06:39:08 | APERTURA_FASE | Frente final del lenguaje SV / cierre operativo del lenguaje / Fase IV | Se abre formalmente el frente final del lenguaje SV / se fijan hoja de ruta / criterios de cierre / registro de deuda viva y tablero de bloques cerrables bajo gobierno técnico del repositorio operativo. | abierto |
| RETP-2026-013 | 19/03/2026 | 07:20:00 | APERTURA_BLOQUE | Frente final del lenguaje SV / Bloque A — Contrato diagnóstico / Fase IV | Se abre la microauditoría cerrada del Bloque A y se crean matriz de concordancia diagnóstica / guía de lectura y plantilla de dictamen de saneamiento. | abierto |
| RETP-2026-014 | 19/03/2026 | 10:07:25 | DECISION_GOBIERNO_TECNICO | Gobierno operativo del lenguaje / normalización de redacción pública / ampliación de Wishlist IRQ / Fase IV | Se amplía la Wishlist IRQ del ecosistema SV con las entradas WIRQ-2026-002 a WIRQ-2026-006 y se normaliza la redacción pública de los documentos afectados para excluir referencias operativas internas. | cerrado |
| RETP-2026-015 | 19/03/2026 | 12:40:45 | SANEAMIENTO_TECNICO | Frente final del lenguaje SV / Bloque A — Contrato diagnóstico / saneamiento local E007 E008 E009 / Fase IV | Se materializa el saneamiento local E007 E008 E009 con validacion efectiva de conectores y tablas de admisibilidad y con ampliacion de la suite adversarial. | cerrado |
| RETP-2026-016 | 19/03/2026 | 12:40:46 | DECISION_GOBIERNO_TECNICO | Gobierno operativo del lenguaje / ampliacion de Wishlist IRQ / agentes especializados / Fase IV | Se incorpora WIRQ-2026-007 a la Wishlist IRQ para recoger la generacion de agentes especializados por el lenguaje SV con configuracion precargada y remision estructural a SVperitus. | cerrado |
| RETP-2026-017 | 19/03/2026 | 13:00:59 | SANEAMIENTO_TECNICO | Frente final del lenguaje SV / Bloque A — Contrato diagnóstico / saneamiento local E001 E507 / Fase IV | Se descomprime parcialmente la sobrecarga diagnostica de E001 al hacer que la coercion implicita de U mediante null o None emita E507 y al actualizar parser catalogo suite y documentacion publica asociada. | cerrado |
| RETP-2026-018 | 19/03/2026 | 13:20:00 | CAMBIO_DOCUMENTACION_PUBLICA | Frente final del lenguaje SV / Bloque A — contrato diagnostico / sincronizacion publica de suite y catalogo / Fase IV | Se sincronizan tests README y catalogo publico con E007 E008 E009 y E507 ya materializados. | cerrado |
| RETP-2026-019 | 19/03/2026 | 13:40:00 | CAMBIO_RUNNER | Frente final del lenguaje SV / Bloque A — Contrato diagnostico / ampliacion de cobertura explicita E005 E010 / Fase IV | Se amplía la suite observable con duplicate_identifier.svp e invalid_role_literal.svp y se sincronizan run_conformance README y catálogo público con la nueva cobertura de E005 y E010. | cerrado |
| RETP-2026-020 | 19/03/2026 | 14:05:00 | CAMBIO_DOCUMENTACION_PUBLICA | Frente final del lenguaje SV / Bloque A — contrato diagnostico / sinceramiento de cobertura explicita de E001 / Fase IV | Se retira E001 de la lista de codigos cubiertos explicitamente por la suite y se documenta que conserva emision observable sin caso adversarial explicito propio tras la extraccion local hacia E507. | cerrado |
| RETP-2026-021 | 19/03/2026 | 14:25:00 | CAMBIO_RUNNER | Frente final del lenguaje SV / Bloque A — Contrato diagnostico / reintroduccion de cobertura explicita de E001 / Fase IV | Se añade invalid_tri_literal.svp y se sincronizan suite y catalogo publico para devolver a E001 una cobertura explicita limpia y observable. | cerrado |
| RETP-2026-022 | 19/03/2026 | 17:10:00 | CAMBIO_DOCUMENTACION_PUBLICA | Frente final del lenguaje SV / Bloque A — contrato diagnostico / normalizacion fina de dictamen y matriz / Fase IV | Se normalizan dictamen matriz y redaccion publica residual del Bloque A. | cerrado |
| RETP-2026-023 | 19/03/2026 | 17:35:00 | CAMBIO_RUNNER | Frente final del lenguaje SV / Bloque A — contrato diagnostico / apertura de familia E101 E111 con cobertura explicita E101 E105 / Fase IV | Se amplía la suite observable con dos adversariales mínimos para E101 y E105 y se sincronizan tests catálogo y matriz diagnóstica con esa cobertura explícita inicial de la familia E101 E111. | cerrado |
| RETP-2026-024 | 19/03/2026 | 18:20:00 | VERIFICABILIDAD_EXTERNA | Frente final del lenguaje SV / verificabilidad externa minima / FrameComparison ejemplos y senalizacion historica / Fase IV | Se refuerza la verificabilidad externa del repositorio con normalizacion de FrameComparison ejemplos canonicos minimos y sondas adversariales documentadas. | cerrado |
| RETP-2026-025 | 19/03/2026 | 19:20:00 | CAMBIO_DOCUMENTACION_PUBLICA | Frente final del lenguaje SV / Bloque A — contrato diagnostico / contraste fino E102 E104 E106 E111 / Fase IV | Se ejecuta el contraste fino publico de E102 E104 E106 y E111 mediante sondas adversariales documentadas y sincronizacion de matriz dictamen y registro. | cerrado |
| RETP-2026-026 | 19/03/2026 | 20:10:00 | CAMBIO_RUNNER | Frente final del lenguaje SV / Bloque A — contrato diagnóstico / apertura de familia E201 E211 con cobertura explícita E202 E204 E205 / Fase IV | Se amplía la suite observable con tres adversariales mínimos para E202 E204 y E205 y se sincronizan tests catálogo matriz dictamen y registro con esa cobertura explícita inicial de la familia E201–E211. | cerrado |
| RETP-2026-027 | 19/03/2026 | 23:10:34 | SORPRESA_TECNICA | Frente final del lenguaje SV / gobernanza de coherencia entre frame histórico reapertura y consulta presente / Fase IV | Se registra una nota técnica de calidad para gobernar la tensión entre trayectoria histórica retorno legítimo a U cobertura y admisibilidad vigentes y consulta presente sin reabrir doctrina ni elevar aún el asunto a paper público. | cerrado |
| RETP-2026-028 | 20/03/2026 | 11:45:22 | CAMBIO_DOCUMENTACION_PUBLICA | Saneamiento de concordancia del bloque pliego-manual / Fase IV | Se regularizan docs/README.md / docs/gobierno/ / docs/manual_svp/ y el registro técnico para que la documentación pública del repositorio operativo refleje exactamente el árbol real: pliego y bloque gráfico en sede doctrinal superior / manual SVP de trabajo en sede operativa local. | cerrado |
| RETP-2026-029 | 20/03/2026 | 15:20:00 | CAMBIO_FUNCIONAL_GOBERNADO | Frente final del lenguaje SV / Bloque E — ABI semántico-diagnóstico y endurecimiento inicial de N4/Uso / Fase IV | Se materializa un contrato autónomo de enganche de interfaces futuras y se endurece la validación mínima de `Domain` `Agent` `QuerySpec` y `query`, sincronizando suite ejemplos catálogo público matriz diagnóstica y registro de deuda viva. | cerrado |
| RETP-2026-030 | 20/03/2026 | 16:51:49 | CAMBIO_DOCUMENTACION_PUBLICA | Frente final del lenguaje SV / Bloque A — regularización dura post RETP-2026-026 y RETP-2026-029 / Fase IV | Se sanean el dictamen del Bloque A y el registro técnico para eliminar la contradicción sobre el siguiente paso, fijar el estado real de E201–E211, E301–E304 y E401–E403, y dejar explícito que E303 no dispone aún de emisión específica autónoma acreditada. | cerrado |
| RETP-2026-031 | 20/03/2026 | 17:12:51 | CAMBIO_DOCUMENTACION_PUBLICA | Frente final del lenguaje SV / Bloque A — regularización pública de la emitibilidad real de E301–E304 / Fase IV | Se sincronizan el catálogo público y el registro técnico con la microauditoría E para dejar explícito que E304 sí es observable y cubierto, mientras E301, E302 y E303 no comparten todavía esa misma emitibilidad pública efectiva. | cerrado |

| RETP-2026-042 | 26/03/2026 | 09:40:00 | DECISION_GOBIERNO_TECNICO | Lenguaje SV / sellado técnico mínimo prebackend y reordenación de continuidad tras la familia VII / Fase IV | Se crea el bloque mínimo de gobierno para cerrar provisionalmente PC-HNA, registrar nueva deuda viva semántica y complementar el rumbo prebackend, fijando además que el siguiente frente correcto será la pieza marco de células especializadas y no NLP. | cerrado |

## 12. Entradas detalladas

### RETP-2026-000

- **Fecha:** 13/03/2026
- **Hora (Europe/Madrid):** 21:02
- **Tipo de hito:** REGISTRO_INICIAL
- **Frente/Fase:** —
- **Resumen del cambio:** Registro inicial de comienzo
- **Motivo o argumento:** —
- **Base doctrinal o técnica invocada:** —
- **Artefactos afectados:** —
- **Evidencia:** —
- **Impacto:** trazabilidad
- **Objeción adversarial considerada:** —
- **Decisión:** —
- **Estado:** cerrado

### RETP-2026-001

- **Fecha:** 13/03/2026
- **Hora (Europe/Madrid):** 19:08:39
- **Tipo de hito:** CIERRE_PARCHE
- **Frente/Fase:** PARCHE 2E-B / Fase IV
- **Resumen del cambio:** Se impone la alternancia constitutiva de Trajectory / se adapta la línea SEC-0 afectada / se añaden casos positivo y negativo de conformidad y se actualiza la suite a 17/17.
- **Motivo o argumento:** La alternancia de Trajectory constaba en la IR / pero no estaba impuesta todavía por el validator.
- **Base doctrinal o técnica invocada:** IR canónica v0.2; gramática superficial mínima v0.1; baseline SEC-0.
- **Artefactos afectados:** validator; runner de conformidad; casos de test; documentación de tests.
- **Evidencia:** paquete de trabajo con efecto estructural del parche; resultados internos verificados; evidencias publicadas de actualización de archivos.
- **Impacto:** adecuacion_funcional; trazabilidad; gobierno_tecnico.
- **Objeción adversarial considerada:** riesgo de reducir el parche a maquillaje de tests y riesgo de romper la línea adversarial previa.
- **Decisión:** cerrar el hueco mediante enforcement mínimo real y cobertura explícita de suite.
- **Estado:** cerrado

### RETP-2026-002

- **Fecha:** 13/03/2026
- **Hora (Europe/Madrid):** 20:12:56
- **Tipo de hito:** DECISION_ARQUITECTONICA
- **Frente/Fase:** Gobierno técnico / Fase IV
- **Resumen del cambio:** Se fija la jerarquía operativa del proyecto / el control dual ligero/duro y la prioridad de sanear la concordancia IR ↔ catálogo de errores ↔ validator ↔ runner ↔ documentación pública antes de abrir nuevos frentes funcionales o backend Rust.
- **Motivo o argumento:** El proyecto ha alcanzado suficiente madurez técnica y metodológica como para requerir una capa mínima de gobierno técnico sin duplicación doctrinal ni burocratización.
- **Base doctrinal o técnica invocada:** repositorio doctrinal vigente; Guía práctica del conocimiento profundo y la crítica de la razón pura; consensos operativos cerrados del frente del lenguaje.
- **Artefactos afectados:** metodología de trabajo; control de cambios; auditoría; registro técnico; planificación de siguientes frentes.
- **Evidencia:** debate técnico consolidado y punto de restauración formal fijado.
- **Impacto:** trazabilidad; gobierno_tecnico; mantenibilidad.
- **Objeción adversarial considerada:** riesgo de burocratización y de creación de una segunda capa doctrinal paralela.
- **Decisión:** adoptar una capa operativa subordinada / ligera durante la marcha y dura sólo en puntos de control.
- **Estado:** cerrado

### RETP-2026-003

- **Fecha:** 14/03/2026
- **Hora (Europe/Madrid):** 23:40:35
- **Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA
- **Frente/Fase:** Microauditoría de concordancia / Fase IV
- **Resumen del cambio:** Se ajusta la documentación pública para distinguir entre validación implementada y concordancia fuerte total; se explicita la diferencia entre catálogo / emisión observable y cobertura de suite; y se sincroniza el registro dual .md/.csv.
- **Motivo o argumento:** La microauditoría ha constatado que el frontend está estable / pero la documentación pública sobreatribuía cobertura del validator y no reflejaba con precisión el estado observable del contrato diagnóstico ni la asimetría entre registro en prosa y registro tabular.
- **Base doctrinal o técnica invocada:** IR canónica v0.2; decisión C1C de regularización por Vía B; procedimiento de auditoría técnica del proyecto; suite y código fuente observables del frontend.
- **Artefactos afectados:** src/svp_validator.py; src/README.md; docs/referencia/ERRORES_CANONICOS_SV_v0_2.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** microauditoría cerrada de concordancia; contraste directo entre código fuente / suite observable y documentación pública; actualización sincronizada de los dos registros técnicos.
- **Impacto:** trazabilidad; gobierno_tecnico; mantenibilidad.
- **Objeción adversarial considerada:** riesgo de convertir una microauditoría en pseudoavance sin corrección funcional real y riesgo de petrificar provisionalmente la ausencia de validación fuerte en N4/Uso.
- **Decisión:** sincerar primero el estado público verificable y reservar para bloque posterior / separado y adversarial / cualquier cierre funcional sobre N4/Uso o convergencia fuerte total.
- **Estado:** cerrado

### RETP-2026-004

- **Fecha:** 14/03/2026
- **Hora (Europe/Madrid):** 18:36:25
- **Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA
- **Frente/Fase:** Beta / espejo doctrinal controlado / Fase IV
- **Resumen del cambio:** Se activa el espejo controlado entre especificaciones/ y beta/ / se alojan la matriz operativa y la publicación de frontera 0-1 en sus rutas diferenciadas y se actualizan los índices públicos del repositorio doctrinal y del repositorio del lenguaje sin alterar la jerarquía normativa.
- **Motivo o argumento:** Era necesario incrustar en ambos repositorios dos publicaciones estratégicas de alta sensibilidad metodológica sin confundir rango doctrinal / sin contaminar la IR ni la semántica canónica y dejando explícita la vigilancia diferenciada de beta/.
- **Base doctrinal o técnica invocada:** repositorio doctrinal SV-matematica-semantica; especificación transversal subordinada sobre la U; matriz operativa del Clasificador C0–C5; publicación Desde la terna (0 / 1 / U) hasta la nueva frontera (0 / 1 / U / 0-1); criterio interno de trazabilidad secuencial.
- **Artefactos afectados:** SV-matematica-semantica/README.md; SV-matematica-semantica/especificaciones/README.md; SV-matematica-semantica/especificaciones/proposiciones/matriz_operativa_completa_clasificador_sv.md; SV-matematica-semantica/especificaciones/laboratorio/desde_la_terna_0_1_u_hasta_la_nueva_frontera_0_1_u_0_1.md; SV-lenguaje-de-computacion/README.md; SV-lenguaje-de-computacion/beta/README.md; SV-lenguaje-de-computacion/beta/Lab.SV/desde_la_terna_0_1_u_hasta_la_nueva_frontera_0_1_u_0_1.md.
- **Evidencia:** publicación secuencial validada por evidencia de confirmación; auditoría de ingestión de ZIP frescos de ambos repositorios; verificación de jerarquía sin beta/ en el repositorio doctrinal y con Lab.SV como hija de beta/ en el repositorio del lenguaje.
- **Impacto:** trazabilidad; gobierno_tecnico; mantenibilidad; compatibilidad
- **Objeción adversarial considerada:** riesgo de que el espejo físico eleve indebidamente el rango doctrinal de piezas experimentales o de que beta/ contamine el repositorio doctrinal.
- **Decisión:** mantener diferenciación estructural estricta: especificaciones/{nucleo /proposiciones /laboratorio} en el repositorio doctrinal y beta/{C1_proposiciones /Lab.SV} en el repositorio del lenguaje / con subordinación expresa en los README públicos.
- **Estado:** cerrado

### RETP-2026-005

- **Fecha:** 14/03/2026
- **Hora (Europe/Madrid):** 21:10:21
- **Tipo de hito:** CIERRE_PARCHE
- **Frente/Fase:** MICROPARCHE B / declaración operativa de rutas / Fase IV
- **Resumen del cambio:** Se registra el cierre del microparche B en el repositorio del lenguaje: declaración explícita de Ruta A / Ruta B / Beta en la matriz operativa / en beta/README.md y en README.md / con precisión expresa de que la triada no obliga a sincronía conceptual completa con el repositorio doctrinal.
- **Motivo o argumento:** El microparche B constaba ya materialmente en los documentos públicos del repositorio del lenguaje / pero faltaba su trazabilidad explícita en el registro técnico propio gobernado por docs/calidad/.
- **Base doctrinal o técnica invocada:** jerarquía entre repositorios fijada en README.md; regla de subordinación de beta/; distinción entre espejo material e identidad funcional; criterio interno de trazabilidad secuencial.
- **Artefactos afectados:** README.md; beta/README.md; beta/C1_proposiciones/matriz_operativa_completa_clasificador_sv.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** ingestión de ZIP fresco del repositorio del lenguaje; contraste directo con el ZIP fresco del repositorio doctrinal; verificación de presencia material de la triada de rutas en los tres documentos públicos; actualización sincronizada del registro dual .md/.csv.
- **Impacto:** trazabilidad; gobierno_tecnico; mantenibilidad; compatibilidad.
- **Objeción adversarial considerada:** riesgo de registrar de forma redundante un cambio ya absorbido por el hito RETP-2026-004 o de deslizar por registro una equivalencia funcional impropia entre Beta del repositorio doctrinal y Beta del repositorio del lenguaje.
- **Decisión:** registrar el microparche B como cierre propio del repositorio del lenguaje y dejar expresamente fijado que la coincidencia material de la triada no impone sincronía conceptual completa entre repositorios de distinta misión.
- **Estado:** cerrado

### RETP-2026-006

- **Fecha:** 16/03/2026
- **Hora (Europe/Madrid):** 03:47:27
- **Tipo de hito:** DECISION_ARQUITECTONICA
- **Frente/Fase:** Gobierno técnico / decisión estratégica ternaria-Beta / Fase IV
- **Resumen del cambio:** Se fija la primacía activa de la célula ternaria SV / se declara la hibernación estratégica de la Beta como frente doctrinal activo y se establecen criterios de reactivación exclusivamente por contradicción local suficiente de la ternaria / dejando además publicada el acta operativa en beta/ACTA_DECISION_TERNARIA_Y_HIBERNACION_BETA.md.
- **Motivo o argumento:** La triple adversarial aplicada al corpus vigente (guía / doctrina fuerte de la U y Compilador) no acredita todavía insuficiencia material suficiente de la ternaria para abrir un frente cuaternario con rango doctrinal propio. Dada la restricción real de energía estratégica / se decide concentrar el esfuerzo vivo del proyecto en la vía ternaria y conservar Beta solo como laboratorio subsidiario e hibernado.
- **Base doctrinal o técnica invocada:** Guía práctica del conocimiento profundo y la crítica de la razón pura; Origen doctrinal / definición y alcance de la U en el Sistema Vectorial SV; Compilador doctrinal y célula meta SV(9 /3)-IA; acta estratégica publicada en beta/ACTA_DECISION_TERNARIA_Y_HIBERNACION_BETA.md.
- **Artefactos afectados:** beta/ACTA_DECISION_TERNARIA_Y_HIBERNACION_BETA.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv; repositorio privado SV-matematica-semantica-cuaternaria.
- **Evidencia:** evidencia de publicación del acta en beta/; contraste adversarial del corpus publicado; decisión estratégica consolidada en acta y registro.
- **Impacto:** trazabilidad; gobierno_tecnico; mantenibilidad; compatibilidad.
- **Objeción adversarial considerada:** riesgo de congelar prematuramente una vía cuaternaria potencialmente fértil y riesgo simétrico de dispersar recursos vitales en dos frentes cuando el corpus vigente todavía sostiene la suficiencia operativa y conceptual de la ternaria.
- **Decisión:** mantener activa la vía ternaria como único camino de desarrollo inmediato; hibernar la Beta como frente doctrinal activo; reabrir Beta solo si aparece contradicción local suficiente / consistente y no resoluble limpiamente en 0/1/U / sin estadística / inferencia ni minería de datos.
- **Estado:** cerrado

### RETP-2026-007

- **Fecha:** 16/03/2026
- **Hora (Europe/Madrid):** 04:50:23
- **Tipo de hito:** DECISION_ARQUITECTONICA
- **Frente/Fase:** Gobierno técnico / control de escalado a Ruta Beta / Fase IV
- **Resumen del cambio:** Se fija en el registro técnico que toda unidad encargada que concluya motivadamente que procede abrir la Ruta Beta deberá elevar antes un informe al núcleo de coordinación Beta / que valorará si autoriza o no el salto.
- **Motivo o argumento:** La hibernación estratégica de Beta exige una compuerta de control adicional para evitar reaperturas por fatiga / presión / anticipación o confusión entre complejidad local y contradicción material suficiente de la ternaria.
- **Base doctrinal o técnica invocada:** beta/ACTA_DECISION_TERNARIA_Y_HIBERNACION_BETA.md; primacía activa de la ternaria; criterio de contradicción local suficiente; criterio interno de relevo.
- **Artefactos afectados:** docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** instrucción expresa del responsable del proyecto; decisión estratégica previa de hibernación Beta; auditoría operativa de coherencia entre Beta y MICROPARCHE C.1.
- **Impacto:** trazabilidad; gobierno_tecnico; control_de_escalado; mantenibilidad.
- **Objeción adversarial considerada:** riesgo de introducir una capa adicional de autorización o de desviar a Beta cuestiones que todavía deben resolverse limpiamente en Ruta B o Ruta A.
- **Decisión:** exigir informe previo al núcleo de coordinación Beta como condición de valoración final antes de autorizar cualquier salto propuesto a Ruta Beta por una unidad encargada.
- **Estado:** cerrado

### RETP-2026-008

- **Fecha:** 18/03/2026
- **Hora (Europe/Madrid):** 16:31:19
- **Tipo de hito:** DECISION_ARQUITECTONICA
- **Frente/Fase:** Continuidad del frente básico y habilitación del backend / Fase IV
- **Resumen del cambio:** Se incorpora un informe público en docs/arquitectura/ para declarar que la pendencia material de los sentidos restantes del frente básico no bloquea el avance del lenguaje SV hacia backend / siempre que el núcleo preserve puertos de extensión previstos y mantenga la subordinación doctrinal y técnica vigente.
- **Motivo o argumento:** La materialización ya observable de semántica / visión y motricidad permite afirmar una equivalencia estructural de clase entre los carriles ya abiertos y los sentidos restantes / de modo que éstos deben tratarse como extensiones subordinadas del frente básico y no como condición previa de refundación del núcleo.
- **Base doctrinal o técnica invocada:** programa de interfaces del Sistema Vectorial SV; IR canónica y bienformación del lenguaje SV — v0.2; Nota de arquitectura mínima del núcleo enganchable del lenguaje SV; README del repositorio SV-lenguaje-de-computacion.
- **Artefactos afectados:** docs/arquitectura/INFORME_DE_CONTINUIDAD_DEL_FRENTE_BASICO_Y_HABILITACION_DEL_BACKEND_SV.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** contraste público entre el programa de interfaces ya abierto en semántica / visión y motricidad / la IR v0.2 vigente / el criterio de núcleo enganchable ya fijado en docs/arquitectura/ y el estado público del repositorio del lenguaje.
- **Impacto:** trazabilidad; gobierno_tecnico; compatibilidad; mantenibilidad.
- **Objeción adversarial considerada:** riesgo de petrificar prematuramente un núcleo incapaz de recibir futuros carriles perceptivos o de presentar el informe como cierre doctrinal superior cuando su naturaleza real es pública / técnica y subordinada.
- **Decisión:** declarar legítima la continuidad del backend bajo condición expresa de preservar puertos de extensión previstos y registrar el criterio como decisión arquitectónica pública en el repositorio del lenguaje.
- **Estado:** cerrado

### RETP-2026-009

- **Fecha:** 18/03/2026
- **Hora (Europe/Madrid):** 22:30:09
- **Tipo de hito:** DECISION_GOBIERNO_TECNICO
- **Frente/Fase:** Auditoría de repos frescos / fijación de sedes del ecosistema SV / Fase IV
- **Resumen del cambio:** Se fija SV-matematica-semantica como sede superior doctrinal y normativa del ecosistema y SV-lenguaje-de-computacion como sede central operativa y técnica / quedando además el pliego reconocido como ley interna de lectura y coherencia para la fase actual.
- **Motivo o argumento:** La auditoría directa de ambos repositorios muestra que la sede doctrinal ha alcanzado estabilidad suficiente de fase / mientras la fricción real del trabajo / la calidad / la validación y la deuda técnica se concentran en el repositorio del lenguaje / que resulta más apto como centro de gobierno operativo.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; auditoría cerrada de SV-matematica-semantica; auditoría cerrada de SV-lenguaje-de-computacion; jerarquía documental ya fijada; regla de no corrección silenciosa entre niveles.
- **Artefactos afectados:** docs/calidad/ACTA_DE_FIJACION_DE_SEDES_Y_GOBIERNO_TECNICO_DEL_ECOSISTEMA_SV_2026_03_18.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv; SV-matematica-semantica/documentos/registros/NOTA_DE_REMISION_SOBRE_SEDE_DOCTRINAL_Y_SEDE_OPERATIVA_DEL_ECOSISTEMA_SV_2026_03_18.md.
- **Evidencia:** ingestión de ZIP frescos de ambos repositorios; verificación directa de jerarquía / estructura / suite y documentos de calidad; cierre adversarial de Fases 1 / 2 y 3 del cruce entre sedes.
- **Impacto:** gobierno_tecnico; trazabilidad; mantenibilidad; coherencia_ecosistema.
- **Objeción adversarial considerada:** riesgo de convertir la sede operativa en autoridad doctrinal de facto o / en el extremo opuesto / de seguir tratando la sede doctrinal como taller técnico diario y contaminar su estabilidad de fase.
- **Decisión:** mantener arriba la sede doctrinal y abajo la sede operativa / impedir la reapertura ordinaria de esta distribución y permitir solo tres supuestos de reconsideración: revisión del pliego obligada por la realidad del sistema / requerimiento muy complejo del lenguaje o acumulación de elementos parciales con fuerza suficiente de reconsideración.
- **Estado:** cerrado

### RETP-2026-010

- **Fecha:** 18/03/2026
- **Hora (Europe/Madrid):** 23:45:11
- **Tipo de hito:** DECISION_GOBIERNO_TECNICO
- **Frente/Fase:** Gobierno operativo del lenguaje / pliego doctrinal enlazado / manual SVP local / Wishlist IRQ / Fase IV
- **Resumen del cambio:** Se fijan el bloque local de gobierno operativo / el constructor del manual del lenguaje SVP con extensión .svp y el Wishlist IRQ del ecosistema SV / manteniendo la consulta pública consolidada del pliego en la sede doctrinal superior.
- **Motivo o argumento:** Era necesario ordenar dos frentes operativos del repositorio del lenguaje —la construcción progresiva del manual y la gestión disciplinada de la presión creativa lateral— sin desplazar la sede doctrinal superior ni duplicar innecesariamente el pliego y su bloque gráfico.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; fijación de sedes del ecosistema; frontera normativa del lenguaje SV; IR canónica v0.2; regla de segregación entre ruta activa y Beta.
- **Artefactos afectados:** docs/README.md; docs/gobierno/README.md; docs/gobierno/MANIFIESTO_PLIEGO_Y_ASSETS.md; docs/gobierno/WISHLIST_IRQ_DEL_ECOSISTEMA_SV.md; docs/gobierno/WISHLIST_IRQ_DEL_ECOSISTEMA_SV.csv; docs/manual_svp/README.md; docs/manual_svp/CONSTRUCTOR_DEL_MANUAL_DEL_LENGUAJE_SVP.md; docs/manual_svp/INDICE_Y_ESTADO_DEL_MANUAL_DEL_LENGUAJE_SVP.csv.
- **Evidencia:** árbol real del repositorio operativo; existencia local del bloque de gobierno y del bloque manual SVP; remisión documental expresa del pliego a la sede doctrinal superior.
- **Impacto:** gobierno_tecnico; trazabilidad; mantenibilidad; autonomia_unidades; control_de_presion_creativa.
- **Objeción adversarial considerada:** riesgo de multiplicar documentos sin eje rector o de abrir un circuito de ideas laterales que compita con la ruta activa del lenguaje.
- **Decisión:** mantener el pliego como referencia pública en la sede doctrinal superior / usar el constructor del manual como máquina de cierre progresivo del manual SVP y canalizar toda idea lateral por el Wishlist IRQ antes de convertirla en tarea o frente real.
- **Estado:** cerrado

### RETP-2026-011

- **Fecha:** 18/03/2026
- **Hora (Europe/Madrid):** 23:58:40
- **Tipo de hito:** DECISION_GOBIERNO_TECNICO
- **Frente/Fase:** Gobierno operativo del lenguaje / compatibilidad interlenguajes / lista de deseos IRQ / Fase IV
- **Resumen del cambio:** Se activa la política y protocolo de compatibilidad interlenguajes del Lenguaje SV / se incorpora la entrada WIRQ-2026-001 como primer deseo sustantivo de la lista de deseos IRQ y se amplía el constructor del manual SVP con un tramo específico de compatibilidad y adopción.
- **Motivo o argumento:** Era necesario dejar ya ordenada la futura compatibilidad del lenguaje con Python / Rust / C / C++ / Kotlin y la vigilancia de legado como Cobol para evitar deuda estructural y lluvia de ideas no gobernada.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; frontera normativa del lenguaje SV; IR canónica v0.2; constructor del manual SVP; lista de deseos IRQ del ecosistema SV.
- **Artefactos afectados:** docs/gobierno/WISHLIST_IRQ_DEL_ECOSISTEMA_SV.md; docs/gobierno/WISHLIST_IRQ_DEL_ECOSISTEMA_SV.csv; docs/gobierno/POLITICA_Y_PROTOCOLO_DE_COMPATIBILIDAD_INTERLENGUAJES_DEL_LENGUAJE_SV.md; docs/manual_svp/CONSTRUCTOR_DEL_MANUAL_DEL_LENGUAJE_SVP.md; docs/manual_svp/INDICE_Y_ESTADO_DEL_MANUAL_DEL_LENGUAJE_SVP.csv; docs/manual_svp/TRAMO_10_COMPATIBILIDAD_INTERLENGUAJES_Y_ADOPCION_DEL_LENGUAJE_SV.md; docs/calidad/ACTA_DE_ACTIVACION_DE_POLITICA_DE_COMPATIBILIDAD_INTERLENGUAJES_Y_REGISTRO_CERO_DE_LISTA_DE_DESEOS_IRQ_2026_03_18.md.
- **Evidencia:** Construcción deliberada del bloque de compatibilidad; registro del deseo sustantivo inicial; ampliación del constructor del manual.
- **Impacto:** gobierno_tecnico; trazabilidad; adopcion_ecosistema; mantenibilidad; control_de_presion_creativa.
- **Objeción adversarial considerada:** riesgo de abrir demasiado pronto un frente de bindings concretos y riesgo simétrico de no dejar sembrada a tiempo la política de compatibilidad y adopción del lenguaje.
- **Decisión:** activar ya la política y el protocolo / mantener abierta la línea en lista de deseos IRQ con IRQ-2 y abrir un tramo específico del manual SVP / dejando las implementaciones concretas por lenguaje para fases posteriores gobernadas por evidencia.
- **Estado:** cerrado

### RETP-2026-012

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 06:39:08
- **Tipo de hito:** APERTURA_FASE
- **Frente/Fase:** Frente final del lenguaje SV / cierre operativo del lenguaje / Fase IV
- **Resumen del cambio:** Se abre formalmente el frente final del lenguaje SV / se fijan hoja de ruta / criterios de cierre / registro de deuda viva y tablero de bloques cerrables bajo gobierno técnico del repositorio operativo.
- **Motivo o argumento:** Una vez fijados pliego / sedes / publicación base / wishlist IRQ / política de compatibilidad y constructor del manual / el trabajo vivo del ecosistema queda concentrado en el cierre técnico del lenguaje y exige un marco operativo explícito.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; frontera normativa del lenguaje SV v0; IR canónica v0.2; nota de arquitectura mínima del núcleo enganchable; informe de continuidad del frente básico y habilitación del backend.
- **Artefactos afectados:** docs/arquitectura/ACTA_DE_APERTURA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV_2026_03_19.md; docs/arquitectura/HOJA_DE_RUTA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md; docs/arquitectura/CRITERIOS_DE_CIERRE_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md; docs/arquitectura/README.md; docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md; docs/calidad/TABLERO_DE_BLOQUES_CERRABLES_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.csv; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv; docs/README.md
- **Evidencia:** cierre previo de auditoría de repos frescos / fijación de sedes del ecosistema / publicación del pliego y consolidación del gobierno operativo de fase.
- **Impacto:** gobierno_tecnico; trazabilidad; mantenibilidad; coherencia_ecosistema; preparacion_backend.
- **Objeción adversarial considerada:** riesgo de abrir un nuevo frente nominal sin reducir deuda real o de desplazar prematuramente el esfuerzo al backend soberano sin cerrar antes el núcleo semántico-diagnóstico.
- **Decisión:** abrir el frente exclusivamente para sanear y cerrar mejor el lenguaje vigente / prohibiendo que esta apertura se use como excusa para reabrir doctrina / multiplicar interfaces o adelantar backend sin ABI mínimo fijado.
- **Estado:** abierto

### RETP-2026-013

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 07:20:00
- **Tipo de hito:** APERTURA_BLOQUE
- **Frente/Fase:** Frente final del lenguaje SV / Bloque A — Contrato diagnóstico / Fase IV
- **Resumen del cambio:** Se abre la microauditoría cerrada del Bloque A y se crean matriz de concordancia diagnóstica / guía de lectura y plantilla de dictamen de saneamiento.
- **Motivo o argumento:** La deuda viva principal del frente final sigue concentrándose en la concordancia entre IR / catálogo público / implementación efectiva / validator y suite.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; Frontera normativa del lenguaje SV v0; IR canónica v0.2; hoja de ruta del frente final del lenguaje SV.
- **Artefactos afectados:** docs/calidad/ACTA_DE_APERTURA_DE_MICROAUDITORIA_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO_2026_03_19.md; docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md; docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv; docs/calidad/DICTAMEN_DE_SANEAMIENTO_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO.md; registros técnicos .md/.csv.
- **Evidencia:** paquete operativo de apertura del Bloque A.
- **Impacto:** gobierno_tecnico; trazabilidad; coherencia_diagnostica; mantenibilidad.
- **Objeción adversarial considerada:** riesgo de abrir una microauditoría sin cerrar nada o de duplicar análisis ya realizados sin una matriz única de contraste.
- **Decisión:** abrir la microauditoría solo para producir matriz / clasificación y tratamiento recomendado del contrato diagnóstico vigente.
- **Estado:** abierto

### RETP-2026-014

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 10:07:25
- **Tipo de hito:** DECISION_GOBIERNO_TECNICO
- **Frente/Fase:** Gobierno operativo del lenguaje / normalización de redacción pública / ampliación de Wishlist IRQ / Fase IV
- **Resumen del cambio:** Se amplía la Wishlist IRQ del ecosistema SV con las entradas WIRQ-2026-002 a WIRQ-2026-006 y se normaliza la redacción pública de los documentos afectados para excluir referencias operativas internas.
- **Motivo o argumento:** Era necesario incorporar nuevas líneas de deseo estratégico del responsable del proyecto sin desplazar el Bloque A — Contrato diagnóstico y / al mismo tiempo / depurar la documentación pública para excluir referencias privadas o procedimentales impropias de un registro público.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; Wishlist IRQ del ecosistema SV; registro tecnico vivo del frente final del lenguaje; criterio de bloques cerrables; regla de subordinacion entre ruta activa Wishlist y gobierno tecnico.
- **Artefactos afectados:** docs/gobierno/WISHLIST_IRQ_DEL_ECOSISTEMA_SV.md; docs/gobierno/WISHLIST_IRQ_DEL_ECOSISTEMA_SV.csv; docs/gobierno/README.md; docs/calidad/ACTA_DE_NORMALIZACION_DE_REDACCION_PUBLICA_Y_AMPLIACION_DE_WISHLIST_IRQ_2026_03_19.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** lectura directa del estado público del repositorio operativo; reevaluación adversarial de las IRQ nuevas; revisión de redacción pública y compilación única de artefactos documentales.
- **Impacto:** gobierno_tecnico; trazabilidad; mantenibilidad; control_de_presion_creativa; descubrilidad.
- **Objeción adversarial considerada:** riesgo de mezclar mecánica interna con documentación pública y riesgo simétrico de absorber nuevas líneas de deseo sin un cauce público sobrio y revisable.
- **Decisión:** incorporar las nuevas entradas a la Wishlist IRQ con prioridad diferenciada / mantener su subordinación a la ruta activa del Bloque A y dejar normalizada la redacción pública de los documentos afectados.
- **Estado:** cerrado

### RETP-2026-015

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 12:40:45
- **Tipo de hito:** SANEAMIENTO_TECNICO
- **Frente/Fase:** Frente final del lenguaje SV / Bloque A — Contrato diagnóstico / saneamiento local E007 E008 E009 / Fase IV
- **Resumen del cambio:** Se materializa el saneamiento local E007 E008 E009 con validacion efectiva de conectores y tablas de admisibilidad y con ampliacion de la suite adversarial.
- **Motivo o argumento:** La microauditoria del Bloque A habia identificado deuda real en la emision observable de E007 E008 y E009 y el repositorio ya muestra una cobertura mas sincera y verificable.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; IR canonica de bienformacion v0.2; Bloque A — Contrato diagnostico; criterio de saneamiento minimo por bloque cerrable.
- **Artefactos afectados:** src/svp_parser.py; src/svp_validator.py; tests/run_conformance.py; tests/conformance/invalid/conector_mapping_incompleto.svp; tests/conformance/invalid/conector_target_no_ternario.svp; tests/conformance/invalid/admissibility_table_incompleta.svp; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** revision directa de los PDFs de confirmacion del repositorio publico con presencia material del saneamiento local y de los tres casos invalidos anadidos.
- **Impacto:** coherencia_diagnostica; trazabilidad; mantenibilidad; cobertura_de_suite.
- **Objeción adversarial considerada:** riesgo de confundir materializacion publica de archivos con ejecucion remota de la suite. El registro declara materializacion visible y no afirma ejecucion remota no acreditada.
- **Decisión:** dar por materializado el saneamiento local E007 E008 E009 y dejar pendiente un cierre posterior del subtramo E001 sin mezclarlo en esta ronda.
- **Estado:** cerrado

### RETP-2026-016

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 12:40:46
- **Tipo de hito:** DECISION_GOBIERNO_TECNICO
- **Frente/Fase:** Gobierno operativo del lenguaje / ampliacion de Wishlist IRQ / agentes especializados / Fase IV
- **Resumen del cambio:** Se incorpora WIRQ-2026-007 a la Wishlist IRQ para recoger la generacion de agentes especializados por el lenguaje SV con configuracion precargada y remision estructural a SVperitus.
- **Motivo o argumento:** El deseo tiene alto valor estrategico de adopcion y especializacion experta pero no debe desplazar el Bloque A ni abrir una implementacion prematura.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; Wishlist IRQ del ecosistema SV; criterio de control de fase; plano N4 ya visible en domain agent y query_spec del lenguaje.
- **Artefactos afectados:** docs/gobierno/WISHLIST_IRQ_DEL_ECOSISTEMA_SV.md; docs/gobierno/WISHLIST_IRQ_DEL_ECOSISTEMA_SV.csv; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** revision del estado publico del Wishlist IRQ; lectura de la estructura superficial vigente del lenguaje; evaluacion adversarial de la nueva propuesta de agentes especializados.
- **Impacto:** adopcion_ecosistema; descubrilidad; gobierno_tecnico; control_de_presion_creativa.
- **Objeción adversarial considerada:** riesgo de leer como compromiso de implementacion inmediata una linea que en esta fase solo puede admitirse como deseo estrategico subordinado y coordinado con SVperitus.
- **Decisión:** clasificar WIRQ-2026-007 como IRQ-4 con ruta aparcado y reabrirla solo con mayor madurez del nucleo y delimitacion formal del cruce con SVperitus.
- **Estado:** cerrado

### RETP-2026-017

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 13:00:59
- **Tipo de hito:** SANEAMIENTO_TECNICO
- **Frente/Fase:** Frente final del lenguaje SV / Bloque A — Contrato diagnóstico / saneamiento local E001 E507 / Fase IV
- **Resumen del cambio:** Se descomprime parcialmente la sobrecarga diagnostica de E001 al hacer que la coercion implicita de U mediante null o None emita E507 y al actualizar parser catalogo suite y documentacion publica asociada.
- **Motivo o argumento:** Tras sanear E007 E008 E009 el siguiente subtramo legitimo del Bloque A era reducir la bolsa generica de E001 sin abrir una refundicion completa del parser y el caso u_coercion.svp ofrecia un cierre local real verificable y de bajo radio de impacto.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; IR canonica de bienformacion v0.2; Bloque A — Contrato diagnostico; catalogo implementativo vigente; criterio de saneamiento minimo por bloque cerrable.
- **Artefactos afectados:** src/svp_errors.py; src/svp_parser.py; tests/run_conformance.py; tests/README.md; docs/referencia/ERRORES_CANONICOS_SV_v0_2.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** verificacion local del caso adversarial u_coercion.svp con emision E507 y ejecucion completa de la suite de conformidad en el estado preparado para el paquete.
- **Impacto:** coherencia_diagnostica; trazabilidad; mantenibilidad; cobertura_de_suite.
- **Objeción adversarial considerada:** riesgo de presentar este ajuste como resolucion total de la sobrecarga de E001. El parche no clausura todo E001 y solo extrae de su bolsa generica el subcaso de coercion implicita de U.
- **Decisión:** materializar el saneamiento local E001/E507 como bloque tecnico autonomo y mantener pendiente una futura depuracion mas amplia del parse diagnostico.
- **Estado:** cerrado

### RETP-2026-018

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 13:20:00
- **Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA
- **Frente/Fase:** Frente final del lenguaje SV / Bloque A — contrato diagnostico / sincronizacion publica de suite y catalogo / Fase IV
- **Resumen del cambio:** Se sincronizan tests README y catalogo publico con E007 E008 E009 y E507 ya materializados.
- **Motivo o argumento:** La documentacion publica seguia reflejando un estado anterior de la suite y del catalogo observable.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; Bloque A — Contrato diagnostico; criterio de sincronizacion publica por bloque.
- **Artefactos afectados:** tests/README.md; docs/referencia/ERRORES_CANONICOS_SV_v0_2.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** revision directa del estado publico ya materializado y contraste de documentacion con suite observable.
- **Impacto:** coherencia_diagnostica; trazabilidad; descubrilidad; mantenibilidad.
- **Objeción adversarial considerada:** riesgo de presentar sincronizacion documental como avance funcional adicional.
- **Decisión:** actualizar documentacion publica en bloque minimo separado sin abrir nuevos frentes funcionales.
- **Estado:** cerrado

### RETP-2026-019

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 13:40:00
- **Tipo de hito:** CAMBIO_RUNNER
- **Frente/Fase:** Frente final del lenguaje SV / Bloque A — Contrato diagnostico / ampliacion de cobertura explicita E005 E010 / Fase IV
- **Resumen del cambio:** Se amplía la suite observable con duplicate_identifier.svp e invalid_role_literal.svp y se sincronizan run_conformance README y catálogo público con la nueva cobertura de E005 y E010.
- **Motivo o argumento:** Tras los saneamientos locales previos el siguiente cierre legítimo era aumentar cobertura explícita de códigos ya emitibles y de bajo riesgo sin abrir una refundición más amplia del parse diagnostico.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; Bloque A — Contrato diagnostico; catalogo implementativo vigente; criterio de bloques cerrables; suite observable del frontend de referencia.
- **Artefactos afectados:** tests/run_conformance.py; tests/README.md; tests/conformance/invalid/duplicate_identifier.svp; tests/conformance/invalid/invalid_role_literal.svp; docs/referencia/ERRORES_CANONICOS_SV_v0_2.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** verificacion local de la suite con 22 de 22 casos superados y comprobacion directa de emision E005 y E010 en los dos adversariales nuevos.
- **Impacto:** coherencia_diagnostica; trazabilidad; cobertura_de_suite; mantenibilidad.
- **Objeción adversarial considerada:** riesgo de presentar ampliacion de cobertura como saneamiento funcional nuevo. El paquete hace explicita y verificable en la suite una parte mas del contrato diagnostico ya existente.
- **Decisión:** ampliar de forma controlada la cobertura observable del Bloque A sobre E005 y E010 / mantener E004 como deuda documentada de alcanzabilidad superficial / y reservar para bloque posterior cualquier descompresion adicional de E001.
- **Estado:** cerrado

### RETP-2026-020

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 14:05:00
- **Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA
- **Frente/Fase:** Frente final del lenguaje SV / Bloque A — contrato diagnostico / sinceramiento de cobertura explicita de E001 / Fase IV
- **Resumen del cambio:** Se retira E001 de la lista de codigos cubiertos explicitamente por la suite y se documenta que conserva emision observable sin caso adversarial explicito propio tras la extraccion local hacia E507.
- **Motivo o argumento:** La cobertura publica seguia sobreatribuyendo E001 aunque tests/run_conformance.py ya no declara ningun caso invalido con codigo esperado E001.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; Bloque A — Contrato diagnostico; catalogo implementativo vigente; criterio de sinceramiento publico del estado observable; suite observable del frontend de referencia.
- **Artefactos afectados:** docs/referencia/ERRORES_CANONICOS_SV_v0_2.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** contraste directo entre tests/run_conformance.py y la seccion de cobertura explicita del catalogo publico con ausencia observable de casos invalidos que esperen E001.
- **Impacto:** coherencia_diagnostica; trazabilidad; mantenibilidad; descubrilidad.
- **Objeción adversarial considerada:** riesgo de registrar como hito separado una mera correccion de redaccion. Procede porque la sobreatribucion afecta al contrato diagnostico publico verificable.
- **Decisión:** sincerar la documentacion publica retirando E001 de la cobertura explicita de suite / mantener E001 como codigo con emision observable directa / y reservar para bloque posterior cualquier reintroduccion de cobertura explicita mediante un adversarial propio.
- **Estado:** cerrado

### RETP-2026-021

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 14:25:00
- **Tipo de hito:** CAMBIO_RUNNER
- **Frente/Fase:** Frente final del lenguaje SV / Bloque A — Contrato diagnostico / reintroduccion de cobertura explicita de E001 / Fase IV
- **Resumen del cambio:** Se añade invalid_tri_literal.svp y se sincronizan suite y catalogo publico para devolver a E001 una cobertura explicita limpia y observable.
- **Motivo o argumento:** Tras el sinceramiento previo E001 habia quedado con emision observable directa pero sin adversarial propio y la superficie actual si permite un caso minimo de literal ternario no reconocido.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; Bloque A — Contrato diagnostico; catalogo implementativo vigente; suite observable del frontend de referencia; criterio de bloques cerrables.
- **Artefactos afectados:** tests/run_conformance.py; tests/README.md; tests/conformance/invalid/invalid_tri_literal.svp; docs/referencia/ERRORES_CANONICOS_SV_v0_2.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** verificacion local del nuevo caso adversarial con emision E001 y ejecucion completa de la suite en el estado preparado para el paquete.
- **Impacto:** coherencia_diagnostica; trazabilidad; cobertura_de_suite; mantenibilidad.
- **Objeción adversarial considerada:** riesgo de reabrir la bolsa diagnostica de E001 de forma indiscriminada. El paquete no deshace E001/E507 y solo cubre el subcaso limpio de literal ternario no reconocido.
- **Decisión:** ampliar de forma controlada la cobertura observable de E001 mediante un adversarial minimo especifico y mantener separada cualquier depuracion posterior de otros errores sintacticos absorbidos por E001.
- **Estado:** cerrado

### RETP-2026-022

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 17:10:00
- **Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA
- **Frente/Fase:** Frente final del lenguaje SV / Bloque A — contrato diagnostico / normalizacion fina de dictamen y matriz / Fase IV
- **Resumen del cambio:** Se normalizan dictamen matriz y redaccion publica residual del Bloque A.
- **Motivo o argumento:** Persistian restos de plantilla y desalineaciones documentales tras el barrido E001 E010.
- **Base doctrinal o técnica invocada:** Pliego de condiciones; Bloque A — Contrato diagnostico; suite observable; auditoria de repo fresco.
- **Artefactos afectados:** docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md; docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv; docs/calidad/DICTAMEN_DE_SANEAMIENTO_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO.md; docs/calidad/ACTA_DE_FIJACION_DE_SEDES_Y_GOBIERNO_TECNICO_DEL_ECOSISTEMA_SV_2026_03_18.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** contraste directo entre repo fresco suite matriz dictamen y documentos publicos de calidad.
- **Impacto:** coherencia_diagnostica; trazabilidad; mantenibilidad; redaccion_publica.
- **Objeción adversarial considerada:** riesgo de tratar el bloque como simple retoque editorial sin reconocer su impacto sobre artefactos canonicos del Bloque A.
- **Decisión:** cerrar la limpieza fina de los artefactos canonicos del Bloque A antes de abrir E101 E111.
- **Estado:** cerrado

### RETP-2026-023

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 17:35:00
- **Tipo de hito:** CAMBIO_RUNNER
- **Frente/Fase:** Frente final del lenguaje SV / Bloque A — contrato diagnostico / apertura de familia E101 E111 con cobertura explicita E101 E105 / Fase IV
- **Resumen del cambio:** Se amplía la suite observable con dos adversariales mínimos para E101 y E105 y se sincronizan tests catálogo y matriz diagnóstica con esa cobertura explícita inicial de la familia E101 E111.
- **Motivo o argumento:** La matriz canónica vigente situaba E101 y E105 como códigos con emisión directa observable pero todavía sin cobertura explícita de suite y eran los dos cierres locales de menor radio de impacto para abrir legítimamente la familia E101 E111.
- **Base doctrinal o técnica invocada:** Pliego de condiciones del Sistema Vectorial SV; Bloque A — Contrato diagnóstico; matriz diagnóstica canónica; catálogo implementativo vigente; suite observable del frontend de referencia; criterio de bloques cerrables.
- **Artefactos afectados:** tests/run_conformance.py; tests/README.md; tests/conformance/invalid/cellstate_vector_length_mismatch.svp; tests/conformance/invalid/bridge_position_fuera_de_rango.svp; docs/referencia/ERRORES_CANONICOS_SV_v0_2.md; docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md; docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** preparación del paquete técnico sobre estado fresco ya sincronizado y verificación local de la suite con dos adversariales mínimos añadidos.
- **Impacto:** coherencia_diagnostica; trazabilidad; cobertura_de_suite; mantenibilidad.
- **Objeción adversarial considerada:** riesgo de presentar como apertura de familia una refundición amplia de la capa de estado. El bloque se limita a dos cierres locales ya emitibles y de bajo radio.
- **Decisión:** abrir la familia E101 E111 por E101 y E105 y reservar para la siguiente ronda el contraste fino E102 / E104 / E106 / E111.
- **Estado:** cerrado

### RETP-2026-024

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 18:20:00
- **Tipo de hito:** VERIFICABILIDAD_EXTERNA
- **Frente/Fase:** Frente final del lenguaje SV / verificabilidad externa minima / FrameComparison ejemplos y senalizacion historica / Fase IV
- **Resumen del cambio:** Se refuerza la verificabilidad externa del repositorio con normalizacion de FrameComparison ejemplos canonicos minimos y sondas adversariales documentadas.
- **Motivo o argumento:** La lectura adversarial externa habia detectado un riesgo de credibilidad publica por redundancia lexica ausencia de ejemplos y senalizacion historica insuficiente en raiz.
- **Base doctrinal o técnica invocada:** Pliego de condiciones del Sistema Vectorial SV; Bloque A — Contrato diagnostico; criterio de verificabilidad externa minima; suite publica vigente; analisis adversarial externo contrastado con repo fresco.
- **Artefactos afectados:** src/svp_lexer.py; src/svp_parser.py; README.md; src/README.md; examples/README.md; examples/celula_basica_con_evaluacion.svp; examples/consulta_framecomparison.svp; tests/adversarial/README.md; tests/adversarial/documentados/README.md; tests/adversarial/documentados/celula_aislada_con_resolucion_u.svp; tests/adversarial/documentados/composicion_serie_con_trayectoria.svp; tests/adversarial/documentados/agente_con_consulta_y_dominio.svp; CONCORDANCIA_CATALOGO_ERRORES_IR_v0_2_IMPLEMENTACION_PARCHE_1A.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** contraste directo sobre repo fresco y verificacion local de la suite de conformidad tras la normalizacion de FrameComparison.
- **Impacto:** verificabilidad_externa; trazabilidad; descubrilidad; redaccion_publica; mantenibilidad.
- **Objeción adversarial considerada:** riesgo de mezclar ejemplos publicos con sondas adversariales y de publicar materiales hibridos sin estatuto claro.
- **Decisión:** cerrar primero un bloque corto de verificabilidad externa minima con dos carriles publicos separados y dejar para la siguiente ronda el contraste fino E102 / E104 / E106 / E111.
- **Estado:** cerrado

### RETP-2026-025

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 19:20:00
- **Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA
- **Frente/Fase:** Frente final del lenguaje SV / Bloque A — contrato diagnostico / contraste fino E102 E104 E106 E111 / Fase IV
- **Resumen del cambio:** Se ejecuta el contraste fino publico de E102 E104 E106 y E111 mediante sondas adversariales documentadas y sincronizacion de matriz dictamen y registro.
- **Motivo o argumento:** Tras el cierre de verificabilidad externa minima el siguiente cuello de botella era sincerar el subtramo residual E102 / E104 / E106 / E111 antes de abandonar la familia E101 E111.
- **Base doctrinal o técnica invocada:** Pliego de condiciones del Sistema Vectorial SV; Bloque A — Contrato diagnostico; matriz diagnostica canonica; criterio de contraste fino por subtramos; repo fresco del frontend de referencia; capa publica de sondas adversariales documentadas.
- **Artefactos afectados:** tests/adversarial/documentados/README.md; tests/adversarial/documentados/e102_output_semantics_ausente_contraste.svp; tests/adversarial/documentados/e104_codominio_de_conector_invalido_contraste.svp; tests/adversarial/documentados/e106_relacion_semantica_ausente_contraste.svp; tests/adversarial/documentados/e111_codominio_sin_orden_total_contraste.md; docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md; docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv; docs/calidad/DICTAMEN_DE_SANEAMIENTO_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** contraste directo sobre repo fresco y lectura comparada entre IR v0.2 catalogo publico efectivo implementacion validator y superficie adversarial documentada.
- **Impacto:** coherencia_diagnostica; trazabilidad; verificabilidad_externa; redaccion_publica; mantenibilidad.
- **Objeción adversarial considerada:** riesgo de presentar como cierre funcional lo que en realidad es un sinceramiento documental. El bloque no afirma convergencia semantica nueva y solo deja documentado con mayor precision el subtramo residual.
- **Decisión:** dar por ejecutado el contraste fino de E102 / E104 / E106 / E111 y trasladar el siguiente barrido legitimo a E201–E211.
- **Estado:** cerrado

### RETP-2026-026

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 20:10:00
- **Tipo de hito:** CAMBIO_RUNNER
- **Frente/Fase:** Frente final del lenguaje SV / Bloque A — contrato diagnóstico / apertura de familia E201 E211 con cobertura explícita E202 E204 E205 / Fase IV
- **Resumen del cambio:** Se amplía la suite observable con tres adversariales mínimos para E202 E204 y E205 y se sincronizan tests catálogo matriz dictamen y registro con esa cobertura explícita inicial de la familia E201–E211.
- **Motivo o argumento:** Dentro de la familia E201–E211 / E202 / E204 y E205 eran los subcasos de menor radio y emisión más directa en la superficie publicada.
- **Base doctrinal o técnica invocada:** Pliego de condiciones del Sistema Vectorial SV / Bloque A — Contrato diagnóstico / matriz diagnóstica canónica / catálogo implementativo vigente / suite observable del frontend.
- **Artefactos afectados:** tests/run_conformance.py / tests/README.md / gate_input_no_evalresult.svp / query_context_opaco.svp / supervise_target_opaco.svp / docs/referencia/ERRORES_CANONICOS_SV_v0_2.md / matriz / dictamen / registro.
- **Evidencia:** paquete técnico sobre repo fresco tras RETP-2026-025 / verificación local de suite con tres adversariales mínimos.
- **Impacto:** coherencia_diagnostica / trazabilidad / cobertura_de_suite / mantenibilidad.
- **Objeción adversarial considerada:** riesgo de sobreatribuir cierre total de la familia E201–E211.
- **Decisión:** abrir la familia por sus tres casos directamente emitibles y reservar el resto residual para contraste fino posterior.
- **Estado:** cerrado

### RETP-2026-027

- **Fecha:** 19/03/2026
- **Hora (Europe/Madrid):** 23:10:34
- **Tipo de hito:** SORPRESA_TECNICA
- **Frente/Fase:** Frente final del lenguaje SV / gobernanza de coherencia entre frame histórico reapertura y consulta presente / Fase IV
- **Resumen del cambio:** Se registra una nota técnica de calidad para gobernar la tensión entre trayectoria histórica retorno legítimo a U cobertura y admisibilidad vigentes y consulta presente sin reabrir doctrina ni elevar aún el asunto a paper público.
- **Motivo o argumento:** El frente final del lenguaje exige cierre honesto entre IR validator runner capa de consulta y documentación pública. La tensión detectada no es wishlist ni papel doctrinal nuevo pero sí riesgo gobernable del frente.
- **Base doctrinal o técnica invocada:** sede doctrinal principal sobre horizonte transducción consulta y U / acta de apertura del frente final / IR canónica v0.2 / semántica formal de U en SVP.
- **Artefactos afectados:** docs/calidad/NOTA_TECNICA_SOBRE_FRAME_HISTORICO_REAPERTURA_Y_CONSULTA_PRESENTE_2026_03_19.md / docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md / docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv / docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md
- **Evidencia:** lectura adversarial comparada del repositorio doctrinal principal y del repositorio del lenguaje con contraste expreso entre trayectoria U transducción y consulta.
- **Impacto:** trazabilidad / gobierno_tecnico / fiabilidad / mantenibilidad
- **Objeción adversarial considerada:** riesgo de duplicar doctrina ya existente y de sobreactuar como crisis basal un problema ya previsto parcialmente por la sede doctrinal superior.
- **Decisión:** registrar la tensión como nota técnica subordinada de calidad abrir deuda viva específica y reservar cualquier eventual publicación doctrinal nueva para un momento posterior solo si la bajada operativa acreditara insuficiencia real del marco vigente.
- **Estado:** cerrado

### RETP-2026-028

- **Fecha:** 20/03/2026
- **Hora (Europe/Madrid):** 11:45:22
- **Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA
- **Frente/Fase:** Saneamiento de concordancia del bloque pliego-manual / Fase IV
- **Resumen del cambio:** Se regularizan docs/README.md / docs/gobierno/ / docs/manual_svp/ y el registro técnico para que la documentación pública del repositorio operativo refleje exactamente el árbol real: pliego y bloque gráfico en sede doctrinal superior / manual SVP de trabajo en sede operativa local.
- **Motivo o argumento:** La microauditoría adversarial del repositorio operativo constató discordancias entre varias afirmaciones documentales y la presencia material real de artefactos. Era necesario sincerar el bloque sin reabrir sedes ni multiplicar copias del pliego.
- **Base doctrinal o técnica invocada:** fijación de sedes del ecosistema SV; pliego de condiciones del Sistema Vectorial SV; protocolo interno ZIP → GitHub → Verificación → Registro → Cierre; árbol real del repositorio operativo contrastado sobre repo fresco.
- **Artefactos afectados:** docs/README.md; docs/gobierno/README.md; docs/gobierno/MANIFIESTO_PLIEGO_Y_ASSETS.md; docs/manual_svp/README.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv; docs/calidad/ACTA_DE_SANEAMIENTO_DE_CONCORDANCIA_DEL_BLOQUE_PLIEGO_MANUAL_RETP_2026_010_2026_03_20.md.
- **Evidencia:** lectura completa del repo fresco SV-lenguaje-de-computacion-main; contraste del árbol real con README / manifiesto / registro y bloque manual; verificación de que el manual SVP permanece local en docs/manual_svp/ y de que el pliego canónico se consulta en SV-matematica-semantica.
- **Impacto:** trazabilidad; gobierno_tecnico; mantenibilidad; fiabilidad.
- **Objeción adversarial considerada:** riesgo de presentar como carencia la ausencia de una copia local del pliego o de arrastrar una remisión ya obsoleta del manual SVP a la sede doctrinal.
- **Decisión:** reescribir el bloque documental y registral para formular de modo positivo y exacto la distribución vigente: pliego en sede doctrinal superior / manual SVP en sede operativa local.
- **Estado:** cerrado

### RETP-2026-029

- **Fecha:** 20/03/2026
- **Hora (Europe/Madrid):** 15:20:00
- **Tipo de hito:** CAMBIO_FUNCIONAL_GOBERNADO
- **Frente/Fase:** Frente final del lenguaje SV / Bloque E — ABI semántico-diagnóstico y endurecimiento inicial de N4/Uso / Fase IV
- **Resumen del cambio:** Se materializa un contrato autónomo de enganche de interfaces futuras y se endurece la validación mínima de `Domain` `Agent` `QuerySpec` y `query`, sincronizando suite ejemplos catálogo público matriz diagnóstica y registro de deuda viva.
- **Motivo o argumento:** La pregunta de continuidad del backend exigía pasar de una mera nota arquitectónica a un contrato operativo verificable; el repo ya contenía la forma IR del enganche futuro pero faltaba volverla contrato diagnóstico observable.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; nota de arquitectura del núcleo enganchable; IR canónica v0.2; criterios de cierre del frente final; repo fresco auditado.
- **Artefactos afectados:** docs/arquitectura/CONTRATO_DE_ENGANCHE_DE_INTERFACES_FUTURAS_Y_ABI_SEMANTICO_DIAGNOSTICO_MINIMO.md; src/svp_validator.py; src/svp_errors.py; tests/run_conformance.py; tests/conformance/valid/query_context_all_variants.svp; tests/conformance/valid/query_context_all_variants.expected.json; tests/conformance/invalid/domain_chain_mismatch.svp; tests/conformance/invalid/agent_architecture_mismatch.svp; tests/conformance/invalid/query_context_type_mismatch.svp; tests/adversarial/deep_nested_query_valid.svp; examples/consulta_framecomparison.svp; docs/referencia/ERRORES_CANONICOS_SV_v0_2.md; docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md; docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv; docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** repo fresco + lectura cruzada con pliego + ejecución local de tests de conformidad smoke CLI y SEC-0 tras el endurecimiento de N4/Uso.
- **Impacto:** trazabilidad; gobierno_tecnico; fiabilidad; mantenibilidad; enganchabilidad_futura.
- **Objeción adversarial considerada:** riesgo de sobreatribuir cierre total del frente N4/Uso o de convertir el ABI mínimo en doctrina material exhaustiva sobre futuras interfaces.
- **Decisión:** fijar únicamente la cadena mínima de enganche y su validación observable; dejar reconocida la deuda residual sobre campos opacos y reservar para rondas posteriores cualquier semántica material adicional.
- **Estado:** cerrado

### RETP-2026-030

- **Fecha:** 20/03/2026
- **Hora (Europe/Madrid):** 16:51:49
- **Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA
- **Frente/Fase:** Frente final del lenguaje SV / Bloque A — regularización dura post RETP-2026-026 y RETP-2026-029 / Fase IV
- **Resumen del cambio:** Se sanean el dictamen del Bloque A y el registro técnico para eliminar la contradicción sobre el siguiente paso, fijar el estado real de E201–E211, E301–E304 y E401–E403, y dejar explícito que E303 no dispone aún de emisión específica autónoma acreditada.
- **Motivo o argumento:** Tras la materialización de RETP-2026-026 y RETP-2026-029 persistía una incoherencia documental interna sobre la secuencia real del Bloque A. Era necesario sanearla sin reabrir funcionalidad.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; criterio de bloques cerrables; prioridad de concordancia fuerte entre IR, catálogo, validator, runner y documentación pública; repo fresco auditado.
- **Artefactos afectados:** docs/calidad/DICTAMEN_DE_SANEAMIENTO_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** lectura dura y adversarial del repo fresco; contraste cruzado entre dictamen, catálogo, deuda viva y registro técnico; verificación local de tests/run_conformance.py, tests/run_cli_smoke.py y tests/run_sec0_smoke.py sin regresión.
- **Impacto:** trazabilidad; gobierno_tecnico; mantenibilidad; fiabilidad.
- **Objeción adversarial considerada:** riesgo de convertir un saneamiento documental en reapertura funcional encubierta o en maquillaje de cierre.
- **Decisión:** regularizar solo la capa documental y registral, dejar explícita la deuda residual de la familia E301–E304 y reservar cualquier apertura funcional posterior para un bloque independiente y justificado.
- **Estado:** cerrado

### RETP-2026-031

- **Fecha:** 20/03/2026
- **Hora (Europe/Madrid):** 17:12:51
- **Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA
- **Frente/Fase:** Frente final del lenguaje SV / Bloque A — regularización pública de la emitibilidad real de E301–E304 / Fase IV
- **Resumen del cambio:** Se sincronizan el catálogo público y el registro técnico con la microauditoría E para dejar explícito que E304 sí es observable y cubierto, mientras E301, E302 y E303 no comparten todavía esa misma emitibilidad pública efectiva.
- **Motivo o argumento:** Tras RETP-2026-030 la capa interna ya había quedado saneada, pero la referencia pública seguía pudiendo inducir una lectura homogénea de la familia E301–E304. La microauditoría E mostró que E304 sí está materializado, que E301 y E302 dependen hoy de invariantes de tipo sin operación superficial publicada, y que E303 permanece sin emisión autónoma acreditada.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_1.md; IR_CANONICA_BIENFORMACION_SV_v0_2.md; src/svp_parser.py; src/svp_validator.py; src/svp_errors.py; docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md; docs/calidad/DICTAMEN_DE_SANEAMIENTO_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO.md; repo fresco auditado.
- **Artefactos afectados:** docs/referencia/ERRORES_CANONICOS_SV_v0_2.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv.
- **Evidencia:** lectura con zoom corto de gramática, parser, validator y catálogo; contraste de casos sintéticos mínimos sobre TransitionData que muestran caída de ausencia de horizon_ref en parseo y de referencia ausente o de tipo incorrecto en E006; verificación local posterior de tests/run_conformance.py, tests/run_cli_smoke.py y tests/run_sec0_smoke.py sin regresión.
- **Impacto:** trazabilidad; redaccion_publica; coherencia_diagnostica; mantenibilidad; verificabilidad_externa.
- **Objeción adversarial considerada:** riesgo de rebajar ilegítimamente el catálogo implementativo o de contradecir el pliego al reinterpretar desde abajo la familia E301–E304. No procede: el lote no modifica parser, validator, IR ni gramática; solo regulariza la lectura pública de la emitibilidad ya constatada por evidencia directa.
- **Decisión:** sincerar la superficie pública de referencia para que el catálogo no sugiera una homogeneidad falsa dentro de E301–E304, mantener E304 como único cierre plenamente observable de la familia en la superficie actual y reservar cualquier eventual apertura funcional posterior para un bloque independiente y justificado.
- **Estado:** cerrado

### RETP-2026-032

- **Fecha:** 22/03/2026
- **Hora (Europe/Madrid):** NO_CONSTA
- **Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA
- **Frente/Fase:** UCBC provisional / regularización integral de numeración subordinada y vigilancia focal sobre suceso local / Fase IV
- **Resumen del cambio:** Se regulariza el bloque UCBC para subordinar la numeración RETP al CSV maestro, se desincrustan identificadores registrales de las actas UCBC y se incorpora la nota pública sobre suceso local y horizonte declarado a la matriz de vigilancia temprana sin abrir deuda viva nueva.
- **Motivo o argumento:** El CSV maestro termina en RETP-2026-031 y por tanto no procede arrastrar una numeración incrustada en un acta como si gobernara el registro; era necesario corregir ahora la divergencia para evitar recaídas futuras y dejar toda la trazabilidad subordinada al registro maestro.
- **Base doctrinal o técnica invocada:** CSV maestro del registro técnico; pliego de condiciones del Sistema Vectorial SV; criterio de vigilancia UCBC subordinada; HTML fresco de la pieza publicada; repos frescos auditados.
- **Artefactos afectados:** docs/calidad/ACTA_TECNICA_DE_VIGILANCIA_UCBC_SOBRE_INTERFACES_Y_ALERTA_TEMPRANA_AL_LENGUAJE_SV_2026_03_21.md; docs/calidad/ACTA_TECNICA_DE_VIGILANCIA_UCBC_SOBRE_NOTA_DE_PRECISION_DE_SUCESO_LOCAL_Y_HORIZONTE_DECLARADO_2026_03_22.md; docs/calidad/MATRIZ_DE_VIGILANCIA_TEMPRANA_UCBC_INTERFACES_LENGUAJE_SV.md; docs/calidad/MATRIZ_DE_VIGILANCIA_TEMPRANA_UCBC_INTERFACES_LENGUAJE_SV.csv; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md; docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv
- **Evidencia:** contraste adversarial entre CSV maestro, acta UCBC del 21/03/2026 y lote de vigilancia focal del 22/03/2026; verificación de que el CSV termina en 031 y de que la nueva regularización restablece una única autoridad de numeración.
- **Impacto:** trazabilidad; gobierno_tecnico; mantenibilidad; fiabilidad; disciplina_registral
- **Objeción adversarial considerada:** riesgo de conservar una pseudo-continuidad cómoda entre el número incrustado en un acta y el siguiente lote, perpetuando así una fuente doble de verdad. No procede: el CSV manda y los documentos quedan subordinados a él.
- **Decisión:** regularizar de forma total el bloque UCBC: el registro maestro reasume la numeración RETP, las actas quedan sin numeración incrustada y la nueva vigilancia focal entra como parte del mismo hito registral 032.
- **Estado:** cerrado

## 13. Regla de estilo

Este registro debe mantenerse con lenguaje formal, sobrio, técnico y revisable por terceros. Las entradas deben describir hechos, fundamento, evidencia, impacto y estado.

| RETP-2026-039 | 23/03/2026 | NO_CONSTA | CAMBIO_GOBIERNO_TECNICO_Y_CONTROL_DE_HITOS | Consolidación auditada de H2 restringido / Fase IV | Se consolida la serie H2.1–H2.3 y se deja asentado que H2 restringido queda respaldado por auditoría mínima suficiente, sin verificarse todavía como hito positivo. |


## RETP-2026-039 — Consolidación auditada de H2 restringido

- **Fecha:** 23/03/2026
- **Hora (Europe/Madrid):** NO_CONSTA
- **Tipo de hito:** CAMBIO_GOBIERNO_TECNICO_Y_CONTROL_DE_HITOS
- **Frente/Fase:** H2 restringido / Fase IV
- **Resumen del cambio:** Se consolida en un único punto de control la serie auditada H2.1–H2.3, dejando asentado que la apertura restringida de H2 ya no descansa solo en un dictamen de no-bloqueo, sino también en auditoría mínima suficiente de elasticidad de la IR, no-rigidez del validator y no-hipoteca del plano ejecutable.
- **Motivo o argumento:** Tras las microauditorías H2.1, H2.2 y H2.3, la base material de H2 restringido quedó suficientemente respaldada para consolidación registral, sin que ello equivalga a verificación positiva de H2 ni apertura de H3 o backend.
- **Base doctrinal o técnica invocada:** `docs/arquitectura/MARCO_ESTABILIDAD_RESILIENCIA_LENGUAJE_SV.md`; `IR_CANONICA_BIENFORMACION_SV_v0_2.md`; `src/svp_validator.py`; `src/svp_main.py`; `docs/calidad/REGISTRO_CALIDAD_HITOS_LENGUAJE_SV.csv`; `docs/calidad/DEUDA_VIVA_HITOS_LENGUAJE_SV.csv`; `docs/calidad/MATRIZ_UCBC_HORIZONTES_LENGUAJE_SV.csv`; repo fresco auditado con suite en verde.
- **Artefactos afectados:** `docs/calidad/README.md`; `docs/calidad/ACTA_TECNICA_DE_CONSOLIDACION_AUDITADA_DE_H2_RESTRINGIDO_2026_03_23.md`; `docs/calidad/REGISTRO_CALIDAD_HITOS_LENGUAJE_SV.csv`; `docs/calidad/DEUDA_VIVA_HITOS_LENGUAJE_SV.csv`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
- **Evidencia:** microauditorías H2.1, H2.2 y H2.3; lectura fresca real del repo; `tests/run_conformance.py` 31/31; `tests/run_cli_smoke.py` 3/3; `tests/run_sec0_smoke.py` 3/3.
- **Impacto:** `gobierno_tecnico`; `control_de_hitos`; `continuidad_operativa`; `resiliencia_prebackend`.
- **Objeción adversarial considerada:** Riesgo de presentar la superación de H2.1–H2.3 como si implicara H2 verificado, o de mantener H2 como mera hipótesis negativa de no-bloqueo pese a haber pasado ya la auditoría mínima suficiente. No procede ninguno de los dos extremos: la decisión correcta es consolidar H2 restringido sin verificar positivamente el hito.
- **Decisión:** consolidar registralmente la serie H2 restringido, mantener `Q-04` y `Q-05` en `Pendiente`, y rebajar la gravedad estructural inmediata de `D-01`, `D-02` y `D-03` sin cerrarlas.
- **Estado:** cerrado

## 14. Vigencia

Este registro permanecerá vigente mientras el proyecto requiera trazabilidad técnica ligera y controlada de su evolución, sin perjuicio de futuras versiones más estructuradas si la complejidad del sistema lo exigiera.

### RETP-2026-033

- **Fecha:** 22/03/2026
- **Hora (Europe/Madrid):** NO_CONSTA
- **Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA
- **Frente/Fase:** Frente final del lenguaje SV / Bloque K — blindaje documental pre-backend del ABI mínimo / Fase IV
- **Resumen del cambio:** Se alinean los README públicos de documentación, arquitectura y gramática con el estado real del frontend, del ABI mínimo y del régimen prudente pre-backend, evitando tanto la invisibilización del contrato de enganche como la apariencia de inmadurez ya superada.
- **Motivo o argumento:** La lectura integral del repo fresco mostró una asimetría pública: el ABI mínimo y la arquitectura mínima ya estaban fijados y materializados, pero no aparecían suficientemente indexados en todos los README rectores; además, `grammar/README.md` seguía presentando la carpeta como pendiente de parser cuando el parser de referencia activo ya existe en `src/`.
- **Base doctrinal o técnica invocada:** README raíz del repositorio; `docs/README.md`; `docs/arquitectura/README.md`; `grammar/README.md`; contrato de enganche mínimo; nota de arquitectura mínima; informe de continuidad del frente básico; repo fresco auditado con suite en verde.
- **Artefactos afectados:** `docs/README.md`; `docs/arquitectura/README.md`; `grammar/README.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
- **Evidencia:** lectura completa del repo fresco con foco reforzado en README, actas y registros; contraste cruzado con la arquitectura vigente; verificación local posterior de `tests/run_conformance.py`, `tests/run_cli_smoke.py` y `tests/run_sec0_smoke.py` sin regresión.
- **Impacto:** trazabilidad; descubribilidad_publica; mantenibilidad; resiliencia_prebackend; gobierno_tecnico.
- **Objeción adversarial considerada:** riesgo doble: por un lado invisibilizar piezas rectoras ya vigentes del ABI mínimo y del núcleo enganchable; por otro lado sobreactuar ese blindaje como si implicara cierre total del frente o apertura automática del backend. No procede ninguno de los dos extremos.
- **Decisión:** alinear la superficie pública mínima para que terceros y futuras unidades lean correctamente el estado real: ABI mínimo visible y arquitectura mínima indexada, pero sin declarar cierre total de FFL-E ni salto automático a backend.
- **Estado:** cerrado

### RETP-2026-034

- **Fecha:** 22/03/2026
- **Hora (Europe/Madrid):** NO_CONSTA
- **Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA
- **Frente/Fase:** Frente final del lenguaje SV / Bloque L — coordinación documental y registral de resiliencia e hitos / Fase IV
- **Resumen del cambio:** Se integra explícitamente el marco de estabilidad, resiliencia y horizontes, junto con los instrumentos de control por hitos, en la cadena de README y en el registro maestro; además se sanea la forma del registro `.md` para que vuelva a ser una fuente de lectura humana coordinada con el CSV.
- **Motivo o argumento:** La lectura íntegra del repo fresco mostró que el núcleo técnico estaba estable, pero todavía persistía una vulnerabilidad de relevo documental: el marco de resiliencia y los CSV de hitos existían, aunque no quedaban suficientemente descubiertos en la cadena principal de lectura, y el registro `.md` había quedado contaminado al final por una línea CSV cruda de `RETP-2026-033`.
- **Base doctrinal o técnica invocada:** `README.md`; `docs/README.md`; `docs/arquitectura/README.md`; `docs/calidad/README.md`; `docs/arquitectura/MARCO_ESTABILIDAD_RESILIENCIA_LENGUAJE_SV.md`; `docs/calidad/MATRIZ_UCBC_HORIZONTES_LENGUAJE_SV.csv`; `docs/calidad/REGISTRO_CALIDAD_HITOS_LENGUAJE_SV.csv`; `docs/calidad/DEUDA_VIVA_HITOS_LENGUAJE_SV.csv`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; repo fresco auditado con suite en verde.
- **Artefactos afectados:** `README.md`; `docs/README.md`; `docs/arquitectura/README.md`; `docs/calidad/README.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
- **Evidencia:** lectura completa del árbol con foco reforzado en README, actas y registros; contraste interno entre arquitectura, calidad y control por hitos; verificación local posterior de `tests/run_conformance.py`, `tests/run_cli_smoke.py` y `tests/run_sec0_smoke.py` sin regresión.
- **Impacto:** trazabilidad; descubribilidad_publica; continuidad_operativa; resiliencia_prebackend; gobierno_tecnico.
- **Objeción adversarial considerada:** riesgo de sobreactuar el marco de resiliencia y los hitos como si autorizaran una implementación inmediata de semántica futura, o de presentar este saneamiento como cierre de `FFL-E`. No procede: el lote solo coordina la lectura pública y registral del estado vigente, sin tocar IR, gramática, validator, runner ni backend.
- **Decisión:** hacer inevitable para futuras unidades Watson la lectura del marco de resiliencia y del control por hitos dentro de la cadena de README y restablecer la coordinación formal entre el registro `.md` y el CSV maestro.
- **Estado:** cerrado


### RETP-2026-035

- **Fecha:** 22/03/2026
- **Hora (Europe/Madrid):** NO_CONSTA
- **Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA
- **Frente/Fase:** Frente final del lenguaje SV / Bloque M — alineación de `docs/index.html` con el estado pre-backend / Fase IV
- **Resumen del cambio:** Se alinea `docs/index.html` con el estado real pre-backend del lenguaje: la puerta pública del playground hace ya visibles la arquitectura mínima del núcleo enganchable, el contrato de enganche y ABI semántico-diagnóstico, el marco de estabilidad y resiliencia, y el bloque activo de calidad, vigilancia e hitos.
- **Motivo o argumento:** La auditoría del repo fresco confirmó que la cadena principal de README había quedado coordinada tras el Bloque L, pero persistía una asimetría residual en la superficie pública de entrada: `docs/index.html` seguía enlazando solo a repositorio, gramática, IR y doctrina, y no dejaba visible el nuevo marco rector pre-backend para lectores o futuras unidades que entrasen por esa puerta.
- **Base doctrinal o técnica invocada:** `docs/index.html`; `README.md`; `docs/README.md`; `docs/arquitectura/README.md`; `docs/calidad/README.md`; `docs/arquitectura/NOTA_DE_ARQUITECTURA_MINIMA_DEL_NUCLEO_ENGANCHABLE_DEL_LENGUAJE_SV.md`; `docs/arquitectura/CONTRATO_DE_ENGANCHE_DE_INTERFACES_FUTURAS_Y_ABI_SEMANTICO_DIAGNOSTICO_MINIMO.md`; `docs/arquitectura/MARCO_ESTABILIDAD_RESILIENCIA_LENGUAJE_SV.md`; repo fresco auditado con suite en verde.
- **Artefactos afectados:** `docs/index.html`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
- **Evidencia:** lectura íntegra del repo fresco con foco reforzado en README, actas, registros y puerta pública operativa; contraste de descubribilidad entre `docs/index.html`, arquitectura y calidad; verificación local posterior de `tests/run_conformance.py`, `tests/run_cli_smoke.py` y `tests/run_sec0_smoke.py` sin regresión.
- **Impacto:** trazabilidad; descubribilidad_publica; continuidad_operativa; resiliencia_prebackend; gobierno_tecnico.
- **Objeción adversarial considerada:** riesgo doble: por un lado considerar `docs/index.html` como pieza secundaria y dejarla desalineada respecto del estado real del lenguaje; por otro lado sobreactuar la nueva visibilidad como si implicara cierre total de `FFL-E` o apertura automática del backend. No procede ninguno de los dos extremos.
- **Decisión:** cerrar la asimetría residual de la puerta pública del playground para que cualquier unidad o lector externo encuentre desde `docs/index.html` el núcleo documental mínimo pre-backend, sin modificar gramática, IR, validator, runner ni el estatuto prudente del frente.
- **Estado:** cerrado

### RETP-2026-036

- **Fecha:** 23/03/2026
- **Hora (Europe/Madrid):** NO_CONSTA
- **Tipo de hito:** CAMBIO_GOBIERNO_TECNICO_Y_CONTROL_DE_HITOS
- **Frente/Fase:** Frente H1 — formalización auditada del primer hito del Lenguaje SV / Fase IV
- **Resumen del cambio:** Se formaliza en un solo lote la verificación auditada de `H1 — Base segura`, se actualiza el registro de calidad de hitos y se propaga su lectura mínima a la cadena de README y a `docs/index.html`, dejando explícito que H1 no cierra el frente final ni abre automáticamente `H2`, `H3` o backend.
- **Motivo o argumento:** La auditoría seria del primer hito mostró que ya existen materialmente gramática mínima estable, IR no incompatible con relaciones y acumulaciones futuras y validator no temporalista; faltaba consolidar ese estado como acto de gobierno técnico legible, resistente al relevo y menos dependiente de microparches dispersos.
- **Base doctrinal o técnica invocada:** `docs/arquitectura/MARCO_ESTABILIDAD_RESILIENCIA_LENGUAJE_SV.md`; `docs/arquitectura/CONTRATO_DE_ENGANCHE_DE_INTERFACES_FUTURAS_Y_ABI_SEMANTICO_DIAGNOSTICO_MINIMO.md`; `docs/arquitectura/NOTA_DE_ARQUITECTURA_MINIMA_DEL_NUCLEO_ENGANCHABLE_DEL_LENGUAJE_SV.md`; `docs/arquitectura/INFORME_DE_CONTINUIDAD_DEL_FRENTE_BASICO_Y_HABILITACION_DEL_BACKEND_SV.md`; `docs/calidad/REGISTRO_CALIDAD_HITOS_LENGUAJE_SV.csv`; `docs/calidad/DEUDA_VIVA_HITOS_LENGUAJE_SV.csv`; `docs/calidad/MATRIZ_UCBC_HORIZONTES_LENGUAJE_SV.csv`; repo fresco auditado con suite en verde.
- **Artefactos afectados:** `README.md`; `docs/README.md`; `docs/arquitectura/README.md`; `docs/calidad/README.md`; `docs/index.html`; `docs/calidad/ACTA_TECNICA_DE_VERIFICACION_DEL_HITO_1_BASE_SEGURA_2026_03_23.md`; `docs/calidad/REGISTRO_CALIDAD_HITOS_LENGUAJE_SV.csv`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
- **Evidencia:** lectura completa del repo fresco con foco en README, arquitectura, calidad, hitos y parser/validator; contraste adversarial de que la persistencia de deuda viva residual no impide por sí sola H1; verificación local posterior de `tests/run_conformance.py`, `tests/run_cli_smoke.py` y `tests/run_sec0_smoke.py` sin regresión.
- **Impacto:** gobierno_tecnico; continuidad_operativa; descubribilidad_publica; resiliencia_prebackend; control_de_hitos.
- **Objeción adversarial considerada:** riesgo de sobreatribuir la verificación de `H1` como si implicara cierre total del frente o apertura automática de hitos posteriores/backend, y riesgo simétrico de negar `H1` por la sola persistencia de deuda residual. No procede ninguno de los dos extremos.
- **Decisión:** declarar auditado `H1 — Base segura` como hito materialmente verificado, propagar esa lectura a la superficie pública mínima y mantener intacta la prudencia sobre `H2`, `H3`, `FFL-A` a `FFL-E` y backend soberano.
- **Estado:** cerrado

### RETP-2026-037

- **Fecha:** 23/03/2026
- **Hora (Europe/Madrid):** 16:59:50
- **Tipo de hito:** CAMBIO_GOBIERNO_TECNICO_Y_CONTROL_DE_CALIDAD
- **Frente/Fase:** Frente H2-pre y control adicional de lectura fresca / Fase IV
- **Resumen del cambio:** Se formaliza `H2-pre` como dictamen auditado de no bloqueo, se institucionaliza el barrido secuencial de actividad y latencia del repo y se crea un sistema de partes de trabajo por agente con taxonomía explícita de lectura del repo fresco.
- **Motivo o argumento:** La evaluación adversarial mostró que la nueva capa de seguridad es útil para reducir memoria espuria, inferencia no declarada y lectura parcial, siempre que no se apoye en fechas brutas del ZIP y que no infle innecesariamente el registro maestro. También se consideró necesario dejar asentado `H2-pre` sin sobreactuarlo como `H2` logrado.
- **Base doctrinal o técnica invocada:** `docs/arquitectura/MARCO_ESTABILIDAD_RESILIENCIA_LENGUAJE_SV.md`; `docs/calidad/REGISTRO_CALIDAD_HITOS_LENGUAJE_SV.csv`; `docs/calidad/DEUDA_VIVA_HITOS_LENGUAJE_SV.csv`; `docs/calidad/MATRIZ_UCBC_HORIZONTES_LENGUAJE_SV.csv`; repo fresco auditado con suite en verde.
- **Artefactos afectados:** `docs/README.md`; `docs/calidad/README.md`; `docs/calidad/ACTA_TECNICA_DE_MICROAUDITORIA_H2_PRE_NO_BLOQUEO_2026_03_23.md`; `docs/calidad/ACTA_DE_ACTIVACION_DEL_CONTROL_DE_BARRIDOS_Y_PARTES_POR_AGENTE_2026_03_23.md`; `docs/calidad/REGISTRO_CALIDAD_HITOS_LENGUAJE_SV.csv`; `docs/calidad/REGISTRO_BARRIDOS_DE_ACTIVIDAD_Y_LATENCIA_DEL_REPO.md`; `docs/calidad/REGISTRO_BARRIDOS_DE_ACTIVIDAD_Y_LATENCIA_DEL_REPO.csv`; `docs/calidad/REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE.md`; `docs/calidad/REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE.csv`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
- **Evidencia:** lectura completa del repo fresco con foco en hitos, deuda viva, marco de resiliencia, cadena de README y registros de calidad; adversarial sobre utilidad y coste del control adicional; verificación local de `tests/run_conformance.py`, `tests/run_cli_smoke.py` y `tests/run_sec0_smoke.py` sin regresión.
- **Impacto:** gobierno_tecnico; trazabilidad_de_agente; control_de_calidad; resiliencia_operativa; continuidad_segura.
- **Objeción adversarial considerada:** riesgo de convertir el nuevo control en burocracia anárquica, o de basarlo en fechas del ZIP como si fueran prueba suficiente de actividad real. No procede ninguno de los dos extremos: se adopta un control cíclico, sobrio y orientado a actividad, latencia y justificabilidad, no a cronología bruta del ZIP.
- **Decisión:** formalizar `H2-pre` como no bloqueo auditado, mantener `H2` en `Pendiente`, crear registros específicos de barridos y partes por agente, y conservar el registro maestro como capa de síntesis, no como bitácora exhaustiva.
- **Estado:** cerrado

### RETP-2026-038

- **Fecha:** 23/03/2026
- **Hora (Europe/Madrid):** 17:45:05
- **Tipo de hito:** CAMBIO_GOBIERNO_TECNICO_Y_CONTROL_DE_HITOS
- **Frente/Fase:** Frente H2-A — autorización restringida de apertura de H2 / Fase IV
- **Resumen del cambio:** Se formaliza la autorización restringida de apertura de `H2` como frente arquitectónico-auditor de trabajo, se actualiza el registro de hitos sin verificar todavía `H2`, y se deja trazabilidad adicional del paso mediante barrido secuencial y parte por agente.
- **Motivo o argumento:** Tras `H1` verificado y `H2-pre` formalizado como no bloqueo, la adversarial mostró que ya existe base positiva suficiente para abrir `H2` en régimen restringido. Faltaba convertir esa situación material en un acto explícito de gobierno técnico, sin confundirla con `H2` logrado ni con apertura automática del backend.
- **Base doctrinal o técnica invocada:** `docs/arquitectura/MARCO_ESTABILIDAD_RESILIENCIA_LENGUAJE_SV.md`; `docs/arquitectura/CONTRATO_DE_ENGANCHE_DE_INTERFACES_FUTURAS_Y_ABI_SEMANTICO_DIAGNOSTICO_MINIMO.md`; `docs/arquitectura/NOTA_DE_ARQUITECTURA_MINIMA_DEL_NUCLEO_ENGANCHABLE_DEL_LENGUAJE_SV.md`; `docs/calidad/ACTA_TECNICA_DE_VERIFICACION_DEL_HITO_1_BASE_SEGURA_2026_03_23.md`; `docs/calidad/ACTA_TECNICA_DE_MICROAUDITORIA_H2_PRE_NO_BLOQUEO_2026_03_23.md`; `docs/calidad/REGISTRO_CALIDAD_HITOS_LENGUAJE_SV.csv`; repo fresco auditado con suite en verde.
- **Artefactos afectados:** `README.md`; `docs/README.md`; `docs/arquitectura/README.md`; `docs/calidad/README.md`; `docs/index.html`; `docs/calidad/ACTA_TECNICA_DE_AUTORIZACION_RESTRINGIDA_DE_APERTURA_DE_H2_2026_03_23.md`; `docs/calidad/REGISTRO_CALIDAD_HITOS_LENGUAJE_SV.csv`; `docs/calidad/REGISTRO_BARRIDOS_DE_ACTIVIDAD_Y_LATENCIA_DEL_REPO.csv`; `docs/calidad/REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE.csv`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
- **Evidencia:** lectura completa del repo fresco con foco en arquitectura, README, calidad, hitos, barridos y partes por agente; adversarial específica sobre la autorización de `H2`; verificación local de `tests/run_conformance.py`, `tests/run_cli_smoke.py` y `tests/run_sec0_smoke.py` sin regresión.
- **Impacto:** gobierno_tecnico; control_de_hitos; continuidad_operativa; trazabilidad_de_agente; resiliencia_prebackend.
- **Objeción adversarial considerada:** riesgo de sobreactuar `H2-pre` como si ya implicara `H2` verificado, o de negar toda apertura de `H2` por la sola permanencia de `Q-04`, `Q-05`, `D-01` y `D-02` en estado pendiente o de deuda viva. No procede ninguno de los dos extremos: la decisión correcta es una autorización restringida y explícitamente limitada.
- **Decisión:** autorizar la apertura restringida de `H2` como frente arquitectónico-auditor de trabajo, mantener `Q-04` y `Q-05` en `Pendiente`, y dejar asentado que esta autorización no abre automáticamente `H3` ni el backend soberano.
- **Estado:** cerrado

### RETP-2026-040

- **Fecha:** 23/03/2026
- **Hora (Europe/Madrid):** 20:02:35
- **Tipo de hito:** CAMBIO_RUNNER
- **Frente/Fase:** Frente final del lenguaje SV / Parche correctivo funcional acotado del frontend / Fase IV
- **Resumen del cambio:** Se cierra y registra el lote correctivo del frontend que recupera especificidad diagnóstica visible, impone no-vaciedad en `compose`, elimina el falso positivo por orden textual en `AdmissibilitySpec` y corrige el diagnóstico de `PendingU`.
- **Motivo o argumento:** El parche ya quedó aplicado y auditado sobre el repo fresco real, pero faltaba completar el paso de registro y cierre exigido por el circuito `ZIP → GITHUB → VERIFICACIÓN → REGISTRO → CIERRE`.
- **Base doctrinal o técnica invocada:** pliego de condiciones del Sistema Vectorial SV; procedimiento de auditoría técnica; mandato operativo de no mezclar este lote con regularización amplia del catálogo; repo fresco auditado tras verificación de cierre.
- **Artefactos afectados:** `src/svp_errors.py`; `src/svp_parser.py`; `src/svp_validator.py`; `tests/run_conformance.py`; `tests/conformance/invalid/output_semantics_no_declarada.svp`; `tests/conformance/invalid/compose_relations_vacias.svp`; `tests/conformance/invalid/compose_patterns_vacios.svp`; `tests/conformance/invalid/pending_u_reconocido_no_habilitado.svp`; `tests/conformance/invalid/transition_data_horizon_no_declarado.svp`; `tests/conformance/valid/admissibility_spec_states_permutados.svp`; `tests/conformance/valid/admissibility_spec_states_permutados.expected.json`; `docs/calidad/ACTA_TECNICA_DE_CIERRE_AUDITADO_DEL_PARCHE_CORRECTIVO_DEL_FRONTEND_2026_03_23.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`; `docs/calidad/REGISTRO_BARRIDOS_DE_ACTIVIDAD_Y_LATENCIA_DEL_REPO.csv`; `docs/calidad/REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE.csv`; `docs/calidad/README.md`.
- **Evidencia:** verificación posterior del repo fresco con confirmación material de los archivos tocados y ejecución local de `tests/run_conformance.py` **37/37**, `tests/run_cli_smoke.py` **3/3** y `tests/run_sec0_smoke.py` **3/3** sin regresión.
- **Impacto:** coherencia_diagnostica; trazabilidad; cierre_auditado; cobertura_de_suite; gobierno_tecnico.
- **Objeción adversarial considerada:** riesgo de aprovechar el cierre para mezclarlo con regularización amplia de `E106`, `E111` o del catálogo muerto. No procede: el registro consolida solo este lote acotado y su verificación posterior.
- **Decisión:** cerrar registralmente el parche correctivo del frontend como lote terminado y dejar separado cualquier frente posterior de regularización técnica limitada.
- **Estado:** cerrado

### RETP-2026-041

- **Fecha:** 24/03/2026
- **Hora (Europe/Madrid):** 21:42:51
- **Tipo de hito:** CAMBIO_DOCUMENTACION_PUBLICA_Y_CONCORDANCIA_DIAGNOSTICA
- **Frente/Fase:** Frente final del lenguaje SV / Microparche documental rebasado del contrato diagnóstico visible / Fase IV
- **Resumen del cambio:** Se resincroniza la documentación viva del contrato diagnóstico observable para reflejar la emitibilidad y cobertura explícita ya acreditadas de `E102`, `E104`, `E208`, `E209` y `E303`, corrigiendo además los nombres implementativos stale de `E102` y `E104` en catálogo y matriz.
- **Motivo o argumento:** La lectura adversarial del repo fresco mostró que el árbol ya acredita materialmente esos cinco códigos por suite y validator, mientras varios documentos vivos seguían describiendo un estado anterior o arrastraban nombres implementativos ya no vigentes.
- **Base doctrinal o técnica invocada:** `src/svp_errors.py`; `tests/run_conformance.py`; `tests/conformance/invalid/output_semantics_no_declarada.svp`; `tests/conformance/invalid/conector_target_no_ternario.svp`; `tests/conformance/invalid/compose_relations_vacias.svp`; `tests/conformance/invalid/compose_patterns_vacios.svp`; `tests/conformance/invalid/transition_data_horizon_no_declarado.svp`; repo fresco auditado con suite en verde.
- **Artefactos afectados:** `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`; `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`; `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`; `docs/calidad/DICTAMEN_DE_SANEAMIENTO_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO.md`; `docs/calidad/ACTA_TECNICA_DE_SINCRONIZACION_DOCUMENTAL_DEL_CONTRATO_DIAGNOSTICO_VISIBLE_2026_03_24.md`; `docs/calidad/README.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
- **Evidencia:** lectura `100_REAL` del repo fresco con contraste corto sobre `src/svp_errors.py`, `tests/run_conformance.py`, los casos inválidos afectados y la documentación viva; ejecución local posterior de `tests/run_conformance.py` **37/37**, `tests/run_cli_smoke.py` **3/3** y `tests/run_sec0_smoke.py` **3/3** sin regresión.
- **Impacto:** coherencia_diagnostica; sincronizacion_documental; trazabilidad; control_de_calidad.
- **Objeción adversarial considerada:** riesgo de sobreactuar este lote como si implicara convergencia semántica plena con la IR v0.2 o reescritura silenciosa de historia cerrada. No procede: el lote se limita a resincronizar documentación viva y a añadir nota posterior explícita en el dictamen.
- **Decisión:** abrir y cerrar como lote separado un microparche documental rebasado que sincronice el contrato diagnóstico visible sin tocar código ni mezclar otros frentes.
- **Estado:** cerrado

### RETP-2026-042

- **Fecha:** 26/03/2026
- **Hora (Europe/Madrid):** 09:40:00
- **Tipo de hito:** DECISION_GOBIERNO_TECNICO
- **Frente/Fase:** Lenguaje SV / sellado técnico mínimo prebackend y reordenación de continuidad tras la familia VII / Fase IV
- **Resumen del cambio:** Se crea el bloque mínimo de gobierno para cerrar provisionalmente `PC-HNA`, registrar nueva deuda viva semántica y complementar el rumbo prebackend, fijando además que el siguiente frente correcto será la pieza marco de células especializadas y no `NLP`.
- **Motivo o argumento:** La doble auditoría externa y la consolidación de la familia VII muestran que el riesgo ya no está en el bloque congelado previamente saneado, sino en avanzar hacia frentes de alta presión semántica sin sellar antes el fundamento mínimo del lenguaje ni corregir expresamente el orden de continuidad.
- **Base doctrinal o técnica invocada:** repos frescos de `SV-lenguaje-de-computacion` y `SV-matematica-semantica`; documentación de calidad vigente; doble auditoría externa absorbida por la coordinación técnica; familia VII consolidada hasta su pieza más reciente.
- **Artefactos afectados:** `docs/calidad/ACTA_TECNICA_DE_CIERRE_PROVISIONAL_DE_PC_HNA_Y_CONDICIONES_DE_REAPERTURA_2026_03_26.md`; `docs/calidad/ACTA_TECNICA_COMPLEMENTARIA_DE_CONTINUIDAD_TRAS_LA_FAMILIA_VII_Y_REORDENACION_DEL_RUMBO_PREBACKEND_2026_03_26.md`; `docs/calidad/DEUDA_VIVA_HITOS_LENGUAJE_SV.csv`; `docs/calidad/MATRIZ_UCBC_HORIZONTES_LENGUAJE_SV.csv`; `docs/calidad/ESPEJO_DOCTRINAL_COLECCIONES_LENGUAJE_SV.md`; `docs/calidad/README.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`; `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`; `docs/calidad/REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE.csv`.
- **Evidencia:** lectura `100_REAL` de repos frescos; triple contraste con auditorías externas; verificación de coherencia interna entre actas, deuda viva, matriz de horizontes, espejo doctrinal y `README` de calidad.
- **Impacto:** gobierno_tecnico; trazabilidad; continuidad_operativa; resiliencia_prebackend; cautela_semantica.
- **Objeción adversarial considerada:** riesgo de retrasar innecesariamente el proyecto con teoría total prematura, o de reordenar la continuidad por intuición sin dejar rastro explícito. No procede ninguno de los dos extremos: se fija un sellado técnico mínimo y una corrección de secuencia limitada, documentada y no implementativa.
- **Decisión:** cerrar provisionalmente `PC-HNA`, abrir deuda viva semántica mínima y complementar el rumbo prebackend para que la siguiente pieza lógica sea la publicación marco de células especializadas.
- **Estado:** cerrado
