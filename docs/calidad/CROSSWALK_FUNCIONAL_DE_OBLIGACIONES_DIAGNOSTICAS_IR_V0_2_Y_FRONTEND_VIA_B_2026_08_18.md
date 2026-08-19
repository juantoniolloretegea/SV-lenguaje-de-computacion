# Tabla de correspondencias funcionales de obligaciones diagnósticas IR v0.2 ↔ etapa frontal bajo Vía B

**Fecha de origen:** 18/08/2026  
**Última resincronización:** 19/08/2026  
**Ámbito:** FFL-A — contrato diagnóstico  
**Autoridad superior:** `IR_CANONICA_BIENFORMACION_SV_v0_2.md`  
**Contrato efectivo subordinado:** `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md` + `src/svp_errors.py`

> **Nota de trazabilidad:** el nombre histórico de este archivo contiene `CROSSWALK` y `FRONTEND`. Se conserva para no romper las referencias registrales ya publicadas. En el contenido vivo se emplea la denominación española **tabla de correspondencias funcionales** y **etapa frontal del compilador**.

## 1. Objeto

La matriz de concordancia vigente clasifica la relación entre IR y catálogo **por identificador**. Esa lectura no basta para determinar si una obligación canónica está protegida mediante otro código, queda impedida por la propia estructura de la superficie o permanece realmente sin materializar.

Esta tabla añade ese segundo plano sin renumerar el catálogo y sin convertir la implementación en autoridad normativa. No adopta la Vía A. Su finalidad es localizar con precisión la deuda que permanece bajo la Vía B.

## 2. Estados utilizados

- `CONVERGENTE_ID`: misma obligación y mismo identificador.
- `CUBIERTO_OTRO_ID`: la obligación material está protegida por otro diagnóstico efectivo.
- `CUBIERTO_ESTRUCTURAL`: la representación o la gramática vigente impiden el estado inválido sin emitir el identificador canónico.
- `PARCIAL`: existe una protección material incompleta respecto del juicio canónico.
- `NO_MATERIALIZADO`: no se acredita una comprobación suficiente de la obligación canónica.

La ausencia del identificador canónico no equivale por sí sola a ausencia de protección. La presencia de un código con el mismo número tampoco acredita que proteja la misma obligación.

## 3. Correspondencias por obligación canónica

| IR | Obligación canónica | Ruta efectiva vigente | Estado funcional | Observación |
|---|---|---|---|---|
| E001 | `InvalidTriCoercion` | `E507` para coerción implícita de `U`; `E001` efectivo para literal Tri inválido | PARCIAL | La protección observable de `U` existe, pero el E001 efectivo no representa la misma obligación canónica general. |
| E002 | `InvalidCellSize` | `E002` valida `b >= 3`; el descenso a IR deriva `n = b²` | CUBIERTO_ESTRUCTURAL | La superficie no declara `n` independientemente; la igualdad `n=b²` se obtiene por construcción. |
| E101 | `EmptyCodomain` | `E004 — EmptyCodomain` | CUBIERTO_OTRO_ID | Existe lugar de validación, aunque la gramática publicada no ofrece hoy un caso superficial vacío explícito. |
| E102 | `MissingOutputSemantics` | `E102 — MissingOutputSemantics` | CONVERGENTE_ID | Emisión directa y cobertura explícita vigentes. |
| E103 | `IllegalBridgePosition` | `E105 — BridgePositionOutOfRange` | CUBIERTO_OTRO_ID | Emisión directa y cobertura explícita. |
| E104 | `InvalidConnectorCodomain` | `E104 — InvalidConnectorCodomain` | CONVERGENTE_ID | Emisión directa y cobertura explícita. |
| E105 | `IncompleteAdmissibilityTable` | `E009 — TableInputMismatch` | CUBIERTO_OTRO_ID | Comprueba faltantes, elementos adicionales y duplicados sobre el producto cartesiano. |
| E106 | `MissingSemanticRelation` | `E106` existe; la forma exige `relation` y una referencia no declarada produce `E006` | CONVERGENTE_ID | Coincidencia semántica del catálogo, sin emisión superficial autónoma de E106. |
| E107 | `InvalidTernarizerPartition` | `E401` comprueba únicamente la presencia no vacía de las tres referencias de partición | PARCIAL | No se acredita cobertura, disjunción ni ausencia de solapamiento de las particiones canónicas. |
| E108 | `MissingResContext` | el analizador sintáctico exige `context` y `mechanism`; `E206/E207` existen en catálogo pero no tienen emisión autónoma acreditada | PARCIAL | La forma sintáctica queda protegida; no queda materializado todo J1.6 bajo diagnóstico canónico. |
| E109 | `InvalidCaptureSpec` | `E401` comprueba `parameter_id`, `observation_space` y `Bottom` | PARCIAL | No se acredita todo el tipado o correspondencia exigidos por J1.7. |
| E110 | `InvalidAdmissibilitySpec` | `E401` comprueba `parameter_id`, conjunto de estados y presencia de `rule` | PARCIAL | No se acredita el determinismo material de `rule`. |
| E111 | `UnorderedCodomain` | `E111` existe; `max/min` están prohibidos en v0.1 mediante `E210` | CUBIERTO_ESTRUCTURAL | La condición canónica no es alcanzable en la superficie actual de `max/min`; E111 permanece como reserva convergente. |
| E201 | `VectorLengthMismatch` | `E101 — VectorLengthMismatch` | CUBIERTO_OTRO_ID | Se comprueba en `CellState` y en ambos vectores de `CoupledState`. |
| E202 | `IllegalBridgeUpdate` | `E112 — IllegalBridgeUpdate` compara `base_vector` y `updated_vector` y rechaza cambios fuera del `BridgeSet` | PARCIAL | La cláusula posicional de J2.2 queda materializada bajo un identificador efectivo libre. El `E202` efectivo sigue siendo `GateInputNotEvalResult`; no se acredita todavía que los valores modificados procedan de un `Connector` bien formado. |
| E203 | `CyclicCompositionGraph` | `E103 — GraphCycleDetected` | CUBIERTO_OTRO_ID | Detección directa de ciclo y cobertura explícita. |
| E204 | `MissingConflictOperator` | `Graph.conflicts` es opcional y el validador no comprueba concurrencia por `(target, position)` | NO_MATERIALIZADO | El E204 efectivo protege otra obligación (`QueryMissingContext`). |
| E205 | `UndeclaredRegime` | el analizador sintáctico exige `regime` y sólo acepta `Simple` o `General`; el fallo de forma produce `E001` efectivo | CUBIERTO_ESTRUCTURAL | La obligación está impuesta por gramática, no por E205 canónico. |
| E206 | `EdgeConnectorMismatch` | `E113 — EdgeConnectorMismatch` comprueba la pertenencia de `Edge.position` al `BridgeSet` del destino, la igualdad con `Connector.target_position` y la coincidencia de `Connector.source_codomain` con el codominio de la célula fuente; `E104/E007` gobiernan la correspondencia Tri y su completitud | CUBIERTO_OTRO_ID | La obligación contextual de J2.3 queda protegida sin reutilizar el `E206` efectivo, que sigue significando `ResolveMissingContext`. |
| E301 | `InconsistentCounts` | no existe materialización ejecutiva completa de `EvalResult.counts` respecto de J3.1 | NO_MATERIALIZADO | El E301 efectivo es `FrameMutationForbidden`. |
| E302 | `WrongThreshold` | no existe comprobación diagnóstica autónoma del `threshold` de `EvalResult` | NO_MATERIALIZADO | El E302 efectivo es `TrajectoryMutationForbidden`. |
| E303 | `ClassificationPrecedenceViolation` | no existe comprobación autónoma de precedencia de clasificación | NO_MATERIALIZADO | El E303 efectivo y observable es `TransitionDataMissingHorizon`; no debe confundirse con E303 canónico. |
| E304 | `NonDeterministicGate` | `E009` impide filas ausentes, adicionales o duplicadas en `AdmissibilityTable`; `gate` se traduce a una operación IR sin ejecución material de `GateResult.output` en la etapa frontal | PARCIAL | La representación impide dos filas distintas para una misma entrada, pero J3.2 no queda plenamente ejercido mientras no exista evaluación ejecutiva del resultado de compuerta. |
| E305 | `UnsafeUResolution` | el analizador sintáctico exige `ResSpec`, contexto y mecanismo; el validador sólo verifica la referencia a `ResSpec` | PARCIAL | No se acredita todavía `ResolutionRecord` completo ni toda J3.4. |
| E306 | `UntaggedSupervisable` | el analizador sintáctico exige constructores de `Supervisable`; el caso opaco se emite como `E205` efectivo | CUBIERTO_OTRO_ID | La etiqueta estructural queda exigida con numeración efectiva distinta. |
| E401 | `FrameMutationAttempt` | no existe operación superficial de mutación de `Frame`; `E301` efectivo conserva el invariante en catálogo | CUBIERTO_ESTRUCTURAL | La inmutabilidad se preserva principalmente por construcción y tipo, no por emisión autónoma. |
| E402 | `NonAppendOnlyTrajectory` | no existe operación superficial para borrar o reordenar entradas previas; `E302` efectivo conserva el invariante en catálogo | CUBIERTO_ESTRUCTURAL | La trayectoria se construye y valida, pero no se expone mutación retroactiva. |
| E403 | `UndeclaredHorizonEvent` | `E307 — UndeclaredHorizonEvent` comprueba que cada `TransitionData.events[].event_type` pertenezca a `Horizon.events` del `horizon_ref` | CUBIERTO_OTRO_ID | La obligación material de J4.3 queda protegida sin reutilizar `E403`, que en el catálogo efectivo mantiene `QueryContractViolation`. |
| E404 | `BrokenAlternation` | `E304 — TrajectoryAlternanceViolation` | CUBIERTO_OTRO_ID | Valida transición obligatoria en entradas no finales y prohibida en la última. |
| E405 | `EmptyTrajectory` | `E304 — TrajectoryAlternanceViolation` | CUBIERTO_OTRO_ID | El validador rechaza `entries` vacío. |
| E406 | `InsufficientTransitionData` | `E406 — InsufficientTransitionData` rechaza `induced_parameters` vacío | CONVERGENTE_ID | Cierra exactamente la obligación diagnóstica de lista no vacía; no acredita toda la suficiencia reconstructiva de J4.3. |
| E501 | `OpaqueJustification` | no existe ejecución material completa de `QueryResult.justification` | NO_MATERIALIZADO | El E501 efectivo es `SerializationNonDeterministic`. |
| E502 | `QueryMutatesTrajectory` | la superficie de consulta vigente no expone una operación de mutación de trayectoria | CUBIERTO_ESTRUCTURAL | Deberá reevaluarse cuando exista ejecución material de consultas. |
| E503 | `StrongConclusionUnderInsufficientCoverage` | no existe todavía cierre ejecutivo completo de consulta o cobertura | NO_MATERIALIZADO | Permanece en el horizonte ABI. |
| E504 | `UndeclaredLossyEncoding` | `AnalyticView` y la codificación con pérdida no están materializados como superficie ejecutiva completa | NO_MATERIALIZADO | Permanece en el horizonte ABI. |
| E505 | `IllegalCompClosure` | la superficie actual no expone un operador canónico `Comp` cerrado; `compose` se traduce a `Architecture` bajo relaciones y patrones declarados | CUBIERTO_ESTRUCTURAL | No autoriza a identificar `compose` con la composición general de sucesos. |
| E506 | `AutomatedDesignDelegation` | no existe en la superficie actual una operación que delegue la modificación del diseño a un proceso automático | CUBIERTO_ESTRUCTURAL | Sigue siendo límite normativo para futuras capas. |
| E507 | `QueryContextMismatch` | `E403 — QueryContractViolation` | CUBIERTO_OTRO_ID | El validador compara `QuerySpec.query_type` con el constructor real de `QueryContext`; existe cobertura explícita. |

### 3.1. Precondición de J3.3 sin código canónico autónomo

`J3.3` exige que `SupervisionResult.meta_eval` sea un `EvalResult` procedente de una célula de segundo orden. La tabla de errores de la IR v0.2 no asigna un código autónomo a la violación de tipo del primer argumento de `supervise`: `E306` se refiere al etiquetado del `target : Supervisable`.

P0-B materializa esa precondición mediante `E212 — SuperviseMetaNotEvalResult`. La comprobación posterior de procedencia desde una célula con rol `Supervisor` permanece bajo `E211 — SuperviseMetaNotSupervisor` y cubre tanto la ruta simple `EvalCmd → CellState → CellSpec` como la ruta acoplada `EvalCmd → CoupledState → CoupledSpec → CellSpec` recibida por P0-A. Esta precisión no modifica ni absorbe `E306`.

### 3.2. Obligación tipada de `AdmissibilityTable` sin código canónico autónomo

La definición de `AdmissibilityTable` en la IR v0.2 establece `table : [Codomain] -> Codomain`. Por tanto, cada salida literal de una fila debe pertenecer al `output_codomain` declarado. J1.4 añade obligaciones de completitud, determinismo, asimetría documentada cuando exista y relación semántica previa, pero la tabla canónica de errores no asigna un identificador autónomo a la pertenencia de la salida al codominio declarado.

`E011 — TableOutputNotInCodomain` materializa únicamente esa condición de tipado bajo Vía B. No se identifica con `E105 — IncompleteAdmissibilityTable`, que continúa referido a la completitud, ni con `E106 — MissingSemanticRelation`. La batería distingue `E011` de `E009 — TableInputMismatch` mediante casos independientes.

## 4. Riesgos de interpretación

La divergencia de numeración no debe ocultar cuatro situaciones distintas:

1. obligaciones canónicas ya protegidas mediante otro identificador;
2. obligaciones impedidas por la superficie vigente;
3. obligaciones parcialmente protegidas, cuya estructura actual no permite atribuir cierre completo;
4. deuda funcional todavía no materializada.

Las situaciones tercera y cuarta deben permanecer visibles como deuda de implementación o de ABI hasta que exista cierre acreditado.

`E112` materializa únicamente la cláusula posicional de la obligación canónica `E202`; no demuestra la procedencia de cada actualización desde un `Connector` bien formado.

`E113` protege la obligación canónica `E206` en el alcance de la compatibilidad contextual representada por la superficie vigente.

`E212` materializa una precondición literal de `J3.3`; permanece separado de `E211`, que protege la procedencia desde rol `Supervisor`, y de `E306`, que protege el etiquetado del objeto supervisado.

`E307` protege la obligación canónica `E403` mediante una comprobación directa de pertenencia entre `TransitionData.events` y `Horizon.events`.

`E406` constituye una convergencia exacta por identificador para la cláusula de `J4.3` que exige al menos un elemento en `induced_parameters`. Esa convergencia no se extiende a la suficiencia para reconstruir el operador inducido.

`E011` protege la pertenencia de las salidas literales de `AdmissibilityTable` a `output_codomain`. No constituye por sí solo cierre completo de J1.4 ni prueba de ejecución material de `GateResult`.

## 5. Efecto sobre FFL-A y FFL-B

FFL-A no exigía implementar de inmediato todos los diagnósticos de la IR. Su criterio de cierre admitió deuda **localizada, gobernada y explicitada**.

Esta tabla permite distinguir qué divergencias son numéricas, cuáles dependen de la alcanzabilidad de la superficie y cuáles representan una obligación canónica aún no materializada.

Los cierres posteriores de FFL-B deberán actualizar esta tabla cuando cambie materialmente la protección de una obligación canónica o de una condición tipada ya fijada por la IR. Ninguna actualización podrá convertir una protección parcial en cierre completo sin evidencia específica.
