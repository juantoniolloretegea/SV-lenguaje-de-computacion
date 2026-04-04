# Sondas adversariales documentadas

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
| `e102_output_semantics_ausente_contraste.svp` | `CellSpec` sin `OutputSemantics` declarada | contraste fino de la divergencia entre E102 normativo y caída actual a E006 |
| `e104_codominio_de_conector_invalido_contraste.svp` | `Connector` con destino no ternario | contraste fino del subcaso visible que hoy cae en E008 y no en E104 |
| `e106_relacion_semantica_ausente_contraste.svp` | `Graph` con `relation` ausente | contraste fino de la coincidencia formal de E106 frente a la caída actual en E006 |
| `e111_codominio_sin_orden_total_contraste.md` | nota pública sobre `max/min` y orden total | documentación explícita de la ausencia de sonda superficial propia en v0.1 |

## Criterio

Estas piezas deben leerse como materiales de verificabilidad externa mínima. Su valor no depende de que cierren todo el ABI semántico-diagnóstico, sino de que permitan a terceros observar con mayor precisión qué comprueba hoy el frontend de referencia y qué sigue reservado para fases posteriores.


## Alcance de esta ampliación

Esta segunda tanda no abre nuevos cierres funcionales del frontend. Su finalidad es dejar documentado, de forma pública y legible, el contraste fino sobre `E102`, `E104`, `E106` y `E111` antes de pasar a la familia `E201–E211`.
