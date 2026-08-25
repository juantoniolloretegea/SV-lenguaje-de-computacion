# R1-5 — Contrato de ligadura decisión–efecto y traza determinista

**Estado:** CERRADO · INTEGRADO  
**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`

## 1. Objeto

R1-5 materializa la ligadura causal explícita entre una decisión protegida y el efecto al que se refiere, y conserva una traza determinista suficiente para reconstruir el resultado gobernado del acto dentro de la continuidad lógica intra-proceso de R1.

R1-5 no reabre la semántica de `Req`, no vuelve a resolver comprobaciones, no vuelve a decidir el permiso y no ejecuta nuevamente el efecto. Recibe como base los objetos ya cerrados por R1-3 y R1-4 y los relaciona sin crear una segunda vía de autoridad o ejecución.

## 2. Punto de partida

El corte de partida fue:

```text
main = bdc21c8dd93dfdde636a6122c60a23f9838a799e
```

En ese estado existían ya, entre otros objetos:

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

La existencia separada de estos objetos no se consideró por sí sola una traza integral de decisión protegida.

## 3. Información mínima de la traza

Para una decisión protegida que alcance un resultado gobernado, la traza permite recuperar sin inferencia heurística, como mínimo:

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

Cuando un efecto no llega a permiso positivo, la traza conserva el bloqueo gobernado sin fabricar un `Permit` negativo ni un efecto inexistente.

## 4. Resultados de obligación y resultado agregado

La traza conserva la correspondencia exacta:

```text
q ∈ Req
↔ resultado sellado de q
```

No se reconstruye el resultado de una obligación por posición de vector, orden de inserción o coincidencia parcial.

El agregado conserva exactamente uno de:

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

Un error estructural no se reinterpreta como `D-N` ni como `Tri.U`.

## 5. Disposición de permiso

La traza distingue:

```text
permiso concedido
bloqueo por D-R
bloqueo por D-N
```

Los errores estructurales que impiden formar una decisión válida se conservan como estados técnicos tipados y no se promueven a `CheckResult`, `Tri`, autoridad o permiso.

Un bloqueo no constituye autoridad negativa ni capacidad ejecutable.

## 6. Ligadura exacta decisión–efecto

Una decisión queda ligada al `EffectDescriptor` completo y no únicamente a `EffectRef` o a la familia del efecto.

La ligadura material comprende:

```text
forma
+ autoridad
+ efecto completo
+ contexto seleccionado
+ Req que sustentó el resultado
```

Una traza de un acto no puede reutilizarse como prueba de decisión de otro acto por coincidencia de nombres parciales.

## 7. Mediación y ejercicio

Cuando existe permiso positivo y mediación, la traza demuestra la continuidad causal:

```text
resultado D-A gobernado
→ Permit
→ MediatedEffectCommitment
```

Cuando existe un intento de ejecución T-E, queda enlazado con:

```text
MediatedEffectCommitment
→ ExerciseRef
→ DispatchCommitted
→ Confirmed | Indeterminate
```

No existe una vía pública ordinaria para crear un enlace de ejercicio a una decisión bloqueada ni para atribuir `Confirmed` sin la entrada causal previa `DispatchCommitted` del mismo `ExerciseRef`.

## 8. Inmutabilidad y no fabricación

La representación productiva de la traza queda cerrada frente a fabricación ordinaria.

El llamador no suministra libremente como hechos de la traza:

- resultado agregado;
- disposición de permiso;
- autoridad aplicable;
- efecto comprometido;
- `ExerciseRef` productivo;
- estado `Confirmed` o `Indeterminate`.

Estos datos derivan de objetos ya sellados o de la traza gobernada existente.

La consulta o copia de una representación de traza no produce por conversión:

```text
Authority
Permit
MediatedEffectCommitment
ExecutionRequest
Tri
```

## 9. Orden determinista

La identidad causal no depende de orden accidental de `HashMap`, orden de llegada de una colección externa ni regla «el último gana» entre decisiones distintas.

Cuando es necesario ordenar conjuntos para serialización o comparación se utiliza una representación canónica estable.

El ordinal estructural que identifica entradas dentro de una continuidad lógica cumple:

```text
ordinal ≠ tiempo
ordinal ≠ vigencia
ordinal ≠ autoridad
```

El orden léxico de las referencias de decisión es una propiedad de representación y no define cronología.

## 10. Tiempo

R1-5 no introduce reloj ambiental, fecha de caducidad, `SystemTime`, `Instant` ni marca temporal como primitiva semántica.

El paso del tiempo no cambia por sí solo el resultado de una decisión ni la validez de una traza.

La vigencia de resultados y reglas continúa dependiendo de ligaduras constituidas conforme a R1-3/3E y de las revalidaciones cerradas en R1-4.

## 11. Relación con la traza de ejercicio existente

`ExerciseTraceEntry` conserva la traza append-only del intento material de T-E dentro de `ExecutionContinuity`.

R1-5 no duplica esa traza como una segunda fuente. La enlaza mediante una relación determinista que preserva:

```text
ExerciseRef
+ autoridad
+ forma
+ EffectDescriptor
+ contexto
+ contrato de acumulación
+ estado técnico del intento
```

La vista agregada es derivable de los registros gobernados y no permite modificar retrospectivamente los eventos fuente.

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

La incapacidad de construir una traza exigida es un fallo técnico o estructural de trazabilidad, separado de los resultados semánticos y de comprobación.

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
T-E = PRODUCTIVA POR VÍA GOBERNADA
T-G = NO PRODUCTIVA
T-C = NO PRODUCTIVA
T-R = NO PRODUCTIVA
```

La traza no puede utilizarse como mecanismo indirecto para constituir autoridad o hacer productiva una clase bloqueada.

## 15. Exclusiones y deuda restante

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

Como endurecimiento defensivo no bloqueante, `append_decision` puede comprobar la eventual colisión de `DecisionTraceRef` antes de insertar. En el estado integrado no existe una vía pública reproducible para provocar esa colisión: la referencia se deriva de un ordinal privado, monotónico y no acotado al ancho de palabra de máquina.

`TraceLinkConflict` conserva además una defensa explícita frente a una asociación incompatible de un mismo `ExerciseRef`. La unicidad de ejercicio cerrada en R1-4 impide que esta condición constituya una vía productiva ordinaria.

## 16. Evidencia de cierre

La realización candidata se verificó en:

```text
head = 1bdb24f205c21ed61898d72b4444553c48156103
```

con:

```text
Conformidad SVP              #194 = success
R0 Rust                      #159 = success
R0-8 Baseline nativa         #111 = success
R0 WASM paridad tres vías    #107 = success

sv_core   = 198/198
sv_wasm   = 2/2
doc-tests = 17/17
R0-7      = 11/11 válidos + 61/61 inválidos
```

La batería integrada cubre, entre otros casos:

- cadena positiva desde T-0 hasta decisión, mediación y ejercicio confirmado;
- bloqueo D-R sin mediación ni ejercicio;
- acreditación resuelta D-A degradada a D-N por cobertura incompleta;
- conservación de comprobaciones individuales incompatibles antes de una resolución D-N;
- inaccesibilidad pública de las vías productivas sin traza;
- imposibilidad ordinaria de convertir una traza en autoridad, permiso o capacidad de ejecución.

La realización fue integrada en:

```text
main = 95a67b3f5ee3056477d5e34a17bcad201aaab9e5
```

## 17. Estado resultante

Los criterios de cierre de R1-5 quedan satisfechos dentro del alcance intra-proceso declarado.

```text
R1-5 = CERRADO · INTEGRADO
R1   = ABIERTO
R1-6 = ABIERTO

T-E = PRODUCTIVA POR VÍA GOBERNADA
T-G / T-C / T-R = NO PRODUCTIVAS

R2 / R3 / R4 = NO INICIADOS
BudgetΣ / IA-SEC = NO ABIERTOS
Garantía I / II = NO_PROBADO
```
