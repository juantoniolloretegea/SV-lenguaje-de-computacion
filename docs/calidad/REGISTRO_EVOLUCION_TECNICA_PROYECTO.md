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

## 13. Regla de estilo

Este registro debe mantenerse con lenguaje formal, sobrio, técnico y revisable por terceros. Las entradas deben describir hechos, fundamento, evidencia, impacto y estado.

## 14. Vigencia

Este registro permanecerá vigente mientras el proyecto requiera trazabilidad técnica ligera y controlada de su evolución, sin perjuicio de futuras versiones más estructuradas si la complejidad del sistema lo exigiera.
