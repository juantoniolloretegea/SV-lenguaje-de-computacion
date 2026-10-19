# Recepción técnica de pertenencia y conservación causal

19 de octubre de 2026 · RETP-2026-325 · Primera recepción técnica de la entrega consolidada. Alcance: R1 intraproceso; contribución parcial a N-01, N-03 y N-10. La adenda OP-CYB-001 y R2 permanecen abiertas.

## Fuentes y condición de entrada

La realización parte de `SV-lenguaje-de-computacion@9f468e6ab45613f2e109f5432d02e43f27b3b717`. Se han leído completos los Pilares del 05/09, el acta de perfiles del 06/09 y el acta de transición del 03/09 con sus relevos. Se reciben la norma Código limpio y auditable, la política de protección de datos, S13, S16 y R2-0. La [entrega consolidada](../../../evoluciones-desde-el-nucleo-no-cotejadas-generadas-por-pruebas-con-la-ia-sin-aprobacion-aun/entrega-consolidada-2026-10-19/ENTREGA.md) conserva las obligaciones y las funciones responsables.

Esta recepción sucede al [contraste RETP-153](../pertenencia-de-permisos-a-su-continuidad/README.md) exclusivamente respecto de la realización de pertenencia. Conserva el antecedente, sus resultados y sus límites. El [contrato RETP-154](../recepcion-gobernada-y-comprobacion-observada/CONTRATO_Y_FRONTERA.md) se ha revisado: requiere una premisa externa ya producida y no realiza su admisión material. No se incorpora como sustituto de esa admisión ni como verificador profesional.

## Defecto y corrección

Dos continuidades distintas pueden tener el mismo contenido constituido y el mismo ordinal local `decision:1`. La versión de partida aceptaba el permiso de una en la otra; también permitía ejecutar un compromiso ajeno cuando ambas tenían trazas equivalentes. Los testigos originales reprodujeron los dos defectos sobre la base identificada: PC01 conforme; PC02 y PC03 desfavorables. En PC03 se observó una llamada al ejecutor sintético y dos eventos en la continuidad receptora.

Cada continuidad protegida recibe ahora una pertenencia interna que se transmite al permiso y al compromiso. Se comprueba antes de mediar y antes del despacho. El ordinal público y las reglas de autoridad se conservan. Un contador atómico asigna valores distintos sin reutilización; su agotamiento impide producir una nueva decisión. La operación legítima sigue disponible tras rechazar una pieza ajena.

La pertenencia está limitada a una instancia cargada del núcleo, dentro del proceso y bajo Rust seguro. No es un secreto, un comprobante institucional, una identidad durable ni una garantía frente a un anfitrión que manipule memoria. Mover el objeto conserva su pertenencia. Reiniciar un proceso no recupera autoridad mediante ese contador. No se añade serialización de permisos ni constructores públicos.

Los cambios se limitan a `decision_trace.rs`, la declaración del módulo de pruebas en `authority/transitions/reuse_tests.rs` y el nuevo archivo `authority/transitions/reuse_tests/continuity_membership.rs`. Las pruebas precedentes conservan sus cuerpos y esperados. No cambian gramática, IR, Frame, terna, polígono, dependencias o contratos profesionales.

## Resultados y revisión adversarial

Los comandos, resultados estructurados y huellas se conservan en [RESULTADOS.json](RESULTADOS.json). Se distinguen ejecuciones, compilación y ausencia de ejecución.

| Contraste | Resultado y límite |
|---|---|
| Cadena propia | Se conserva una única llamada al ejecutor y su traza. |
| Permiso y compromiso ajenos con contenido igual | Rechazo anterior a mediación o despacho; no se añaden efectos en el receptor. |
| Agotamiento y concurrencia | No hay desbordamiento ni reutilización; 128 asignaciones concurrentes distintas en ocho hilos. |
| Movimiento, destrucción del emisor y rechazo previo | El movimiento conserva pertenencia; la destrucción del emisor no legitima su compromiso en otro receptor; el rechazo ajeno no consume la operación propia. |
| Fallo posterior al despacho | La vía pública conserva decisión, causa del adaptador y estado indeterminado. Una nueva decisión no permite repetir el efecto de uso único; el segundo ejecutor no recibe llamada. |
| Sensibilidad | Al retirar las guardas de mediación, ejecución o uso único, o permitir que el contador desborde, falla el testigo correspondiente. Cuatro alteraciones detectadas; fuentes restituidas por huella y diez controles posteriores conformes. No se cuentan errores de compilación como detección del defecto. |
| Windows | 221 pruebas unitarias conformes en depuración y 221 en optimización. |
| WASI | 220 pruebas unitarias conformes; se excluye expresamente la prueba concurrente porque este soporte no ofrece hilos. |
| Conjunto Linux | 379 pruebas conformes, incluidas las de documentación, en 27 bloques. No equivalen a todos los procedimientos de conformidad de la integración continua. |
| Módulo para navegador | Compilación comprobada; no se afirma ejecución del recorrido de autoridad en navegador. Ese adaptador no expone R1. |
| Clippy | Sin errores. Dieciséis diagnósticos únicos presentes también en la base, sin incremento por causa, emisor o mensaje. No se presenta como ausencia de advertencias. |
| Cliente externo | El cliente de control compila y se ejecuta. Mutar la pertenencia privada falla con E0616; invocar el mecanismo exclusivo de pruebas falla con E0599. Son códigos de Rust, no diagnósticos SV. |

La prueba del agotamiento sólo existe en la compilación de pruebas. El montaje de autoridad es el banco sintético de R1; no acredita una raíz profesional, una entrada remota explotable ni funcionamiento de producción. La ausencia de contenido a proveedores se conserva; estas pruebas no realizan inferencia ni examen de modelos.

Los observadores comprueban la causa concreta, llamadas, eventos, pertenencia, conservación y repetición. Rechazar todo no superaría PC01, PC07, PC08 o PC09. La base defectuosa y las alteraciones controladas aportan controles distintos de una mera coincidencia entre realizaciones.

## Diagnósticos y compatibilidad

| Variante tipada | Explicación en español | English explanation |
|---|---|---|
| `TracedDecisionError::ContinuityExhausted` | No se dispone de una nueva identidad interna; no se ha producido la decisión. | No fresh internal identity is available; no decision was produced. |
| `TracedMediationError::ForeignContinuity` | El permiso pertenece a otra continuidad; no se ha mediado. | The permit belongs to another continuity; mediation did not occur. |
| `TracedExecutionError::ForeignContinuity` | El compromiso pertenece a otra continuidad; no se ha despachado al ejecutor. | The commitment belongs to another continuity; nothing was dispatched to the executor. |

Son causas técnicas de R1, sin nuevas claves canónicas E ni conversión a U. Se emiten en el punto de comprobación, sin interpretar prosa ni interpolar datos privados. Las variantes públicas son aditivas y pueden exigir adaptar coincidencias exhaustivas de consumidores externos. En los consumidores incluidos en el conjunto revisado no se han encontrado coincidencias exhaustivas adicionales afectadas. La localización global del compilador y su entrega visible conservan su alcance pendiente; esta tabla no las sustituye.

## Recepción y continuación

Se recibe técnicamente la corrección de pertenencia dentro del alcance ensayado. La incorporación a las fuentes del núcleo no activa una operación profesional ni constituye una aceptación independiente de todo el sistema.

| Obligación restante | Función responsable y condición de recepción |
|---|---|
| N-01; adenda B y G; DFL-005/006 | Integración de autoridad y frontera: materializar la admisión del acto y la producción de sus comprobaciones; probar origen, alcance, sustitución, vigencia y rechazo antes del efecto. El comparador de bytes no puede acreditar facultades. |
| N-02, N-04 y N-08; adenda A/C/D/H/I | Integración documental y dominio: comprobar cobertura independiente, contenido realmente consumido, contexto y fidelidad hasta la operación concreta. |
| N-05 y N-09; adenda E/K/L | Integración de revisión y entrega: imponer objeto presentado, competencia, finalidad, contenido mínimo y destinatario; observar los canales comprendidos. |
| N-06 y obligaciones S13 | Desarrollo nuclear: realizar consulta ejecutiva, transición y transducción bajo sus contratos; la representación nominal no basta. |
| N-07 | Soporte y Árbitro-Director: imponer y observar las cotas en los recorridos ofrecidos. La cota del ejecutor WASI no acredita el control de todos los recursos del SV. |
| N-03/N-10 y R2 | Continuidad nuclear: realizar conservación, dependencias, revocación y recuperación sin resucitar autorizaciones. El efecto indeterminado probado en memoria debe conservarse también en la recuperación durable. |

Se mantiene el orden rector y la matriz consolidada, sin nueva numeración paralela de deuda. No se solicita al autor que vuelva a definir la autoridad del experto: está recibida; su realización y comprobación siguen siendo obligaciones técnicas. No se declara completada la adenda ni R2. La aceptación integrada exige ejecutar el recorrido profesional legítimo con sus productores materiales y recibir cada capacidad por su alcance.

El Manual del Lenguaje, capítulo «Desarrollo y validación del núcleo», conserva una explicación aplicable: distingue realización parcial y recepción integrada y remite al registro de deuda vigente. Esta recepción se enlaza desde ese registro, sin duplicar sus detalles en el manual ni modificar su fecha por una capacidad general que continúa pendiente.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
