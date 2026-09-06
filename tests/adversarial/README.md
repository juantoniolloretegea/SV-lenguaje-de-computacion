# tests/adversarial/ — Semilla adversarial mínima de SEC-0B

**Fecha y Versión: V.1 del conjunto**  
**Fecha:** 4 de abril de 2026  
**Versión del conjunto:** V.1 del conjunto  
**Autor del corpus:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Titularidad y autoría:** © Juan Antonio Lloret Egea, 2026. Este conjunto se distribuye con atribución explícita de autoría y bajo la licencia indicada, sin autorización para apropiación de la paternidad intelectual del Sistema Vectorial SV.  

---


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


## Sondas documentadas

La subcarpeta `documentados/` reúne piezas públicas de contraste técnico inspiradas en lecturas adversariales externas. No forman parte de la suite principal de conformidad ni deben leerse como ejemplos canónicos de iniciación. Su función es mostrar, de forma legible, qué zonas del lenguaje ya son verificables y qué zonas siguen tensionando huecos de validación o de representación superficial.


## Continuidad del ejecutor · 06/09/2026

El archivo `deep_nested_query_valid.svp` fue reclasificado y archivado por DFL-007
como antecedente de Gramática 0.1. El ejecutor utiliza ahora el caso vigente
`tests/conformance/valid/query_context_all_variants.svp` para la comprobación de
consulta. La tabla anterior conserva su carácter histórico. El determinismo se
comprueba sobre los bytes UTF-8 de ambas salidas, sin ordenar ni reserializar JSON.
La ausencia de miembros homónimos se verifica por separado.
