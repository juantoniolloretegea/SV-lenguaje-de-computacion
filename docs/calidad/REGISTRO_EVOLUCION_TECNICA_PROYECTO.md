# Registro de evolución técnica del proyecto

## 1. Finalidad

Este documento ofrece la lectura humana del tramo vigente del historial técnico del Lenguaje SV. El archivo `REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv` mantiene la numeración RETP y la relación completa de asientos.

El detalle comprendido entre `RETP-2026-000` y `RETP-2026-047` permanece preservado en:

`docs/calidad/historico/REGISTRO_EVOLUCION_TECNICA_PROYECTO_HASTA_RETP_2026_047.md`.

La continuidad documental se organiza así:

`histórico 000–047 → registro vigente desde 048 → CSV maestro de numeración`.

## 2. Tabla del tramo vigente

| ID | Fecha | Hora | Tipo | Ámbito / fase | Estado |
|---|---|---|---|---|---|
| RETP-2026-048 | 18/08/2026 | 11:34:13 | REAPERTURA_GOBERNADA | Lenguaje SV / Ruta A / retorno a FFL-A | cerrado |
| RETP-2026-049 | 18/08/2026 | 11:58:10 | CIERRE_BLOQUE_Y_APERTURA_SECUENCIAL | Lenguaje SV / FFL-A → FFL-B | cerrado |
| RETP-2026-050 | 18/08/2026 | 12:28:08 | DECISION_GOBIERNO_TECNICO | Lenguaje SV / FFL-B / materialización diagnóstica subordinada | cerrado |
| RETP-2026-051 | 18/08/2026 | 13:05:54 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J2.2 parcial / E112 | cerrado |
| RETP-2026-052 | 18/08/2026 | 13:15:04 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J2.3 / E113 | cerrado |
| RETP-2026-053 | 18/08/2026 | 13:25:03 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J4.3 / E307 | cerrado |
| RETP-2026-054 | 18/08/2026 | 13:30:27 | SORPRESA_TECNICA_Y_REVERSION | Lenguaje SV / FFL-B / primer intento E406 revertido | cerrado |
| RETP-2026-055 | 18/08/2026 | 21:35:25 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / P0-A / estado evaluable | cerrado |
| RETP-2026-056 | 18/08/2026 | 21:35:25 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / P0-B / J3.3 / E212-E211 | cerrado |
| RETP-2026-057 | 18/08/2026 | 22:11:00 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J4.3 / E406 | cerrado |
| RETP-2026-058 | 19/08/2026 | 06:32:40 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J1.4 / E011 | cerrado |
| RETP-2026-059 | 19/08/2026 | 07:03:00 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / proyección estructural / E213-E214 | cerrado |
| RETP-2026-060 | 19/08/2026 | 09:21:19 | RECEPCION_DOCTRINAL_LATENTE | Lenguaje SV / fundamentos / no clausura certificada | cerrado |
| RETP-2026-061 | 19/08/2026 | 12:04:50 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / `resolve` / E206-E207 efectivos | cerrado |
| RETP-2026-062 | 19/08/2026 | 20:40:45 | CORRECCION_ESTRUCTURAL | Lenguaje SV / FFL-B / `graph_decl` | cerrado |
| RETP-2026-063 | 19/08/2026 | 21:11:57 | CAMBIO_DOCUMENTACION_PUBLICA | Lenguaje SV / documentación principal | cerrado |
| RETP-2026-064 | 19/08/2026 | 21:31:26 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J2.3 / régimen `Simple` / E114 | cerrado |
| RETP-2026-065 | 19/08/2026 | 21:50:14 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J3.3 / `Supervisable` | cerrado |
| RETP-2026-066 | 19/08/2026 | 22:18:33 | SINCRONIZACION_REGISTRAL_Y_CALIDAD | Lenguaje SV / calidad / actualización de registros | cerrado |
| RETP-2026-067 | 19/08/2026 | 22:31:48 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J3.2 parcial / firma de `gate` / E215 | cerrado |
| RETP-2026-068 | 19/08/2026 | 23:40:14 | CIERRE_BLOQUE_Y_SINCRONIZACION_REGISTRAL | Lenguaje SV / cierre de FFL-B / deuda técnica delimitada | cerrado |
| RETP-2026-069 | 20/08/2026 | 07:23:44 | APERTURA_BLOQUE_Y_PREPARACION_PROBATORIA | Lenguaje SV / FFL-C / pruebas y evidencia | cerrado |
| RETP-2026-070 | 20/08/2026 | 08:16:26 | CIERRE_BLOQUE_Y_SINCRONIZACION_DE_EVIDENCIA | Lenguaje SV / FFL-C / cierre probatorio | cerrado |
| RETP-2026-071 | 29/08/2026 | NO_CONSTA | INTEGRACION_PUBLICACION_Y_CIERRE_DE_CONFORMIDAD | Lenguaje SV / Beta B2 / realización estable bilingüe / cierre DFL-007 | cerrado |
| RETP-2026-072 | 04/09/2026 | 22:04:26 | FIJACION_ARQUITECTONICA | Lenguaje SV / arquitectura de software / núcleo, frontera y host | cerrado |
| RETP-2026-073 | 05/09/2026 | 13:42:20 | FIJACION_RESTRICCIONES_DE_DISENO | Lenguaje SV / pilares / frontera de autoridad dominio-agente-núcleo | cerrado |

## 3. Entradas detalladas

### RETP-2026-048 — Reapertura por Ruta A

- **Hecho:** se levanta la pausa preventiva del 16/08/2026 en el alcance autorizado y FFL-A pasa a ser la prioridad inmediata.
- **Fundamento:** las comprobaciones previas sobre continuidad semántica y arquitectónica no muestran un bloqueo del ámbito técnico inmediato.
- **Decisión:** reabrir únicamente desde FFL-A y mantener sin apertura los bloques posteriores no autorizados.
- **Estado:** cerrado.

### RETP-2026-049 — Cierre de FFL-A y apertura secuencial de FFL-B

- **Hecho:** FFL-A se cierra bajo Vía B con deuda residual explícita; FFL-B pasa a ser el único bloque técnico activo.
- **Fundamento:** la matriz diagnóstica y la tabla de correspondencias funcionales permiten localizar la divergencia restante sin exigir igualdad nominal completa entre la IR y el catálogo efectivo.
- **Decisión:** mantener FFL-C, FFL-D y FFL-E sin apertura mientras no exista una decisión posterior expresa.
- **Estado:** cerrado.

### RETP-2026-050 — Regla de materialización diagnóstica de FFL-B

- **Hecho:** FFL-B queda limitado a obligaciones ya representables, con fundamento normativo expreso, diagnóstico inequívoco y prueba trazable.
- **Decisión:** todo nuevo diagnóstico efectivo deberá conservar la correspondencia entre definición, emisión, prueba y documentación pública.
- **Estado:** cerrado.

### RETP-2026-051 — J2.2 parcial / E112

- **Hecho:** `CoupledState` sólo puede diferir de su vector de partida en posiciones pertenecientes al `BridgeSet` declarado.
- **Límite:** no queda acreditada la procedencia completa de cada actualización desde un `Connector` concreto.
- **Decisión:** cerrar únicamente la condición posicional mediante `E112`.
- **Estado:** cerrado.

### RETP-2026-052 — J2.3 / E113

- **Hecho:** se comprueban la posición puente de la arista, su concordancia con `Connector.target_position` y la compatibilidad del codominio de la célula transmisora con el conector.
- **Decisión:** utilizar `E113 — EdgeConnectorMismatch` como diagnóstico efectivo de estas incompatibilidades y mantener separada la numeración definida en la IR.
- **Estado:** cerrado.

### RETP-2026-053 — J4.3 / E307

- **Hecho:** cada tipo de suceso de `TransitionData.events` debe pertenecer al `Horizon` referido.
- **Decisión:** materializar esta condición mediante `E307`, sin ampliar la semántica de sucesos.
- **Estado:** cerrado.

### RETP-2026-054 — Incidencia técnica y reversión del primer intento E406

- **Hecho:** la primera modificación destinada a E406 fue revertida porque la comparación de cambios excedía el alcance necesario.
- **Decisión:** restaurar el estado anterior y exigir una modificación estrictamente acotada antes de reintentar E406.
- **Estado:** cerrado.

### RETP-2026-055 — P0-A / estado evaluable

- **Hecho:** `evaluate` acepta `CellState` y `CoupledState`, mientras `Frame.cell_states` conserva exclusivamente `CoupledState`.
- **Evidencia:** conformidad **42/42**, interfaz de línea de órdenes **3/3** y SEC-0 **3/3**.
- **Decisión:** preservar la distinción entre estado simple y estado acoplado.
- **Estado:** cerrado.

### RETP-2026-056 — P0-B / J3.3 / E212-E211

- **Hecho:** `supervise.meta_eval` debe ser un `EvalResult`; `E212` protege el tipo y `E211` la procedencia desde una célula con rol `Supervisor`, incluida la ruta mediante `CoupledState`.
- **Evidencia:** conformidad **44/44**, interfaz de línea de órdenes **3/3** y SEC-0 **3/3**.
- **Decisión:** mantener separados el error de tipo y el error de procedencia.
- **Estado:** cerrado.

### RETP-2026-057 — J4.3 / E406

- **Hecho:** `E406 — InsufficientTransitionData` rechaza `TransitionData` con `induced_parameters` vacío.
- **Evidencia:** conformidad **45/45**, interfaz de línea de órdenes **3/3** y SEC-0 **3/3**.
- **Límite:** una lista no vacía no demuestra por sí sola la suficiencia para reconstruir el operador inducido.
- **Estado:** cerrado.

### RETP-2026-058 — J1.4 / E011

- **Hecho:** cada salida literal de `AdmissibilityTable.table` debe pertenecer al `output_codomain` declarado.
- **Evidencia:** conformidad **46/46**, interfaz de línea de órdenes **3/3** y SEC-0 **3/3**.
- **Límite:** la condición no agota J1.4 ni acredita ejecución material de `GateResult`.
- **Estado:** cerrado.

### RETP-2026-059 — Proyección estructural / E213-E214

- **Hecho:** `E213` exige una fuente que produzca un resultado proyectable y `E214` exige que el campo pertenezca al esquema del resultado correspondiente.
- **Evidencia:** conformidad **48/48**, interfaz de línea de órdenes **3/3** y SEC-0 **3/3**.
- **Límite:** la comprobación no ejecuta el resultado ni calcula el valor proyectado.
- **Estado:** cerrado.

### RETP-2026-060 — Recepción latente de no clausura certificada

- **Hecho:** la publicación «No clausura certificada en sistemas finitos de resolución» queda recibida con estatuto `LATENTE_LEGITIMO`, sin modificación de código, gramática ni IR.
- **Fundamento:** DOI `10.21428/39829d0b.f0892864` y documentación de fundamentos del Sistema SV.
- **Evidencia temporal:** commit `be68345d…`, 19/08/2026 a las **09:21:19** (Europe/Madrid).
- **Decisión:** preservar la recepción sin traducirla automáticamente a construcciones del lenguaje.
- **Estado:** cerrado.

### RETP-2026-061 — E206/E207 efectivos en `resolve`

- **Hecho:** `E206 — ResolveMissingContext` y `E207 — ResolveMissingMechanism` son diagnósticos efectivos para la ausencia de `context` y `mechanism` en la forma superficial de `resolve`.
- **Evidencia:** conformidad **50/50**, interfaz de línea de órdenes **3/3** y SEC-0 **3/3**; lote funcional `02dc7c4e…` a las **12:04:50** (Europe/Madrid). La integración documental posterior se produjo a las 19:02:26 y no sustituye la hora del hito funcional.
- **Límite:** no se cierra J1.6, no se tipan `Context` ni `Mechanism` y no se ejecuta una resolución material de `U`.
- **Estado:** cerrado.

### RETP-2026-062 — Retirada de `conflicts` de `graph_decl`

- **Hecho:** `conflicts` deja de formar parte de `graph_decl`, `GraphDecl` y del descenso a IR. `CompositionGraph` conserva `nodes`, `edges`, `relation` y `regime`.
- **Fundamento:** la definición de `CompositionGraph` no contiene ese campo y la superficie v0.1 no dispone de una declaración completa de `ConflictOperator`.
- **Evidencia:** commit `058befbd6402c80ac7bc1d10eab0d8d035126531`; conformidad **51/51**, interfaz de línea de órdenes **3/3**, SEC-0 **3/3** y `graph_conflicts_fuera_de_v0_1.svp` → `E001`.
- **Límite:** la retirada del campo no materializa `MissingConflictOperator` ni elimina el régimen `General`.
- **Estado:** cerrado.

### RETP-2026-063 — Actualización del README principal

- **Hecho:** el README principal se actualiza para reflejar el estado técnico vigente, conservar los fundamentos y antecedentes médicos del Sistema SV, mantener la dedicatoria final y ofrecer una orientación pública acorde con el repositorio actual.
- **Evidencia:** commits `e0a58ea6af6a370960b244e9c9f06636994523d6` y `a2c4cdfdc17408ce6a989a8a1c52a8b3ff7d5e02`; la comparación respecto del estado anterior afecta únicamente a `README.md`.
- **Límite:** la actualización documental no modifica la implementación, la gramática, la IR ni el contrato diagnóstico.
- **Estado:** cerrado.

### RETP-2026-064 — J2.3 / régimen `Simple` / E114

- **Hecho:** en régimen `Simple`, cada par `(target, position)` puede recibir como máximo una arista; la concurrencia se rechaza mediante `E114 — SimpleRegimeConcurrency`.
- **Evidencia:** commit `0911d246050a9f739a3b8235f3bfd861584b4273`; conformidad **52/52**, interfaz de línea de órdenes **3/3**, SEC-0 **3/3** y emisión exacta de E114.
- **Límite:** la comprobación no se extiende al régimen `General`; `MissingConflictOperator` permanece sin materializar.
- **Estado:** cerrado.

### RETP-2026-065 — Contenido estructural de `Supervisable`

- **Hecho:** `CellTarget` exige un `EvalResult`, `ComposedTarget` un `GateResult` y `SystemTarget` un `CompositionGraph`. Una referencia existente de tipo incompatible se rechaza mediante `E006`; `E205` continúa reservado a la exigencia de constructor explícito.
- **Evidencia:** commit `59e055ea8a66437c91558e1e8096eb5c6e670b8c`; conformidad **55/55**, interfaz de línea de órdenes **3/3**, SEC-0 **3/3** y tres casos negativos específicos con E006.
- **Límite:** no se cierra la semántica completa de J3.3. La aceptación positiva de `SystemTarget(CompositionGraph)` fue comprobada de forma adicional, pero en este hito todavía no disponía de un caso válido específico conservado en la batería principal.
- **Decisión:** cerrar únicamente la correspondencia estructural entre cada constructor y su contenido.
- **Estado:** cerrado.

### RETP-2026-066 — Sincronización de los registros de calidad

- **Hecho:** los documentos vivos de calidad se actualizan para incorporar RETP-062 a RETP-065, reflejar la conformidad **55/55**, registrar las limitaciones todavía abiertas y corregir el tablero de bloques.
- **Fundamento:** el registro de evolución terminaba en RETP-061, la deuda viva no recogía E114 ni el tipado de `Supervisable`, el README de calidad mantenía **52/52** y el tablero mostraba FFL-C, FFL-D y FFL-E como abiertos pese a la secuencia vigente.
- **Evidencia temporal:** commit `359c61bc…`, 19/08/2026 a las **22:18:33** (Europe/Madrid).
- **Alcance:** sólo se modificaron documentos de `docs/calidad/`; el historial CSV hasta RETP-061 se conservó sin alteración.
- **Decisión:** mantener FFL-B como único bloque técnico activo y FFL-C, FFL-D y FFL-E en estado pendiente hasta decisión expresa.
- **Estado:** cerrado.

### RETP-2026-067 — Firma posicional de `gate` / E215

- **Hecho:** `E215 — GateTableSignatureMismatch` exige que la secuencia de `EvalResult` recibida por `gate` coincida, en número y codominio por posición, con `AdmissibilityTable.input_codomains`.
- **Fundamento:** la IR v0.2 conserva la firma ordenada de codominios de entrada y la implementación ya permite obtener el codominio de cada evaluación desde `CellState` o `CoupledState`.
- **Evidencia:** commit `15398f3441c80168f5d09866b0cba4e74221a6aa`; conformidad **57/57**, interfaz de línea de órdenes **3/3**, SEC-0 **3/3**; dos casos negativos específicos con E215 y comprobaciones positivas adicionales.
- **Límite:** la validación no ejecuta la tabla ni calcula `GateResult.output`.
- **Decisión:** cerrar únicamente la firma estructural de entrada de `gate`.
- **Estado:** cerrado.

### RETP-2026-068 — Cierre de FFL-B

- **Hecho:** FFL-B se cierra tras E215 y la revisión final de las obligaciones restantes.
- **Fundamento:** las obligaciones todavía abiertas identificadas exigen ampliar representación, semántica o ejecución: `ConflictOperator` en régimen `General`, procedencia completa de `CoupledState`, suficiencia reconstructiva de `TransitionData`, producción de `CriticalityResult`, ejecución de `GateResult.output` y semántica ejecutiva completa de `SupervisionResult`.
- **Evidencia:** base funcional `15398f3441c80168f5d09866b0cba4e74221a6aa`; conformidad **57/57**, interfaz de línea de órdenes **3/3** y SEC-0 **3/3**; deuda viva y tablas de correspondencias actualizadas; commit documental `d7b15e9…` a las **23:40:14** (Europe/Madrid).
- **Decisión:** cerrar FFL-B con deuda técnica explícita y mantener FFL-C, FFL-D y FFL-E en estado pendiente. Cualquier reapertura o apertura posterior requerirá decisión expresa y fundamento técnico identificable.
- **Estado:** cerrado.

### RETP-2026-069 — Apertura y preparación probatoria de FFL-C

- **Hecho:** FFL-C se abre para comprobar la suficiencia de la evidencia reproducible, con escritura funcional limitada a `tests/` y modo de solo lectura sobre `src/`, gramática, AST, IR, validador, catálogo diagnóstico y manual.
- **Fundamento:** FFL-B estaba cerrado y el tablero exigía una decisión expresa antes de activar el bloque de pruebas y evidencia.
- **Evidencia:** commit de apertura `910af1dd0c7ba1f811f4b77a7872626e1c3e695d` a las **07:23:44**; caso permanente `SystemTarget(CompositionGraph)` en `a0af722b0ba51cabd5d5fea9c7d719d3ba775e5d`; caracterización de E006 en `1e7ffa795fe0528888834dd35d09c321cb00c9f3`; inventario de cobertura en `3d48c422915b0e0bed65ba2e7ce8b807d7a94c33`.
- **Límite:** no se modifica implementación, gramática, IR, validador, catálogo diagnóstico ni manual.
- **Decisión:** persistir únicamente las comprobaciones que acreditan afirmaciones técnicas ya existentes y clasificar las ausencias de cobertura sin ampliar el lenguaje.
- **Estado:** cerrado.

### RETP-2026-070 — Cierre de FFL-C

- **Hecho:** una verificación independiente en modo de solo lectura ejecuta las cuatro baterías sobre `3d48c422915b0e0bed65ba2e7ce8b807d7a94c33`, con árbol limpio antes y después y código de retorno 0 en todos los ejecutores.
- **Evidencia:** conformidad **58/58** — 10 casos válidos y 48 inválidos; pruebas rápidas de la interfaz de línea de órdenes **3/3**; SEC-0 **3/3**; caracterización de E006 **4/4**; ausencia de divergencias de resultado.
- **Cobertura:** los 48 casos inválidos cubren directamente 37 de los 47 códigos efectivos. Los diez restantes se clasifican por inalcanzabilidad desde la superficie vigente, ruta diagnóstica alternativa o preservación estructural, sin fabricar casos mediante ampliación del lenguaje.
- **Límite:** la evidencia no ejecuta `GateResult.output`, no materializa la semántica completa de supervisión, no incorpora `ConflictOperator` ni produce `CriticalityResult`. La deuda de precisión de E006 permanece documentada.
- **Decisión:** cerrar FFL-C y mantener FFL-D y FFL-E pendientes, sin apertura automática de ningún bloque posterior.
- **Estado:** cerrado.

### RETP-2026-071 — Integración, publicación y cierre de conformidad de B2

- **Hecho:** B2 queda integrada y publicada como realización estable bilingüe `SVP-ES` / `SVP-EN`; se cierran los dominios gramaticales DG-01, DG-02 y DG-03 sobre identidad canónica, se constituye normativamente la capa de perfiles fuente, se corrige la navegación efectiva del Historial Beta y se completan las reconciliaciones DD-01 y VH-01 exigidas por DFL-007.
- **Fundamento:** la verificación ampliada de B2 descubrió huecos heredados de conformidad gramatical y desajustes documentales que afectaban a una base previamente cerrada. El cierre exige corrección material, regresión permanente, reconciliación normativa y revalidación de dependencias, no la mera publicación del artefacto.
- **Evidencia:** PR #55; corte de realización `c1acf943a7a44ce81080881e59283de8a2019606`; WebAssembly `378956` bytes, SHA-256 `95c7d1e0313567ef099c6e426a7fcee8ff4a5ac8adb670265f859f1bf03caab3`; paquete de despliegue `167503` bytes, SHA-256 `566200f97bfea86a0b7ce7c4919bac9d5367a67b8cba719eef1c573942d696f5`; conformidad 79/79; `sv_core` 210/210; dominios cerrados 5/5; seis sondas DG en navegador; `sv_wasm` 2/2; documentación ejecutable 17/17; comprobación material del Historial Beta; acta de conformidad de 29/08/2026.
- **Decisión:** cerrar DFL-007, re-cerrar el perímetro R0 afectado, revalidar R1 y levantar únicamente la suspensión de R2 causada por esa deuda. R2 recupera su estado abierto previo.
- **Límites:** `ConflictOperator` y J2.3 para concurrencia en régimen `General` permanecen abiertos; R2 no queda cerrado; R3 y R4 no se inician; Garantía I y Garantía II permanecen `NO_PROBADO`; la verificación externa independiente del corte final se registrará por separado.
- **Criterio registral:** el ciclo B2 se registra como un único hito material; el detalle mecánico de sus correcciones permanece en Git y en la evidencia enlazada.
- **Estado:** cerrado.

### RETP-2026-072 — Arquitectura de software: núcleo, frontera y host

- **Hecho:** se constituye un único núcleo semántico en Rust, un contrato de frontera canónico pendiente de especificación y un host operacional desacoplado. WebAssembly es candidata inicial no exclusiva; .NET es candidata de host; FFI permanece condicionada a necesidad y prueba.
- **Fundamento:** la integración hospitalaria, la periferia conectada y la ciberseguridad exigen responsabilidades explícitas sin trasladar protocolos, conectores ni reglas de dominio al núcleo.
- **Evidencia:** [acta técnica de arquitectura de software](./ACTA_TECNICA_DE_ARQUITECTURA_DE_SOFTWARE_NUCLEO_FRONTERA_Y_HOST_SV_2026_09_04.md); base `main@736ea643d7f65ba4bf26dbbb321383b8becc8d64`; revisión estructural de `PR #61@fafd65b887658d8aecf429aa1fb78b7f78174e92`, incluidos los dos comparadores que aplican `sort_keys=True` y los 68 casos inválidos sin contraejemplo de `failure_symbol != Bottom`.
- **Decisión:** conservar una sola fuente semántica en `sv_core`; especificar la frontera antes de elegir el host; mantener plataforma, ABI, conectores y persistencia sujetos a prototipo, licencias y comparación reproducible; declarar por separado perfiles fuente, perfiles de dominio y cobertura de agentes.
- **Límites:** no se integra la PR #61, no se abre fase material, no se incorpora el laboratorio privado, no se fija .NET ni WebAssembly como solución definitiva y no se autoriza uso clínico ni datos reales.
- **Estado:** cerrado.

### RETP-2026-073 — Pilares y restricciones de diseño del Lenguaje SV

- **Hecho:** se fija una pieza rectora que preserva la célula exacta como vector plano, ordenado y posicional de longitud `n=b²`; separa `U` de cualquier fallo o relleno; y delimita la autoridad entre las unidades de dominio, las unidades de agente, el Lenguaje, la frontera, el host y cualquier IA auxiliar.
- **Fundamento:** la realización vigente acredita `b≥3`, la derivación de `n` y la longitud del vector, pero no representa todavía el contrato completo dominio→células→agente. Esa ausencia no puede permitir que el núcleo decida tamaños, distribuya inventarios, rellene posiciones o acepte en silencio una constitución incompleta.
- **Evidencia:** [Pilares y restricciones de diseño](./PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md); base `main@ec00a5464239df081d0165fd09a1b738c579555b`; inspección de `frontend.rs`, `ir.rs`, `wellformed.rs`, `svp_validator.py`, `svp_ir.py` e IR v0.3; contraste con los fundamentos algebraico-semánticos, la doctrina de `U` y la arquitectura general de agentes especializados.
- **Decisión:** el dominio constituye sus células y asignaciones; el agente recibe esa constitución y declara cobertura y capacidades; el Lenguaje valida y preserva sin suplantar. Se prohíben inferencia, redondeo, relleno, reordenación, reparación silenciosa y conversión de fallos a `U`. La ausencia de representación se declara como obligación pendiente.
- **Límites:** no se elige `b` para ningún dominio, no se decide el número de células, no se asignan parámetros, no se constituye un bus o un perfil central, no se selecciona host y no se abre fase material. La ejecución algebraica completa en `sv_core` permanece no acreditada.
- **Corrección registral:** se repara la serialización CSV de `RETP-2026-072`, que había quedado encapsulada como una sola celda, sin alterar su contenido material.
- **Estado:** cerrado.

## 4. Estado de continuidad

FFL-A, FFL-B, FFL-C y FFL-E permanecen cerrados; FFL-D permanece pendiente dentro de su alcance propio.

El estado de realización posterior al cierre correctivo B2 es:

```text
R0 = CERRADO, incluido el perímetro correctivo de DFL-007
R1 = CERRADO y revalidado sobre la base corregida
R2 = ABIERTO; levantada la suspensión específica causada por DFL-007
R3 = NO INICIADO
R4 = NO INICIADO

Garantía I  = NO_PROBADO
Garantía II = NO_PROBADO
```

La deuda técnica restante permanece registrada en `REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`. El cierre de DFL-007 no materializa `ConflictOperator` ni completa J2.3 para concurrencia en régimen `General`. La verificación externa independiente del corte final permanece pendiente y se documentará separadamente.

## 5. Numeración registral

La numeración RETP se mantiene en `REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
