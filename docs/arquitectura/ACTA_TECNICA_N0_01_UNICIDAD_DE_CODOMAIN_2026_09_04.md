# Acta técnica N0-01 — unicidad de `Codomain`

**Fecha:** 4 de septiembre de 2026  
**Rama:** `cierre-nuclear-20260904`  
**Acto precedente:** `N0_RADIOGRAFIA_DE_OBJETOS_INVARIANTES_Y_ORACULOS_DEL_NUCLEO_SV_2026_09_04.md`  
**Objeto:** cerrar la invalidez intrínseca de un `Codomain` con miembros repetidos, sin incorporar todavía contratos de perfil, dominio, salida, JSON o ensamblaje.

**Revalidación posterior:** compatible con `main@230a205b08f4c54c9c8d9c1c7ad35b2f6ddbbfc4`, `AGENTS.md`, `RETP-2026-072`, `RETP-2026-073` y los Pilares y restricciones de diseño del Lenguaje SV.

**Cierre probatorio posterior:** la cabeza material `49fa4e2c9afee9a7bcb3ed7a792f3490477c69f6` superó Conformidad SVP #276 (`33967623538`), R0 Rust #205 (`33967623535`), R0-8 Baseline nativa #156 (`33967623518`) y R0 WASM paridad de tres vías #151 (`33967623548`).

N0-01 actúa exclusivamente sobre un invariante intrínseco ya representable de `Codomain`. No decide células, `b`, parámetros, dominio, agente, bus, host ni operación algebraica, y no atribuye a `sv_core` ejecución soberana distinta de la validación aquí probada.

## 1. Decisión

`Codomain` representa un conjunto finito y explícito mediante una secuencia que conserva el orden declarado. Su bienformación exige:

```text
values ≠ []
card(values) = card(set(values))
```

El orden de representación no constituye un orden total semántico. N0-01 tampoco resuelve `E111 — UnorderedCodomain` ni autoriza `max` o `min`.

## 2. Conducta obligatoria

Ante un miembro repetido:

1. el programa se rechaza antes de exponer una IR aceptada;
2. no se deduplica, ordena ni repara la declaración;
3. Python y Rust hacen observable `E004 — InvalidCodomain`;
4. el ensamblaje no puede rescatar una unidad inválida;
5. los codominios no vacíos con miembros distintos permanecen admitidos en SVP-EN y SVP-ES.

## 3. Identidad diagnóstica

La tabla histórica de IR v0.2 asignó `E101 — EmptyCodomain`. La realización efectiva ya utiliza `E101 — VectorLengthMismatch`; reutilizar ese código produciría una colisión observable.

N0-01 fija por ello:

```text
E004 = InvalidCodomain
```

Su alcance es `Codomain` vacío o con miembros repetidos. La decisión no reescribe el documento histórico v0.2 y reduce DFL-001 sólo en este objeto.

## 4. Realización y oráculos

| Plano | Evidencia exigida |
|---|---|
| Contrato | J-K0 y ecuación de unicidad en IR v0.3 |
| Python | validación previa al lowering observable y diagnóstico `E004` |
| Rust | validación soberana previa a devolver `IrProgram` y diagnóstico `E004 (InvalidCodomain)` |
| Conformidad | `codomain_miembro_duplicado.svp` rechazado con `E004` |
| Recíproco | miembros distintos aceptados en SVP-EN y SVP-ES |
| Ensamblaje | una unidad con duplicados invalida el ensamblaje completo |
| No regresión | los doce JSON válidos permanecen byte-idénticos |

El corpus de conformidad pasa de 79 a 80 casos: 12 válidos y 68 inválidos.

## 5. Exclusiones constitutivas

N0-01 no decide:

- totalidad o unicidad de `OutputSemantics`;
- orden total semántico de un codominio;
- colisiones de claves o forma canónica del JSON;
- integridad referencial de `Horizon.architecture`;
- cobertura, disjunción o ausencia de solapamiento de las particiones de `Ternarizer`, incluida la repetición nominal de `partition_zero`, `partition_one` y `partition_u`;
- unicidad de las posiciones de `CoupledSpec.bridges` ni el cierre representacional de `BridgeSet`;
- estatuto de multiplicidad o unicidad de `Horizon.events`;
- suficiencia de `Domain`, cobertura de `Agent` o perfiles de dominio;
- composición de dominios o superagentes;
- modificación de los perfiles fuente SVP-ES y SVP-EN;
- actualización de toolchain, MSRV o dependencias Rust.

Estas exclusiones no declaran admisibles los tres estados anteriores. Impiden presentar el cierre de `Codomain` como cierre general de toda secuencia con semántica de conjunto. Su examen exige actos separados y no puede resolverse por analogía dentro de N0-01.

## 6. Criterio de cierre y continuidad

N0-01 queda técnicamente cerrado sólo si el corpus Python es verde, las pruebas Rust de ambos perfiles y ensamblaje son verdes, los destinos nativo y WebAssembly conservan paridad y el diff no contiene cambios fuera del radio declarado.

El siguiente acto nuclear admisible es N0-02: relación total y sin claves repetidas entre `CellSpec`, `OutputSemantics` y `Codomain`. No se anticipa en este acta.

## 7. Dictamen de alcance

```text
CODOMAIN_COMO_CONJUNTO                = CERRADO_EN_N0_01
DIAGNOSTICO_E004                      = IDENTIDAD_VIGENTE
RENORMALIZACION_SILENCIOSA            = PROHIBIDA
PERFILES_DE_DOMINIO                   = NO_AFECTADOS
OUTPUT_SEMANTICS                      = DIFERIDO_A_N0_02
ENSAMBLAJE_SEMANTICO                  = NO_AMPLIADO
```

<a id="recepcion-20260906"></a>

## 8. Recepción sobre main actualizado · 06/09/2026

**Asiento de recepción:** RETP-2026-077; RETP-074 conserva íntegro su cierre histórico. La cabeza de entrada `ad8e8dd30930e35b75bf5f2fad78938d36233b78` se reconcilia con main `981159d6428197d1ad1d748649f1d8f690b2f588`, que incorpora perfiles RETP-075 y secuencia RETP-076. Se conservan ambos asientos, las piezas rectoras incorporadas a main y el código, pruebas y workflows de N0-01. Los cambios adicionales son de recepción y continuidad documental.

La comprobación local del árbol reconciliado, con Rust/cargo 1.98.0, acredita: conformidad Python **80/80** (12 válidos y 68 inválidos), CLI **3/3**, caracterización E006 **4/4**; Rust **210** pruebas internas de `sv_core`, **3** específicas de unicidad, **5** de dominios gramaticales cerrados, **2** del adaptador WASM y **17** pruebas documentales; equivalencia R0-7 **12/12 + 68/68** y comprobación de compilación de `sv_core`/`sv_wasm` para `wasm32-unknown-unknown`. La ejecución WASI y navegador se exige en CI sobre la cabeza reconciliada; la compilación local no la sustituye.

La conservación de los doce oráculos comprometidos se comprueba por identidad de sus blobs, sin regenerarlos. Además, se comparan directamente los bytes de las doce salidas de cada emisor contra ese mismo emisor en main de entrada. No se afirma igualdad literal entre emisores: sus comprobadores generales normalizan JSON, limitación que corresponde reparar en la fila 2. Como control causal del cambio, el caso duplicado es aceptado por ambas realizaciones en main de entrada y rechazado en la candidata con `rc=1`, `E004/InvalidCodomain` y stdout vacío. Los controles positivos, ES/EN y ensamblaje permanecen en la suite específica.

La promoción exige examinar de nuevo **Conformidad SVP, R0 Rust, R0-8 Baseline nativa y R0 WASM paridad de tres vías** del candidato exacto. Sus identidades y resultados, así como el commit de integración, quedan vinculados en [PR #61](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/61) y sus [controles](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/61/checks). Los verdes históricos de los apartados iniciales no acreditan esta reconciliación. La diferencia entre `stable` flotante en R0 Rust y 1.98.0 en los otros flujos permanece como deuda de infraestructura de N0 §8; no se cambia la toolchain junto con este cierre semántico.

**Continuidad vigente:** [acta de transición, §18](../dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md#recepcion-n0-01-20260906). Tras la integración, corresponde **reparar los oráculos y después continuar K1 por N0-02**, conforme a RETP-076. Esta precisión actualiza el relevo de §6 sin borrar su antecedente y conserva todas las exclusiones de §5.
