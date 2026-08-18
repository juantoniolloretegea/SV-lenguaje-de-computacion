# Crosswalk funcional de obligaciones diagnósticas IR v0.2 ↔ frontend bajo Vía B

**Fecha:** 18/08/2026  
**Ámbito:** FFL-A — contrato diagnóstico  
**Autoridad superior:** `IR_CANONICA_BIENFORMACION_SV_v0_2.md`  
**Contrato efectivo subordinado:** `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md` + `src/svp_errors.py`

## 1. Objeto

La matriz de concordancia vigente clasifica correctamente la relación **por identificador**, pero esa lectura no basta para saber si una obligación canónica de la IR está protegida hoy bajo otro código, por una restricción estructural de la superficie o si permanece realmente sin materializar.

Este crosswalk añade ese segundo plano sin renumerar el catálogo y sin convertir la implementación en autoridad normativa.

No adopta la Vía A. Su finalidad es localizar con precisión la deuda que queda bajo la Vía B.

## 2. Estados utilizados

- `CONVERGENTE_ID`: misma obligación y mismo identificador.
- `CUBIERTO_OTRO_ID`: la obligación material está protegida por otro diagnóstico efectivo.
- `CUBIERTO_ESTRUCTURAL`: la representación o gramática vigente impide el estado inválido sin emitir el identificador canónico.
- `PARCIAL`: existe protección material incompleta respecto del juicio canónico.
- `NO_MATERIALIZADO`: no se acredita hoy comprobación suficiente de la obligación canónica.

La ausencia del identificador canónico no equivale por sí sola a ausencia de protección; y la mera presencia de un código con el mismo número tampoco acredita que proteja la misma obligación.

## 3. Crosswalk por obligación canónica

| IR | Obligación canónica | Ruta efectiva vigente | Estado funcional | Observación |
|---|---|---|---|---|
| E001 | `InvalidTriCoercion` | `E507` para coerción implícita de `U`; `E001` efectivo para literal Tri inválido | PARCIAL | La protección observable de `U` existe, pero el E001 efectivo no representa la misma obligación canónica general. |
| E002 | `InvalidCellSize` | `E002` valida `b >= 3`; lowering deriva `n = b²` | CUBIERTO_ESTRUCTURAL | La superficie no declara `n` independientemente; la igualdad `n=b²` se obtiene por construcción. |
| E101 | `EmptyCodomain` | `E004 — EmptyCodomain` | CUBIERTO_OTRO_ID | Existe sitio de validación, aunque la gramática publicada no ofrece hoy un caso superficial vacío explícito. |
| E102 | `MissingOutputSemantics` | `E102 — MissingOutputSemantics` | CONVERGENTE_ID | Emisión directa y cobertura explícita vigentes. |
| E103 | `IllegalBridgePosition` | `E105 — BridgePositionOutOfRange` | CUBIERTO_OTRO_ID | Emisión directa y cobertura explícita. |
| E104 | `InvalidConnectorCodomain` | `E104 — InvalidConnectorCodomain` | CONVERGENTE_ID | Emisión directa y cobertura explícita. |
| E105 | `IncompleteAdmissibilityTable` | `E009 — TableInputMismatch` | CUBIERTO_OTRO_ID | Comprueba faltantes, extras y duplicados sobre el producto cartesiano. |
| E106 | `MissingSemanticRelation` | `E106` existe; la forma exige `relation` y la referencia no declarada cae en `E006` | CONVERGENTE_ID | Coincidencia semántica del catálogo, sin emisión superficial autónoma de E106. |
| E107 | `InvalidTernarizerPartition` | `E401` comprueba únicamente presencia no vacía de las tres referencias de partición | PARCIAL | No se acredita cobertura, disjunción ni ausencia de solapamiento de las particiones canónicas. |
| E108 | `MissingResContext` | parser exige campos `context` y `mechanism`; `E206/E207` existen en catálogo pero no tienen emisión autónoma acreditada | PARCIAL | La forma sintáctica queda protegida; no queda materializado todo J1.6 bajo diagnóstico canónico. |
| E109 | `InvalidCaptureSpec` | `E401` comprueba `parameter_id`, `observation_space` y `Bottom` | PARCIAL | No se acredita todo el tipado/mapeo exigido por J1.7. |
| E110 | `InvalidAdmissibilitySpec` | `E401` comprueba `parameter_id`, conjunto de estados y presencia de `rule` | PARCIAL | No se acredita determinismo material de `rule`. |
| E111 | `UnorderedCodomain` | `E111` existe; `max/min` están prohibidos en v0.1 mediante `E210` | CUBIERTO_ESTRUCTURAL | La condición canónica no es alcanzable en la superficie actual de `max/min`; E111 permanece reserva convergente. |
| E201 | `VectorLengthMismatch` | `E101 — VectorLengthMismatch` | CUBIERTO_OTRO_ID | Se comprueba en `CellState` y en ambos vectores de `CoupledState`. |
| E202 | `IllegalBridgeUpdate` | no se compara hoy `base_vector` frente a `updated_vector` fuera de posiciones puente | NO_MATERIALIZADO | El E202 efectivo protege otra obligación (`GateInputNotEvalResult`). |
| E203 | `CyclicCompositionGraph` | `E103 — GraphCycleDetected` | CUBIERTO_OTRO_ID | Detección directa de ciclo y cobertura explícita. |
| E204 | `MissingConflictOperator` | `Graph.conflicts` es opcional y el validator no comprueba concurrencia por `(target, position)` | NO_MATERIALIZADO | El E204 efectivo protege otra obligación (`QueryMissingContext`). |
| E205 | `UndeclaredRegime` | parser exige `regime` y solo acepta `Simple` o `General`; fallo de forma cae en `E001` efectivo | CUBIERTO_ESTRUCTURAL | La obligación está impuesta por gramática, no por E205 canónico. |
| E206 | `EdgeConnectorMismatch` | el validator actual no acredita compatibilidad del `connector` de cada `Edge` | NO_MATERIALIZADO | El E206 efectivo protege otra obligación (`ResolveMissingContext`). |
| E301 | `InconsistentCounts` | no existe hoy materialización ejecutiva completa de `EvalResult.counts` contra J3.1 | NO_MATERIALIZADO | El E301 efectivo es `FrameMutationForbidden`. |
| E302 | `WrongThreshold` | no existe hoy comprobación diagnóstica autónoma del `threshold` de `EvalResult` | NO_MATERIALIZADO | El E302 efectivo es `TrajectoryMutationForbidden`. |
| E303 | `ClassificationPrecedenceViolation` | no existe hoy comprobación autónoma de precedencia de clasificación | NO_MATERIALIZADO | El E303 efectivo y observable es `TransitionDataMissingHorizon`; no debe confundirse con E303 canónico. |
| E304 | `NonDeterministicGate` | `E009` impide filas ausentes, extra o duplicadas en `AdmissibilityTable`; `gate` se baja hoy a una operación IR, sin ejecución material de `GateResult.output` en el frontend | PARCIAL | La representación de la tabla impide dos filas distintas para una misma entrada, pero J3.3 no queda plenamente ejercido mientras no exista evaluación ejecutiva del resultado de compuerta. |
| E305 | `UnsafeUResolution` | parser exige `ResSpec`, contexto y mecanismo; validator solo verifica la referencia a `ResSpec` | PARCIAL | No se acredita todavía `ResolutionRecord` completo ni toda J3.4. |
| E306 | `UntaggedSupervisable` | parser exige constructores de `Supervisable`; el caso opaco se emite como `E205` efectivo | CUBIERTO_OTRO_ID | La etiqueta estructural queda exigida con numeración efectiva distinta. |
| E401 | `FrameMutationAttempt` | no existe operación superficial de mutación de `Frame`; `E301` efectivo conserva el invariante en catálogo | CUBIERTO_ESTRUCTURAL | La inmutabilidad se preserva hoy principalmente por construcción/tipo, no por disparo autónomo. |
| E402 | `NonAppendOnlyTrajectory` | no existe operación superficial para borrar o reordenar entradas previas; `E302` efectivo conserva el invariante en catálogo | CUBIERTO_ESTRUCTURAL | La trayectoria se construye y valida, pero no se expone mutación retroactiva. |
| E403 | `UndeclaredHorizonEvent` | `TransitionData.horizon_ref` se valida con `E303` efectivo, pero no se comprueba que cada `event_type` pertenezca a `Horizon.events` | NO_MATERIALIZADO | Es una deuda funcional concreta de N3; el E403 efectivo pertenece a consultas. |
| E404 | `BrokenAlternation` | `E304 — TrajectoryAlternanceViolation` | CUBIERTO_OTRO_ID | Valida transición obligatoria en entradas no finales y prohibida en la última. |
| E405 | `EmptyTrajectory` | `E304 — TrajectoryAlternanceViolation` | CUBIERTO_OTRO_ID | El validator rechaza `entries` vacío. |
| E406 | `InsufficientTransitionData` | parser admite lista vacía de `induced_parameters` y el validator no la rechaza | NO_MATERIALIZADO | Deuda funcional concreta de N3. |
| E501 | `OpaqueJustification` | no existe ejecución material completa de `QueryResult.justification` | NO_MATERIALIZADO | El E501 efectivo es `SerializationNonDeterministic`. |
| E502 | `QueryMutatesTrajectory` | la superficie de consulta vigente no expone una operación de mutación de trayectoria | CUBIERTO_ESTRUCTURAL | Debe reevaluarse cuando exista ejecución material de consultas. |
| E503 | `StrongConclusionUnderInsufficientCoverage` | no existe todavía cierre ejecutivo completo de consulta/cobertura | NO_MATERIALIZADO | Permanece en el horizonte ABI. |
| E504 | `UndeclaredLossyEncoding` | `AnalyticView`/codificación con pérdida no está materializada como superficie ejecutiva completa | NO_MATERIALIZADO | Permanece en el horizonte ABI. |
| E505 | `IllegalCompClosure` | la superficie actual no expone un operador canónico `Comp` cerrado; `compose` baja a `Architecture` bajo relaciones/patrones declarados | CUBIERTO_ESTRUCTURAL | No autoriza inferir equivalencia entre `compose` implementativo y composición general de sucesos. |
| E506 | `AutomatedDesignDelegation` | no existe en la superficie actual una operación que delegue modificación de diseño a un proceso automático | CUBIERTO_ESTRUCTURAL | Sigue siendo límite normativo para futuras capas. |
| E507 | `QueryContextMismatch` | `E403 — QueryContractViolation` | CUBIERTO_OTRO_ID | El validator compara `QuerySpec.query_type` con el constructor real de `QueryContext`; existe cobertura explícita. |

## 4. Hallazgos de mayor riesgo

La divergencia por numeración no debe ocultar cuatro clases distintas de situación:

1. **obligaciones canónicas ya protegidas con otro ID**, por ejemplo `E101→E004`, `E103→E105`, `E105→E009`, `E201→E101`, `E203→E103`, `E404/E405→E304` y `E507→E403`;
2. **obligaciones precluidas por la superficie vigente**, como mutación de `Frame`, borrado/reordenación de trayectoria o `max/min` sobre codominio no ordenado;
3. **obligaciones parcialmente protegidas**, donde la estructura actual cubre una parte pero no permite atribuir cierre completo del juicio canónico, como `E107–E110`, `E304` y `E305`;
4. **deuda funcional real todavía no materializada**, destacando `E202`, `E204`, `E206`, `E301–E303` canónicos, `E403` canónico y `E406`, además de obligaciones posteriores de consulta/ABI.

Las clases tercera y cuarta no pueden desaparecer por documentación. Deben permanecer visibles como deuda de implementación o ABI para los bloques posteriores correspondientes.

## 5. Efecto sobre FFL-A

FFL-A no exige implementar de inmediato todos los diagnósticos de la IR. Su criterio de cierre admite una deuda **localizada, gobernada y explicitada**.

Este crosswalk permite distinguir qué divergencias son puramente numéricas, cuáles son de alcanzabilidad y cuáles representan una obligación canónica aún no materializada. Por tanto, puede servir como evidencia de gobierno del contrato diagnóstico bajo Vía B, pero **no cierra por sí solo FFL-A ni transfiere la deuda funcional a estado de resuelta**.

## 6. Regla de continuidad

Toda modificación futura de un diagnóstico que afecte a una obligación canónica deberá actualizar simultáneamente:

`IR/juicio afectado → catálogo efectivo → sitio de emisión → suite → matriz por ID → este crosswalk funcional`.

No se renumerará por semejanza ni se declarará equivalencia sin contraste material.
