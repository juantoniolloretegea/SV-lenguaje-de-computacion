# R1-4 — Unidad 1: decisión sellada de permiso

**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte:** R1-4  
**Estado:** realización candidata de cierre de la unidad 1

## 1. Objeto

Esta unidad materializa la primera frontera de R1-4: la decisión de permiso para un efecto protegido concreto.

La unidad no ejecuta efectos y no hace productivas T-G, T-C o T-R. Su único producto positivo es un `Permit` sellado cuya formación exige estado constituido de autoridad y un resultado técnico gobernado completo de R1-3.

Se conserva:

```text
CheckResult ≠ Permit
Permit ≠ efecto ejecutado
```

## 2. Punto de partida

La realización parte de:

```text
main = c724b9debc520628dc83c7bbc582e21c933d5dc6
```

con:

```text
R1-3 = CERRADO · INTEGRADO
R1-4 = ABIERTO
```

R1-3 entrega la cadena gobernada de requisitos, aplicabilidad, comprobaciones, resolución de conflicto, cobertura y reutilización ligada. Esta unidad consume esa frontera sin redefinir sus resultados.

## 3. Regla de formación

La decisión se obtiene mediante una única operación gobernada:

```text
AuthorityContinuity
+ FormRef
+ EffectDescriptor constituido
+ ResolvedRequirementResult[]
        ↓
forma constituida recuperada del estado
        ↓
autoridad requerida recuperada del estado
        ↓
comprobación de forma, familia y contexto
        ↓
comprobación de E_max y D_a
        ↓
Req exacto recuperado del estado
        ↓
agregación cubierta de R1-3
        ↓
PermitDecision
```

La operación no recibe como parámetros elegibles por el llamador:

- un `CheckResult` final;
- la autoridad que se pretende aplicar;
- un indicador de pertenencia a `E_max`;
- un indicador de pertenencia a `D_a`;
- una regla de cobertura;
- un booleano de permiso.

## 4. Condición positiva

La formación positiva exige conjuntamente:

1. que la forma exista en la continuidad constituida;
2. que la forma nombre una autoridad requerida;
3. que esa autoridad exista en la misma continuidad;
4. que la forma describa la familia y el contexto del efecto;
5. que el efecto concreto pertenezca al alcance constituido de la autoridad;
6. que su objeto pertenezca al dominio gobernado de la autoridad;
7. que exista el `RequirementSet` constituido para la misma forma, familia y contexto;
8. que la agregación gobernada por cobertura de R1-3 produzca `D-A`.

Sólo entonces:

```text
D-A gobernado
+ ligaduras de forma válidas
+ autoridad aplicable
+ efecto ∈ E_max
+ objeto ∈ D_a
→ PermitDecision::Granted(Permit)
```

No existe una conversión:

```text
CheckResult::Accredited → Permit
```

## 5. Fallo cerrado

Los resultados técnicos no positivos de R1-3 producen ausencia de permiso positivo:

```text
D-R → NotGranted(RefutedRequirements)
D-N → NotGranted(NotVerifiableRequirements)
```

La ausencia de permiso no se convierte en `Tri.U` ni cambia el resultado técnico original.

Las incompatibilidades estructurales se mantienen como errores tipados, entre ellos:

- forma desconocida;
- forma sin autoridad requerida;
- autoridad requerida no disponible en el estado;
- efecto incompatible con la forma;
- efecto fuera del alcance de autoridad;
- ausencia del `RequirementSet` exacto;
- error estructural en la agregación gobernada de R1-3.

Un error estructural nunca produce un permiso positivo.

## 6. Identidad material del permiso

El `Permit` positivo conserva como mínimo:

```text
autoridad
+ titular de autoridad
+ contexto de autoridad
+ forma
+ clase de transición
+ familia de efectos de la forma
+ contexto del acto
+ autoridad requerida por la forma
+ contrato de acumulación
+ efecto concreto
+ objeto gobernado
+ ligadura forma/familia/contexto de Req
+ resultado técnico D-A
```

La mera conservación de una referencia nominal no sustituye esas ligaduras.

El tipo `Permit` no implementa `Clone` y no ofrece constructor público.

## 7. Cobertura no eludible

La decisión no agrega comprobaciones por una vía nueva. Reutiliza la agregación gobernada de R1-3.

Por tanto:

```text
comprobaciones acreditadas
+ cobertura incompleta
→ D-N final de R1-3
→ no permiso positivo
```

Un verificador aplicable pero no exigido no sustituye a un verificador requerido por la regla de cobertura constituida.

## 8. No fabricación

La frontera pública impide fabricar `Permit` desde:

- `CheckResult`;
- `AuthorityRef`;
- `FormRef`;
- `EffectRef`;
- `ControlId`;
- un booleano;
- una propuesta externa.

Los campos del permiso son privados. La única función productiva de formación es `decide_permit`.

## 9. Ausencia de ejecución

Esta unidad no contiene una operación que consuma `Permit` para comprometer un efecto protegido.

Por tanto:

```text
Permit válido
≠ efecto ejecutado
```

La mediación productiva deberá añadirse en una unidad posterior de R1-4 y tendrá que comprobar nuevamente, en el punto de compromiso, que el permiso corresponde al mismo efecto y a las mismas ligaduras materiales.

No se considera cerrada la mediación por la mera existencia de `Permit`.

## 10. Clases T-*

La unidad no modifica `transition_disposition`.

En particular:

```text
T-G = BlockedPendingRequirements
T-C = BlockedPendingRequirements
T-R = BlockedPendingRequirements
```

La etiqueta histórica de esa variante no implica que R1-3 siga abierto; expresa que esas clases aún no disponen de la mediación productiva exigida por R1-4.

T-I, T-V, T-H y T-E continúan sin constituir autoridad por sí mismas.

## 11. Pruebas de la unidad

La batería específica comprueba, sobre una continuidad constituida mediante T-0, que:

1. un `D-A` gobernado, con cobertura completa y alcance correcto, forma un `Permit` sellado;
2. `D-R` no forma permiso positivo;
3. `D-N` no forma permiso positivo;
4. una acreditación con cobertura incompleta no forma permiso positivo;
5. una familia de efecto incompatible con la forma produce rechazo cerrado;
6. un efecto perteneciente a otra autoridad no puede emplearse con la autoridad requerida por la forma;
7. una forma sin autoridad requerida no puede formar permiso protegido;
8. T-G, T-C y T-R continúan no productivas;
9. la API pública no permite construir `Permit` desde un resultado técnico nominal ni mediante constructor ordinario.

Las regresiones completas del repositorio deberán permanecer correctas antes de integrar la unidad.

## 12. Límites y trabajo posterior

Esta unidad no materializa:

- mediación productiva del efecto;
- consumo de `Permit`;
- ejecución material;
- constatación de ejecución;
- reutilización de permisos;
- persistencia durable;
- recuperación durable;
- continuidad entre procesos;
- `BudgetΣ`;
- seguridad de plataforma de R3;
- Garantía I;
- Garantía II.

La unidad siguiente de R1-4 deberá cerrar la mediación no eludible del efecto antes de hacer productiva cualquier clase que pueda comprometer un cambio protegido.

## 13. Estado

La realización de esta unidad mantiene:

```text
R0   = CERRADO
R1   = ABIERTO
R1-0 = CERRADO
R1-1 = CERRADO
R1-2 = CERRADO · INTEGRADO
R1-3 = CERRADO · INTEGRADO
R1-4 = ABIERTO

R2   = NO INICIADO
R3   = NO INICIADO
R4   = NO INICIADO

Garantía I  = NO_PROBADO
Garantía II = NO_PROBADO
```
