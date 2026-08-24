# R1-1 — Formas, autoridad, envolvente máxima y dominio gobernado

**Fecha:** 24 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte:** R1-1  
**Estado:** cerrado

## 1. Objeto

R1-1 fija la representación mínima de las formas sometidas a control, de la autoridad acotada, de la envolvente máxima de efectos `E_max(a | C)` y del dominio gobernado `D_a`.

El corte establece estructura y comprobaciones de alcance. No materializa todavía las transiciones T-*, la constitución operativa de autoridad, habilitación, `Req`, `Permit` ni ejecución de efectos.

## 2. Forma de transición

`FormDescriptor` fija de manera conjunta:

```text
identidad de forma
clase T-*
familia de efectos
ligaduras de contexto
autoridad previa requerida, cuando corresponda
contrato de acumulación
```

Los campos no disponen de mutadores públicos.

La comprobación de pertenencia de un efecto a una forma exige conjuntamente la familia de efectos y una ligadura de contexto constituida. Un resultado positivo sólo establece compatibilidad estructural con la forma; no acredita autoridad, habilitación, requisitos ni permiso.

## 3. Contrato de acumulación

R1-1 representa las clases siguientes:

```text
NotApplicable
SingleUse
Idempotent
GovernedAggregator(regla)
DecidableTracePredicate(regla)
```

El corte registra la clase del contrato de acumulación, pero no ejecuta todavía agregadores ni predicados sobre trazas. La aplicabilidad efectiva de esos contratos deberá conservar las reglas de SEC.0-A en los cortes posteriores.

## 4. Efecto descrito

`EffectDescriptor` liga un efecto a cuatro dimensiones:

```text
identidad
familia
objeto gobernado
contexto
```

La identidad nominal no sustituye las restantes ligaduras. Reutilizar una misma `EffectRef` con otra familia, otro objeto o otro contexto no produce el mismo alcance constituido.

## 5. Envolvente máxima `E_max(a | C)`

`EffectEnvelope` representa una envolvente finita e inmutable de efectos completamente descritos.

La pertenencia se decide sobre el descriptor completo del efecto y no únicamente sobre su identificador.

Por tanto:

```text
mismo EffectRef
+ familia distinta
⇒ fuera de E_max

mismo EffectRef
+ objeto distinto
⇒ fuera de E_max

mismo EffectRef
+ contexto distinto
⇒ fuera de E_max
```

No existen operaciones públicas de ampliación de la envolvente.

Esta primera realización no incorpora reglas generativas de `E_max`. La ausencia de esa capacidad es deliberadamente conservadora: una información, verificación, habilitación o ejercicio ordinarios no pueden ensanchar la envolvente por construcción.

## 6. Dominio gobernado `D_a`

`GovernedDomain` representa un conjunto finito e inmutable de objetos gobernados cuya pertenencia puede decidirse antes del ejercicio.

```text
x ∈ D_a
```

se resuelve mediante pertenencia exacta al conjunto constituido.

R1-1 no incorpora reglas generativas de dominio ni operaciones públicas que añadan objetos a `D_a`.

## 7. Autoridad acotada

`ConstitutedAuthority` representa la estructura de una autoridad ligada a:

```text
referencia de autoridad
titular
contexto
E_max(a | C)
D_a
```

La estructura no implementa `Clone`. Copiar una `AuthorityRef` sólo copia una referencia nominal y no duplica la autoridad representada.

La coherencia estructural exige que todo efecto incluido en `E_max`:

1. pertenezca al mismo contexto de la autoridad;
2. actúe sobre un objeto contenido en `D_a`.

Las pruebas del corte rechazan una envolvente que contenga efectos fuera del contexto o del dominio gobernado de la autoridad.

## 8. Ausencia de constitución productiva en R1-1

R1-1 no incorpora una vía operativa capaz de crear formas o autoridad en compilaciones de producción.

Los constructores brutos utilizados para ejercer los invariantes estructurales existen exclusivamente bajo `cfg(test)`.

En consecuencia, el código de producción de R1-1 no puede producir por sí mismo:

```text
FormDescriptor constituido
EffectDescriptor constituido
ConstitutedAuthority
EffectEnvelope constituida
GovernedDomain constituido
```

La primera vía productiva de constitución deberá incorporarse en R1-2 y demostrar que una forma o autoridad sólo nace mediante la clase T-* legítima que corresponda.

Esta reserva evita que una función interna de conveniencia se convierta anticipadamente en una vía de fabricación de autoridad.

## 9. Alcance de la autoridad de R1

La autoridad representada en este corte es exclusivamente intra-proceso y no se identifica con identidad o privilegio de implantación.

Se aplica conjuntamente la adenda:

`ADENDA_R1_0_ALCANCE_AUTORIDAD_CONTINUIDAD_Y_FRONTERA_DE_R1_2026_08_24.md`.

Por tanto, R1-1 no materializa credenciales externas, firma humana o criptográfica, identidad de proveedor, privilegios de sistema operativo, raíz material de confianza, ITI completo, agentes especializados ni conexión con motores de IA.

## 10. Pruebas estructurales

El corte incorpora casos que comprueban, entre otros extremos:

- inmovilidad de las dimensiones de una forma después de su construcción de prueba;
- exigencia conjunta de familia y contexto al contrastar una forma con un efecto;
- distinción de efectos con la misma referencia nominal y diferente familia, objeto o contexto;
- pertenencia decidible y exacta a `D_a`;
- ligadura conjunta de titular, contexto, `E_max` y `D_a`;
- rechazo de un efecto situado fuera del contexto de la autoridad;
- rechazo de un efecto situado fuera de su dominio gobernado;
- ausencia de ampliación por miembros duplicados.

## 11. Obligación trasladada a R1-2: unicidad constitutiva

R1-1 no materializa todavía un registro productivo de formas o autoridades y, por tanto, no atribuye unicidad global a sus referencias nominales.

Cuando R1-2 introduzca las primeras vías productivas de constitución deberá impedir, dentro del SUT de R1, que una misma referencia identifique simultáneamente constituciones incompatibles.

Como mínimo deberán quedar preservadas:

```text
mismo AuthorityRef
⇒ no dos autoridades constituidas incompatibles

mismo FormRef
⇒ no dos descriptores constituidos incompatibles
```

La mera igualdad de identificadores no resolverá conflictos entre constituciones. La creación, sustitución o modificación deberá proceder de la transición autorizante que corresponda y conservar la clasificación no discrecional de SEC.0-A.

Esta obligación no se considera satisfecha por R1-1; queda expresamente abierta para R1-2.

## 12. Límites

R1-1 no materializa:

- vías autorizantes T-0, T-C, T-G o T-R;
- restricción operativa de T-0 por continuidad;
- unicidad productiva de `AuthorityRef` o `FormRef`;
- habilitación;
- `Req(F,e | C)`;
- aplicabilidad de verificadores;
- agregación de `D-A`, `D-R` y `D-N`;
- `Permit`;
- ejecución o mediación de efectos protegidos;
- ligadura de una decisión concreta con su efecto;
- persistencia autoritativa durable;
- recuperación material;
- identidad externa;
- criptografía;
- Garantía I;
- Garantía II.

No modifica Gramática 0.2, IR 0.3, serializador 0.1.0 ni la semántica cerrada de R0.

## 13. Evidencia de cierre

El candidato técnico cerrado es:

```text
0ba7c69a482b8398f150e398979e58ef39e38692
```

Sobre ese corte concluyeron correctamente:

```text
Conformidad SVP              #103  SUCCESS
R0 Rust                       #76  SUCCESS
R0-8 Baseline nativa          #28  SUCCESS
R0 WASM paridad de tres vías  #24  SUCCESS
```

La regresión confirma que la incorporación de los tipos y comprobaciones de R1-1 no altera la conformidad, la realización Rust cerrada en R0, la referencia basal nativa ni la paridad WebAssembly heredada.

## 14. Cierre

Quedan satisfechos los criterios de R1-1:

1. los tipos y pruebas compilan en el mismo `sv_core`;
2. una referencia nominal no basta para ampliar `E_max` ni `D_a`;
3. familia, objeto y contexto permanecen ligados al efecto sometido a comprobación;
4. la autoridad no dispone de mutadores públicos de su alcance;
5. no existe una vía productiva de constitución anterior a R1-2;
6. las regresiones heredadas de R0 permanecen correctas;
7. R2–R4 y las Garantías I y II continúan fuera del alcance.

Estado resultante:

```text
R0 = CERRADO
R1 = ABIERTO
R1-0 = CERRADO
R1-1 = CERRADO
R1-2 = NO INICIADO
R2–R4 = NO INICIADOS
Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```

El cierre de R1-1 no abre automáticamente R1-2.
