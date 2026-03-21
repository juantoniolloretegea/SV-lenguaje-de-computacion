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

## Casos válidos (8)

| Archivo | Qué verifica |
|---------|-------------|
| `cell_basic.svp` | Célula simple b=3, codominio, semántica, estado, evaluación |
| `gate_table.svp` | Dos células + compuerta con tabla explícita de admisibilidad |
| `resolve_projection.svp` | Resolución de U con ResolutionRecord + proyección `resolved_to` |
| `supervise_targets.svp` | Supervisión meta con CellTarget y ComposedTarget, rol Supervisor |
| `compose_basic.svp` | Composición con relación semántica y patrón declarados |
| `transition_data_events.svp` | TransitionData con sucesos tipados (`event_state_literal`) e `induced_parameters` |
| `query_context_all_variants.svp` | Cobertura conjunta de las cinco variantes de `QueryContext` en operaciones `query` |
| `trajectory_alternance_valid.svp` | Trayectoria mínima que respeta la alternancia constitutiva de `TrajectoryEntry` |

## Casos inválidos (23)

| Archivo | Qué debe rechazar | Error esperado |
|---------|-------------------|----------------|
| `admissibility_table_incompleta.svp` | tabla de admisibilidad que no cubre el producto cartesiano completo | E009 |
| `bad_b_value.svp` | b=2 (debe ser ≥ 3) | E002 |
| `bridge_position_fuera_de_rango.svp` | posición puente fuera de rango en `coupledspec` | E105 |
| `cellstate_vector_length_mismatch.svp` | `cellstate` con vector de longitud distinta de b² | E101 |
| `compose_cycle_graph.svp` | ciclo en grafo de composición declarado | E103 |
| `conector_mapping_incompleto.svp` | conector con `mapping` incompleto respecto del codominio fuente | E007 |
| `conector_target_no_ternario.svp` | conector cuyo destino de `mapping` no es literal ternario | E008 |
| `duplicate_identifier.svp` | redeclaración de identificador en el mismo ámbito | E005 |
| `gate_input_no_evalresult.svp` | `gate` con un `CellState` donde se esperaba `EvalResult` | E202 |
| `gate_undeclared_input.svp` | `gate` con una entrada no declarada | E006 |
| `invalid_role_literal.svp` | rol no reconocido en `cellspec` | E010 |
| `invalid_tri_literal.svp` | literal ternario no reconocido en vector de `cellstate` | E001 |
| `max_keyword.svp` | Uso de `max` (no disponible en v0.1) | E210 |
| `projection_undeclared_source.svp` | proyección `resolved_to` sobre fuente no declarada | E006 |
| `query_context_opaco.svp` | `query` sin constructor explícito de `QueryContext` | E204 |
| `supervise_target_opaco.svp` | `supervise` sin constructor explícito de `Supervisable` | E205 |
| `supervise_undeclared_target.svp` | `supervise` con target no declarado | E006 |
| `supervise_wrong_role.svp` | supervise con célula de rol Base (debe ser Supervisor) | E211 |
| `trajectory_alternance_violation.svp` | `Trajectory` con entrada no final sin `transition` | E304 |
| `u_coercion.svp` | `null` como literal ternario (coerción de U prohibida) | E507 |

## Qué no comprueba todavía

- Cobertura exhaustiva del catálogo implementativo vigente
- Convergencia exhaustiva entre catálogo implementativo vigente e IR v0.2
- Cobertura exhaustiva de la CLI sobre todos los válidos e inválidos

## Resultado actual

31 de 31 casos de conformidad pasan.

Además, la batería de smoke tests CLI aporta 3 comprobaciones mínimas de contrato externo y la batería SEC-0 añade 3 comprobaciones de humo adicionales. Estas sondas complementarias no sustituyen la suite principal de conformidad, pero sí forman parte del estado observable actual del repositorio.

---

*Lenguaje de computación del Sistema Vectorial SV.*  
*Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351 | CC BY-NC-ND 4.0 | ISSN 2695-6411*
