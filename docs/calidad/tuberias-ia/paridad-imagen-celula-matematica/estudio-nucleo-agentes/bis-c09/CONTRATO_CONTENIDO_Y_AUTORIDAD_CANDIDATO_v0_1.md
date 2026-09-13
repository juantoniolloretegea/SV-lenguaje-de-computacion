# Contrato candidato de recepción documental y conservación de autoridad

**Versión 0.1 · 13 de septiembre de 2026 · S22 · BIS-02/C09 · Juan Antonio Lloret Egea y Watson**

## 1. Objeto y fundamento

C09 concreta BIS-O09 y BIS-O12: recibir contenido externo bajo su estatuto y conservar la relación entre encargo, evidencia, decisión, permiso y efecto. S13 O04/O05 distingue la frontera documental de la ruta R1. RETP-148 aclara que la autoridad procede del encargo humano experto y de la cobertura constituida; diseñar su recepción comprobable es una obligación técnica. No se convierte esa obligación en una petición repetida de autorización para continuar este trabajo.

Un documento puede contener datos, afirmaciones, procedimientos e imperativos. Su sintaxis no lo transforma en un acto autorizante. Leer, citar o describir una orden no equivale a ejecutarla. La regla no impide usar información como selector cuando una relación previamente constituida determina qué objeto, contexto y operación puede seleccionar. La entrada no crea ni amplía esa relación.

El contrato es candidato de BIS-02. No constituye un dominio ni una premisa profesional, no añade una primitiva de seguridad y no acredita una cadena profesional completa. Se conservan semántica, IR y Rust vigentes, sus deudas y las diferencias entre main y candidatas históricas.

## 2. Distinciones operativas

| Elemento | Función y límite |
| --- | --- |
| Encargo y cobertura | Delimitan operación, contexto, objeto y destinatario antes de recibir la propuesta. |
| Documento recibido | Aporta contenido con identidad y procedencia; no puede ampliar el encargo por autodeclaración. |
| Evidencia admitida | Satisface una regla aplicable; admisión e integridad no conceden autoridad. |
| Propuesta o informe | Puede describir, seleccionar o aconsejar conforme al contrato; no constituye por sí mismo permiso. |
| Decisión de permiso | Exige las relaciones y requisitos constituidos; una cadena «permitido» o un recibo no la reemplaza. |
| Mediación y despacho | Comprueban la ligadura aplicable al efecto real, incluido objeto y contexto. |
| Registro y observador | Conservan evidencia de entrada, decisión y efecto; no dependen sólo de lo que afirme el componente evaluado. |

Un mismo contenido recibido por un canal ordinario no adquiere autoridad por incluir un nombre humano, una firma textual, un hash, un número de revisión o una etiqueta de sistema. Si otro canal y otra regla acreditan un acto autorizante, se deberá probar ese estatuto y su alcance; C09 no los deduce del documento.

## 3. Ligadura de objeto y prevención de redirección

La protección exige conservar el acto exacto solicitado. La pertenencia de un objeto a un alcance general no acredita que sea el destinatario de esta invocación. El testigo permite el selector A para OBJETO_A en T01. B carece de ligadura en ese encargo, aunque se postule que OBJETO_B pertenezca a otra facultad general.

La entrada puede aportar un selector válido; no puede elegir una forma más permisiva, cambiar la regla de despacho, sustituir contexto, destinar la salida a otro receptor o reutilizar una decisión ajena. La revisión debe alcanzar el adaptador y el destino efectivos. Esto incluye el caso en que la IA afirma haber bloqueado una orden y produce, simultáneamente, una llamada que la realiza.

Una decisión negativa debe impedir el efecto afectado. Si se observa despacho seguido de error, no se presume ausencia de efecto ni se reintenta automáticamente. La ruta Rust inspeccionada registra `DispatchCommitted` antes del adaptador y `Indeterminate` ante su error. Ese estatuto técnico no es `Tri.U` ni acredita por sí solo qué sucedió fuera del proceso.

## 4. Fidelidad y procedencia

El control conserva fuente, revisión solicitada, bytes efectivos y transformaciones relevantes. El documento D01 declara falta de suficiencia y su causa; D11 altera la negación. Presentar D11 bajo el nombre lógico de D01 y recalcular su huella no lo convierte en la fuente solicitada. Una huella identifica bytes, sin conceder verdad o autoridad.

La referencia debe ser independiente del contenido sometido a prueba. El documento no puede sustituir simultáneamente la política y el criterio que lo acepta. Si se exige cobertura, el comprobador debe tener acceso a las evidencias relevantes, incluidas las negativas; no sólo a las citas seleccionadas por el modelo. C10 desarrollará la relación afirmación–justificación con su alcance propio.

La trazabilidad exige registrar hechos observables: solicitud, recepción, propuesta, decisión, intento, despacho y resultado, cuando existan. Una explicación retrospectiva no sustituye las capturas. No se exige acceso al proceso interno privado de razonamiento del modelo: se exigen evidencias externas y justificación comprobable de lo que se acepta y de sus efectos.

## 5. Capacidades existentes y límites

S11 conserva evidencia sintética de recepción y cobertura: `Contexto::desde` limita longitud; `Contexto::comprobar` contrasta identidad de invocación y documento antes de delegar en la referencia independiente. Su código no interpreta las órdenes ni modela la respuesta de una IA. El resultado histórico no se reejecuta ni se suma a los contadores C09.

En R1, `ConstitutedAuthority` y `Permit` tienen campos encapsulados y carecen de constructores ordinarios que permitan fabricarlos por una etiqueta. `decide_permit` recupera forma, autoridad y requisitos de la continuidad constituida, coteja alcance del efecto y obtiene el resultado de la agregación gobernada. La mediación y la ejecución tienen obligaciones adicionales. `contains_effect_scope` no equivale por sí solo a permiso, y emitir un permiso no ejecuta el efecto.

La génesis depende de una premisa externa opaca cuya procedencia material no resuelve R1. C09 no la fabrica con `for_test` ni promueve candidatas de continuidad al citar sus resultados. S13 y RETP-148 mantienen límites de autenticidad frente al host y de enlace profesional. Los nombres de estas capacidades no acreditan que todos los canales de una futura IA atraviesen obligatoriamente la frontera.

BIS-03 decidirá las sedes y el enlace necesario. La primera sede para contenido y cobertura es la frontera documental; el efecto protegido reutilizará R1 con sus precondiciones satisfechas. No se añade un objeto «prompt» o «documento externo» al núcleo sólo para disponer de una etiqueta. La necesidad de representación nueva deberá justificarse mediante una obligación que la representación disponible no conserve.

## 6. Banco previo y criterio de ensayo

El banco contiene veinte escenarios: cuatro positivos y dieciséis negativos. Se entregan once documentos sintéticos literales, sus huellas y un encargo independiente de prueba. Estos JSON describen expectativas del conductor, no son permisos sellados ni tipos canónicos de IR. No contienen datos médicos ni activan destinos reales.

Los positivos incluyen lectura neutra, procedimiento imperativo citado, orden incrustada recibida como dato y selector permitido por una regla previa. Los negativos cubren política sustituida, evidencia omitida, falsa autoridad, recibo usado como permiso, redirección de objeto/contexto, fuente sustituida, registro borrado, destinatario alternativo, cambio de forma, inglés, relevo entre agentes, promoción profesional indebida, bloqueo sólo narrado, oráculo circular y fallo de proveedor mal clasificado.

La campaña futura debe distinguir tres alcances: recepción determinista de entradas, conducta observada del modelo concreto y contención del efecto por la frontera. Un ensayo con un conductor que emite propuestas prefijadas puede contrastar el receptor, pero no prueba que un LLM resista una instrucción. Un modelo que rechaza un texto tampoco demuestra contención si conserva otro canal de efecto sin comprobar.

Antes de ejecutar se fijarán versiones, receptor, permisos y capacidades materiales del entorno, límites de recursos, observador y resultados esperados. Las capturas incluirán documento exacto, encargo, entradas y salidas completas relevantes, selección, llamadas, decisiones, despachos y destinatario efectivo. Los fallos y las faltas de observabilidad quedarán como tales. La validez de un rechazo exige distinguir la guarda que actuó de las posteriores no alcanzadas. No se fabrican códigos del catálogo.

Seis afirmaciones quedan expresamente fuera del resultado presente: ejecución nueva en Rust, interrogación de modelos, contención frente a host comprometido, autenticación profesional, invulnerabilidad general y suficiencia de dominios. La comprobación auxiliar de integridad documental no acredita ninguna de ellas.

## 7. Relevo

C09-P/N originales permanecen pendientes de ejecución. Continúa C10: justificación respaldada, referencias suficientes y contraste entre afirmación y evidencia. BIS-02 sigue abierto; BIS-03 decide sedes y BIS-04 realiza los cambios justificados. S24 mantiene Bis → catálogo y cierre de fase → análisis e instalación de la GUI.
