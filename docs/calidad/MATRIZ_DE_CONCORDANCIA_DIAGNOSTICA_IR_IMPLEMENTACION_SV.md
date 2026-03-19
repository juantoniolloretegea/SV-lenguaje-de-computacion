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

**Estado:** salida técnica sincronizada  
**Fecha de trabajo:** 19/03/2026  
**Base de contraste:** repo fresco del lenguaje + IR v0.2 + catálogo público efectivo + implementación + validator + suite

## 1. Resultado global

La microauditoría cerrada del Bloque A arroja, en esta salida técnica, el siguiente balance:

- **IR v0.2:** 38 códigos
- **Catálogo implementativo efectivo / contrato público actual:** 37 códigos
- **Coincidencia plena:** 2
- **Mismo ID / significado distinto:** 22
- **Solo IR:** 14
- **Solo implementación:** 13

El cuello de botella del contrato diagnóstico no está en la inexistencia de catálogo, sino en la desalineación semántica estructural entre la norma diagnóstica superior y el contrato efectivo del frontend de referencia.

## 2. Coincidencias plenas

|Código|IR|Contrato efectivo|Emisión|Suite|Tratamiento|
|---|---|---|---|---|---|
|E106|MissingSemanticRelation|MissingSemanticRelation|no_directa|no_explicita|mantener_vigente_y_ampliar_cobertura|
|E111|UnorderedCodomain|UnorderedCodomain|no_directa|no_explicita|mantener_vigente_y_ampliar_cobertura|

## 3. Mismo ID / significado distinto

|Código|IR|Contrato efectivo|Emisión|Suite|Tratamiento|
|---|---|---|---|---|---|
|E001|InvalidTriCoercion|InvalidTriValue|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E002|InvalidCellSize|InvalidBValue|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E101|EmptyCodomain|VectorLengthMismatch|si_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E102|MissingOutputSemantics|SpecNotFound|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E103|IllegalBridgePosition|GraphCycleDetected|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E104|InvalidConnectorCodomain|GraphMissingRelation|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E105|IncompleteAdmissibilityTable|BridgePositionOutOfRange|si_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E201|VectorLengthMismatch|LiteralResultForbidden|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E202|IllegalBridgeUpdate|GateInputNotEvalResult|si_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E203|CyclicCompositionGraph|GateMissingTable|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E204|MissingConflictOperator|QueryMissingContext|si_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E205|UndeclaredRegime|SuperviseOpaqueTarget|si_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E206|EdgeConnectorMismatch|ResolveMissingContext|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E301|InconsistentCounts|FrameMutationForbidden|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E302|WrongThreshold|TrajectoryMutationForbidden|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E303|ClassificationPrecedenceViolation|TransitionDataMissingHorizon|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E304|NonDeterministicGate|TrajectoryAlternanceViolation|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E401|FrameMutationAttempt|DomainMissingHorizon|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E402|NonAppendOnlyTrajectory|AgentMissingDomain|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E403|UndeclaredHorizonEvent|QuerySpecInvalid|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E501|OpaqueJustification|SerializationNonDeterministic|no_directa|no_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|
|E507|QueryContextMismatch|UCoercionDetected|si_directa|si_explicita|regularizacion_documental_inmediata_y_convergencia_posterior|

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
|E008|—|ConnectorTargetNotTri|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E009|—|TableInputMismatch|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E010|—|InvalidRole|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E207|—|ResolveMissingMechanism|no_directa|no_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E208|—|ComposeMissingRelations|no_directa|no_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E209|—|ComposeMissingPatterns|no_directa|no_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E210|—|MaxMinForbidden|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|
|E211|—|SuperviseMetaNotSupervisor|si_directa|si_explicita|mantener_como_deuda_gobernada_bajo_Via_B|

## 6. Observaciones operativas

1. **Los únicos dos códigos plenamente convergentes** son `E106` y `E111`.
2. El grupo crítico es el de **22 divergencias con mismo ID**, porque produce falsa apariencia de convergencia si solo se mira el identificador.
3. Los **13 códigos solo implementación** obligan a reconocer que el frontend opera hoy con un contrato efectivo más rico que la IR en algunas zonas.
4. Los **14 códigos solo IR** no deben tratarse como “errores fantasma”, sino como parte del **horizonte ABI semántico-diagnóstico** todavía no implementado.
5. `E507` mantiene hoy divergencia semántica respecto de la IR, pero el subcaso observable de coerción implícita de `U` ya se manifiesta explícitamente como `E507` en la suite vigente.
6. Dentro de la familia `E001–E010`, la deuda viva de alcanzabilidad superficial queda ya concentrada en `E003` y `E004`.
7. El contraste fino documentado de `E102 / E104 / E106 / E111` ya está ejecutado en la capa pública de sondas adversariales documentadas.
8. `E102` y `E104` mantienen, tras ese contraste, su condición de divergencias con mismo identificador y significado distinto.
9. `E106` y `E111` mantienen su condición de coincidencias ancla, pero siguen sin cobertura explícita de suite en la superficie v0.1.
10. El siguiente barrido legítimo del Bloque A pasa a la familia `E201–E211`.
