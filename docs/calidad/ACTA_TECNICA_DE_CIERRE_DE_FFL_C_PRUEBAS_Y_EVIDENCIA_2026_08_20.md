# Acta técnica de cierre de FFL-C — Pruebas y evidencia

**Fecha:** 20/08/2026  
**Hora (Europe/Madrid):** 08:16:26  
**Estado:** CERRADO  
**Ámbito:** Lenguaje SV / FFL-C / pruebas y evidencia

## 1. Hecho

FFL-C fue abierto el 20/08/2026 para comprobar si la evidencia reproducible disponible sostenía las afirmaciones técnicas ya publicadas y para conservar las comprobaciones necesarias dentro de `tests/`, sin modificar la implementación del lenguaje.

Durante el bloque se incorporaron:

- un caso válido permanente para `SystemTarget(CompositionGraph)`;
- una caracterización específica de las dos situaciones observables que actualmente emiten E006;
- un inventario de cobertura que distingue entre caso persistido, emisión alcanzable y propiedad preservada por la forma del lenguaje.

No se modificaron `src/`, la gramática superficial mínima, el AST, la IR v0.2, el validador, el catálogo diagnóstico ni `docs/manual_svp/`.

## 2. Evidencia reproducible

Una verificación independiente en modo de solo lectura se realizó sobre el estado exacto:

`3d48c422915b0e0bed65ba2e7ce8b807d7a94c33`

El árbol de trabajo permaneció sin modificaciones antes y después de la ejecución. Los cuatro ejecutores terminaron con código de retorno 0 y produjeron los siguientes resultados:

| Comprobación | Resultado |
|---|---:|
| Conformidad | **58/58** — 10 casos válidos + 48 inválidos |
| Pruebas rápidas de la interfaz de línea de órdenes | **3/3** |
| SEC-0 | **3/3** |
| Caracterización de E006 | **4/4** |

No se observaron divergencias de resultado.

## 3. Cobertura observable

Los 48 casos inválidos cubren de forma explícita 37 de los 47 códigos del catálogo efectivo. Los diez códigos restantes carecen de un caso `.svp` inválido directo de extremo a extremo por motivos distintos que deben conservarse separados:

- `E003 — NSquaredViolation`;
- `E004 — EmptyCodomain`;
- `E008 — ConnectorTargetNotTri`;
- `E106 — MissingSemanticRelation`;
- `E111 — UnorderedCodomain`;
- `E201 — LiteralResultForbidden`;
- `E203 — GateMissingTable`;
- `E301 — FrameMutationForbidden`;
- `E302 — TrajectoryMutationForbidden`;
- `E501 — SerializationNonDeterministic`.

La ausencia de un caso inválido directo no se interpreta de forma uniforme como falta de cobertura. Según el código, responde a inalcanzabilidad desde la superficie vigente, protección por otra ruta diagnóstica o preservación estructural de la propiedad. No procede ampliar artificialmente el lenguaje para fabricar una emisión.

La lectura de cobertura queda, por tanto, separada en tres niveles:

1. **caso persistido:** existe una entrada concreta y una expectativa verificable;
2. **emisión observable:** el procesamiento completo puede alcanzar el diagnóstico indicado;
3. **propiedad estructural:** la superficie vigente impide la operación que vulneraría la obligación, aunque no exista una emisión diagnóstica directa.

## 4. E006

La comprobación específica acredita que E006 se emite actualmente tanto ante una referencia inexistente como ante una referencia existente de tipo incompatible. La batería distingue ambas situaciones y verifica cuatro casos.

Esta evidencia no modifica el código, el nombre, el mensaje ni el significado declarado del diagnóstico. Permanece como deuda de precisión la adecuación del nombre y del mensaje base de `E006 — UndeclaredReference` al segundo supuesto.

## 5. Límites

El cierre de FFL-C acredita evidencia de la etapa frontal del compilador y de sus contratos observables. No acredita una infraestructura de ejecución general ni materializa capacidades que permanecen fuera de la superficie vigente, entre ellas:

- ejecución de `GateResult.output`;
- semántica ejecutiva completa de `SupervisionResult`, `verdict` o `Veto`;
- `ConflictOperator` para la concurrencia del régimen `General`;
- producción material de `CriticalityResult`;
- procedencia completa de `CoupledState` desde un `Connector` concreto;
- suficiencia reconstructiva completa de `TransitionData`.

Estas limitaciones continúan registradas como deuda técnica y no invalidan la evidencia acreditada en FFL-C.

## 6. Decisión

Se considera satisfecho el criterio de cierre de FFL-C: la evidencia reproducible sostiene el estado técnico declarado, las ausencias relevantes de cobertura están identificadas y las comprobaciones necesarias se encuentran persistidas o delimitadas de forma expresa.

Se decide:

- cerrar FFL-C;
- mantener FFL-A y FFL-B cerrados;
- mantener FFL-D y FFL-E pendientes;
- no abrir automáticamente ningún bloque posterior por efecto de este cierre.

## 7. Estado

**FFL-C: cerrado.**  
**FFL-D: pendiente.**  
**FFL-E: pendiente.**
