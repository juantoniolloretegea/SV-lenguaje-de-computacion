# S26 R06 · Recepción de terminación y referentes materiales

**RETP-2026-233 · 13 de septiembre de 2026. Estado: recepción contractual; realización pendiente.**

## 1. Corte, necesidad y sede

Continuación autorizada por Juan Antonio Lloret Egea después de S26 R05. Corte del Lenguaje: `b6be761da7223d9f26e5d7a41b83092330597276`; laboratorio: `8a989c78b69b7a944a4fa1d5195c82d254d80b8a`, rama `lab/playground-sv-permanente`.

Se reciben dos límites observados en [R05](../r05/README.md): P04/P05 terminan por pánico sin devolver `MaterialRun`; P07 devuelve concordancia de bytes y recibo pese a sustitución del objeto de archivo. P06 distingue los bytes del objeto abierto de los que después obtiene otra apertura de la misma ruta. Estos resultados no prueban una pérdida de identidad de ocurrencia SV, ni una ejecución exactamente una vez, ni protección frente al host.

R06 desarrolla [R02 C01–C04 y C06–C08](../RECEPCION_CONTRACTUAL_R02.md), [R2-0 §§4–7 y 10–15](../../../arquitectura/CONTRATO_R2_0_PERSISTENCIA_CONTINUIDAD_Y_RECUPERACION_2026_08_25.md) y [LIG/0.1 §§2–4](../../../arquitectura/CONTRATO_MATERIAL_DE_LIGADURAS_DFL_005_2026_09_08.md). Reutiliza las sedes de operación, expectativa, autoridad, indeterminación y recuperación. No crea otra clave universal, almacén, primitiva temporal, clase de Frame ni diagnóstico SV. Las etiquetas de este expediente son documentales de ensayo.

Pilares, acta de perfiles/contratos/ensamblaje y transición secuencial fueron leídos completos en esta continuidad; se cotejan sus bytes contra el corte vigente, junto con las fuentes materiales aplicables. [FUENTES.json](FUENTES.json) fija la comprobación. R06 no altera los resultados anteriores ni decide constitución de dominio, cobertura del agente o tecnología productiva.

## 2. Tres preguntas que el receptor debe mantener separadas

1. **¿Qué material recibió?** Identidad y versión del encargo, referentes esperados, bytes u objetos efectivamente consumidos y alcance de las comprobaciones.
2. **¿Qué observó del recorrido?** Apertura, admisión, despacho, captura o terminación, sólo cuando existe observación de esa frontera. Una intención declarada de despacho no se convierte en despacho observado.
3. **¿Qué efecto está acreditado?** Resultado limitado a las evidencias y dependencias del contrato de efecto aplicable. Un informe ausente, un código de salida o una excepción no contestan esta pregunta por sí solos.

La igualdad de las dos lecturas R05 acredita esa concordancia puntual. No demuestra que el archivo permaneciera invariable entre ambas, que todas las dependencias fueran coherentes, ni que el objeto del sistema de archivos fuera el mismo. El campo histórico `entrega_previa` puede contener un rechazo sin recibo: debe leerse su resultado, no inferirse entrega por el nombre del campo.

## 3. Recepción de una terminación sin informe

### 3.1. Correlación constituida antes de ejecutar

El montaje que ofrezca esta recepción debe conservar previamente el encargo y la identificación del intento, vinculados a la operación y a su expectativa. Una nueva recepción, un nuevo intento y una nueva operación son relaciones diferentes. Reutilizar un identificador no prueba idempotencia; generar uno nuevo no concede permiso para repetir un efecto.

Se reutilizarán identificadores constituidos cuando satisfagan ese alcance. Si el ensayo necesita correlación instrumental adicional, deberá declarar emisor, ámbito, unicidad, tratamiento de duplicados y vida de esa correlación. No se resolverá por PID, hora, ruta, orden de llegada ni hash del valor como única identidad de la operación.

El observador declarará dónde conserva la apertura, qué puede perderse y qué fallo puede sobrevivir. Una marca sólo en la RAM que se destruye no acredita después una apertura recuperable. R06 no exige persistencia general para una sonda local; exige que su afirmación permanezca dentro de la cobertura efectivamente disponible.

### 3.2. Informe de recorrido y observación de terminación

El receptor conservará dos piezas distinguibles:

- El informe emitido por el recorrido, si lo recibió completo, dentro de cuota y correctamente ligado al encargo. Podrá contener resultado favorable, rechazo o falta de acreditación.
- La observación de terminación disponible desde la frontera declarada: retorno, pánico capturado por el arnés, terminación de proceso observada u otra causa expresamente cubierta. Si esa frontera no pudo observarse, también se conservará esa ausencia de evidencia.

No se requiere una estructura única para ambos. La realización deberá impedir que una pieza se presente como la otra. Un receptor que sobreviva al proceso observado podría documentar su terminación; compartir host no lo hace independiente frente al fallo de ese host. El protocolo y sus recursos se fijarán antes del ensayo de proceso, sin constituir aquí una plataforma de host del SV.

| Situación observada | Afirmación admisible | Afirmación que no queda acreditada |
|---|---|---|
| Retorno con informe completo, ligado y validado | Resultado y observaciones que ese informe acredita en su frontera. | Durabilidad, presentación en pantalla o efecto externo no observados. |
| Retorno sin informe, informe truncado, fuera de cuota o de otro intento | Falta de un informe aceptable; causa concreta cuando se conoce. | Éxito por código de salida cero, fracaso sin efecto o una entrega reconstruida. |
| Pánico observado; informe final ausente | Pánico en la frontera observada y ausencia de informe final recibido. | Cero intentos, rollback, ausencia de efectos o relectura final que no se ejecutó. |
| Captura parcial válida anterior y posterior interrupción | Hecho parcial en su alcance, junto con terminación incompleta. | Borrar lo ya observado o extenderlo hasta una confirmación no observada. |
| Canal cerrado o plazo técnico agotado sin observar terminación | Cierre de canal o agotamiento del plazo. | Que el proceso haya terminado o que no vaya a producir efectos posteriores. |
| Observador perdido con el proceso | Límite de cobertura del montaje. | Una conclusión favorable generada desde el silencio. |

La aceptación del informe exige la correlación y completitud definidas por el protocolo, además del resultado del recorrido. Recibir EOF no será la única prueba de completitud. Un informe tardío o duplicado se cotejará con su intento y las evidencias anteriores; un conflicto no se resolverá reemplazando silenciosamente la primera pieza.

### 3.3. Efecto incierto y continuación

Si falta evidencia suficiente para determinar el efecto, se conserva esa insuficiencia. No se fabrica `Tri.U`, D-A, un `Confirmed` ni un evento R1 que no ocurrió. Cuando aplique la cadena R1/R2, se conserva su régimen de indeterminación conforme a su contrato; la etiqueta del arnés no se introduce por sí sola en esa cadena.

La acción dependiente de confirmación no se habilita. La investigación, consulta u observación con alcance propio puede continuar si está autorizada. No se reintenta el efecto automáticamente para resolver la incertidumbre. Una reconciliación, compensación o reutilización necesita su regla constituida y evidencia del destino pertinente.

Capturar un pánico no revierte escrituras ni otros efectos. Si la lectura posterior se efectúa desde el arnés, se identifica como una observación nueva de ese arnés. Si encuentra alteración, conserva el hecho sin reescribir la entrega anterior. Si encuentra igualdad, no prueba ausencia de modificaciones intermedias.

## 4. Qué identidad de fuente exige este recorrido

### 4.1. Distinciones necesarias

| Objeto de la afirmación | Qué debe enlazarse | Qué no basta |
|---|---|---|
| Contenido bajo un perfil | Representación y comprobación exigidas por ese perfil; bytes literales cuando el contrato así lo pide. | Nombre de fichero, puntero, igualdad visual o huella autopropuesta como autoridad. |
| Ocurrencia o revisión SV | Referentes constituidos de operación, Frame/historia, instancia, posición, versión y dependencias aplicables. | Igual contenido, polígono o fecha. Dos ocurrencias distintas pueden tener el mismo contenido. |
| Objeto abierto del soporte | Identidad y alcance que el soporte permita observar; relación con el objeto realmente leído. | La cadena de texto de una ruta o un inode aislado tomado como identificador universal. |
| Procedencia y custodia | Fuente y expectativa autorizadas, con protección declarada frente al fallo ensayado. | Que propuesta y hashes sean coherentes entre sí. |
| Continuidad o autoridad para actuar | Dependencias vigentes del contrato aplicable, incluidas revocación y consumo cuando procedan. | Cualquiera de las igualdades anteriores por sí sola. |

### 4.2. Decisión acotada que se recibe

**Para el siguiente montaje local, la afirmación mínima exigible es la ligadura del encargo y de sus referentes autorizados con el contenido efectivamente consumido.** No se añadirá la obligación general de conservar un mismo inode: R05 P07 no demuestra que esa propiedad sea necesaria para toda operación.

Se distinguirán expresamente dos pretensiones posibles:

- Si el perfil sólo exige contenido y referentes autorizados, una sustitución de soporte que conserve esas obligaciones no es automáticamente una corrupción semántica. No habilita cambiar procedencia, ocurrencia, versión o autoridad.
- Si el perfil exige además continuidad del objeto de soporte o del productor, tendrá que constituir esa exigencia, el modo de comprobarla y su modelo de fallo. El comparador de bytes no ofrece esa capacidad y deberá declararla no acreditada cuando se le solicite.

No se escogerá el perfil más débil después de observar un fallo. El encargo fijará antes qué afirmación necesita. Un perfil superior no se degradará silenciosamente para producir respuesta.

### 4.3. Del objeto comprobado al objeto consumido

El candidato experimental deberá mostrar el vínculo entre los datos admitidos y los utilizados. Se podrá ensayar la recepción en buffers propios conservados para comprobación y consumo, reutilizando `ReceivedBytes` y la custodia existentes. Eso es una posibilidad de realización local, no una selección de almacenamiento persistente.

Conservar el descriptor abierto evita algunas sustituciones por reapertura, pero no demuestra inmutabilidad de su contenido frente a escrituras sobre ese mismo objeto. Abrir en lectura tampoco inmoviliza las escrituras de otros actores. Deben ensayarse separadamente sustitución de ruta, modificación del objeto y reemplazo de dependencia entre validación y consumo.

Si se reciben varias piezas, la copia propia de cada una no prueba que pertenezcan al mismo corte. Todas deberán satisfacer el conjunto de referentes fijado. Un campo opcional ausente no se recupera de otra versión ni se rellena con U. Los límites de recepción deben cubrir el conjunto y su salida según el perfil; cambiar de array a Vec o HashMap no resuelve estas obligaciones.

## 5. Discriminadores para la realización siguiente

Los siguientes ocho discriminadores son **especificaciones, no pruebas ejecutadas**. Antes de correr una campaña deben fijarse fixtures, código, protocolo, límites, puntos de inyección, observadores y oráculos literales. [DISCRIMINADORES.json](DISCRIMINADORES.json) conserva esta matriz.

| ID | Estímulo adversarial | Control y evidencia exigida |
|---|---|---|
| R06-T01 | Retorno sin informe, truncamiento o informe de otro intento. | Informe completo del intento correcto admitido; los tres defectos distinguidos sin éxito por código cero. Conservar bytes recibidos y frontera de terminación. |
| R06-T02 | Pánico antes del despacho y después de una observación parcial. | Retorno ordinario; barreras de inyección observadas; conservar hecho parcial y ausencia final. No inventar contadores ni rollback. |
| R06-T03 | Abort o terminación del proceso observado. | Proceso ordinario con informe íntegro; observador superviviente declara estado de salida y cobertura de canal. Cero acuses de efecto inferidos del estado del proceso. |
| R06-T04 | Informe tardío, duplicado concordante o duplicado contradictorio. | Correlación legítima; orden controlado; ninguna atribución a otro intento ni sobrescritura del conflicto. Tratamiento de duplicación fijado antes de ejecutar. |
| R06-T05 | Sustituir la ruta por archivo de contenido diferente tras validación. | Conjunto autorizado intacto; capturar qué bytes llegan al consumidor. Debe usarse el objeto admitido o rechazarse antes del uso protegido según la realización fijada. |
| R06-T06 | Sustituir soporte conservando bytes y referentes; solicitar luego una garantía de continuidad del soporte que no ofrece el montaje. | Contenido y referentes válidos admitidos bajo el alcance mínimo; pretensión superior explícitamente no acreditada. No cambiar el alcance tras observar el resultado. |
| R06-T07 | Modificar el mismo objeto entre lecturas, incluida restitución de bytes iniciales, o mezclar una dependencia de otro corte. | Copia consumida y referencias fijadas como control; observar punto de mutación y bytes usados. Dos lecturas iguales no deben acreditarse como preservación continua. |
| R06-T08 | Cerrar canal o agotar una cuota/plazo técnico sin conocer terminación ni efecto. | Recorrido completo dentro de límites; distinguir recepción, terminación y efecto. Ningún reintento automático ni ausencia de efecto inferida del silencio. |

T01/T02 y la ligadura de consumo de T05/T06 son el incremento local inicial. T03/T04/T08 requieren el protocolo del observador de proceso y límites del montaje; no se satisfacen con `catch_unwind` dentro del proceso que puede abortar. T07 requiere barreras de modificación y dependencias declaradas. El orden conserva la necesidad de fijar antes cada realización; no supone ejecutados todos los discriminadores por compartir una etiqueta.

## 6. Relevo, exclusiones y resultado documental

R06 recibe un contrato material de terminación y de alcance de identidad. Actualiza su sede antecesora R02 y la recepción de R2-0, además de Sucesos, RETP, Léame primero y el seguimiento transversal de S26. S27 conserva el reparto dominio/agente/Lenguaje; el Lenguaje no decide aquí qué conocimiento clínico debe recorrerse.

El siguiente incremento podrá preparar una copia experimental de recepción con correlación explícita, informe y observación de terminación separados y contenido admitido ligado al consumo. Deberá publicarse su precompromiso antes de ejecutar. Se reutilizarán las piezas disponibles; no se abrirá otro sistema de estados soberanos ni otro almacén.

**Cero nuevas pruebas de comportamiento en R06; cero casos globales S26 cerrados.** R01/R03/R04/R05 conservan sus campañas y alcances. No se acredita todavía una recepción ejecutable conforme a R06. Permanecen pendientes persistencia durable, recuperación, identidad de adquisición, resistencia al host y GUI; S22/Bis → catálogo/cierre → S24 mantiene su secuencia. Las obligaciones que requieran BD o perfil material R2/R3 seguirán en su sede.

## 7. Servicios, scripts y conservación

Se utilizan shell y Python para lectura, cotejo de fuentes, edición documental y registros; el conector GitHub existente para leer cortes, publicar y comparar árboles completos, sin force. No se invoca compilador ni otra IA en este incremento. `registrar.py` conserva el contenido de las adendas y las modificaciones administrativas; `publicar.py` declara el conjunto de archivos y el espejo. `VERIFICACION.json` registra conservación de antecedentes y comprobaciones documentales. Estas verificaciones no son ensayos del comportamiento propuesto. Los archivos privados adjuntos no se copian a esta publicación.

## Recepción posterior · LOCAL01 · RETP-236

[Trece casos por perfil en Rust](local01/RESULTADOS.md): primer incremento T01/T02/T05/T06 ejecutado después del precompromiso. Se conservan apertura y captura parcial ante pánico; el consumo usa el descriptor admitido pese a sustitución de ruta. T03/T04/T07/T08 e integración permanecen pendientes. La sección 7 anterior describe el incremento contractual histórico; LOCAL01 no ejecuta Python. R06 no es cierre de R0.

## S26 R06 PROCESO01 · Banco previo

[Protocolo y catorce casos](proceso01/README.md): observador de un proceso hijo, correlación previa, informes duplicados o tardíos y separación de canal, terminación y efecto. Banco sin ejecutar al incorporar este precompromiso. Seguimiento por Sucesos y expediente de Calidad con espejo de laboratorio; LOCAL01/RETP-236 conserva su alcance anterior. T07 e integración pendientes; S26/Bis y S24 mantienen su secuencia.

## S26 R06 PROCESO02 · Precompromiso tras impedimento instrumental

[Banco con tuberías anónimas](proceso02/README.md). PROCESO01 conserva el impedimento EPERM al crear el canal Unix, anterior al proceso hijo; cero casos completados. PROCESO02 fija catorce casos y sus recursos antes de ejecutar. Sucesos mantiene la revisión y los enlaces a Calidad y laboratorio; T07 e integración permanecen pendientes.

## S26 R06 PROCESO03 · Corrección precomprometida de transporte

[Banco con captura fragmentada](proceso03/README.md). PROCESO02 conserva su campaña incompleta por incompatibilidad entre tamaño de mensaje y cuota del decodificador. PROCESO03 alinea las cuotas sin modificar el núcleo y conserva el canal antes de la espera final. Catorce casos previstos, sin ejecución al publicar este banco. T07 e integración pendientes; seguimiento en Sucesos con enlaces a ambas sedes.

## S26 R06 PROCESO03 · Recepción del observador de proceso

[Resultados y evidencia](proceso03/RESULTADOS.md): catorce casos conformes en debug y catorce en release, con precompromiso intacto. Se conservan captura y escritura anteriores a la terminación del hijo; informes tardíos/contradictorios y estados de canal/proceso quedan separados. PROCESO01 impedido y PROCESO02 incompleto conservan sus resultados. Sucesos recibe la revisión 17 y enlaza Calidad y laboratorio. Siguiente: T07 con barreras de modificación, restitución y dependencias; integración pendiente. Sin cierre global de S26, Bis, R2 o S24.
