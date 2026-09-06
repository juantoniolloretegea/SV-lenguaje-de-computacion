# Catálogo efectivo de errores del Lenguaje SV — v0.3

## 1. Naturaleza y sucesión

Este documento describe el catálogo efectivo utilizado por la etapa frontal de referencia correspondiente a Gramática v0.2 e IR v0.3.

La versión v0.3 conserva el catálogo v0.2, añade tres diagnósticos para las correcciones de admisibilidad, revisión identificada de `U` y cierre de `Frame`, y precisa el alcance efectivo de `E004` para `Codomain`.

```text
catálogo v0.3
= catálogo v0.2
+ E110
+ E305
+ E308
```

El catálogo v0.2 se conserva como antecedente histórico y no se reescribe.

---

## 2. Estado vigente

El catálogo efectivo contiene **50 códigos**.

Los tres códigos incorporados y el código precisado en esta versión son:

| Código | Nombre | Capa | Fase | Alcance |
|---|---|---|---|---|
| E004 | `InvalidCodomain` | Definición | `validate` | `Codomain` debe ser finito, explícito, no vacío y no contener miembros repetidos |
| E110 | `InvalidAdmissibilitySpec` | Definición | `validate` | `AdmissibilitySpec` debe usar exclusivamente `Ok`, `Degraded` y `NotAdmitted`, con `parameter_id > 0` y regla no vacía |
| E305 | `UnsafeUResolution` | Resultado | `validate` | `resolve` exige una `U` constituida e identificable y una instancia compatible con su `ResSpec` |
| E308 | `FrameClosureViolation` | Evolución | `validate` | `Frame` contiene una referencia fuera de su cierre estructural o causal, una identidad duplicada o una criticidad no producible por la superficie vigente |

Todos los códigos restantes mantienen el nombre y alcance del catálogo v0.2 salvo que una especificación posterior los sustituya expresamente.

### 2.1. Precisión de `E004 — InvalidCodomain`

`E004` se emite ante un `Codomain` vacío o con uno o más miembros repetidos. La realización debe rechazar la declaración y conservar como observables el código y el nombre; no puede deduplicar ni reordenar los miembros para fabricar una entrada válida.

La tabla histórica de IR v0.2 asignó `E101 — EmptyCodomain`. Ese identificador no se reutiliza porque el catálogo efectivo ya lo asigna a `VectorLengthMismatch`. La correspondencia histórica permanece registrada como divergencia y N0-01 establece `E004` como identidad vigente, sin renumeración retroactiva.

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

La batería vigente contiene 80 casos:

```text
12 válidos
68 inválidos
```

Los diagnósticos E004, E110, E305 y E308 disponen de contraejemplos ejecutables específicos. Los casos válidos incluyen, además, codominios con miembros distintos, admisibilidad con orden permutado de los tres estados permitidos, revisión de una `U` constituida y un `Frame` con dos nodos distintos que comparten legítimamente un mismo `CellSpec`.

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
