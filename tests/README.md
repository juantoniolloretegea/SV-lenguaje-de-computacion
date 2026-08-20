# `tests/` — Batería de conformidad SVP → IR

**Fecha de resincronización:** 20 de agosto de 2026  
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
python tests/run_e006_characterization.py
```

Los nombres históricos de dos ejecutores contienen `smoke`; se conservan como identificadores de archivo. Su función es realizar pruebas rápidas de la interfaz de línea de órdenes y de la línea SEC-0.

## 3. Estado acreditado

Sobre el estado `3d48c422915b0e0bed65ba2e7ce8b807d7a94c33`, una verificación independiente en modo de solo lectura acreditó:

- conformidad: **58/58** — 10 casos válidos y 48 inválidos;
- pruebas rápidas de la interfaz de línea de órdenes: **3/3**;
- SEC-0: **3/3**;
- caracterización de E006: **4/4**.

Los cuatro ejecutores finalizaron con código de retorno 0 y el árbol de trabajo permaneció sin modificaciones antes y después de la ejecución.

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
| `supervise_systemtarget_valido.svp` | supervisión de `CompositionGraph` mediante `SystemTarget` |
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

## 6. Caracterización de E006

`tests/run_e006_characterization.py` comprueba de forma separada cuatro situaciones que actualmente emiten E006:

- una referencia inexistente;
- `CellTarget` con referencia existente de tipo incompatible;
- `ComposedTarget` con referencia existente de tipo incompatible;
- `SystemTarget` con referencia existente de tipo incompatible.

La caracterización obtuvo **4/4** y no modifica el código diagnóstico, su nombre, su mensaje ni el validador. La diferencia entre ambos supuestos permanece documentada como deuda de precisión diagnóstica.

## 7. Alcance de E215

E215 comprueba la correspondencia entre la secuencia de `EvalResult` recibida por `gate` y `AdmissibilityTable.input_codomains`:

- igual número de entradas;
- igual codominio nominal en cada posición.

La comprobación posicional distingue codominios diferentes aunque contengan el mismo conjunto de valores. También se verificó de forma adicional la ruta de una evaluación procedente de `CoupledState`.

E215 no ejecuta la tabla ni calcula `GateResult.output`.

## 8. Cobertura observable y límites

Los 48 casos inválidos cubren explícitamente 37 de los 47 códigos del catálogo efectivo. Los diez códigos restantes se encuentran clasificados en `COBERTURA_OBSERVABLE_FFL_C_2026_08_20.md` según su alcanzabilidad real, la existencia de rutas diagnósticas alternativas o la preservación estructural de la obligación.

La evidencia se interpreta distinguiendo:

1. caso persistido;
2. emisión observable;
3. propiedad estructural.

Esta batería comprueba la conformidad de la etapa frontal y su descenso a IR. No constituye por sí sola una ejecución material del sistema ni acredita capacidades de infraestructura de ejecución que no estén implementadas.

FFL-A, FFL-B y FFL-C están cerrados. FFL-D y FFL-E permanecen pendientes hasta decisión expresa.

---

*Lenguaje de computación del Sistema Vectorial SV.*  
*Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351 | CC BY-NC-ND 4.0 | ISSN 2695-6411*
