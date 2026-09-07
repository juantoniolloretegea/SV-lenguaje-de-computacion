# Catálogo efectivo de errores del Lenguaje SV — v0.3

## 1. Naturaleza y sucesión

Este documento describe el catálogo efectivo utilizado por la etapa frontal de referencia correspondiente a Gramática v0.2 e IR v0.3.

La versión v0.3 conserva el catálogo v0.2, añade cuatro diagnósticos para las correcciones de admisibilidad, revisión identificada de `U`, cierre de `Frame` y totalización unívoca de la semántica de `CellSpec`, y precisa el alcance efectivo de `E004` para `Codomain`.

```text
catálogo v0.3
= catálogo v0.2
+ E110
+ E115
+ E305
+ E308
```

El catálogo v0.2 se conserva como antecedente histórico y no se reescribe.

---

## 2. Estado vigente

El catálogo efectivo contiene **51 códigos**.

Los cuatro códigos incorporados y el código precisado en esta versión son:

| Código | Nombre | Capa | Fase | Alcance |
|---|---|---|---|---|
| E004 | `InvalidCodomain` | Definición | `validate` | `Codomain` debe ser finito, explícito, no vacío y no contener miembros repetidos |
| E110 | `InvalidAdmissibilitySpec` | Definición | `validate` | `AdmissibilitySpec` debe usar exclusivamente `Ok`, `Degraded` y `NotAdmitted`, con `parameter_id > 0` y regla no vacía |
| E115 | `InvalidOutputSemantics` | Estado | `validate` | `OutputSemantics` exige claves únicas; cada `CellSpec` exige además cobertura exacta de su codominio, sin claves ajenas |
| E305 | `UnsafeUResolution` | Resultado | `validate` | `resolve` exige una `U` constituida e identificable y una instancia compatible con su `ResSpec` |
| E308 | `FrameClosureViolation` | Evolución | `validate` | `Frame` contiene una referencia fuera de su cierre estructural o causal, una identidad duplicada o una criticidad no producible por la superficie vigente |

Todos los códigos restantes mantienen el nombre y alcance del catálogo v0.2 salvo que una especificación posterior los sustituya expresamente.

### 2.1. Precisión de `E004 — InvalidCodomain`

`E004` se emite ante un `Codomain` vacío o con uno o más miembros repetidos. La realización debe rechazar la declaración y conservar como observables el código y el nombre; no puede deduplicar ni reordenar los miembros para fabricar una entrada válida.

La tabla histórica de IR v0.2 asignó `E101 — EmptyCodomain`. Ese identificador no se reutiliza porque el catálogo efectivo ya lo asigna a `VectorLengthMismatch`. La correspondencia histórica permanece registrada como divergencia y N0-01 establece `E004` como identidad vigente, sin renumeración retroactiva.

---

### 2.2. Incorporación de `E115 — InvalidOutputSemantics` (N0-02)

`E115` se emite después de resolver las referencias de una `CellSpec`, si las claves de su semántica no cubren exactamente su codominio o contienen repeticiones. Identifica celda, semántica y codominio; informa claves repetidas, ausentes y ajenas. La unicidad se exige también cuando los textos repetidos coinciden. Compartir textos entre símbolos distintos y declarar las claves en otro orden siguen siendo conformes.

`E102 — MissingOutputSemantics` conserva su significado para referencia ausente o de tipo incorrecto. N0-02 no completa los diagnósticos de todo el Lenguaje ni la protección global de la proyección JSON, concretada posteriormente por N0-03 (§2.3). El nombre y código E115 se observan en Python y en `CompileError::InvalidProgram` de Rust; esa concordancia concreta no equivale a un formato diagnóstico estructurado compartido. Véase [IR v0.3 §6.2](../../IR_CANONICA_BIENFORMACION_SV_v0_3.md#relacion-n0-02) y el [acta N0-02](../arquitectura/ACTA_TECNICA_N0_02_TOTALIDAD_Y_UNICIDAD_DE_OUTPUT_SEMANTICS_2026_09_06.md).

---

### 2.3. Precisión de E115 para declaraciones no enlazadas (N0-03)

Toda `OutputSemantics` con claves repetidas produce E115, aunque ninguna `CellSpec` la use. Sin vínculo, el diagnóstico identifica sólo la semántica y las claves repetidas. La guarda complementaria se aplica después de las validaciones existentes y conserva la precedencia relacional de N0-02. No exige cobertura de un codominio ausente ni impide que mapas distintos compartan claves.

Se precisa el texto general Python de E115 para expresar ambos alcances: unicidad propia de la semántica y cobertura de cada relación constituida. Se conservan código, nombre, capa, fase y detalles relacionales; no se afirma conservación literal de ese texto descriptivo anterior. El catálogo permanece en **51 códigos**. `Connector.mapping` conserva E007 en Python y su rechazo textual previo en Rust; N0-03 no renumera esos diagnósticos ni cierra DFL-001. Fuentes: [IR §6.3](../../IR_CANONICA_BIENFORMACION_SV_v0_3.md#proyeccion-n0-03) y [acta N0-03](../arquitectura/ACTA_TECNICA_N0_03_UNICIDAD_DE_MIEMBROS_Y_ESTABILIDAD_DE_PROYECCION_JSON_2026_09_06.md).

---

### 2.4. Referencia de arquitectura de Horizon (N0-04)

`Horizon.architecture` utiliza la resolución tipada existente: `E006 — UndeclaredReference`, fase `validate`, capa efectiva 0, para referencia ausente o de tipo distinto a `GraphDecl` (representación AST de `CompositionGraph`). Se conservan nombre, mensaje base y detalles del comprobador Python. Su uso para tipo incorrecto ya está documentado en v0.2 y permanece como precisión diagnóstica pendiente en DFL-001.

Rust conserva `CompileError::InvalidProgram(String)` y los mensajes del resolutor común: `referencia no declarada: X`, `X: se esperaba CompositionGraph` o `X no es un objeto declarado` cuando X nombra una operación. No se añade un literal E006 a esos mensajes ni se declara concordancia estructurada completa. La incompatibilidad `Agent–Domain` conserva E402 y su diagnóstico textual Rust. El catálogo sigue en **51 códigos**.

La guarda se aplica sobre el programa completo después de las comprobaciones anteriores. Los negativos heredados pueden contener más de una infracción; conservar su primer diagnóstico no los convierte en controles aislados de J-H0. Los nuevos casos sí separan ausencia, tipo incorrecto y dos grafos reales distintos. Fuente: [IR §6.4](../../IR_CANONICA_BIENFORMACION_SV_v0_3.md#arquitectura-n0-04).

---

## 3. E110 — `InvalidAdmissibilitySpec`

E110 se emite cuando la declaración de admisibilidad no satisface el contrato v0.2.

Estados válidos:

```text
{Ok, Degraded, NotAdmitted}
```

No son estados de admisibilidad válidos:

```text
Failed
U
```

La separación es constitutiva:

```text
fallo técnico ≠ estado de admisibilidad ≠ Tri.U
```

La presencia de E110 no asigna por sí sola ningún valor ternario; el programa queda rechazado como no bien formado.

---

## 4. E305 — `UnsafeUResolution`

E305 protege el contrato de `ResolutionTarget` y la compatibilidad de la instancia de revisión.

Se emite, al menos, cuando:

- el estado objetivo no es evaluable;
- la posición está fuera de rango;
- la posición no contiene `U`;
- `context` no coincide con `ResSpec.context`;
- `mechanism` no coincide con `ResSpec.mechanism`.

E305 no ejecuta la revisión ni decide una clausura positiva. Su función es impedir que el frontend represente como resolución una operación cuyo objetivo o contrato de revisión no están constituidos de forma suficiente.

---

## 5. E308 — `FrameClosureViolation`

E308 protege la coherencia relacional interna de `Frame`.

Se emite, al menos, ante:

- `CoupledState` ajeno a `Frame.architecture`;
- más de un estado para el mismo nodo;
- referencia duplicada al mismo estado;
- evaluación de un estado no incluido en el `Frame`;
- evaluación material duplicada de la misma fuente;
- compuerta dependiente de evaluaciones externas;
- supervisión cuya meta-evaluación o destino queda fuera del `Frame`;
- `SystemTarget` distinto de `Frame.architecture`;
- lista `criticalities` no vacía mientras no exista productor superficial de `CriticalityResult`.

Dos nodos de arquitectura distintos pueden compartir el mismo `CellSpec`; ese supuesto es conforme si mantienen identidades de nodo distintas mediante sus respectivos `CoupledSpec`.

---

## 6. Esquema proyectable de resultados

El esquema reconocido por la validación de proyecciones queda así para los cinco productores superficiales:

| Productor | Resultado | Campos proyectables |
|---|---|---|
| `evaluate` | `EvalResult` | `source_state`, `counts`, `threshold`, `classification`, `criticality`, `deltas` |
| `gate` | `GateResult` | `inputs`, `table`, `output` |
| `resolve` | `ResolutionRecord` | `target`, `previous`, `reviewed_to`, `resolved_to`, `context_ref`, `mechanism_ref` |
| `query` | `QueryResult` | `response`, `justification`, `metadata` |
| `supervise` | `SupervisionResult` | `meta_eval`, `target`, `verdict` |

`CriticalityResult` no se incorpora a esta tabla porque la superficie vigente carece de productor constituido.

---

## 7. Divergencia conocida de E204

Permanece una divergencia histórica por identificador:

```text
IR canónica histórica:
E204 = MissingConflictOperator

catálogo efectivo del frontend:
E204 = QueryMissingContext
```

Esta divergencia no se oculta ni se corrige mediante renombrado retrospectivo.

`E114 — SimpleRegimeConcurrency` protege la unicidad de `(target, position)` en régimen `Simple`, pero no materializa `MissingConflictOperator` ni completa el authoring superficial de régimen `General`.

Por tanto:

```text
ConflictOperator / J2.3 = obligación normativa conservada
E204 canónico = no materializado por este frontend
régimen General = cobertura superficial incompleta
```

E110, E305 y E308 no constituyen una solución lateral de esa deuda.

---

## 8. Cobertura reproducible

La batería vigente contiene 91 casos:

```text
14 válidos
77 inválidos
```

Los diagnósticos E004, E110, E115, E305 y E308 disponen de contraejemplos ejecutables específicos. Los casos válidos incluyen, además, codominios con miembros distintos, admisibilidad con orden permutado de los tres estados permitidos, revisión de una `U` constituida y un `Frame` con dos nodos distintos que comparten legítimamente un mismo `CellSpec`.

La existencia de un caso diagnóstico demuestra cobertura observable de ese supuesto; no equivale por sí sola al cierre de todos los juicios normativos relacionados.

---

## 9. Regla de mantenimiento

Toda modificación posterior del catálogo efectivo deberá mantener sincronizados:

```text
código diagnóstico
punto de emisión
especificación pública aplicable
caso de conformidad cuando sea representable
```

No deberá reutilizarse un identificador existente para ocultar una divergencia semántica sin una decisión explícita de versionado.
