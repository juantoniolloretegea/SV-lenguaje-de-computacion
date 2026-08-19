# `tests/` — Batería de conformidad SVP → IR

**Fecha de resincronización:** 19 de agosto de 2026  
**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0

## 1. Objeto

Esta carpeta contiene la evidencia ejecutable de conformidad de la etapa frontal de referencia del Lenguaje SV.

La secuencia comprobada es:

`.svp → análisis sintáctico → validación → descenso a IR v0.2 → JSON normalizado`

Para cada caso válido, `tests/run_conformance.py` exige que la salida normalizada coincida con su archivo `.expected.json`.

Para cada caso inválido, el mismo ejecutor exige que el procesamiento termine con el código diagnóstico exacto declarado en `EXPECTED_INVALID_CODES`.

## 2. Ejecución

```bash
python tests/run_conformance.py
python tests/run_cli_smoke.py
python tests/run_sec0_smoke.py
```

Los nombres históricos de los dos últimos ejecutores contienen `smoke`; se conservan como identificadores de archivo. Su función es realizar pruebas rápidas de la interfaz de línea de órdenes y de la línea SEC-0.

## 3. Estado acreditado

Tras la incorporación de `E215 — GateTableSignatureMismatch` se acredita:

- conformidad: **57/57**;
- pruebas rápidas de la interfaz de línea de órdenes: **3/3**;
- SEC-0: **3/3**.

La batería principal se compone de **9 casos válidos** y **48 casos inválidos**.

## 4. Casos válidos

| Archivo | Objeto principal |
|---|---|
| `admissibility_spec_states_permutados.svp` | admisibilidad con orden de estados no significativo |
| `cell_basic.svp` | célula simple, estado y evaluación |
| `compose_basic.svp` | composición con relación semántica y patrón declarados |
| `gate_table.svp` | compuerta con tabla explícita de admisibilidad |
| `query_context_all_variants.svp` | cinco variantes vigentes de `QueryContext` |
| `resolve_projection.svp` | resolución de `U` y proyección estructural de `resolved_to` |
| `supervise_targets.svp` | supervisión con objetos supervisables tipados |
| `transition_data_events.svp` | `TransitionData` con sucesos tipados y cambios inducidos |
| `trajectory_alternance_valid.svp` | alternancia constitutiva de `TrajectoryEntry` |

## 5. Casos inválidos

La relación completa de casos inválidos y códigos esperados se mantiene en `EXPECTED_INVALID_CODES`, dentro de `tests/run_conformance.py`. Esa tabla es la referencia ejecutable para la correspondencia caso → diagnóstico.

Entre las comprobaciones recientes figuran:

- `admissibility_table_output_fuera_codominio.svp` → `E011`;
- `supervise_meta_no_evalresult.svp` → `E212`;
- `supervise_coupled_wrong_role.svp` → `E211`;
- `transition_event_fuera_horizon.svp` → `E307`;
- `transition_induced_parameters_vacios.svp` → `E406`;
- `projection_source_no_resultado.svp` → `E213`;
- `projection_campo_inexistente.svp` → `E214`;
- `projection_undeclared_source.svp` → `E006`;
- `resolve_missing_context.svp` → `E206`;
- `resolve_missing_mechanism.svp` → `E207`;
- `graph_conflicts_fuera_de_v0_1.svp` → `E001`;
- `graph_simple_concurrencia_mismo_puente.svp` → `E114`;
- `supervise_celltarget_tipo_incorrecto.svp` → `E006`;
- `supervise_composedtarget_tipo_incorrecto.svp` → `E006`;
- `supervise_systemtarget_tipo_incorrecto.svp` → `E006`;
- `gate_numero_entradas_incompatible_con_tabla.svp` → `E215`;
- `gate_codominio_posicional_incompatible_con_tabla.svp` → `E215`.

La cobertura de un código mediante un caso explícito no implica por sí sola la cobertura exhaustiva de todo el juicio de la IR relacionado.

## 6. Alcance de E215

E215 comprueba la correspondencia entre la secuencia de `EvalResult` recibida por `gate` y `AdmissibilityTable.input_codomains`:

- igual número de entradas;
- igual codominio nominal en cada posición.

La comprobación posicional distingue codominios diferentes aunque contengan el mismo conjunto de valores. También se verificó de forma adicional la ruta de una evaluación procedente de `CoupledState`.

E215 no ejecuta la tabla ni calcula `GateResult.output`.

## 7. Límites

Esta batería comprueba la conformidad de la etapa frontal y su descenso a IR. No constituye por sí sola una ejecución material del sistema ni acredita capacidades de infraestructura de ejecución que no estén implementadas.

FFL-B queda cerrado con esta evidencia acumulada. FFL-C, FFL-D y FFL-E permanecen pendientes hasta decisión expresa.

---

*Lenguaje de computación del Sistema Vectorial SV.*  
*Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351 | CC BY-NC-ND 4.0 | ISSN 2695-6411*
