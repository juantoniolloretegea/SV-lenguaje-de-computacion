# Acta técnica de puente entre el JSON canónico local del motor y la IR v0.2

**Fecha:** 2026-04-03

## Objeto

Fijar con claridad que el JSON canónico local producido por `SV-motor` y la IR canónica v0.2 del Lenguaje SV no son hoy el mismo artefacto.

## Hechos constatados

- La IR v0.2 conserva su contrato propio (`objects`, `operations`, `ir_version`, `grammar_version`).
- El runner Python del frente motor produce un expediente local con `programa`, `traza` y `dictamen`.
- Ambos artefactos pueden convivir, pero requieren un puente explícito si van a intercambiarse sin ambigüedad.

## Regla

No debe declararse identidad de esquema ni intercambiabilidad plena entre ambos formatos mientras no exista un adaptador formal, trazable y probado.
