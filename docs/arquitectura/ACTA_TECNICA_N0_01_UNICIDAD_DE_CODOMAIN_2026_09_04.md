# Acta técnica N0-01 — unicidad de `Codomain`

**Fecha:** 4 de septiembre de 2026  
**Rama:** `cierre-nuclear-20260904`  
**Acto precedente:** `N0_RADIOGRAFIA_DE_OBJETOS_INVARIANTES_Y_ORACULOS_DEL_NUCLEO_SV_2026_09_04.md`  
**Objeto:** cerrar la invalidez intrínseca de un `Codomain` con miembros repetidos, sin incorporar todavía contratos de perfil, dominio, salida, JSON o ensamblaje.

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
- suficiencia de `Domain`, cobertura de `Agent` o perfiles de dominio;
- composición de dominios o superagentes;
- modificación de los perfiles fuente SVP-ES y SVP-EN;
- actualización de toolchain, MSRV o dependencias Rust.

## 6. Criterio de cierre y continuidad

N0-01 queda técnicamente cerrado sólo si el corpus Python es verde, las pruebas Rust de ambos perfiles y ensamblaje son verdes, los destinos nativo y WebAssembly conservan paridad y el diff no contiene cambios fuera del radio declarado.

El siguiente acto nuclear admisible es N0-02: relación total y sin claves repetidas entre `CellSpec`, `OutputSemantics` y `Codomain`. No se anticipa en este acta.

## 7. Dictamen de alcance

```text
CODOMAIN_COMO_CONJUNTO                = CERRADO_EN_N0_01_SI_CI_VERDE
DIAGNOSTICO_E004                      = IDENTIDAD_VIGENTE
RENORMALIZACION_SILENCIOSA            = PROHIBIDA
PERFILES_DE_DOMINIO                   = NO_AFECTADOS
OUTPUT_SEMANTICS                      = DIFERIDO_A_N0_02
ENSAMBLAJE_SEMANTICO                  = NO_AMPLIADO
```
