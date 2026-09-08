# Registro de calidad RETP-2026-093 — corrección local de `TransitionData` y ampliación de cobertura

**Fecha:** 7 de septiembre de 2026  
**Base material:** `bc3b22c9e9319e8f191390c8cfe9fa1577904d87`  
**Recepción G/H de base:** `9b32edda21072b3d2a138d9e92d95febdbc21ac4`  
**Corte material verificado:** `80aee0a48adb72710770adbfbe9f5d0d71585a6f`  
**Solicitud de incorporación:** `#78`  
**Estado:** `CANDIDATA_VERIFICADA_NO_PROMOVIDA`

## 1. Controles realizados

La realización impone antes de exponer la representación intermedia:

- resolución de cada `NodeId` como `CoupledSpec`;
- pertenencia del nodo al `CompositionGraph` del horizonte;
- posición dentro de `[1,n]`, sin estrechamiento de `Nat`;
- unicidad del destino `(node_ref, position)`;
- unicidad local del tipo de suceso dentro de cada `TransitionData`.

Dos nodos diferentes pueden compartir `CellSpec` y conservar destinos distintos. También permanecen protegidas las guardas tipadas de `Frame.architecture` y `compose(graph)`, la geometría `b/n`, la longitud del vector y los metadatos de proyección.

## 2. Insuficiencia de cobertura y corrección

El corte `9a79e1370afa3f22ab85f94b455bfc5aa1ac0d82` acreditó los nueve casos causales mediante una batería nativa separada. Una mutación de la comprobación de pertenencia arquitectónica hizo fallar las pruebas Rust, pero no el corpus compartido, porque éste aún no contenía esos testigos.

La candidata verificada:

- incorpora los nueve casos en `tests/conformance/invalid/`;
- hace que el comprobador causal consuma los mismos archivos y sus fragmentos causales específicos;
- elimina las copias separadas;
- amplía el corpus de `14 + 86` a `14 + 95`;
- alinea la comprobación de navegador con el cardinal de 95 inválidos;
- ejecuta el corpus ampliado en nativo, WASI y navegador.

## 3. Deuda expresamente registrada

- **DFL-012:** la clave histórica `cell_ref` transporta una identidad `NodeId` resuelta como `CoupledSpec`; la migración de nombre queda reservada a una versión incompatible posterior del esquema.
- **DFL-013:** nativo, WASI y navegador comparten `sv_core`; su paridad no constituye independencia entre realizaciones semánticas.

## 4. Evidencia

| Control | Ejecución o artefacto | Resultado |
|---|---:|---|
| Conformidad SVP | `34164984055` | conforme |
| R0-8 Baseline nativa | `34164984130` | conforme |
| R0 Rust | `34164984049` | conforme |
| R0 WASM, WASI y navegador | `34164984231` | conforme |
| `sv_core` | dentro de R0 Rust | `211/211` |
| Corpus compartido | nativo, WASI y navegador | `14/14` válidos y `95/95` inválidos |
| Casos causales | dentro de R0 Rust | `9/9` |
| Mutaciones dirigidas | dentro de R0 Rust | `12/12` detectadas; `0` supervivientes; `0` inválidas |
| Observador | `10033851370` | SHA-256 `bab862e9254ee3b5ba2902f5e58ab433bcf8e6ed8493c034411dd8b8e8c78438` |
| Mutaciones | `10033855035` | SHA-256 `b4744a0e595aba79f71b824f8a164bda09a40dda27973127259d828ed0e6facb` |
| Paridad WebAssembly | `10033866342` | SHA-256 `b701aad62c25b5db3834860affb9df82d66e8a70cfc20c58b98c76fd5322ff33` |

La comprobación de referencia utilizó Rust 1.98.0 y la compatibilidad complementaria resolvió Rust 1.98.1. La campaña de doce mutaciones es finita y no acredita cobertura exhaustiva.

## 5. Decisión

```text
H04 = CERRADO_EN_CANDIDATA
H05 = CERRADO_EN_CANDIDATA
COBERTURA_COMPARTIDA_H04_H05 = VERIFICADA_EN_NATIVO_WASI_Y_NAVEGADOR
DFL_012 = ABIERTA_Y_GOBERNADA
DFL_013 = ABIERTA_Y_GOBERNADA
PROMOCION = PENDIENTE
FILA_7 = ABIERTA
```

La promoción exige la comparación final contra la recepción G/H y el mantenimiento del orden de integración. La fila 7 continúa abierta; DFL-005, H06, H07, la causalidad entre datos de transición y marcos, la migración de `cell_ref` y la independencia semántica siguen sin cerrar.

## 6. Documentos y pruebas vinculados

- `docs/arquitectura/ACTA_TECNICA_DE_CORRECCION_LOCAL_DE_TRANSITIONDATA_Y_REFUERZO_DE_ORACULOS_NATIVOS_2026_09_07.md`;
- `docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`;
- `docs/calidad/RETP_2026_093_CORRECCION_TRANSITIONDATA_Y_ORACULOS_NATIVOS.csv`;
- `tests/run_row7_transitiondata_conformance.py`;
- `tests/run_directed_mutation_sensitivity.py`;
- `tests/conformance/invalid/`.

## 7. Rectificación registral de 08/09/2026

DFL-011 pertenece al mandato del español. Las referencias candidatas anteriores se corrigen: nombre histórico `cell_ref`, DFL-011 → DFL-012; independencia semántica, DFL-012 → DFL-013. Se conservan contenido, límites y evidencia de cada obligación. La evidencia de §4 pertenece al corte `80aee0a`; la nueva corrección del observador y su verificación se registran en [RETP-094 del maestro](./REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-2026-094--discriminacion-causal-e115-y-rectificacion-registral).
