# Parte de trabajo y alcance: privacidad, seguridad y relación con OP-CYB-001

**Fecha:** 15 de septiembre de 2026.  
**Seguimiento:** S32.  
**Estado:** en ejecución.

## 1. Qué hace este trabajo

Define las condiciones de privacidad y seguridad que deben conservar las consultas, las evidencias y las salidas del Sistema Vectorial SV, y establece su correspondencia con las obligaciones ya previstas en el primer universo de ciberseguridad inteligente, OP-CYB-001.

El trabajo comprende la identificación de requisitos, su asignación a los componentes responsables de cumplirlos y los criterios que permitirán comprobarlos. Distingue el conocimiento necesario para evaluar una actuación de los mecanismos que protegen el sistema que realiza esa evaluación.

## 2. Qué cubre

| Materia | Cobertura |
|---|---|
| Identidad y autenticación | Diferenciar persona, cuenta, sesión, agente, servicio y principal representado; determinar qué acredita cada comprobación. |
| Autorización | Delimitar operación, recurso, finalidad, destinatario, delegación, vigencia y revocación. |
| Confidencialidad y privacidad | Clasificar datos y derivados; limitar acceso y difusión; determinar vistas, conservación y tratamiento de metadatos. |
| Integridad y autenticidad | Conservar procedencia y transformaciones; detectar sustituciones y omisiones relevantes; distinguir integridad técnica y veracidad. |
| Disponibilidad y continuidad | Identificar dependencias, recursos, recuperación, reintentos y efectos inciertos. |
| Consultas distribuidas | Determinar cuándo conviene consultar en los custodios de origen y qué información puede circular entre ellos. |
| Terceros | Examinar solicitudes, respuestas, registros, retención y accesos de proveedores de API y otros servicios. |
| Verificación | Relacionar requisitos con contratos concretos, casos positivos, casos negativos y evidencia observable. |

Se consideran por separado datos personales, categorías especiales, información confidencial no personal y datos cuya combinación permita identificar o inferir atributos de una persona. Las restricciones también alcanzan explicaciones, enlaces, registros, cachés y copias de respaldo cuando contengan esa información.

La federación no constituye por sí misma anonimización. Una respuesta agregada tampoco acredita el estado de un activo concreto si pierde la identidad o la evidencia que exige su evaluación.

## 3. Hasta dónde llega

OP-CYB-001 mantiene su objeto: fundamentar la evidencia y legitimidad de la corrección de una vulnerabilidad mediante actualización de un activo. Se conservan sus 32 definiciones paramétricas, 17 controles y requisitos existentes.

La correspondencia utiliza especialmente RS01–RS04 para identidad, autoridad y vistas; RS07 para cobertura de flujos; RS09–RS12 para historia, custodia y continuidad; y las condiciones del consejo de la adenda §12. No amplía su catálogo ni convierte el universo en una plataforma general de autenticación, protección de datos o entrenamiento de modelos.

| Resultado alcanzado | Límite de ese resultado |
|---|---|
| Estudio documental de privacidad y marco europeo disponible | No acredita cumplimiento integral de un servicio o tratamiento concreto. |
| Correspondencia identificada con las obligaciones de OP-CYB-001 | No equivale a implementación de controles ni a ejecución del universo. |
| Diez pares de aceptación propuestos en el estudio | Son especificaciones conceptuales; sus pruebas de privacidad no se han ejecutado. |
| Condiciones de incorporación delimitadas | Los contratos concretos de los flujos afectados siguen pendientes. |

Se mantiene la diferencia entre la evidencia original bajo custodia y la vista mínima destinada al consejo. Suprimir toda atribución puede impedir una comprobación legítima; divulgar el original completo puede exceder la finalidad autorizada.

El alcance no incluye nuevos parámetros, otro universo, cambios de la semántica del núcleo, selección de plataforma, habilitación de API, tratamiento de datos personales reales ni ejecución de campañas. La GUI y el punto de retorno del trabajo principal conservan su situación vigente.

## 4. Qué queda pendiente y cuándo corresponde

1. **Durante BIS-03:** completar la correspondencia por flujo entre requisito, dato, relación, componente y prueba. Reutilizar los controles y casos existentes; justificar cualquier necesidad adicional.
2. **Antes de congelar una interfaz afectada o cerrar BIS-03:** constituir su contrato con identidad, finalidad, permisos, destinatarios, conservación, recursos, fallos y criterios de aceptación.
3. **Antes de habilitar el conector o iniciar el tratamiento personal correspondiente, lo que ocurra primero:** determinar las obligaciones jurídicas del caso, realizar la evaluación de impacto cuando proceda y aportar implementación y pruebas de los controles exigibles. Si ya existe tratamiento personal, su evaluación no se difiere.
4. **En la realización que corresponda:** preparar entradas artificiales y resultados esperados, ejecutar pruebas en Rust y registrar el alcance comprobado. Se conserva la prioridad y secuencia de las tareas principales.
5. **Ante cambios relevantes:** revisar las finalidades, datos, proveedores, permisos o riesgos afectados antes de ofrecer la nueva capacidad.

El siguiente trabajo concreto es completar la matriz de correspondencia de los flujos afectados, con referencia a las obligaciones existentes de OP-CYB-001 y a las interfaces del Lenguaje. Este parte no inicia una campaña ni habilita un servicio.

## 5. Condición de cierre del seguimiento

S32 permanecerá abierto mientras falte la correspondencia verificable de los flujos incluidos o la resolución de sus contratos y condiciones de habilitación.

Para cerrar el alcance documental deberán quedar identificados, para cada flujo, el requisito, su componente responsable, la prueba exigida y su situación. La implementación o prueba que corresponda a una fase posterior deberá tener un seguimiento concreto enlazado y una condición de habilitación explícita; no bastará con consignar «pendiente».

Un eventual cierre documental no acreditará seguridad material, anonimato, cumplimiento integral ni cierre de (p1+p3)-Bis.

## 6. Documentos de referencia

- [Estudio de privacidad BIS-03: clasificación, fuentes europeas y aceptación](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/8fa67a917d78072582c24af76f3ce1f47d6c079d/docs/calidad/tuberias-ia/continuacion-15-09-2026/ESTUDIO_PRIVACIDAD_BIS03_2026_09_15.md).
- [OP-CYB-001: alcance, obligaciones de soporte y condiciones del consejo, §§3, 6–10 y 12](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/bbac1b44b1d3b845305e9cde492a08221206d631/dominios/ciberseguridad-inteligente/dominio-04-09-26/universos/OP-CYB-001/ACTA_CONTINUIDAD_Y_RELEVO_OP_CYB_001_AL_LENGUAJE_SV_2026_09_09.md).
- [Acta de continuidad del trabajo](ACTA_001_CONTINUIDAD_Y_RUMBO_2026_09_15.md).
- [Registro del suceso S32](../../Inventario-sv/sucesos/SUCESOS_SV.md#s32).

<a id="s32-diseno-2026-09-16"></a>

## 7. Continuación autorizada: obligaciones desde el diseño · 16/09/2026

**Seguimiento:** S32, sin alta de otro suceso. **Estado:** en ejecución. **Unidad receptora:** Watson / W-S32; releva a W-S26-02 sólo en este seguimiento. La autorización humana permite continuar el alcance tras revisión adversarial; no declara su cierre.

**Corte de entrada:** Lenguaje `7b6d722760d7a8e017e516e881ce1bfb9c9a8aab`, rama `main`. Leídos los pilares de 05/09, el acta de perfiles de 06/09, el acta de transición de Inmunología de 03/09 con sus adendas, Léame primero, el registro de sucesos, el estudio BIS-03 y el workflow V2. Se conservan las rutas de conocimiento recibidas en S28 y el corte CYB `bbac1b44b1d3b845305e9cde492a08221206d631` de §6. Las secciones 1–6 permanecen como antecedente; esta continuación precisa la oportunidad de §4.3 sin sustituir su historia.

### 7.1. Objeto y límites de la continuación

Preparar las condiciones de diseño necesarias para admitir o excluir usos futuros de forma verificable no obliga a implantar todos esos usos. Se mantiene una única matriz de correspondencia, desarrollada en §8 sobre los flujos y P01–P10 del estudio BIS-03. No se constituye otro universo, parámetro, plataforma o semántica nuclear.

Se distinguen dos ejes: consulta profesional general o caso particular; aplicación del conocimiento admitido o investigación externa expresamente solicitada. Consultar sobre un caso no habilita Internet. La aplicación no aprende por el camino ni modifica autónomamente conocimiento, pesos o constitución de dominio. La investigación externa requiere petición específica del experto y no incorpora sus resultados al dominio activo ni al caso por sí misma. La incorporación gobernada conserva su procedimiento separado y sus reservas de S28; no se crea aquí una vía de promoción. El apoyo probabilístico a la interfaz lingüística no recibe autoridad sobre la decisión soberana.

En el caso particular, el profesional mantiene su trabajo principal y selecciona los datos pertinentes. Que pueda acceder a la HCE no concede automáticamente igual acceso al agente. Una fuente hospitalaria preparada, otra institución o un servicio centralizado conserva su propia custodia; se deben determinar los permisos del trayecto concreto. Un informe aportado por el paciente no obliga a digitalizarlo ni a repetir una prueba: sólo se examina el flujo efectivamente elegido y autorizado. Leer un documento, transcribir una observación e importar un archivo son operaciones distintas.

Se separan los datos externos consumidos de los resultados SV y de sus representaciones. Un PNG no demuestra por su formato paridad, significado ni anonimato; una imagen médica externa no adquiere estatuto de frame celular. Tampoco todo ejemplo genérico es una célula constituida: los casos artificiales de prueba admitidos por los pilares no constituyen conocimiento de dominio ni evidencia de un caso real.

### 7.2. Lo que corresponde resolver ahora

El [RGPD, artículo 25](https://eur-lex.europa.eu/eli/reg/2016/679/oj?locale=es), exige al responsable integrar medidas al determinar los medios y durante el tratamiento. Su considerando 78 contempla el papel de los productores. No atribuye automáticamente al Lenguaje la condición de responsable. Roles, finalidades, minimización, seguridad y conservación deben examinarse según el tratamiento previsto; la EIPD, cuando proceda conforme al artículo 35, es previa al tratamiento.

**Decisión de diseño de esta continuación:** evaluar desde ahora las obligaciones que puedan condicionar las interfaces. No relegar esa evaluación al despliegue. Cuando falte el contexto para decidir, conservar la incógnita con sede y condición de habilitación; no darla por satisfecha ni congelar una interfaz incompatible. La intervención del responsable competente y su DPD, cuando corresponda, no queda sustituida por esta revisión técnica.

FHIR, CDA, DICOM, HIS/PACS/RIS, API o FFI no son capacidades impuestas por este parte. Si se selecciona alguna, habrá que fijar versión, perfil, límites, procedencia, transformaciones, campos y rechazo de entradas inválidas. Una API no presupone FFI. La aplicabilidad sectorial del [EHDS, Reglamento (UE) 2025/327](https://eur-lex.europa.eu/eli/reg/2025/327/oj?locale=es), depende del producto, función, uso y calendario: aquí no se declara al SV sistema HCE certificado ni se obliga a conectarlo a MyHealth@EU. Las demás fuentes del estudio conservan su estatuto; no se promueven orientaciones o borradores a obligaciones universales.

## 8. Matriz de correspondencia y contratos por concretar

**Estatuto:** matriz documental inicial de la continuación, no contrato ejecutable ni prueba de control material. Las sedes siguientes son asignaciones funcionales que deben ligarse a componentes e interfaces identificados; no afirman que esos componentes estén constituidos. Las referencias RS son las de OP-CYB-001 §6: no se extrapola su finalidad profesional a otro universo.

| Flujo y finalidad | Información y correspondencia existente | Sede funcional y contrato requerido | Decisión de diseño ahora; brecha concreta | Aceptación que debe materializarse |
|---|---|---|---|---|
| Fixtures del laboratorio: contrastes artificiales | Valores, texto, rutas y metadatos; estudio §§5 y 7 | Custodio del banco: procedencia y exclusión de datos reales | Mantener separado banco y caso profesional. Falta banco ejecutable propio de privacidad | P01/P09/P10: positivo artificial autorizado; negativo con vínculo o contenido protegido rechazado |
| Ingesta clínica: aportar al consejo el subconjunto autorizado del caso | HCE, analíticas, observaciones o adjuntos; RS01/RS02/RS04/RS07 como presión de soporte, sin constituir Inmunología | Custodio de origen, admisor y adaptador: identidad, procedencia, finalidad, campos y vigencia | No asumir ingesta masiva ni permiso heredado del médico. Faltan interfaz, responsable efectivo y selección de campos | P01/P02/P03/P07: admisión autorizada; otro ámbito, delegación excedida o revocación sin efecto no autorizado |
| Consulta por persona: uso profesional general o particular | Pregunta, contexto y resultado; RS01–RS04/RS07 | Servicio de consulta: operación, recurso, organización, destinatario y autorización | Distinguir ambos usos sin equipararlos a modos de red. Faltan contrato y observador del efecto | P01/P02/P03/P07: consulta dentro del permiso; consulta fuera del ámbito rechazada |
| Inferencia local: asistencia bajo conocimiento admitido | Contexto, caché, salida y derivados si existen; adenda CYB §12 y rutas S28 | Realización de agente y soporte: cobertura, aislamiento, límites y ausencia de promoción autónoma | No convertir el contexto de un caso en aprendizaje. Faltan ligadura de sede y contrato operativo | P02/P03/P04/P09: consumo permitido; ampliación de permisos, aprendizaje no autorizado o egreso impedidos |
| API externa: investigación específica autorizada, si se habilita | Solicitud, respuesta, URL, cabeceras, errores y proveedor; RS02/RS04/RS07, adenda §12 | Frontera de salida: petición humana, destino, campos y separación del contexto del caso | Petición de investigar no autoriza transmitir el expediente. Falta contrato de salida y tercero; no se habilita API | P02/P04/P07/P09: solicitud autorizada y mínima; fuga de dato clínico por canal lateral o promoción al dominio impedida |
| Consulta estadística federada: sólo si se justifica necesidad entre custodios | Consultas y resultados acumulados; RS04/RS07 | Nodos y coordinación: finalidades, vistas, composición y recursos | Opción no obligatoria ni constituida. Faltan necesidad, política y umbrales; no elegir mecanismo de protección aquí | P05/P06: agregado permitido; diferencias, intersecciones o reintentos que eluden política impedidos |
| Entrenamiento federado: capacidad distinta, no incluida en esta realización | Participantes, actualizaciones y modelo; estudio §§5–6 | Contrato separado únicamente si una decisión futura lo autoriza | Fuera de la aplicación y del incremento. No planificar entrenamiento por defecto ni alterar el dominio | P03: uso autorizado para consejo no habilita entrenamiento; eventual capacidad exigiría banco propio previo |
| Frame, documento o exportación: producir, presentar o compartir resultado | Estado, imagen, explicación, enlace y atributos derivados; RS04/RS08 | Productor, consumidor y publicador: estatuto, procedencia, vista y destinatario | Generar no equivale a publicar. Faltan ligaduras entre salida y política; mantener paridad exacta separada de privacidad | P04/P09/P10: salida permitida; PNG, enlace o derivado no eluden acceso ni conservación |
| Telemetría CYB: evaluar un caso autorizado sin ampliar OP-CYB-001 | Activo, IP, usuario, tiempos, incidente y secretos; RS01–RS04/RS07/RS09–RS12 | Perfil CYB y servicios de custodia/consulta: finalidad, vistas y cobertura | Distinguir datos personales y secretos no personales. Falta contrato del trayecto; no convertir el universo en plataforma | P01/P02/P07/P09/P10: vista suficiente permitida; acceso transversal o revelación de secreto rechazados |
| Logs, respaldos y consultas históricas: trazabilidad y recuperación | Evidencias mínimas, referencias y copias; RS03/RS04/RS09–RS12 | Persistencia y custodia: R2-0 y sedes pendientes de S26/F01/F02 | Separar historia de decisiones y contenido protegido. Append-only no determina retención ilimitada. Faltan plazos y reglas de restauración del flujo | P06/P07/P08/P10: recuperación autorizada; revocación eludida, reaparición prohibida o payload indebido impedidos |

**Evidencia actual común:** fuentes documentales del corte de §7, estudio BIS-03 y esta matriz. P01–P10 siguen siendo especificaciones conceptuales: cero nuevas pruebas de privacidad ejecutadas. Los positivos y negativos de la última columna precisan la observación esperada, pero aún requieren fixtures, oráculos independientes, versión de interfaz y observador material. No se cuentan como Q1/Q2, E1–E16 ni campaña del reconocedor.

**Puertas comunes, con seguimiento en S32:** antes de congelar cada interfaz afectada o cerrar BIS-03, resolver su sede concreta, contrato identificable, interpretación de campos, autoridad competente y aplicabilidad; declarar qué conserva/valida el Lenguaje y qué impone el soporte. Antes de habilitar el flujo, acreditar los controles y ensayos aplicables. Si un contrato exige una distinción no representable, registrar la pérdida y el contraste discriminante y detener esa promoción; no añadir una primitiva por intuición ni representar el fallo como Tri.U.

Las capacidades opcionales pueden excluirse mediante decisión motivada, sin hacerlas requisitos de cierre material del sistema. Si se aplaza una realización necesaria para un uso retenido, su entrega deberá quedar enlazada a un seguimiento concreto antes de cerrar S32. Esta matriz no finge cumplido ese enlace: las brechas siguen a cargo de S32; S22 y S26 conservan sus encargos, no reciben automáticamente nuevos ensayos.

## 9. Revisión adversarial de esta continuación

Revisión documental finita, no campaña experimental ni certificación jurídica.

| Objeción examinada | Disposición |
|---|---|
| §4.3 podría leerse como aplazamiento de toda evaluación jurídica hasta el conector | Precisado en §7.2: las decisiones que condicionan el diseño se examinan ahora; queda la habilitación material como puerta posterior |
| El acceso del experto podría convertirse en autorización implícita del agente | Separados persona, delegación, recurso y finalidad en §7.1 y en la matriz |
| Investigar fuera podría significar exportar el caso, aprender o modificar el universo | Se prohíbe esa equivalencia; petición específica, control de salida y procedimiento separado de incorporación |
| Prever interoperabilidad podría obligar a implementar formatos, federación o FFI | Se conservan como opciones sujetas a necesidad y contrato, no como despliegues obligatorios |
| Imágenes, hashes, explicaciones o registros podrían declararse inocuos por ser derivados | Deben clasificarse y conservar sus restricciones; ningún formato prueba anonimato |
| Trazabilidad append-only podría exigir retención perpetua de contenido personal | Historia de decisiones y contenido protegido separados; conservación y restauración quedan por contratar |
| Asignar una sede y escribir un negativo podría presentarse como control probado | Sedes funcionales, contratos pendientes y cero ensayos nuevos expresamente separados |
| Cerrar este incremento podría cerrar S32, BIS-03 o Bis | No procede ninguno de esos cierres; se conserva el estado registrado y la evidencia parcial |

**Dictamen:** sin objeción bloqueante para publicar esta continuación documental con estas precisiones. Sí hay brechas bloqueantes para cerrar S32 o habilitar los flujos: contratos concretos, responsables efectivos, condiciones jurídicas del uso seleccionado y evidencia material. No se atribuye al hospital ni a la presencia humana una exención de esas condiciones.

## 10. Resultado, verificación y siguiente acción

Se publica el perímetro aclarado, la matriz inicial y la disposición adversarial. Continúa S32 **en ejecución**, con revisión incremental y relevo de unidad; no se crea otro suceso ni se modifica OP-CYB-001. La siguiente acción es resolver fila por fila las brechas contractuales de §8, empezando por los trayectos comunes de consulta, autorización, salida y persistencia; distinguir después usos opcionales excluidos de usos retenidos con realización diferida. No abrir conectores ni campañas por este asiento.

El cierre documental de §5 sigue sin alcanzarse. Tampoco se cierra **(p1+p3)-Bis**: su definición de workflow está finalizada, su ejecución sigue en S22. Al corte consultado, BIS-00/01 figuran finalizado, BIS-02/03/04 en ejecución y BIS-05/06/07/08 pendiente. La cualificación conjunta de leyenda aún requiere precompromiso; no se ejecutaron Q1/Q2 ni E1–E16 en esta continuación. S26/F01/F02 conserva sus pendientes materiales. No se reactiva la secuencia automática de GUI.

La comprobación de edición y registros se ha realizado con Rust y se conserva en `COTEJO_S32_DISENO.rs` y `COTEJO_S32_DISENO_SALIDA.txt`; no se confunde con una prueba de privacidad. La instalación anterior no estaba disponible en este contenedor. Para esta tarea se recibe el [paquete oficial Rust 1.98.0 x86_64-unknown-linux-gnu](https://static.rust-lang.org/dist/rust-1.98.0-x86_64-unknown-linux-gnu.tar.xz), SHA-256 `ed8ee2df70909c88cbaf87a6cfa3920dac00b537de12a6abe6906641e0f5952f`, cotejado con su [fichero oficial de huella](https://static.rust-lang.org/dist/rust-1.98.0-x86_64-unknown-linux-gnu.tar.xz.sha256), instalando sólo rustc y la biblioteca estándar nativa. No se afirma recuperado Cargo, rustup, componentes adicionales ni persistencia del entorno. Se declaran curl, sha256sum, tar e instalador oficial para transporte y provisión; apply_patch para edición; Git para identidad de blobs; conector GitHub coordinado mediante JavaScript para lectura/publicación. Sin Python ni nueva ejecución funcional SV.

**Provisión por etapa, ratificada por el autor durante esta continuación:** no basta recuperar el compilador. Se deben identificar, descargar cuando falten y verificar las dependencias efectivamente necesarias para la sesión, sin sustituir versiones silenciosamente. Este cotejo usa sólo la biblioteca estándar nativa, sin crates externos ni Cargo.lock propio. Compilación y ejecución conformes; soporte observado: cc 13.3.0, GNU ld 2.42 y Git 2.51.1; libc, libgcc_s y cargador dinámico resueltos por ldd. Esto no acredita las dependencias del reconocedor PNG o de ninguna campaña posterior, que requerirán su inventario y provisión específicos.

Publicación canónica en el Lenguaje, con RETP y Sucesos concordantes. Las copias históricas del laboratorio conservan su corte; no se afirma su sincronización material.


<a id="s32-recepcion-parche-c17-2026-09-16"></a>

### Recepción documental de la propuesta de correspondencia con C17 · 16/09/2026

**Seguimiento:** S32, en ejecución. **Referencia de calidad:** RETP-2026-248. Esta recepción identifica y vincula una propuesta documental; su contenido no se incorpora todavía como correspondencia aceptada.

**Objeto recibido:** [PARCHE_S32_CORRESPONDENCIA_C17.diff](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/19bb22c0cb614c7c05184c017e3dc3859a11b1e7/dominios/inmunologia/cambio-rumbo/05-grok-aportes/PARCHE_S32_CORRESPONDENCIA_C17.diff), depositado en `SVperitus-dataset`, rama `dominio-inmunologia`, carpeta `dominios/inmunologia/cambio-rumbo/05-grok-aportes/`. Commit de depósito: `19bb22c0cb614c7c05184c017e3dc3859a11b1e7`; blob Git: `e8b5e1d7cbb1fef1f84cd87c8f7f946e71cc1de6`. El commit añade únicamente ese archivo. La base declarada del parche es el corte del Lenguaje `29fbcb022a731aace46a64ca881c02b4b1bcb13a`.

**Relación con el trabajo:** la propuesta desarrolla la correspondencia de los flujos de §8 con los controles, requisitos y elementos de contexto de OP-CYB-001. Cita como fuente del control C17 el [expediente predecisional v0.4, §8](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/bbac1b44b1d3b845305e9cde492a08221206d631/dominios/ciberseguridad-inteligente/dominio-04-09-26/universos/OP-CYB-001/EXPEDIENTE_PREDECISIONAL_ESTADO_DE_PARTIDA_Y_CONTINUIDAD_OP_CYB_001_v0.4.md), blob `602d1c5ad33a27da1eb76ce2e72400de0902f42f`. Se conserva esa referencia para el contraste sustantivo posterior; esta recepción no lo da por ejecutado.

| Objeto propuesto | Destino identificado | Situación de recepción |
|---|---|---|
| Adición de una sección 11 de correspondencia | Este parte | Propuesta íntegra bajo custodia; pendiente de dictamen sustantivo e integración. |
| Actualización del asiento S32 | Sucesos SV, Markdown y CSV | No aplicada. La presente actualización registra exclusivamente la recepción. |
| Instantánea de revisión 2 de S32 | Historial de Sucesos SV | No aplicada. La revisión 2 efectiva corresponde a esta recepción; una incorporación posterior deberá tomar el registro entonces vigente. |

**Comprobación instrumental:** lectura completa del parche; identidad de sus bytes cotejada con el blob del depósito; comprobación de aplicabilidad textual sobre su base declarada mediante `git apply --check --verbose`, sin aplicación. Las cuatro rutas propuestas pertenecen al parte y al registro de S32. La aplicabilidad textual no valida las correspondencias ni acredita pruebas de privacidad.

**Condiciones para la incorporación posterior:** contrastar las correspondencias con las fuentes fijadas; resolver los enlaces relativos de la sección propuesta sobre su ruta de destino; distinguir expresamente admisibilidad de evidencia y autorización del acto, también en títulos y resúmenes; conservar las evidencias anteriores del seguimiento. Las fechas vacías y las afirmaciones de publicación del candidato deberán sustituirse por los hechos efectivos de la incorporación. El número de revisión y la referencia RETP se determinarán entonces, sin reutilizar los asignados a esta recepción.

**Relación con las pruebas:** este documento recibido constituye entrada para la revisión de correspondencia y la preparación de contratos. No es un resultado experimental ni justifica por sí solo un ensayo. Antes de ejecutar una prueba derivada deberán fijarse el requisito y apartado que la motivan, el contrato y su versión, las entradas, el resultado esperado y el criterio de aceptación. Su ejecución y sus resultados conservarán esas referencias dentro del seguimiento existente.

La conservación de los antecedentes y la concordancia de esta recepción entre el parte, Sucesos, historial y RETP se comprueban mediante [el cotejo Rust](COTEJO_RECEPCION_PARCHE_S32.rs) y [su salida](COTEJO_RECEPCION_PARCHE_S32_SALIDA.txt). Son comprobaciones documentales; no se han ejecutado pruebas de privacidad, Q1/Q2 ni E1–E16. S32 permanece en ejecución; esta recepción no cierra BIS-03 ni S22. Las copias históricas conservan sus cortes.
