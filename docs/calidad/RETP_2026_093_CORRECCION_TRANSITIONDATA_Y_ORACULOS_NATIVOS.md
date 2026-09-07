# Registro de calidad RETP-2026-093 — corrección local de `TransitionData` y ampliación de cobertura

**Fecha:** 7 de septiembre de 2026  
**Base material:** `bc3b22c9e9319e8f191390c8cfe9fa1577904d87`  
**Recepción G/H de base:** `9b32edda21072b3d2a138d9e92d95febdbc21ac4`  
**Corte material previamente verificado:** `9a79e1370afa3f22ab85f94b455bfc5aa1ac0d82`  
**Solicitud de incorporación:** `#78`  
**Estado:** `CANDIDATA_EN_REVERIFICACION`

## 1. Controles realizados

La realización impone antes de exponer la representación intermedia:

- resolución de cada `NodeId` como `CoupledSpec`;
- pertenencia del nodo al `CompositionGraph` del horizonte;
- posición dentro de `[1,n]`, sin estrechamiento de `Nat`;
- unicidad del destino `(node_ref, position)`;
- unicidad local del tipo de suceso dentro de cada `TransitionData`.

Dos nodos diferentes pueden compartir `CellSpec` y conservar destinos distintos. También permanecen protegidas las guardas tipadas de `Frame.architecture` y `compose(graph)`, la geometría `b/n`, la longitud del vector y los metadatos de proyección.

## 2. Insuficiencia de cobertura detectada

El corte `9a79e137…` acreditó los nueve casos nuevos mediante una batería causal nativa separada. Una mutación de la comprobación de pertenencia arquitectónica hizo fallar las pruebas Rust, pero no el corpus compartido, porque éste aún no contenía esos testigos.

La candidata migra los nueve casos a `tests/conformance/invalid/` y hace que el comprobador causal consuma esos mismos archivos. De este modo:

- se conserva la comprobación exacta de la causa;
- se ejercitan las reglas en nativo, WASI y navegador;
- se evita duplicar las fuentes de prueba;
- el corpus esperado pasa de `14 + 86` a `14 + 95`.

Los resultados anteriores se conservan como evidencia del corte material previo. La ampliación requiere una ejecución nueva sobre la cabeza exacta.

## 3. Deuda expresamente registrada

- **DFL-011:** la clave histórica `cell_ref` transporta una identidad `NodeId` resuelta como `CoupledSpec`; la migración de nombre queda reservada a una versión incompatible posterior del esquema.
- **DFL-012:** nativo, WASI y navegador comparten `sv_core`; su paridad no constituye independencia entre realizaciones semánticas.

## 4. Evidencia antecedente

| Control | Resultado sobre `9a79e137…` |
|---|---|
| `sv_core` | `211/211` |
| Pruebas correctivas nuevas | `16/16` |
| Corpus compartido anterior | `14/14` válidos y `86/86` inválidos |
| Casos causales nativos | `9/9` |
| Mutaciones dirigidas | `12/12` detectadas; `0` supervivientes; `0` inválidas |
| Rust 1.98.0 y compatibilidad 1.98.1 | conforme |
| R0 Rust `34161898907` | conforme |
| Conformidad SVP `34161898934` | conforme |
| R0-8 `34161898968` | conforme |
| R0 WASM y navegador `34161898894` | conforme |

## 5. Condición de cierre

La promoción exige, sobre la nueva cabeza exacta:

```text
CORPUS_COMPARTIDO = 14_VALIDOS + 95_INVALIDOS
CONFORMIDAD_CAUSAL = 9/9
MUTACIONES_DIRIGIDAS = 12/12_DETECTADAS
NATIVO_WASI_NAVEGADOR = CONFORMES
DFL_011 = ABIERTA_Y_GOBERNADA
DFL_012 = ABIERTA_Y_GOBERNADA
```

Hasta obtener esa evidencia, el cierre local permanece en reverificación. La fila 7 continúa abierta; DFL-005, H06, H07, la causalidad entre datos de transición y marcos, y la independencia semántica siguen sin cerrar.

## 6. Documentos y pruebas vinculados

- `docs/arquitectura/ACTA_TECNICA_DE_CORRECCION_LOCAL_DE_TRANSITIONDATA_Y_REFUERZO_DE_ORACULOS_NATIVOS_2026_09_07.md`;
- `docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`;
- `docs/calidad/RETP_2026_093_CORRECCION_TRANSITIONDATA_Y_ORACULOS_NATIVOS.csv`;
- `tests/run_row7_transitiondata_conformance.py`;
- `tests/run_directed_mutation_sensitivity.py`;
- `tests/conformance/invalid/`.
