# R1-3 — Unidad 3D: cobertura constituida de comprobaciones exigibles

**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte:** R1-3  
**Estado:** realización candidata de cierre

## 1. Objeto

Esta unidad gobierna la suficiencia del conjunto de comprobaciones empleado para resolver una obligación individual `q`.

Las unidades 3A–3C distinguen comprobación individual, resolución de conflictos y agregación entre obligaciones. Permanecía abierto un hueco: un conjunto suministrado a la resolución podía ser materialmente incompleto aunque su contenido fuese internamente válido.

La unidad 3D separa de forma expresa:

```text
Applicable(V,q,C)
≠ V exigido para cubrir q

participación
≠ cobertura
```

La cobertura positiva sólo puede derivar de una regla previamente constituida.

## 2. Primera regla concreta

La primera realización admite una única forma cerrada de regla de cobertura:

```text
CoverageRule(q)
= RequiredVerifierSet({V1,...,Vn})
```

con conjunto no vacío.

Cada verificador requerido debe disponer de una relación constituida `Applicable(V,q,C)` para la misma obligación y contexto antes de que T-0 pueda constituir la regla.

La existencia de varias relaciones `Applicable(V,q,C)` no convierte por sí sola a todos esos verificadores en obligatorios. La aplicabilidad habilita la participación; la cobertura determina qué participaciones son materialmente exigibles.

## 3. Prohibición de cobertura vacía implícita

La ausencia de regla de cobertura no equivale a cobertura completa con conjunto vacío:

```text
CoverageRule(q) = ∅
↛ cobertura completa
```

Sin regla constituida suficiente, la cobertura positiva no queda acreditada.

Por tanto, un conjunto de comprobaciones favorables no puede producir `D-A` final si la cobertura exigible no está acreditada.

## 4. Participantes requeridos y participantes observados

Para un resultado resuelto se distinguen:

```text
Required(q)      = verificadores exigidos por la regla constituida
Participating(q) = verificadores cuyas comprobaciones fueron realmente resueltas
Missing(q)       = Required(q) \ Participating(q)
```

La cobertura es completa únicamente cuando existe regla constituida y:

```text
Missing(q) = ∅
```

Los verificadores participantes que no sean requeridos no sustituyen a uno ausente, no añaden peso, no forman quórum implícito y no confieren autoridad.

## 5. Constitución productiva por T-0

La regla de cobertura no puede fabricarse durante el acto de evaluación.

`RequirementProposal` puede transportar una `CoverageRuleProposal`, pero propuesta y regla constituida permanecen separadas.

T-0 rechaza la constitución cuando concurre cualquiera de estas condiciones:

1. referencia de regla de cobertura reutilizada;
2. conjunto requerido vacío;
3. un mismo verificador requerido aparece más de una vez;
4. la obligación no existe en el régimen inicial;
5. algún verificador requerido carece de `Applicable(V,q,C)` para la misma obligación y contexto.

Las ligaduras de obligación, forma, familia de efectos y contexto se derivan de objetos ya constituidos. No son parámetros libres de la regla.

La validación se completa antes de comprometer el estado de la continuidad, por lo que un rechazo de cobertura pertenece al rechazo atómico de la génesis completa.

## 6. Evaluación de cobertura

La evaluación conserva de forma explícita:

- referencia de la obligación;
- referencia de la regla, cuando existe;
- verificadores requeridos;
- verificadores participantes;
- verificadores requeridos ausentes;
- disposición `Complete` o `Incomplete`.

La disposición de cobertura no pertenece a `Tri` y no constituye un nuevo resultado `D-*`.

La ausencia de regla produce `Incomplete` y nunca cobertura positiva vacía.

## 7. Composición con los resultados técnicos

La cobertura cualifica la suficiencia de un resultado ya resuelto; no vuelve a resolver conflictos entre comprobaciones.

La composición realizada conserva:

```text
resultado resuelto = D-R
→ D-R final aunque la cobertura sea incompleta

resultado resuelto = D-A
+ cobertura completa
→ D-A final

resultado resuelto = D-A
+ cobertura incompleta o sin regla
→ D-N final

resultado resuelto = D-N
→ D-N final
```

De este modo, una ausencia de cobertura no borra una refutación conocida y tampoco puede promover una no verificabilidad.

## 8. Agregación productiva

La frontera productiva de agregación de R1-3 pasa a ser:

```text
aggregate_covered_requirement_results
```

La función realiza dos etapas cerradas:

1. reutiliza la validación estructural de 3C para exigir un resultado resuelto por cada obligación de `Req`, rechazar duplicados, obligaciones inesperadas y ligaduras ajenas;
2. cualifica cada resultado mediante su regla de cobertura antes de aplicar la precedencia inter-obligaciones.

La precedencia permanece:

```text
D-R > D-N > D-A
```

La agregación no cualificada de 3C deja de formar parte de la frontera pública. `requirements_bridge` pasa a ser un módulo interno y `aggregate_resolved_requirement_results` no se reexporta.

Así se evita la vía:

```text
ResolvedRequirementResult acreditado
→ agregación pública sin comprobar cobertura
```

## 9. Frontera con reglas futuras

Esta unidad no introduce:

- mayoría;
- quórum;
- «uno de N»;
- ponderación;
- selección por orden;
- sustitución automática de un verificador requerido por otro aplicable;
- cobertura por familia sin identificar verificadores concretos.

Cualquiera de esas formas sería una regla de cobertura diferente y requeriría constitución y pruebas propias.

## 10. Frontera con reutilización histórica

La unidad 3D gobierna cobertura dentro del acto y de su ligadura constituida.

No decide si un resultado previamente obtenido puede reutilizarse en otro estado, versión, contexto, antecedente o momento posterior.

Por tanto:

```text
cobertura completa ahora
↛ reutilización histórica automática
```

La vigencia y reutilización permanecen fuera de esta unidad.

## 11. Pruebas mínimas de cierre

La realización deberá demostrar, como mínimo, que:

1. la regla no acepta conjunto requerido vacío;
2. un verificador requerido duplicado se rechaza;
3. ausencia de regla no produce cobertura positiva;
4. presencia de todos los requeridos produce cobertura completa;
5. ausencia de un requerido queda identificada explícitamente;
6. un participante adicional no reemplaza a un requerido ausente;
7. una regla ligada a otro contexto se rechaza;
8. un resultado de otra obligación se rechaza;
9. T-0 rechaza atómicamente una regla con verificador requerido no aplicable;
10. T-0 rechaza atómicamente referencias de cobertura reutilizadas, conjuntos vacíos y requeridos duplicados;
11. resultados acreditados sin regla suficiente agregan a `D-N`, no a `D-A`;
12. cobertura completa de todas las obligaciones acreditadas agrega a `D-A`;
13. falta de un requerido impide `D-A`;
14. una refutación no se borra por ausencia de cobertura;
15. la agregación no cualificada de 3C no permanece accesible como frontera pública;
16. la cobertura no produce `Tri`, `Permit`, autoridad ni efecto protegido;
17. T-G, T-C y T-R permanecen no productivas;
18. las regresiones de R0 y de las unidades anteriores de R1 permanecen correctas.

## 12. Estado

```text
R0 = CERRADO
R1 = ABIERTO
R1-0 = CERRADO
R1-1 = CERRADO
R1-2 = CERRADO · INTEGRADO
R1-3 = ABIERTO

R1-3 / unidad 1 = CERRADA · INTEGRADA
R1-3 / unidad 2 = CERRADA · INTEGRADA
R1-3 / unidad 3A = CERRADA · INTEGRADA
R1-3 / unidad 3B = CERRADA · INTEGRADA
R1-3 / unidad 3C = CERRADA · INTEGRADA
R1-3 / unidad 3D = CANDIDATA DE CIERRE

R1-4 = NO INICIADO
R2–R4 = NO INICIADOS
Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```
