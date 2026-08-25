# R1-3 — Unidad 3A: conflicto sin regla de resolución constituida

**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte:** R1-3  
**Estado:** contrato de realización de la primera subunidad de conflicto

## 1. Objeto

Esta unidad materializa el caso cerrado de SEC.0-D en el que varias comprobaciones selladas de una misma obligación producen resultados incompatibles y no existe una regla de resolución previamente constituida y aplicable.

La consecuencia es obligatoria:

```text
conflicto(q) + ausencia de regla constituida
→ Check(q) = D-N
```

La unidad no introduce todavía reglas productivas de resolución de conflictos. Tampoco modifica la constitución inicial de `Req` o `Applicable`, no produce `Permit`, no ejecuta efectos y no hace productivas T-G, T-C o T-R.

Base de realización:

```text
main = a113f0627b5414aa8dd8dfd78d3fd465180ce024
```

## 2. Entrada admisible

La operación de resolución sólo acepta `RequirementCheck` ya selladas. No acepta pares libres `(verificador, resultado)` ni valores `CheckResult` crudos en su superficie pública.

Por tanto:

```text
CheckResult nominal
≠ comprobación admisible para conflicto
```

La validez material de cada `RequirementCheck` continúa dependiendo de las ligaduras ya fijadas por R1-3:

- obligación;
- forma;
- familia de efectos;
- contexto;
- verificador;
- familia de verificadores;
- regla de aplicabilidad.

## 3. Unidad del conflicto

Todas las comprobaciones suministradas a una misma resolución deben pertenecer a la misma `RequirementRef`.

Si aparecen obligaciones distintas en el mismo conjunto, la entrada es estructuralmente inválida y no se interpreta como conflicto entre evidencias de una obligación.

```text
q1 ≠ q2
→ no forman un conflicto único de q
```

## 4. Repetición del mismo verificador

Una misma identidad de verificador no puede aparecer más de una vez dentro del conjunto que pretende caracterizar el conflicto.

La repetición se rechaza como entrada inválida. No puede utilizarse para:

- aumentar artificialmente el peso de un resultado;
- simular mayoría;
- transformar un reintento en evidencia adicional independiente.

Esta unidad no atribuye independencia material entre verificadores distintos; sólo impide la duplicación nominal de uno mismo dentro del conjunto.

## 5. Regla sin conflicto

Si todas las comprobaciones selladas de la obligación coinciden en el mismo resultado técnico, la resolución conserva exactamente ese resultado:

```text
∀i Check_i(q) = D-A → D-A
∀i Check_i(q) = D-R → D-R
∀i Check_i(q) = D-N → D-N
```

La coincidencia no constituye por sí sola una regla de cobertura, reutilización histórica o autoridad adicional.

## 6. Regla de conflicto sin resolución constituida

Existe conflicto cuando, para una misma obligación, aparecen al menos dos resultados técnicos distintos entre comprobaciones selladas de verificadores nominalmente distintos.

En esta unidad no existe regla de resolución constituida que permita escoger uno de los resultados incompatibles. Por tanto:

```text
resultados incompatibles
→ D-N
```

Queda excluido obtener `D-A` o `D-R` mediante:

- selección favorable;
- orden de llegada;
- orden del vector de entrada;
- mayoría implícita;
- repetición de una comprobación;
- preferencia local por un verificador.

El orden de las entradas no modifica el resultado.

## 7. Separaciones obligatorias

La resolución de esta unidad conserva:

```text
D-N ≠ Tri.U
D-N ≠ D-R
D-N ≠ D-A
conflicto ≠ permiso
conflicto ≠ autoridad
```

Un conflicto no abre por sí mismo T-G, T-C, T-R ni una forma de emergencia.

## 8. Relación con la agregación de obligaciones

Esta unidad actúa antes de la agregación cerrada de la primera unidad de R1-3.

Conceptualmente:

```text
múltiples comprobaciones de q
→ resolución técnica de q
→ un único resultado técnico para q
→ agregación entre obligaciones
```

No se modifica la precedencia de agregación ya fijada entre obligaciones:

```text
algún D-R                     → D-R
ningún D-R + algún D-N        → D-N
todos D-A                     → D-A
```

La regla de esta unidad sólo determina el resultado de una obligación cuando existen varias comprobaciones de esa misma obligación.

## 9. Límites

Quedan fuera de esta subunidad:

- constitución productiva de reglas capaces de resolver un conflicto a `D-A` o `D-R`;
- cobertura parcial;
- reutilización histórica;
- vigencia de resultados almacenados;
- sustitución gobernada de verificadores;
- persistencia durable;
- `Permit`;
- mediación o ejecución de efectos.

La siguiente subunidad de conflicto podrá introducir reglas previamente constituidas únicamente si preserva las ligaduras de obligación, contexto y verificadores aplicables y no convierte una regla local del acto en autoridad de resolución.

## 10. Pruebas mínimas

La realización deberá demostrar, como mínimo:

1. conjunto vacío ⇒ rechazo estructural;
2. obligaciones distintas en una sola resolución ⇒ rechazo;
3. verificador repetido ⇒ rechazo;
4. varios `D-A` homogéneos ⇒ `D-A`;
5. varios `D-R` homogéneos ⇒ `D-R`;
6. varios `D-N` homogéneos ⇒ `D-N`;
7. `D-A` + `D-R` ⇒ `D-N`;
8. `D-A` + `D-N` ⇒ `D-N`;
9. `D-R` + `D-N` ⇒ `D-N`;
10. permutar el orden de un conflicto no altera `D-N`;
11. la agregación de obligaciones de la primera unidad permanece intacta;
12. T-G, T-C y T-R continúan no productivas;
13. no existe `Permit` productivo;
14. las regresiones cerradas de R0, R1-0, R1-1 y R1-2 permanecen correctas.

## 11. Estado

```text
R0 = CERRADO
R1 = ABIERTO
R1-0 = CERRADO
R1-1 = CERRADO
R1-2 = CERRADO · INTEGRADO
R1-3 = ABIERTO

R1-3 / unidad 1 = CERRADA · AUDITADA · INTEGRADA
R1-3 / unidad 2 = CERRADA · AUDITADA · INTEGRADA
R1-3 / unidad 3A = EN REALIZACIÓN

R1-4 = NO INICIADO
R2–R4 = NO INICIADOS
Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```
