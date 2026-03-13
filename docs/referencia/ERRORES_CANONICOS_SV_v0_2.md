# ERRORES_CANONICOS_SV_v0_2.md

## 1. Naturaleza y subordinación

Este documento tiene carácter **operativo, técnico y subordinado**.
No sustituye la autoridad doctrinal vigente del Sistema Vectorial SV ni modifica por sí mismo la IR canónica v0.2.

Su función es describir, de forma completa y revisable, el **catálogo implementativo efectivo** utilizado por el frontend de referencia en el estado actual del proyecto, conforme a la decisión:

- `docs/calidad/C1C_DECISION_REGULARIZACION_CONTRATO_DIAGNOSTICO.md`

## 2. Objeto

Documentar el contrato diagnóstico actualmente vigente en:

- `src/svp_errors.py`
- `src/svp_validator.py`
- `tests/run_conformance.py`

y ofrecer una referencia humana única para el significado efectivo de los códigos que hoy gobiernan el parser/lowering, la validación y la suite de conformidad.

## 3. Alcance

Este documento describe el catálogo implementativo **tal como existe hoy**.
No debe interpretarse como sustitución de la IR v0.2 ni como cancelación de la convergencia futura hacia la norma superior.

## 4. Resumen del estado actual

Constan actualmente:

- **37 códigos** en el catálogo implementativo efectivo;
- **2 códigos** coincidentes con la IR v0.2;
- **22 códigos** con el mismo identificador pero significado divergente respecto de la IR v0.2;
- **13 códigos** presentes en implementación y no codificados en la IR v0.2.

## 5. Regla de uso

Mientras siga vigente la regularización por Vía B:

- este documento describe el **contrato diagnóstico efectivo** del frontend de referencia;
- la IR v0.2 conserva su autoridad normativa superior;
- y toda futura convergencia deberá tratarse como acto formal separado.

## 6. Catálogo implementativo vigente

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
| E101 | `VectorLengthMismatch` | Estado | `validate` | divergente respecto de IR v0.2 | La longitud del vector no coincide con b² de la CellSpec referenciada |
| E102 | `SpecNotFound` | Estado | `validate` | divergente respecto de IR v0.2 | La CellSpec o CoupledSpec referenciada no existe |
| E103 | `GraphCycleDetected` | Estado | `validate` | divergente respecto de IR v0.2 | El grafo de composición contiene ciclos (prohibido) |
| E104 | `GraphMissingRelation` | Estado | `validate` | divergente respecto de IR v0.2 | El grafo de composición no tiene relación semántica declarada |
| E105 | `BridgePositionOutOfRange` | Estado | `validate` | divergente respecto de IR v0.2 | Posición puente fuera del rango [1, n] de la célula |
| E106 | `MissingSemanticRelation` | Estado | `validate` | coincidente con IR v0.2 | Composición sin relación semántica previa declarada |
| E111 | `UnorderedCodomain` | Estado | `validate` | coincidente con IR v0.2 | Codominio usado en compuerta sin orden documentado |
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
| E301 | `FrameMutationForbidden` | Evolución | `validate` | divergente respecto de IR v0.2 | No se permite modificar un Frame existente (inmutable por tipo) |
| E302 | `TrajectoryMutationForbidden` | Evolución | `validate` | divergente respecto de IR v0.2 | No se permite modificar, eliminar ni reordenar entradas de una Trajectory (append-only por tipo) |
| E303 | `TransitionDataMissingHorizon` | Evolución | `validate` | divergente respecto de IR v0.2 | TransitionData declarado sin referencia a Horizon |
| E304 | `TrajectoryAlternanceViolation` | Evolución | `validate` | divergente respecto de IR v0.2 | La secuencia de entradas de Trajectory no respeta las invariantes de alternancia |
| E401 | `DomainMissingHorizon` | Uso | `validate` | divergente respecto de IR v0.2 | Domain declarado sin horizonte de sucesos |
| E402 | `AgentMissingDomain` | Uso | `validate` | divergente respecto de IR v0.2 | Agent declarado sin dominio |
| E403 | `QuerySpecInvalid` | Uso | `validate` | divergente respecto de IR v0.2 | QuerySpec con tipo o alcance no reconocido |
| E501 | `SerializationNonDeterministic` | Serialización/conformidad | `lower` | divergente respecto de IR v0.2 | La serialización JSON no es determinista |
| E507 | `UCoercionDetected` | Serialización/conformidad | `validate` | divergente respecto de IR v0.2 | Coerción implícita de U detectada (prohibición constitutiva) |

## 7. Emisión observable y cobertura explícita de suite

Para evitar sobreatribución de cobertura, se distinguen aquí tres planos distintos del estado actual del frontend de referencia.

### 7.1. Códigos con emisión directa observable en el código fuente

Constan con sitio de emisión directo observable en el frontend actual, al menos, los siguientes códigos:

- `E001`
- `E002`
- `E004`
- `E005`
- `E006`
- `E010`
- `E101`
- `E103`
- `E105`
- `E202`
- `E204`
- `E205`
- `E210`
- `E211`
- `E304`

### 7.2. Códigos cubiertos explícitamente por la suite de conformidad observable

La suite de conformidad vigente cubre de forma explícita, al menos, los siguientes códigos esperados en casos inválidos:

- `E001`
- `E002`
- `E006`
- `E103`
- `E210`
- `E211`
- `E304`

### 7.3. Nota de cautela sobre catálogo, emisión y cobertura

La mera presencia de un código en `src/svp_errors.py` no implica, por sí sola, que exista hoy un sitio de emisión directo observable ni que esté cubierto por la suite.

En particular, `E507` permanece catalogado como prohibición constitutiva, pero el caso adversarial `u_coercion.svp` hoy se manifiesta observacionalmente con `E001` en la suite vigente.

## 8. Regla de continuidad

Si el catálogo implementativo cambia, este documento deberá actualizarse en el mismo bloque de trabajo que altere:

- `src/svp_errors.py`,
- `src/svp_validator.py`,
- `tests/run_conformance.py`,
- o la documentación pública correspondiente.

## 9. Vigencia

Este documento permanece vigente mientras el frontend de referencia mantenga un catálogo implementativo efectivo no plenamente reconciliado con la IR v0.2.
