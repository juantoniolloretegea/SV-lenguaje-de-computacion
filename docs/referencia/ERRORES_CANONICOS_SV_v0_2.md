# Catálogo efectivo de errores del Lenguaje SV — v0.2

## 1. Naturaleza y subordinación

Este documento describe el catálogo efectivo utilizado por la etapa frontal de referencia. Su función es operativa y técnica. La IR v0.2 conserva la autoridad normativa superior sobre los juicios de bienformación.

La relación entre ambos contratos se documenta mediante la regularización por Vía B y los documentos de concordancia de `docs/calidad/`.

## 2. Estado vigente

Tras la incorporación de `E215 — GateTableSignatureMismatch` constan:

- **47 códigos** en el catálogo efectivo;
- **5 coincidencias semánticas por mismo identificador** con la IR v0.2: `E102`, `E104`, `E106`, `E111`, `E406`;
- **20 códigos** con el mismo identificador y significado distinto;
- **22 códigos** presentes sólo en la implementación efectiva;
- **13 códigos** presentes sólo en la IR v0.2.

Los recuentos anteriores describen la relación por identificador. Una obligación definida por la IR puede estar protegida mediante otro diagnóstico; esa correspondencia se registra por separado.

## 3. Precisiones vigentes

`E112` materializa la restricción posicional verificable de `J2.2`; no acredita la procedencia completa de cada actualización desde un `Connector`.

`E113` protege la compatibilidad representable de `J2.3` entre `Edge`, `BridgeSet`, `target_position` y codominio fuente.

`E114` materializa la unicidad de `(target, position)` exigida por `J2.3` en régimen `Simple`. No posee identificador autónomo equivalente en la IR v0.2 y no materializa `E204 — MissingConflictOperator` ni la cláusula de régimen `General`.

`E212` exige que `supervise.meta_eval` sea un `EvalResult`; `E211` mantiene separada la obligación de procedencia desde una célula con rol `Supervisor`.

`E006` se utiliza tanto para una referencia inexistente como para una referencia existente cuyo tipo no corresponde a la clase exigida. Esta reutilización es funcional, pero el nombre `UndeclaredReference` y su mensaje base describen con mayor precisión el primer supuesto. La diferencia queda registrada como deuda diagnóstica.

`E307` exige que cada tipo de suceso de `TransitionData.events` pertenezca al `Horizon` declarado.

`E406` coincide con la IR v0.2 para la condición `induced_parameters` no vacío. No acredita por sí solo la suficiencia reconstructiva completa de `J4.3`.

`E011` exige que cada salida literal de `AdmissibilityTable.table` pertenezca a `output_codomain`.

`E213` y `E214` protegen la legalidad estructural de la proyección de campos de resultados. No ejecutan el resultado ni calculan el valor proyectado.

`E206 — ResolveMissingContext` y `E207 — ResolveMissingMechanism` se emiten ante la ausencia acreditada de los campos obligatorios `context` y `mechanism` en la forma superficial de `resolve`. No cierran J1.6 ni ejecutan una resolución de `U`.

`E215 — GateTableSignatureMismatch` exige que la lista de `EvalResult` recibida por `gate` tenga la misma longitud que `AdmissibilityTable.input_codomains` y que el codominio correspondiente a cada posición coincida exactamente. La comparación es nominal y posicional: dos codominios distintos no se consideran equivalentes por compartir el mismo conjunto de valores. E215 no ejecuta la tabla ni calcula `GateResult.output`.

## 4. Catálogo efectivo

| Código | Nombre | Capa | Fase | Situación respecto de IR v0.2 | Mensaje base o alcance |
|---|---|---|---|---|---|
| E001 | `InvalidTriValue` | Definición | `parse` | divergente | Valor ternario no reconocido: se esperaba Zero, One o U |
| E002 | `InvalidBValue` | Definición | `validate` | divergente | El valor de b debe ser un entero >= 3 |
| E003 | `NSquaredViolation` | Definición | `lower` | sólo implementación | n debe ser b² |
| E004 | `EmptyCodomain` | Definición | `validate` | sólo implementación | El codominio declarado no puede estar vacío |
| E005 | `DuplicateIdentifier` | Definición | `validate` | sólo implementación | Identificador ya declarado en el ámbito actual |
| E006 | `UndeclaredReference` | Definición | `validate` | sólo implementación | Referencia inexistente; se reutiliza también para incompatibilidad estructural de tipo |
| E007 | `InvalidConnectorMapping` | Definición | `validate` | sólo implementación | El mapeo del conector no cubre todos los valores del codominio fuente |
| E008 | `ConnectorTargetNotTri` | Definición | `validate` | sólo implementación | El destino del mapeo del conector debe ser un literal ternario |
| E009 | `TableInputMismatch` | Definición | `validate` | sólo implementación | Las filas de la tabla no cubren exactamente el producto cartesiano de los codominios de entrada |
| E010 | `InvalidRole` | Definición | `parse` | sólo implementación | Rol no reconocido |
| E011 | `TableOutputNotInCodomain` | Definición | `validate` | sólo implementación | La salida de una fila debe pertenecer al codominio de salida declarado |
| E101 | `VectorLengthMismatch` | Estado | `validate` | divergente | La longitud del vector no coincide con b² de la `CellSpec` referenciada |
| E102 | `MissingOutputSemantics` | Estado | `validate` | coincidente | La `CellSpec` referencia una `OutputSemantics` no declarada o de tipo incorrecto |
| E103 | `GraphCycleDetected` | Estado | `validate` | divergente | El grafo de composición contiene ciclos |
| E104 | `InvalidConnectorCodomain` | Estado | `validate` | coincidente | El conector declara un destino fuera del alfabeto ternario permitido |
| E105 | `BridgePositionOutOfRange` | Estado | `validate` | divergente | Posición puente fuera del rango [1, n] de la célula |
| E106 | `MissingSemanticRelation` | Estado | `validate` | coincidente | Composición sin relación semántica previa declarada |
| E111 | `UnorderedCodomain` | Estado | `validate` | coincidente | Codominio usado en compuerta sin orden documentado |
| E112 | `IllegalBridgeUpdate` | Estado | `validate` | sólo implementación | `CoupledState` modifica una posición fuera del `BridgeSet` declarado |
| E113 | `EdgeConnectorMismatch` | Estado | `validate` | sólo implementación | `Edge` incompatible con `BridgeSet`, `target_position` o codominio fuente declarados |
| E114 | `SimpleRegimeConcurrency` | Estado | `validate` | sólo implementación | El régimen Simple no admite más de una arista sobre la misma posición puente de una célula receptora |
| E201 | `LiteralResultForbidden` | Resultado | `parse` | divergente | No se permite construir literales de objetos de resultado |
| E202 | `GateInputNotEvalResult` | Resultado | `validate` | divergente | Los argumentos de `gate` deben ser identificadores de `EvalResult` |
| E203 | `GateMissingTable` | Resultado | `parse` | divergente | `gate` invocado sin tabla de admisibilidad nombrada |
| E204 | `QueryMissingContext` | Resultado | `parse` | divergente | `query` invocado sin constructor explícito de `QueryContext` |
| E205 | `SuperviseOpaqueTarget` | Resultado | `parse` | divergente | `supervise` invocado sin constructor explícito de `Supervisable` |
| E206 | `ResolveMissingContext` | Resultado | `parse` | divergente | `resolve` invocado sin contexto de evidencia |
| E207 | `ResolveMissingMechanism` | Resultado | `parse` | sólo implementación | `resolve` invocado sin mecanismo de revisión |
| E208 | `ComposeMissingRelations` | Resultado | `parse` | sólo implementación | `compose` invocado sin lista de relaciones |
| E209 | `ComposeMissingPatterns` | Resultado | `parse` | sólo implementación | `compose` invocado sin lista de patrones |
| E210 | `MaxMinForbidden` | Resultado | `parse` | sólo implementación | `max/min` no están disponibles en la superficie v0.1 |
| E211 | `SuperviseMetaNotSupervisor` | Resultado | `validate` | sólo implementación | El primer argumento de `supervise` debe provenir de una célula con rol `Supervisor` |
| E212 | `SuperviseMetaNotEvalResult` | Resultado | `validate` | sólo implementación | El primer argumento de `supervise` debe ser un identificador de `EvalResult` |
| E213 | `ProjectionSourceNotResult` | Resultado | `validate` | sólo implementación | La fuente de una proyección debe ser un objeto de resultado producido por un operador compatible |
| E214 | `ProjectionFieldNotFound` | Resultado | `validate` | sólo implementación | El campo proyectado debe pertenecer al esquema del tipo de resultado de la fuente |
| E215 | `GateTableSignatureMismatch` | Resultado | `validate` | sólo implementación | La secuencia de entradas de `gate` debe coincidir, en número y codominio por posición, con los codominios de entrada de la tabla |
| E301 | `FrameMutationForbidden` | Evolución | `validate` | divergente | No se permite modificar un `Frame` existente |
| E302 | `TrajectoryMutationForbidden` | Evolución | `validate` | divergente | No se permite modificar, eliminar ni reordenar entradas de una `Trajectory` |
| E303 | `TransitionDataMissingHorizon` | Evolución | `validate` | divergente | `TransitionData` declarado sin referencia válida a `Horizon` |
| E304 | `TrajectoryAlternanceViolation` | Evolución | `validate` | divergente | La secuencia de entradas de `Trajectory` no respeta la alternancia constitutiva |
| E307 | `UndeclaredHorizonEvent` | Evolución | `validate` | sólo implementación | `TransitionData` referencia un tipo de suceso ajeno al `Horizon` declarado |
| E406 | `InsufficientTransitionData` | Evolución | `validate` | coincidente | `TransitionData` debe especificar al menos un cambio en `induced_parameters` |
| E401 | `DomainPortContractViolation` | Uso | `validate` | divergente | `Domain` incumple el contrato mínimo de enlace declarado |
| E402 | `AgentDomainContractViolation` | Uso | `validate` | divergente | `Agent` incompatible con el `Domain` o con la arquitectura declarada |
| E403 | `QueryContractViolation` | Uso | `validate` | divergente | `QuerySpec` o `QueryContext` incompatibles con el contrato de uso |
| E501 | `SerializationNonDeterministic` | Serialización/conformidad | `lower` | divergente | La serialización JSON no es determinista |
| E507 | `UCoercionDetected` | Serialización/conformidad | `parse` | divergente | Coerción implícita de U detectada |

## 5. Emisión observable y cobertura explícita

Disponen de punto de emisión directo observable, al menos:

`E001`, `E002`, `E004`, `E005`, `E006`, `E007`, `E009`, `E010`, `E011`, `E101`, `E102`, `E103`, `E104`, `E105`, `E112`, `E113`, `E114`, `E202`, `E204`, `E205`, `E206`, `E207`, `E208`, `E209`, `E210`, `E211`, `E212`, `E213`, `E214`, `E215`, `E303`, `E304`, `E307`, `E406`, `E401`, `E402`, `E403`, `E507`.

La batería de conformidad vigente contiene casos con código esperado declarado, al menos, para:

`E001`, `E002`, `E005`, `E006`, `E007`, `E009`, `E010`, `E011`, `E101`, `E102`, `E103`, `E104`, `E105`, `E112`, `E113`, `E114`, `E202`, `E204`, `E205`, `E206`, `E207`, `E208`, `E209`, `E210`, `E211`, `E212`, `E213`, `E214`, `E215`, `E303`, `E304`, `E307`, `E406`, `E401`, `E402`, `E403`, `E507`.

`E008` permanece en el catálogo por trazabilidad, sin punto de emisión directo ni caso explícito en la batería vigente. El subcaso superficial de destino no ternario de conector se emite actualmente como `E104`.

`E004` mantiene punto de emisión directo, pero la superficie v0.1 no permite declarar actualmente un `codomain` vacío y por ello no dispone de un caso superficial específico.

La cobertura de un diagnóstico no equivale por sí sola al cierre completo del juicio de la IR relacionado.

## 6. Resultados y límites estructurales

Los campos reconocidos por E213/E214 son los fijados por la IR v0.2 para los cinco resultados producidos por operadores de superficie:

| Productor superficial | Tipo de resultado IR | Campos |
|---|---|---|
| `evaluate` | `EvalResult` | `source_state`, `counts`, `threshold`, `classification`, `criticality`, `deltas` |
| `gate` | `GateResult` | `inputs`, `table`, `output` |
| `resolve` | `ResolutionRecord` | `parameter`, `previous`, `resolved_to`, `context`, `mechanism` |
| `query` | `QueryResult` | `response`, `justification`, `metadata` |
| `supervise` | `SupervisionResult` | `meta_eval`, `target`, `verdict` |

`CriticalityResult` no se incorpora porque la superficie v0.1 no dispone de un operador que lo produzca.

E215 completa la comprobación estructural de la firma de entrada de `gate`. La determinación de `GateResult.output` permanece fuera del alcance de esta etapa.

## 7. Cierre de FFL-B

FFL-B queda cerrado con la evidencia **57/57**, pruebas rápidas de la interfaz de línea de órdenes **3/3** y SEC-0 **3/3**. Las obligaciones restantes que requieren ampliar representación, semántica o ejecución permanecen registradas como deuda y no se consideran resueltas.

Cualquier modificación posterior del catálogo efectivo deberá mantener sincronizados el punto de emisión, la evidencia ejecutable y los documentos públicos de concordancia.
