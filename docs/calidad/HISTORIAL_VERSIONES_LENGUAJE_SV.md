# Historial de versiones del Lenguaje SV

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Última actualización:** 4 de septiembre de 2026

## 1. Objeto

Este registro ofrece una lectura cronológica y verificable de la evolución técnica del Lenguaje SV. Permite identificar qué versiones fueron incorporadas al repositorio, qué realización o entorno público las materializó, qué modificación material introdujeron y qué evidencia permite comprobarlo.

El registro complementa, pero no sustituye, el historial de Git, las especificaciones, las pruebas de conformidad ni los registros de Calidad detallados.

## 2. Criterios de registro

Se distinguen las situaciones **vigente**, **histórica**, **en desarrollo** y **desplegada no integrada**. La situación de una realización, la integración de su código, el despliegue y el estado de una fase son hechos distintos y se registran de forma separada cuando no coinciden temporalmente.

Las correcciones que no justifican un cambio de número de versión se registran separadamente. El cierre o reapertura de una fase tampoco fuerza por sí mismo un cambio de versión de Gramática, IR o serializador.

## 3. Versiones integradas e hitos de realización

| Fecha | Situación actual | Gramática | IR | Serializador | Realización o entorno | Modificación material | Evidencia pública |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 12/03/2026 | Histórica | — | 0.2 | — | Especificación | Primera IR canónica v0.2 y sistema de bienformación. | [Confirmación 233b8ab](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/233b8ab71e542864fd87ab7580e581ca6a7e34b6) · [IR v0.2](../../IR_CANONICA_BIENFORMACION_SV_v0_2.md) |
| 12/03/2026 | Histórica | 0.1 | 0.2 | 0.1.0 | Especificación e implementación de referencia | Primera gramática superficial mínima v0.1, subordinada a la IR v0.2. | [Confirmación 3503ab6](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/3503ab6074217c1c411a106382c29c9392b313cc) · [Gramática v0.1](../../GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_1.md) |
| 12/03/2026 | Histórica | 0.1 | 0.2 | 0.1.0 | Implementación de referencia en Python | Incorporación del analizador sintáctico, validación, descenso a IR y JSON canónico; batería inicial de 10 casos. | [Confirmación 9ae1b40](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/9ae1b40ac917ea21533cbdb5c74c645027efcc96) |
| 12/03/2026 | Histórica | 0.1 | 0.2 | 0.1.0 | SVP Playground | Primera publicación del entorno de comprobación en navegador. | [Confirmación 011902b](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/011902b4c49e2d9b34cdda9ea4df31b8438bf37e) |
| 23/08/2026 | Vigente como referencia diferencial | 0.2 | 0.3 | 0.1.0 | Etapa frontal de referencia en Python | Integración de C01–C03: separación entre admisibilidad técnica y `Tri.U`; resolución identificada de una `U` constituida; coherencia de `Frame`. La batería del corte quedó en 72/72. | [PR #7](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/7) · [Integración 59a022a](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/59a022a7691d28c6afcf18456ad1ef0aae562362) · [Gramática 0.2](../../GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_2.md) · [IR 0.3](../../IR_CANONICA_BIENFORMACION_SV_v0_3.md) |
| 23/08/2026 | Histórica como acceso público | 0.2 | 0.3 | 0.1.0 | SVP Playground mediante Python/Pyodide | Alineación del entorno público con Gramática 0.2 e IR 0.3. El 24/08 dejó de ser el acceso público principal; Python permanece como referencia diferencial. | [PR #8](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/8) · [Instantánea histórica](../historico/PLAYGROUND_PYTHON_PYODIDE_2026_08_24.md) |
| 24/08/2026 | Histórica como realización publicada | 0.2 | 0.3 | Proyección diferencial 0.1.0 | Rust nativo y WebAssembly de navegador sobre `sv_core` | Primera integración Rust/WebAssembly con paridad sobre el corpus comprometido y una única implementación del núcleo compartida por ambos destinos. | [PR #22](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/22) · [Integración befc666](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/befc666fabe54ecd541416610bf31ddfe776aa69) |
| 24/08/2026 | Hito histórico de cierre | 0.2 | 0.3 | 0.1.0 de referencia / proyección Rust 0.1.0 | Primera realización soberana del núcleo semántico | Cierre integral de R0 en el alcance entonces acreditado. Un defecto heredado de conformidad gramatical sería descubierto posteriormente durante B2 y tratado mediante reapertura correctiva acotada. | [Acta de cierre R0](./ACTA_TECNICA_DE_CIERRE_INTEGRAL_R0_PRIMERA_REALIZACION_SOBERANA_SV_2026_08_24.md) |
| 25/08/2026 | R1 cerrado | 0.2 | 0.3 | Proyección Rust 0.1.0 | `sv_core` · autoridad y mediación intra-proceso | Cierre técnico de R1: autoridad, génesis, requisitos, aplicabilidad, cobertura, conflicto, reutilización, permiso, mediación, decisiones, efectos protegidos y trazas intra-proceso. | [Acta de cierre R1](../arquitectura/ACTA_TECNICA_CIERRE_R1_2026_08_25.md) |
| 25/08/2026 | R2 abierto | 0.2 | 0.3 | Proyección Rust 0.1.0 | Fase de persistencia y continuidad material | Apertura de R2 sobre R0 y R1 cerrados. La fase no modifica por sí sola Gramática, IR ni semántica. | [Acta de apertura R2](../arquitectura/ACTA_TECNICA_APERTURA_R2_PERSISTENCIA_Y_CONTINUIDAD_MATERIAL_2026_08_25.md) |
| 29/08/2026 | **Vigente** | **0.2 canónica** | **0.3** | **Proyección Rust 0.1.0** | **Realización estable bilingüe SVP-ES / SVP-EN · Rust/WebAssembly** | Integración estable de B2; constitución normativa de perfiles fuente; cierre de DG-01/02/03 sobre identidad canónica; regresiones permanentes ES/EN; reconciliación EBNF y vector histórico; re-cierre del perímetro R0 afectado, revalidación de R1 y levantamiento de la suspensión específica de R2 causada por DFL-007. | [PR #55](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/55) · [Perfiles fuente](../../ESPECIFICACION_NORMATIVA_PERFILES_FUENTE_SVP_ES_EN_v1_2026_08_29.md) · [Acta de conformidad](./ACTA_TECNICA_DE_CONFORMIDAD_CIERRE_CORRECTIVO_B2_Y_RESTAURACION_CONTINUIDAD_2026_08_29.md) · [Entorno público](https://lenguaje-sv.itvia.online/) |

El guion largo (`—`) indica que el elemento correspondiente no disponía en esa entrada de un número de versión independiente que este registro pueda acreditar con la misma precisión.

## 4. Diferencias materiales entre Gramática 0.1 / IR 0.2 y Gramática 0.2 / IR 0.3

| Aspecto | Gramática 0.1 / IR 0.2 | Gramática 0.2 / IR 0.3 |
|---|---|---|
| `AdmissibilitySpec` | Admitía en superficie `{Ok, Degraded, Failed, U}`. | Usa exactamente `{Ok, Degraded, NotAdmitted}` y separa la admisibilidad técnica de `Tri.U`. |
| `resolve` | Operaba sobre el literal abstracto `U`. | Usa `resolve((estado, posición), …)` sobre una ocurrencia constituida e identificable de `U`. |
| `Frame` | No incorporaba las restricciones relacionales introducidas por C03. | Exige coherencia estructural y causal dentro del mismo `Frame`, sin imponer exhaustividad. |
| Perfiles fuente | No constituidos. | `SVP-ES` y `SVP-EN` convergen sobre una única Gramática canónica 0.2 y una única IR 0.3. |
| Serializador | `0.1.0`. | `0.1.0`; la revisión gramatical no modifica el serializador canónico de referencia. |
| Conformidad | Baterías anteriores, ampliadas progresivamente. | El corpus vigente R0-7 contiene 79/79 casos: 12 válidos y 67 inválidos. |

No cambian `Tri = {Zero, One, U}`, la ausencia de tiempo, reloj o UTC como primitivas universales del Lenguaje ni las deudas que permanecen expresamente abiertas, incluida `ConflictOperator`/J2.3 para régimen `General`.

## 5. Correcciones relevantes sin cambio de versión

| Fecha | Versión afectada | Corrección | Consecuencia | Evidencia pública |
| --- | --- | --- | --- | --- |
| 12/03/2026 | Gramática 0.1 / IR 0.2 | Correcciones sucesivas del Playground hasta disponer de una API estable, errores tipados y ejecución operativa. | El entorno público quedó utilizable sin alterar los números de Gramática e IR. | [Confirmación 9136270](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/9136270a082b7fb42496c8c0f91f8d74453e751a) |
| 19/08/2026 | Gramática 0.1 / IR 0.2 | Retirada de `conflicts` de `graph_decl`; el régimen `General` conservó la deuda de `ConflictOperator`. | Se eliminó una capacidad superficial que no disponía de realización semántica suficiente. | [Confirmación 058befb](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/058befbd6402c80ac7bc1d10eab0d8d035126531) |
| 23/08/2026 | Playground 0.2 / 0.3 | Corrección de rótulos antiguos que no coincidían con la implementación cargada. | La interfaz pasó a mostrar las versiones vigentes. | [PR #8](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/8) |
| 24/08/2026 | Frontera Normativa v0 respecto de Gramática 0.2 / IR 0.3 | Reconciliación de las cláusulas superadas por C01–C03. | La Frontera Normativa v0 se conserva como antecedente y la adenda fija la precedencia vigente. | [Adenda técnica de vigencia](../../ADENDA_TECNICA_VIGENCIA_FRONTERA_NORMATIVA_C01_C03_2026_08_24.md) |
| 29/08/2026 | Gramática canónica 0.2 / IR 0.3 / realización Rust | Cierre de los dominios `SemanticRelation.kind`, `Pattern.kind` y `Graph.regime`; constitución de perfiles fuente; reconciliación de cierres EBNF internos y reclasificación del vector adversarial histórico. | Se cierra DFL-007 sin crear una segunda gramática ni modificar J2.3-General. Las regresiones quedan permanentes para ES y EN. | [PR #55](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/55) · [Acta de conformidad](./ACTA_TECNICA_DE_CONFORMIDAD_CIERRE_CORRECTIVO_B2_Y_RESTAURACION_CONTINUIDAD_2026_08_29.md) |
| 29/08/2026 | Entorno estable B2 | El Historial Beta español e inglés pasa a servirse como página local efectiva; se elimina la sobrescritura dinámica hacia una representación `blob`. | El entorno desplegado representa ambos historiales como HTML y no como código fuente. | [Entorno público](https://lenguaje-sv.itvia.online/) · [PR #55](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/55) |
| 04/09/2026 | IR 0.3 / catálogo efectivo / realizaciones Python y Rust | N0-01 fija `Codomain` como conjunto representado sin miembros repetidos y `E004 — InvalidCodomain` como identidad diagnóstica vigente. | El rechazo es común a ambos perfiles fuente y no amplía `OutputSemantics`, JSON, dominios ni ensamblaje. | [Acta N0-01](../arquitectura/ACTA_TECNICA_N0_01_UNICIDAD_DE_CODOMAIN_2026_09_04.md) |

## 6. Realización Rust/WebAssembly y entorno público vigente

La realización Rust mantiene una sola implementación del núcleo en `sv_core`, compartida por el destino nativo y WebAssembly. Python conserva el papel de referencia diferencial.

Los perfiles fuente se resuelven antes del análisis sintáctico y convergen sobre una misma identidad canónica:

```text
SVP-ES ─┐
        ├→ identidad canónica → Gramática 0.2 → IR 0.3 → semántica única
SVP-EN ─┘
```

Identidad del WebAssembly vigente:

```text
main de realización = c1acf943a7a44ce81080881e59283de8a2019606

sv_wasm.wasm
bytes   = 378956
SHA-256 = 95c7d1e0313567ef099c6e426a7fcee8ff4a5ac8adb670265f859f1bf03caab3
```

Paquete de despliegue inicialmente acreditado el 29 de agosto de 2026:

```text
SV_LENGUAJE_PRODUCCION_B2_CLOUDFLARE_2026-08-29_FINAL_CONFORMIDAD.zip
bytes   = 167503
SHA-256 = 566200f97bfea86a0b7ce7c4919bac9d5367a67b8cba719eef1c573942d696f5
```

Paquete reconciliado cargado el 30 de agosto de 2026 para el cierre del Playground:

```text
SV_LENGUAJE_PRODUCCION_B2_CLOUDFLARE_2026-08-30_FINAL_RECONCILIADO.zip
archivos = 39
bytes    = 168612
SHA-256  = 11e53a6c9b836006d0f01eb8af69b3bfbedae29524078a40966fe87acf5c19db
```

El segundo paquete no invalida la identidad histórica del primero: documenta la reconciliación final del mismo cierre B2 y constituye el artefacto de despliegue vigente.

El entorno público se sirve en `lenguaje-sv.itvia.online`. La distribución web entrega los activos estáticos; la compilación del texto `.svp` se ejecuta localmente en el navegador mediante `sv_wasm` y `sv_core`.

## 7. Evidencia vigente de conformidad

La base correctiva obtuvo:

```text
R0-7                        = 79/79
  válidos                   = 12/12
  inválidos                 = 67/67
sv_core                     = 210/210
dominios cerrados Rust      = 5/5
sondas DG navegador ES+EN   = 6/6
sv_wasm                     = 2/2
doc-tests sv_core           = 17/17
```

La evidencia acredita el corte y los casos declarados, no propiedades universales. En particular, no acredita compatibilidad con todos los motores de navegador, serialización canónica Rust completa, paridad diagnóstica textual integral ni las Garantías I o II.

## 8. Continuidad de fases

El cierre correctivo de DFL-007 deja:

```text
R0 = CERRADO, incluido el perímetro correctivo
R1 = CERRADO y revalidado
R2 = ABIERTO; suspensión específica DFL-007 levantada
R3 = NO INICIADO
R4 = NO INICIADO

Garantía I  = NO_PROBADO
Garantía II = NO_PROBADO
```

El estado `R2 = ABIERTO` procede de su acta de apertura de 25/08/2026. El levantamiento de una suspensión correctiva no constituye una nueva apertura ni un cierre de R2.

## 9. Relación con Calidad y trazabilidad

La entrada vigente debe leerse junto con el registro de deuda viva, el registro de evolución técnica, la especificación normativa de perfiles fuente y el acta de conformidad de 29/08/2026.

La verificación externa independiente del corte final se documentará separadamente. Hasta que exista esa segunda comprobación, este historial no la presume.

## 10. Regla de continuidad

Toda nueva versión de Gramática, IR, serialización, realización o entorno público que modifique de forma observable la referencia vigente deberá añadir una entrada a este historial.

Las versiones anteriores permanecerán accesibles como antecedentes. Los errores y correcciones relevantes deberán registrarse cuando sean necesarios para comprender la evolución técnica, sin ocultarlos ni convertirlos por sí solos en una nueva versión.
