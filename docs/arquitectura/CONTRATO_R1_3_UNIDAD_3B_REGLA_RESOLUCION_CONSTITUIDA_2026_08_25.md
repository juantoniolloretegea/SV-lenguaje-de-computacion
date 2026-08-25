# R1-3 — Unidad 3B: regla de resolución previamente constituida

**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte:** R1-3  
**Estado:** contrato de realización

## 1. Objeto

Esta unidad materializa el caso complementario de conflicto fijado por SEC.0-D: varias comprobaciones selladas de una misma obligación producen resultados incompatibles y existe una regla de resolución previamente constituida y aplicable.

La unidad conserva el principio de fallo cerrado:

```text
conflicto(q) + ausencia de regla constituida
→ Check(q) = D-N
```

Sólo una regla constituida antes del acto de comprobación puede resolver un conflicto a `D-A` o `D-R`.

La unidad no produce `Permit`, no ejecuta efectos protegidos, no hace productivas T-G, T-C o T-R y no abre R1-4.

## 2. Regla cerrada materializada

La primera regla concreta de resolución de R1-3 es una regla de **verificador decisivo previamente constituido**.

Para una obligación `q`, la regla fija de antemano un verificador concreto `Vd` cuya comprobación decide el resultado cuando existe conflicto entre comprobaciones aplicables de `q`.

```text
ResolveRule(q) = DecisiveVerifier(Vd)
```

Esta regla no equivale a declarar que `Vd` es siempre correcto. Sólo determina qué comprobación gobierna un conflicto dentro de la ligadura constituida para la que la regla es válida.

## 3. Constitución previa

La regla no puede crearse durante la resolución del conflicto.

Su propuesta forma parte de la propuesta de la obligación que T-0 ya recibe para constituir el régimen inicial de R1-3. La conversión de propuesta a regla constituida exige la misma capacidad interna de génesis utilizada para constituir `Req` y `Applicable`.

Se conserva:

```text
propuesta de regla ≠ regla constituida
referencia nominal ≠ regla constituida
regla constituida ≠ autoridad
```

La propuesta no recibe libremente forma, familia de efectos, contexto, familia del verificador ni regla de aplicabilidad. Esas dimensiones se derivan de la obligación y de la relación `Applicable(Vd,q,C)` constituidas en T-0.

## 4. Condición de aplicabilidad del verificador decisivo

Una regla sólo puede constituirse si el verificador decisivo dispone de una relación de aplicabilidad válida para la misma obligación y contexto:

```text
Applicable(Vd,q,C) = true
```

Si no existe esa relación, la génesis completa se rechaza.

La regla no puede legitimar al verificador que pretende hacer decisivo ni crear una relación `Applicable` ausente.

## 5. Ligadura material de la regla

La regla constituida conserva, como mínimo:

- identidad propia de la regla;
- obligación `q`;
- forma constituida;
- familia de efectos;
- contexto constitutivo;
- verificador decisivo;
- familia constituida de ese verificador;
- regla constituida de aplicabilidad.

Estas dimensiones se obtienen de los objetos constituidos, no de parámetros ordinarios del acto de resolución.

Una comprobación que no coincida con la ligadura material del descriptor de `q` no puede participar en la resolución gobernada.

## 6. Unicidad

Esta unidad admite como máximo una regla de resolución constituida por obligación.

La reutilización de una misma referencia de regla para obligaciones distintas se rechaza durante la constitución inicial.

No existe selección local entre varias reglas concurrentes.

## 7. Resolución del conflicto

Si las comprobaciones son homogéneas, se conserva el resultado común sin necesidad de aplicar precedencia:

```text
D-A, D-A, ... → D-A
D-R, D-R, ... → D-R
D-N, D-N, ... → D-N
```

Si existe conflicto y no hay regla constituida:

```text
→ D-N
```

Si existe conflicto y hay una regla constituida de verificador decisivo:

```text
resultado(Vd) = D-A → D-A
resultado(Vd) = D-R → D-R
resultado(Vd) = D-N → D-N
```

Si `Vd` no está presente entre las comprobaciones del conjunto, la regla no puede resolver el conflicto y el resultado es `D-N`.

## 8. Prohibiciones

La unidad no permite resolver por:

- mayoría no constituida;
- orden de entrada;
- mera cronología;
- selección del resultado favorable;
- repetición de un verificador;
- elección del verificador decisivo durante el acto;
- sustitución local de la regla constituida;
- una relación de aplicabilidad creada por la propia comprobación.

La existencia de dos o más resultados iguales no confiere mayor peso al resultado repetido frente al verificador decisivo constituido.

## 9. Relación con la unidad 3A

La unidad 3A permanece válida como caso sin regla constituida.

La unidad 3B añade una vía gobernada para resolver incompatibilidades, pero no modifica la semántica de fallo cerrado cuando la regla falta o no puede aplicarse.

```text
sin regla suficiente → D-N
con regla suficiente → resultado gobernado
```

La función gobernada de esta unidad recibe el `RequirementDescriptor` constituido y comprobaciones `RequirementCheck` selladas. De este modo vuelve a comprobar la ligadura material de cada comprobación antes de aplicar la regla.

## 10. Frontera con la agregación

Esta unidad sigue resolviendo el resultado técnico de una obligación individual.

No crea todavía un objeto nuevo que pueda sustituir directamente a una `RequirementCheck` dentro de la agregación cerrada de obligaciones. El puente de composición entre resolución de conflicto y agregación deberá materializarse separadamente antes del cierre completo de R1-3.

No debe presentarse esta unidad como integración completa del flujo:

```text
múltiples comprobaciones de q
→ resolución gobernada de q
```

queda materializado, mientras que la conversión del resultado resuelto en una entrada sellada de la agregación permanece fuera de esta unidad.

## 11. Exclusión de doctrina histórica

Esta unidad no gobierna reutilización temporal, vigencia histórica ni sustitución de resultados almacenados.

En particular:

```text
conflicto simultáneo D-R + D-N
```

no debe confundirse con la regla distinta según la cual un `D-R` histórico todavía vigente no puede ser borrado por un `D-N` posterior.

La segunda materia pertenece a cobertura, reutilización y vigencia.

## 12. Pruebas mínimas

La realización deberá demostrar, como mínimo:

1. una regla no puede constituirse para un verificador sin `Applicable(Vd,q,C)`;
2. una referencia de regla no puede reutilizarse para dos obligaciones;
3. la regla no tiene constructor público;
4. comprobaciones con ligadura material ajena se rechazan;
5. conflicto sin regla sigue produciendo `D-N`;
6. conflicto con verificador decisivo acreditado produce `D-A`;
7. conflicto con verificador decisivo refutado produce `D-R`;
8. verificador decisivo en `D-N` produce `D-N`;
9. conflicto sin presencia del verificador decisivo produce `D-N`;
10. una mayoría contraria al verificador decisivo no altera el resultado gobernado;
11. el orden de las comprobaciones no altera el resultado;
12. un verificador repetido sigue siendo inválido;
13. `D-N` permanece fuera de `Tri`;
14. la agregación cerrada de obligaciones no se modifica en esta unidad;
15. T-G, T-C y T-R permanecen no productivas;
16. no se produce `Permit` ni efecto protegido;
17. las regresiones de R0 y de las unidades anteriores de R1 permanecen correctas.

## 13. Estado

```text
R0 = CERRADO
R1 = ABIERTO
R1-0 = CERRADO
R1-1 = CERRADO
R1-2 = CERRADO · INTEGRADO
R1-3 = ABIERTO

R1-3 / unidad 1 = CERRADA · INTEGRADA
R1-3 / unidad 2 = CERRADA · INTEGRADA
R1-3 / unidad 3A = CANDIDATA DE INTEGRACIÓN
R1-3 / unidad 3B = EN REALIZACIÓN

R1-4 = NO INICIADO
R2–R4 = NO INICIADOS
Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```
