# R1-3 — Unidad 3C: puente sellado entre resolución y agregación

**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte:** R1-3  
**Estado:** contrato de realización

## 1. Objeto

Esta unidad materializa el puente entre la resolución gobernada de las comprobaciones suministradas para una obligación individual y la agregación técnica del conjunto completo de obligaciones `Req(F,e | C)`.

Las unidades 3A y 3B ya determinan el resultado técnico de una obligación `q` a partir del conjunto de comprobaciones que recibe la resolución. La agregación entre obligaciones no debe poder sustituir ese resultado resuelto por una `RequirementCheck` individual seleccionada localmente.

La composición de esta unidad es:

```text
RequirementCheck(q,V1), ..., RequirementCheck(q,Vn)
→ resolución gobernada del conjunto suministrado para q
→ ResolvedRequirementResult(q)
→ agregación sobre Req(F,e | C)
```

Esta unidad no acredita que el conjunto de comprobaciones suministrado para `q` sea exhaustivo respecto de todas las comprobaciones o evidencias materialmente exigibles. Esa propiedad pertenece a la cobertura posterior de R1-3.

La unidad no produce `Permit`, no ejecuta efectos protegidos, no modifica `Tri` y no abre R1-4.

## 2. Resultado resuelto sellado

`ResolvedRequirementResult` representa el resultado técnico obtenido tras resolver gobernadamente el conjunto de comprobaciones suministrado para una obligación constituida.

No es un alias de `CheckResult` ni una comprobación elegida entre varias.

```text
CheckResult nominal
≠ RequirementCheck
≠ ResolvedRequirementResult
```

El tipo no ofrece constructor público. Sólo la vía de resolución gobernada puede producirlo a partir de un `RequirementDescriptor` constituido y de comprobaciones `RequirementCheck` selladas.

## 3. Ligadura material y participación

El resultado resuelto conserva las dimensiones del descriptor cuya variación puede cambiar su interpretación o validez dentro de este corte:

- identidad de la obligación;
- clase de obligación;
- forma constituida;
- familia de efectos;
- contexto constitutivo;
- familias de verificadores admisibles;
- regla de aplicabilidad;
- identidad de la regla de resolución de conflicto, cuando exista.

Además conserva el conjunto de verificadores cuyas comprobaciones participaron efectivamente en la resolución.

```text
verificador participante
≠ cobertura acreditada
```

La identidad de los participantes proporciona trazabilidad para la cobertura posterior, pero no demuestra por sí misma que no existan otras comprobaciones materialmente exigibles.

La agregación vuelve a contrastar la ligadura del resultado resuelto contra el `RequirementDescriptor` contenido en el `RequirementSet` recibido.

Una coincidencia meramente nominal de `RequirementRef` no basta para reutilizar un resultado con otra ligadura.

## 4. Vía de formación

La formación sigue obligatoriamente esta secuencia:

```text
RequirementDescriptor
+ RequirementCheck[] suministradas
→ resolve_requirement_checks(...)
→ CheckResult gobernado
→ ResolvedRequirementResult sellado
```

Queda excluido:

```text
CheckResult
→ ResolvedRequirementResult
```

La unidad también impide que una `RequirementCheck` individual entre directamente en la agregación inter-obligaciones y sustituya al objeto resuelto.

Esta restricción no equivale a afirmar que la unidad detecte por sí sola toda comprobación omitida antes de llamar a la resolución. Determinar qué comprobaciones o evidencias debían estar presentes pertenece a la cobertura posterior de R1-3.

## 5. Conflicto

El puente no introduce una regla nueva de conflicto.

Conserva exactamente las reglas ya materializadas:

```text
conflicto(q) + ausencia de regla constituida
→ D-N

conflicto(q) + regla constituida y aplicable
→ resultado gobernado por esa regla
```

El resultado obtenido se sella después de resolver el conjunto suministrado y sólo entonces puede entrar en la agregación entre obligaciones.

## 6. Agregación cerrada entre obligaciones

La agregación productiva de esta unidad acepta exclusivamente resultados resueltos sellados, uno por obligación constituida del `RequirementSet`.

Debe comprobar:

1. conjunto `Req` no vacío;
2. ninguna obligación inesperada;
3. ninguna obligación repetida;
4. ligadura material exacta;
5. presencia de un resultado resuelto para cada obligación constituida de `Req`.

Esta cobertura es cobertura del conjunto de obligaciones de `Req`; no acredita exhaustividad interna de las comprobaciones empleadas para resolver cada obligación.

La precedencia permanece:

```text
D-R  si existe al menos una obligación REFUTADA;
D-N  si ninguna está REFUTADA y existe al menos una NO_VERIFICABLE;
D-A  sólo si todas están ACREDITADAS.
```

La resolución intra-obligación y la agregación inter-obligaciones son operaciones distintas y no deben fusionarse mediante selección implícita.

## 7. Cierre de la vía anterior

La función de agregación de la unidad 1 que aceptaba directamente una `RequirementCheck` por obligación fue una superficie transitoria anterior a la materialización del conflicto.

Tras 3A y 3B no puede permanecer como vía pública productiva, porque permitiría presentar una comprobación individual de `q` directamente a la agregación sin atravesar el objeto resuelto de la obligación.

Por tanto, la vía directa `RequirementCheck → agregación` queda restringida a pruebas internas de regresión y deja de formar parte de la frontera productiva de R1-3.

La retirada de esta vía no resuelve por sí misma la exhaustividad del conjunto de comprobaciones pasado a la resolución.

## 8. Prohibiciones

La unidad no permite:

- construir un resultado resuelto desde un `CheckResult` nominal;
- agregar directamente una `RequirementCheck` seleccionada;
- reutilizar un resultado resuelto bajo una ligadura material distinta;
- duplicar una obligación para aumentar su peso;
- convertir mayoría, orden o repetición en regla de agregación;
- convertir `D-A`, `D-R` o `D-N` en `Tri`;
- convertir el resultado agregado en `Permit`.

La detección de una comprobación materialmente exigible que no fue suministrada a la resolución no se atribuye a esta unidad y debe quedar gobernada por la cobertura posterior.

## 9. Frontera con cobertura y reutilización

Esta unidad gobierna la composición de resultados resueltos dentro de un acto técnico ya constituido. No materializa todavía:

- exhaustividad de comprobaciones o evidencias por obligación;
- reglas de cobertura parcial;
- reutilización histórica;
- vigencia temporal;
- sustitución de resultados almacenados.

En particular:

```text
ResolvedRequirementResult(q) válido sobre comprobaciones suministradas
↛ cobertura exhaustiva de q

ResolvedRequirementResult(q) válido en una ligadura
↛ reutilizable automáticamente en otra ligadura o estado posterior
```

La conservación de los verificadores participantes permite que la unidad posterior de cobertura disponga de trazabilidad sin confundir participación con suficiencia.

## 10. Pruebas mínimas

La realización deberá demostrar, como mínimo:

1. `ResolvedRequirementResult` no tiene constructor público;
2. un `CheckResult` nominal no puede convertirse directamente en resultado resuelto;
3. una `RequirementCheck` individual no puede entrar en la agregación productiva de esta unidad;
4. conflicto sin regla produce un resultado resuelto `D-N`;
5. conflicto con regla constituida conserva el resultado gobernado antes de sellarlo;
6. el resultado resuelto conserva la identidad de los verificadores participantes;
7. participación de verificadores no se presenta como prueba de exhaustividad;
8. una ligadura material distinta se rechaza en la agregación;
9. una obligación inesperada se rechaza;
10. una obligación repetida se rechaza;
11. falta de una obligación de `Req` se rechaza;
12. un resultado acreditado para cada obligación de `Req` agrega a `D-A`, sin prejuzgar la cobertura interna posterior;
13. una refutación agrega a `D-R` aunque exista además `D-N`;
14. `D-N` se conserva cuando no existe refutación;
15. el orden de los resultados resueltos no altera la agregación;
16. `D-N` permanece fuera de `Tri`;
17. no se produce `Permit`, autoridad ni efecto protegido;
18. T-G, T-C y T-R permanecen no productivas;
19. las regresiones de R0 y de las unidades anteriores de R1 permanecen correctas.

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
R1-3 / unidad 3C = EN REALIZACIÓN

R1-4 = NO INICIADO
R2–R4 = NO INICIADOS
Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```
