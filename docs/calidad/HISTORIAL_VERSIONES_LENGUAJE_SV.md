# Historial de versiones del Lenguaje SV

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Última actualización:** 24 de agosto de 2026

## 1. Objeto

Este registro ofrece una lectura cronológica y verificable de la evolución técnica del Lenguaje SV. Permite identificar qué versiones fueron incorporadas al repositorio, qué realización o entorno público las materializó, qué modificación material introdujeron y qué evidencia permite comprobarlo.

El registro complementa, pero no sustituye, el historial de Git, las especificaciones, las pruebas de conformidad ni los registros de calidad detallados.

## 2. Criterios de registro

Se distinguen cuatro situaciones:

- **Vigente:** versión integrada en `main` que constituye la referencia técnica actual dentro de su alcance.
- **Histórica:** versión o entorno anteriormente vigente que se conserva para trazabilidad y comparación.
- **En desarrollo:** modificación disponible en una rama o solicitud de incorporación, pero todavía no integrada en `main`.
- **Desplegada, no integrada:** realización material accesible públicamente cuya línea de código o cierre de integración todavía no forma parte de `main`.

La situación de un entorno público y la integración de su código se registran por separado cuando no coinciden temporalmente. Un despliegue no convierte por sí mismo una rama en referencia de `main`, y una pieza histórica no se elimina por haber sido sustituida.

Las correcciones que no justifican un cambio de número de versión se registran separadamente. Una corrección factual del presente documento debe dejar constancia de la razón de la rectificación.

## 3. Versiones integradas

| Fecha | Situación actual | Gramática | IR | Serializador | Realización o entorno | Modificación material | Evidencia pública |
|---|---|---:|---:|---:|---|---|---|
| 12/03/2026 | Histórica | — | 0.2 | — | Especificación | Primera IR canónica v0.2 y sistema de bienformación. | [Confirmación `233b8ab`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/233b8ab71e542864fd87ab7580e581ca6a7e34b6) · [IR v0.2](../../IR_CANONICA_BIENFORMACION_SV_v0_2.md) · [Catálogo efectivo v0.2](../referencia/ERRORES_CANONICOS_SV_v0_2.md) |
| 12/03/2026 | Histórica | 0.1 | 0.2 | 0.1.0 | Especificación e implementación de referencia | Primera gramática superficial mínima v0.1, subordinada a la IR v0.2. La implementación de referencia emitía ya la versión 0.1.0 del serializador. | [Confirmación `3503ab6`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/3503ab6074217c1c411a106382c29c9392b313cc) · [Gramática v0.1](../../GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_1.md) |
| 12/03/2026 | Histórica | 0.1 | 0.2 | 0.1.0 | Implementación de referencia en Python | Incorporación del analizador sintáctico, validación, descenso a IR y JSON canónico; batería inicial de 10 casos. | [Confirmación `9ae1b40`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/9ae1b40ac917ea21533cbdb5c74c645027efcc96) |
| 12/03/2026 | Histórica | 0.1 | 0.2 | 0.1.0 | SVP Playground | Primera publicación del entorno de comprobación en navegador. | [Confirmación `011902b`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/011902b4c49e2d9b34cdda9ea4df31b8438bf37e) |
| 23/08/2026 | **Vigente** | **0.2** | **0.3** | **0.1.0** | Etapa frontal de referencia en Python | Integración de C01–C03: separación entre admisibilidad técnica y `Tri.U`; resolución identificada de una `U` constituida; coherencia estructural y causal de `Frame`. Batería de conformidad: 72/72. | [PR #7](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/7) · [integración `59a022a`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/59a022a7691d28c6afcf18456ad1ef0aae562362) · [Gramática v0.2](../../GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_2.md) · [IR v0.3](../../IR_CANONICA_BIENFORMACION_SV_v0_3.md) |
| 23/08/2026 | **Histórica como acceso público** | **0.2** | **0.3** | **0.1.0** | SVP Playground mediante Python/Pyodide | Alineación del entorno público con Gramática 0.2 e IR 0.3. El 24/08/2026 dejó de ser el punto de acceso público principal al materializarse el entorno Rust/WebAssembly; la implementación Python permanece como referencia diferencial. | [PR #8](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/8) · [integración `af1491b`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/af1491b53f070a75989cbdd90f76dfeadfa3e9ab) · [instantánea histórica](../historico/PLAYGROUND_PYTHON_PYODIDE_2026_08_24.md) |
| 24/08/2026 | **Vigente** | **0.2** | **0.3** | **Proyección diferencial 0.1.0** | **Rust nativo y WebAssembly de navegador sobre `sv_core` compartido** | Integración de la realización Rust/WebAssembly y de la comprobación de paridad sobre el corpus comprometido. La admisión/rechazo coincide en Python, Rust nativo y WebAssembly; el observable textual de proyección es idéntico entre Rust nativo y WebAssembly. No se acredita identidad textual Python↔`equivalence_json` ni paridad diagnóstica exacta `E***`. | [PR #22](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/22) · [integración `befc666`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/befc666fabe54ecd541416610bf31ddfe776aa69) · [R0 WASM paridad de tres vías #11](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/32742397555) · [entorno público](https://lenguaje-sv.itvia.online/) |

El guion largo (`—`) indica que el elemento correspondiente no disponía en esa entrada de un número de versión independiente que este registro pueda acreditar con la misma precisión.

## 4. Diferencias materiales entre Gramática 0.1 / IR 0.2 y Gramática 0.2 / IR 0.3

| Aspecto | Gramática 0.1 / IR 0.2 | Gramática 0.2 / IR 0.3 |
|---|---|---|
| `AdmissibilitySpec` | Admitía en superficie `{Ok, Degraded, Failed, U}`. | Usa exactamente `{Ok, Degraded, NotAdmitted}` y separa la admisibilidad técnica de `Tri.U`. |
| `resolve` | Operaba sobre el literal abstracto `U`. | Usa `resolve((estado, posición), …)` sobre una ocurrencia constituida e identificable de `U`. |
| `Frame` | No incorporaba las restricciones relacionales introducidas por C03. | Exige coherencia estructural y causal entre los estados, evaluaciones, compuertas y supervisiones incluidos en el mismo `Frame`, sin imponer exhaustividad. |
| Diagnósticos efectivos | Catálogo efectivo v0.2. | El catálogo efectivo v0.3 conserva v0.2 y añade `E110`, `E305` y `E308`. |
| Serializador | `0.1.0`. | `0.1.0`; la revisión no modifica el algoritmo de serialización canónica. |
| Conformidad | Baterías anteriores, ampliadas progresivamente. | 72/72 casos, con oráculos canónicos comprometidos y comprobación de que la ejecución no los modifica. |
| Alcance de la revisión | Configuración anterior a C01–C03. | La revisión se limita a C01–C03; no existe una corrección C04. |

No cambian por esta revisión `Tri = {Zero, One, U}`, la ausencia de tiempo, reloj o UTC como primitivas del Lenguaje, la versión `0.1.0` del serializador ni las deudas relativas a `ConflictOperator`/J2.3, la divergencia histórica de `E204` y `RG1`.

## 5. Correcciones relevantes sin cambio de versión

| Fecha | Versión afectada | Corrección | Consecuencia | Evidencia pública |
|---|---|---|---|---|
| 12/03/2026 | Gramática 0.1 / IR 0.2 | Correcciones sucesivas del Playground hasta disponer de una API estable, errores tipados y ejecución operativa. | El entorno público quedó utilizable sin alterar los números de Gramática e IR. | [Confirmación `9136270`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/9136270a082b7fb42496c8c0f91f8d74453e751a) |
| 19/08/2026 | Gramática 0.1 / IR 0.2 | Retirada de `conflicts` de `graph_decl`; el régimen `General` conservó la deuda de `ConflictOperator`. | Se eliminó una capacidad superficial que no disponía de realización semántica suficiente. | [Confirmación `058befb`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/058befbd6402c80ac7bc1d10eab0d8d035126531) |
| 23/08/2026 | Playground 0.2 / 0.3 | La página pública conservó temporalmente rótulos antiguos aunque cargaba la implementación actualizada. | La interfaz fue corregida para mostrar las versiones vigentes. | [PR #8](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/8) · [integración `af1491b`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/af1491b53f070a75989cbdd90f76dfeadfa3e9ab) |

## 6. Realización Rust/WebAssembly y entorno público

La realización Rust mantiene una sola implementación del núcleo en `sv_core`, compartida por el destino nativo y por WebAssembly. Python conserva el papel de referencia diferencial; no se crea una segunda semántica para el navegador.

La paridad acreditada en el corte integrado se formula así:

```text
mismo texto .svp
+ aceptación/rechazo alineado en Python · Rust nativo · WebAssembly
+ observable textual de proyección idéntico Rust nativo ↔ WebAssembly
≠ identidad textual Python ↔ equivalence_json
```

Identidad del corte integrado y desplegado:

```text
fuente
20a1f95cbf1bdbfb4f16cd39335bd71ca1d1c606

sv_wasm.wasm
bytes   = 337366
SHA-256 = 7b49228624f101dc8d863a2b4d631b7ed8eacb4ee4a29c2459d32f6b63aff5dc
```

El entorno público se sirve en `lenguaje-sv.itvia.online`. La distribución web entrega HTML, CSS, JavaScript de transporte y el módulo WebAssembly; la compilación del texto `.svp` se ejecuta localmente en el navegador mediante `sv_wasm` y `sv_core`.

La URL auxiliar `workers.dev` no forma parte del acceso público normal. El dominio institucional constituye el punto de acceso público principal del entorno desplegado.

La integración de la línea Rust/WebAssembly en `main` no cierra por sí sola R0 ni modifica el estatuto histórico del Playground Python/Pyodide como acceso público anterior.

## 7. Límites de la comprobación WebAssembly

La evidencia disponible acredita el corte y el corpus declarados, no propiedades universales:

- la proyección diferencial 0.1.0 no constituye el serializador canónico completo;
- la paridad exacta de códigos `E***` y mensajes de error no está acreditada;
- no se acredita identidad textual bit a bit entre la salida Python y `equivalence_json`;
- la ejecución en un navegador real no demuestra compatibilidad universal con todos los motores;
- una entrada no admitida o un fallo técnico no se convierten en `Tri.U`;
- el despliegue no prueba por sí solo las Garantías I o II;
- R0 permanece abierto hasta su cierre integral expreso;
- R1–R4 no se consideran iniciados por esta materialización.

## 8. Relación con la calidad y la trazabilidad

Este historial permite responder, entre otras, a estas preguntas:

- qué versión estaba integrada en una fecha determinada;
- qué documentos definían esa versión;
- qué implementación o entorno público la materializaba;
- qué comprobación acompañó su incorporación o despliegue;
- qué correcciones relevantes se efectuaron sin cambiar el número de versión;
- qué desarrollos posteriores permanecen todavía sin integrar.

La existencia de una entrada no sustituye la comprobación técnica de los documentos y artefactos enlazados. En caso de discrepancia, la evidencia material del repositorio y la documentación técnica vigente determinan el alcance acreditado.

## 9. Regla de continuidad

Toda nueva versión de Gramática, IR, serialización, realización o entorno público que modifique de forma observable la referencia vigente deberá añadir una entrada a este historial.

Las versiones anteriores permanecerán accesibles como antecedentes. Los errores y correcciones relevantes deberán registrarse cuando sean necesarios para comprender la evolución técnica, sin ocultarlos ni convertirlos por sí solos en una nueva versión.
