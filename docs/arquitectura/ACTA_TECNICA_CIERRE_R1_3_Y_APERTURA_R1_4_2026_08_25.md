# Acta técnica de cierre de R1-3 y apertura de R1-4

**Fecha:** 25 de agosto de 2026  
**Ámbito:** `sv_core`  
**Fase:** R1 — autoridad, mediación y decisiones protegidas  
**Corte cerrado:** R1-3  
**Corte abierto:** R1-4

## 1. Objeto

Este documento constata el cierre de R1-3, dedicado a requisitos, aplicabilidad y resultados técnicos de comprobación, y abre R1-4, dedicado a la decisión de permiso y a la mediación de efectos protegidos.

El cierre no modifica la semántica ternaria del Lenguaje SV, la gramática ni la representación intermedia canónica. Tampoco acredita las Garantías I o II.

## 2. Identidad del corte integrado

R1-3 queda consolidado sobre:

```text
main = d15d93c96095663bcada5c9e567cde7c59345f20
```

Ese corte incorpora las unidades materiales de constitución inicial de requisitos y aplicabilidad, conflicto sin regla, resolución mediante regla constituida, sellado de resultados por obligación, cobertura constituida y reutilización ligada de resultados cualificados.

La batería integrada mantiene, entre otras comprobaciones:

```text
sv_core   = 172/172
sv_wasm   = 2/2
doc-tests = 8/8
R0-7      = 11/11 casos válidos equivalentes
            61/61 casos inválidos rechazados
```

Las líneas de conformidad, núcleo nativo/WebAssembly, línea base nativa y paridad de tres vías permanecen correctas en el corte de integración de la última unidad de R1-3.

## 3. Resultado técnico consolidado de R1-3

R1-3 establece una cadena cerrada de control técnico:

```text
forma + efecto/familia + contexto
→ Req constituido
→ Applicable(V,q,C) constituido
→ RequirementCheck sellada
→ resolución intra-obligación
→ ResolvedRequirementResult
→ cobertura constituida
→ resultado técnico cualificado
→ reutilización histórica sólo bajo continuidad exacta de ligaduras
```

Los únicos resultados técnicos siguen siendo:

```text
D-A = ACREDITADO
D-R = REFUTADO
D-N = NO_VERIFICABLE
```

con:

```text
D-N ≠ Tri.U
CheckResult ≠ Authority
CheckResult ≠ Permit
resultado técnico ≠ efecto ejecutado
```

Los errores estructurales de formación, ligadura o agregación pueden cerrarse como errores tipados. No se convierten por ello en `Tri.U`. La no verificabilidad gobernada se representa mediante `D-N` cuando corresponde.

## 4. Comprobación de las condiciones de cierre

| Nº | Condición de cierre de R1-3 | Evidencia material consolidada | Estado |
|---:|---|---|---|
| 1 | Una forma sujeta a control no puede producir efecto con `Req = ∅`. | Constitución inicial exige conjunto de requisitos para toda ligadura controlada; `RequirementSet` rechaza el vacío. | CUMPLIDA |
| 2 | El beneficiario o ejecutor no puede eliminar obligaciones aplicables durante el acto. | `RequirementSet` y `RequirementDescriptor` tienen constitución cerrada; el acto de comprobación sólo consume el estado ya constituido. | CUMPLIDA |
| 3 | El núcleo no eludible no puede omitirse localmente. | T-0 rechaza la ausencia de obligaciones nucleares; la batería conserva `MissingMandatoryCore`. | CUMPLIDA |
| 4 | `D-A`, `D-R` y `D-N` permanecen disjuntos. | `CheckResult` es una enumeración cerrada de tres variantes. | CUMPLIDA |
| 5 | `D-N` no puede convertirse en `Tri.U`. | `CheckResult` y `Tri` son tipos distintos; existen pruebas negativas de compilación y regresión. | CUMPLIDA |
| 6 | Un fallo técnico o estructural no fabrica `Tri.U`. | Los fallos de formación y ligadura son errores tipados; la falta gobernada de base suficiente cierra en `D-N`. | CUMPLIDA |
| 7 | Un verificador no aplicable no puede producir un `D-A` válido. | `RequirementCheck` sólo se forma contra `VerifierApplicability` compatible; T-0 constituye `Applicable(V,q,C)`. | CUMPLIDA |
| 8 | Un verificador no puede autolegitimar su propia aplicabilidad. | La relación `Applicable(V,q,C)` no tiene constructor público y se constituye antes del acto por la puerta T-0. | CUMPLIDA |
| 9 | Un conflicto sin regla constituida produce `D-N`. | La resolución sin regla conserva el fallo cerrado ante resultados incompatibles. | CUMPLIDA |
| 10 | Una refutación se conserva en la agregación. | La precedencia inter-obligación es `D-R > D-N > D-A`. | CUMPLIDA |
| 11 | Sin refutación, la presencia de una no verificabilidad produce `D-N`. | Agregación gobernada determinista y pruebas de regresión. | CUMPLIDA |
| 12 | `D-A` agregado exige acreditación íntegra. | La agregación requiere resultado sellado por cada obligación y sólo acredita si no existe `D-R` ni `D-N`. | CUMPLIDA |
| 13 | Una acreditación parcial no acredita la totalidad sin cobertura constituida. | `CoverageRule` se constituye en T-0; una cobertura ausente o incompleta impide `D-A` final. | CUMPLIDA |
| 14 | Un resultado histórico no se reutiliza fuera de sus ligaduras causales declaradas. | `ReuseRule` se constituye en T-0; el resultado histórico conserva la cualificación de cobertura y compara contenido material de las reglas y ligaduras. | CUMPLIDA |
| 15 | R1-3 no produce `Permit` ni ejecuta efectos. | No existe un tipo productivo de permiso ni una operación de mediación de efectos en R1-3. | CUMPLIDA |
| 16 | T-G, T-C y T-R no producen cambios efectivos antes de R1-4. | Permanecen con disposición no productiva en el corte integrado. | CUMPLIDA |
| 17 | Las regresiones de R0 y R1-0/R1-1/R1-2 permanecen correctas. | Pruebas nativas, WebAssembly, R0-7 y paridad de tres vías continúan correctas. | CUMPLIDA |

No queda una condición de cierre de R1-3 sin representación o sin frontera material identificable.

## 5. Decisión de cierre

Se establece:

```text
R1-3 = CERRADO · INTEGRADO
```

El cierre acredita exclusivamente el estado técnico de control descrito en este documento. No acredita permiso, ejecución, persistencia durable, continuidad entre procesos, recuperación material, seguridad de plataforma ni las Garantías I o II.

Las observaciones de mantenimiento que no abren una vía productiva —por ejemplo, pruebas nominadas adicionales o eliminación futura de código interno sin uso— no modifican este cierre.

## 6. Frontera que se entrega a R1-4

La entrada válida de R1-4 es un estado de control ya gobernado por R1-3. En particular:

```text
Req completo
+ resultado técnico final D-A
```

es una condición necesaria para considerar un permiso positivo, pero no constituye por sí sola el permiso:

```text
D-A final ≠ Permit
```

Del mismo modo:

```text
D-R → no puede producir Permit positivo
D-N → no puede producir Permit positivo
error estructural/técnico → no puede producir Permit positivo
```

R1-4 deberá ligar cualquier decisión positiva a la autoridad, forma, efecto, contexto, dominio y demás condiciones constituidas que resulten causales para el efecto protegido.

## 7. Apertura de R1-4

Cumplidas las condiciones de cierre de R1-3, procede abrir R1-4 con objeto limitado a:

1. representar una decisión de permiso no fabricable desde un resultado nominal;
2. derivar esa decisión exclusivamente de autoridad y ligaduras constituidas junto con el resultado técnico gobernado de R1-3;
3. impedir que `D-R`, `D-N` o un error técnico produzcan permiso positivo;
4. ligar el permiso al efecto protegido concreto y a su contexto material;
5. mediar el compromiso del efecto de modo que ninguna vía protegida pueda eludir la decisión gobernada;
6. preservar la distinción entre permiso concedido y efecto efectivamente ejecutado;
7. mantener `Tri`, gramática e IR fuera de esta decisión de control salvo modificación expresamente justificada por otro corte.

La apertura no hace productiva por declaración ninguna clase T-*. La productividad material sólo podrá introducirse cuando la realización de R1-4 demuestre la mediación necesaria y pase sus pruebas negativas.

## 8. Exclusiones de R1-4 en la apertura

La apertura de R1-4 no materializa todavía:

- persistencia durable;
- continuidad entre procesos;
- recuperación durable;
- presupuesto de recursos `BudgetΣ`;
- perfiles de inteligencia artificial auxiliar;
- canales de red, gestión de secretos o aislamiento de plataforma;
- Garantía I;
- Garantía II;
- R2, R3 o R4.

El agotamiento de recursos, los fallos de canal y las indisponibilidades de plataforma seguirán siendo estados técnicos y no valores de `Tri`.

## 9. Estado resultante

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
