# R1-3 — Unidad 3E: reutilización y vigencia ligada de resultados

**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte:** R1-3  
**Estado:** abierta

## 1. Objeto

Esta unidad gobierna si un resultado técnico obtenido en un estado constituido anterior puede emplearse de nuevo como resultado válido para la misma obligación sin repetir íntegramente su comprobación.

La reutilización no se presume por identidad nominal, por proximidad cronológica ni por disponibilidad del resultado. Sólo es admisible cuando permanecen acreditadas todas las ligaduras materialmente causales de su validez.

Por tanto:

```text
resultado histórico existente
≠ resultado históricamente reutilizable
```

## 2. Punto de partida

Las unidades 3A–3D ya separan:

```text
comprobación individual
→ resolución de conflicto
→ resultado resuelto y sellado
→ cualificación de cobertura
→ agregación técnica
```

La unidad 3E no reabre esas reglas.

En particular, la reutilización histórica no podrá operar directamente sobre un `ResolvedRequirementResult` sin haber conservado antes la cualificación de cobertura exigida por 3D.

La frontera requerida es:

```text
resultado resuelto
+ cobertura constituida
→ resultado cualificado del acto
→ posible reutilización histórica
```

De este modo se impide que una acreditación intra-obligación que 3D habría degradado a `D-N` por cobertura insuficiente reaparezca posteriormente como `D-A` mediante reutilización.

## 3. Regla de cierre para la reutilización

Para una obligación constituida `q`, la reutilización sólo podrá preservar el resultado anterior cuando el resultado histórico y el estado constituido actual mantengan exactamente las ligaduras relevantes exigidas por una regla previamente constituida.

La primera realización adoptará una regla cerrada de la forma:

```text
ReuseRule(q)
= ExactBindingSet({b1,...,bn})
```

con conjunto no vacío.

La ausencia de `ReuseRule(q)` no equivale a una regla con conjunto vacío:

```text
ReuseRule(q) = ∅
↛ reutilización libre
```

Sin regla constituida suficiente, el resultado previo no es reutilizable.

## 4. Ligaduras históricas

Las ligaduras de reutilización representan hechos o referencias constituidas cuya variación puede alterar la validez material del resultado.

Cuando sean causales, deberán poder representar, entre otras, las dimensiones siguientes:

- objeto gobernado;
- operación o familia de operaciones;
- contexto constitutivo;
- versión o régimen aplicable;
- antecedente de autoridad;
- evidencia utilizada;
- regla de verificación;
- condición de vigencia o no revocación;
- cualquier otra ligadura específica cuya variación pueda modificar el resultado.

Las dimensiones ya selladas por `RequirementDescriptor` y por las unidades 3A–3D continúan formando parte de la identidad material. La regla de reutilización no las sustituye ni las debilita.

## 5. Constitución y elección de la regla

La regla de reutilización deberá quedar ligada al `RequirementDescriptor` durante una transición constitutiva legítima. En el estado material actualmente disponible de R1, la única puerta productiva sigue siendo T-0.

El acto de reutilización no podrá:

- escoger una regla alternativa;
- omitir una regla constituida;
- sustituir una ligadura por otra;
- declarar irrelevante una ligadura porque favorezca el resultado;
- fabricar vigencia por mera afirmación.

La propuesta de una regla no equivale a su constitución.

## 6. Vigencia sin tiempo implícito

La vigencia no introduce un reloj semántico ni una fecha privilegiada en `sv_core`.

No se admite ninguna regla implícita de la forma:

```text
más reciente = vigente
último resultado = preferente
fecha posterior = resultado superior
```

Cuando una versión, régimen, autorización, antecedente o condición de validez cambie con consecuencias materiales, ese cambio deberá quedar representado mediante una ligadura constituida distinta.

Por tanto:

```text
vigencia
= continuidad acreditada de ligaduras explícitas
≠ paso del tiempo
```

La fecha o el tiempo de un dominio externo podrán formar parte de evidencia situada cuando una regla de dominio lo exija, pero no constituyen una primitiva universal de esta unidad.

## 7. Resultado de la evaluación de reutilización

Para la misma obligación `q`:

```text
resultado histórico cualificado
+ ReuseRule constituida
+ todas las ligaduras exigidas continúan
→ se conserva el resultado histórico
```

Por tanto:

```text
D-A reutilizable → D-A
D-R reutilizable → D-R
D-N reutilizable → D-N
```

La reutilización nunca promociona un resultado.

Si falta la regla, falta una ligadura exigida, una ligadura ha cambiado o su continuidad no puede acreditarse:

```text
resultado previo = no reutilizable
Check(q) = D-N
```

El resultado histórico original no se reescribe ni se borra; simplemente deja de ser admisible como sustituto de una nueva comprobación bajo el estado actual.

## 8. Obligación distinta

Un resultado de `q1` no puede reutilizarse como resultado de `q2` aunque sus restantes ligaduras coincidan.

```text
q1 ≠ q2
→ no existe reutilización entre obligaciones
```

La presentación de un resultado histórico de otra obligación constituye una incompatibilidad estructural, no una forma de `D-N` dentro de la obligación actual.

## 9. Interacción con cobertura

La unidad 3E deberá conservar el resultado ya cualificado por 3D.

No podrá reutilizar directamente un `D-A` resuelto si la cobertura histórica correspondiente era incompleta.

En particular:

```text
D-A resuelto
+ cobertura incompleta
→ D-N cualificado
→ sólo D-N puede ser candidato histórico
```

La reutilización no vuelve a calcular mayorías, conflictos ni cobertura. Sólo decide si el resultado cualificado previamente puede seguir sustituyendo una nueva comprobación.

## 10. Interacción con revocación

R1-3 no materializa persistencia durable de revocaciones ni continuidad entre procesos.

En esta unidad, una revocación o pérdida de vigencia sólo afecta a la reutilización cuando queda representada por una variación de una ligadura constituida relevante.

Por tanto:

```text
cambio de condición de vigencia
→ ligadura distinta
→ resultado histórico no reutilizable
→ D-N
```

Esta regla no produce por sí misma una transición de revocación ni ejecuta efectos protegidos.

## 11. Reintento

Un reintento no transforma por sí solo un resultado anterior:

```text
D-N histórico + reintento
↛ D-A
```

Si se realiza una nueva comprobación completa bajo el estado actual, su resultado pertenece al nuevo acto y deberá atravesar de nuevo las reglas 3A–3D. No es una promoción del resultado histórico.

## 12. Frontera productiva

La unidad deberá impedir las vías siguientes:

```text
ResolvedRequirementResult
→ reutilización directa sin cobertura

resultado histórico
→ D-A por coincidencia nominal de RequirementRef

misma regla nominal + ligaduras distintas
→ reutilización

ausencia de ReuseRule
→ reutilización positiva

cronología
→ vigencia implícita
```

La API pública de reutilización deberá recibir el resultado histórico ya sellado y el `RequirementDescriptor` constituido actual. La regla y las ligaduras actuales deberán obtenerse del descriptor, no de parámetros libres suministrados para el acto.

## 13. Pruebas mínimas de cierre

La realización deberá demostrar, como mínimo, que:

1. una regla de reutilización no acepta un conjunto vacío de ligaduras;
2. una ligadura repetida se rechaza;
3. una referencia de regla reutilizada de forma incompatible se rechaza durante la constitución;
4. la regla queda ligada a la obligación, forma, familia de efectos y contexto constituidos;
5. la regla no puede fabricarse fuera de la puerta constitutiva;
6. ausencia de regla impide la reutilización positiva;
7. cambio de una sola ligadura material impide la reutilización;
8. continuidad exacta de todas las ligaduras conserva `D-A`;
9. continuidad exacta conserva `D-R`;
10. continuidad exacta conserva `D-N` sin promocionarlo;
11. un resultado de otra obligación se rechaza estructuralmente;
12. una variación de contexto impide reutilización;
13. una variación de regla de aplicabilidad, conflicto o cobertura impide reutilización;
14. una variación de condición de vigencia impide reutilización;
15. no existe selección por fecha, orden de llegada o «último resultado»;
16. un resultado sin cobertura suficiente no puede reaparecer como `D-A` histórico;
17. la reutilización no produce `Tri`, `Permit`, autoridad ni efecto protegido;
18. T-G, T-C y T-R permanecen no productivas;
19. no se introduce reloj, ejecución asíncrona, dependencia externa ni estado durable;
20. las regresiones de R0 y de las unidades 1, 2 y 3A–3D permanecen correctas.

## 14. Exclusiones

Esta unidad no materializa:

- persistencia durable de resultados;
- almacenamiento histórico externo;
- revocación durable;
- continuidad entre procesos;
- orden temporal canónico;
- reloj semántico;
- caducidad automática por fecha;
- `Permit`;
- mediación de efectos protegidos;
- R1-4;
- BudgetΣ;
- Garantía I;
- Garantía II.

## 15. Estado

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
R1-3 / unidad 3D = CERRADA · INTEGRADA
R1-3 / unidad 3E = ABIERTA

R1-4 = NO INICIADO
R2–R4 = NO INICIADOS
BudgetΣ = NO ABIERTO
Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```
