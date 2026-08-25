# R1-3 — Unidad 2: constitución inicial de requisitos y aplicabilidad

**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte:** R1-3  
**Estado:** contrato de realización de la segunda unidad

## 1. Objeto

Esta unidad materializa la constitución inicial de `Req(F,e | C)` y de las relaciones `Applicable(V,q,C)` necesarias para formas sujetas a control que nacen en la génesis lógica T-0.

La unidad se superpone al contrato ya cerrado de R1-2 sin redefinirlo. Conserva la premisa externa opaca, la restricción de T-0 a una continuidad no habitada, el consumo único de la premisa en caso de éxito y la atomicidad del rechazo.

No ejecuta comprobaciones, no produce `D-A`, `D-R` o `D-N`, no produce `Permit`, no ejecuta efectos protegidos y no hace productivas T-G, T-C o T-R.

Base de realización:

```text
main = 6fee28df75ae9c14067b15a7f1f6ab4341fa098e
```

## 2. Regla de superposición sobre T-0

R1-2 permanece íntegro:

```text
premisa externa opaca
+ continuidad no habitada
+ plan inicial válido
→ T-0
```

R1-3 añade únicamente la condición siguiente para las formas que, por su constitución, quedan sujetas a control:

```text
T-0 de R1-2
+
para cada ligadura controlada (F, familia de efectos, C):
    Req(F,e | C) constituible y no vacío
    + relaciones de aplicabilidad iniciales coherentes, si se declaran
──────────────────────────────────────────────────────────────
T-0 completa o rechazo atómico
```

La ausencia de formas sujetas a control no obliga a introducir requisitos artificiales. Una génesis compuesta exclusivamente por formas no sujetas a control conserva el comportamiento de R1-2.

## 3. Predicado cerrado de sujeción a control

SEC.0-D establece que la sujeción a control deriva de la constitución de la forma y no puede decidirla durante la ejecución el ejecutor, el beneficiario, el verificador o un adaptador.

En la representación disponible en R1-2, una forma inicial queda sujeta a control cuando se cumple al menos una de estas condiciones constitutivas:

```text
required_authority(F) ≠ ∅

O

class(F) ∈ {T-G, T-C, T-R}
```

La primera condición cubre cualquier forma cuya ejecución, habilitación, verificación o información haya sido constituida con dependencia de autoridad previa. La segunda cubre las clases que, por definición contractual, pueden modificar gobierno, constitución o recuperación y que R1-2 ya exige ligar a autoridad.

No existe un campo libre `subject_to_control: bool`.

T-I, T-V, T-H o T-E sin autoridad previa no quedan sujetas a control por su mera etiqueta en esta primera realización. Si una evolución posterior incorpora efectos protegidos o persistencia cuya naturaleza no quede representada por las ligaduras actuales, deberá ampliar la constitución de la forma mediante el corte que corresponda; no podrá inferirlo localmente durante una comprobación.

## 4. Propuestas y objetos constituidos

La entrada ordinaria de T-0 utiliza propuestas, no objetos ya constituidos:

```text
RequirementProposal
≠ RequirementDescriptor
≠ RequirementSet

ApplicabilityProposal
≠ VerifierApplicability
```

Las propuestas forman parte del mismo `GenesisPlan` consumido por T-0. No existe una operación posterior que permita completar requisitos o aplicabilidad sobre una continuidad ya habitada dentro de esta unidad.

`RequirementSet` y `VerifierApplicability` continúan sin constructor público productivo.

## 5. Constitución de `Req`

Para cada combinación controlada de:

```text
forma
+ familia de efectos
+ contexto constitutivo
```

debe constituirse un conjunto no vacío de obligaciones.

La constitución rechazará:

- conjunto vacío;
- referencia de obligación duplicada;
- reutilización materialmente incoherente de una misma referencia;
- forma inexistente en el plan;
- familia de efectos distinta de la constituida por la forma;
- contexto no ligado por la forma;
- obligación sin familia de verificadores admisibles;
- ausencia de cualquiera de las cuatro obligaciones nucleares incondicionales.

Las obligaciones nucleares incondicionales son:

```text
FormValidity
ApplicableAuthority
VerifierAdmissibilityAndApplicability
NoSelfAccreditation
```

`GovernedDomainMembership` y `ValidityOrNonRevocation` permanecen condicionales conforme a SEC.0-D. Esta unidad no las convierte en obligaciones universales.

Las obligaciones específicas adicionales forman parte de la constitución inicial y no pueden añadirse ni suprimirse durante el acto posterior de comprobación.

## 6. Constitución de `Applicable(V,q,C)`

Una propuesta de aplicabilidad sólo puede convertirse en relación constituida si:

- la obligación referida existe en el `Req` que se está constituyendo;
- el contexto coincide;
- la familia del verificador pertenece a las familias admisibles de la obligación;
- la regla de aplicabilidad coincide con la fijada por la obligación;
- no existe una relación duplicada para la misma ligadura material.

La constitución de `Applicable(V,q,C)` no ejecuta al verificador ni acredita el cumplimiento de `q`.

```text
Applicable(V,q,C)
≠ Check(q)
≠ D-A
```

La falta de una relación concreta de aplicabilidad no convierte la génesis en éxito diagnóstico ni en permiso. R1-3 conserva para las unidades posteriores la consecuencia `D-N` cuando no pueda acreditarse un verificador aplicable suficiente.

El verificador no dispone de una operación pública capaz de constituir su propia aplicabilidad. La relación sólo puede nacer dentro de la puerta constitutiva de T-0 de esta unidad.

## 7. Prohibición de circularidad diagnóstica

T-0 no depende de ejecutar `Check`.

Queda prohibido:

```text
Check(q) = D-A
→ autorizar la propia constitución de Req o Applicable
```

La unidad 2 constituye el régimen inicial de comprobación; no lo utiliza como pasaporte para la génesis que lo hace existir.

## 8. Atomicidad

La validación de formas, autoridades, requisitos y aplicabilidad se completa antes de comprometer el nuevo estado.

Cualquier defecto produce:

```text
T-0 = rechazada
continuidad = no habitada
premisa externa = no consumida
formas nuevas = 0
autoridades nuevas = 0
Req nuevos = 0
Applicable nuevos = 0
```

No existe aceptación parcial por forma ni por autoridad.

## 9. Régimen inicial y límites

Esta unidad materializa exclusivamente la constitución inicial en T-0.

No crea una segunda puerta productiva de requisitos. La mutación posterior de `Req` o de `Applicable` queda fuera de esta unidad y no habilita por inferencia T-G, T-C o T-R.

La existencia de requisitos completos tampoco vuelve productivas esas transiciones:

```text
Req completo
+ todos los resultados técnicos eventualmente en D-A
≠ Permit
≠ efecto ejecutado
```

La decisión y mediación del efecto pertenecen a R1-4.

## 10. Pruebas mínimas de esta unidad

La realización deberá demostrar, como mínimo:

1. forma sujeta a control sin `Req` válido ⇒ rechazo atómico;
2. omisión de núcleo incondicional ⇒ rechazo atómico;
3. autoridad y forma válidas + `Req` inválido ⇒ ninguna constitución parcial;
4. `RequirementSet` y `VerifierApplicability` siguen sin construcción pública ordinaria;
5. una propuesta de aplicabilidad con familia, regla, obligación o contexto incoherentes ⇒ rechazo;
6. una génesis formada sólo por formas no sujetas a control puede seguir completándose sin `Req`;
7. una segunda T-0 continúa rechazada;
8. una continuidad habitada no ofrece vía para completar `Req` después;
9. la agregación de la primera unidad conserva `CheckResult::Accredited ≠ obligación acreditada`;
10. no existe `Permit` productivo ni ejecución de efecto en esta unidad;
11. las regresiones de R0, R1-0, R1-1 y R1-2 permanecen correctas en sus propiedades cerradas.

## 11. Estado

```text
R0 = CERRADO
R1 = ABIERTO
R1-0 = CERRADO
R1-1 = CERRADO
R1-2 = CERRADO · INTEGRADO
R1-3 = ABIERTO

R1-3 / unidad 1 = CERRADA · AUDITADA · INTEGRADA
R1-3 / unidad 2 = EN REALIZACIÓN

R1-4 = NO INICIADO
R2–R4 = NO INICIADOS
Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```
