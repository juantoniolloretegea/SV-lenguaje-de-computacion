# tests/adversarial/ — Semilla adversarial mínima de SEC-0B

Esta carpeta contiene una batería mínima de robustez del compilador de referencia.

Su función no es ampliar la conformidad doctrinal principal, sino tensionar de forma controlada:
- determinismo de parseo/lowering,
- rechazo limpio,
- estabilidad del error dominante,
- resistencia básica frente a entradas frontera razonables.

## Alcance

Esta batería es:
- pequeña,
- separada de `tests/run_conformance.py`,
- no normativa,
- no sustitutiva de la suite principal.

## Casos incluidos

| Archivo | Tipo | Qué tensiona |
|---------|------|--------------|
| `long_identifier_valid.svp` | válido | identificadores largos pero lícitos |
| `duplicate_name_invalid.svp` | inválido | colisión nominal y estabilidad de `E005` |
| `deep_nested_query_valid.svp` | válido | estructura declarativa moderadamente profunda con `query` |

## Ejecución

```bash
python tests/run_sec0_smoke.py
```

## Criterio

El runner SEC-0B sólo comprueba:
- que los casos válidos produzcan JSON parseable y determinista en dos ejecuciones consecutivas;
- que el caso inválido falle limpiamente con `E005`.

No mide rendimiento, no hace fuzzing y no sustituye la suite oficial de conformidad.
