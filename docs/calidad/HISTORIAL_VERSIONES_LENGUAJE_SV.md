# Historial de versiones del Lenguaje SV

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Última actualización:** 23 de agosto de 2026

## 1. Objeto

Este registro ofrece una lectura cronológica y verificable de la evolución técnica del Lenguaje SV. Su finalidad es permitir que cualquier lector identifique, sin reconstruir manualmente el historial de confirmaciones de Git, qué versiones fueron incorporadas al repositorio, cuándo comenzaron a regir, qué modificación material introdujeron y qué evidencia pública permite comprobarlo.

El registro complementa, pero no sustituye, el historial de Git, los documentos de especificación, las pruebas de conformidad ni los registros de calidad detallados.

## 2. Criterios de registro

Una entrada se incorpora al historial principal cuando existe una modificación material integrada en `main` que afecta a una versión de la gramática, de la representación intermedia, de la serialización, de la implementación de referencia o del entorno público de comprobación.

Se distinguen tres situaciones:

- **Vigente:** versión integrada en `main` que constituye la referencia técnica actual dentro de su alcance.
- **Histórica:** versión anteriormente vigente que permanece conservada para trazabilidad y comparación.
- **En desarrollo:** modificación disponible en una rama o solicitud de incorporación, pero todavía no integrada en `main`.

Las correcciones que no justifican un cambio de número de versión se registran separadamente. Ninguna entrada histórica se elimina por haber sido sustituida. Una corrección factual del presente documento debe dejar constancia de la razón de la rectificación.

## 3. Versiones integradas

| Fecha | Situación actual | Gramática | IR | Serializador | Realización o entorno | Modificación material | Evidencia pública |
|---|---|---:|---:|---:|---|---|---|
| 12/03/2026 | Histórica | — | 0.2 | — | Especificación | Primera IR canónica v0.2 y sistema de bienformación. | [Confirmación `233b8ab`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/233b8ab71e542864fd87ab7580e581ca6a7e34b6) · [IR v0.2](../../IR_CANONICA_BIENFORMACION_SV_v0_2.md) · [Catálogo efectivo v0.2](../referencia/ERRORES_CANONICOS_SV_v0_2.md) |
| 12/03/2026 | Histórica | 0.1 | 0.2 | 0.1.0 | Especificación e implementación de referencia | Primera gramática superficial mínima v0.1, subordinada a la IR v0.2. La implementación de referencia emitía ya la versión 0.1.0 del serializador. | [Confirmación `3503ab6`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/3503ab6074217c1c411a106382c29c9392b313cc) · [Gramática v0.1](../../GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_1.md) · [IRProgram inicial](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/9ae1b40ac917ea21533cbdb5c74c645027efcc96/src/svp_ir.py) |
| 12/03/2026 | Histórica | 0.1 | 0.2 | 0.1.0 | Implementación de referencia en Python | Incorporación del analizador sintáctico, validación, descenso a IR y JSON canónico; batería inicial de 10 casos. | [Confirmación `9ae1b40`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/9ae1b40ac917ea21533cbdb5c74c645027efcc96) |
| 12/03/2026 | Histórica | 0.1 | 0.2 | 0.1.0 | SVP Playground | Primera publicación del entorno de comprobación en navegador. | [Confirmación `011902b`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/011902b4c49e2d9b34cdda9ea4df31b8438bf37e) |
| 23/08/2026 | **Vigente** | **0.2** | **0.3** | **0.1.0** | Etapa frontal de referencia en Python | Integración de C01–C03: separación entre admisibilidad técnica y `Tri.U`; resolución identificada de una `U` constituida; cierre estructural y causal de `Frame`. Batería de conformidad: 72/72. | [PR #7](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/7) · [integración `59a022a`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/59a022a7691d28c6afcf18456ad1ef0aae562362) · [Conformidad SVP #11](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/32652744484) · [Gramática v0.2](../../GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_2.md) · [IR v0.3](../../IR_CANONICA_BIENFORMACION_SV_v0_3.md) · [Catálogo efectivo v0.3](../referencia/ERRORES_CANONICOS_SV_v0_3.md) |
| 23/08/2026 | **Vigente** | **0.2** | **0.3** | **0.1.0** | SVP Playground mediante Python/Pyodide | Alineación del entorno público con las versiones ya integradas y lectura de las versiones efectivas desde los metadatos de la IR. | [PR #8](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/8) · [integración `af1491b`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/af1491b53f070a75989cbdd90f76dfeadfa3e9ab) · [Playground](https://juantoniolloretegea.github.io/SV-lenguaje-de-computacion/) |

El guion largo (`—`) indica que el elemento correspondiente no disponía en esa entrada de un número de versión independiente que este registro pueda acreditar con la misma precisión. No debe interpretarse como ausencia del componente técnico.

## 4. Diferencias materiales entre Gramática 0.1 / IR 0.2 y Gramática 0.2 / IR 0.3

La tabla siguiente resume únicamente las diferencias que determinan el salto de versión. No sustituye a las especificaciones completas.

| Aspecto | Gramática 0.1 / IR 0.2 | Gramática 0.2 / IR 0.3 |
|---|---|---|
| `AdmissibilitySpec` | Admitía en superficie `{Ok, Degraded, Failed, U}`. | Usa exactamente `{Ok, Degraded, NotAdmitted}` y separa la admisibilidad técnica de `Tri.U`. |
| `resolve` | Operaba sobre el literal abstracto `U`. | Usa `resolve((estado, posición), …)` sobre una ocurrencia constituida e identificable de `U`. |
| `Frame` | No incorporaba las restricciones relacionales introducidas por C03. | Exige cierre estructural y causal entre los estados, evaluaciones, compuertas y supervisiones incluidos en el mismo `Frame`, sin imponer exhaustividad. |
| Diagnósticos efectivos | Catálogo efectivo v0.2. | El catálogo efectivo v0.3 conserva v0.2 y añade `E110`, `E305` y `E308`. |
| Serializador | `0.1.0`. | `0.1.0`; la revisión no modifica el algoritmo de serialización canónica. |
| Conformidad | Baterías anteriores, ampliadas progresivamente durante el desarrollo. | 72/72 casos en el corte de integración, con oráculos canónicos comprometidos y comprobación de que la ejecución no los modifica. |
| Alcance de la revisión | Configuración anterior a C01–C03. | La revisión se limita a C01–C03; no existe una corrección C04. |

Referencias diagnósticas: [catálogo efectivo v0.2](../referencia/ERRORES_CANONICOS_SV_v0_2.md) · [catálogo efectivo v0.3](../referencia/ERRORES_CANONICOS_SV_v0_3.md).

No cambian por esta revisión `Tri = {Zero, One, U}`, la ausencia de tiempo, reloj o UTC como primitivas del Lenguaje, la versión `0.1.0` del serializador ni las deudas relativas a `ConflictOperator`/J2.3, la divergencia histórica de `E204` y `RG1`.

## 5. Correcciones relevantes sin cambio de versión

Una versión puede recibir correcciones que preserven su número cuando no se modifica el contrato versionado en su conjunto. Estas correcciones se registran porque forman parte de la historia técnica y pueden explicar diferencias entre dos revisiones de una misma versión.

| Fecha | Versión afectada | Corrección | Consecuencia | Evidencia pública |
|---|---|---|---|---|
| 12/03/2026 | Gramática 0.1 / IR 0.2 | Correcciones sucesivas del Playground hasta disponer de una API estable, errores tipados y ejecución operativa del análisis sintáctico y de la limpieza del editor. | El entorno público quedó utilizable sin alterar los números de Gramática e IR. | [Confirmación `9136270`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/9136270a082b7fb42496c8c0f91f8d74453e751a) |
| 19/08/2026 | Gramática 0.1 / IR 0.2 | Retirada de `conflicts` de `graph_decl`; el régimen `General` conservó la deuda de `ConflictOperator`. | Se corrigió una capacidad superficial que no disponía de realización semántica suficiente. | [Confirmación `058befb`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/058befbd6402c80ac7bc1d10eab0d8d035126531) |
| 23/08/2026 | Playground 0.2 / 0.3 | Tras integrar Gramática 0.2 e IR 0.3, la página pública conservó temporalmente rótulos de Gramática 0.1 e IR 0.2, aunque cargaba desde `main` la implementación actualizada. | La interfaz fue corregida para mostrar las versiones vigentes y obtener la identificación efectiva desde la propia IR. | [PR #8](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/8) · [integración `af1491b`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/af1491b53f070a75989cbdd90f76dfeadfa3e9ab) |

## 6. Desarrollo no integrado en `main`

Las entradas de esta sección son informativas. No forman parte de la versión vigente mientras no exista una integración expresa en `main`.

| Fecha de apertura | Situación | Gramática de referencia | IR de referencia | Desarrollo | Comprobación disponible | Referencia |
|---|---|---:|---:|---|---|---|
| 23/08/2026 | **En desarrollo** | 0.2 | 0.3 | Inicio de R0 en Rust: `sv_core` compartido por ejecución nativa y WebAssembly; `Tri = {Zero, One, U}` y adaptador `sv_wasm` sin semántica independiente. | Pruebas Rust nativas correctas; compilación de `sv_core` y `sv_wasm` para `wasm32-unknown-unknown`; conformidad Python preservada. | [PR #9](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/9) · [confirmación `ad04445`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/ad04445cee142a0fff3082c7453d923af7d584ad) · [R0 Rust #1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/32654720941) · [Conformidad SVP #13](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/32654720920) |

Cuando un desarrollo de esta sección se integre, deberá añadirse una nueva entrada a la tabla de versiones integradas con la fecha y la confirmación de integración. La entrada de desarrollo podrá conservarse como antecedente o sustituirse por una referencia a la nueva entrada, siempre sin atribuir vigencia retrospectiva.

## 7. Relación con la calidad y la trazabilidad

Este historial responde a preguntas distintas de las que resuelve el historial de Git:

- qué versión estaba vigente en una fecha determinada;
- qué documentos definían esa versión;
- qué implementación o entorno público la materializaba;
- qué comprobación acompañó su incorporación;
- qué correcciones relevantes se efectuaron sin cambiar el número de versión;
- qué desarrollos posteriores permanecen todavía sin integrar.

La existencia de una entrada en este historial no sustituye la comprobación técnica de los documentos y artefactos enlazados. En caso de discrepancia, la evidencia material del repositorio y la documentación técnica vigente determinan el alcance realmente acreditado.

## 8. Regla de continuidad

Toda nueva versión de Gramática, IR, serialización, realización soberana o entorno público que modifique de forma observable la referencia vigente deberá añadir una entrada a este historial.

Las versiones anteriores permanecerán accesibles como antecedentes. Los errores y correcciones relevantes deberán registrarse cuando sean necesarios para comprender la evolución técnica, sin ocultarlos ni convertirlos por sí solos en una nueva versión.