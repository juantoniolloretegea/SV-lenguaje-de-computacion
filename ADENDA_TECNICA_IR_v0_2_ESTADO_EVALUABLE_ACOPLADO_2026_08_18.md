# Adenda técnica a la IR canónica v0.2 — estado evaluable simple y acoplado

**Fecha:** 18/08/2026  
**Estado:** corrección técnica acotada vigente hasta su incorporación expresa a una revisión posterior de la IR  
**Autor del corpus:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0

---

## 1. Objeto

Esta adenda corrige una estrechez material localizada en `IR_CANONICA_BIENFORMACION_SV_v0_2.md` respecto del tipo de estado admisible como fuente de `evaluate` y de `EvalResult`.

No crea un objeto doctrinal nuevo, no amplía la gramática superficial y no altera la exigencia de que `Frame.cell_states` contenga `CoupledState` bien formados. Su función exclusiva es reconciliar la IR operativa con la matemática de composición intercelular ya constituida.

## 2. Fundamento superior

El Documento I de la **Álgebra de composición intercelular del marco SV — Transmisión en serie por parámetro puente** distingue:

- el estado base `x_i^(0)` de una célula acoplable;
- el estado actualizado `x̃_i` tras las transmisiones;
- y la evaluación final de la célula receptora sobre ese estado actualizado, expresada como `y_i = χ_i(C_i[x̃_i])`.

Por tanto, una célula simple se evalúa sobre su `CellState.vector`, mientras que una célula acoplada debe poder evaluarse sobre su `CoupledState.updated_vector`. Restringir `evaluate` exclusivamente a `CellState` impediría representar directamente la evaluación posterior a una transmisión ya definida por la matemática superior.

La Gramática superficial mínima v0.1 ya recoge esta dualidad al admitir como argumento de `evaluate` un identificador de `CellState` o `CoupledState`.

## 3. Corrección normativa acotada

A efectos de la IR v0.2 y hasta que esta corrección sea incorporada a una revisión posterior, queda fijada la unión de referencia:

```text
EvaluableStateRef = CellStateRef | CoupledStateRef
```

La configuración efectiva evaluada se obtiene de forma determinista:

```text
si source_state : CellState:
    effective_vector = source_state.vector
    effective_spec   = source_state.spec

si source_state : CoupledState:
    effective_vector = source_state.updated_vector
    effective_spec   = source_state.spec.cell
```

En el segundo caso, `base_vector` permanece preservado como estado de procedencia y `updated_vector` es el vector sobre el que actúa la evaluación. No se identifican ambos vectores ni se pierde la traza de transmisión.

## 4. Efecto sobre EvalResult y J3.1

Donde la IR v0.2 declara:

```text
EvalResult.source_state : CellStateRef
```

se leerá, en el alcance de esta adenda:

```text
EvalResult.source_state : EvaluableStateRef
```

El juicio J3.1 queda precisado así:

> Un `EvalResult` es bien formado respecto de un estado evaluable si `source_state` referencia un `CellState` bien formado conforme a J2.1 o un `CoupledState` bien formado conforme a J2.2; los conteos, el umbral, la clasificación, la criticidad y los deltas se derivan del `effective_vector` y del `effective_spec` definidos en la sección 3, de manera determinista y reproducible.

Cuando `CriticalityResult` conserve una referencia al estado fuente, esa referencia queda sometida a la misma unión `EvaluableStateRef`, pues la criticidad de una célula acoplada se calcula sobre su configuración efectiva actualizada.

## 5. Efecto sobre Frame y J4.1

No se modifica la forma canónica:

```text
Frame.cell_states : [CoupledState]
```

La frase de J4.1 relativa a los resultados de evaluación se leerá como:

> todos los `EvalResult` son bien formados respecto de sus estados evaluables conforme a J3.1.

Esta adenda **no cierra** todavía una regla general de correspondencia uno a uno entre cada elemento de `Frame.cell_states` y cada `EvalResult` almacenado en el mismo `Frame`. Esa comprobación queda como deuda técnica separada para microauditoría posterior y no puede ser inferida por esta corrección.

## 6. Efecto sobre lowering y validator

`evaluate` conserva una única forma superficial:

```text
evaluate(identifier)
```

El identificador debe resolver a `CellState` o `CoupledState`. No se introduce coerción implícita, estado transitorio oculto ni sintaxis nueva.

La implementación de referencia deberá aceptar ambos tipos y rechazar cualquier tercero. Dado que el frontend actual no ejecuta materialmente `χ`, sino que valida y baja la operación a IR, esta corrección no autoriza por sí sola cálculo de conteos, clasificación, criticidad, runtime ni backend.

## 7. Exclusiones expresas

Esta adenda no:

- modifica la matemática del Documento I;
- convierte `CoupledState` en subtipo de `CellState`;
- permite `CellState` dentro de `Frame.cell_states`;
- crea nuevos códigos de error;
- modifica `TransitionData`, `Trajectory` o el régimen de sucesos;
- abre `E406` ni otros microcierres de FFL-B;
- abre runtime, backend, Rust, WASM o ejecución productiva;
- resuelve la correspondencia interna completa `Frame.cell_states ↔ Frame.eval_results`.

## 8. Prevalencia y vigencia

Por subordinación doctrinal, esta corrección prevalece únicamente sobre las formulaciones incompatibles de J3.1, `EvalResult.source_state`, `CriticalityResult.source_state` y la firma descriptiva de `evaluate` contenidas en la IR v0.2 de marzo de 2026.

El resto de la IR v0.2 permanece sin modificación. La adenda deberá ser absorbida expresamente por la siguiente revisión canónica que corresponda; hasta entonces constituye el ajuste técnico vigente para este punto concreto.

---

*Lenguaje de computación del Sistema Vectorial SV.*  
*Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351 | CC BY-NC-ND 4.0 | ISSN 2695-6411*
