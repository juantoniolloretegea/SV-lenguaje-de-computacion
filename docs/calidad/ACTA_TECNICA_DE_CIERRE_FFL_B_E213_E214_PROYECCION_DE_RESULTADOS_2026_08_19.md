# Acta técnica de cierre FFL-B — E213/E214 y proyección estructural de resultados

**Fecha:** 19/08/2026  
**Estado:** CERRADO  
**Ámbito:** Lenguaje SV / FFL-B / proyección de campos de objetos de resultado  
**Base técnica:** gramática superficial mínima v0.1, IR canónica v0.2 y contrato efectivo bajo Vía B

## 1. Objeto

Esta acta registra el cierre de dos precondiciones estructurales de la operación de proyección ya presente en la superficie del Lenguaje SV:

1. la fuente de una proyección debe ser un objeto de resultado producido por un operador compatible;
2. el campo solicitado debe pertenecer al esquema del tipo de resultado correspondiente.

El cierre no ejecuta resultados, no calcula el valor proyectado y no modifica la gramática ni la IR canónica.

## 2. Fundamento

La superficie vigente admite expresiones de la forma:

`let X = Resultado.campo;`

La IR v0.2 fija los esquemas de los objetos de resultado producidos por los operadores superficiales:

- `EvalResult`: `source_state`, `counts`, `threshold`, `classification`, `criticality`, `deltas`;
- `GateResult`: `inputs`, `table`, `output`;
- `ResolutionRecord`: `parameter`, `previous`, `resolved_to`, `context`, `mechanism`;
- `QueryResult`: `response`, `justification`, `metadata`;
- `SupervisionResult`: `meta_eval`, `target`, `verdict`.

El descenso a IR conserva la fuente y el nombre del campo y produce una operación de tipo `Projected`. Antes de este cierre, el validador comprobaba únicamente que la fuente estuviera declarada.

La tabla canónica de errores de la IR v0.2 no asigna identificadores autónomos a estas dos precondiciones. Bajo Vía B se utilizan dos identificadores efectivos libres:

- `E213 — ProjectionSourceNotResult`;
- `E214 — ProjectionFieldNotFound`.

## 3. Materialización

La validación mantiene la precedencia de `E006 — UndeclaredReference`: una fuente inexistente se rechaza antes de examinar su tipo.

Para una fuente declarada:

- `E213` rechaza declaraciones que no sean productores superficiales de `EvalResult`, `GateResult`, `ResolutionRecord`, `QueryResult` o `SupervisionResult`;
- `E214` rechaza nombres de campo que no pertenezcan al esquema canónico asociado al productor de resultado.

La operación `compose`, los objetos `Frame` y los resultados de una proyección previa no se consideran fuentes proyectables en este cierre.

## 4. Evidencia

La modificación funcional fue comprobada sobre la base:

`bce17c5936b07ca0dddd76c1f6f021a242166676`

con la confirmación funcional:

`dc5761bec9ebb4e7aabec3af10bc8aa77896e2f1`

Una verificación independiente en modo de solo lectura confirmó:

- batería de conformidad: **48/48**, código de salida 0;
- pruebas rápidas de la interfaz de línea de órdenes: **3/3**, código de salida 0;
- SEC-0: **3/3**, código de salida 0;
- `projection_source_no_resultado.svp`: emisión exacta de `E213 — ProjectionSourceNotResult`;
- `projection_campo_inexistente.svp`: emisión exacta de `E214 — ProjectionFieldNotFound`;
- `projection_undeclared_source.svp`: conservación de `E006 — UndeclaredReference`;
- `resolve_projection.svp`: aceptación y producción exacta de la IR esperada para `RR1.resolved_to`.

Sondas adicionales, ejecutadas fuera del árbol del repositorio, confirmaron la aceptación estructural de los campos superficialmente expresables de `EvalResult`, `GateResult`, `QueryResult`, `SupervisionResult` y `ResolutionRecord`, así como el rechazo mediante `E213` de `Architecture`, `Frame` y `Projected` como fuentes.

## 5. Alcance y límites

Este cierre acredita únicamente la legalidad estructural de la forma `Resultado.campo` para los cinco tipos de resultado producidos por operadores de superficie.

No acredita:

- ejecución material de `EvalResult`, `GateResult`, `ResolutionRecord`, `QueryResult` o `SupervisionResult`;
- cálculo del valor del campo proyectado;
- proyección de `Architecture`;
- proyecciones encadenadas sobre `Projected`;
- incorporación de campos nuevos a los esquemas canónicos;
- disponibilidad superficial de todo campo canónico.

En particular, `target`, `context` y `mechanism` son nombres reservados por la superficie léxica vigente. Aunque pertenecen respectivamente a los esquemas canónicos de `SupervisionResult` y `ResolutionRecord`, no pueden escribirse actualmente como identificadores después del punto sin que intervenga antes el análisis sintáctico. Este cierre no modifica esa restricción ni la presenta como capacidad disponible.

El texto detallado de `E214` identifica el tipo de nodo productor en la implementación. Esa referencia no sustituye ni renombra el tipo de resultado fijado por la IR.

`CriticalityResult` permanece fuera del cierre porque la superficie v0.1 no dispone de un operador que produzca ese resultado.

## 6. Estado resultante

`E213 — ProjectionSourceNotResult` y `E214 — ProjectionFieldNotFound` quedan materializados y cubiertos por casos explícitos de conformidad dentro del contrato diagnóstico efectivo bajo Vía B.

FFL-B permanece abierto. Este cierre no abre FFL-C, FFL-D, FFL-E, infraestructura de ejecución, Rust, WASM, IA propia ni programación de SVP mediante lenguaje natural.
