# Interlocución del experto, idiomas y consejo verificable en el SV

**Fecha:** 20 de septiembre de 2026.  
**Versión documental:** 0.1.  
**Seguimiento:** [S40, pendiente](../../Inventario-sv/sucesos/SUCESOS_SV.md#s40). **Calidad:** RETP-2026-260.  
**Naturaleza:** revisión explicativa de fuentes y delimitación de trabajo futuro. No constituye un contrato ejecutable ni acredita una interfaz profesional disponible.  
**Responsable de la revisión:** Watson / W-S37, bajo instrucción de Juan Antonio Lloret Egea.  
**Corte canónico leído:** Lenguaje `23fcf1fbe5d7bd997d2e993006d8780d374475e3`.  
**Decisión recibida:** conservar esta necesidad en un documento y un suceso pendiente, sin abrir su implementación ni desviar el ensayo actual.

## 1. Objeto y conclusión de la revisión

El experto necesita comunicarse en lenguaje natural con un agente de cobertura explícita, recibir consejo fundado en conocimiento autorizado y examinar el recorrido que lo sustenta. La libertad expresiva del interlocutor no debe alterar el significado de la solicitud admitida, la identidad de los parámetros, las rutas obligatorias ni el resultado que se entrega.

Esta necesidad ya tiene fundamento y antecedentes materiales en el SV. La presente pieza los reúne y delimita lo pendiente; no reinicia ES27, IE-004 ni la constitución de los dominios.

Se distinguen tres afirmaciones:

1. **Requisito documentado:** conservar significado, autoridad, cobertura, resultado y trazabilidad bajo las condiciones del contrato.
2. **Evidencia acotada existente:** realizaciones de consulta española, resolución semántica y recepción A/V en bancos identificados.
3. **Capacidad pendiente de acreditación:** interlocución profesional integrada, con el modelo y soporte elegidos, conocimiento del agente, permisos efectivos, presentación fiel y custodia suficiente.

Una conversación fluida, un JSON válido, una compilación correcta o un resultado repetido no demuestran por separado esa capacidad.

## 2. Fuentes y prelación

Se han leído AGENTS y las tres piezas rectoras completas en el corte indicado. Se han consultado las actas lingüísticas, el acta aprobada de rutas, las entradas RETP pertinentes y el seguimiento S28/S37–S39. Los antecedentes experimentales se reciben mediante sus actas y registros; no se han reejecutado ni se han recalculado todos sus manifiestos.

| Ref. | Fuente leída o antecedente expresamente identificado | Función y límite |
|---|---|---|
| F01 | [Pilares, 05/09](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/23fcf1fbe5d7bd997d2e993006d8780d374475e3/docs/calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md), §§1–8, 12 y 14 | Invariantes, competencias, prohibición de reparación silenciosa y de inferencia opaca en la cadena soberana. Sus estados históricos conservan fecha. |
| F02 | [Perfiles, contratos y ensamblaje, 06/09](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/23fcf1fbe5d7bd997d2e993006d8780d374475e3/docs/calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md), §§3–9 y 12–15 | Distingue perfil fuente, dominio, agente y soporte; exige suficiencia por operación y prueba del conjunto. |
| F03 | [Transición secuencial desde OP-IMM-001](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/23fcf1fbe5d7bd997d2e993006d8780d374475e3/docs/dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md), §§12–30 | Dependencias, retornos y puertas de integración; no autoriza adelantar agentes ni alterar dominios. |
| F04 | [Fase 003, 10/09](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/23fcf1fbe5d7bd997d2e993006d8780d374475e3/docs/calidad/tuberias-ia/FASE_003_NLP_ES_CONOCIMIENTO_DETERMINISTA.md) | Consulta española controlada y emisión exacta; especificación candidata, con sucesores materiales que deben leerse aparte. |
| F05 | [Encaje de perfiles lingüísticos, 10/09](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/23fcf1fbe5d7bd997d2e993006d8780d374475e3/docs/calidad/tuberias-ia/ACTA_ENCAJE_DE_PERFILES_LINGUISTICOS_Y_SEPARACION_INVESTIGACION_APLICACION_2026_09_10.md), §§2–6 | Separa idiomas de código, interfaz e interlocución; fija repetibilidad, versiones, transformaciones y límites de extensión. |
| F06 | [Acta aprobada de rutas, 14/09](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/23fcf1fbe5d7bd997d2e993006d8780d374475e3/docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/ACTA_EVALUACION_Y_RECEPCION_DOCUMENTAL_RUTAS_CONOCIMIENTO_SV_2026_09_14.md), §§3.1, 4 y 5 | Consejo probabilístico auxiliar, parámetros singulares de decisión, cobertura, presentación, auditoría y separación de modos. Su reserva de incorporación §3 permanece en su sede. |
| F07 | [RETP, corte revisado](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/23fcf1fbe5d7bd997d2e993006d8780d374475e3/docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md), 115–142 | Sucesión ES27/IE-004: especificaciones, fallos, correcciones, evidencia acotada y límites. No se usa el estado de la Fase 003 como descripción de toda la sucesión. |
| F08 | [Acta 003 / S39](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/23fcf1fbe5d7bd997d2e993006d8780d374475e3/docs/calidad/tuberias-ia/continuacion-15-09-2026/ACTA_003_CONCILIACION_EIO_SUCESOS_Y_CALIDAD_2026_09_20.md) | Síntesis EIO en sus cortes; resultados tecnológicos parciales y preparación nativa. No acredita aptitud profesional. |
| F09 | [Acta de nacimiento del banco de idiomas](https://github.com/juantoniolloretegea/SV-banco-de-idiomas/blob/1ed5dcc1914e42072cee665ccd29847cff83e86c/registros/ACTA_TECNICA_NACIMIENTO_BANCO_IDIOMAS_SV_2026_03_30.md) y [piloto español](https://github.com/juantoniolloretegea/SV-banco-de-idiomas/blob/1ed5dcc1914e42072cee665ccd29847cff83e86c/idiomas/espanol/README.md) | Infraestructura lingüística auxiliar y casos tipados; no corpus de entrenamiento ni autoridad semántica. |
| F10 | [Qwen Team: Qwen3, 29/04/2025](https://qwenlm.github.io/blog/qwen3/) | Fuente del fabricante para capacidades multilingües y separación entre pesos y aplicación Qwen Chat; no evidencia de conformidad SV. Consulta: 20/09/2026. |
| F11 | [Sucesos, corte revisado](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/23fcf1fbe5d7bd997d2e993006d8780d374475e3/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md) | S28 conserva la recepción documental; S37/S38 son estudios pendientes distintos; S39 identifica el ensayo EIO en seguimiento. |

F06 incorpora la recepción documental de *Fundamentos, exigencias y arquitectura general de los agentes especializados* y de *Aprendizaje trazable*. Esta revisión utiliza esa recepción y sus límites; no declara una nueva lectura íntegra de los originales ni una auditoría exhaustiva de toda la literatura SV. No se publican originales privados aportados a la conversación.

## 3. Lo ya ensayado y lo que no se puede atribuirle

| Antecedente | Evidencia consignada en Calidad | Límite para este objeto |
|---|---|---|
| ES27, RETP-116–118 | Consulta de registros artificiales en español, correcciones tras formulaciones de Grok y paridad nativo/WASI. ES27/3 registra 110 controles y repetición de positivos. | No es comprensión universal ni conocimiento profesional; los rechazos pertinentes originales y sus reparaciones se conservan. |
| IE-004, RETP-119–124 | Se documentó que tipo, permiso y cita correctos podían acompañar una selección equivocada de referente. Las capturas conservaron errores y límites de independencia. | Un validador estructural no basta para acreditar correspondencia semántica; no se atribuye este fallo histórico como inalterado a todos los sucesores. |
| Resolución A, RETP-135 | 61 controles semánticos y 72 sintácticos por configuración; paridad y conservación de ambigüedad antes de política en el banco. | Evidencia finita de un perfil explícito; límites de texto y coste propios, sin comprensión general ni latencia de servicio acreditada. |
| Recepción A/V, RETP-137 | Conservación del cuerpo A ante ausencia, invalidez, exceso o demora de V, en escenarios fijados. | La entrega de A no demuestra utilidad de la IA; tampoco acredita por sí sola aislamiento o integración soberana. A/V son canales de aquel expediente, no las vías navegador/nativa del EIO. |
| Correspondencia, RETP-142 | Localización separada de receptor, cadena protegida y Frame; carencias G1–G4 identificadas en ese corte. | Su mera coexistencia no demostraba integración. Sus desarrollos posteriores conservan expedientes propios; aquí no se reabre ni se reclasifica cada cierre. |
| EIO / S39 | Controles directos, complemento JSON y controles de navegador; inferencias nativas rechazadas e inferencia de navegador interrumpida en los informes recibidos. | No acredita la interlocución profesional completa. El modelo pequeño es una referencia técnica; no constituye un agente médico o de ciberseguridad. |

Esta lectura corrige dos simplificaciones: no es cierto que todo esté sólo especificado, y tampoco que los bancos ya acrediten el producto final. Antes de abrir S40 habrá que recibir las versiones y sucesores vigentes, no reconstruir el trabajo únicamente desde esta síntesis.

## 4. Tres planos lingüísticos y la interfaz del experto

### 4.1. Idiomas

Deben conservarse separados:

- **Perfil fuente SVP-ES/SVP-EN:** formas constitutivas del programa; su convergencia no prueba comprensión de preguntas humanas.
- **Idioma de interfaz:** rótulos, ayuda y presentación.
- **Perfil de interlocución:** formas admitidas, referencias, contexto, negación, unidades, equivalencias y reglas de aclaración.

Qwen3 declara soporte multilingüe, incluidos español e inglés [F10]. Eso no acredita que la variante cuantizada, el tokenizador y el adaptador del ensayo resuelvan correctamente consultas especializadas. El banco SV aporta casos y contratos auxiliares; no instala idiomas en los pesos [F09].

El arranque español está documentado. En las piezas del banco leídas no se acredita una segunda fase inglesa materializada. Una extensión requerirá versiones y equivalencias explícitas. La detección automática sólo podrá proponer un idioma: el contrato debe fijar su selección o confirmación, el tratamiento de entradas mixtas y la posibilidad de corregirla. No se admite una sustitución tácita de perfil [F05].

### 4.2. Interfaz propia y medios de ejecución

La pantalla comercial de Qwen Chat no forma parte de los pesos del modelo ni define la futura interfaz SV. La interfaz propia deberá mostrar, en su alcance admitido:

1. identidad, cobertura y modo de trabajo del agente;
2. solicitud original, contexto pertinente e interpretación admitida;
3. aclaraciones necesarias sin selección silenciosa;
4. consejo, fuentes, condiciones, límites y estado de la operación;
5. acceso a evidencia, parada y resultado de esa parada cuando proceda.

La vía navegador/WASM y la vía de servicio nativo son soportes alternativos del ensayo, no contratos profesionales distintos. Una página no necesita acceso general a una terminal, al escritorio o a archivos ajenos a la operación. La ausencia de un botón no demuestra que esa facultad esté impedida: hay que comprobar los permisos materiales.

El diseño visual detallado y su implementación quedan pendientes. Esta pieza no selecciona framework, hosting, proveedor ni nueva infraestructura.

## 5. Universo del agente y conservación del objeto SV

El dominio constituye conocimiento, células, parámetros y relaciones. El agente recibe una cobertura explícita y permisos; el Lenguaje valida y preserva dentro de su alcance. **El universo accesible al agente no es todo lo que el modelo pudiera recordar de su entrenamiento** [F01, F05, F06].

La selección de información debe respetar las rutas obligatorias, condiciones, ramas conjuntas, vetos y dependencias constituidas. No exige cargar o enumerar todo el universo en cada consulta. Exige demostrar que el recorrido satisface la operación; una lista de lecturas o una selección por similitud no prueba completitud.

Se conserva la denominación **parámetros singulares de decisión** [F06 §4.3]. Cada valor debe mantener la ligadura entre constitución, célula, posición, parámetro y evidencia. Para una célula exacta rigen b≥3 y n=b², con vector plano y ordenado. SV(9,3) es el mínimo, no el único tamaño; un inventario de 27 parámetros no constituye una célula ni autoriza relleno.

El polígono debe representar el mismo estado exacto. Ordenar claves de un objeto JSON no autoriza a ordenar el vector; huella, JSON válido y forma gráfica correcta no demuestran correspondencia semántica. La representación deberá preservar además la información necesaria para justificar la operación: dos valores ternarios iguales pueden proceder de condiciones diferentes. No se reconstruye una distinción perdida mediante prosa del modelo.

## 6. Determinismo, equivalencia y presentación

### 6.1. Condición verificable

Para la consulta admitida, los datos y contexto relevantes, cobertura, autorización vigente y versiones de conocimiento, reglas, perfil lingüístico, algoritmos y emisor fijados, se exige el mismo resultado contractual. La Fase 003 precisa identidad literal del cuerpo bajo sus condiciones; identificadores técnicos, tiempos y medidas de ejecución no quedan incluidos automáticamente en esa igualdad [F04].

En una presentación gobernada y versionada podrá comprobarse identidad de bytes. Entre presentaciones españolas e inglesas se exigirá correspondencia del contenido, condiciones y procedencia conforme al contrato; no igualdad literal de dos idiomas [F05].

Cambiar conocimiento, observaciones, permisos o contexto relevante puede cambiar legítimamente el resultado. Reproducir una respuesta antigua no autoriza a volver a entregarla tras una revocación.

### 6.2. Paráfrasis, erratas y ambigüedad

La equivalencia de dos solicitudes se debe constituir y comprobar. Un modelo no puede declararla por autoridad propia. Una negación, una unidad, una referencia temporal o un dígito pueden modificar la operación.

El recorrido debe conservar el original y las transformaciones autorizadas. Si hay varias interpretaciones pertinentes, falta contexto o la forma no está cubierta, se comunica la causa y se solicita aclaración cuando el contrato lo permita. La autorización no debe servir para eliminar interpretaciones rivales hasta obtener artificialmente una sola [F05; RETP-125 y 135].

Una propuesta de reformulación del auxiliar se presenta como propuesta; no sustituye silenciosamente la pregunta. La confirmación humana queda ligada a una versión concreta y no reemplaza las comprobaciones de estructura, cobertura o permisos. Un fallo lingüístico, técnico o de admisión no produce automáticamente Tri.U.

### 6.3. Papel del modelo y precisión sobre la respuesta anterior

La Fase 003 excluye que la prosa libre del modelo determine o se añada al cuerpo aceptado. F05 mantiene que la salida profesional no se reescribe libremente. F06 admite interlocución, organización y exposición probabilísticas auxiliares, conservando la prohibición de sustituir o corregir silenciosamente el resultado.

Estas fuentes no acreditan un permiso general para añadir texto generativo al consejo profesional. La sugerencia conversacional de una explicación libre suplementaria debe entenderse como posibilidad sometida a contrato y verificación, **no como capacidad ya autorizada**. Cualquier futura presentación auxiliar deberá fijar su estatuto, contenido admisible, relación con el resultado, oráculo de fidelidad y conducta ante contradicción. Una etiqueta de advertencia no basta si el usuario recibe un consejo alterado.

Fijar una semilla, un prompt o una configuración de generación no constituye esa prueba de equivalencia semántica. La repetibilidad tampoco demuestra verdad: el sistema puede repetir un error.

## 7. Recorrido y trazabilidad exigibles

| Paso | Producto que debe poder examinarse | Condición de rechazo o detención |
|---|---|---|
| Recepción | Original, identidad del episodio, idioma/perfil y contexto autorizado | Entrada fuera de límites o contexto no identificable |
| Interpretación | Consulta tipada y derivación comprobable en el perfil; alternativas y aclaraciones | Ambigüedad, referencia no resuelta o transformación no autorizada |
| Admisión y cobertura | Objeto, operación, permisos y versiones resueltos | Falta de cobertura, permiso o vigencia |
| Recorrido | Dependencias obligatorias, fuentes y operadores realmente utilizados | Omisión crítica, referencia rota o recurso agotado |
| Resultado | Estado exacto y fundamento suficiente para la operación | Incoherencia, pérdida de ligadura o fallo técnico |
| Presentación y entrega | Cuerpo autorizado, representación visual correspondiente y continuidad de permiso | Cambio de significado, entrega revocada o evidencia obligatoria ausente |
| Custodia | Registros y artefactos recuperables ligados al episodio | Pérdida o cierre no observado, con su diagnóstico explícito |

El orden material y los mecanismos se concretarán en el contrato pertinente. Los pasos no constituyen por sí mismos una implementación.

La auditoría permitirá consultar entradas, versiones, reglas, rutas activadas, causas de inactividad, operaciones, programas o conectores efectivamente utilizados, salidas, fallos y autorizaciones pertinentes [F06 §4.4]. No equivale a conocer exhaustivamente los procesos internos neuronales. Una explicación del modelo no reemplaza registros instrumentados; OpenTelemetry no acredita por su mera presencia completitud, independencia ni custodia de la traza.

La observabilidad permite examinar lo sucedido. Los mecanismos de seguridad activa deben impedir, detener o revocar operaciones según reglas. La seguridad pasiva delimita accesos y contiene consecuencias mediante separación y aislamiento. Son categorías de trabajo para localizar obligaciones; su realización y eficacia se comprueban por separado, reutilizando S32/S38 y sus contratos, sin crear otra arquitectura aquí.

## 8. Consulta, investigación e incorporación

Consulta profesional, investigación y actualización del conocimiento conservan episodios, permisos y registros diferenciados [F05; F06 §5]. En la consulta se usa la versión admitida; en el alcance clínico documentado no se consulta Internet. Investigar no autoriza a actualizar conocimiento, equivalencias lingüísticas, reglas, pesos o permisos durante la aplicación.

La IA actúa como auxiliar bajo dirección humana. No se introduce autonomía decisoria ni delegación entre modelos por esta pieza. La valoración y autorización competen al humano facultado. La observación técnica puede acompañar la ejecución sin convertirse en investigación simultánea ni en otra decisión de dominio.

## 9. Examen adversarial documental

No se han ejecutado los casos siguientes. Se identifican objeciones para orientar la aceptación futura y reutilizar controles existentes.

| Ataque o confusión | Qué debe discriminar la futura comprobación | Antecedente |
|---|---|---|
| Pregunta sobre A, referencia y cita válidas de B | Correspondencia de significado, no sólo formato, permiso o hash | IE-004 / RETP-119–125 |
| Paráfrasis legítima rechazada siempre | Utilidad junto a seguridad; el rechazo sistemático no demuestra éxito | ES27 / RETP-117–118 |
| Eliminar una negación o cambiar una unidad al corregir una errata | Original preservado, transformación declarada y operación correcta o aclaración | F05; RETP-127 |
| Resolver ambigüedad escogiendo sólo la lectura permitida | Significado antes de política; no desambiguación por permisos | RETP-135 |
| Omitir una dependencia y mostrar un polígono válido | Cobertura contra obligaciones constituidas y ligadura de la figura | F06 |
| Traducir o resumir suprimiendo una condición | Fidelidad de salida, incluso con resultado interno correcto | F04–F06 |
| Reutilizar respuesta tras revocación o entre episodios | Vigencia y aislamiento de contextos y entregas | F04–F05 |
| Instrucción oculta en una fuente, conversación o propuesta de otra IA | Contenido sin autoridad; efectos materialmente mediados | F01, F06; S38 |
| Log narrado o huella de evidencia irrecuperable | Observación efectiva y custodia, no mera declaración | RETP-142 y sucesores |
| Cien repeticiones del mismo error | Corrección contra oráculo independiente del resultado observado | F04; RETP-122 |
| Modelo ausente con respuesta correcta producida íntegramente por A | Aportación real del modelo separada de la conformidad del receptor | RETP-129, 137 |

**Dictamen de esta revisión:** procede registrar y conservar el objeto; no procede dar por alcanzada la invariancia lingüística general, la interfaz profesional ni la aptitud del agente. El banco futuro deberá contener positivos, negativos, contrastes mínimos y controles de sensibilidad, con esperados comprometidos antes de ejecutar. Se reutilizará evidencia cuya identidad y condiciones sigan siendo aplicables.

## 10. Momento de tratamiento y relación con otros sucesos

| Suceso | Objeto | Relación con S40 |
|---|---|---|
| S28, finalizado en su alcance documental | Rutas y consejo auditable | Fuente recibida; no se reabre |
| S37, pendiente | Contraste humano de propuestas de distintas IAs y síntesis | Estudio distinto; S40 no exige varios modelos |
| S38, pendiente | Encapsulación y coexistencia con otros servicios o IA | Dependencias materiales a recibir cuando corresponda; no se absorbe |
| S39, en ejecución como seguimiento | Ensayo EIO de inferencia y observabilidad | Aporta evidencia tecnológica; no demuestra S40 |
| S40, pendiente | Interlocución del experto y fidelidad del consejo en el universo autorizado del agente | Alta actual y evaluación material futura |

**Ahora:** documento explicativo, alta en Sucesos y remisión de Calidad. No nueva ejecución, modelo, dependencia, gasto ni modificación del encargo de Holmes.

**Antes de consolidar un alcance que incluya esta capacidad:** revisar si la representación conserva significado, identidad, cobertura y fundamento. Si una pérdida concreta afecta a una operación incluida, deberá tratarse en su sede antes de cerrar ese alcance; no basta diferirla por comodidad. F02 §7 y F05 §6 proporcionan el procedimiento para justificar una intervención temprana.

**Cuando se autorice su apertura:** recibir los sucesores de ES27/IE-004 y de EIO, seleccionar una única operación delimitada con conocimiento y permisos constituidos, fijar perfil lingüístico, contrato de presentación, criterios y presupuesto. Empezar con casos sintéticos cuando falte una constitución de dominio apta. Rust y los destinos aplicables conservarán las reglas de prueba del proyecto.

**Antes del uso profesional:** acreditar el recorrido completo y sus garantías materiales en el perímetro ofrecido, con recepción humana y límites explícitos. No basta terminar el ensayo del motor.

No se fija una fecha de calendario ni se obliga a terminar todo el SV antes de examinar requisitos que afectarían a su suficiencia. Tampoco se abre una línea paralela.

## 11. Entrega futura y criterio de cierre

La futura entrega de S40 deberá relacionar, para cada obligación: fuente, operación, representación, componente responsable, realización, caso positivo y negativo, evidencia, límite y decisión receptora. Deberá identificar las capacidades incluidas y las excluidas.

Como mínimo deberá resolver o excluir expresamente: interpretación y aclaración; idioma y equivalencias; cobertura y rutas; ligaduras parámetro/posición/evidencia; autoridad y vigencia; fidelidad de presentación; custodia; aislamiento y recursos; utilidad del modelo. Los resultados adversos se conservarán.

El expediente documental actual queda preparado. **S40 permanece pendiente porque su evaluación integrada, contrato concreto, implementación y campaña futura no se inician por este documento.** Inicio y fin del suceso quedan vacíos. La continuidad material dependerá de un encargo humano acotado.

## 12. Control de esta incorporación

Se añaden este documento, S40 revisión 0 en CSV/Markdown/historial, RETP-2026-260 en CSV/Markdown y un enlace desde el inicio de continuidad del 15/09. Se conservan los asientos previos y los estados S37/S38/S39. No se modifican rectores, fuentes experimentales, guardas, oráculos ni el mapa HTML histórico.

El commit de publicación identifica la incorporación. Los enlaces de fuentes fijan cortes anteriores; no se anticipa un hash futuro ni se declara un espejo de laboratorio actualizado. La revisión es documental, no una prueba del SV ni una nueva auditoría independiente.
