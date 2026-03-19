# Contraste fino de E111 — codominio sin orden total explícito

## Estatuto

Esta nota pública documenta el contraste fino de `E111` dentro de la familia `E101–E111`.

## Hallazgo

La IR v0.2 y el catálogo público efectivo coinciden en `E111 — UnorderedCodomain`.

Sin embargo, en la superficie v0.1 actualmente publicada no existe todavía una sonda `.svp` propia y mínima que active ese caso de forma directa y verificable, porque el lowering que exige orden total explícito para `max/min` sigue sin estar expuesto superficialmente.

## Consecuencia operativa

- `E111` mantiene coincidencia semántica formal;
- no dispone todavía de cobertura explícita de suite;
- y queda documentado, en esta fase, como coincidencia ancla sin sonda superficial propia.
