# R1-4 — Unidad 2: mediación no eludible del efecto protegido

**Estado:** CANDIDATA DE CIERRE  
**Fecha:** 25 de agosto de 2026

## 1. Objeto

Esta unidad materializa la frontera entre una decisión positiva de permiso y el punto gobernado desde el que podrá comprometerse posteriormente un efecto protegido.

La unidad no ejecuta efectos externos. Su salida positiva es un compromiso mediado sellado.

La relación mínima es:

```text
Permit concedido
+ mismo efecto concreto
+ forma vigente
+ autoridad vigente
+ alcance vigente E_max / D_a
+ Req gobernante vigente
→ MediatedEffectCommitment
```

La mera posesión de un `Permit` no constituye compromiso ni ejecución.

## 2. Separación de objetos

Se mantienen separados:

```text
CheckResult
≠ Permit
≠ MediatedEffectCommitment
≠ efecto ejecutado
```

Ninguno de estos objetos pertenece por conversión a `Tri`.

## 3. Consumo del permiso

La operación de mediación consume el `Permit` por valor.

El permiso no implementa `Clone` ni `Copy`. El compromiso mediado tampoco.

Por tanto, una misma instancia de permiso no puede utilizarse mediante la API ordinaria para formar dos compromisos mediados.

Esta propiedad no interpreta todavía contratos de acumulación como `SingleUse`; sólo impide la reutilización estructural de la misma instancia sellada durante la mediación.

## 4. Identidad del efecto

La mediación exige igualdad del `EffectDescriptor` completo sellado en el permiso y presentado al acto de mediación.

La igualdad comprende:

```text
EffectRef
+ EffectFamilyRef
+ GovernedObjectRef
+ ContextRef
```

No basta igualdad de familia, contexto o referencia nominal aislada.

## 5. Revalidación de la forma

La forma vigente se recupera de `AuthorityContinuity` mediante la referencia sellada en el permiso.

Deben conservarse las dimensiones materiales de la forma:

```text
FormRef
+ TransitionClass
+ EffectFamilyRef
+ conjunto de contextos constituidos
+ contexto seleccionado para el acto
+ autoridad requerida
+ AccumulationContract
```

Si la forma no existe o alguna de estas dimensiones ha cambiado, no se forma compromiso mediado.

## 6. Revalidación de la autoridad

La autoridad vigente se recupera de la continuidad usando exclusivamente la autoridad requerida por la forma sellada.

Deben conservarse:

```text
AuthorityRef
+ AuthorityHolderRef
+ contexto de autoridad
```

Además, el efecto concreto debe seguir perteneciendo al alcance actual de la autoridad:

```text
effect ∈ E_max(a | C)
objeto(effect) ∈ D_a
```

La mediación no recibe booleanos de pertenencia suministrados por el llamador.

## 7. Instantánea gobernante de Req

La unidad 1 conservaba la ligadura de `Req` y el resultado técnico `D-A`. La unidad 2 refuerza el sello del permiso con una instantánea material del contenido gobernante de cada obligación.

Para cada `RequirementDescriptor` se conservan, como mínimo:

```text
RequirementRef
+ RequirementClass
+ familias admisibles de verificadores
+ ApplicabilityRuleRef
+ regla de conflicto y su contenido, si existe
+ regla de cobertura y su contenido, si existe
+ regla de reutilización y sus ligaduras, si existe
```

La mediación vuelve a obtener el `RequirementSet` vigente y exige igualdad exacta de esa instantánea.

Por tanto, conservar la misma referencia nominal de una regla cambiando su contenido no mantiene vigente el permiso.

## 8. Resultado técnico

Todo `Permit` productivo procede de un resultado final `D-A` de R1-3.

La mediación conserva defensivamente la comprobación:

```text
technical_result = D-A
```

Un estado distinto no puede formar compromiso mediado.

La mediación no recalcula ni promociona `D-R` o `D-N`, porque esos resultados no producen `Permit` en la unidad 1.

## 9. Salida positiva

La única salida positiva es:

```text
MediatedEffectCommitment
```

El objeto:

- tiene campos privados;
- no tiene constructor público;
- no implementa `Clone` ni `Copy`;
- conserva internamente el permiso consumido;
- mantiene la identidad concreta del efecto y las ligaduras asociadas.

## 10. Fallo cerrado

La mediación falla de forma cerrada cuando, entre otros casos:

- el efecto presentado no coincide con el autorizado;
- la forma ya no existe;
- la forma ha cambiado materialmente;
- la autoridad ya no existe;
- titular o contexto de autoridad han cambiado;
- el efecto ya no pertenece a `E_max` o su objeto a `D_a`;
- el `RequirementSet` ya no existe;
- el contenido gobernante de `Req` ha cambiado.

Estos fallos:

```text
↛ Tri.U
↛ D-A
↛ nueva autoridad
↛ efecto ejecutado
```

## 11. No ejecución en esta unidad

`MediatedEffectCommitment` no ejecuta el efecto.

La unidad no introduce una función de adaptador, llamada externa, red, sistema de archivos, proceso, dispositivo ni otra operación material de efecto.

La siguiente frontera deberá cumplir:

```text
efecto protegido ejecutado
⇒ consumo de MediatedEffectCommitment
```

No será admisible una vía productiva que acepte directamente `Permit`, `EffectRef`, `EffectDescriptor` o un booleano de autorización para comprometer el efecto externo.

## 12. Clases de transición

Esta unidad no hace productivas por sí sola T-G, T-C ni T-R.

T-I, T-V, T-H y T-E continúan sin constituir autoridad.

La existencia de un compromiso mediado tampoco equivale a haber ejecutado una transición de dominio.

## 13. Tiempo y estado técnico

La mediación no introduce:

- reloj ambiental;
- marcas temporales como fuente de vigencia;
- orden cronológico implícito;
- caducidad por paso del tiempo;
- `async` como requisito semántico.

La vigencia se decide por igualdad de ligaduras constituidas.

## 14. Fuera de alcance

Quedan fuera de esta unidad:

- ejecución adaptadora del efecto;
- materialización de `ExerciseRef` como hecho ejecutado;
- interpretación productiva de contratos de acumulación;
- persistencia durable;
- R2, R3 y R4;
- `BudgetΣ`;
- IA-SEC;
- Garantía I;
- Garantía II.

## 15. Criterio de cierre

La unidad será cerrable cuando se demuestre que:

1. sólo un `Permit` legítimo puede entrar en la mediación;
2. la mediación consume ese permiso;
3. el efecto presentado coincide exactamente con el autorizado;
4. forma, autoridad, alcance y `Req` se revalidan contra el estado vigente;
5. cambios materiales conservando referencias nominales impiden el compromiso;
6. el compromiso mediado no es fabricable;
7. no existe ejecución externa en esta unidad;
8. T-G, T-C y T-R permanecen no productivas;
9. R0 y R1-3/R1-4 unidad 1 no sufren regresión.

Hasta que estas condiciones no estén acreditadas, R1-4 permanece abierto.
