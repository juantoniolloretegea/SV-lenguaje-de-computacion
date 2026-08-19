# Tabla de correspondencias funcionales entre obligaciones diagnósticas de la IR v0.2 y la etapa frontal bajo Vía B

**Fecha de origen:** 18/08/2026  
**Última resincronización:** 19/08/2026  
**Ámbito:** FFL-A/FFL-B — contrato diagnóstico  
**Autoridad superior:** `IR_CANONICA_BIENFORMACION_SV_v0_2.md`  
**Contrato efectivo subordinado:** `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md` + `src/svp_errors.py`

El nombre histórico de este archivo contiene `CROSSWALK` y `FRONTEND`. Se conserva para mantener la trazabilidad de referencias ya publicadas. Su función vigente es documentar correspondencias funcionales entre la IR y la etapa frontal del compilador.

## 1. Objeto

La matriz de concordancia clasifica la relación por identificador. Esta tabla determina, además, si una obligación definida por la IR está protegida mediante otro código, queda impedida estructuralmente, está cubierta sólo en parte o permanece sin materializar.

Estados utilizados:

- `CONVERGENTE_ID`;
- `CUBIERTO_OTRO_ID`;
- `CUBIERTO_ESTRUCTURAL`;
- `PARCIAL`;
- `NO_MATERIALIZADO`.

La ausencia del identificador previsto por la IR no implica por sí sola ausencia de protección. La coincidencia numérica tampoco acredita identidad material.

## 2. Correspondencias por obligación de la IR

| IR | Obligación | Ruta efectiva vigente | Estado funcional | Observación |
|---|---|---|---|---|
| E001 | `InvalidTriCoercion` | `E507` para coerción implícita de `U`; `E001` efectivo para literal Tri inválido | PARCIAL | La protección observable de `U` existe, pero E001 efectivo no representa toda la obligación de la IR. |
| E002 | `InvalidCellSize` | `E002` valida `b >= 3`; el descenso a IR deriva `n=b²` | CUBIERTO_ESTRUCTURAL | La superficie no declara `n` independientemente. |
| E101 | `EmptyCodomain` | `E004 — EmptyCodomain` | CUBIERTO_OTRO_ID | Existe punto de emisión, aunque la superficie no permite hoy un caso vacío explícito. |
| E102 | `MissingOutputSemantics` | `E102 — MissingOutputSemantics` | CONVERGENTE_ID | Emisión directa y cobertura explícita. |
| E103 | `IllegalBridgePosition` | `E105 — BridgePositionOutOfRange` | CUBIERTO_OTRO_ID | Emisión directa y cobertura explícita. |
| E104 | `InvalidConnectorCodomain` | `E104 — InvalidConnectorCodomain` | CONVERGENTE_ID | Emisión directa y cobertura explícita. |
| E105 | `IncompleteAdmissibilityTable` | `E009 — TableInputMismatch` | CUBIERTO_OTRO_ID | Comprueba faltantes, elementos adicionales y duplicados del producto cartesiano. |
| E106 | `MissingSemanticRelation` | `E106` existe; la forma exige `relation` y una referencia ausente cae en `E006` | CONVERGENTE_ID | Coincidencia del catálogo sin emisión superficial autónoma de E106. |
| E107 | `InvalidTernarizerPartition` | `E401` comprueba únicamente presencia no vacía de las tres referencias de partición | PARCIAL | No se acredita cobertura, disjunción ni ausencia de solapamiento. |
| E108 | `MissingResContext` | ausencia de `context` en `resolve` → `E206` efectivo; ausencia de `mechanism` → `E207` efectivo | PARCIAL | Se protege la presencia de ambos campos; no se acredita el resto de J1.6. |
| E109 | `InvalidCaptureSpec` | `E401` comprueba `parameter_id`, `observation_space` y `Bottom` | PARCIAL | No se acredita todo J1.7. |
| E110 | `InvalidAdmissibilitySpec` | `E401` comprueba `parameter_id`, estados y presencia de `rule` | PARCIAL | No se acredita el determinismo material de `rule`. |
| E111 | `UnorderedCodomain` | `E111` existe; `max/min` están prohibidos mediante `E210` | CUBIERTO_ESTRUCTURAL | La condición no es alcanzable mediante `max/min` en la superficie actual. |
| E201 | `VectorLengthMismatch` | `E101 — VectorLengthMismatch` | CUBIERTO_OTRO_ID | Se comprueba en `CellState` y en ambos vectores de `CoupledState`. |
| E202 | `IllegalBridgeUpdate` | `E112 — IllegalBridgeUpdate` | PARCIAL | La cláusula posicional está materializada; la procedencia desde un `Connector` permanece abierta. |
| E203 | `CyclicCompositionGraph` | `E103 — GraphCycleDetected` | CUBIERTO_OTRO_ID | Detección directa y cobertura explícita. |
| E204 | `MissingConflictOperator` | no existe `ConflictOperator` superficial completo ni comprobación de RG1 | NO_MATERIALIZADO | E204 efectivo protege otra obligación. E114 cubre sólo la unicidad en régimen `Simple`. |
| E205 | `UndeclaredRegime` | el análisis sintáctico exige `regime` y restringe sus valores | CUBIERTO_ESTRUCTURAL | La obligación se impone por forma. |
| E206 | `EdgeConnectorMismatch` | `E113 — EdgeConnectorMismatch`, con `E104/E007` para el conector | CUBIERTO_OTRO_ID | Se protege la compatibilidad contextual representada. |
| E301 | `InconsistentCounts` | sin ejecución material completa de `EvalResult.counts` | NO_MATERIALIZADO | E301 efectivo protege otra obligación. |
| E302 | `WrongThreshold` | sin comprobación autónoma del `threshold` de `EvalResult` | NO_MATERIALIZADO | E302 efectivo protege otra obligación. |
| E303 | `ClassificationPrecedenceViolation` | sin comprobación autónoma de precedencia de clasificación | NO_MATERIALIZADO | E303 efectivo es `TransitionDataMissingHorizon`. |
| E304 | `NonDeterministicGate` | `E009` impide filas ausentes, adicionales o duplicadas; `E215` exige firma posicional compatible | PARCIAL | La estructura determinista de la tabla y su firma están protegidas; no se ejecuta `GateResult.output`. |
| E305 | `UnsafeUResolution` | la forma exige `ResSpec`, contexto y mecanismo; el validador comprueba la referencia a `ResSpec` | PARCIAL | No se acredita `ResolutionRecord` completo ni toda J3.4. |
| E306 | `UntaggedSupervisable` | constructores explícitos de `Supervisable`; caso opaco mediante `E205`; contenido tipado mediante `E006` | CUBIERTO_OTRO_ID | La etiqueta y el tipo de contenido se protegen con numeración distinta. |
| E401 | `FrameMutationAttempt` | no existe operación superficial de mutación de `Frame` | CUBIERTO_ESTRUCTURAL | Inmutabilidad preservada por construcción y tipo. |
| E402 | `NonAppendOnlyTrajectory` | no existe operación superficial para borrar o reordenar entradas previas | CUBIERTO_ESTRUCTURAL | No se expone mutación retroactiva. |
| E403 | `UndeclaredHorizonEvent` | `E307 — UndeclaredHorizonEvent` | CUBIERTO_OTRO_ID | Cada tipo de suceso debe pertenecer al `Horizon` declarado. |
| E404 | `BrokenAlternation` | `E304 — TrajectoryAlternanceViolation` | CUBIERTO_OTRO_ID | Transición obligatoria en entradas no finales y prohibida en la última. |
| E405 | `EmptyTrajectory` | `E304 — TrajectoryAlternanceViolation` | CUBIERTO_OTRO_ID | Rechaza `entries` vacío. |
| E406 | `InsufficientTransitionData` | `E406 — InsufficientTransitionData` | CONVERGENTE_ID | Cierra la no vaciedad de `induced_parameters`, no toda la suficiencia reconstructiva. |
| E501 | `OpaqueJustification` | sin ejecución material completa de `QueryResult.justification` | NO_MATERIALIZADO | E501 efectivo protege serialización. |
| E502 | `QueryMutatesTrajectory` | la consulta vigente no expone mutación de trayectoria | CUBIERTO_ESTRUCTURAL | Requiere reevaluación si aparece ejecución material de consultas. |
| E503 | `StrongConclusionUnderInsufficientCoverage` | sin cierre ejecutivo completo de consulta o cobertura | NO_MATERIALIZADO | Permanece para una fase posterior. |
| E504 | `UndeclaredLossyEncoding` | `AnalyticView` y codificación con pérdida no están materializados como superficie ejecutiva completa | NO_MATERIALIZADO | Permanece para una fase posterior. |
| E505 | `IllegalCompClosure` | no existe operador `Comp` cerrado con semántica fuerte en la superficie actual | CUBIERTO_ESTRUCTURAL | `compose` no se identifica con la composición general de sucesos. |
| E506 | `AutomatedDesignDelegation` | no existe operación superficial de delegación automática del diseño | CUBIERTO_ESTRUCTURAL | Mantiene su condición de límite para capas futuras. |
| E507 | `QueryContextMismatch` | `E403 — QueryContractViolation` | CUBIERTO_OTRO_ID | Se compara `QuerySpec.query_type` con el constructor real de `QueryContext`. |

## 3. Condiciones tipadas sin identificador autónomo equivalente en la IR

### 3.1. `supervise.meta_eval : EvalResult`

J3.3 exige que `SupervisionResult.meta_eval` sea un `EvalResult` procedente de una célula de segundo orden. `E212 — SuperviseMetaNotEvalResult` materializa la precondición de tipo. `E211 — SuperviseMetaNotSupervisor` mantiene separada la procedencia desde una célula con rol `Supervisor`.

### 3.1.1. Contenido estructural de `Supervisable`

La IR v0.2 define `Supervisable` como unión etiquetada `CellTarget(EvalResult) | ComposedTarget(GateResult) | SystemTarget(CompositionGraph)`. El análisis sintáctico exige el constructor explícito mediante `E205`. El validador exige, mediante `E006`, que la referencia contenida exista y pertenezca a la clase estructural correspondiente (`EvalCmd`, `GateCmd` o `GraphDecl`).

Esta protección no acredita el determinismo de `verdict`, el efecto de `Veto` ni la semántica ejecutiva completa de la supervisión.

### 3.2. Salida de `AdmissibilityTable`

La firma `table : [Codomain] -> Codomain` exige que cada salida literal pertenezca a `output_codomain`. `E011 — TableOutputNotInCodomain` materializa esa condición. No se identifica con `E105 — IncompleteAdmissibilityTable` ni con `E106 — MissingSemanticRelation`.

### 3.3. Proyección estructural de resultados

La superficie v0.1 permite proyectar un campo desde un resultado nombrado y la IR v0.2 fija los esquemas de `EvalResult`, `GateResult`, `ResolutionRecord`, `QueryResult` y `SupervisionResult`.

- `E213 — ProjectionSourceNotResult` exige que la fuente declarada sea un productor superficial de uno de esos resultados;
- `E214 — ProjectionFieldNotFound` exige que el campo pertenezca al esquema del resultado correspondiente;
- `E006 — UndeclaredReference` conserva precedencia para una fuente inexistente.

Esta protección es estructural. No ejecuta el resultado ni calcula el valor proyectado. Tampoco habilita `Architecture`, `Frame`, `Projected` o `CriticalityResult` como fuentes.

Los campos `target`, `context` y `mechanism` están afectados por una restricción léxica independiente: son palabras reservadas y no pueden escribirse actualmente como identificadores después del punto.

### 3.4. Unicidad de `(target, position)` en régimen `Simple`

J2.3 exige que, en régimen `Simple`, cada par `(target, position)` reciba a lo sumo una arista. `E114 — SimpleRegimeConcurrency` materializa esa cláusula sobre el grafo ya comprobado en sus aristas individuales y en su aciclicidad.

No se identifica con `E204 — MissingConflictOperator`, que permanece `NO_MATERIALIZADO` porque la superficie v0.1 no representa `ConflictOperator` de forma suficiente para comprobar la concurrencia de régimen `General`.

### 3.5. Firma posicional de `gate`

`E215 — GateTableSignatureMismatch` exige que `gate([E₁, …, Eₖ], using: T)` coincida con `T.input_codomains = [K₁, …, Kₖ]` en dos dimensiones:

1. igual número de entradas;
2. igual codominio nominal en cada posición.

E006 mantiene precedencia para una referencia inexistente y E202 para una entrada que no sea `EvalResult`.

La comprobación positiva se verificó tanto con evaluaciones procedentes de `CellState` como con una evaluación procedente de `CoupledState`. La comprobación negativa posicional utiliza codominios distintos con el mismo conjunto de valores para demostrar que el orden y la identidad nominal son significativos.

E215 no ejecuta la tabla ni calcula `GateResult.output`.

## 4. Cierre de FFL-B

FFL-A permanece cerrado bajo Vía B. FFL-B queda también cerrado tras E215 y una revisión final de representabilidad.

Las obligaciones restantes identificadas no admiten otra materialización estructural honesta sin ampliar representación, semántica o ejecución. Entre ellas figuran `ConflictOperator` en régimen `General`, la procedencia completa de `CoupledState`, la suficiencia reconstructiva de `TransitionData`, la producción de `CriticalityResult`, la ejecución de `GateResult.output` y la semántica ejecutiva completa de `SupervisionResult`.

FFL-C, FFL-D y FFL-E permanecen pendientes. Ninguno se abre por el solo cierre de FFL-B.
