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
