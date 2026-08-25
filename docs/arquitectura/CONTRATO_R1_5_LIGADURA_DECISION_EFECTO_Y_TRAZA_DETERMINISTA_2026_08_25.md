# R1-5 — Contrato de ligadura decisión–efecto y traza determinista

**Estado:** ABIERTO  
**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`

## 1. Objeto

R1-5 materializará la ligadura causal explícita entre una decisión protegida y el efecto al que se refiere, y conservará una traza determinista suficiente para reconstruir el resultado gobernado del acto dentro de la continuidad lógica intra-proceso de R1.

R1-5 no reabre la semántica de `Req`, no vuelve a resolver comprobaciones, no vuelve a decidir el permiso y no ejecuta nuevamente el efecto. Recibe como base los objetos ya cerrados por R1-3 y R1-4 y debe relacionarlos sin crear una segunda vía de autoridad o ejecución.

## 2. Punto de partida

El corte de partida es:

```text
main = bdc21c8dd93dfdde636a6122c60a23f9838a799e
```

En ese estado existen ya, entre otros objetos:

```text
ResolvedRequirementResult
HistoricalQualifiedRequirementResult
PermitDecision
Permit
MediatedEffectCommitment
ExerciseRef
ExerciseTraceEntry
ExerciseConfirmation
```

La existencia separada de estos objetos no se considerará por sí sola una traza integral de decisión protegida.

## 3. Información mínima de la traza

Para una decisión protegida que alcance un resultado gobernado, la traza deberá permitir recuperar sin inferencia heurística, como mínimo:

```text
FormRef
+ EffectDescriptor completo
+ ContextRef
+ AuthorityRef
+ conjunto Req aplicable
+ resultado individual sellado de cada q
+ resultado agregado D-A | D-R | D-N
+ disposición de permiso
+ ligadura de mediación, cuando exista
+ ExerciseRef y estado de ejercicio, cuando exista
```

Cuando un efecto no llegue a permiso positivo, la traza deberá conservar el bloqueo gobernado sin fabricar un `Permit` negativo ni un efecto inexistente.

## 4. Resultados de obligación y resultado agregado

La traza deberá conservar la correspondencia exacta:

```text
q ∈ Req
↔ resultado sellado de q
```

No será admisible reconstruir el resultado de una obligación por posición de vector, orden de inserción o coincidencia parcial.

El agregado deberá conservar exactamente uno de:

```text
D-A = ACREDITADO
D-R = REFUTADO
D-N = NO_VERIFICABLE
```

con:

```text
D-A ≠ D-R
D-A ≠ D-N
D-R ≠ D-N
D-N ≠ Tri.U
```

La traza no podrá reinterpretar un error estructural como `D-N` ni como `Tri.U`.

## 5. Disposición de permiso

La traza deberá distinguir al menos:

```text
permiso concedido
bloqueo por D-R
bloqueo por D-N
```

Los errores estructurales que impidan formar una decisión válida podrán conservarse como estados técnicos tipados, pero no se promoverán a `CheckResult`, `Tri`, autoridad o permiso.

Un bloqueo no constituye autoridad negativa ni capacidad ejecutable.

## 6. Ligadura exacta decisión–efecto

Una decisión deberá quedar ligada al `EffectDescriptor` completo y no únicamente a `EffectRef` o a la familia del efecto.

La ligadura material mínima comprende:

```text
forma
+ autoridad
+ efecto completo
+ contexto seleccionado
+ Req que sustentó el resultado
```

Una traza de un acto no podrá reutilizarse como prueba de decisión de otro acto por coincidencia de nombres parciales.

## 7. Mediación y ejercicio

Cuando exista permiso positivo y mediación, la traza deberá poder demostrar la continuidad causal:

```text
resultado D-A gobernado
→ Permit
→ MediatedEffectCommitment
```

Cuando exista un intento de ejecución T-E, deberá enlazarse con:

```text
MediatedEffectCommitment
→ ExerciseRef
→ DispatchCommitted
→ Confirmed | Indeterminate
```

No será admisible crear un enlace de ejercicio a una decisión bloqueada ni atribuir `Confirmed` sin la entrada causal previa `DispatchCommitted` del mismo `ExerciseRef`.

## 8. Inmutabilidad y no fabricación

La representación productiva de la traza deberá quedar cerrada frente a fabricación ordinaria.

El llamador no podrá suministrar libremente como hechos de la traza:

- resultado agregado;
- disposición de permiso;
- autoridad aplicable;
- efecto comprometido;
- `ExerciseRef` productivo;
- estado `Confirmed` o `Indeterminate`.

Estos datos deberán derivar de objetos ya sellados o de la traza gobernada existente.

La consulta o copia de una representación de traza no podrá producir por conversión:

```text
Authority
Permit
MediatedEffectCommitment
ExecutionRequest
Tri
```

## 9. Orden determinista

La identidad causal no dependerá de orden accidental de `HashMap`, orden de llegada de una colección externa ni regla «el último gana» entre decisiones distintas.

Cuando sea necesario ordenar conjuntos para serialización o comparación se utilizará una representación canónica estable.

Un ordinal estructural podrá utilizarse para identificar entradas dentro de una continuidad lógica, pero:

```text
ordinal ≠ tiempo
ordinal ≠ vigencia
ordinal ≠ autoridad
```

## 10. Tiempo

R1-5 no introduce reloj ambiental, fecha de caducidad, `SystemTime`, `Instant` ni marca temporal como primitiva semántica.

El paso del tiempo no cambia por sí solo el resultado de una decisión ni la validez de una traza.

La vigencia de resultados y reglas continúa dependiendo de ligaduras constituidas conforme a R1-3/3E y de las revalidaciones ya cerradas en R1-4.

## 11. Relación con la traza de ejercicio existente

`ExerciseTraceEntry` ya conserva una traza append-only del intento material de T-E dentro de `ExecutionContinuity`.

R1-5 no duplicará esa traza como una segunda fuente contradictoria. Deberá enlazarla o incorporarla mediante una relación determinista que preserve:

```text
ExerciseRef
+ autoridad
+ forma
+ EffectDescriptor
+ contexto
+ contrato de acumulación
+ estado técnico del intento
```

Si se adopta una vista agregada, ésta deberá ser derivable de los registros gobernados sin permitir modificar retrospectivamente los eventos fuente.

## 12. Fallo cerrado de la trazabilidad

Una ausencia de información requerida para formar una traza completa no autoriza a inventarla.

En particular:

```text
traza incompleta
↛ D-A
↛ Permit
↛ autoridad
↛ efecto confirmado
↛ Tri.U
```

La incapacidad de construir una traza exigida será un fallo técnico o estructural de trazabilidad, separado de los resultados semánticos y de comprobación.

## 13. Separación respecto de evidencia externa

La traza de R1-5 es producida por el propio SUT y sirve para reconstrucción causal intra-proceso.

No constituye por sí sola:

- atestación independiente;
- prueba física de que un adaptador actuó honestamente;
- persistencia durable;
- prueba contra retroceso o clonación;
- firma humana o criptográfica externa;
- Garantía I;
- Garantía II.

Estas propiedades permanecen fuera del alcance de R1-5.

## 14. Relación con las clases T-*

R1-5 no altera la productividad de las clases de transición.

Se mantiene:

```text
T-E = productiva por la vía gobernada de R1-4
T-G = NO PRODUCTIVA
T-C = NO PRODUCTIVA
T-R = NO PRODUCTIVA
```

La traza no podrá utilizarse como mecanismo indirecto para constituir autoridad o hacer productiva una clase bloqueada.

## 15. Exclusiones

Quedan fuera de R1-5:

- persistencia durable de la traza;
- identidad durable entre procesos o réplicas;
- recuperación material;
- exclusión distribuida;
- atestación de plataforma;
- productividad de T-G, T-C y T-R;
- `BudgetΣ`;
- IA-SEC;
- R2, R3 y R4;
- Garantía I;
- Garantía II.

## 16. Criterios de cierre

R1-5 será cerrable cuando se demuestre que:

1. R1-4 permanece integrado sin regresión;
2. cada obligación de `Req` puede ligarse de forma exacta a su resultado sellado;
3. el resultado agregado queda conservado sin reinterpretación;
4. permiso y bloqueo quedan distinguidos explícitamente;
5. una decisión positiva queda ligada al efecto concreto, forma, autoridad y contexto que la sustentan;
6. la mediación, cuando exista, queda ligada a la misma decisión positiva;
7. cualquier ejercicio registrado queda ligado a la mediación correspondiente mediante el mismo efecto y `ExerciseRef`;
8. una decisión bloqueada no puede adquirir mediación o ejercicio por fabricación de traza;
9. la traza no puede convertirse en autoridad, permiso o capacidad de ejecución;
10. el orden de la representación es determinista y no depende de reloj ambiental ni de orden accidental de colecciones;
11. la traza de ejercicio existente no puede reescribirse mediante la nueva representación;
12. D-A, D-R y D-N permanecen disjuntos y fuera de `Tri`;
13. T-G, T-C y T-R continúan no productivas;
14. R0, R1-3 y R1-4 permanecen correctos en nativo y WebAssembly.

Hasta cumplir estas condiciones:

```text
R1-5 = ABIERTO
R1   = ABIERTO
R1-6 = NO INICIADO
```
