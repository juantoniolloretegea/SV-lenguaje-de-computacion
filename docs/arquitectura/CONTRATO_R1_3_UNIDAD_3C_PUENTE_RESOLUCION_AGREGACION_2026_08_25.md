# R1-3 — Unidad 3C: puente sellado entre resolución y agregación

**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte:** R1-3  
**Estado:** realizado e integrado

## 1. Objeto

Esta unidad materializa el puente entre la resolución gobernada de las comprobaciones suministradas para una obligación individual y la validación estructural del conjunto completo de obligaciones `Req(F,e | C)`.

Las unidades 3A y 3B determinan el resultado técnico de una obligación `q` a partir del conjunto de comprobaciones que recibe la resolución. La agregación entre obligaciones no debe poder sustituir ese resultado resuelto por una `RequirementCheck` individual seleccionada localmente.

La composición materializada es:

```text
RequirementCheck(q,V1), ..., RequirementCheck(q,Vn)
→ resolución gobernada del conjunto suministrado para q
→ ResolvedRequirementResult(q)
→ validación estructural sobre Req(F,e | C)
```

La unidad no acredita que el conjunto de comprobaciones suministrado para `q` sea exhaustivo respecto de todas las comprobaciones o evidencias materialmente exigibles. Esa propiedad pertenece a la cobertura constituida de la unidad 3D.

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

El resultado resuelto conserva las dimensiones del descriptor cuya variación puede cambiar su interpretación o validez:

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

La unidad 3D amplía esta ligadura con la identidad de la regla de cobertura constituida, cuando existe, de modo que un cambio de cobertura también invalida la reutilización estructural del sello contra un descriptor distinto.

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

La unidad también impide que una `RequirementCheck` individual sustituya al objeto resuelto dentro de la composición inter-obligaciones.

Esta restricción no equivale a afirmar que la unidad detecte por sí sola toda comprobación omitida antes de llamar a la resolución. Determinar qué comprobaciones o evidencias debían estar presentes pertenece a la cobertura constituida de 3D.

## 5. Conflicto

El puente no introduce una regla nueva de conflicto.

Conserva exactamente las reglas ya materializadas:

```text
conflicto(q) + ausencia de regla constituida
→ D-N

conflicto(q) + regla constituida y aplicable
→ resultado gobernado por esa regla
```

El resultado obtenido se sella después de resolver el conjunto suministrado y sólo entonces puede continuar hacia la composición entre obligaciones.

## 6. Validación estructural entre obligaciones

La unidad valida exclusivamente resultados resueltos sellados, uno por obligación constituida del `RequirementSet`.

Comprueba:

1. conjunto `Req` no vacío;
2. ninguna obligación inesperada;
3. ninguna obligación repetida;
4. ligadura material exacta;
5. presencia de un resultado resuelto para cada obligación constituida de `Req`.

Esta cobertura es cobertura del conjunto de obligaciones de `Req`; no acredita exhaustividad interna de las comprobaciones empleadas para resolver cada obligación.

La precedencia estructural preservada es:

```text
D-R  si existe al menos una obligación REFUTADA;
D-N  si ninguna está REFUTADA y existe al menos una NO_VERIFICABLE;
D-A  sólo si todas están ACREDITADAS.
```

Tras la materialización de 3D, esta agregación no cualificada permanece como validación interna. La frontera productiva pública pasa por la cobertura constituida antes de obtener el resultado agregado final.

## 7. Cierre de la vía anterior

La función de agregación de la unidad 1 que aceptaba directamente una `RequirementCheck` por obligación fue una superficie transitoria anterior a la materialización del conflicto.

Tras 3A y 3B no puede permanecer como vía pública productiva, porque permitiría presentar una comprobación individual de `q` directamente a la agregación sin atravesar el objeto resuelto de la obligación.

Por tanto, la vía directa `RequirementCheck → agregación` queda restringida a pruebas internas de regresión y deja de formar parte de la frontera productiva de R1-3.

3D aplica el mismo criterio a la agregación no cualificada de `ResolvedRequirementResult`: se conserva internamente como validación estructural, pero ya no constituye la frontera pública final.

## 8. Prohibiciones

La unidad no permite:

- construir un resultado resuelto desde un `CheckResult` nominal;
- agregar directamente una `RequirementCheck` seleccionada;
- reutilizar un resultado resuelto bajo una ligadura material distinta;
- duplicar una obligación para aumentar su peso;
- convertir mayoría, orden o repetición en regla de agregación;
- convertir `D-A`, `D-R` o `D-N` en `Tri`;
- convertir el resultado agregado en `Permit`.

La detección de una comprobación materialmente exigible que no fue suministrada a la resolución no se atribuye a esta unidad y queda gobernada por 3D.

## 9. Frontera con cobertura y reutilización

Esta unidad gobierna la composición de resultados resueltos dentro de un acto técnico ya constituido. No materializa por sí sola:

- exhaustividad de comprobaciones o evidencias por obligación;
- reglas de cobertura;
- reutilización histórica;
- vigencia temporal;
- sustitución de resultados almacenados.

En particular:

```text
ResolvedRequirementResult(q) válido sobre comprobaciones suministradas
↛ cobertura suficiente de q

ResolvedRequirementResult(q) válido en una ligadura
↛ reutilizable automáticamente en otra ligadura o estado posterior
```

La conservación de los verificadores participantes permite que 3D evalúe cobertura sin confundir participación con suficiencia.

## 10. Pruebas de cierre

La realización demuestra, como mínimo, que:

1. `ResolvedRequirementResult` no tiene constructor público;
2. un `CheckResult` nominal no puede convertirse directamente en resultado resuelto;
3. una `RequirementCheck` individual no puede sustituir al resultado resuelto en la composición inter-obligaciones;
4. conflicto sin regla produce un resultado resuelto `D-N`;
5. conflicto con regla constituida conserva el resultado gobernado antes de sellarlo;
6. el resultado resuelto conserva la identidad de los verificadores participantes;
7. participación de verificadores no se presenta como prueba de exhaustividad;
8. una ligadura material distinta se rechaza;
9. una obligación inesperada se rechaza;
10. una obligación repetida se rechaza;
11. falta de una obligación de `Req` se rechaza;
12. la precedencia `D-R > D-N > D-A` permanece determinista;
13. el orden de los resultados resueltos no altera la validación;
14. `D-N` permanece fuera de `Tri`;
15. no se produce `Permit`, autoridad ni efecto protegido;
16. T-G, T-C y T-R permanecen no productivas;
17. las regresiones de R0 y de las unidades anteriores de R1 permanecen correctas.

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
R1-3 / unidad 3D = CANDIDATA DE CIERRE

R1-4 = NO INICIADO
R2–R4 = NO INICIADOS
Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```
