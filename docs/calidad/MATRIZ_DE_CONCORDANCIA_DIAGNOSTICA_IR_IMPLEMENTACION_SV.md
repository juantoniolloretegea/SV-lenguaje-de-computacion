# Matriz de concordancia diagnóstica IR ↔ implementación del lenguaje SV

## Finalidad

Esta matriz sirve para registrar, en una sola superficie auditable, el estado de concordancia entre:

- el régimen diagnóstico de la **IR canónica v0.2**,
- el catálogo canónico público de errores,
- la implementación efectiva,
- la emisión observable del validator,
- y la cobertura real de suite.

## Criterio de lectura

Cada fila deberá corresponder a una unidad diagnóstica concreta.  
La clasificación mínima de situación será:

- `coincide`
- `mismo_id_significado_distinto`
- `solo_ir`
- `solo_implementacion`
- `regularizado_provisionalmente`
- `no_acreditado`

## Regla de uso

No se rellenarán celdas por inferencia blanda.  
Toda clasificación deberá apoyarse en lectura directa del artefacto correspondiente o en evidencia observable de suite/emisión.

## Campos de la matriz

- `ID_IR`
- `Nombre_IR`
- `ID_Catalogo_Publico`
- `Nombre_Catalogo_Publico`
- `ID_Implementacion`
- `Nombre_Implementacion`
- `Emision_Observable`
- `Cobertura_Suite`
- `Situacion`
- `Tratamiento_Recomendado`
- `Observaciones`

