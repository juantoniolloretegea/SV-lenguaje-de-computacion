# Matriz de concordancia diagnóstica IR ↔ implementación del lenguaje SV

## Finalidad

Esta matriz sirve para registrar, en una sola superficie auditable, el estado de concordancia entre:

- el régimen diagnóstico de la **IR canónica v0.2**,
- el catálogo canónico público de errores,
- la implementación efectiva,
- la emisión observable del validator,
- y la cobertura real de suite.

## Criterio de lectura

Cada fila deberá corresponder a una unidad diagnóstica concreta.

La clasificación mínima de situación será:

- `coincide`
- `mismo_id_significado_distinto`
- `solo_ir`
- `solo_implementacion`
- `regularizado_provisionalmente`
- `no_acreditado`

No se rellenarán celdas por inferencia blanda. Toda clasificación deberá apoyarse en lectura directa del artefacto correspondiente o en evidencia observable de suite o emisión.

## Regla de uso

Esta matriz no autoriza por sí sola a modificar la IR, el catálogo público ni la implementación. Su función es **hacer visible la fractura diagnóstica** y ordenar el saneamiento en los términos fijados por el frente final del lenguaje SV.

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
**Resincronización vigente:** 18/08/2026  
**Base de contraste:** árbol fresco del repositorio + IR v0.2 + catálogo público efectivo + parser + validator + lowering + suite

La resincronización de 18/08/2026 no reescribe la historia del Bloque A. Corrige únicamente afirmaciones que quedaron superadas por cambios posteriores ya materializados y hoy observables, en particular la alcanzabilidad de `E008`, la cobertura de `E101/E105`, la convergencia vigente de `E102/E104` y la materialización parcial de la obligación canónica `E202 — IllegalBridgeUpdate` mediante el identificador efectivo `E112`.

## 1. Resultado global

El balance vigente del contrato por identificador es:

- **IR v0.2:** 38 códigos
- **Catálogo implementativo efectivo / contrato público actual:** 38 códigos
- **Coincidencia semántica por mismo ID:** 4 (`E102`, `E104`, `E106`, `E111`)
- **Mismo ID / significado distinto:** 20
- **Solo IR:** 14
- **Solo implementación:** 14

El cuello de botella del contrato diagnóstico no está en la inexistencia de catálogo, sino en la desalineación semántica estructural entre la norma diagnóstica superior y el contrato efectivo del frontend de referencia.

## 2. Coincidencias por mismo identificador

|Código|IR|Contrato efectivo|Emisión|Suite|Tratamiento|
|---|---|---|---|---|---|
|E102|MissingOutputSemantics|MissingOutputSemantics|si_directa|si_explicita|mantener_vigente|
|E104|InvalidConnectorCodomain|InvalidConnectorCodomain|si_directa|si_explicita|mantener_vigente|
|E106|MissingSemanticRelation|MissingSemanticRelation|no_directa|no_explicita|mantener_vigente_y_ampliar_cobertura|
|E111|UnorderedCodomain|UnorderedCodomain|no_directa|no_explicita|mantener_vigente_y_ampliar_cobertura|

`E102` y `E104` se incorporan aquí por estado vigente, no por retroproyección. En marzo existieron sondas en las que esos subcasos caían respectivamente a `E006` y `E008`; la implementación posterior corrigió esa situación y hoy ambos códigos disponen de emisión propia y caso explícito de conformidad.

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

La fila `E202` sigue siendo divergente por identificador: el `E202` efectivo continúa significando `GateInputNotEvalResult`. La cláusula posicional de la obligación canónica `IllegalBridgeUpdate` se protege ahora mediante `E112`, registrado separadamente como código solo implementación; la cláusula de procedencia por conector permanece abierta.

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
|E406|InsufficientTransitionData|—|no_directa|no_explicita|reservar_para_ABI_y_fase_posterior|
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
|E112|—|IllegalBridgeUpdate|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E207|—|ResolveMissingMechanism|no_directa|no_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E208|—|ComposeMissingRelations|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E209|—|ComposeMissingPatterns|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E210|—|MaxMinForbidden|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E211|—|SuperviseMetaNotSupervisor|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|

## 6. Observaciones operativas

1. `E102`, `E104`, `E106` y `E111` son hoy las cuatro coincidencias semánticas por mismo identificador entre IR y contrato efectivo.
2. El grupo crítico restante es el de **20 divergencias con mismo ID**, porque produce falsa apariencia de convergencia si solo se mira el identificador.
3. Los **14 códigos solo implementación** obligan a reconocer que el frontend opera hoy con un contrato efectivo más rico que la IR en algunas zonas; `E112` es una materialización deliberada bajo Vía B de una subobligación canónica cuyo identificador `E202` ya está ocupado por otro significado efectivo.
4. Los **14 códigos solo IR** no deben tratarse como “errores fantasma”, sino como parte del horizonte ABI semántico-diagnóstico todavía no implementado o no expuesto bajo su identificador canónico.
5. `E507` mantiene divergencia semántica respecto de la IR, pero el subcaso observable de coerción implícita de `U` se manifiesta explícitamente como `E507` en la suite vigente.
6. Dentro de la familia `E001–E010`, la deuda viva de alcanzabilidad superficial queda concentrada en `E003`, `E004` y `E008`.
7. `E102` y `E104` ya no deben describirse como divergencias vigentes: poseen nombre, obligación material, emisión y caso explícito compatibles con la IR v0.2.
8. `E106` y `E111` mantienen coincidencia semántica, pero siguen sin caso explícito de suite en la superficie v0.1.
9. Dentro de la familia `E201–E211`, `E208` y `E209` disponen de emisión observable y casos explícitos mediante `compose_relations_vacias.svp` y `compose_patterns_vacios.svp`.
10. La familia `E301–E304` deja acreditada emisión observable y caso explícito para `E303` y `E304`; `E301` y `E302` permanecen como invariantes de tipo sin cierre superficial equivalente.
11. `E112` dispone de sitio de emisión y caso explícito `coupledstate_update_fuera_bridges.svp`; no autoriza declarar cerrado todo `J2.2`, porque la procedencia de los valores actualizados desde un `Connector` bien formado no está representada de forma verificable en `CoupledState`.

## 7. Límite de esta matriz

Esta matriz clasifica la relación **por identificador**. No debe confundirse con una tabla de equivalencias funcionales entre códigos distintos.

Por ejemplo, una obligación canónica puede estar materialmente protegida en la superficie actual por otro código implementativo (`EmptyCodomain`, `VectorLengthMismatch`, `CyclicCompositionGraph`, `BrokenAlternation` o `QueryContextMismatch` son casos que requieren lectura cruzada). La correspondencia funcional completa deberá mantenerse separada de esta clasificación por ID para no confundir convergencia semántica con convergencia numérica.

La marca `si_explicita` en la columna de suite significa que existe un caso con código esperado declarado en la batería; no equivale por sí sola a afirmar una nueva ejecución global posterior a cada modificación. Esa evidencia dinámica se acreditará en `FFL-C`.
