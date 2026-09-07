# Acta técnica de corrección local de `TransitionData` y refuerzo de sus comprobaciones

**Fecha:** 7 de septiembre de 2026  
**Identificador registral:** `RETP-2026-093`  
**Base material:** `bc3b22c9e9319e8f191390c8cfe9fa1577904d87`  
**Recepción G/H de base:** `9b32edda21072b3d2a138d9e92d95febdbc21ac4`  
**Corte material verificado:** `80aee0a48adb72710770adbfbe9f5d0d71585a6f`  
**Solicitud de incorporación:** `#78`  
**Estado:** `CANDIDATA_VERIFICADA_NO_PROMOVIDA`

## 1. Objeto

Este acto corrige la bienformación local de `TransitionData`, protege invariantes estructurales relacionados y extiende la comprobación de las reglas nuevas a los destinos nativo, WASI y navegador.

La corrección es intrínseca al Lenguaje. No ejecuta transiciones, no constituye causalidad entre marcos, no incorpora contenido de Inmunología y no abre el segundo contraste de dominio.

## 2. Identidad del destino inducido

La representación intermedia define:

```text
induced_parameters : [(NodeId, Nat, Tri)]
```

En la representación vigente, cada `NodeId` debe resolver la identidad de un `CoupledSpec` perteneciente al `CompositionGraph` referido por el horizonte. No puede reducirse a la `CellSpec` subyacente, porque nodos distintos pueden compartir una misma especificación celular sin perder su identidad arquitectónica.

El juicio aplicado es:

```text
(node_ref, position, value)
```

con las condiciones siguientes:

1. `node_ref` resuelve un `CoupledSpec` declarado;
2. el nodo pertenece al `CompositionGraph` del `Horizon` referido;
3. `position` pertenece a `[1,n]`, donde `n` deriva de la `CellSpec` enlazada por el nodo;
4. la comparación no estrecha `Nat` a un entero de máquina;
5. cada par `(node_ref, position)` aparece como máximo una vez dentro del objeto.

Dos nodos distintos que compartan `CellSpec` pueden, por tanto, emplear la misma posición sin convertirse en el mismo destino.

## 3. Funcionalidad local de sucesos

Dentro de un mismo `TransitionData`, cada tipo de suceso aparece como máximo una vez. La recurrencia del mismo tipo en objetos de datos de transición diferentes permanece permitida.

La restricción corresponde a la representación actual, que no contiene una identidad de ocurrencia capaz de distinguir dos apariciones del mismo tipo dentro del mismo objeto.

## 4. Realización y diagnóstico

La comprobación se materializa en `rust/sv_core/src/transition_data_wellformed.rs` y se aplica, después de la bienformación general, en las tres vías de compilación:

- `compile_svp`;
- `compile_svp_profile`;
- `compile_svp_assembly`.

Los rechazos se encuadran actualmente en `E406 — InsufficientTransitionData` y distinguen por mensaje la referencia ausente, el tipo incorrecto, la pertenencia arquitectónica, el rango, el destino repetido y el tipo de suceso repetido. La encapsulación general en `CompileError::InvalidProgram(String)` conserva la deuda de diagnóstico estructurado.

También se mantienen regresiones para:

- `Frame.architecture : CompositionGraph`;
- `compose(graph) : CompositionGraph`;
- `b ≥ 3` y `n=b²`;
- longitud del vector;
- metadatos de la proyección.

## 5. Cierre de la insuficiencia de cobertura por destino

El corte material anterior `9a79e1370afa3f22ab85f94b455bfc5aa1ac0d82` contenía nueve casos causales específicos en una batería nativa separada. La campaña compartida mantenía 14 casos válidos y 86 inválidos, pero no incluía esos nueve testigos.

Una verificación independiente neutralizó la comprobación de pertenencia del nodo al grafo del horizonte. Las pruebas Rust detectaron la mutación, mientras la campaña compartida permaneció conforme. El resultado demostró una insuficiencia de cobertura por destino: la regla estaba realizada y protegida nativamente, pero las ejecuciones WASI y de navegador no la ejercitaban mediante el corpus común.

La candidata verificada resuelve esa insuficiencia de la forma siguiente:

1. los nueve testigos causales pertenecen ahora al corpus compartido `tests/conformance/invalid/`;
2. el comprobador causal consume esos mismos archivos y conserva la identidad textual exacta de cada rechazo;
3. se eliminaron las copias separadas para impedir divergencias entre fuentes de prueba;
4. el corpus compartido pasa a 14 casos válidos y 95 inválidos;
5. la aserción de navegador quedó alineada con el cardinal efectivo de 95 inválidos;
6. las vías nativa, WASI y de navegador ejecutaron el corpus ampliado sobre el corte material identificado.

El fallo previo de la comprobación de navegador tras ampliar el corpus correspondía a la expectativa cardinal obsoleta de 86 casos. No reveló una divergencia semántica; su corrección consistió exclusivamente en exigir el cardinal nuevo de 95.

## 6. Deuda de esquema e independencia semántica

La proyección 0.1.0 conserva la clave histórica `cell_ref` para el primer componente de `induced_parameters`, aunque su valor representa una identidad `NodeId` resuelta como `CoupledSpec`. Cambiar esa clave dentro de la misma versión alteraría el esquema de forma encubierta. La deuda queda registrada como DFL-011, con migración explícita a `node_ref` o denominación equivalente en una versión incompatible posterior.

Las vías nativa, WASI y de navegador ejecutan la misma custodia Rust de `sv_core` sobre destinos diferentes. La paridad entre ellas acredita conservación de transporte y de observables, pero no independencia entre realizaciones semánticas. Esta pérdida queda registrada como DFL-012. Los resultados esperados comprometidos, las pruebas causales, las mutaciones y la comparación exacta de bytes son controles compensatorios; no sustituyen una segunda realización independiente ni un comprobador derivado de la doctrina.

## 7. Evidencia de verificación

La comprobación de referencia utilizó Rust 1.98.0. La comprobación complementaria resolvió el canal estable como Rust 1.98.1. Los cuatro flujos terminaron correctamente sobre `80aee0a48adb72710770adbfbe9f5d0d71585a6f`:

| Comprobación | Ejecución | Resultado |
|---|---:|---|
| Conformidad SVP | `34164984055` | conforme |
| R0-8 Baseline nativa | `34164984130` | conforme |
| R0 Rust | `34164984049` | conforme |
| R0 WASM, WASI y navegador | `34164984231` | conforme |
| Pruebas unitarias de `sv_core` | dentro de R0 Rust | `211/211` |
| Corpus compartido | dentro de R0 Rust y del flujo WebAssembly | `14/14` válidos y `95/95` inválidos |
| Conformidad causal de la fila 7 | dentro de R0 Rust | `9/9` |
| Mutaciones dirigidas | dentro de R0 Rust | `12/12` detectadas; `0` supervivientes; `0` inválidas |
| Sensibilidad del observador | dentro de R0 Rust y WASI | conforme |
| Ejecución en navegador real | dentro del flujo WebAssembly | conforme |

Evidencia preservada:

```text
oracle-sensitivity
artefacto = 10033851370
sha256 = bab862e9254ee3b5ba2902f5e58ab433bcf8e6ed8493c034411dd8b8e8c78438

directed-mutation-sensitivity
artefacto = 10033855035
sha256 = b4744a0e595aba79f71b824f8a164bda09a40dda27973127259d828ed0e6facb

r0-wasm-three-way-parity
artefacto = 10033866342
sha256 = b701aad62c25b5db3834860affb9df82d66e8a70cfc20c58b98c76fd5322ff33
```

La relación `12/12` describe únicamente la muestra dirigida. No constituye cobertura exhaustiva de mutación del repositorio.

## 8. Alcance del cierre candidato

Quedan cerrados o protegidos en la candidata verificada:

```text
H04_REFERENCIA_DE_NODO_PERTENENCIA_Y_RANGO = CERRADO_EN_CANDIDATA
H05_FUNCIONALIDAD_LOCAL_DE_SUCESOS_Y_DESTINOS = CERRADO_EN_CANDIDATA
M08B_FRAME_ARCHITECTURE = PROTEGIDO_POR_REGRESION
M08C_COMPOSE_GRAPH = PROTEGIDO_POR_REGRESION
P02_GEOMETRIA_CELULAR = PROTEGIDA_POR_REGRESION
METADATOS_DE_PROYECCION = PROTEGIDOS_POR_REGRESION
COBERTURA_COMPARTIDA_H04_H05 = VERIFICADA_EN_NATIVO_WASI_Y_NAVEGADOR
```

La promoción sigue separada de la verificación. Requiere revisar la comparación final contra la recepción G/H, mantener el orden de integración de las solicitudes y confirmar que el diferencial no incorpora objetos ajenos al alcance.

```text
CIERRE_CORRECTIVO = CANDIDATA_VERIFICADA
PROMOCION = PENDIENTE
FILA_7 = ABIERTA
CANDIDATA_PARA_SEGUNDO_CONTRASTE = NO_EMITIDA
NUCLEO_CONSOLIDADO = NO
```

## 9. Límites

Permanecen abiertas:

- DFL-005 y las ligaduras completas de `Domain` y `Agent`;
- la ejecución y causalidad material de una transición;
- la relación entre `TransitionData` y los marcos anterior y posterior;
- la coherencia arquitectónica completa de `Trajectory`;
- la pertenencia de consultas, evaluaciones e informes de cobertura;
- la independencia entre realizaciones semánticas;
- la migración del nombre histórico `cell_ref`.
