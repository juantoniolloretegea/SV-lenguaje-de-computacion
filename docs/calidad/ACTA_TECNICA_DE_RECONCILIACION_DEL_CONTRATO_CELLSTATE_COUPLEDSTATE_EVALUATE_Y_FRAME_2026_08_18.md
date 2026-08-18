# Acta técnica de reconciliación del contrato CellState / CoupledState / evaluate / Frame

**Fecha:** 18/08/2026  
**Frente:** FFL-B — P0-A de estabilización previa a nuevos microcierres  
**Estado:** CERRADO  
**Autor del corpus:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**ISSN:** 2695-6411

---

## 1. Disparador

Una auditoría independiente de lectura completa del repositorio detectó que, tras el commit `3255ae6438a6affbcc660caf8c9077d81ae5286b`, la suite principal de conformidad permanecía verde, mientras una sonda SEC-0 y ejemplos documentados dejaban de atravesar el validator por una discordancia `CellState` / `CoupledState`.

La sorpresa tiene impacto de contrato y activa auditoría dura conforme al `PROCEDIMIENTO_AUDITORIA_TECNICA_SV.md`.

## 2. Hechos comprobados

1. `Frame.cell_states` está tipado en la IR v0.2 como lista de `CoupledState`, y el validator vigente exige `CoupledStateDecl` en ese campo.
2. El commit `3255ae6` endureció `_validate_eval` para aceptar exclusivamente `CellStateDecl`.
3. La Gramática superficial mínima v0.1 declara que `evaluate` admite `CellState` o `CoupledState`.
4. El Documento I de composición intercelular establece que, tras la transmisión, la evaluación de una célula acoplada se realiza sobre su vector actualizado `x̃_i`, mediante `y_i = χ_i(C_i[x̃_i])`.
5. La IR v0.2 conservaba, sin embargo, `EvalResult.source_state : CellStateRef` y formulaba J3.1 exclusivamente respecto de `CellState`.
6. Existen sondas y ejemplos que representan ambos regímenes: evaluación simple sobre `CellState` y evaluación compositiva sobre `CoupledState`.
7. La misma auditoría ha localizado además un hueco distinto en `supervise`: el validator comprueba la existencia de `meta_eval` y, si ya es un `EvalCmd`, el rol Supervisor de su célula fuente, pero no rechaza de forma expresa un primer argumento que no sea `EvalResult`.

## 3. Clasificación

**Error real P0-A:** estrechez localizada de la IR v0.2 y del validator respecto de la evaluación de estado acoplado ya constituida por la matemática superior.

**Error real de fixtures P0-A:** algunas sondas antiguas introducían un `CellState` directamente en `Frame.cell_states`, contradiciendo el tipado vigente de `Frame`.

**Error real P0-B, separado:** `supervise` debe exigir un `EvalResult` como primer argumento antes de comprobar su procedencia desde una célula con rol `Supervisor`. Se tratará en un microbloque inmediatamente posterior, con juicio, ruta diagnóstica y prueba propios.

**Deuda futura separada:** la IR y el validator no imponen todavía una correspondencia general entre los `CoupledState` almacenados en un `Frame` y las fuentes de sus `EvalResult`.

**Fuera de P0-A:** salidas de tablas, `CoverageReport`, `compose`, proyecciones, `E406`, índices y portada no se incorporan a este lote.

## 4. Decisión

Se adopta una reconciliación acotada para P0-A:

- preservar `IR_CANONICA_BIENFORMACION_SV_v0_2.md` como versión histórica de marzo;
- añadir una adenda técnica vigente que fija `EvaluableStateRef = CellStateRef | CoupledStateRef`;
- para `CellState`, evaluar `vector`;
- para `CoupledState`, evaluar `updated_vector` y conservar `base_vector` como procedencia;
- mantener `Frame.cell_states : [CoupledState]` sin relajación;
- modificar únicamente `_validate_eval` para admitir ambos tipos y rechazar cualquier tercero;
- corregir las sondas que introducían `CellState` dentro de `Frame`, convirtiéndolas en representaciones acopladas explícitas cuando su propio objeto es un frame de arquitectura.

El hueco de `supervise` no se mezcla en este commit. Se abre como P0-B inmediatamente después de verificar P0-A, evitando un parche compuesto y preservando la regla operativa de un juicio por commit.

## 5. Adversarial

### Objeción A — aceptar ambos tipos sólo en el validator

Se rechaza. Haría verde la implementación dejando la IR v0.2 formalmente más estrecha y permitiría que una capa inferior corrigiera silenciosamente a su especificación.

### Objeción B — restringir la gramática a CellState

Se rechaza. Contradiría el Documento I, que exige evaluar la configuración actualizada de la célula acoplada, y convertiría en ilegítima una operación compositiva ya constituida.

### Objeción C — permitir CellState en Frame

Se rechaza. Debilitaría J4.1 y borraría la distinción entre estado simple y estado acoplado dentro de la evaluación completa de una arquitectura.

### Objeción D — derivar implícitamente un CellState oculto desde CoupledState

No se adopta. El frontend actual ya baja `evaluate(identifier)` conservando la referencia al estado fuente; introducir un estado transitorio implícito añadiría una transformación no especificada y degradaría la procedencia. La solución mínima es tipar explícitamente la unión de estados evaluables.

### Objeción E — añadir también `supervise` al mismo parche

Se rechaza por radio de cambio. El defecto es real, pero no es la causa de la regresión SEC-0 producida por `3255ae6` y dispone de un contrato distinto en J3.3. Integrarlo ahora debilitaría la trazabilidad causal del microcierre y dificultaría atribuir cualquier nueva regresión.

## 6. Artefactos del lote P0-A

- `ADENDA_TECNICA_IR_v0_2_ESTADO_EVALUABLE_ACOPLADO_2026_08_18.md`;
- `src/svp_validator.py`;
- `tests/adversarial/deep_nested_query_valid.svp`;
- `examples/consulta_framecomparison.svp`;
- `tests/adversarial/documentados/agente_con_consulta_y_dominio.svp`.

La sonda `tests/adversarial/documentados/composicion_serie_con_trayectoria.svp` no requiere modificación: ya representa correctamente `CoupledState` tanto en `evaluate` como en `Frame` y vuelve a ser aceptada al restaurarse legítimamente `evaluate(CoupledState)`.

## 7. Evidencia dinámica de cierre

La rama `agent/ffl-b-evaluable-state-reconcile`, con `HEAD` previo de verificación `b9db1a268e7acf8283f99eb6d7d09da243a9293c`, fue ejecutada en solo lectura por una unidad auditora independiente, sin commits ni parches de esa unidad.

Resultados comunicados y recibidos como evidencia externa de cierre:

- `tests/run_conformance.py`: **42/42**, `rc=0`;
- `tests/run_cli_smoke.py`: **3/3**, `rc=0`;
- `tests/run_sec0_smoke.py`: **3/3**, `rc=0`;
- `examples/consulta_framecomparison.svp`: `rc=0`, IR JSON producido;
- `tests/adversarial/documentados/agente_con_consulta_y_dominio.svp`: `rc=0`, IR JSON producido;
- `tests/adversarial/documentados/composicion_serie_con_trayectoria.svp`: `rc=0`, IR JSON producido sin modificación del archivo.

La última sonda constituye una evidencia especialmente discriminante: atraviesa el frontend sólo por la restauración legítima de `evaluate(CoupledState)`, no por maquillaje del fixture.

La inspección del diff acredita además radio corto: el cambio funcional de `src/svp_validator.py` es de cinco adiciones y una eliminación; no existe remaquetación masiva ni contaminación ajena al juicio.

Con esta evidencia, P0-A queda **cerrado**.

## 8. Continuidad de P0

El cierre de P0-A no autoriza por sí solo abrir nuevos diagnósticos FFL-B. El paso inmediato es P0-B: materializar la exigencia `supervise(meta_eval, ...)` con `meta_eval : EvalResult`, conservar después la comprobación de rol `Supervisor`, dotar el hueco de prueba trazable y reejecutar las tres suites.

P0 completo sólo podrá considerarse estabilizado cuando P0-A y P0-B estén cerrados y la evidencia dinámica global sea verde.

## 9. Límites

No se abre backend, Rust, WASM, runtime, IA productiva ni `NL → SVP`. No se declara cerrada FFL-B. No se modifica la gramática superficial, porque su formulación dual de `evaluate` queda confirmada por la matemática superior.

---

*Documento técnico subordinado del Lenguaje SV.*
