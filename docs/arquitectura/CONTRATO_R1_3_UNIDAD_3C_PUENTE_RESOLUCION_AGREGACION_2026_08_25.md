# R1-3 — Unidad 3C: puente sellado entre resolución y agregación

**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte:** R1-3  
**Estado:** contrato de realización

## 1. Objeto

Esta unidad materializa el puente entre la resolución gobernada de las comprobaciones de una obligación individual y la agregación técnica del conjunto completo de obligaciones `Req(F,e | C)`.

Las unidades 3A y 3B ya determinan el resultado técnico de una obligación `q` cuando existen varias comprobaciones aplicables. La agregación entre obligaciones no debe poder omitir esa resolución escogiendo localmente una comprobación particular.

La composición cerrada es:

```text
RequirementCheck(q,V1), ..., RequirementCheck(q,Vn)
→ resolución gobernada de q
→ ResolvedRequirementResult(q)
→ agregación sobre Req(F,e | C)
```

La unidad no produce `Permit`, no ejecuta efectos protegidos, no modifica `Tri` y no abre R1-4.

## 2. Resultado resuelto sellado

`ResolvedRequirementResult` representa el resultado técnico ya resuelto de una obligación constituida.

No es un alias de `CheckResult` ni una comprobación elegida entre varias.

```text
CheckResult nominal
≠ RequirementCheck
≠ ResolvedRequirementResult
```

El tipo no ofrece constructor público. Sólo la vía de resolución gobernada puede producirlo a partir de un `RequirementDescriptor` constituido y de comprobaciones `RequirementCheck` selladas.

## 3. Ligadura material

El resultado resuelto conserva las dimensiones del descriptor cuya variación puede cambiar su interpretación o validez dentro de este corte:

- identidad de la obligación;
- clase de obligación;
- forma constituida;
- familia de efectos;
- contexto constitutivo;
- familias de verificadores admisibles;
- regla de aplicabilidad;
- identidad de la regla de resolución de conflicto, cuando exista.

La agregación vuelve a contrastar esta ligadura contra el `RequirementDescriptor` contenido en el `RequirementSet` recibido.

Una coincidencia meramente nominal de `RequirementRef` no basta para reutilizar un resultado con otra ligadura.

## 4. Única vía de formación

La formación sigue obligatoriamente esta secuencia:

```text
RequirementDescriptor
+ RequirementCheck[]
→ resolve_requirement_checks(...)
→ CheckResult gobernado
→ ResolvedRequirementResult sellado
```

Queda excluido:

```text
CheckResult
→ ResolvedRequirementResult
```

También queda excluida la selección local de una `RequirementCheck` para representar a `q` cuando existen varias comprobaciones cuya resolución corresponde a 3A o 3B.

## 5. Conflicto

El puente no introduce una regla nueva de conflicto.

Conserva exactamente las reglas ya materializadas:

```text
conflicto(q) + ausencia de regla constituida
→ D-N

conflicto(q) + regla constituida y aplicable
→ resultado gobernado por esa regla
```

El resultado obtenido se sella después de la resolución y sólo entonces puede entrar en la agregación entre obligaciones.

## 6. Agregación cerrada

La agregación pública de R1-3 acepta exclusivamente resultados resueltos sellados, uno por obligación constituida del `RequirementSet`.

Debe comprobar:

1. conjunto `Req` no vacío;
2. ninguna obligación inesperada;
3. ninguna obligación repetida;
4. ligadura material exacta;
5. cobertura completa de todas las obligaciones constituidas.

La precedencia permanece:

```text
D-R  si existe al menos una obligación REFUTADA;
D-N  si ninguna está REFUTADA y existe al menos una NO_VERIFICABLE;
D-A  sólo si todas están ACREDITADAS.
```

La resolución intra-obligación y la agregación inter-obligaciones son operaciones distintas y no deben fusionarse mediante selección implícita.

## 7. Cierre de la vía anterior

La función de agregación de la unidad 1 que aceptaba directamente una `RequirementCheck` por obligación fue una superficie transitoria anterior a la materialización del conflicto.

Tras 3A y 3B no puede permanecer como vía pública productiva, porque permitiría presentar una sola comprobación de `q` sin acreditar que el conjunto de comprobaciones aplicables de esa obligación ya fue resuelto.

Por tanto, la vía directa `RequirementCheck → agregación` queda restringida al interior de la realización y deja de formar parte de la frontera pública de R1-3.

## 8. Prohibiciones

La unidad no permite:

- construir un resultado resuelto desde un `CheckResult` nominal;
- agregar directamente una `RequirementCheck` seleccionada;
- omitir comprobaciones incompatibles de una obligación mediante selección local;
- reutilizar un resultado resuelto bajo una ligadura material distinta;
- duplicar una obligación para aumentar su peso;
- convertir mayoría, orden o repetición en regla de agregación;
- convertir `D-A`, `D-R` o `D-N` en `Tri`;
- convertir el resultado agregado en `Permit`.

## 9. Frontera con cobertura y reutilización

Esta unidad gobierna la composición dentro de un acto técnico ya constituido. No materializa todavía reglas de cobertura parcial, reutilización histórica, vigencia temporal o sustitución de resultados almacenados.

En particular:

```text
ResolvedRequirementResult(q) válido en una ligadura
↛ reutilizable automáticamente en otra ligadura o estado posterior
```

La cobertura y la reutilización permanecen como materias separadas de R1-3.

## 10. Pruebas mínimas

La realización deberá demostrar, como mínimo:

1. `ResolvedRequirementResult` no tiene constructor público;
2. un `CheckResult` nominal no puede convertirse directamente en resultado resuelto;
3. una `RequirementCheck` individual no puede entrar en la agregación pública final;
4. conflicto sin regla produce un resultado resuelto `D-N`;
5. conflicto con regla constituida conserva el resultado gobernado antes de sellarlo;
6. una ligadura material distinta se rechaza en la agregación;
7. una obligación inesperada se rechaza;
8. una obligación repetida se rechaza;
9. falta de una obligación se rechaza;
10. cobertura completa acreditada agrega a `D-A`;
11. una refutación agrega a `D-R` aunque exista además `D-N`;
12. `D-N` se conserva cuando no existe refutación;
13. el orden de los resultados resueltos no altera la agregación;
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
R1-3 / unidad 3C = EN REALIZACIÓN

R1-4 = NO INICIADO
R2–R4 = NO INICIADOS
Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```
