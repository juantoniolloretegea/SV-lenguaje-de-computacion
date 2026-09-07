# Acta técnica de corrección local de `TransitionData` y refuerzo de sus comprobaciones

**Fecha:** 7 de septiembre de 2026  
**Identificador registral:** `RETP-2026-093`  
**Base material:** `bc3b22c9e9319e8f191390c8cfe9fa1577904d87`  
**Recepción G/H de base:** `9b32edda21072b3d2a138d9e92d95febdbc21ac4`  
**Corte material previamente verificado:** `9a79e1370afa3f22ab85f94b455bfc5aa1ac0d82`  
**Solicitud de incorporación:** `#78`  
**Estado:** `CANDIDATA_EN_REVERIFICACION`

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

## 5. Cobertura multientorno

El corte material `9a79e137…` contenía nueve casos causales específicos en una batería nativa separada. La campaña compartida de conformidad mantenía entonces 14 casos válidos y 86 inválidos, pero no incluía esos nueve testigos.

Una comprobación independiente neutralizó la regla de pertenencia del nodo al grafo del horizonte. Las pruebas Rust detectaron la mutación, mientras la campaña compartida permaneció conforme. El resultado demostró una insuficiencia de cobertura por destino: la regla estaba realizada y protegida nativamente, pero las ejecuciones WASI y de navegador no la ejercitaban mediante el corpus común.

La candidata corrige esa insuficiencia de la forma siguiente:

1. los nueve testigos causales pasan al corpus compartido `tests/conformance/invalid/`;
2. el comprobador causal mantiene mensajes exactos, pero consume los mismos archivos del corpus común;
3. se evita conservar dos copias divergentes de las mismas fuentes;
4. el corpus esperado pasa a 14 casos válidos y 95 inválidos en los destinos aplicables;
5. la promoción exige una ejecución nueva sobre la cabeza exacta resultante.

La conformidad anterior de los destinos se conserva como antecedente del corte `9a79e137…`, pero no acredita por sí sola la ampliación del corpus.

## 6. Deuda de esquema e independencia semántica

La proyección 0.1.0 conserva la clave histórica `cell_ref` para el primer componente de `induced_parameters`, aunque su valor representa ahora una identidad `NodeId` resuelta como `CoupledSpec`. Cambiar esa clave dentro de la misma versión alteraría el esquema de forma encubierta. La deuda queda registrada como DFL-011, con migración explícita a `node_ref` o denominación equivalente en una versión incompatible posterior.

Las vías nativa, WASI y de navegador ejecutan la misma custodia Rust de `sv_core` sobre destinos diferentes. La paridad entre ellas acredita conservación de transporte y de observables, pero no independencia entre realizaciones semánticas. Esta pérdida queda registrada como DFL-012. Los resultados esperados comprometidos, las pruebas causales, las mutaciones y la comparación exacta de bytes son controles compensatorios; no sustituyen una segunda realización independiente ni un comprobador derivado de la doctrina.

## 7. Evidencia antecedente reproducida

Sobre el corte material `9a79e137…` quedaron acreditados:

| Comprobación | Resultado |
|---|---|
| `sv_core` | `211/211` |
| Nuevas pruebas correctivas | `16/16` |
| Corpus compartido anterior | `14/14` válidos y `86/86` inválidos |
| Casos causales nativos | `9/9` |
| Mutaciones dirigidas | `12/12` detectadas; `0` supervivientes; `0` inválidas |
| Rust de referencia | `1.98.0`, conforme |
| Compatibilidad adicional | `stable = 1.98.1`, conforme |
| Conformidad SVP | ejecución `34161898934`, conforme |
| R0-8 nativa | ejecución `34161898968`, conforme |
| R0 Rust | ejecución `34161898907`, conforme |
| R0 WASM y navegador | ejecución `34161898894`, conforme |

La relación `12/12` describe la muestra de mutaciones dirigida y no constituye cobertura exhaustiva del repositorio.

## 8. Condición de promoción

La cabeza revisada sólo podrá promoverse si:

1. las cuatro comprobaciones de integración terminan correctamente sobre la cabeza exacta;
2. el corpus compartido acredita 14 casos válidos y 95 inválidos en nativo, WASI y navegador;
3. los nueve casos conservan su causa exacta en el comprobador causal;
4. las mutaciones dirigidas continúan detectadas y los archivos quedan restaurados;
5. la comparación de cambios no incorpora dominio, infraestructura, plataforma ni documentación ajena al alcance;
6. DFL-011 y DFL-012 permanecen explícitas y no se presentan como capacidades resueltas.

Hasta entonces:

```text
CIERRE_CORRECTIVO = CANDIDATO_EN_REVERIFICACION
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
