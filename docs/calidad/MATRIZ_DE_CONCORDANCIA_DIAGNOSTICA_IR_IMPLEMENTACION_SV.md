# Matriz de concordancia diagnóstica IR ↔ implementación del lenguaje SV

## Finalidad

Esta matriz sirve para registrar, en una sola superficie verificable, el estado de concordancia entre:

- el régimen diagnóstico de la **IR canónica v0.2**;
- el catálogo público efectivo de errores;
- la implementación efectiva;
- la emisión observable del validador;
- la cobertura real de la batería de conformidad.

## Criterio de lectura

Cada fila deberá corresponder a una unidad diagnóstica concreta.

La clasificación mínima de situación será:

- `coincide`;
- `mismo_id_significado_distinto`;
- `solo_ir`;
- `solo_implementacion`;
- `regularizado_provisionalmente`;
- `no_acreditado`.

No se rellenarán celdas por inferencia blanda. Toda clasificación deberá apoyarse en lectura directa del artefacto correspondiente o en evidencia observable de prueba o emisión.

## Regla de uso

Esta matriz no autoriza por sí sola a modificar la IR, el catálogo público ni la implementación. Su función es **hacer visible la fractura diagnóstica** y ordenar el saneamiento en los términos fijados por el frente final del Lenguaje SV.

## Campos de la matriz

- `ID_IR`
- `Nombre_IR`
- `ID_Catalogo_Publico`
- `Nombre_Catalogo_Publico`
- `ID_Implementacion`
- `Nombre_Implementacion`
- `Emision_Observable`
- `Cobertura_Suite`
- `Situacion`
- `Tratamiento_Recomendado`
- `Observaciones`

**Fecha base de trabajo:** 24/03/2026  
**Resincronización vigente:** 19/08/2026  
**Base de contraste:** árbol vigente del repositorio + IR v0.2 + catálogo público efectivo + analizador sintáctico + validador + descenso a IR + batería de conformidad

La resincronización vigente no reescribe la historia del Bloque A. Registra el estado material observable, incluida la alcanzabilidad de `E008`, la cobertura de `E101/E105`, la convergencia vigente de `E102/E104`, la materialización parcial de `E202 — IllegalBridgeUpdate` mediante `E112`, la ruta funcional de `E206 — EdgeConnectorMismatch` mediante `E113`, la materialización de la precondición `J3.3 meta_eval : EvalResult` mediante `E212`, la ruta funcional de `E403 — UndeclaredHorizonEvent` mediante `E307`, la convergencia material de `E406 — InsufficientTransitionData` para la no-vaciedad de `induced_parameters` y la incorporación de `E011 — TableOutputNotInCodomain` para la pertenencia de las salidas literales de `AdmissibilityTable` a su `output_codomain`.

## 1. Resultado global

El balance vigente del contrato por identificador es:

- **IR v0.2:** 38 códigos
- **Catálogo efectivo / contrato público actual:** 43 códigos
- **Coincidencia semántica por mismo ID:** 5 (`E102`, `E104`, `E106`, `E111`, `E406`)
- **Mismo ID / significado distinto:** 20
- **Solo IR:** 13
- **Solo implementación:** 18

El principal problema pendiente del contrato diagnóstico no está en la inexistencia de catálogo, sino en la desalineación semántica estructural entre la norma diagnóstica superior y el contrato efectivo de la etapa frontal de referencia.

## 2. Coincidencias por mismo identificador

|Código|IR|Contrato efectivo|Emisión|Suite|Tratamiento|
|---|---|---|---|---|---|
|E102|MissingOutputSemantics|MissingOutputSemantics|si_directa|si_explicita|mantener_vigente|
|E104|InvalidConnectorCodomain|InvalidConnectorCodomain|si_directa|si_explicita|mantener_vigente|
|E106|MissingSemanticRelation|MissingSemanticRelation|no_directa|no_explicita|mantener_vigente_y_ampliar_cobertura|
|E111|UnorderedCodomain|UnorderedCodomain|no_directa|no_explicita|mantener_vigente_y_ampliar_cobertura|
|E406|InsufficientTransitionData|InsufficientTransitionData|si_directa|si_explicita|mantener_vigente|

`E102` y `E104` se incorporan aquí por estado vigente, no por retroproyección. En marzo existieron sondas en las que esos subcasos caían respectivamente a `E006` y `E008`; la implementación posterior corrigió esa situación y hoy ambos códigos disponen de emisión propia y caso explícito de conformidad.

`E406` converge exactamente en la cláusula de `J4.3` que exige que `induced_parameters` no esté vacío. Esta coincidencia no autoriza a declarar cerrada por extensión la cláusula adicional de suficiencia para reconstruir el operador inducido.

## 3. Mismo ID / significado distinto

|Código|IR|Contrato efectivo|Emisión|Suite|Tratamiento|
|---|---|---|---|---|---|
|E001|InvalidTriCoercion|InvalidTriValue|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E002|InvalidCellSize|InvalidBValue|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E101|EmptyCodomain|VectorLengthMismatch|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E103|IllegalBridgePosition|GraphCycleDetected|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E105|IncompleteAdmissibilityTable|BridgePositionOutOfRange|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E201|VectorLengthMismatch|LiteralResultForbidden|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E202|IllegalBridgeUpdate|GateInputNotEvalResult|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E203|CyclicCompositionGraph|GateMissingTable|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E204|MissingConflictOperator|QueryMissingContext|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E205|UndeclaredRegime|SuperviseOpaqueTarget|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E206|EdgeConnectorMismatch|ResolveMissingContext|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E301|InconsistentCounts|FrameMutationForbidden|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E302|WrongThreshold|TrajectoryMutationForbidden|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E303|ClassificationPrecedenceViolation|TransitionDataMissingHorizon|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E304|NonDeterministicGate|TrajectoryAlternanceViolation|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E401|FrameMutationAttempt|DomainPortContractViolation|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E402|NonAppendOnlyTrajectory|AgentDomainContractViolation|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E403|UndeclaredHorizonEvent|QueryContractViolation|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E501|OpaqueJustification|SerializationNonDeterministic|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E507|QueryContextMismatch|UCoercionDetected|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|

`E202` sigue siendo divergente por identificador: el `E202` efectivo continúa significando `GateInputNotEvalResult`. La cláusula posicional de la obligación canónica `IllegalBridgeUpdate` se protege mediante `E112`; la cláusula de procedencia por conector permanece abierta.

`E206` sigue siendo divergente por identificador porque el `E206` efectivo continúa significando `ResolveMissingContext`. Su obligación canónica `EdgeConnectorMismatch` dispone de ruta funcional bajo `E113`.

`E403` sigue siendo divergente por identificador porque el `E403` efectivo continúa significando `QueryContractViolation`. Su obligación canónica `UndeclaredHorizonEvent` dispone de ruta funcional bajo `E307`.

## 4. Presentes solo en IR

|Código|IR|Contrato efectivo|Emisión|Suite|Tratamiento|
|---|---|---|---|---|---|
|E107|InvalidTernarizerPartition|—|no_directa|no_explicita|reservar_para_ABI_y_fase_posterior|
|E108|MissingResContext|—|no_directa|no_explicita|reservar_para_ABI_y_fase_posterior|
|E109|InvalidCaptureSpec|—|no_directa|no_explicita|reservar_para_ABI_y_fase_posterior|
|E110|InvalidAdmissibilitySpec|—|no_directa|no_explicita|reservar_para_ABI_y_fase_posterior|
|E305|UnsafeUResolution|—|no_directa|no_explicita|reservar_para_ABI_y_fase_posterior|
|E306|UntaggedSupervisable|—|no_directa|no_explicita|reservar_para_ABI_y_fase_posterior|
|E404|BrokenAlternation|—|no_directa|no_explicita|reservar_para_ABI_y_fase_posterior|
|E405|EmptyTrajectory|—|no_directa|no_explicita|reservar_para_ABI_y_fase_posterior|
|E502|QueryMutatesTrajectory|—|no_directa|no_explicita|reservar_para_ABI_y_fase_posterior|
|E503|StrongConclusionUnderInsufficientCoverage|—|no_directa|no_explicita|reservar_para_ABI_y_fase_posterior|
|E504|UndeclaredLossyEncoding|—|no_directa|no_explicita|reservar_para_ABI_y_fase_posterior|
|E505|IllegalCompClosure|—|no_directa|no_explicita|reservar_para_ABI_y_fase_posterior|
|E506|AutomatedDesignDelegation|—|no_directa|no_explicita|reservar_para_ABI_y_fase_posterior|

## 5. Presentes solo en implementación

|Código|IR|Contrato efectivo|Emisión|Suite|Tratamiento|
|---|---|---|---|---|---|
|E003|—|NSquaredViolation|no_directa|no_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E004|—|EmptyCodomain|si_directa|no_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E005|—|DuplicateIdentifier|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E006|—|UndeclaredReference|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E007|—|InvalidConnectorMapping|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E008|—|ConnectorTargetNotTri|no_directa|no_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E009|—|TableInputMismatch|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E010|—|InvalidRole|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E011|—|TableOutputNotInCodomain|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E112|—|IllegalBridgeUpdate|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E113|—|EdgeConnectorMismatch|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E207|—|ResolveMissingMechanism|no_directa|no_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E208|—|ComposeMissingRelations|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E209|—|ComposeMissingPatterns|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E210|—|MaxMinForbidden|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E211|—|SuperviseMetaNotSupervisor|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E212|—|SuperviseMetaNotEvalResult|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E307|—|UndeclaredHorizonEvent|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|

## 6. Observaciones operativas

1. `E102`, `E104`, `E106`, `E111` y `E406` son hoy las cinco coincidencias semánticas por mismo identificador entre IR y contrato efectivo.
2. El grupo crítico restante es el de **20 divergencias con mismo ID**.
3. Los **18 códigos solo implementación** incluyen `E011`, `E112`, `E113`, `E212` y `E307` como materializaciones deliberadas bajo Vía B de obligaciones o precondiciones ya representadas cuyos identificadores canónicos están ocupados, no existen como código autónomo o no deben reutilizarse por semejanza.
4. Los **13 códigos solo IR** permanecen como horizonte ABI o deuda no materializada bajo su identificador canónico.
5. `E011` exige que las salidas literales de una `AdmissibilityTable` pertenezcan a su `output_codomain`; no se identifica con `E105` o `E106` canónicos y no cierra por sí solo J1.4.
6. `E112` no autoriza declarar cerrado todo `J2.2`, porque la procedencia de los valores actualizados desde un `Connector` bien formado no está representada de forma verificable en `CoupledState`.
7. `E113` dispone de punto de emisión y tres casos explícitos que separan incompatibilidades de posición, `target_position` y codominio fuente.
8. `E212` dispone de punto de emisión y caso explícito para la precondición `J3.3 meta_eval : EvalResult`; `E211` conserva la comprobación de rol `Supervisor` y se extiende al camino de estado acoplado autorizado por P0-A. `E306` permanece separado porque corresponde al etiquetado de `target : Supervisable`.
9. `E307` dispone de punto de emisión y caso explícito `transition_event_fuera_horizon.svp`; protege funcionalmente la obligación canónica `E403` sin cambiar el `E403` efectivo.
10. `E406` dispone de punto de emisión y caso explícito `transition_induced_parameters_vacios.svp`; su convergencia se limita a la no-vaciedad de `induced_parameters` y no acredita por sí sola toda la suficiencia reconstructiva exigida por `J4.3`.

## 7. Límite de esta matriz

Esta matriz clasifica la relación **por identificador**. No debe confundirse con una tabla de equivalencias funcionales entre códigos distintos.

La marca `si_explicita` en la columna de batería significa que existe un caso con código esperado declarado; no equivale por sí sola a afirmar una nueva ejecución global posterior a cada modificación. Esa evidencia dinámica se acredita en los cierres específicos y se consolidará en `FFL-C`.
