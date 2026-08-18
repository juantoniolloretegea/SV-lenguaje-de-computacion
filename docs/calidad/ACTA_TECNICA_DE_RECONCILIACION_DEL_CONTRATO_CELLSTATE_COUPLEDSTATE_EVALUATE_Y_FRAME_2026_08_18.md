# Acta técnica de reconciliación del contrato `CellState` / `CoupledState` / `evaluate` / `Frame`

**Fecha:** 18/08/2026  
**Frente:** FFL-B — P0-A de estabilización  
**Estado:** CERRADO  
**Autor del corpus:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**ISSN:** 2695-6411

## 1. Objeto

Esta acta registra la reconciliación de la relación entre los estados simples y acoplados, la operación `evaluate` y el contenido de `Frame.cell_states`.

La revisión se abrió al constatar una regresión posterior a la confirmación de cambios `3255ae6438a6affbcc660caf8c9077d81ae5286b`: la batería principal de conformidad permanecía satisfactoria, pero una prueba de SEC-0 y varios ejemplos documentados dejaban de superar la validación por una discordancia entre `CellState` y `CoupledState`.

## 2. Hechos comprobados

1. `Frame.cell_states` está tipado en la IR v0.2 como lista de `CoupledState` y el validador exige `CoupledStateDecl` en ese campo.
2. La revisión `3255ae6` restringió `_validate_eval` a `CellStateDecl`.
3. La Gramática superficial mínima v0.1 admite que `evaluate` reciba `CellState` o `CoupledState`.
4. El Documento I de composición intercelular establece que, tras la transmisión, una célula acoplada se evalúa sobre su vector actualizado `x̃_i`, mediante `y_i = χ_i(C_i[x̃_i])`.
5. La IR v0.2 mantenía `EvalResult.source_state : CellStateRef` y formulaba J3.1 sólo respecto de `CellState`.
6. El repositorio contiene casos legítimos de evaluación simple y de evaluación sobre estado acoplado.

## 3. Clasificación

Se constataron dos cuestiones distintas:

- una estrechez de la IR v0.2 y del validador respecto de la evaluación de un estado acoplado ya sustentada por la matemática superior;
- varios casos de prueba antiguos que introducían un `CellState` directamente en `Frame.cell_states`, en contradicción con el tipado vigente de `Frame`.

Durante la misma revisión se identificó una cuestión separada relativa a `supervise(meta_eval, ...)`. Esa cuestión se trató posteriormente en P0-B y no forma parte del juicio material de esta acta.

## 4. Decisión

Se adopta la siguiente reconciliación acotada:

- se preserva `IR_CANONICA_BIENFORMACION_SV_v0_2.md` como versión histórica de marzo;
- una adenda técnica vigente fija `EvaluableStateRef = CellStateRef | CoupledStateRef`;
- para `CellState`, la evaluación toma `vector` como configuración efectiva;
- para `CoupledState`, la evaluación toma `updated_vector`, conservando `base_vector` como procedencia;
- `Frame.cell_states` continúa siendo una lista de `CoupledState`;
- `_validate_eval` admite exclusivamente `CellStateDecl` o `CoupledStateDecl`;
- los casos de prueba que introducían indebidamente un `CellState` en `Frame` se corrigen mediante una representación acoplada explícita.

No se introduce un estado transitorio oculto ni una coerción implícita entre ambos tipos.

## 5. Contraste crítico

Se descartaron expresamente estas alternativas:

- aceptar ambos tipos únicamente en el validador sin corregir la estrechez documental de la IR;
- restringir la gramática a `CellState`, en contradicción con la evaluación compositiva ya constituida;
- relajar `Frame.cell_states` para admitir `CellState`;
- derivar implícitamente un `CellState` oculto desde `CoupledState`.

La solución adoptada mantiene la distinción de tipos, preserva la procedencia y limita el cambio al contrato realmente afectado.

## 6. Artefactos afectados

El cierre comprende:

- `ADENDA_TECNICA_IR_v0_2_ESTADO_EVALUABLE_ACOPLADO_2026_08_18.md`;
- `src/svp_validator.py`;
- `tests/adversarial/deep_nested_query_valid.svp`;
- `examples/consulta_framecomparison.svp`;
- `tests/adversarial/documentados/agente_con_consulta_y_dominio.svp`.

`tests/adversarial/documentados/composicion_serie_con_trayectoria.svp` no necesitó modificación: ya representaba correctamente `CoupledState` tanto en `evaluate` como en `Frame`.

## 7. Evidencia de cierre

La rama `agent/ffl-b-evaluable-state-reconcile`, en la revisión `b9db1a268e7acf8283f99eb6d7d09da243a9293c`, fue sometida a verificación externa en modo de solo lectura.

Resultados acreditados:

- `tests/run_conformance.py`: **42/42**;
- `tests/run_cli_smoke.py`: **3/3**;
- `tests/run_sec0_smoke.py`: **3/3**;
- `examples/consulta_framecomparison.svp`: válido, con producción de IR JSON;
- `tests/adversarial/documentados/agente_con_consulta_y_dominio.svp`: válido, con producción de IR JSON;
- `tests/adversarial/documentados/composicion_serie_con_trayectoria.svp`: válido, con producción de IR JSON y sin modificación del archivo.

La última comprobación es especialmente discriminante: confirma que `evaluate(CoupledState)` vuelve a ser aceptado sin alterar el caso de prueba que ya representaba correctamente la composición.

La comparación de cambios mostró un alcance funcional estrictamente acotado en `src/svp_validator.py`, sin reordenación general del archivo ni modificaciones ajenas al juicio.

## 8. Estado y límites

**P0-A queda cerrado.**

El cierre no modifica la gramática superficial, no declara cerrado FFL-B y no abre infraestructura de ejecución, Rust, WASM, IA productiva, biblioteca estándar ni `NL → SVP`.

La correspondencia general entre cada `CoupledState` almacenado en un `Frame` y cada `EvalResult` del mismo `Frame` permanece como cuestión técnica separada cuando resulte materialmente necesaria.
