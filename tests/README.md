# tests/ — Suite de conformidad DSL → IR

## Qué comprueba

Verifica mecánicamente que la cadena `.svp` → parser → validación → IR v0.2 → JSON canónico funciona correctamente. Para cada caso válido, comprueba que se produce JSON con los campos obligatorios. Para cada caso inválido, comprueba que falla con un error específico del catálogo E001–E507.

## Ejecución

```bash
python tests/run_conformance.py
```

## Casos válidos (6)

| Archivo | Qué verifica |
|---------|-------------|
| `cell_basic.svp` | Célula simple b=3, codominio, semántica, estado, evaluación |
| `gate_table.svp` | Dos células + compuerta con tabla explícita de admisibilidad |
| `resolve_projection.svp` | Resolución de U con ResolutionRecord + proyección `resolved_to` |
| `supervise_targets.svp` | Supervisión meta con CellTarget y ComposedTarget, rol Supervisor |
| `compose_basic.svp` | Composición con relación semántica y patrón declarados |
| `transition_data_events.svp` | TransitionData con sucesos tipados (event_state_literal) e induced_parameters |

## Casos inválidos (4)

| Archivo | Qué debe rechazar | Error esperado |
|---------|-------------------|----------------|
| `bad_b_value.svp` | b=2 (debe ser ≥ 3) | E002 |
| `max_keyword.svp` | Uso de `max` (no disponible en v0.1) | E210 |
| `supervise_wrong_role.svp` | supervise con célula de rol Base (debe ser Supervisor) | E205 |
| `u_coercion.svp` | `null` como literal ternario (coerción de U prohibida) | E001 |

## Qué no comprueba todavía

- Comparación bit a bit con expected JSON (pendiente de generar `.expected.json` para cada caso válido)
- Cobertura exhaustiva de los 39 errores E001–E507
- Ciclos en grafos de composición
- Invariantes de alternancia en trayectorias
- QueryContext con las cinco variantes

## Resultado actual

10 de 10 tests pasan.

---

*Lenguaje de computación del Sistema Vectorial SV.*
*Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351 | CC BY-NC-ND 4.0 | ISSN 2695-6411*
