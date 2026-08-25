# R1-4 — Unidad 3: ejecución gobernada y registro de ejercicio

**Estado:** CERRADA · INTEGRADA  
**Fecha:** 25 de agosto de 2026

## 1. Objeto

Esta unidad materializa la primera frontera de ejecución material de R1-4 después de la decisión de permiso y de la mediación no eludible ya integradas.

El punto de entrada exigible es exclusivamente:

```text
MediatedEffectCommitment
```

La relación mínima de esta unidad es:

```text
MediatedEffectCommitment
+ clase de transición ejecutable en este corte
+ contrato de acumulación admisible
+ registro previo del intento
→ despacho único al adaptador de ejecución
→ registro técnico del resultado
```

La unidad no reabre `Req`, no vuelve a decidir el permiso y no admite una vía alternativa basada en `Permit`, `EffectRef`, `EffectDescriptor`, `CheckResult`, `AuthorityRef` o un booleano de autorización.

## 2. Corte de clases de transición

La primera clase susceptible de ejecución material en esta unidad es:

```text
T-E = Exercise
```

Su productividad no constituye autoridad nueva:

```text
T-E productiva como ejercicio
↛ ConstitutedAuthority
↛ ampliación de E_max
↛ ampliación de D_a
```

T-I, T-V y T-H continúan sin constituir autoridad y no se convierten en esta unidad en rutas genéricas de ejecución material.

T-G, T-C y T-R permanecen no productivas en este corte. Sus efectos pueden modificar gobierno, constitución o recuperación y no se ejecutarán mediante el adaptador genérico de T-E. Deberán disponer de una realización gobernada propia antes de producir cambios efectivos.

## 3. Separación de objetos

Se mantiene la cadena estricta:

```text
CheckResult
≠ Permit
≠ MediatedEffectCommitment
≠ intento de ejecución
≠ ejercicio confirmado
```

Ninguno de esos objetos pertenece por conversión a `Tri`.

La existencia de un `MediatedEffectCommitment` tampoco prueba por sí sola que el efecto haya sido ejecutado.

## 4. Consumo lineal del compromiso mediado

La operación de ejecución deberá consumir `MediatedEffectCommitment` por valor.

El compromiso no implementa `Clone` ni `Copy` y no podrá reconstruirse mediante un constructor público.

Por tanto, una misma instancia de compromiso no podrá despacharse dos veces mediante la API ordinaria de R1.

Un fallo posterior al consumo no devuelve el mismo compromiso al llamador. Un nuevo intento, cuando resulte admisible, exigirá una nueva cadena gobernada de decisión y mediación.

## 5. Solicitud de ejecución no fabricable

El adaptador de ejecución no recibirá directamente un `Permit`, un `EffectRef`, un `EffectDescriptor` aislado ni una decisión booleana.

La realización deberá formar internamente una solicitud de ejecución no fabricable a partir del compromiso mediado consumido.

La solicitud conservará, como mínimo, la identidad material ya sellada del acto:

```text
AuthorityRef
+ FormRef
+ TransitionClass
+ EffectDescriptor completo
+ ContextRef
+ AccumulationContract
```

El adaptador podrá consultar las dimensiones necesarias para realizar el efecto, pero no podrá alterar el permiso, la autoridad, `Req`, la cobertura ni las reglas de vigencia.

## 6. Puerto de ejecución

La llamada material se realizará mediante un puerto explícito de ejecución.

El puerto pertenece a la frontera de ejecución del sistema, no a la semántica ternaria ni a la constitución de autoridad. Su implementación concreta podrá residir fuera de `sv_core`.

La interfaz de `sv_core` deberá ser síncrona y no depender de red, sistema de archivos, proceso, dispositivo, proveedor, `async` ni biblioteca externa concreta.

Una implementación de adaptador podrá realizar esas operaciones en la capa correspondiente, pero sólo recibirá una solicitud sellada producida por la frontera gobernada.

La conformidad SV sólo puede afirmarse respecto de la vía controlada. Código exterior que invoque directamente un sistema operativo, dispositivo o servicio sin atravesar esta frontera no constituye una ejecución gobernada del SV.

## 7. Registro previo al despacho

La unidad no presumirá que un error técnico posterior a llamar al adaptador significa que el efecto no ocurrió.

Antes de despachar el efecto deberá quedar registrado, dentro de la continuidad lógica de R1, un intento de ejercicio identificado de forma única en esa continuidad.

La identificación no utilizará reloj ambiental, marca temporal, aleatoriedad ni identidad de proceso como fuente semántica. Podrá utilizar una identidad opaca y un ordinal estructural interno sin significado temporal.

El registro previo deberá conservar al menos:

```text
ExerciseRef
+ efecto concreto
+ forma
+ autoridad
+ contexto
+ contrato de acumulación
+ estado técnico del intento
```

La traza de intentos será append-only en el alcance lógico intra-proceso de R1.

Esta propiedad no acredita persistencia durable ni resistencia a retroceso o clonación; esas garantías pertenecen a fases posteriores.

## 8. Resultado después del despacho

Una vez invocado el adaptador sólo se distinguirán, como mínimo, dos resultados técnicos de la llamada:

```text
confirmación positiva del adaptador
→ ejercicio confirmado en la traza lógica

error, interrupción o ausencia de confirmación después del despacho
→ resultado de ejecución indeterminado
```

El segundo caso no significa que el efecto no haya ocurrido.

Por tanto:

```text
fallo posterior al despacho
↛ "no ejecutado" por inferencia
↛ Tri.U
↛ D-N
↛ reintento automático
```

La indeterminación de ejecución es un estado técnico de la infraestructura de ejercicio. No es un cuarto valor de `Tri` ni un resultado de comprobación de R1-3.

## 9. Fallos anteriores al despacho

Todo rechazo que pueda decidirse antes de invocar el adaptador deberá cerrarse antes del despacho.

Entre otros casos:

- la clase no es T-E;
- el compromiso no corresponde a un estado ejecutable en esta unidad;
- el contrato de acumulación impide el ejercicio;
- existe una colisión de identidad de ejercicio dentro de la continuidad;
- falta una condición estructural exigida por la realización.

Un fallo anterior al despacho no crea un registro de ejercicio confirmado y no se convertirá en `Tri.U`.

## 10. `ExerciseRef` y alcance de unicidad

`ExerciseRef` identificará un intento o ejercicio materializado dentro de la continuidad lógica de R1.

La referencia será formada por el núcleo de ejecución; no será una declaración libre del adaptador ni prueba externa de que el efecto ocurrió.

La unicidad exigida en esta unidad es intra-continuidad e intra-proceso.

No se afirmará todavía unicidad durable entre procesos, réplicas, restauraciones o bifurcaciones.

## 11. Contratos de acumulación

La unidad 3 no podrá ignorar `AccumulationContract` en el punto de ejecución.

A efectos de acumulación, el alcance exacto de ejercicio de esta unidad será:

```text
ExerciseScope
=
FormRef
+ AuthorityRef
+ EffectDescriptor completo
+ ContextRef seleccionado
```

Como `EffectDescriptor` conserva `EffectRef`, `EffectFamilyRef`, `GovernedObjectRef` y `ContextRef`, no se identifican por accidente dos ejercicios sólo por compartir familia, referencia nominal u objeto.

El primer régimen será:

```text
NotApplicable
→ sin restricción adicional de acumulación en R1-4/3

SingleUse
→ un ejercicio confirmado previo del mismo ExerciseScope impide un nuevo despacho
→ un intento previo indeterminado del mismo ExerciseScope también bloquea
  un nuevo despacho ordinario

Idempotent
→ puede existir un nuevo intento del mismo ExerciseScope, pero siempre con
  un nuevo Permit, un nuevo MediatedEffectCommitment y un nuevo ExerciseRef

GovernedAggregator(rule)
DecidableTracePredicate(rule)
→ no ejecutables mientras la regla correspondiente no disponga de una
   realización gobernada y no elegible por el adaptador
```

La mera presencia de una referencia `AccumulationRuleRef` no equivale a haber evaluado la regla.

## 12. Alcance de `SingleUse`

`SingleUse` no significa solamente que una instancia de `Permit` o de `MediatedEffectCommitment` no pueda clonarse.

La ejecución deberá consultar la traza de ejercicios de la continuidad para impedir un segundo despacho cuando exista un ejercicio confirmado anterior del mismo `ExerciseScope`.

Un intento indeterminado posterior al despacho se tratará conservadoramente como bloqueo para `SingleUse` mientras no exista una recuperación gobernada capaz de resolver su estado.

La recuperación de esa situación no forma parte de esta unidad.

## 13. `Idempotent`

La clasificación `Idempotent` es una propiedad constituida de la forma; no la decide el adaptador en el momento del fallo.

La idempotencia permite un nuevo acto gobernado después de un intento anterior, pero no reutiliza el mismo compromiso mediado ni borra la traza previa.

La unidad no utilizará la idempotencia para fabricar una confirmación positiva cuando el adaptador no la haya producido.

## 14. Relación con el adaptador

El adaptador ejecuta; no autoriza.

No podrá decidir por sí mismo:

- qué forma es aplicable;
- qué autoridad gobierna el efecto;
- si `Req` está satisfecho;
- si el permiso es válido;
- si la mediación puede omitirse;
- si una clase bloqueada debe hacerse productiva;
- si una regla de acumulación puede ignorarse.

El adaptador tampoco podrá transformar un error técnico en `Tri.U`, `D-A`, `D-R` o `D-N`.

## 15. Confirmación de ejecución y Garantía I

Una respuesta positiva del adaptador permitirá registrar una confirmación lógica de ejercicio en R1.

Esa confirmación no se presentará todavía como prueba material independiente de que el mundo externo realizó exactamente el efecto declarado.

En particular, R1-4/3 no acredita por sí sola:

- atestación de la plataforma realmente ejecutada;
- independencia entre ejecutor y observador;
- persistencia durable del registro;
- atomicidad entre efecto externo y registro local;
- resistencia a caída, retroceso, clonación o bifurcación.

Por tanto, la Garantía I permanece `NO_PROBADO` mientras las fases posteriores no acrediten la correspondencia material exigida.

## 16. Fallo durante el adaptador

Si el adaptador devuelve error o la llamada no produce confirmación positiva después de haber comenzado el despacho, la frontera no podrá afirmar `no ejecutado`.

El intento quedará técnicamente indeterminado y consumido.

Un reintento automático sobre el mismo compromiso queda prohibido.

Para formas no idempotentes, cualquier nuevo intento requerirá además que el régimen aplicable permita resolver previamente la indeterminación. Esta unidad no inventa una recuperación local para hacerlo.

## 17. T-E productiva sin autoridad nueva

El cierre positivo de esta unidad permite afirmar únicamente:

```text
T-E puede materializar un ejercicio protegido
si y sólo si atraviesa decisión, mediación y ejecución gobernadas
```

No permite afirmar:

```text
T-E → nueva autoridad
T-E → ampliación de E_max
T-E → ampliación de D_a
T-E → nueva forma
```

La autoridad y la forma siguen procediendo de las vías constitutivas gobernadas correspondientes.

## 18. T-G, T-C y T-R

T-G, T-C y T-R continúan no productivas en esta unidad.

El hecho de que R1-4 ya disponga de `Permit` y `MediatedEffectCommitment` no basta para aplicar cambios de gobierno, constitución o recuperación mediante el puerto genérico de T-E.

Su futura productividad deberá demostrar, como mínimo, que:

```text
cambio protegido
⇒ Req aplicable
⇒ D-A gobernado
⇒ Permit
⇒ mediación
⇒ operación específica de la clase
```

No se adelanta aquí esa realización.

## 19. Tiempo, concurrencia y vigencia entre mediación y despacho

La unidad no introduce reloj semántico, marcas temporales de vigencia ni regla «el último gana».

El ordinal de ejercicio, si se utiliza, será exclusivamente estructural.

La primera realización podrá ser secuencial y síncrona. No se afirmará seguridad frente a concurrencia entre procesos ni exclusión distribuida; esas propiedades requieren continuidad y persistencia materiales posteriores.

Dentro de una misma `AuthorityContinuity`, la realización deberá evitar que dos despachos secuenciales ordinarios reutilicen la misma identidad o violen un `SingleUse` ya registrado.

En el corte actual T-G, T-C y T-R permanecen no productivas. Por ello, entre una mediación válida y el despacho de T-E no existe todavía una vía productiva que pueda modificar forma, autoridad, `E_max`, `D_a`, `Req` o sus reglas constituidas.

Esta propiedad deja de ser suficiente en cuanto una clase capaz de modificar esas ligaduras se vuelva productiva. Antes de permitir esa coexistencia deberá demostrarse una de estas garantías equivalentes:

```text
revalidación completa inmediatamente antes del despacho
```

u otra ligadura explícita de versión/estado constituido que impida ejecutar un compromiso mediado contra un estado gobernante posterior incompatible.

La futura productividad de T-G, T-C o T-R no podrá apoyarse en el cierre de esta unidad para omitir esa comprobación.

## 20. Regla de no elusión

Para el perfil de ejecución gobernada del SV deberá cumplirse:

```text
efecto T-E ejecutado por la vía SV
⇒ solicitud no fabricable
⇒ consumo de MediatedEffectCommitment
```

Será un defecto bloqueante que una API productiva del perfil gobernado permita comprometer el mismo efecto aceptando directamente:

```text
Permit
EffectRef
EffectDescriptor
CheckResult
AuthorityRef
bool
```

sin consumir el compromiso mediado.

## 21. Exclusiones

Quedan fuera de esta unidad:

- productividad de T-G, T-C y T-R;
- modificación posterior de formas, autoridad, `E_max` o `D_a`;
- recuperación de un intento indeterminado;
- persistencia durable de la traza de ejercicios;
- identidad durable entre procesos;
- exclusión distribuida;
- atestación de plataforma;
- confianza material en el adaptador;
- R2, R3 y R4;
- `BudgetΣ`;
- IA-SEC;
- Garantía I;
- Garantía II.

## 22. Criterio de cierre

La unidad queda cerrada porque se ha demostrado que:

1. #41 permanece integrada sin regresión;
2. sólo `MediatedEffectCommitment` puede abrir la ejecución gobernada de T-E;
3. el compromiso se consume y no puede despacharse dos veces mediante la API ordinaria;
4. la solicitud entregada al adaptador no es fabricable por el llamador;
5. el intento queda registrado antes del despacho;
6. una confirmación positiva queda ligada al mismo efecto, forma y autoridad del compromiso;
7. un error posterior al despacho no se interpreta como «no ejecutado»;
8. la indeterminación técnica no se convierte en `Tri.U` ni en `D-*`;
9. `SingleUse` bloquea un segundo despacho del mismo `ExerciseScope` tras ejercicio confirmado o intento indeterminado;
10. `Idempotent` no reutiliza el mismo compromiso ni borra intentos anteriores;
11. contratos gobernados de acumulación permanecen cerrados mientras no exista su evaluador;
12. T-E productiva no constituye ni amplía autoridad;
13. T-G, T-C y T-R permanecen no productivas;
14. no existe una vía productiva alternativa que eluda `MediatedEffectCommitment`;
15. el cierre no se usa para justificar una futura ejecución contra ligaduras gobernantes modificadas sin revalidación o versión explícita;
16. R0, R1-3 y las unidades 1–2 de R1-4 permanecen correctas en nativo y WebAssembly.

Estas condiciones quedan acreditadas para el alcance de la unidad 3. R1-4 permanece abierto; T-G, T-C y T-R continúan no productivas; las Garantías I y II permanecen `NO_PROBADO`.