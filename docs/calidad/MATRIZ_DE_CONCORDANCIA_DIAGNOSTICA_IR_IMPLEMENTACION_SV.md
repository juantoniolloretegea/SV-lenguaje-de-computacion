# Matriz de concordancia diagnóstica IR ↔ implementación del Lenguaje SV

## 1. Finalidad

Esta matriz registra el estado de concordancia entre la IR v0.2, el catálogo público efectivo, la implementación, la emisión observable y la cobertura explícita de la batería de conformidad.

La clasificación por identificador se complementa con la tabla de correspondencias funcionales de `docs/calidad/`, porque un mismo identificador puede tener significados distintos y una obligación definida por la IR puede estar protegida mediante otro código.

**Última resincronización:** 19/08/2026  
**Base de contraste:** IR v0.2 + catálogo efectivo + análisis sintáctico + validador + descenso a IR + batería de conformidad.  
**Base funcional:** `15398f3441c80168f5d09866b0cba4e74221a6aa`.

> **Nota posterior N0-01 — 04/09/2026.** Esta matriz y su CSV conservan la fotografía histórica del 19/08/2026. N0-01 no recalcula retrospectivamente sus balances globales: fija en la sucesión v0.3 `E004 — InvalidCodomain` como emisión directa y explícita para `Codomain` vacío o con miembros repetidos. El detalle vigente y su relación con el `E101 — EmptyCodomain` histórico constan en `ACTA_TECNICA_N0_01_UNICIDAD_DE_CODOMAIN_2026_09_04.md`.

## 2. Resultado global

El balance vigente por identificador es:

- **IR v0.2:** 38 códigos;
- **catálogo efectivo:** 47 códigos;
- **coincidencia semántica por mismo identificador:** 5 (`E102`, `E104`, `E106`, `E111`, `E406`);
- **mismo identificador con significado distinto:** 20;
- **sólo IR:** 13;
- **sólo implementación:** 22.

La incorporación de `E215 — GateTableSignatureMismatch` aumenta en una unidad el catálogo efectivo y el conjunto de códigos presentes sólo en la implementación. E215 no posee equivalente autónomo por identificador en la IR v0.2; materializa una condición estructural derivada de la firma de `AdmissibilityTable` y de J3.2.

## 3. Coincidencias por mismo identificador

| Código | IR | Contrato efectivo | Emisión | Batería |
|---|---|---|---|---|
| E102 | `MissingOutputSemantics` | `MissingOutputSemantics` | directa | explícita |
| E104 | `InvalidConnectorCodomain` | `InvalidConnectorCodomain` | directa | explícita |
| E106 | `MissingSemanticRelation` | `MissingSemanticRelation` | no autónoma | no explícita |
| E111 | `UnorderedCodomain` | `UnorderedCodomain` | no autónoma | no explícita |
| E406 | `InsufficientTransitionData` | `InsufficientTransitionData` | directa | explícita |

La coincidencia de `E406` se limita a la cláusula que exige `induced_parameters` no vacío y no acredita toda la suficiencia reconstructiva de J4.3.

## 4. Divergencias por mismo identificador

Persisten **20** identificadores compartidos con significado distinto. La relación detallada se conserva en `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`.

Entre las divergencias con ruta funcional alternativa ya acreditada destacan:

- `E202` de la IR (`IllegalBridgeUpdate`) → cláusula posicional protegida por `E112` efectivo;
- `E206` de la IR (`EdgeConnectorMismatch`) → ruta efectiva `E113`;
- `E403` de la IR (`UndeclaredHorizonEvent`) → ruta efectiva `E307`.

La existencia de una ruta alternativa no autoriza a renumerar ni a declarar equivalencia donde el alcance sea parcial.

## 5. Códigos sólo presentes en la implementación

Constan **22**:

`E003`, `E004`, `E005`, `E006`, `E007`, `E008`, `E009`, `E010`, `E011`, `E112`, `E113`, `E114`, `E207`, `E208`, `E209`, `E210`, `E211`, `E212`, `E213`, `E214`, `E215`, `E307`.

### 5.1. Materializaciones acreditadas bajo Vía B

- `E011` exige que cada salida literal de `AdmissibilityTable.table` pertenezca a `output_codomain`;
- `E112` protege la restricción posicional verificable de J2.2;
- `E113` protege la compatibilidad contextual representable de J2.3;
- `E114` protege la unicidad de `(target, position)` en régimen `Simple`; no materializa `E204 — MissingConflictOperator` ni RG1;
- `E212` exige `supervise.meta_eval : EvalResult`;
- `E211` mantiene separada la procedencia desde rol `Supervisor`;
- `E307` exige que los tipos de suceso de `TransitionData` pertenezcan al `Horizon` declarado;
- `E213` exige que la fuente de una proyección sea un productor superficial de un objeto de resultado proyectable;
- `E214` exige que el campo proyectado pertenezca al esquema del resultado correspondiente;
- `E206` efectivo exige la presencia del campo obligatorio `context` en `resolve`;
- `E207` efectivo exige la presencia del campo obligatorio `mechanism` en `resolve`;
- `E215` exige que `gate` y la tabla de admisibilidad coincidan en aridad y codominio por posición.

`E215` dispone de emisión directa y de dos casos negativos específicos. E006 conserva precedencia para una referencia inexistente y E202 para una entrada existente que no sea `EvalResult`.

### 5.2. Firma de entrada de `gate`

La secuencia `gate([E₁, …, Eₖ], using: T)` debe corresponder exactamente a `T.input_codomains = [K₁, …, Kₖ]`.

La comprobación efectiva exige:

1. igual número de entradas y codominios declarados;
2. coincidencia nominal del codominio en cada posición;
3. conservación del orden declarado por la tabla.

La prueba posicional utiliza dos codominios distintos con el mismo conjunto de valores para descartar una comparación basada sólo en igualdad extensional. Una comprobación adicional confirmó también la ruta de un `EvalResult` procedente de `CoupledState`.

Este cierre no ejecuta la tabla ni determina `GateResult.output`.

### 5.3. Alcanzabilidad de campos de proyección

Los esquemas de resultados contienen nombres que la superficie léxica actual reserva como palabras clave. En particular, `target`, `context` y `mechanism` no pueden escribirse actualmente como identificadores de campo tras el punto. Su presencia en el esquema reconocido por el validador no equivale a disponibilidad superficial.

El cierre E213/E214 tampoco habilita `Architecture`, `Frame`, `Projected` ni `CriticalityResult` como fuentes de proyección en la superficie vigente.

## 6. Códigos sólo presentes en la IR

Permanecen **13**:

`E107`, `E108`, `E109`, `E110`, `E305`, `E306`, `E404`, `E405`, `E502`, `E503`, `E504`, `E505`, `E506`.

Su mera presencia en la IR no ordena una materialización automática. Cada obligación deberá tratarse conforme a su representabilidad real y a la arquitectura vigente.

## 7. Límites que permanecen fuera de FFL-B

La revisión final identifica obligaciones que no pueden resolverse mediante una validación estructural adicional sin ampliar el lenguaje o su capacidad de ejecución:

- `ConflictOperator` para concurrencia en régimen `General`;
- procedencia completa de una actualización de `CoupledState` desde un `Connector` concreto;
- suficiencia reconstructiva completa de `TransitionData`;
- producción superficial de `CriticalityResult` y validación material de `Frame.criticalities`;
- ejecución de `GateResult.output`;
- determinismo material de `SupervisionResult.verdict` y efecto de `Veto`.

Estas limitaciones quedan registradas como deuda o como capacidad todavía no representada.

## 8. Estado de FFL-B

FFL-B se declara **cerrado** con la evidencia vigente:

- conformidad: **57/57** — 9 casos válidos y 48 inválidos;
- pruebas rápidas de la interfaz de línea de órdenes: **3/3**;
- SEC-0: **3/3**.

FFL-C, FFL-D y FFL-E permanecen pendientes. Su apertura requiere decisión expresa y no se deriva automáticamente del cierre de FFL-B.
