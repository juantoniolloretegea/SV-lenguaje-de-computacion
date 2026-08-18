# Acta técnica de P0-B — contrato `meta_eval` de `supervise`

**Fecha:** 18/08/2026  
**Frente:** FFL-B — P0-B de estabilización  
**Estado:** CERRADO  
**Autor del corpus:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**ISSN:** 2695-6411

## 1. Objeto

Esta acta registra el cierre acotado de la precondición de `J3.3` relativa al primer argumento de `supervise`, sin ampliar gramática, IR ni semántica ejecutiva.

La IR v0.2 establece que `SupervisionResult.meta_eval` debe ser un `EvalResult` y proceder de una célula de segundo orden. El validador comprobaba la existencia del identificador y el rol `Supervisor` únicamente cuando el objeto ya era un `EvalCmd`; por ello, una referencia existente de otro tipo podía superar indebidamente esa precondición.

## 2. Relación con P0-A

P0-A fijó que un `EvalResult` puede proceder de un estado evaluable simple o acoplado:

`CellState | CoupledState → evaluate → EvalResult`.

En consecuencia, la comprobación de procedencia de `supervise` debe contemplar tanto:

`EvalCmd → CellState → CellSpec`

como:

`EvalCmd → CoupledState → CoupledSpec → CellSpec`.

No se introduce doctrina nueva. Se aplica el mismo requisito de rol `Supervisor` a las dos formas de estado evaluable ya autorizadas.

## 3. Decisión diagnóstica bajo Vía B

Se incorpora el identificador efectivo libre:

`E212 — SuperviseMetaNotEvalResult`.

Su función exclusiva es rechazar una referencia existente utilizada como primer argumento de `supervise` cuando no sea un `EvalResult` representado por `EvalCmd`.

No se reutilizan otros códigos porque protegen obligaciones distintas:

- `E006` corresponde a una referencia no declarada;
- `E202` está reservado en el contrato efectivo a entradas de `gate`;
- `E211` expresa que `meta_eval` sí es un `EvalResult`, pero no procede de una célula con rol `Supervisor`;
- `E306` canónico corresponde al etiquetado del `target : Supervisable`.

## 4. Implementación acotada

El validador aplica el siguiente orden:

1. comprueba que `meta_eval` exista;
2. exige que su tipo efectivo sea `EvalCmd`, con `E212` en caso contrario;
3. conserva la comprobación de existencia del objetivo etiquetado;
4. resuelve la célula fuente tanto por `CellState` como por `CoupledState`;
5. exige que la `CellSpec` subyacente tenga `role: Supervisor`, con `E211` en caso contrario.

El tipado interno de cada variante de `target` queda fuera de esta unidad técnica porque pertenece a un juicio distinto.

## 5. Pruebas incorporadas

Se añaden dos casos negativos específicos:

- `tests/conformance/invalid/supervise_meta_no_evalresult.svp` → `E212`: la referencia existe, pero es `CellState`, no `EvalResult`;
- `tests/conformance/invalid/supervise_coupled_wrong_role.svp` → `E211`: el primer argumento es `EvalResult`, obtenido desde `CoupledState`, pero la `CellSpec` subyacente tiene rol `Base`.

El caso válido `tests/conformance/valid/supervise_targets.svp` continúa protegiendo el camino simple correcto con célula `Supervisor`.

## 6. Sincronización del cierre

En el mismo cierre se actualizan:

- `src/svp_errors.py`;
- `src/svp_validator.py`;
- `tests/run_conformance.py`;
- los dos casos de prueba nuevos;
- `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`;
- la matriz de concordancia diagnóstica;
- la tabla de correspondencias funcionales;
- el registro de deuda viva;
- esta acta.

## 7. Evidencia de cierre

La rama `agent/ffl-b-p0b-supervise-meta`, en la revisión funcional `8080e22ddd103b6a33ae157ce86bdf1de540025d`, fue sometida a verificación externa en modo de solo lectura.

Resultados acreditados:

- `tests/run_conformance.py`: **44/44**;
- `tests/run_cli_smoke.py`: **3/3**;
- `tests/run_sec0_smoke.py`: **3/3**;
- `supervise_meta_no_evalresult.svp`: emisión exacta de `E212 — SuperviseMetaNotEvalResult`;
- `supervise_coupled_wrong_role.svp`: emisión exacta de `E211 — SuperviseMetaNotSupervisor`.

La verificación distingue de forma observable los dos juicios: `E212` protege el tipo del primer argumento y `E211` protege su procedencia desde una célula con rol `Supervisor`, incluida la ruta acoplada recibida de P0-A.

También se comprobó que:

- `supervise_targets.svp` continúa siendo válido;
- `evaluate(CoupledState)` continúa aceptado;
- `Frame.cell_states` continúa exigiendo `CoupledStateDecl`;
- `_validate_eval` conserva `CellStateDecl | CoupledStateDecl`.

La comparación de cambios confirmó que la modificación funcional se limita al alta de `E212` y a `_validate_supervise`, con las pruebas y la documentación correspondientes.

## 8. Estado y límites

**P0-B queda cerrado y P0 queda estabilizado en sus dos partes.**

Este cierre no abre por sí mismo ninguna obligación posterior de FFL-B ni autoriza infraestructura de ejecución, Rust, WASM, IA productiva, biblioteca estándar o `NL → SVP`.
