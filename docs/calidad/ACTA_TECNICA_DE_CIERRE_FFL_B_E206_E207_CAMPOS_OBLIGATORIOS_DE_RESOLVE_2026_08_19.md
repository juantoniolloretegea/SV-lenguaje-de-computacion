# Acta técnica de cierre FFL-B — E206/E207 y campos obligatorios de resolve

**Fecha:** 19/08/2026
**Hora (Europe/Madrid):** NO_CONSTA
**Estado:** CERRADO
**Ámbito:** Lenguaje SV / FFL-B / superficie de `resolve` / diagnósticos efectivos E206-E207
**Base técnica:** gramática superficial mínima v0.1, IR canónica v0.2 y contrato efectivo bajo Vía B

## 1. Hecho

Quedan alcanzables, exclusivamente en el análisis sintáctico de `resolve`, dos diagnósticos efectivos ya presentes en el catálogo:

- `E206 — ResolveMissingContext`, ante la ausencia acreditada del campo obligatorio `context`;
- `E207 — ResolveMissingMechanism`, ante la ausencia acreditada del campo obligatorio `mechanism` una vez reconocido `context`.

El lote funcional correspondiente está integrado en `main` desde el commit `02dc7c4ec2d171c994a3a9e937bed5d43021cde8`. Esta acta no modifica código.

## 2. Fundamento

La producción superficial de `resolve` exige, en este orden, los huecos `with:`, `context:` y `mechanism:`. La IR v0.2 formula, en J1.6 y en `E108 — MissingResContext`, obligaciones de contexto de resolución que exceden la mera presencia léxica de esos campos.

Bajo Vía B, la especialización diagnóstica se limita a hacer distinguible la ausencia acreditada de cada campo obligatorio. No crea tipos `Context` ni `Mechanism`, no compara identificadores con `ResSpec` y no ejecuta `resolve`.

El identificador canónico `E206` continúa denotando `EdgeConnectorMismatch`. Esa obligación permanece protegida funcionalmente por `E113`. La coincidencia numérica con el `E206` efectivo es una divergencia semántica ya registrada.

## 3. Frontera diagnóstica

`E206` se emite cuando, reconocidos `resolve(U, with: <id>`, puede determinarse que el campo obligatorio `context` no ocupa su posición.

`E207` se emite cuando `context:` y su identificador han sido reconocidos y puede determinarse que el campo obligatorio `mechanism` no ocupa su posición.

Si faltan ambos campos, se emite `E206` por la primera obligación ausente. El orden `mechanism` antes de `context` se diagnostica igualmente como `E206`.

## 4. Precedencia respecto de E001

Permanece en `E001` la puntuación o la forma dañada que no acredita por sí sola la ausencia del campo:

- identificador ausente tras `context:` o `mechanism:`;
- dos puntos ausentes o malformados;
- coma obligatoria ausente cuando el campo esperado está presente;
- coma duplicada u otra puntuación estructural malformada;
- token inesperado que no representa el campo siguiente ni el cierre de la invocación.

Un `res_spec` incompleto continúa por su ruta actual y no recibe `E206` ni `E207`. Un `resolve` válido existente permanece válido.

## 5. Relación con E108 y J1.6

La presencia sintáctica de `context` y `mechanism` queda protegida. `E108` canónico permanece en estado funcional `PARCIAL`. J1.6 no se cierra.

Este cierre no acredita:

- semántica material de contexto o mecanismo;
- tipado de `Context` o `Mechanism`;
- igualdad entre los identificadores de `ResSpec` y los usados por `resolve`;
- ejecución de `resolve`;
- resolución automática de `U`;
- materialización canónica de `E108`.

## 6. Evidencia

Una verificación independiente en modo de solo lectura, sobre el lote funcional integrado en `02dc7c4ec2d171c994a3a9e937bed5d43021cde8`, confirmó:

- batería de conformidad: **50/50**, con 9 casos válidos y 41 inválidos;
- pruebas rápidas de la interfaz de línea de órdenes: **3/3**;
- SEC-0: **3/3**;
- `resolve_missing_context.svp`: emisión exacta de `E206`;
- `resolve_missing_mechanism.svp`: emisión exacta de `E207`;
- conservación de un `resolve` válido existente.

Las sondas de discriminación entre ausencia acreditada y puntuación malformada se utilizaron como comprobación del radio del cambio. No constituyen norma adicional del lenguaje.

## 7. Límites

El alcance queda restringido al análisis sintáctico de los campos obligatorios ya existentes en la superficie de `resolve`.

No se modifican gramática, AST, IR, validador, descenso, serialización, interfaz de línea de órdenes ni infraestructura de ejecución. No se implementa `E203`. No se abren FFL-C, FFL-D ni FFL-E.

## 8. Decisión

Documentar `E206` y `E207` efectivos como diagnósticos alcanzables de análisis sintáctico y dejar explícita su no equivalencia con el cierre de `E108` o de J1.6.

## 9. Estado

Cerrado para este objeto. FFL-B permanece abierto. El siguiente acto previsto es la auditoría residual de cierre de FFL-B, no un microcierre automático adicional.
