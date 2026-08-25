# Contrato preparatorio de R1-3 — requisitos, aplicabilidad y resultados de comprobación

**Fecha:** 25 de agosto de 2026  
**Ámbito previsto:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte previsto:** R1-3  
**Estado:** preparación contractual · R1-3 no iniciado

## 1. Objeto

Este documento prepara el contrato de apertura de R1-3. No inicia su realización, no modifica `main` y no atribuye a R1-3 propiedades todavía no materializadas.

El objeto del corte será representar, dentro de `sv_core`, los requisitos aplicables a una forma sujeta a control y los resultados técnicos de su comprobación, preservando la separación entre diagnóstico, semántica ternaria, autoridad y permiso.

R1-3 deberá apoyarse en SEC.0-D y en las fronteras ya cerradas de R1-0, R1-1 y R1-2. No reabrirá R0 ni modificará la génesis T-0 ya cerrada.

## 2. Punto de partida

Estado gobernado previo a la apertura de R1-3:

```text
R0 = CERRADO
R1 = ABIERTO
R1-0 = CERRADO
R1-1 = CERRADO
R1-2 = CERRADO · INTEGRADO
R1-3 = NO INICIADO
R2 = NO INICIADO
R3 = NO INICIADO
R4 = NO INICIADO

Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```

R1-2 mantiene T-G, T-C y T-R no productivas. La existencia de descriptores constituidos de esas clases no equivale a autorización para ejecutarlos.

## 3. Requisitos aplicables

Para toda forma sujeta a control, efecto `e` y contexto constitutivo `C`, R1-3 deberá materializar un conjunto no vacío de obligaciones:

```text
Req(F,e | C) = N(F,e | C) ∪ S(F,e | C)
```

con:

```text
Req(F,e | C) = ∅
⇒ forma inválida para producir el efecto
```

La aplicabilidad no podrá decidirla durante el acto el ejecutor, el beneficiario del efecto, el verificador ni un adaptador. Deberá derivar de la forma constituida y de las ligaduras de contexto previamente gobernadas.

`N(F,e | C)` representa el núcleo no eludible de control. `S(F,e | C)` contiene obligaciones adicionales específicas de la forma.

## 4. Núcleo mínimo no eludible

El contrato de realización deberá preservar, cuando sean aplicables, las obligaciones nucleares fijadas por SEC.0-D:

1. validez y aplicabilidad de la forma;
2. autoridad aplicable;
3. pertenencia al dominio gobernado `D_a`, cuando corresponda;
4. admisibilidad y aplicabilidad de la regla de verificación;
5. prohibición de acreditación propia;
6. vigencia o no revocación, cuando la forma, la autoridad o el antecedente dependan de ella.

Las obligaciones 1, 2, 4 y 5 deberán permanecer nucleares en toda forma sujeta a control. Las obligaciones 3 y 6 se incorporarán cuando su condición constitutiva sea aplicable.

Omitir una obligación nuclear no podrá modelarse como una decisión operativa local. Exigirá una modificación constitutiva legítima de la forma en una fase capaz de producirla.

## 5. Identidad de las obligaciones

R1-3 deberá impedir que dos obligaciones materialmente distintas se confundan por compartir una etiqueta textual o una referencia reutilizada.

La representación prevista deberá ligar cada obligación, como mínimo, a:

```text
identidad de obligación
+ forma constituida
+ efecto o familia de efectos
+ contexto constitutivo
+ familia de verificadores admisibles
+ condición de aplicabilidad
```

Las ligaduras que afecten a vigencia, autoridad, objeto gobernado o antecedente deberán formar parte de la identidad material cuando su variación pueda cambiar el resultado.

La mera copia de una referencia nominal no podrá extender la aplicabilidad de la obligación a otro objeto, efecto o contexto.

## 6. Resultados técnicos cerrados

R1-3 utilizará exclusivamente los resultados abstractos ya fijados:

```text
D-A = ACREDITADO
D-R = REFUTADO
D-N = NO_VERIFICABLE
```

con las separaciones obligatorias:

```text
D-A ≠ D-R
D-A ≠ D-N
D-R ≠ D-N
D-N ≠ Tri.U
fallo técnico ≠ Tri.U
```

Un resultado de comprobación no constituye autoridad, no modifica por sí mismo `E_max` o `D_a` y no produce un permiso.

## 7. Aplicabilidad de verificadores

Para una obligación `q`, un verificador concreto `V` sólo podrá intervenir cuando la aplicabilidad correspondiente esté previamente constituida:

```text
Applicable(V, q, C) = true
```

La mera disponibilidad o ejecución de `V` no acreditará su aplicabilidad.

Si la aplicabilidad no puede acreditarse y no existe otra regla admisible suficiente:

```text
Check(q) = D-N
```

Un verificador no podrá acreditar en el mismo acto la autoridad o legitimidad de la que dependa su propio uso.

## 8. Conflicto entre evidencias o verificadores

Si evidencias admisibles o verificadores aplicables producen resultados incompatibles, R1-3 sólo podrá obtener `D-A` o `D-R` mediante una regla de resolución previamente constituida y aplicable.

En ausencia de esa regla:

```text
Check(q) = D-N
```

Quedan excluidos como reglas implícitas de resolución:

- elegir el resultado favorable;
- elegir por mera cronología;
- elegir por mayoría no autorizada;
- escoger el verificador interesado en el efecto;
- degradar la incompatibilidad a advertencia para continuar.

## 9. Agregación técnica

R1-3 deberá materializar la agregación de resultados de requisitos de forma determinista:

```text
D-R  si existe al menos una obligación REFUTADA;
D-N  si ninguna está REFUTADA y existe al menos una NO_VERIFICABLE;
D-A  sólo si todas están ACREDITADAS.
```

La precedencia de `D-R` sobre `D-N` conservará la existencia de una refutación material. No implica que `D-R` sea un valor ternario ni un permiso negativo del dominio.

La agregación describe el estado técnico del control. No ejecuta el efecto.

## 10. Cobertura y reutilización de resultados

Una acreditación parcial no podrá promoverse a acreditación total sin una regla de cobertura previamente constituida.

```text
Check(subconjunto) = D-A
↛ Check(totalidad) = D-A
```

Un resultado histórico sólo podrá reutilizarse cuando continúen acreditadas todas las ligaduras materialmente relevantes de su validez. Cuando esa continuidad no pueda acreditarse:

```text
resultado previo = no reutilizable
Check(q) = D-N
```

Un reintento no transformará por sí mismo `D-N` en `D-A`.

## 11. Relación con R1-2

R1-3 no modificará la regla cerrada de génesis:

```text
T-0
= única vía productiva de autoridad de R1-2
```

T-I, T-V, T-H y T-E seguirán sin constituir autoridad.

T-G, T-C y T-R podrán disponer en R1-3 de requisitos y resultados de comprobación aplicables, pero **seguirán sin producir efectos protegidos por el mero hecho de haber sido comprobadas**. La producción efectiva de un permiso y la mediación del efecto pertenecen a R1-4.

Por tanto:

```text
Req completo
+ todos los Check(q) = D-A
≠ efecto ejecutado
```

R1-3 prepara el antecedente técnico necesario para la regla de fallo cerrado; no materializa todavía la ejecución protegida.

## 12. Fuentes auxiliares y propuestas externas

Una fuente externa, un adaptador, una herramienta, una persona o un sistema auxiliar podrá aportar información o evidencia candidata dentro de las reglas aplicables, pero no podrá decidir por sí mismo:

- qué obligaciones integran `Req`;
- qué verificador es aplicable;
- que un resultado debe ser `D-A`;
- que una incompatibilidad puede omitirse;
- que un requisito no resulta aplicable para favorecer el efecto.

La coincidencia de varias fuentes no constituye por sí sola una regla de resolución ni una autoridad adicional.

Este principio es general. Los perfiles concretos de canales o sistemas auxiliares deberán gobernarse separadamente cuando corresponda y no forman parte del objeto específico de R1-3.

## 13. Frontera con R1-4

R1-3 no deberá introducir un tipo productivo de `Permit` ni una operación capaz de comprometer un efecto protegido.

El cierre de R1-3 sólo acreditará que existe una representación cerrada y verificable de:

```text
forma + efecto + contexto
→ Req aplicable
→ comprobaciones individuales
→ resultado técnico agregado
```

La transformación de ese resultado en una decisión de permiso y la mediación del efecto se reservarán a R1-4.

## 14. Pruebas mínimas previstas

La futura realización deberá incluir, al menos, pruebas negativas y positivas para demostrar que:

1. una forma sujeta a control con `Req = ∅` es inválida para producir efecto;
2. el beneficiario del efecto no puede eliminar obligaciones aplicables;
3. una obligación nuclear no puede omitirse localmente;
4. `D-A`, `D-R` y `D-N` permanecen disjuntos;
5. `D-N` no puede convertirse en `Tri.U`;
6. un fallo técnico de comprobación produce `D-N` cuando corresponda y no `Tri.U`;
7. un verificador no aplicable no puede producir `D-A` válido;
8. un verificador no puede autolegitimar su propia aplicabilidad;
9. un conflicto sin regla de resolución produce `D-N`;
10. la agregación conserva `D-R` si existe una refutación;
11. la agregación produce `D-N` cuando no existe refutación y hay al menos una no verificabilidad;
12. sólo un conjunto íntegramente acreditado agrega a `D-A`;
13. una acreditación parcial no acredita la totalidad sin regla de cobertura;
14. un resultado histórico no puede reutilizarse fuera de sus ligaduras;
15. R1-3 no produce `Permit` ni ejecuta efectos;
16. T-G, T-C y T-R siguen sin producir cambios efectivos antes de R1-4;
17. las regresiones de R0, R1-0, R1-1 y R1-2 permanecen correctas.

## 15. Exclusiones

R1-3 no materializará:

- `Permit` productivo;
- mediación de efectos protegidos;
- persistencia durable de resultados;
- revocación durable;
- continuidad entre procesos;
- recuperación material;
- verificación criptográfica concreta;
- identidad externa;
- canales de red o dispositivos;
- gestión de secretos;
- aislamiento de plataforma;
- interfaz de usuario como autoridad;
- cliente o motor de inteligencia artificial;
- Garantía I;
- Garantía II.

Tampoco abre R2, R3 o R4.

## 16. Condición de apertura

La existencia de este contrato preparatorio no inicia R1-3.

Para abrir R1-3 será necesario un acto separado que, como mínimo:

1. fije el `main` exacto de partida;
2. confirme que R1-2 permanece cerrado e integrado;
3. adopte o corrija este contrato;
4. determine el primer corte de tipos y fronteras a materializar;
5. mantenga R1-4, R2, R3 y R4 sin iniciar por inferencia.

Hasta entonces:

```text
R0 = CERRADO
R1 = ABIERTO
R1-0 = CERRADO
R1-1 = CERRADO
R1-2 = CERRADO · INTEGRADO
R1-3 = NO INICIADO
R1-4 = NO INICIADO
R2–R4 = NO INICIADOS
Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```
