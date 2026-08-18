# Acta técnica de P0-B — contrato `meta_eval` de `supervise`

**Fecha:** 18/08/2026  
**Frente:** FFL-B — P0-B de estabilización previa a nuevos microcierres  
**Estado:** APLICADO EN RAMA / PENDIENTE DE EVIDENCIA DINÁMICA  
**Autor del corpus:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**ISSN:** 2695-6411

---

## 1. Objeto

Cerrar de forma acotada la precondición de `J3.3` relativa al primer argumento de `supervise`, sin ampliar gramática, IR ni semántica ejecutiva.

La IR v0.2 establece que `SupervisionResult.meta_eval` es un `EvalResult` y que debe proceder de una célula de segundo orden. El frontend vigente comprobaba la existencia del identificador y el rol `Supervisor` únicamente cuando el objeto ya resultaba ser un `EvalCmd`; por tanto, una referencia existente de otro tipo podía atravesar esa precondición sin diagnóstico.

## 2. Efecto de P0-A

P0-A fijó que un `EvalResult` puede proceder de un estado evaluable simple o acoplado:

`CellState | CoupledState → evaluate → EvalResult`.

En consecuencia, la comprobación de procedencia de `supervise` no puede limitarse a `EvalCmd → CellState → CellSpec`. Debe resolver también:

`EvalCmd → CoupledState → CoupledSpec → CellSpec`.

No se introduce una nueva doctrina: se preserva el mismo requisito de rol `Supervisor` a través de las dos formas de estado evaluable ya autorizadas.

## 3. Decisión diagnóstica bajo Vía B

Se crea el identificador efectivo libre:

`E212 — SuperviseMetaNotEvalResult`.

Su única función es rechazar una referencia existente usada como primer argumento de `supervise` cuando no sea un `EvalResult`/`EvalCmd`.

No se reutilizan:

- `E006`, porque el identificador sí existe;
- `E202`, porque el contrato efectivo lo reserva a entradas de `gate`;
- `E211`, porque éste expresa una violación distinta: `meta_eval` sí es un `EvalResult`, pero no procede de una célula con rol `Supervisor`;
- `E306` canónico, porque éste corresponde al etiquetado del `target : Supervisable`, no al tipo de `meta_eval`.

## 4. Implementación acotada

El validator realiza, en este orden:

1. exige que `meta_eval` exista;
2. exige que su tipo efectivo sea `EvalCmd`, emitiendo `E212` en caso contrario;
3. conserva la comprobación de existencia del objetivo etiquetado;
4. resuelve la célula fuente del `EvalCmd` tanto por `CellState` como por `CoupledState`;
5. exige que la `CellSpec` subyacente tenga `role: Supervisor`, emitiendo `E211` si no se cumple.

No se endurece en este microbloque el tipado interno de cada variante de `target`. Ese frente es separable y no debe confundirse con la precondición `meta_eval` de J3.3.

## 5. Casos adversariales añadidos

- `tests/conformance/invalid/supervise_meta_no_evalresult.svp` → `E212`: la referencia existe, pero es `CellState`, no `EvalResult`.
- `tests/conformance/invalid/supervise_coupled_wrong_role.svp` → `E211`: el primer argumento sí es `EvalResult`, obtenido desde `CoupledState`, pero la `CellSpec` subyacente tiene rol `Base`.

El caso válido `tests/conformance/valid/supervise_targets.svp` continúa protegiendo el camino simple correcto con célula `Supervisor`.

## 6. Sincronización del bloque

Se actualizan en el mismo lote:

- `src/svp_errors.py`;
- `src/svp_validator.py`;
- `tests/run_conformance.py`;
- los dos casos adversariales nuevos;
- `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`;
- `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`;
- `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`;
- `docs/calidad/CROSSWALK_FUNCIONAL_DE_OBLIGACIONES_DIAGNOSTICAS_IR_V0_2_Y_FRONTEND_VIA_B_2026_08_18.md`;
- `docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`;
- esta acta.

## 7. Criterio de cierre

P0-B no se declarará cerrado por la mera aplicación del parche. Requiere:

1. inspección del diff y confirmación de radio corto;
2. `tests/run_conformance.py` en **44/44**;
3. `tests/run_cli_smoke.py` en **3/3**;
4. `tests/run_sec0_smoke.py` en **3/3**;
5. confirmación de que `supervise_meta_no_evalresult.svp` emite exactamente `E212`;
6. confirmación de que `supervise_coupled_wrong_role.svp` emite exactamente `E211`.

Hasta disponer de esa evidencia dinámica el estado permanece **aplicado en rama / pendiente de cierre**.

## 8. Continuidad

Sólo después del cierre de P0-B podrá considerarse P0 estabilizado y reabrirse la secuencia de microcierres FFL-B, empezando por la nueva auditoría mínima de `E406 — InsufficientTransitionData`.

No se abre backend, Rust, WASM, runtime, IA productiva, stdlib ni `NL → SVP`.

---

*Documento técnico subordinado del Lenguaje SV.*
