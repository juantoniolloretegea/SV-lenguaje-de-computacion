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
- creación de ZIP operativo;
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
- pasos mecánicos intermedios de subida a GitHub;
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
- **Evidencia:** ZIP operativo del parche; resultados internos verificados; evidencias publicadas de actualización de archivos.
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
- **Base doctrinal o técnica invocada:** repositorio doctrinal `SV-matematica-semantica`; especificación transversal subordinada sobre la U; matriz operativa del Clasificador C0–C5; publicación `Desde la terna (0, 1, U) hasta la nueva frontera (0, 1, U, 0-1)`; protocolo ZIP → GITHUB → VERIFICACIÓN → REGISTRO → CIERRE.
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
- **Base doctrinal o técnica invocada:** jerarquía entre repositorios fijada en `README.md`; regla de subordinación de `beta/`; distinción entre espejo material e identidad funcional; protocolo ZIP → GITHUB → VERIFICACIÓN → REGISTRO → CIERRE.
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
- Evidencia: PDF de confirmación de subida del acta a `beta/`; contraste adversarial del chat y del corpus publicado; decisión estratégica consolidada por la unidad WB-Lenguaje1 SV.
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
