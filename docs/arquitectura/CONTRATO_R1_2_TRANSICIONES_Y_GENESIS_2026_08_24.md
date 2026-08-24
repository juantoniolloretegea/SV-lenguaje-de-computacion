# R1-2 — Transiciones de autoridad y restricción de T-0 por continuidad

**Fecha:** 24 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte:** R1-2  
**Estado:** abierto · en realización

## 1. Objeto

R1-2 materializa la clasificación operativa de las transiciones T-* que afectan a la existencia de autoridad y la restricción de T-0 al primer estado legítimo de autoridad de una continuidad autoritativa lógica todavía no habitada.

El corte no materializa todavía `Req`, la aplicabilidad de obligaciones ni la agregación de `D-A`, `D-R` y `D-N`. Por ello, las transiciones T-G, T-C y T-R se representan como clases autorizantes abstractas, pero no pueden producir cambios efectivos de autoridad en este corte.

R1-2 no modifica la semántica cerrada de R0 ni convierte identidad de proceso, reinicio, réplica o identificador técnico en autoridad SV.

## 2. Premisa constituyente externa

T-0 no puede demostrar desde el propio núcleo la legitimidad de la autoridad constituyente que permite su génesis.

R1-2 representa esa condición mediante una premisa constituyente externa opaca. Su función es servir de condición necesaria de entrada a T-0. La premisa:

```text
no es Tri
no es autoridad SV por sí sola
no es una firma
no es una identidad de proceso
no es una credencial del sistema operativo
no es una afirmación fabricada por el propio núcleo
```

`sv_core` no incorpora en R1-2 un constructor público capaz de acuñar esa premisa. En consecuencia, los adaptadores nativo y WebAssembly no pueden autodeclararla mediante la API ordinaria del núcleo.

La acreditación material de la procedencia, identidad, integridad o legitimidad externa de esa premisa queda fuera de R1. R1-2 sólo materializa la consecuencia intra-proceso de consumir una premisa ya dada dentro del modelo declarado.

## 3. Continuidad lógica de R1

Se representa una continuidad autoritativa lógica con dos estados cerrados:

```text
Uninhabited
Inhabited
```

En el alcance de R1:

```text
Uninhabited
⇒ no contiene autoridad admitida ni formas constituidas por una génesis previa

Inhabited
⇒ contiene el estado constituido por una T-0 anterior
```

Esta representación no acredita continuidad material entre procesos, restauraciones, réplicas o estados persistentes. Es una condición lógica intra-proceso.

No puede inferirse:

```text
nuevo proceso ⇒ Uninhabited legítimo
nuevo PID ⇒ nueva continuidad
nuevo instance_id ⇒ nueva T-0
reinicio ⇒ nueva T-0
```

## 4. T-0

T-0 es la única transición productiva de autoridad materializada en R1-2.

Para que una T-0 pueda completar una génesis deben satisfacerse conjuntamente:

1. existe una premisa constituyente externa opaca;
2. la continuidad lógica está `Uninhabited`;
3. la propuesta contiene al menos una forma inicial;
4. la propuesta contiene al menos una autoridad inicial;
5. las referencias de forma son únicas dentro de la constitución;
6. las referencias de autoridad son únicas dentro de la constitución;
7. las autoridades propuestas tienen `E_max` y `D_a` estructuralmente coherentes;
8. toda autoridad previa exigida por una forma inicial queda identificada dentro del mismo estado inicial constituido.

Una génesis rechazada no consume T-0 ni cambia el estado de ocupación.

Una génesis completada produce conjuntamente:

```text
formas iniciales constituidas
+ autoridades iniciales constituidas
+ continuidad = Inhabited
```

Desde ese instante:

```text
T0_disponible = false
```

para esa continuidad lógica.

## 5. Unicidad constitutiva

R1-2 materializa la obligación pendiente de R1-1:

```text
mismo FormRef
⇒ no dos FormDescriptor constituidos incompatibles

mismo AuthorityRef
⇒ no dos ConstitutedAuthority incompatibles
```

La primera realización adopta la regla conservadora más fuerte: dentro de una génesis, una referencia de forma o autoridad sólo puede aparecer una vez. Una duplicación se rechaza antes de constituir el estado.

R1-2 no afirma unicidad global durable entre procesos o réplicas. Esa propiedad requiere continuidad material y pertenece a R2 o fases posteriores.

## 6. Transiciones no autorizantes

Las clases:

```text
T-I
T-V
T-H
T-E
```

se clasifican expresamente como no autorizantes.

En R1-2 no existe operación asociada a esas clases que pueda:

- crear `ConstitutedAuthority`;
- crear una forma constituida;
- ampliar `E_max`;
- ampliar `D_a`;
- cambiar `Uninhabited` a `Inhabited`;
- reabrir T-0.

La ejecución de una T-E tampoco confiere autoridad al ejecutor.

## 7. T-G, T-C y T-R antes de R1-3

T-G, T-C y T-R permanecen reconocidas como clases abstractas capaces de crear, modificar o restaurar autoridad únicamente bajo sus condiciones contractuales.

En R1-2 su disposición operativa es:

```text
T-G = BLOQUEADA_PENDIENTE_DE_REQUISITOS
T-C = BLOQUEADA_PENDIENTE_DE_REQUISITOS
T-R = BLOQUEADA_PENDIENTE_DE_REQUISITOS
```

La razón es estructural: R1-3 todavía no ha materializado `Req(F,e | C)` ni los resultados aplicables `D-A/D-R/D-N`. Permitir cambios efectivos antes de esa frontera permitiría eludir la condición de fallo cerrado que R1 debe demostrar.

El bloqueo no produce `Tri.U`, no constituye autoridad y no equivale a una clausura semántica.

## 8. Clasificación no discrecional

La clase T-* de una forma continúa fijada por su `FormDescriptor` constituido. El ejecutor, adaptador o consumidor no puede reclasificar durante el acto una operación para obtener un régimen más permisivo.

R1-2 no incorpora todavía ejecución protegida. Sólo fija la relación entre clase de transición y capacidad de constituir autoridad.

## 9. Frontera de construcción

Los objetos propuestos para génesis son datos de construcción y no autoridad.

La conversión desde propuesta a forma o autoridad constituida sólo puede producirse dentro de la puerta T-0 de R1-2 y mediante una capacidad interna sellada que no forma parte de la API ordinaria.

No existe un constructor público alternativo para:

```text
FormDescriptor
EffectDescriptor
EffectEnvelope
GovernedDomain
ConstitutedAuthority
```

La mera posesión de referencias nominales tampoco permite construir esos objetos.

## 10. Pruebas obligatorias

R1-2 deberá ejercer al menos los casos siguientes:

1. T-0 válida sobre continuidad `Uninhabited` constituye el estado inicial y deja la continuidad `Inhabited`;
2. segunda T-0 sobre la misma continuidad falla;
3. una génesis vacía falla sin consumir T-0;
4. duplicar `FormRef` falla antes de constituir;
5. duplicar `AuthorityRef` falla antes de constituir;
6. una autoridad con efecto fuera de su contexto falla;
7. una autoridad con efecto fuera de `D_a` falla;
8. una forma que exige una autoridad no incluida en el estado inicial falla;
9. T-I, T-V, T-H y T-E se clasifican como no autorizantes;
10. T-G, T-C y T-R quedan bloqueadas hasta R1-3;
11. ninguna denegación o bloqueo se proyecta a `Tri`;
12. las regresiones completas de R0 permanecen correctas.

## 11. Límites

R1-2 no materializa:

- verificación material de la premisa constituyente externa;
- identidad externa;
- credenciales o firmas;
- persistencia durable;
- continuidad entre procesos;
- bifurcación o selección de vigencia;
- recuperación material;
- `Req`;
- `D-A`, `D-R`, `D-N` aplicados a decisiones;
- `Permit`;
- mediación de efectos protegidos;
- traza completa de decisión;
- Garantía I;
- Garantía II.

Tampoco abre R2, R3 ni R4.

## 12. Criterio de cierre

R1-2 podrá cerrarse cuando exista evidencia reproducible de que:

1. T-0 es la única vía productiva de autoridad del corte;
2. T-0 requiere simultáneamente premisa constituyente y continuidad no habitada;
3. una segunda T-0 no puede ejecutarse sobre la misma continuidad lógica;
4. una génesis rechazada no altera el estado;
5. `FormRef` y `AuthorityRef` no admiten duplicaciones constitutivas dentro del estado inicial;
6. T-I/T-V/T-H/T-E no producen autoridad;
7. T-G/T-C/T-R no aplican cambios antes de R1-3;
8. ninguna vía ordinaria del adaptador puede fabricar la premisa o los objetos constituidos;
9. R0 permanece sin regresiones.

Hasta ese cierre:

```text
R0 = CERRADO
R1 = ABIERTO
R1-0 = CERRADO
R1-1 = CERRADO
R1-2 = EN DESARROLLO
R2–R4 = NO INICIADOS
Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```
