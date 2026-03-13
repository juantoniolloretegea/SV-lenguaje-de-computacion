# tests/ — Suite de conformidad DSL → IR

## Qué comprueba

Verifica mecánicamente que la cadena `.svp` → parser → validación → IR v0.2 → JSON canónico funciona correctamente.

Para cada caso válido, comprueba que se produce JSON con los campos obligatorios y que coincide bit a bit, en forma canónica, con su `.expected.json` correspondiente.

Para cada caso inválido, comprueba que falla con el código exacto esperado del catálogo.

Además, incluye una batería mínima de **smoke tests de CLI** para verificar el contrato externo de la implementación de referencia:
- ejecución válida por `stdout`
- ejecución válida con `-o` y creación de archivo
- ejecución inválida con `rc=1` y prefijo `ERROR:` en `stderr`

## Ejecución

```bash
python tests/run_conformance.py
python tests/run_cli_smoke.py
```

## Casos válidos (7)

| Archivo | Qué verifica |
|---------|-------------|
| `cell_basic.svp` | Célula simple b=3, codominio, semántica, estado, evaluación |
| `gate_table.svp` | Dos células + compuerta con tabla explícita de admisibilidad |
| `resolve_projection.svp` | Resolución de U con ResolutionRecord + proyección `resolved_to` |
| `supervise_targets.svp` | Supervisión meta con CellTarget y ComposedTarget, rol Supervisor |
| `compose_basic.svp` | Composición con relación semántica y patrón declarados |
| `transition_data_events.svp` | TransitionData con sucesos tipados (`event_state_literal`) e `induced_parameters` |
| `query_context_all_variants.svp` | Cobertura conjunta de las cinco variantes de `QueryContext` en operaciones `query` |

## Casos inválidos (8)

| Archivo | Qué debe rechazar | Error esperado |
|---------|-------------------|----------------|
| `bad_b_value.svp` | b=2 (debe ser ≥ 3) | E002 |
| `compose_cycle_graph.svp` | ciclo en grafo de composición declarado | E103 |
| `gate_undeclared_input.svp` | `gate` con una entrada no declarada | E006 |
| `max_keyword.svp` | Uso de `max` (no disponible en v0.1) | E210 |
| `projection_undeclared_source.svp` | proyección `resolved_to` sobre fuente no declarada | E006 |
| `supervise_undeclared_target.svp` | `supervise` con target no declarado | E006 |
| `supervise_wrong_role.svp` | supervise con célula de rol Base (debe ser Supervisor) | E211 |
| `u_coercion.svp` | `null` como literal ternario (coerción de U prohibida) | E001 |

## Qué no comprueba todavía

- Cobertura exhaustiva de los 39 errores E001–E507
- Invariantes de alternancia en trayectorias
- Cobertura exhaustiva de la CLI sobre todos los válidos e inválidos

## Resultado actual

15 de 15 tests de conformidad pasan.

La batería de smoke tests CLI se limita a tres comprobaciones de contrato externo y no sustituye la suite principal de conformidad.

---

*Lenguaje de computación del Sistema Vectorial SV.*  
*Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351 | CC BY-NC-ND 4.0 | ISSN 2695-6411*
