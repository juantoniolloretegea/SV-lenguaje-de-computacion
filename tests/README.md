# `tests/` — Baterías ejecutables del Lenguaje SV

**Fecha de resincronización:** 21 de agosto de 2026  
**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0

## 1. Objeto

Esta carpeta contiene dos ámbitos de evidencia ejecutable que deben mantenerse diferenciados:

1. la conformidad de la etapa frontal de referencia del Lenguaje SV, cuya secuencia es `.svp → análisis sintáctico → validación → descenso a IR v0.2 → JSON normalizado`;
2. la materialización ejecutable inicial de obligaciones seleccionadas de los contratos SEC.0-A, SEC.0-D, SEC.0-M, SEC.0-X y SEC.0-T.

Para cada caso válido de la etapa frontal, `tests/run_conformance.py` exige que la salida normalizada coincida con su archivo `.expected.json`. Para cada caso inválido, el mismo ejecutor exige que el procesamiento termine con el código diagnóstico exacto declarado en `EXPECTED_INVALID_CODES`.

La batería contractual SEC.0 se mantiene separada en `tests/sec0/` y `tests/run_sec0_contracts.py`. No modifica gramática, IR v0.2, validador ni catálogo diagnóstico y no constituye un entorno de ejecución de producción.

## 2. Ejecución

```bash
python tests/run_conformance.py
python tests/run_cli_smoke.py
python tests/run_sec0_smoke.py
python tests/run_e006_characterization.py
python tests/run_sec0_contracts.py
```

Los nombres históricos de dos ejecutores contienen `smoke`; se conservan como identificadores de archivo. `tests/run_sec0_smoke.py` corresponde a la línea previa de resistencia del compilador y sus tres casos no deben interpretarse como cobertura de los contratos SEC.0-A/D/M/X/T.

La batería contractual nueva se documenta en `tests/sec0/README.md`.

## 3. Estado acreditado de la etapa frontal

Sobre el estado `3d48c422915b0e0bed65ba2e7ce8b807d7a94c33`, una verificación independiente en modo de solo lectura acreditó:

- conformidad: **58/58** — 10 casos válidos y 48 inválidos;
- pruebas rápidas de la interfaz de línea de órdenes: **3/3**;
- SEC-0 histórico de resistencia del compilador: **3/3**;
- caracterización de E006: **4/4**.

Los cuatro ejecutores finalizaron con código de retorno 0 y el árbol de trabajo permaneció sin modificaciones antes y después de la ejecución.

Estos resultados son históricos y están ligados al estado indicado. No se transfieren automáticamente a commits posteriores.

## 4. Batería contractual SEC.0

`tests/run_sec0_contracts.py` y `tests/sec0/reference_model.py` constituyen la primera materialización ejecutable de propiedades seleccionadas de los contratos SEC.0 cerrados el 21 de agosto de 2026.

Su objetivo inicial es comprobar la traducibilidad de obligaciones como fallo cerrado, autoridad preconstituida, presupuesto de recursos, continuidad, independencia frente al mismo fallo, actualidad de atestación, ligadura presentación-firma, falsabilidad y cobertura de pruebas.

Un resultado satisfactorio de esta batería acredita únicamente el modelo ejecutable y los casos efectivamente ensayados. No equivale a conformidad completa de la implementación vigente ni a certificación de una plataforma material.

## 5. Casos válidos de conformidad SVP → IR

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

## 6. Casos inválidos

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

## 7. Caracterización de E006

`tests/run_e006_characterization.py` comprueba de forma separada cuatro situaciones que actualmente emiten E006:

- una referencia inexistente;
- `CellTarget` con referencia existente de tipo incompatible;
- `ComposedTarget` con referencia existente de tipo incompatible;
- `SystemTarget` con referencia existente de tipo incompatible.

La caracterización obtuvo **4/4** sobre el estado histórico indicado y no modifica el código diagnóstico, su nombre, su mensaje ni el validador. La diferencia entre ambos supuestos permanece documentada como deuda de precisión diagnóstica.

## 8. Alcance de E215

E215 comprueba la correspondencia entre la secuencia de `EvalResult` recibida por `gate` y `AdmissibilityTable.input_codomains`:

- igual número de entradas;
- igual codominio nominal en cada posición.

La comprobación posicional distingue codominios diferentes aunque contengan el mismo conjunto de valores. También se verificó de forma adicional la ruta de una evaluación procedente de `CoupledState`.

E215 no ejecuta la tabla ni calcula `GateResult.output`.

## 9. Cobertura observable y límites

Los 48 casos inválidos del estado acreditado cubren explícitamente 37 de los 47 códigos del catálogo efectivo. Los diez códigos restantes se encuentran clasificados en `COBERTURA_OBSERVABLE_FFL_C_2026_08_20.md` según su alcanzabilidad real, la existencia de rutas diagnósticas alternativas o la preservación estructural de la obligación.

La evidencia de la etapa frontal se interpreta distinguiendo:

1. caso persistido;
2. emisión observable;
3. propiedad estructural.

La batería SVP → IR comprueba conformidad de la etapa frontal y su descenso a IR. La batería SEC.0 comprueba, de forma separada, un modelo ejecutable de referencia de obligaciones contractuales. Ninguna de las dos constituye por sí sola una certificación de ejecución material completa.

FFL-A, FFL-B, FFL-C y FFL-E están cerrados. FFL-D permanece pendiente hasta decisión expresa.

---

*Lenguaje de computación del Sistema Vectorial SV.*  
*Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351 | CC BY-NC-ND 4.0 | ISSN 2695-6411*
