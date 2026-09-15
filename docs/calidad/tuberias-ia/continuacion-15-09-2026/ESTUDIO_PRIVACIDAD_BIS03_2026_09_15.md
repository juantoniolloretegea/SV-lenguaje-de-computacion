# Privacidad, consultas federadas y fronteras de terceros · estudio acotado BIS-03

Fecha de consulta: 15/09/2026. S22 permanece en ejecución. Registro: RETP-2026-242. Autoría de la revisión documental: Watson / W-S26-02.

## 1. Resultado y momento de incorporación

Se completa este estudio documental de sedes y condiciones de habilitación dentro de BIS-03. No acredita implementación, anonimización de un conjunto real, cumplimiento jurídico integral ni cierre de (p1+p3)-Bis.

**Decisión de alcance:** incorporar ahora los requisitos de flujo, responsabilidad y aceptación; concretar el contrato de cada interfaz afectada antes de congelarla o cerrar BIS-03. Implementar y ensayar sus controles antes de habilitar el conector o iniciar el tratamiento personal correspondiente, lo que ocurra primero. Si ya hubiera tratamiento personal, su evaluación no se difiere hasta el cierre de Bis.

| Momento | Trabajo | Condición de salida |
|---|---|---|
| Ahora, BIS-03 | Este estudio; clasificación y sedes; controles y pruebas especificadas | Documento enlazado, responsabilidades funcionales identificadas y pendientes explícitos |
| Antes del cierre de BIS-03 o de congelar una interfaz afectada | Contrato concreto de flujo: campos, finalidades, permisos, destinatarios, conservación y errores | Cada obligación tiene sede y criterio observable; lo no constituido no se considera habilitado |
| Antes del primer uso personal o activación del conector | Revisión jurídica contextual y EIPD cuando proceda; implementación y ensayos Rust con datos artificiales | Evidencia positiva y negativa del flujo completo, autorización competente y configuración efectiva |
| Al cambiar finalidad, proveedor, datos o riesgos | Reevaluar sólo lo afectado | Decisión versionada y pruebas pertinentes antes del nuevo uso |

El trabajo mecánico y las correcciones del reconocedor continúan por su expediente. La revisión entregada en [00a8a1ae](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/00a8a1aebd99cb1ac2984146148d4c61d2edbfe1/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/realizacion-leyenda-01/REVISION_PREVIA_CUALIFICACION_01.md) identifica objeciones de atribución, cuotas y selección. Q1/Q2 y E1–E16 siguen sin ejecución acreditada en ese corte; el reconocedor permanece candidato y no cualificado. Este estudio no modifica su código, parámetros, contrato o reservas. Una nueva entrega de Grok requiere recepción propia.

GUI cancelada según Acta 001/S24. Se conserva el retorno rector: completar Bis y retomar el catálogo/cierre de fase aplicable. No se renumeran actas.

## 2. Base del proyecto y límites de la revisión

Corte de entrada del Lenguaje: `64feede79ae46de4efc172561edb3ab1f27089f0`, rama `main`. Se han leído íntegramente AGENTS.md, Pilares, Perfiles/Ensamblaje y el acta de transición, incluidas sus precisiones §§12–30. Referencias rectoras:

- [Pilares y restricciones](../../PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md).
- [Perfiles, contratos y ensamblaje](../../ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md).
- [Transición secuencial](../../../dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md).
- [Workflow Bis](../paridad-imagen-celula-matematica/estudio-nucleo-agentes/WORKFLOW_P1_P3_BIS_v2.md), [Acta 001](ACTA_001_CONTINUIDAD_Y_RUMBO_2026_09_15.md) y [recepción del reconocedor](ACTA_002_RECEPCION_COTEJO_IDENTIDAD_Y_CONTRATO_LEYENDA_R06_2026_09_15.md).

La base reutilizable separa capacidad, permiso y autoridad; conserva obligaciones al componer y delegar; exige evidencia antes de ampliar el núcleo. La revisión es dirigida a decisiones de arquitectura, no una revisión sistemática exhaustiva ni auditoría de una biblioteca. Se consultaron fuentes oficiales, documentación técnica y resúmenes originales de investigación; no se atribuye revisión íntegra de los protocolos criptográficos ni validación experimental propia.

## 3. Marco europeo: obligación, orientación y calendario

| Fuente primaria y estado consultado | Consecuencia para el SV |
|---|---|
| [RGPD, Reglamento 2016/679](https://eur-lex.europa.eu/eli/reg/2016/679/oj/eng), vigente; arts. 5, 6, 9, 25, 28, 32, 35 y capítulo V | Finalidad y minimización; base jurídica y condición adicional para categorías especiales; diseño seguro, responsabilidades, EIPD según riesgo y condiciones de transferencias. No impone una tecnología federada universal. |
| [CEPD 4/2019, versión 2.0 final, 20/10/2020](https://www.edpb.europa.eu/sites/default/files/files/file1/edpb_guidelines_201904_dataprotection_by_design_and_by_default_v2.0_en.pdf) | Protección desde la determinación de medios y por defecto. El diseño no se aplaza hasta producción; las medidas se eligen según contexto y eficacia. |
| [CEPD 07/2020, versión final 07/07/2021](https://www.edpb.europa.eu/documents/guideline/guidelines-072020-on-the-concepts-of-controller-and-processor-in-the-gdpr_en) | Identificar responsable, corresponsables y encargados según funciones reales. Una IA o una API no sustituye a la persona u organización responsable. |
| [CEPD, opinión sobre modelos de IA, diciembre de 2024](https://www.edpb.europa.eu/news/edpb-opinion-on-ai-models-gdpr-principles-support-responsible-ai_en), síntesis oficial consultada | Anonimato del modelo se evalúa por caso, considerando identificación y extracción mediante consultas. Uso para ciberseguridad no constituye por sí solo base jurídica suficiente. |
| [CEPD 02/2026 sobre anonimización](https://www.edpb.europa.eu/public-consultations/guidelines-022026-on-anonymisation_en), consulta 08/07–30/10/2026 | Borrador consultivo en la fecha del estudio; seguimiento pertinente, sin tratarlo como directriz final ni obligación nueva. |
| [EHDS, Reglamento 2025/327](https://eur-lex.europa.eu/eli/reg/2025/327/oj/eng), arts. 61, 69, 73–74 y 105 | Para el uso secundario regulado: permisos/peticiones, entornos seguros y revisión de descargas no personales; roles específicos. El capítulo IV se aplica generalmente desde 26/03/2029, con excepciones y categorías desde 2031. El art. 73.5 prevé actos de ejecución antes de 26/03/2027. No se afirma que éstos estén ya aprobados. |
| [Comisión: calendario EHDS](https://health.ec.europa.eu/ehealth-digital-health-and-care/european-health-data-space-regulation-ehds_en) | Diferenciar entrada en vigor, aplicación y categorías de datos. Preparar compatibilidad sin atribuir aplicación plena en septiembre de 2026. |
| [TEHDAS2: salvaguardas de privacidad, 05/06/2026](https://tehdas.eu/results/tehdas2-publishes-guidance-on-applying-privacy-safeguards-under-the-ehds/) | Orientación técnica no vinculante: minimización durante solicitud, análisis y salida; escoger anonimización o seudonimización según uso y riesgo. |
| [TEHDAS2: entornos seguros, 05/06/2026](https://tehdas.eu/results/tehdas2-publishes-new-specifications-for-secure-processing-environments-under-the-ehds/) | Especificaciones técnicas no vinculantes; distinguir un entorno seguro autónomo de su eventual federación. Reutilizar esta estructura al contratar el entorno. |
| [NIS2, Directiva 2022/2555](https://eur-lex.europa.eu/eli/dir/2022/2555/oj/eng), arts. 2 y 21 | Determinar entidad, sector y transposición nacional aplicables. Gestionar seguridad y cadena de suministro cuando corresponda. No sustituye al RGPD ni convierte toda telemetría en anónima. |
| [Reglamento de IA 2024/1689](https://eur-lex.europa.eu/eli/reg/2024/1689/oj/eng) | Clasificar sistema, finalidad y papel de cada organización antes del despliegue; no atribuir alto riesgo por la mera palabra «salud». Evaluación contextual pendiente, sin certificar aplicabilidad completa aquí. |

La federación describe dónde se calcula. La anonimización exige evaluar identificación razonablemente posible; la seudonimización mantiene vinculabilidad bajo condiciones. Para el custodio que conserva la información de enlace, cambiar el nombre por un código no elimina el carácter personal. Cifrado y hash tampoco justifican por sí solos declarar anonimato. La valoración debe cubrir cada destinatario y sus medios, no sólo el fichero aislado.

## 4. Separaciones que debe conservar el contrato

Las categorías son ejes independientes: (a) dato personal/no personal y categorías especiales; (b) secreto clínico, empresarial u operativo; (c) identificabilidad y contexto del receptor; (d) finalidad autorizada; (e) conservación y territorio. Un dato público puede ser personal; un dato no personal puede ser confidencial. Un atributo inferido puede revelar salud.

| Actor o componente | Función técnica | Decisión competente y límite |
|---|---|---|
| Persona interesada | Persona a la que se refieren los datos | Sus derechos no se confunden con los permisos de un operador |
| Profesional, investigador u operador | Solicita, examina o usa resultados | Identidad, función, finalidad y ámbito autorizados; ser humano o experto no concede acceso universal |
| Organización responsable | Decide finalidades y medios esenciales | Justifica tratamiento, contratos, conservación y acceso; identifica apoyo del DPD cuando proceda |
| IA local o agente | Ejecuta una tarea en un entorno | Permiso técnico delimitado, sin poder de autoampliación; inferencia y entrenamiento son operaciones distintas |
| API y proveedor externo | Recibe solicitudes y produce respuestas | Contrastar función jurídica real, subencargados, soporte, retención, reutilización, ubicaciones y acceso internacional |
| Coordinador federado y nodos | Distribuyen cálculo y combinan respuestas | Precisar qué ve cada participante y qué coaliciones contempla el modelo de amenaza |

Una política de «no entrenamiento» no demuestra ausencia de almacenamiento, acceso humano o registros. Ubicación europea del servidor no demuestra ausencia de acceso desde terceros países. Estas son preguntas de contratación y verificación del flujo, no presuposiciones sobre un proveedor concreto.

## 5. Matriz de flujos y sedes

Las siguientes condiciones son decisiones de diseño para constituir interfaces. **No se afirma que el código actual las imponga.**

| Flujo | Información que debe clasificarse | Sede del control y condición de habilitación |
|---|---|---|
| Fixtures del laboratorio | Valores, nombres, texto, rutas y metadatos | Banco artificial independiente de personas reales; revisar que no contiene copias identificables |
| Ingesta clínica | Identificadores, salud, genética, texto libre, adjuntos | Adaptador y almacén competente: autorización, finalidad, campos mínimos y conservación antes de admitir |
| Consulta por persona | Consulta, cohorte, contexto y resultado | Servicio: autenticar y limitar por finalidad, organización y recurso; registrar evidencia mínima |
| Inferencia local | Prompts, contexto, embeddings, cachés, salida | Runtime: permisos acotados, aislamiento y egreso controlado; salida sometida a revisión propia |
| API externa | Cuerpo, cabeceras, identificadores, errores, soporte | Frontera de salida: lista de campos y destinatarios permitidos; contrato del proveedor y revisión territorial |
| Consulta estadística federada | Consultas, conteos, agregados, sucesión de resultados | Nodo y coordinador: consultas permitidas y control de revelación acumulada; no basta ocultar filas |
| Entrenamiento federado | Gradientes, actualizaciones, modelo y participantes | Capacidad separada, no habilitada por este estudio; justificar finalidad y amenazas antes de elegir protocolo |
| Frame, documento o exportación | Contenido, enlaces, identidad, procedencia, atributos inferidos | Consumidor y publicador: nueva decisión de acceso/exportación; transformar formato no elimina restricciones |
| Telemetría de ciberseguridad | IP, usuario, dispositivo, tiempos, incidentes, secretos | Perfil CYB y servicio: distinguir datos personales y confidenciales; necesidad, acceso y retención específicos |
| Logs, respaldos y consultas históricas | Copias, errores, identificadores y trazas | Plataforma/persistencia: mínimo necesario, acceso, plazos, restauración y revocación comprobables |

Se reutiliza la separación de sedes del acta de transición, especialmente la frontera operativa de su fila 13 y las obligaciones de persistencia/plataforma. F01/F02 conservan sus pendientes; la capacidad de localizar o resolver contenido no constituye permiso para leerlo.

**Decisión de núcleo:** este estudio no demuestra necesidad de una primitiva clínica o jurídica nueva en el álgebra, Tri, Frame o IR. Las políticas concretas se constituyen en contratos de perfil, adaptadores y servicios. El núcleo debe conservar y hacer verificables las distinciones que le competan. Si una interfaz no puede representar una obligación necesaria, se documentará la pérdida, el contraste discriminante y el cambio mínimo antes de promoverla; una etiqueta opaca «privado» no acredita cumplimiento.

**Conservación:** distinguir el historial de decisiones del contenido personal. Un registro append-only no autoriza retener indefinidamente datos clínicos. Los plazos dependen de finalidad y obligaciones aplicables; la supresión tampoco es absoluta. En Git público no se deposita contenido personal protegido. La futura prueba de retención debe cubrir cachés, respaldos y restauraciones; conservar únicamente una huella no garantiza anonimato.

## 6. Soluciones existentes que merece la pena reutilizar

| Fuente y nivel examinado | Aporte reutilizable | Límite para esta decisión |
|---|---|---|
| [DataSHIELD / OBiBa, documentación oficial](https://www.obiba.org/) | Análisis distribuido con datos individuales en los nodos; patrón de llevar consultas al dato | Referencia de arquitectura; no se instala R ni se selecciona esa plataforma en el SV |
| [Bonawitz et al., CCS 2017, Practical Secure Aggregation](https://research.google/pubs/practical-secure-aggregation-for-privacy-preserving-machine-learning/), ficha y resumen originales | Agregar actualizaciones protegiendo aportaciones individuales según el modelo del protocolo | No resuelve por sí sola revelación del agregado, finalidad ilícita o envenenamiento |
| [Zhu, Liu y Han, NeurIPS 2019, Deep Leakage from Gradients](https://papers.nips.cc/paper_files/paper/2019/hash/60a6c4002cc7b29142def8871531281a-Abstract.html), resumen original | Evidencia de reconstrucción de datos a partir de gradientes en las condiciones estudiadas | No atribuir ataque universal; sí rechazar «los datos no salen, luego hay anonimato» |
| [Dwork y Roth, 2014, The Algorithmic Foundations of Differential Privacy](https://www.cis.upenn.edu/~aaroth/Papers/privacybook.pdf), definición 2.4 y composición | Garantía formal y contabilización de consultas repetidas | Requiere adyacencia, sensibilidad, parámetros y utilidad definidos; no es un certificado jurídico automático |

La opción inicial a evaluar es **consulta federada con salida controlada**, si la necesidad real exige varios custodios. Una consulta local autorizada puede bastar para un solo custodio. El aprendizaje federado queda como capacidad diferente, fuera de la realización actual. No se implementa criptografía propia ni se elige una biblioteca sin examinar mantenimiento, licencia, interfaces y evidencia de su versión.

La privacidad diferencial es opcional y requiere un contrato estadístico: para bases adyacentes D y D' y eventos A, la garantía limita Pr[M(D)∈A] por exp(ε)Pr[M(D')∈A]+δ. Importa la composición entre consultas, no sólo una respuesta. Aquí no se fijan ε, δ o un umbral universal de cohorte. Tampoco se altera con ruido un estado clínico exacto del núcleo: una salida estadística protegida debe declarar su semántica y error.

## 7. Criterio operativo y contrastes pendientes

Esquema de diseño, sin añadir estados al lenguaje:

`admitir = identidad válida ∧ finalidad autorizada ∧ recurso autorizado ∧ destinatario permitido ∧ condiciones vigentes ∧ cuotas disponibles`.

Una condición desconocida o incompleta impide ese uso hasta resolverse. Emitir un resultado exige además una comprobación de salida que considere historial de consultas y contexto receptor. La autorización debe seguir vigente al efectuar la operación; reintentos y concurrencia no pueden eludir revocación o duplicar una liberación no permitida.

| Caso propuesto | Positivo esperado | Negativo discriminante |
|---|---|---|
| P01 Identidad/ámbito | Acceso al recurso autorizado | Mismo identificador solicitado desde otro ámbito rechazado |
| P02 Delegación | Agente actúa dentro del permiso recibido | Prompt, documento o herramienta intenta ampliar permisos: ningún efecto adicional |
| P03 Finalidad | Inferencia autorizada con campos mínimos | Reutilización para entrenamiento sin autorización específica impedida |
| P04 Egreso | Solicitud permitida y minimizada | Campo clínico en cabecera, URL o mensaje de error no sale |
| P05 Federación | Agregado permitido por política constituida | Consultas por diferencias o intersecciones que revelan individuo bloqueadas o protegidas |
| P06 Presupuesto | Consulta contabilizada una vez conforme al contrato | Reintento/concurrencia no evade límites ni genera liberaciones extra |
| P07 Revocación | Permiso vigente comprobado antes del efecto | Revocación entre admisión y efecto evita el efecto no autorizado |
| P08 Conservación | Información necesaria disponible durante plazo aplicable | Caducada o suprimida no reaparece accesible tras restauración |
| P09 Derivados | Resultado y frame con destinatario autorizado | Embedding, enlace o atributo inferido no elude la restricción original |
| P10 Evidencia | Registro mínimo permite auditar la decisión | Logs y respaldo no contienen payload prohibido; acceso al registro también limitado |

**Estado de las pruebas: especificadas conceptualmente, cero ejecutadas.** Antes de ejecutarlas hay que materializar fixtures artificiales, oráculos independientes, cuotas, versión de interfaz y observador del efecto. Se ensayarán en Rust; las pruebas de privacidad no se cuentan como Q1/Q2 ni como E1–E16 del reconocedor.

## 8. Pendientes y cierre documental

Quedan pendientes para cada flujo concreto: responsable efectivo, finalidad/base jurídica, categorías y procedencia, contrato de terceros, EIPD cuando corresponda, umbrales de revelación y utilidad, plazos, implementación y ensayos. No se dan por resueltos mediante una lectura bibliográfica.

El estudio termina cuando su alcance y decisiones quedan localizables en continuidad, workflow y registros. BIS-03 permanece abierto por sus otras sedes y contratos; Bis no se cierra. Grok continúa exclusivamente la corrección ya encargada en su sede; este documento no le ordena nuevas funciones.

Trazabilidad de esta incorporación: Acta 001 §7, S22 y RETP-2026-242. En el corte de entrada existen remisiones a RETP-2026-241, pero no su asiento en los dos registros centrales leídos. Se conserva esa referencia sin reconstruirla ni reutilizar su número. Las copias históricas de laboratorio mantienen su corte; esta entrega documental tiene sede canónica en el Lenguaje y no afirma ensayo o sincronización material de esos espejos.
