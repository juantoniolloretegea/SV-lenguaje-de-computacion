# R1-3 — Unidad 3D: cobertura constituida de comprobaciones exigibles

**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte:** R1-3  
**Estado:** contrato de realización

## 1. Objeto

Esta unidad gobierna la suficiencia del conjunto de comprobaciones empleado para resolver una obligación individual `q`.

Las unidades 3A–3C ya distinguen comprobación individual, resolución de conflictos y agregación entre obligaciones. Quedaba abierto un hueco: un conjunto suministrado a la resolución podía ser materialmente incompleto aunque su contenido fuese internamente válido.

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

Cada verificador requerido deberá quedar acreditado como aplicable a la misma obligación y contexto antes de que la regla pueda considerarse constituida productivamente.

La mera existencia de varias relaciones `Applicable(V,q,C)` no convierte a todos esos verificadores en obligatorios. La aplicabilidad habilita la participación; la cobertura determina qué participaciones son materialmente exigibles.

## 3. Prohibición de cobertura vacía implícita

La ausencia de regla de cobertura no equivale a cobertura completa con conjunto vacío.

```text
CoverageRule(q) = ∅
↛ cobertura completa
```

Sin regla constituida suficiente, la cobertura positiva no queda acreditada.

Esta unidad no permitirá que una obligación agregue a `D-A` por el solo hecho de que las comprobaciones suministradas sean todas favorables si no se ha acreditado antes la cobertura exigible.

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

## 5. Resultado técnico y cobertura

La cobertura no introduce un cuarto resultado técnico y no modifica `Tri`.

La regla de cobertura cualifica la suficiencia de un resultado ya resuelto; no vuelve a resolver conflictos entre comprobaciones.

Cuando se materialice la composición completa de esta unidad deberá conservarse:

```text
resultado resuelto = D-R
→ la refutación no se borra por cobertura incompleta

resultado resuelto = D-A
+ cobertura incompleta o sin regla suficiente
→ no puede promoverse a D-A final

resultado resuelto = D-N
→ no puede promoverse por cobertura
```

La materialización exacta del paso de cobertura a resultado agregable pertenece a esta misma unidad 3D, pero no se considera cerrada hasta que la constitución productiva de la regla y ese puente estén realizados.

## 6. Constitución productiva

La regla de cobertura no puede ser fabricada durante el acto de evaluación.

Su constitución productiva deberá quedar gobernada por T-0, del mismo modo que `Req`, `Applicable` y las reglas de conflicto ya materializadas.

La propuesta deberá identificar únicamente los verificadores que pretende hacer exigibles y la referencia de la regla. Las ligaduras de obligación, forma, familia de efectos y contexto deberán derivarse de objetos ya constituidos.

Se exigirá, como mínimo:

1. referencia de regla no reutilizada indebidamente;
2. conjunto requerido no vacío;
3. ausencia de verificadores requeridos duplicados;
4. obligación existente;
5. cada verificador requerido con `Applicable(V,q,C)` previamente válido;
6. rechazo atómico de T-0 ante cualquier incumplimiento.

## 7. Frontera con reglas futuras

Esta unidad no introduce:

- mayoría;
- quórum;
- «uno de N»;
- ponderación;
- selección por orden;
- sustitución automática de un verificador requerido por otro aplicable;
- cobertura por familia sin identificar verificadores concretos.

Cualquiera de esas formas sería una regla de cobertura diferente y requeriría constitución y pruebas propias.

## 8. Frontera con reutilización histórica

La unidad 3D gobierna cobertura dentro del acto y de su ligadura constituida.

No decide todavía si un resultado previamente obtenido puede reutilizarse en otro estado, versión, contexto, antecedente o momento posterior.

Por tanto:

```text
cobertura completa ahora
↛ reutilización histórica automática
```

La vigencia y reutilización permanecen fuera de esta unidad.

## 9. Primera subunidad material

La primera subunidad de 3D materializa únicamente:

1. referencia nominal distinta para reglas de cobertura;
2. representación cerrada de una regla de conjunto requerido;
3. rechazo de conjunto requerido vacío o duplicado;
4. evaluación trazable de participantes requeridos, presentes y ausentes;
5. ausencia de regla como cobertura incompleta;
6. separación entre aplicabilidad y obligatoriedad de cobertura.

Esta primera subunidad todavía no constituye productivamente la regla por T-0 y todavía no sustituye la agregación pública de 3C. Por ello 3D permanece abierta tras su integración eventual.

## 10. Pruebas mínimas de la primera subunidad

Deberá demostrarse, como mínimo, que:

1. la regla no acepta conjunto requerido vacío;
2. un verificador requerido duplicado se rechaza;
3. ausencia de regla no produce cobertura positiva;
4. presencia de todos los requeridos produce cobertura completa;
5. ausencia de un requerido queda identificada explícitamente;
6. un participante adicional no reemplaza a un requerido ausente;
7. una regla ligada a otro contexto se rechaza;
8. un resultado de otra obligación se rechaza;
9. la evaluación de cobertura no produce `CheckResult`, `Tri`, `Permit`, autoridad ni efecto protegido;
10. T-G, T-C y T-R permanecen no productivas;
11. las regresiones de R0 y de las unidades anteriores de R1 permanecen correctas.

## 11. Estado

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
R1-3 / unidad 3D = EN REALIZACIÓN

R1-4 = NO INICIADO
R2–R4 = NO INICIADOS
Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```
