# Matriz de concordancia diagnóstica IR ↔ implementación del Lenguaje SV

## 1. Finalidad

Esta matriz registra el estado de concordancia entre la IR canónica v0.2, el catálogo público efectivo, la implementación, la emisión observable y la cobertura explícita de la batería de conformidad.

La clasificación por identificador se complementa con la tabla de correspondencias funcionales de `docs/calidad/`, porque un mismo identificador puede tener significados distintos y una obligación canónica puede estar protegida mediante otro código.

**Resincronización vigente:** 19/08/2026  
**Base de contraste:** IR v0.2 + catálogo efectivo + análisis sintáctico + validador + descenso a IR + batería de conformidad.

## 2. Resultado global

El balance vigente por identificador es:

- **IR v0.2:** 38 códigos;
- **catálogo efectivo:** 46 códigos;
- **coincidencia semántica por mismo identificador:** 5 (`E102`, `E104`, `E106`, `E111`, `E406`);
- **mismo identificador con significado distinto:** 20;
- **sólo IR:** 13;
- **sólo implementación:** 21.

La resincronización incorpora `E114 — SimpleRegimeConcurrency` como diagnóstico efectivo de la unicidad de `(target, position)` en régimen `Simple`. Los recuentos por identificador canónico no cambian: `E114` no posee equivalente autónomo en la IR v0.2 y no materializa `E204 — MissingConflictOperator`.

## 3. Coincidencias por mismo identificador

| Código | IR | Contrato efectivo | Emisión | Batería |
|---|---|---|---|---|
| E102 | `MissingOutputSemantics` | `MissingOutputSemantics` | directa | explícita |
| E104 | `InvalidConnectorCodomain` | `InvalidConnectorCodomain` | directa | explícita |
| E106 | `MissingSemanticRelation` | `MissingSemanticRelation` | no autónoma | no explícita |
| E111 | `UnorderedCodomain` | `UnorderedCodomain` | no autónoma | no explícita |
| E406 | `InsufficientTransitionData` | `InsufficientTransitionData` | directa | explícita |

La convergencia de `E406` se limita a la cláusula que exige `induced_parameters` no vacío y no acredita toda la suficiencia reconstructiva de J4.3.

## 4. Divergencias por mismo identificador

Persisten **20** identificadores compartidos con significado distinto. La relación detallada se conserva en `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`.

Entre las divergencias con ruta funcional alternativa ya acreditada destacan:

- `E202` canónico (`IllegalBridgeUpdate`) → cláusula posicional protegida por `E112` efectivo;
- `E206` canónico (`EdgeConnectorMismatch`) → ruta efectiva `E113`;
- `E403` canónico (`UndeclaredHorizonEvent`) → ruta efectiva `E307`.

La existencia de una ruta alternativa no autoriza a renumerar ni a declarar equivalencia donde el alcance sea parcial.

## 5. Códigos sólo presentes en la implementación

Constan **21**:

`E003`, `E004`, `E005`, `E006`, `E007`, `E008`, `E009`, `E010`, `E011`, `E112`, `E113`, `E114`, `E207`, `E208`, `E209`, `E210`, `E211`, `E212`, `E213`, `E214`, `E307`.

### 5.1. Materializaciones recientes bajo Vía B

- `E011` exige que cada salida literal de `AdmissibilityTable.table` pertenezca a `output_codomain`;
- `E112` protege la restricción posicional verificable de J2.2;
- `E113` protege la compatibilidad contextual representable de J2.3;
- `E114` protege la unicidad de `(target, position)` en régimen `Simple` exigida por J2.3; no materializa `E204` canónico ni RG1;
- `E212` exige `supervise.meta_eval : EvalResult`;
- `E211` mantiene separada la procedencia desde rol `Supervisor`;
- `E307` exige que los tipos de suceso de `TransitionData` pertenezcan al `Horizon` declarado;
- `E213` exige que la fuente de una proyección sea un productor superficial de un objeto de resultado proyectable;
- `E214` exige que el campo proyectado pertenezca al esquema canónico del resultado correspondiente;
- `E206` efectivo exige la presencia del campo obligatorio `context` en `resolve` y permanece divergente respecto del `E206` canónico `EdgeConnectorMismatch`;
- `E207` efectivo exige la presencia del campo obligatorio `mechanism` en `resolve` y continúa siendo sólo implementación.

`E213` y `E214` disponen de emisión directa y de casos explícitos. La fuente inexistente continúa gobernada por `E006`, que mantiene precedencia.

`E206` y `E207` efectivos disponen de emisión directa y de casos explícitos. Su alcance se limita al análisis sintáctico de `resolve`. No cierran `E108` canónico ni J1.6.

### 5.2. Alcanzabilidad de campos de proyección

Los esquemas canónicos contienen nombres que la superficie léxica actual reserva como palabras clave. En particular, `target`, `context` y `mechanism` no pueden escribirse actualmente como identificadores de campo tras el punto. Su presencia en el esquema reconocido por el validador no equivale a disponibilidad superficial.

El cierre E213/E214 tampoco habilita `Architecture`, `Frame`, `Projected` ni `CriticalityResult` como fuentes de proyección en la superficie vigente.

## 6. Códigos sólo presentes en la IR

Permanecen **13**:

`E107`, `E108`, `E109`, `E110`, `E305`, `E306`, `E404`, `E405`, `E502`, `E503`, `E504`, `E505`, `E506`.

Su mera presencia en la IR no ordena una materialización automática. Cada obligación deberá tratarse conforme a su representabilidad real y a la jerarquía normativa vigente.

## 7. Límites

Esta matriz clasifica la relación por identificador. La correspondencia material entre obligaciones distintas se documenta en la tabla de correspondencias funcionales.

La marca de cobertura explícita significa que existe un caso con código esperado declarado. La ejecución dinámica global se acredita en los cierres técnicos específicos y se consolidará en el bloque de evidencia correspondiente.
