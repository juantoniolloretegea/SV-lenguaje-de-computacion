# Recibo custodiado de IE-004 — G1/1

**RETP-2026-143 · 11/09/2026 · contrato fijado para contraste nativo en Rust.** Responsable: Watson. Mandato: continuación autorizada por Juan Antonio Lloret Egea de G1 en RETP-142. **Realización y conformidad del receptor: pendientes.**

## 1. Objeto, corte y alcance

Objetivo verificable: que el receptor pueda recuperar y cotejar la petición exacta, el montaje aplicado, el cuerpo y la traza de una misma invocación A; después, vincular la propuesta observada y el resultado de V sin modificar el recibo A ni esperar a V para cerrarlo.

Corte del Lenguaje: `282e020f8bff2b68abd9a2e9138fa6b467debaf8`. Laboratorio: `c074b1b0796081501ea0c0c1ac2be206dcf49999`. Se conserva el [workflow V2](../../WORKFLOW_ACOTADO_SUBORDINACION_IA_ES_V2_2026_09_11.md) y se desarrolla exclusivamente G1 del [expediente RETP-142](../../EXPEDIENTE_CORRESPONDENCIA_TRAZABILIDAD_IE004_2026_09_11.md).

AGENTS, Pilares, perfiles, transición y arquitectura conservan los mismos blobs que los leídos en RETP-141/142. Se han cotejado nuevamente en este corte. Se ha releído el workflow y el contrato A/V junto con sus funciones de recepción, entrega y anexado. La lectura de actas privadas documentada en RETP-141 mantiene su alcance y custodia.

La sede prevista es el receptor custodio nativo del banco, fuera de `Frame`. Este contrato no declara una nueva API externa, bus, dominio, agente o forma de ejecución soberana. Los nombres de estados de recibo que siguen pertenecen a G1; no se añaden a Tri, al catálogo de diagnósticos del Lenguaje o al cuerpo canónico A.

## 2. Tres objetos distintos

| Objeto | Qué significa | Qué no acredita |
| --- | --- | --- |
| Intento de recepción | El custodio abrió una invocación y conserva lo observado, incluso fragmentos y fallos | Que A haya entregado un resultado completo |
| Recibo A completo | Las evidencias obligatorias de A están completas, vinculadas y recuperadas por el custodio bajo las comprobaciones de §5 | Que la resolución sea DATO, que la IA sea segura o que exista un SUCESO de dominio |
| Anexo de observación V | Una observación posterior de V, su propuesta cuando sea recuperable y su desenlace, vinculados a un recibo A concreto | Autoridad para sustituir A o para convertir fallo técnico en U |

Una petición rechazada o un presupuesto A agotado pueden tener **recibo completo** si cuerpo y traza completos describen honestamente ese resultado. Si el proceso termina sin cuerpo o traza completos, queda **intento incompleto**, aunque se conozca la causa técnica. El fallo funcional y la pérdida de evidencia son ejes separados.

## 3. Identidad y pertenencia

El custodio asigna la pareja `(ambito_custodia, ordinal_invocacion)` antes de entregar bytes a A. El ámbito es una referencia exacta emitida por la continuidad responsable; el ordinal es un entero comprobado sin reutilización dentro de ese ámbito. No procede del participante, del identificador público del caso, del hash del cuerpo, del reloj ni del PID.

El alcance de esta versión es una continuidad viva intra-proceso. La creación, conservación y recuperación durable de ámbitos únicos corresponde a G4; esta versión no acredita unicidad entre reinicios. Al agotarse la representación del ordinal se rechaza la nueva admisión, sin vuelta a cero. No se incorpora este ordinal a `Frame.index`.

La apertura entrega al receptor un manejador interno no construible por el participante. El mismo manejador gobierna entrada, canales de salida y finalización. Si se ensaya con procesos, cada pipe pertenece a esa apertura; si se ensaya con llamadas nativas, pertenece al producto devuelto por esa llamada. Un importador de archivos históricos sólo acredita custodia documental, no prueba por sí solo que esos bytes salieran de esa invocación.

**Límite adversarial:** dos invocaciones distintas pueden producir cuerpos y trazas idénticos. Ningún hash distingue dos copias idénticas. Detectar su cruce exige pertenencia conservada por el receptor; añadir a posteriori una etiqueta a un archivo no prueba su origen. Un host capaz de falsificar el custodio queda fuera de la garantía G1/1 y pendiente de imposición material.

## 4. Evidencia obligatoria y representación

Cada artefacto se describe por papel, longitud observada, SHA-256, estado de completitud y referencia de recuperación. Sólo una referencia que permite recuperar y cotejar todos los bytes en la custodia declarada sirve para un recibo completo. La forma nativa podrá usar referencias tipadas; no se exige introducir una serialización nueva en el núcleo.

| Pieza | Contenido obligatorio y origen |
| --- | --- |
| Apertura | Identidad de invocación; versión G1/1; identidad del receptor y realización A; parámetros de ejecución y límites efectivos; condición de recuperación declarada |
| Entrada | Bytes exactos suministrados a A, incluida la sintaxis JSON y sus espacios; identificador, pregunta y contexto decodificados cuando la admisión lo permita; sin reconstruir el original a partir de esos campos |
| Montaje efectivo | Versión `M-IE004-AV/1`, perfil, base, política, fijación de fuente, vigencia seleccionada y opciones efectivas. Lo fija el conductor confiable antes de A; la propuesta no lo rellena |
| Realización | Corte/fuentes y artefacto ejecutado identificados por referencia y huella, opciones de compilación y ejecución. Identidad del binario no equivale a identidad del cuerpo entre plataformas |
| Producto A | Marco binario completo `SVAC0001`, cuerpo canónico recuperado, traza original completa y resultado del proceso o llamada |
| Cierre A | Resultado de cada comprobación obligatoria, causa de rechazo del recibo si corresponde y referencias verificadas; cierre inmutable |
| Apertura V, si procede | Identidad del recibo A y su huella; copia exacta de solicitud confiable; lote/compromiso de solicitudes aplicable; cupo V; pertenencia al manejador V emitido por el custodio |
| Observación V | Bytes de propuesta observados y su completitud; anexo `IE004-V-ANEXO/1` si existe; estado del proceso, ausencia, interrupción o exceso; causa y referencias recuperables |

La traza del modelo, si alguna vez se entrega, será un artefacto atribuido al modelo, separado de la traza del receptor. No se usará como demostración de su pensamiento interno. G1 conserva las transformaciones exportadas por A y sus fuentes, sin prometer recuperar cada microestado no exportado.

Para el caso público de esta versión el montaje es el fijado por RETP-137 y el testigo A01. No se reconstruye el montaje reservado P3 ni se decide su asociación /2–/3.

## 5. Condiciones exactas de cierre A

El constructor nativo de recibo completo deberá ser privado al receptor y exigir conjuntamente:

1. Apertura viva, identidad no finalizada y pertenencia de todos los canales al manejador de esa apertura.
2. Entrada observada completa dentro de su límite; montaje y realización fijados antes de la ejecución. Si la captura se corta por exceso sólo se conserva el prefijo y el indicador de exceso: nunca se presenta como original completo.
3. Terminación satisfactoria de la llamada o proceso de entrega A, según su contrato. Un texto que afirme éxito no sustituye ese resultado observado.
4. Marco completo, sin cola, aceptado por las comprobaciones existentes de longitud, SHA-256, JSON y versión. Se conserva el cuerpo literalmente; no se reserializa para ocultar diferencias.
5. Traza completa y de versión admitida. Decodificación estricta: duplicados, cola y truncación rechazan la traza. El máximo y la estructura de la traza no se confunden con los del JSON de entrada A.
6. `original_transporte_sha256` igual a la huella de la entrada exacta; `cuerpo_sha256` igual a la del cuerpo extraído. Si la solicitud se admitió, `id` y `original` coinciden con sus cadenas decodificadas; contexto, versiones y vigencia del cuerpo coinciden con el montaje efectivo y la entrada admitida.
7. Cuerpo y traza concuerdan en su resolución cuando ésta figura en ambos. La variante de rechazo de admisión conserva `iniciado_a:false`, su error y las huellas: no se le inventan campos de la variante admitida. Un `analisis_completo:false` no se cambia por éxito.
8. Recuperación y cotejo de los artefactos obligatorios desde la custodia declarada antes de afirmar recibo completo. Un archivo perdido o una referencia que apunta a otra cosa impide ese cierre.
9. Una única finalización de la identidad. Un reintento exige nueva invocación vinculada al intento previo; no reescribe ni reclasifica el cierre anterior.

El cierre acredita la recuperación comprobada en ese momento y bajo la retención declarada. No promete disponibilidad perpetua. Si una recuperación posterior falla, se conserva el cierre histórico y se registra la pérdida presente cuando sea posible; no se borra el fallo ni se sigue afirmando que el artefacto está disponible.

El orden de comprobación de G1 es identidad/pertenencia → límites/completitud → estructura → correlaciones → recuperación → cierre. El informe conserva las comprobaciones realizadas y la primera barrera incumplida; no afirma haber evaluado campos que no pudo decodificar. Este orden no sustituye el orden semántico de A ni elimina las causas concurrentes ya presentes en su traza.

En el alcance nativo inicial la traza debe entregarse por un canal dedicado a sus bytes. Para importar la evidencia histórica WASI se conservan aparte los avisos observados y su partición exacta, como en RETP-142. Extraer arbitrariamente una subcadena entre llaves no es un método de recepción. La adaptación WASI productiva conserva su contraste pendiente.

## 6. Anexado V y ausencia

El recibo A se cierra sin depender de la llegada, éxito o terminación de V. Puede notificarse el cuerpo ya entregado conforme a A/V; esa notificación no se etiqueta como recibo completo hasta cumplir §5. Si falla la custodia después de entregar el cuerpo, se declara esa pérdida de evidencia sin reescribir el cuerpo ni fingir un recibo.

Un anexo V sólo puede vincularse a un recibo A completo y a su manejador. Se exige correlación de la copia confiable de solicitud, identidad de caso cuando proceda, hash de cuerpo y `solicitudes_sha256`; las dos últimas huellas no bastan para distinguir invocaciones por sí solas. La pertenencia procede del custodio. El resultado V conserva `autoridad_sobre_cuerpo_a:false`.

Se distinguen:

- **Anexo recuperable:** propuesta completa y anexo completos, comprobados y recuperables. Incluye `V_RECHAZADA` cuando el comprobador rechazó una propuesta completa, incluso si ésta contiene JSON truncado: transporte completo de bytes malformados no equivale a captura incompleta.
- **Observación V incompleta:** interrupción, límite, fallo de recuperación o pérdida de anexo. Se conservan causa, prefijos y cantidades realmente observadas; no se afirma una propuesta completa ni un anexo conforme.
- **V no solicitada:** decisión explícita del conductor dentro de este ensayo. El silencio, por sí solo, no prueba ausencia definitiva. Si V estaba prevista y se agotó el plazo, se registra interrupción por plazo.

Sólo se admite una observación terminal V por apertura V en este alcance. Repeticiones y nuevas rondas requieren nueva identidad y el presupuesto autorizado que corresponda. Duplicar un anexo no produce otro ejercicio ni modifica A. El orden lo da la continuidad de operaciones, no la hora.

## 7. Recursos y fallo del propio custodio

Se heredan los máximos A/V: entrada A 65.536 bytes; cuerpo 4.096; marco 4.144; traza 16.842.752; propuesta V 262.144. El byte adicional para detectar exceso no se acumula sin límite. El máximo de traza es independiente del límite de 8.192 valores del decodificador de entrada: reutilizar ese límite podría rechazar trazas permitidas.

Antes de admitir una invocación, la realización candidata deberá fijar y registrar su perfil de custodia: cupos propios de buffers agregados, metadatos, lectura/validación, almacenamiento, concurrencia y plazo de retención. Las reservas serán fallibles y los crecimientos comprobados; las cuentas del custodio no se ocultarán en A o V. Ese perfil y su contraste de fronteras son requisito previo a ejecutar la campaña G1; **este contrato no acredita todavía su coste ni impone una cifra nueva al motor**.

Se acota inicialmente a una invocación A y como máximo una observación terminal V por unidad de recibo. Esto limita la unidad documental, no establece un servidor ni una política global de concurrencia. Falta de capacidad para custodiar exige rechazo técnico de apertura o intento incompleto, según el momento del fallo. No se promete registrar indefinidamente anomalías cuando se agota el almacenamiento.

Si falla el propio medio de registro, el componente superior debe recibir el fallo de custodia cuando el canal exista. Si también se pierde ese canal, no hay evidencia de cierre y el alcance permanece no acreditado. G1 no soluciona por definición una pérdida total de observación. La recuperación tras reinicio y la resistencia a un custodio comprometido siguen en sus puertas G4/P4.

## 8. Testigos fijados antes de implementar

[TESTIGOS_G1.json](TESTIGOS_G1.json) identifica el estado previo, la entrada material o acción, la distinción y el esperado. [CAPSULA_TESTIGOS_G1.json](CAPSULA_TESTIGOS_G1.json) conserva los bytes públicos y sus mutaciones controladas. **Son testigos preparados, no pruebas pasadas por un receptor G1.**

| Testigo | Situación discriminante | Esperado del futuro receptor |
| --- | --- | --- |
| G1-01 | A01 íntegra, pertenencia propia y recuperación completa | Recibo A completo |
| G1-02 | Mismo cuerpo; traza ausente | Intento incompleto |
| G1-03 | Traza truncada | Intento incompleto |
| G1-04 | Entrada modificada sólo con espacio JSON y traza antigua | Rechazo por correlación; no normalizar el original |
| G1-05 | Traza sintácticamente válida con resolución alterada, cuerpo intacto | Rechazo por discordancia |
| G1-06 | Marco completo y cuerpo alterado sin renovar hash | Rechazo por integridad |
| G1-07 | Huellas correctas pero recuperación de traza falla | Intento incompleto |
| G1-08 | Segunda finalización de la misma apertura | Rechazo de duplicado; cierre anterior intacto |
| G1-09 | Bytes idénticos, manejador de otra invocación | Rechazo por pertenencia; los bytes no son testigo suficiente |
| G1-10 | Tras cerrar A, propuesta y anexo V02 válidos y propios | Anexo recuperable; A inalterado |
| G1-11 | Propuesta V04 malformada recibida entera y anexo de rechazo íntegro | Anexo recuperable de rechazo, no pérdida de captura |
| G1-12 | V prevista sin EOF, interrupción por plazo | Observación V incompleta; A inalterado |
| G1-13 | Anexo con otra huella de cuerpo | Rechazo de correlación; A inalterado |
| G1-14 | Segundo anexado terminal con la misma apertura V | Rechazo de duplicado; historia intacta |
| G1-15 | Intentar anexar antes del recibo A completo | Rechazo por estado; sin forzar éxito A |
| G1-16 | V no solicitada explícitamente | Registro de esa decisión; A completo no depende de V |
| G1-17 | Montaje efectivo revocado y cuerpo de montaje activo | Rechazo de montaje; no corregir A silenciosamente |
| G1-18 | Captura cortada por límite, sólo prefijo disponible | Intento/observación incompletos; sin huella fingida del original entero |

Las inyecciones de pérdida de recuperación, pertenencia, estado y plazo necesitan un arnés receptor. El JSON conserva su preparación lógica, pero no sustituye esa inyección. Las fronteras del perfil de recursos se incorporarán a la misma campaña cuando se fije ese perfil, antes de ejecutar; no se ajustarán a los resultados.

La campaña deberá añadir, antes de su ejecución y con productos A públicos fijados, las variantes positivas de G1-01 para rechazo de admisión y fallo técnico A con cuerpo/traza completos. Son necesarias para verificar la separación del §2; el testigo A01 por sí solo sólo cubre su variante DATO. Esos productos aún no forman parte de la cápsula preparada y no se cuentan entre sus 18 casos.

## 9. Revisión adversarial y condición de siguiente paso

Esta versión debe resistir cuatro errores de diseño: confundir integridad con origen; exigir éxito semántico para admitir evidencia; permitir que V retenga el cierre A; y declarar recuperación por tener sólo hashes. Los testigos distinguen esos casos y conservan el original antes de cada mutación.

Sigue existiendo un límite: un custodio malicioso puede fabricar recibos plausibles. Por tanto, G1/1 es un contrato de recepción bajo un custodio confiable declarado, no una prueba independiente contra ese custodio. También puede faltar parte del análisis interno de A: el recibo completa la evidencia exigida por A/V, no una historia inexistente de todos sus pasos.

**Siguiente objeto único:** realización candidata nativa Rust del receptor G1/1 y de su perfil de recursos, seguida del contraste con estos testigos y las fronteras fijadas de antemano. Si se detecta contradicción contractual, se conserva el fallo y se sucede expresamente la versión; no se modifica el esperado para acomodarlo al código.

G2 (operación gobernada), G3 (inscripción/Frame exigible) y G4 (recuperación material) conservan su sede y condición en RETP-142. No se abre P4 ni P3 reservada. Cerrar esta especificación no declara terminada la segunda necesidad de trazabilidad.

## 10. Fuentes materiales

- [Contrato A/V existente](../recepcion-av/CONTRATO_RECEPCION_AV_1.md), [recepción y traza](../recepcion-av/recepcion_a.rs), [entrega](../recepcion-av/main.rs), [comprobador y anexo V](../recepcion-av/verificar_v.rs).
- [Evidencia pública cotejada RETP-142](../trazabilidad-142/EVIDENCIA_RECUPERABLE.json) y [resultado con incidencias preservadas](../trazabilidad-142/RESULTADO_CONTRASTE.json).
- [Constructor de los testigos](preparar-testigos.mjs) y [resultado de preparación](PREPARACION_TESTIGOS.json). Su ejecución sólo coteja procedencia, bytes y mutaciones; no evalúa un receptor G1.

Para reproducir la preparación: descargar la cápsula RETP-142 del corte indicado y ejecutar `node preparar-testigos.mjs /ruta/EVIDENCIA_RECUPERABLE.json /ruta/salida-nueva`. El constructor exige su SHA-256 exacto, coteja sus 43 archivos y genera 14 artefactos y 18 casos. No sobrescribe un directorio previo. El hash del propio constructor queda en el resultado. Las identidades, manejadores y estados previos del JSON describen inyecciones del futuro arnés; no son recibos emitidos ni una nueva gramática SV.

Estado: **CONTRATO_G1_FIJADO_TESTIGOS_PREPARADOS_REALIZACION_PENDIENTE**.
