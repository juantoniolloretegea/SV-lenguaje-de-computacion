# ERRORES_CANONICOS_SV_v0_2.md

## 1. Naturaleza y subordinación

Este documento tiene carácter **operativo, técnico y subordinado**.
No sustituye la autoridad doctrinal vigente del Sistema Vectorial SV ni modifica por sí mismo la IR canónica v0.2.

Su función es describir, de forma completa y revisable, el **catálogo efectivo de errores** utilizado por la etapa frontal de referencia en el estado actual del proyecto, conforme a la decisión:

- `docs/calidad/C1C_DECISION_REGULARIZACION_CONTRATO_DIAGNOSTICO.md`

## 2. Objeto

Documentar el contrato diagnóstico actualmente vigente en:

- `src/svp_errors.py`;
- `src/svp_validator.py`;
- `tests/run_conformance.py`;

y ofrecer una referencia humana única para el significado efectivo de los códigos que hoy gobiernan el análisis sintáctico y el descenso a IR, la validación y la batería de conformidad.

## 3. Alcance

Este documento describe el catálogo efectivo **tal como existe hoy**.
No debe interpretarse como sustitución de la IR v0.2 ni como cancelación de la convergencia futura hacia la norma superior.

## 4. Resumen del estado actual

Constan actualmente:

- **43 códigos** en el catálogo efectivo;
- **5 códigos** coincidentes con la IR v0.2 (`E102`, `E104`, `E106`, `E111`, `E406`);
- **20 códigos** con el mismo identificador pero significado divergente respecto de la IR v0.2;
- **18 códigos** presentes en la implementación y no codificados en la IR v0.2.

La actualización de `E102` y `E104` no reescribe el contraste histórico: registra que la etapa frontal vigente emite ya esos identificadores para las mismas obligaciones diagnósticas que fija la IR v0.2. Las antiguas caídas de esos subcasos a `E006` y `E008` quedaron superadas por la resincronización posterior de la implementación.

`E112` materializa bajo Vía B, con identificador efectivo libre, la parte verificable de la obligación canónica `E202 — IllegalBridgeUpdate` que exige que `updated_vector` difiera de `base_vector` únicamente en posiciones del `BridgeSet`. La procedencia de los valores desde un `Connector` bien formado permanece fuera de ese cierre parcial.

`E113` materializa bajo Vía B la obligación canónica `E206 — EdgeConnectorMismatch` en la superficie actualmente representable: la posición de la arista debe pertenecer al `BridgeSet` del destino, debe coincidir con `Connector.target_position` y `Connector.source_codomain` debe coincidir con el codominio de la célula transmisora. La validez interna de `Connector.mapping` continúa gobernada por las comprobaciones efectivas `E104` y `E007`.

`E212` materializa bajo Vía B la precondición de `J3.3` según la cual `SupervisionResult.meta_eval` debe ser un `EvalResult`. `E211` conserva la comprobación distinta de procedencia desde una célula con rol `Supervisor`, aplicable tanto a evaluaciones de `CellState` como de `CoupledState` conforme a la adenda técnica de estado evaluable acoplado. Esta materialización no altera `E306 — UntaggedSupervisable`, que gobierna el etiquetado del objeto supervisado.

`E307` materializa bajo Vía B la obligación canónica `E403 — UndeclaredHorizonEvent`: cada `event_type` de `TransitionData.events` debe pertenecer a `Horizon.events` del `horizon_ref` declarado. El identificador canónico `E403` no se reutiliza porque en el contrato efectivo vigente significa `QueryContractViolation`.

`E406` converge por identificador y significado con la IR v0.2 para la obligación explícita de `J4.3` que exige que `induced_parameters` no esté vacío. Esta convergencia **no cierra por sí sola todo J4.3**: la cláusula adicional de suficiencia para reconstruir el operador inducido conserva el estatuto que corresponda mientras no exista comprobación material independiente.

`E011` materializa bajo Vía B una condición tipada de `AdmissibilityTable` ya fijada por la IR: toda salida literal de `table` debe pertenecer al `output_codomain` declarado. La tabla canónica de errores no asigna un identificador autónomo a esta condición. `E011` no sustituye ni redefine `E105 — IncompleteAdmissibilityTable` ni `E106 — MissingSemanticRelation`, y no cierra por sí solo todo J1.4.

## 5. Regla de uso

Mientras siga vigente la regularización por Vía B:

- este documento describe el **contrato diagnóstico efectivo** de la etapa frontal de referencia;
- la IR v0.2 conserva su autoridad normativa superior;
- toda futura convergencia deberá tratarse como acto formal separado.

## 6. Catálogo efectivo vigente

| Código | Nombre | Capa | Fase | Situación respecto de IR v0.2 | Mensaje base |
|---|---|---|---|---|---|
| E001 | `InvalidTriValue` | Definición | `parse` | divergente respecto de IR v0.2 | Valor ternario no reconocido: se esperaba Zero, One o U |
| E002 | `InvalidBValue` | Definición | `validate` | divergente respecto de IR v0.2 | El valor de b debe ser un entero >= 3 |
| E003 | `NSquaredViolation` | Definición | `lower` | no consta en IR v0.2 | n debe ser b² (derivado automáticamente, no declarado por el usuario) |
| E004 | `EmptyCodomain` | Definición | `validate` | no consta en IR v0.2 | El codominio declarado no puede estar vacío |
| E005 | `DuplicateIdentifier` | Definición | `validate` | no consta en IR v0.2 | Identificador ya declarado en el ámbito actual |
| E006 | `UndeclaredReference` | Definición | `validate` | no consta en IR v0.2 | Referencia a identificador no declarado |
| E007 | `InvalidConnectorMapping` | Definición | `validate` | no consta en IR v0.2 | El mapeo del conector no cubre todos los valores del codominio fuente |
| E008 | `ConnectorTargetNotTri` | Definición | `validate` | no consta en IR v0.2 | El destino del mapeo del conector debe ser un literal ternario (Zero, One, U) |
| E009 | `TableInputMismatch` | Definición | `validate` | no consta en IR v0.2 | Las entradas de la tabla de admisibilidad no cubren el producto cartesiano de los codominios |
| E010 | `InvalidRole` | Definición | `parse` | no consta en IR v0.2 | Rol no reconocido: se esperaba Base, Supervisor o Composite |
| E011 | `TableOutputNotInCodomain` | Definición | `validate` | no consta en IR v0.2 | La salida de una fila de la tabla de admisibilidad debe pertenecer al codominio de salida declarado |
| E101 | `VectorLengthMismatch` | Estado | `validate` | divergente respecto de IR v0.2 | La longitud del vector no coincide con b² de la CellSpec referenciada |
| E102 | `MissingOutputSemantics` | Estado | `validate` | coincidente con IR v0.2 | La CellSpec referencia una OutputSemantics no declarada o de tipo incorrecto |
| E103 | `GraphCycleDetected` | Estado | `validate` | divergente respecto de IR v0.2 | El grafo de composición contiene ciclos (prohibido) |
| E104 | `InvalidConnectorCodomain` | Estado | `validate` | coincidente con IR v0.2 | El conector declara un destino fuera del alfabeto ternario permitido |
| E105 | `BridgePositionOutOfRange` | Estado | `validate` | divergente respecto de IR v0.2 | Posición puente fuera del rango [1, n] de la célula |
| E106 | `MissingSemanticRelation` | Estado | `validate` | coincidente con IR v0.2 | Composición sin relación semántica previa declarada |
| E111 | `UnorderedCodomain` | Estado | `validate` | coincidente con IR v0.2 | Codominio usado en compuerta sin orden documentado |
| E112 | `IllegalBridgeUpdate` | Estado | `validate` | no consta en IR v0.2 | CoupledState modifica una posición fuera del BridgeSet declarado |
| E113 | `EdgeConnectorMismatch` | Estado | `validate` | no consta en IR v0.2 | Edge incompatible con BridgeSet, target_position o codominio fuente declarados |
| E201 | `LiteralResultForbidden` | Resultado | `parse` | divergente respecto de IR v0.2 | No se permite construir literales de EvalResult, GateResult, ResolutionRecord, QueryResult ni SupervisionResult |
| E202 | `GateInputNotEvalResult` | Resultado | `validate` | divergente respecto de IR v0.2 | Los argumentos de gate deben ser identificadores de EvalResult |
| E203 | `GateMissingTable` | Resultado | `parse` | divergente respecto de IR v0.2 | gate invocado sin tabla de admisibilidad nombrada (using) |
| E204 | `QueryMissingContext` | Resultado | `parse` | divergente respecto de IR v0.2 | query invocado sin constructor explícito de QueryContext |
| E205 | `SuperviseOpaqueTarget` | Resultado | `parse` | divergente respecto de IR v0.2 | supervise invocado sin constructor explícito de Supervisable |
| E206 | `ResolveMissingContext` | Resultado | `parse` | divergente respecto de IR v0.2 | resolve invocado sin contexto de evidencia |
| E207 | `ResolveMissingMechanism` | Resultado | `parse` | no consta en IR v0.2 | resolve invocado sin mecanismo de revisión |
| E208 | `ComposeMissingRelations` | Resultado | `parse` | no consta en IR v0.2 | compose invocado sin lista de relaciones |
| E209 | `ComposeMissingPatterns` | Resultado | `parse` | no consta en IR v0.2 | compose invocado sin lista de patrones |
| E210 | `MaxMinForbidden` | Resultado | `parse` | no consta en IR v0.2 | max/min no están disponibles en la superficie v0.1 |
| E211 | `SuperviseMetaNotSupervisor` | Resultado | `validate` | no consta en IR v0.2 | El primer argumento de supervise debe provenir de una célula con rol Supervisor |
| E212 | `SuperviseMetaNotEvalResult` | Resultado | `validate` | no consta en IR v0.2 | El primer argumento de supervise debe ser un identificador de EvalResult |
| E301 | `FrameMutationForbidden` | Evolución | `validate` | divergente respecto de IR v0.2 | No se permite modificar un Frame existente (inmutable por tipo) |
| E302 | `TrajectoryMutationForbidden` | Evolución | `validate` | divergente respecto de IR v0.2 | No se permite modificar, eliminar ni reordenar entradas de una Trajectory (append-only por tipo) |
| E303 | `TransitionDataMissingHorizon` | Evolución | `validate` | divergente respecto de IR v0.2 | TransitionData declarado sin referencia a Horizon |
| E304 | `TrajectoryAlternanceViolation` | Evolución | `validate` | divergente respecto de IR v0.2 | La secuencia de entradas de Trajectory no respeta las invariantes de alternancia |
| E307 | `UndeclaredHorizonEvent` | Evolución | `validate` | no consta en IR v0.2 | TransitionData referencia un tipo de suceso que no pertenece al Horizon declarado |
| E406 | `InsufficientTransitionData` | Evolución | `validate` | coincidente con IR v0.2 | TransitionData debe especificar al menos un cambio en induced_parameters |
| E401 | `DomainPortContractViolation` | Uso | `validate` | divergente respecto de IR v0.2 | Domain incumple el contrato mínimo de enganche declarado |
| E402 | `AgentDomainContractViolation` | Uso | `validate` | divergente respecto de IR v0.2 | Agent incompatible con el Domain o con la arquitectura declarada |
| E403 | `QueryContractViolation` | Uso | `validate` | divergente respecto de IR v0.2 | QuerySpec o QueryContext incompatibles con el contrato de uso |
| E501 | `SerializationNonDeterministic` | Serialización/conformidad | `lower` | divergente respecto de IR v0.2 | La serialización JSON no es determinista |
| E507 | `UCoercionDetected` | Serialización/conformidad | `parse` | divergente respecto de IR v0.2 | Coerción implícita de U detectada (prohibición constitutiva) |

## 7. Emisión observable y casos explícitos de conformidad

### 7.1. Códigos con emisión directa observable en el código fuente

Constan con punto de emisión directo observable en la etapa frontal actual, al menos, los siguientes códigos:

`E001`, `E002`, `E004`, `E005`, `E006`, `E007`, `E009`, `E010`, `E011`, `E101`, `E102`, `E103`, `E104`, `E105`, `E112`, `E113`, `E202`, `E204`, `E205`, `E208`, `E209`, `E210`, `E211`, `E212`, `E303`, `E304`, `E307`, `E406`, `E401`, `E402`, `E403`, `E507`.

### 7.2. Códigos con caso explícito de conformidad declarado

La batería de conformidad vigente contiene casos inválidos con código esperado declarado, al menos, para:

`E001`, `E002`, `E005`, `E006`, `E007`, `E009`, `E010`, `E011`, `E101`, `E102`, `E103`, `E104`, `E105`, `E112`, `E113`, `E202`, `E204`, `E205`, `E208`, `E209`, `E210`, `E211`, `E212`, `E303`, `E304`, `E307`, `E406`, `E401`, `E402`, `E403`, `E507`.

La presencia de un caso y de su código esperado no equivale, por sí sola, a una nueva ejecución global acreditada de la batería. La suficiencia dinámica de la evidencia se cerrará específicamente en `FFL-C`.

### 7.3. Cautelas sobre catálogo, emisión y cobertura

La mera presencia de un código en `src/svp_errors.py` no implica, por sí sola, que exista hoy un punto de emisión directo observable ni que esté cubierto por la batería.

`E008` permanece en el catálogo por trazabilidad, pero **no dispone hoy de punto de emisión directo ni de caso explícito en la batería**. El subcaso superficial de destino no ternario de conector se emite actualmente como `E104`.

El caso `coupledstate_update_fuera_bridges.svp` fija el caso explícito de `E112` para la parte verificable de `J2.2`. Este cierre es **parcial respecto de J2.2**: no acredita todavía que el valor actualizado proceda de un `Connector` bien formado.

Los casos `edge_position_fuera_bridges.svp`, `edge_connector_target_position_mismatch.svp` y `edge_connector_source_codomain_mismatch.svp` separan las tres incompatibilidades contextuales verificables de `J2.3` que se emiten como `E113`. Junto con las comprobaciones internas ya existentes de `Connector.mapping` (`E104`/`E007`), constituyen la ruta efectiva de la obligación canónica `E206 — EdgeConnectorMismatch` bajo Vía B.

Los casos `supervise_meta_no_evalresult.svp` y `supervise_coupled_wrong_role.svp` separan las dos precondiciones del primer argumento de `supervise`: `E212` rechaza una referencia existente que no sea `EvalResult`, mientras `E211` rechaza un `EvalResult` cuya célula fuente no tenga rol `Supervisor`, incluido el camino acoplado `CoupledState → CoupledSpec → CellSpec`.

El caso `transition_event_fuera_horizon.svp` fija la ruta efectiva `E307` para la obligación canónica `E403 — UndeclaredHorizonEvent`, sin sustituir el significado efectivo vigente de `E403`.

El caso `transition_induced_parameters_vacios.svp` fija `E406` para la obligación literal de `J4.3` que prohíbe un `TransitionData` sin cambios inducidos declarados. El caso no acredita por sí solo la cláusula más amplia de reconstrucción del operador inducido.

El caso `admissibility_table_output_fuera_codominio.svp` fija `E011` para una tabla cuyas entradas cubren el producto cartesiano pero cuya salida literal no pertenece al `output_codomain`. Se mantiene separado del caso `admissibility_table_incompleta.svp`, que conserva `E009` para la cobertura de entradas.

El código `E004` mantiene punto de emisión directo en el validador, pero sigue sin caso explícito en la superficie v0.1 porque el analizador sintáctico no permite actualmente declarar un `codomain` vacío.

### 7.4. Estado fino de la emitibilidad pública de `E301–E304`

- `E304` dispone de emisión observable directa y caso explícito.
- `E301` y `E302` permanecen como invariantes de tipo sin operación superficial autónoma publicada.
- `E303` dispone de emisión observable directa y caso explícito mediante `transition_data_horizon_no_declarado.svp`.

## 8. Regla de continuidad

Si el catálogo efectivo cambia, este documento deberá actualizarse en el mismo bloque de trabajo que altere `src/svp_errors.py`, `src/svp_validator.py`, `tests/run_conformance.py` o la documentación pública correspondiente.

## 9. Vigencia

Este documento permanece vigente mientras la etapa frontal de referencia mantenga un catálogo efectivo no plenamente reconciliado con la IR v0.2.
