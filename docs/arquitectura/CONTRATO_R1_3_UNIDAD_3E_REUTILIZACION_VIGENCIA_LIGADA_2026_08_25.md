# R1-3 — Unidad 3E: reutilización y vigencia ligada de resultados

**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte:** R1-3  
**Estado:** realización candidata de cierre

## 1. Objeto

Esta unidad gobierna si un resultado técnico obtenido en un estado constituido anterior puede emplearse de nuevo como resultado válido para la misma obligación sin repetir íntegramente su comprobación.

La reutilización no se presume por identidad nominal, proximidad cronológica ni mera disponibilidad del resultado. Sólo es admisible cuando el resultado histórico ya fue cualificado por la cobertura aplicable y continúan acreditadas todas las ligaduras materialmente causales de su validez.

Por tanto:

```text
resultado histórico existente
≠ resultado históricamente reutilizable
```

## 2. Punto de partida y frontera

Las unidades 3A–3D separan:

```text
comprobación individual
→ resolución de conflicto
→ resultado resuelto y sellado
→ cualificación de cobertura
→ agregación técnica
```

La unidad 3E no reabre esas reglas.

En particular, la reutilización histórica no opera directamente sobre un `ResolvedRequirementResult`. La frontera realizada es:

```text
RequirementCheck[]
→ resolución 3A / 3B
→ ResolvedRequirementResult
→ cualificación de cobertura 3D
→ HistoricalQualifiedRequirementResult
→ evaluación de reutilización 3E
```

`HistoricalQualifiedRequirementResult` carece de constructor público. Su formación vuelve a validar la ligadura completa del resultado resuelto frente al `RequirementDescriptor` constituido y conserva el resultado posterior a la cualificación de cobertura.

De este modo:

```text
D-A resuelto
+ cobertura incompleta
→ D-N cualificado
→ resultado histórico = D-N
```

y una acreditación insuficientemente cubierta no puede reaparecer posteriormente como `D-A` mediante reutilización.

## 3. Regla constituida de reutilización

Para una obligación constituida `q`, la primera realización materializa:

```text
ReuseRule(q)
= ExactBindingSet({(k1,v1),...,(kn,vn)})
```

con conjunto no vacío.

Cada `ki` identifica una dimensión de continuidad y cada `vi` el valor constituido que esa dimensión debe conservar. Clave y valor emplean tipos nominales distintos:

```text
ReuseBindingKeyRef
≠ ReuseBindingValueRef
```

Por tanto, cambiar el valor de una dimensión no puede confundirse con declarar una dimensión diferente.

La ausencia de regla no equivale a una regla con conjunto vacío:

```text
ReuseRule(q) = ∅
↛ reutilización libre
```

Sin regla constituida suficiente, el resultado histórico no es reutilizable.

## 4. Constitución por T-0

`RequirementProposal` puede transportar una `ReuseRuleProposal`, formada por una referencia de regla y una colección de `ReuseBindingProposal`.

Propuesta y regla constituida permanecen separadas. La conversión productiva exige la capacidad interna de T-0 y liga la regla al `RequirementDescriptor` correspondiente.

T-0 rechaza de forma atómica:

1. una `ReuseRuleRef` reutilizada entre obligaciones;
2. un conjunto de ligaduras vacío;
3. una misma `ReuseBindingKeyRef` repetida dentro de la regla.

La regla constituida deriva del descriptor, y por tanto congela además:

- obligación;
- forma;
- familia de efectos;
- contexto constitutivo.

El acto de reutilización no recibe una regla elegible por el llamador, no puede omitir la regla constituida ni sustituirla por otra.

## 5. Ligaduras históricas

Las ligaduras de reutilización representan hechos o referencias constituidas cuya variación puede alterar la validez material del resultado.

Cuando sean causales, pueden representar, entre otras, las dimensiones siguientes:

- objeto gobernado;
- operación o familia de operaciones;
- contexto constitutivo;
- versión o régimen aplicable;
- antecedente de autoridad;
- evidencia utilizada;
- regla de verificación;
- condición de vigencia o no revocación;
- cualquier otra ligadura específica cuya variación pueda modificar el resultado.

La regla no interpreta esos valores como texto, fecha o puntuación. Son referencias opacas del dominio de control y su semántica concreta debe proceder del régimen que las constituya.

Las dimensiones ya gobernadas por `RequirementDescriptor` y por 3A–3D continúan formando parte de la identidad material y no pueden ser sustituidas por el conjunto adicional de reutilización.

## 6. Sellado material del resultado

La reutilización histórica no se apoya sólo en referencias nominales de reglas.

`ResolvedRequirementResult` conserva materialmente:

- obligación y clase;
- forma;
- familia de efectos;
- contexto;
- familias admisibles de verificadores;
- regla de aplicabilidad;
- contenido material de la regla de conflicto, cuando existe;
- contenido material de la regla de cobertura, cuando existe;
- contenido material de la regla de reutilización, cuando existe;
- verificadores participantes;
- resultado técnico resuelto.

En particular, el sello conserva:

```text
ConflictResolutionRule
→ referencia
+ verificador decisivo
+ familia del verificador
+ regla de aplicabilidad

CoverageRule
→ referencia
+ conjunto requerido de verificadores

ReuseRule
→ referencia
+ mapa exacto de ligaduras
```

Por tanto:

```text
misma referencia nominal de regla
+ contenido material distinto
→ ligadura distinta
```

Esto impide tratar como continuidad válida un cambio de regla encubierto bajo el mismo identificador.

## 7. Vigencia sin tiempo implícito

La vigencia no introduce un reloj semántico ni una fecha privilegiada en `sv_core`.

No se admite ninguna regla implícita de la forma:

```text
más reciente = vigente
último resultado = preferente
fecha posterior = resultado superior
```

Cuando una versión, régimen, autorización, antecedente o condición de validez cambie con consecuencias materiales, ese cambio debe quedar representado mediante una ligadura constituida distinta.

Por tanto:

```text
vigencia
= continuidad acreditada de ligaduras explícitas
≠ paso del tiempo
```

La fecha o el tiempo de un dominio externo pueden formar parte de evidencia situada cuando una regla de dominio lo exija, pero no constituyen una primitiva universal de esta unidad.

## 8. Formación del resultado histórico cualificado

`seal_historical_qualified_result` recibe únicamente:

- el `RequirementDescriptor` constituido del acto;
- el `ResolvedRequirementResult` ya formado por la vía gobernada.

Antes de formar el sello histórico:

1. comprueba que corresponde a la misma obligación;
2. verifica la ligadura material completa del resultado resuelto contra el descriptor;
3. evalúa la cobertura constituida;
4. guarda exclusivamente el resultado ya cualificado por cobertura.

La operación conserva también los verificadores participantes y una instantánea material de las reglas gobernadas relevantes.

No produce persistencia durable ni acredita continuidad entre procesos. Sólo forma el objeto cerrado que puede ser presentado posteriormente a una evaluación de reutilización.

## 9. Evaluación de reutilización

`reuse_historical_requirement_result` recibe únicamente:

- el `RequirementDescriptor` constituido actual;
- un `HistoricalQualifiedRequirementResult` sellado.

La regla y las ligaduras actuales se obtienen del descriptor. No existen parámetros de tiempo, regla alternativa, vigencia libre o preferencia aportados por el llamador.

Para la misma obligación `q`:

```text
resultado histórico cualificado
+ ReuseRule histórica
+ ReuseRule actual
+ ligadura material base idéntica
+ conjunto exacto de ligaduras idéntico
→ resultado reutilizable
```

La continuidad exacta conserva, sin promoción:

```text
D-A → D-A
D-R → D-R
D-N → D-N
```

La reutilización no vuelve a resolver conflictos ni recalcula mayorías. Tampoco transforma una cobertura incompleta en completa.

## 10. Fallo cerrado de reutilización

La reutilización no positiva produce `D-N` para la obligación del acto actual cuando:

- el resultado histórico no estaba ligado a una regla de reutilización;
- el descriptor actual carece de regla de reutilización;
- cambia una dimensión material base;
- cambia el contenido de una regla gobernada relevante;
- cambia una clave o un valor del conjunto exacto de reutilización;
- no puede acreditarse la continuidad exigida.

Por tanto:

```text
resultado histórico no reutilizable
→ Check(q) = D-N
```

El resultado histórico original no se modifica ni se borra.

Un resultado histórico de otra obligación constituye, en cambio, una incompatibilidad estructural:

```text
q1 ≠ q2
→ error estructural
```

No se degrada silenciosamente una obligación ajena a `D-N` de la obligación actual.

## 11. Interacción con cambios de regla

Una referencia nominal idéntica no basta para acreditar continuidad.

Quedan cerrados, entre otros, los casos:

```text
misma ConflictResolutionRuleRef
+ distinto verificador decisivo
→ no reutilizable

misma CoverageRuleRef
+ distinto conjunto requerido
→ no reutilizable

misma ReuseRuleRef
+ distinto valor para una clave
→ no reutilizable
```

También una variación de clase, forma, familia de efectos, contexto, familias admisibles de verificadores o regla de aplicabilidad impide la reutilización positiva.

## 12. Revocación y reintento

R1-3 no materializa persistencia durable de revocaciones ni continuidad entre procesos.

En esta unidad, una revocación o pérdida de vigencia afecta a la reutilización únicamente cuando queda representada por una variación de una ligadura constituida relevante:

```text
cambio de condición de vigencia
→ ligadura distinta
→ resultado histórico no reutilizable
→ D-N
```

La regla no produce por sí misma una transición de revocación ni ejecuta efectos protegidos.

Un reintento tampoco promociona un resultado:

```text
D-N histórico + reintento
↛ D-A
```

Si se realiza una nueva comprobación bajo el estado actual, su resultado pertenece a un nuevo acto y debe atravesar de nuevo 3A–3D.

## 13. Frontera productiva

La realización impide las vías siguientes:

```text
ResolvedRequirementResult
→ reutilización histórica directa

resultado histórico
→ D-A por coincidencia nominal de RequirementRef

misma referencia nominal de regla
+ contenido distinto
→ continuidad falsa

ausencia de ReuseRule
→ reutilización positiva

cronología
→ vigencia implícita
```

`HistoricalQualifiedRequirementResult`, `ReuseRule` y las instantáneas materiales de reglas no producen por sí mismos `Tri`, `Permit`, autoridad o efecto protegido.

## 14. Evidencia de realización

La batería de esta unidad cubre, además de las regresiones anteriores:

1. rechazo de regla de reutilización con conjunto vacío;
2. rechazo de clave de ligadura duplicada;
3. constitución positiva de una regla mediante `AuthorityContinuity::apply_genesis`;
4. rechazo atómico por T-0 de conjunto vacío;
5. rechazo atómico por T-0 de clave duplicada;
6. rechazo atómico por T-0 de `ReuseRuleRef` reutilizada entre obligaciones;
7. imposibilidad de fabricar el resultado histórico desde un `CheckResult` nominal;
8. imposibilidad de pasar un `ResolvedRequirementResult` directamente a la API de reutilización;
9. cobertura incompleta sellada como `D-N` y conservada como `D-N` al reutilizar;
10. continuidad exacta que conserva `D-A`;
11. continuidad exacta que conserva `D-R`;
12. continuidad exacta que conserva `D-N`;
13. ausencia de regla actual que cierra en `D-N`;
14. cambio de una sola ligadura que cierra en `D-N`;
15. misma referencia de regla con contenido de ligadura distinto que cierra en `D-N`;
16. cambio de contexto que impide reutilización;
17. resultado de otra obligación rechazado estructuralmente;
18. conservación de las regresiones de R0 y de 3A–3D.

Los rechazos T-0 verifican atomicidad de la génesis: continuidad no habitada, premisa no consumida, T-0 disponible y ausencia de estado parcial comprometido.

## 15. Exclusiones

Esta unidad no materializa:

- persistencia durable de resultados;
- almacenamiento histórico externo;
- revocación durable;
- continuidad material entre procesos;
- orden temporal canónico;
- reloj semántico;
- caducidad automática por fecha;
- `Permit`;
- mediación de efectos protegidos;
- R1-4;
- BudgetΣ;
- Garantía I;
- Garantía II.

T-G, T-C y T-R permanecen no productivas.

## 16. Estado

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
R1-3 / unidad 3E = CANDIDATA DE CIERRE

R1-4 = NO INICIADO
R2–R4 = NO INICIADOS
BudgetΣ = NO ABIERTO
Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```
