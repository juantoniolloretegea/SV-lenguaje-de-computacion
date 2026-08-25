# Acta técnica de cierre de R1-4 y apertura de R1-5

**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte cerrado:** R1-4  
**Corte abierto:** R1-5

## 1. Objeto

Este documento constata el cierre de R1-4, dedicado al fallo cerrado, la decisión de permiso, la mediación no eludible y la ejecución gobernada de T-E dentro del perímetro intra-proceso de R1.

El cierre no acredita persistencia durable, correspondencia material independiente, confianza en el adaptador, productividad de T-G/T-C/T-R ni las Garantías I o II.

R1 permanece abierto porque su descomposición vigente incluye todavía R1-5, dedicado a la ligadura decisión–efecto y a la traza determinista, y R1-6, dedicado a regresión, contraste adversarial y cierre de fase.

## 2. Identidad del corte integrado

R1-4 queda consolidado sobre:

```text
main = bdc21c8dd93dfdde636a6122c60a23f9838a799e
```

El corte incorpora tres unidades materiales:

1. decisión sellada de permiso;
2. mediación no eludible del efecto protegido;
3. ejecución gobernada de T-E y registro lógico de ejercicio.

La última unidad integrada conserva una batería de referencia con:

```text
sv_core   = 192/192
sv_wasm   = 2/2
doc-tests = 12/12
R0-7      = 11/11 casos válidos equivalentes
            61/61 casos inválidos rechazados
```

Las líneas de conformidad, núcleo nativo/WebAssembly, línea base nativa y paridad de tres vías permanecen correctas en el corte final de R1-4.

## 3. Cadena material cerrada por R1-4

La vía gobernada de un efecto T-E protegido queda separada en objetos no intercambiables:

```text
resultado técnico R1-3
→ Permit
→ MediatedEffectCommitment
→ ExecutionRequest
→ registro DispatchCommitted
→ despacho al EffectExecutor
→ Confirmed | Indeterminate
```

Se preservan las separaciones:

```text
CheckResult ≠ Permit
Permit ≠ MediatedEffectCommitment
MediatedEffectCommitment ≠ ExecutionRequest
confirmación lógica ≠ prueba material independiente del mundo externo
fallo técnico de ejecución ≠ Tri.U
```

T-E sólo puede alcanzar el puerto de ejecución gobernado consumiendo un `MediatedEffectCommitment` válido. La operación vuelve a comprobar las ligaduras vigentes antes del despacho y registra el intento antes de invocar el adaptador.

## 4. Comprobación de las obligaciones de apertura de R1-4

| Nº | Obligación abierta al cerrar R1-3 | Evidencia material consolidada | Estado |
|---:|---|---|---|
| 1 | Representar una decisión de permiso no fabricable desde un resultado nominal. | `Permit` carece de constructor público y no existe conversión ordinaria desde `CheckResult`; sólo `decide_permit` puede formarlo. | CUMPLIDA |
| 2 | Derivar el permiso exclusivamente de autoridad y ligaduras constituidas junto con el resultado gobernado de R1-3. | `decide_permit` recupera forma, autoridad y `Req` de `AuthorityContinuity`, comprueba el alcance del efecto y agrega los resultados mediante la frontera gobernada de R1-3. | CUMPLIDA |
| 3 | Impedir que D-R, D-N o un error técnico produzcan permiso positivo. | D-R y D-N producen `PermitDecision::NotGranted`; los fallos estructurales permanecen como errores tipados. | CUMPLIDA |
| 4 | Ligar el permiso al efecto protegido concreto y a su contexto material. | `Permit` conserva autoridad, forma, contexto seleccionado, `EffectDescriptor` completo, acumulación y resultados gobernantes necesarios para revalidación. | CUMPLIDA |
| 5 | Mediar el compromiso del efecto de forma no eludible dentro de `sv_core`. | `mediate_permit` consume `Permit`, revalida efecto, forma, autoridad, `Req`, reutilización 3E y aplicabilidades participantes, y sólo entonces forma `MediatedEffectCommitment`. | CUMPLIDA |
| 6 | Preservar la diferencia entre permiso concedido y efecto ejecutado. | La ejecución exige un objeto posterior distinto; `execute_mediated` consume `MediatedEffectCommitment`, forma una solicitud sellada y registra el intento antes del adaptador. | CUMPLIDA |
| 7 | Mantener `Tri`, gramática e IR fuera de esta decisión de control salvo corte expreso. | R1-4 no modifica la semántica ternaria, la gramática ni la representación intermedia; las regresiones nativa y WebAssembly permanecen correctas. | CUMPLIDA |

No queda una obligación propia de R1-4 sin sede material identificable.

## 5. Propiedades adicionales preservadas

El cierre mantiene las siguientes fronteras:

```text
T-E = productiva sólo por decisión + mediación + ejecución gobernadas
T-G = NO PRODUCTIVA
T-C = NO PRODUCTIVA
T-R = NO PRODUCTIVA
```

T-E no constituye autoridad nueva, no amplía `E_max`, no amplía `D_a` y no crea formas.

Los contratos `GovernedAggregator` y `DecidableTracePredicate` permanecen cerrados mientras no exista su evaluador gobernado. `SingleUse` e `Idempotent` se aplican en la traza lógica intra-proceso sin atribuir durabilidad entre reinicios o réplicas.

## 6. Límites que no se convierten en deuda de R1-4

Quedan expresamente fuera del cierre:

- identidad durable entre procesos;
- almacenamiento autoritativo durable;
- atomicidad entre efecto externo y registro local;
- exclusión distribuida;
- atestación de plataforma;
- honestidad material del adaptador;
- recuperación durable;
- productividad de T-G, T-C y T-R;
- `BudgetΣ` e IA-SEC;
- R2, R3 y R4;
- Garantía I;
- Garantía II.

Una respuesta positiva del adaptador sólo permite registrar una confirmación lógica local. No prueba por sí sola que el mundo externo haya realizado exactamente el efecto declarado.

## 7. Decisión de cierre

Se establece:

```text
R1-4 = CERRADO · INTEGRADO
R1   = ABIERTO
```

No procede abrir una unidad adicional dentro de R1-4. La siguiente obligación vigente de la fase es R1-5.

## 8. Apertura de R1-5

R1-5 se abre con el objeto definido por la descomposición original de R1:

```text
R1-5 = ligadura decisión–efecto y traza determinista
```

R1-4 entrega ya piezas causales suficientes para impedir la elusión —`Permit`, `MediatedEffectCommitment`, `ExerciseRef` y la traza de ejercicios—, pero eso no equivale todavía a una traza gobernada única de la decisión protegida.

R1-5 deberá conservar de manera determinista, para cada decisión protegida que alcance una conclusión gobernada, al menos:

```text
forma
+ efecto pretendido
+ contexto
+ autoridad
+ Req aplicable
+ resultado individual de cada obligación
+ resultado agregado
+ permiso o bloqueo
+ compromiso mediado, cuando exista
+ ejercicio comprometido, cuando exista
```

La traza deberá preservar D-A, D-R y D-N sin convertirlos entre sí ni en `Tri`. Un bloqueo deberá ser trazable sin necesidad de fabricar un `Permit` negativo.

## 9. Frontera específica de R1-5

R1-5 no volverá a decidir autoridad, requisitos, permiso o ejecución. Su función será ligar de forma explícita los objetos ya gobernados y demostrar que la historia causal puede reconstruirse sin depender de reloj ambiental, orden accidental de colecciones o texto libre interpretable.

La traza no será autoridad, evidencia externa independiente ni capacidad de ejecución. Copiar o consultar una entrada no podrá producir `Permit`, `MediatedEffectCommitment`, `ExecutionRequest` o autoridad.

La persistencia durable de esa traza sigue reservada a R2 o fases posteriores.

## 10. Estado resultante

```text
R0   = CERRADO
R1   = ABIERTO
R1-0 = CERRADO
R1-1 = CERRADO
R1-2 = CERRADO · INTEGRADO
R1-3 = CERRADO · INTEGRADO
R1-4 = CERRADO · INTEGRADO
R1-5 = ABIERTO
R1-6 = NO INICIADO

R2   = NO INICIADO
R3   = NO INICIADO
R4   = NO INICIADO

T-E             = PRODUCTIVA POR VÍA GOBERNADA
T-G / T-C / T-R = NO PRODUCTIVAS

Garantía I  = NO_PROBADO
Garantía II = NO_PROBADO
```
