# Acta técnica de reconciliación del contrato CellState / CoupledState / evaluate / Frame

**Fecha:** 18/08/2026  
**Frente:** FFL-B — cadena de implementación  
**Estado:** aplicación preparada; cierre condicionado a verificación dinámica posterior  
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

## 3. Clasificación

**Error real:** estrechez localizada de la IR v0.2 y del validator respecto de la evaluación de estado acoplado ya constituida por la matemática superior.

**Error real de fixtures:** algunas sondas antiguas introducían un `CellState` directamente en `Frame.cell_states`, contradiciendo el tipado vigente de `Frame`.

**Deuda futura separada:** la IR y el validator no imponen todavía una correspondencia general entre los `CoupledState` almacenados en un `Frame` y las fuentes de sus `EvalResult`.

**Fuera de alcance:** los demás hallazgos de la auditoría independiente —`supervise`, salidas de tablas, `CoverageReport`, `compose`, proyecciones, `E406`, índices y portada— no se incorporan a este lote.

## 4. Decisión

Se adopta una reconciliación acotada:

- preservar `IR_CANONICA_BIENFORMACION_SV_v0_2.md` como versión histórica de marzo;
- añadir una adenda técnica vigente que fija `EvaluableStateRef = CellStateRef | CoupledStateRef`;
- para `CellState`, evaluar `vector`;
- para `CoupledState`, evaluar `updated_vector` y conservar `base_vector` como procedencia;
- mantener `Frame.cell_states : [CoupledState]` sin relajación;
- modificar únicamente `_validate_eval` para admitir ambos tipos y rechazar cualquier tercero;
- corregir las sondas que introducían `CellState` dentro de `Frame`, convirtiéndolas en representaciones acopladas explícitas cuando su propio objeto es un frame de arquitectura.

## 5. Adversarial

### Objeción A — aceptar ambos tipos sólo en el validator

Se rechaza. Haría verde la implementación dejando la IR v0.2 formalmente más estrecha y permitiría que una capa inferior corrigiera silenciosamente a su especificación.

### Objeción B — restringir la gramática a CellState

Se rechaza. Contradiría el Documento I, que exige evaluar la configuración actualizada de la célula acoplada, y convertiría en ilegítima una operación compositiva ya constituida.

### Objeción C — permitir CellState en Frame

Se rechaza. Debilitaría J4.1 y borraría la distinción entre estado simple y estado acoplado dentro de la evaluación completa de una arquitectura.

### Objeción D — derivar implícitamente un CellState oculto desde CoupledState

No se adopta. El frontend actual ya baja `evaluate(identifier)` conservando la referencia al estado fuente; introducir un estado transitorio implícito añadiría una transformación no especificada y degradaría la procedencia. La solución mínima es tipar explícitamente la unión de estados evaluables.

## 6. Artefactos del lote

- `ADENDA_TECNICA_IR_v0_2_ESTADO_EVALUABLE_ACOPLADO_2026_08_18.md`;
- `src/svp_validator.py`;
- `tests/adversarial/deep_nested_query_valid.svp`;
- `examples/consulta_framecomparison.svp`;
- `tests/adversarial/documentados/agente_con_consulta_y_dominio.svp`.

La sonda `tests/adversarial/documentados/composicion_serie_con_trayectoria.svp` no requiere modificación: debe volver a ser aceptada al restaurarse legítimamente `evaluate(CoupledState)`.

## 7. Criterio de cierre

El lote no se declarará cerrado sólo por estar aplicado. Requiere, como mínimo:

1. inspección posterior del diff y confirmación de radio corto;
2. conformidad principal sin regresión;
3. CLI smoke sin regresión;
4. SEC-0 nuevamente verde;
5. aceptación de `examples/consulta_framecomparison.svp`;
6. aceptación de las sondas documentadas `agente_con_consulta_y_dominio.svp` y `composicion_serie_con_trayectoria.svp`.

Hasta disponer de esa evidencia dinámica, el estado es **aplicado / pendiente de cierre** y no se reabre `E406`.

## 8. Límites

No se abre backend, Rust, WASM, runtime, IA productiva ni `NL → SVP`. No se declara cerrada FFL-B. No se modifica la gramática superficial, porque su formulación dual de `evaluate` queda confirmada por la matemática superior.

---

*Documento técnico subordinado del Lenguaje SV.*
