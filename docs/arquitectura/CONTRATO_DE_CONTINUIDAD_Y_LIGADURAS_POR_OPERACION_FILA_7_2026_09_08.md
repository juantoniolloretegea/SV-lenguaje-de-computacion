# Continuidad y ligaduras por operación · candidata de fila 7

**Fecha:** 8 de septiembre de 2026. **Estado:** candidata; no cierre de la fila 7.

## 1. Cortes, mandato y orden de prueba

Se recibe `main@1706099aef4a0e3846706c3963e7c76313adaf68`, árbol `3f1858439745236530b6f19c6761055f8a8478a2`, tras integrar las PR #77 y #78. El retorno G/H procede de `SVperitus-dataset@54fe0d89c9e59065eae2bc8a38f5ec0832ece4b9`. Su dictamen sigue siendo **suficiencia no acreditada para ejecutar Q0**.

Fuentes leídas: [Pilares](../calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md), [acta de perfiles y contratos de 06/09/2026](../calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md), tabla de catorce filas de la transición, F-SV/0.1-candidata, IR 0.2/J4.2/J5.1, sucesiones IR 0.3 y RETP-091…095. Los localizadores exactos se conservan en esos registros; la fecha histórica no sustituye la sucesión vigente.

Este contrato fija las condiciones y los esperados antes de cambiar la realización. Las correcciones de contexto se justifican por G/H §6.4/SP-02, SP-05, SP-06 y SP-12; no se emplea el caso clínico para inventar tipos universales. PT01/PT03 delimitan constitución y consumo; PT04 exige comprobar la frontera; PT14 liga la corrección, su versión y sus pruebas.

## 2. H06: coherencia estructural de una trayectoria

La superficie actual no constituye un operador de cambio de arquitectura. En este alcance, la arquitectura de la trayectoria es la referencia explícita del primer marco. Cada marco posterior y el horizonte de cada transición deben referir **esa misma identidad de `CompositionGraph`**. Igualdad de nodos, de `CellSpec` o de contenido no sustituye igualdad de referente. Un cambio futuro de arquitectura necesita un contrato explícito; no se deduce de la yuxtaposición de marcos.

Esta regla añade la comprobación relacional que J4.2 exige a la alternancia ya comprobada. No exige aumentar índices, rehacer vectores, ni deduce que el marco siguiente sea el resultado de ejecutar la transición. Una secuencia de longitud uno es válida. Se conservan el orden y todos los objetos de entrada.

No se acredita con esta comprobación estática la relación entre versiones sucesivas de una historia, su persistencia ni la ejecución del operador inducido. `append_only: true` en una proyección no prueba por sí solo esas propiedades temporales.

## 3. H07: pertenencia estructural del contexto

La arquitectura y el dominio son los referidos explícitamente por `Agent`. La realización comprobará las siguientes condiciones necesarias antes de admitir la declaración de consulta:

| Contexto | Condición local |
|---|---|
| `PointEval` | El marco pertenece a la arquitectura del agente. |
| `TrajectoryView` | La trayectoria satisface §2 y su arquitectura coincide con la del agente. |
| `FrameComparison` | Ambos marcos pertenecen a la arquitectura del agente. La comparación no constituye migración de arquitectura. |
| `ArchitectureView.cells` | Cada `CellSpec` es la especificación de al menos un nodo declarado del grafo. La vista contiene especificaciones, no identidades de instancia. |
| `ArchitectureView.evals` | La evaluación procede de un `CoupledState` cuyo nodo pertenece al grafo, o de un `CellState` cuya especificación está representada en él. El segundo caso no determina un nodo cuando varios comparten especificación. |
| `ArchitectureView.gates` | Cada evaluación de entrada de cada compuerta satisface la condición anterior, aunque no figure además en la lista explícita de evaluaciones de la vista. |
| `CoverageReport` | Los tres referentes coinciden con `Agent.domain`, `Domain.interface` y `Domain.silent_u`, respectivamente. No basta comprobar el primero. |

Las dos últimas cadenas de `CoverageReport` siguen siendo nombres nominales de la representación recibida: esta corrección **no** las convierte en objetos `Interface` o `SilentU` materializados. Tampoco se infiere permiso ni cobertura completa por pertenecer al mismo grafo. La aceptación no ejecuta `query`, no materializa `restrictions` ni satisface CQ1–CQ6 completos. DFL-003, DFL-004 y DFL-005 conservan esas obligaciones.

No se asignan códigos E nuevos por semejanza. Los rechazos locales identificarán trayectoria o consulta, referente atacado y causa concreta. DFL-001 conserva la concordancia diagnóstica general.

## 4. DFL-005: contrato de entrada, todavía sin realización

La unidad mínima que debe constituirse es **la ligadura consumida por una operación identificada**, no una lista universal de campos clínicos. Se propone un contrato tipado versionado, enlazado al programa exacto y distinguible de la proyección 0.1.0. La sede final entre extensión de IR y manifiesto externo comprobado por el núcleo no queda escogida por conveniencia del serializador: debe permitir rechazar la ligadura inválida en toda entrada que ofrezca esa capacidad.

| Componente necesario | Condición propuesta | Fuente y refutador previo |
|---|---|---|
| Constitución | Identidad, versión y referente exactos de dominio y programa; ninguna elección de «vigente» o «última». | F §3.1; SP-01: sustituir versión o contenido conservando el nombre debe invalidar el enlace. |
| Instancia paramétrica | Identificador propio, propietario declarado y vínculo al parámetro del dominio. Dos instancias no colapsan por compartir tipo o numeral. | SP-02; GH-DOC-07: mismo inventario con ligaduras permutadas debe conservar la diferencia. |
| Destino celular, cuando se consume | `(NodeId, posición)`; nodo constituido en el grafo, posición natural en `[1,n]`, `n` derivado de la especificación enlazada. Ausencia de célula constituida es insuficiencia para esa operación. | F §3.1; SP-02: nodo ajeno, posición inválida o nueve nombres sin constitución no producen célula. |
| Captura y admisión | Referencias tipadas a `CaptureSpec` y `AdmissibilitySpec` del dominio; identidad paramétrica concordante, más referente de la regla/versionado. | SP-02/03: coincidencia de `parameter_id` o de espacio por sí sola no prueba la ligadura completa. |
| Transducción | Referente sólo declarativo mientras K1-T siga cerrado. La presencia de `Ternarizer` no habilita producción de ningún `Tri`. | SP-03: `Bottom`, `NotAdmitted` o configuración ausente nunca se sustituyen por U. |
| Uso y orden | Operación y versión, lista ordenada de ligaduras consumidas y distinciones imprescindibles. Una pérdida no invalida automáticamente otro uso que no consume esa distinción. | SP-05/06: permutar orden debe ser observable; el resumen no recupera una dependencia omitida. |
| Compartición | Régimen expreso para uso repetido de una instancia; alias y duplicación de instancia son relaciones distintas. Sin regla no se selecciona, deduplica ni replica. | F §3.1; SP-02: dos nodos con una misma `CellSpec` conservan identidades distintas. |
| Procedencia y autoridad | Referentes exactos de fuente, regla y autor constituyente. Una huella comprueba identidad de bytes; no autentica ni concede autorización. | GH-DOC-01/02/04/05/08; SP-01/10: mismo resumen con distinto antecedente no acredita igual fundamento. |
| Información lateral | Si se necesita S, identificarla y certificar la entrada conjunta `(H,S)`. | SP-05 y F §4: usar S mientras se atribuye suficiencia a H sola refuta la certificación. |

Estos refutadores son **obligaciones de la futura candidata DFL-005**, no pruebas ejecutadas por este incremento. Faltan su representación material, entradas completas y comprobación por el núcleo. Se mantiene bloqueada toda capacidad que dependa de esas ligaduras; corregir H06/H07 localmente no cierra DFL-005.

## 5. Matriz de tratamiento del retorno

Esta matriz gobierna el incremento; no se presenta como matriz final de resolución de pérdidas de la fila 7.

| Retorno | Tratamiento actual | Responsable, efecto y condición de retorno |
|---|---|---|
| SP-01/02 | Contrato de ligaduras §4; cierre H04/H05 integrado; H06/H07 locales candidatos. | Lenguaje, fila 7. DFL-005 sigue bloqueante por operación; falta su candidata material. No pedir al dominio que diseñe la IR. |
| SP-03 | K1-T sin producción observación→Tri. | Lenguaje para realización; dominio/institución para reglas. No habilitar sin partición y función constituidas y probadas. |
| SP-04 | DFL-006 abierta. | Lenguaje para productor y traza; dominio para criticidad y autoridad. Ninguna operación de criticidad integrada ofrecida. |
| SP-05 | Se conservan las ocho pérdidas GH-DOC y F-IF; §3 sólo refuerza pertenencia. | Lenguaje/interfaz, fila 7: falta recuperación por operación y declaración de información lateral. |
| SP-06/07 | Orden, veto y cuatro salidas de Q0 conservados como contrato recibido; sin ejecución integrada. | Lenguaje/motor con reglas de dominio; no inferir esta capacidad de `compose`, `Codomain` o `query`. |
| SP-08/09 | Conformidad de compilación y paridad de destinos en su alcance. | Lenguaje/motor: reproducción OP y fallo técnico integrado pendientes; no salida clínica ante fallo. |
| SP-10 | Identidad y legitimidad humana sin suplir por nombres. | Institución/dominio y circuito R1 integrado; no se otorgan permisos desde §3. |
| SP-11 | Obligaciones materiales asignadas a R2/R3/R4; DFL-009 fila 9, frontera fila 13. | Soporte competente; bloquean ofrecer la operación dependiente, no este retorno representacional. |
| SP-12 | Perímetro Q0 v0 conservado. | Inmunología en pausa; CYB fila 8 sólo tras candidata identificada de fila 7. No se constituye aquí su universo. |

`REQ-IMM-SV-011` conserva `U_NO_DECIDIDO`; 27 tipos y seis agrupaciones no constituyen células. DFL-011 conserva la revisión integral del español antes del cierre final. DFL-012 conserva el versionado incompatible de `cell_ref`; DFL-013 conserva la ausencia de segunda realización semántica. Ninguna se renumera.

## 6. Criterio de comprobación del incremento

Cada negativo local tendrá un programa sintácticamente válido, una única condición atacada, diagnóstico completo esperado y un control positivo que repare sólo esa condición. Se ejecutará por las entradas ordinaria, perfilada y de ensamblaje. Los negativos comprometidos entrarán en el corpus común nativo/WASI/navegador. Las mutaciones retirarán las guardas por separado; un fallo de compilación del mutante no contará como detección.

La promoción de este incremento sólo acreditará la coherencia local descrita en §§2–3. La fila 7 permanecerá abierta hasta resolver su contrato de ligaduras y la matriz final, identificar la capacidad realmente ofrecida y obtener la candidata apta para el segundo falsador.
