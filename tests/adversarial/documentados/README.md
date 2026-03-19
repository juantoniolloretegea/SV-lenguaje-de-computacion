# Sondas adversariales documentadas

Esta carpeta reúne piezas públicas de contraste técnico.

## Estatuto

- no son ejemplos canónicos de iniciación;
- no sustituyen la suite de conformidad;
- no introducen autoridad normativa nueva;
- y pueden servir tanto para confirmar cierres parciales como para exponer huecos de validación todavía abiertos.

## Piezas incluidas

| Archivo | Foco principal | Lectura recomendada |
|---------|----------------|---------------------|
| `celula_aislada_con_resolucion_u.svp` | cadena `res_spec` / `resolve` / proyección | sonda de referencias opacas y resolución de `U` |
| `composicion_serie_con_trayectoria.svp` | `Trajectory` con dos frames y una transición | sonda de alternancia constitutiva y N3 |
| `agente_con_consulta_y_dominio.svp` | `Domain` / `Agent` / `QuerySpec` | sonda de N4 y de validación de campos opacos |

## Criterio

Estas piezas deben leerse como materiales de verificabilidad externa mínima. Su valor no depende de que cierren todo el ABI semántico-diagnóstico, sino de que permitan a terceros observar con mayor precisión qué comprueba hoy el frontend de referencia y qué sigue reservado para fases posteriores.
