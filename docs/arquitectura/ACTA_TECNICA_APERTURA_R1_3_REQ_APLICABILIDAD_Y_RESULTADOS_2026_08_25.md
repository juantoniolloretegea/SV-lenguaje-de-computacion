# Apertura de R1-3 — requisitos, aplicabilidad y resultados de comprobación

**Fecha:** 25 de agosto de 2026  
**Repositorio:** `SV-lenguaje-de-computacion`  
**Base de apertura:** `2d0d2ebb69364f81090ce019345bcc33c0a3e613`  
**Estado:** R1-3 abierto

## 1. Decisión

Se abre R1-3 como corte separado de R1 para materializar `Req(F,e | C)`, la aplicabilidad previamente constituida de verificadores y los resultados técnicos `D-A`, `D-R` y `D-N`.

La apertura adopta el contrato `CONTRATO_R1_3_REQ_APLICABILIDAD_Y_RESULTADOS_2026_08_25.md`.

No se abre R1-4 por inferencia y no se modifica el alcance cerrado de R0, R1-0, R1-1 o R1-2.

## 2. Frontera de realización

R1-3 queda limitado al estado técnico de control:

```text
forma + efecto + contexto
→ Req aplicable
→ Check(q)
→ resultado agregado
```

Se mantiene expresamente:

```text
Req completo
+ todos los Check(q) = D-A
≠ Permit
≠ efecto ejecutado
```

T-G, T-C y T-R no adquieren productividad por la apertura de R1-3.

## 3. Aplicabilidad no discrecional

Las relaciones siguientes no son parámetros libres del acto de comprobación:

```text
N(F,e | C)
Applicable(V,q,C)
```

Su contenido debe derivar de estado previamente constituido y de ligaduras gobernadas.

El ejecutor, beneficiario, adaptador, verificador o fuente auxiliar no pueden suprimir obligaciones ni declarar por sí mismos la aplicabilidad de un verificador.

Un verificador no puede acreditar en el mismo acto la legitimidad de la que depende su propio uso.

## 4. Resultados técnicos

R1-3 conserva exclusivamente:

```text
D-A = ACREDITADO
D-R = REFUTADO
D-N = NO_VERIFICABLE
```

con:

```text
D-N ≠ Tri.U
fallo técnico ≠ Tri.U
```

La agregación deberá ser determinista:

```text
algún D-R                     ⇒ D-R
ningún D-R + algún D-N        ⇒ D-N
todos D-A                     ⇒ D-A
conjunto vacío                ⇒ inválido
```

Ningún resultado técnico constituye autoridad, amplía `E_max` o `D_a`, produce permiso o ejecuta un efecto.

## 5. Primera unidad material

La primera unidad de realización de R1-3 queda limitada a:

- referencias cerradas de obligaciones y verificadores;
- estructuras inmutables para obligación y aplicabilidad;
- reutilización del tipo cerrado `CheckResult` ya introducido en R1;
- agregación determinista con rechazo del conjunto vacío;
- pruebas negativas de separación respecto de `Tri`, autoridad y permiso.

Esta primera unidad no ofrecerá una vía productiva para constituir requisitos durante el propio acto de comprobación. La constitución productiva necesaria para cerrar R1-3 deberá quedar gobernada dentro del mismo corte en una unidad posterior.

## 6. Exclusiones

La apertura no materializa:

- `Permit`;
- mediación de efectos protegidos;
- ejecución productiva de T-G, T-C o T-R;
- persistencia durable;
- recuperación material;
- identidad externa;
- canales de red o dispositivos;
- gestión de secretos;
- aislamiento de plataforma;
- cliente o motor de inteligencia artificial;
- Garantía I;
- Garantía II.

## 7. Estado

```text
R0 = CERRADO
R1 = ABIERTO
R1-0 = CERRADO
R1-1 = CERRADO
R1-2 = CERRADO · INTEGRADO
R1-3 = ABIERTO
R1-4 = NO INICIADO
R2 = NO INICIADO
R3 = NO INICIADO
R4 = NO INICIADO

Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```
