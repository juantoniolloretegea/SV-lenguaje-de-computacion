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


<a id="s32-c17-2026-09-16"></a>

## 11. Revisión sustantiva e incorporación de la correspondencia con C17 · 16/09/2026

**Seguimiento:** S32, en ejecución. **Referencia:** RETP-2026-249. **Corte receptor:** `e2cd67c5b11c8382ab32532e5c66a63ab2e73eda`. Se incorpora la correspondencia revisada que sigue. Los apartados 1–10, incluida la recepción RETP-2026-248, conservan su texto y su valor histórico; las precisiones de este apartado rigen la lectura actual de los extremos afectados. No se modifica el universo OP-CYB-001 ni se acredita una realización material.

### 11.1. Objeto, fuentes y método de revisión

La entrada es el [parche de correspondencia depositado](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/19bb22c0cb614c7c05184c017e3dc3859a11b1e7/dominios/inmunologia/cambio-rumbo/05-grok-aportes/PARCHE_S32_CORRESPONDENCIA_C17.diff), blob `e8b5e1d7cbb1fef1f84cd87c8f7f946e71cc1de6`, recibido en RETP-2026-248. Su base es `29fbcb022a731aace46a64ca881c02b4b1bcb13a`; la incorporación se reconcilia con el corte receptor posterior, sin aplicar su instantánea registral de revisión 2.

Se han leído íntegramente los Pilares, el acta de perfiles y ensamblaje y el acta de transición desde OP-IMM-001, incluidas sus adendas hasta §30, en el corte receptor. Se han cotejado asimismo el parte, el Acta 001 —especialmente §9—, el estudio de privacidad BIS-03 y las fuentes de estado. El contraste sustantivo utiliza estos documentos de OP-CYB-001, leídos completos en `bbac1b44b1d3b845305e9cde492a08221206d631`:

| Fuente fijada | Identidad del objeto Git | Uso en esta revisión |
|---|---|---|
| [Expediente predecisional v0.4](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/bbac1b44b1d3b845305e9cde492a08221206d631/dominios/ciberseguridad-inteligente/dominio-04-09-26/universos/OP-CYB-001/EXPEDIENTE_PREDECISIONAL_ESTADO_DE_PARTIDA_Y_CONTINUIDAD_OP_CYB_001_v0.4.md) | `602d1c5ad33a27da1eb76ce2e72400de0902f42f` | §2: EP01–EP18; §§3–5: evidencia, relaciones y conservación; §8: constitución de C17. |
| [Ampliación atómica y cobertura v0.3](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/bbac1b44b1d3b845305e9cde492a08221206d631/dominios/ciberseguridad-inteligente/dominio-04-09-26/universos/OP-CYB-001/AMPLIACION_ATOMICA_Y_COBERTURA_OP_CYB_001_v0.3.md) | `80ff6cd7febb0610b2f42e5a5846a84af3b46cba` | §2: P25–P32; §4: significado y condiciones de C01–C16. |
| [Acta de continuidad y relevo](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/bbac1b44b1d3b845305e9cde492a08221206d631/dominios/ciberseguridad-inteligente/dominio-04-09-26/universos/OP-CYB-001/ACTA_CONTINUIDAD_Y_RELEVO_OP_CYB_001_AL_LENGUAJE_SV_2026_09_09.md) | `54d72e794b13f5b785f76121f641768351046a8d` | §§5–6: correspondencias REQ-CYB y RS; §§7–9: soporte y competencia receptora; §12: integridad del consejo y contraste pendiente. |

El método consiste en contrastar cada atribución con su definición y sus condiciones de aplicación, distinguir correspondencias expresas de su aplicación documental a los flujos de §8 y comprobar que no se amplían autoridad, conocimiento ni resultados experimentales. La revisión no constituye una nueva calificación jurídica del producto o de un tratamiento.

### 11.2. C17 y correspondencias de continuidad

El expediente v0.4, §8, constituye C17 como **control compuesto de estado inicial y continuidad**, coordinador de C01–C16 y de las nueve clases documentales de relación definidas en §4: REFIERE, DERIVA_DE, PRECEDE_A, CAUSA_ACREDITADA, CORRIGE, RECIBE_CUSTODIA, DESIGNA, ACEPTA_ENCARGO y REVOCA_O_SUSTITUYE. Se conservan 32 definiciones paramétricas y 18 elementos de contexto; estas cantidades no constituyen células, matrices ni átomos adicionales.

El inventario del expediente asigna expresamente C17 como destino de EP03, EP08, EP09, EP10, EP11 y EP18. Esa lista de destinos no sustituye a los referentes diferenciados del acta de relevo, §6:

| Requisito de OP-CYB-001 | Elementos de contexto citados por el acta, §6 | Distinción que debe conservarse |
|---|---|---|
| RS09 | EP01–EP04; C17 | Referencia aprobada, estado observado y límite del pasado conocido. |
| RS10 | EP10/EP11; C17 | Antecedentes múltiples, extremos y tipos de relación; precedencia y causa diferenciadas. |
| RS11 | EP07/EP08/EP10 | Fuente competente, obligación pendiente, designación y custodia. Su relación con C17 se desarrolla en el expediente, §§2 y 4; no se atribuye esa mención literal a la celda RS11 del acta. |
| RS12 | EP05/EP06/EP09/EP12–EP14/EP18 | Identidades por clase, conciliación, cobertura y reevaluación selectiva. La coordinación de C17 procede del expediente, §§2 y 8. |

### 11.3. Admisibilidad, legitimidad y aplicabilidad

**RS06 distingue admisión, no aplicabilidad, insuficiencia y fallo técnico respecto de Tri.** Sus referentes son C02/C06/C14. C02 exige examinar procedencia, integridad, autenticidad, legalidad de adquisición y permiso para observar; distingue todo ello de la legitimidad del acto observado. Una evidencia admisible puede acreditar un acto no autorizado. Esto no dispensa las condiciones de adquisición ni convierte la admisión en una segunda autorización de ese acto.

La recepción de archivos tampoco equivale a designación ni a aceptación de una obligación. Se aplica el régimen competente y sólo se exige aceptación cuando ese régimen la requiere, conforme al expediente, §§4 y 7. Un fallo técnico no produce Tri.U; la insuficiencia de una evidencia admisible sólo recibe U bajo el contrato correspondiente.

**C16 exige determinar la aplicabilidad mediante contexto acreditado:** entidad, jurisdicción, actividad, función, categoría, dimensiones, fecha y versión de la disposición. No decide por sí mismo que un banco esté libre de datos personales, ni subordina la aplicabilidad de las obligaciones a la conveniencia de una capacidad. C15 exige examinar también originales, vistas y huellas capaces de vinculación. La denominación «artificial», «local» o «federado» no satisface ese examen.

Se mantiene la evaluación desde el diseño establecida en §7.2 y la condición «cuando proceda» de la evaluación de impacto. La selección de una capacidad opcional y la determinación de las obligaciones de su eventual uso son decisiones distintas. No se habilita una interfaz mientras permanezca sin resolver una condición necesaria para ella.

### 11.4. Plan, tiempo y conservación

RS05 conserva plan, razón y permiso, con el parámetro P32 y los controles C08–C12 de OP-CYB-001. Puede examinar una actuación ya ejecutada: P32 compara el acto con la versión del plan fijada antes de ejecutarlo. La falta de previsión no demuestra por sí sola ilicitud. Se preserva la condición particular de C10: su análisis de viabilidad temporal se activa si el consejo contempla una actuación pendiente; no se extiende automáticamente a toda revisión histórica.

RS03 significa **tiempo, revocación y antecedentes**, con P28–P30 y C07 de OP-CYB-001. La síntesis de §3 debe leerse con esta distinción expresa. Se conservan emisión, registro, recepción y efecto, los intervalos y su precisión; no se presume retroactividad.

El expediente, §5, delimita la conservación por adición frente a un adversario y una infraestructura. Encadenar huellas no impide por sí solo la sustitución de la cadena presentada ni prueba un hecho nunca registrado. Tampoco determina conservación ilimitada de datos: C15/C16 mantienen sus exigencias de acceso, retención y supresión o limitación aplicables.

### 11.5. Correspondencia revisada de los diez flujos

**Ámbito de los códigos:** en las columnas siguientes, RS, REQ-CYB, EP y C pertenecen exclusivamente a OP-CYB-001 y a las fuentes fijadas en §11.1. Son correspondencias documentales aplicadas a los flujos del Lenguaje, no nuevos contratos de dominio. Los pares P01–P10 de aceptación conservan su identidad como **casos del estudio de privacidad BIS-03, §7**, en el [corte receptor](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e2cd67c5b11c8382ab32532e5c66a63ab2e73eda/docs/calidad/tuberias-ia/continuacion-15-09-2026/ESTUDIO_PRIVACIDAD_BIS03_2026_09_15.md). No designan los parámetros P01–P32 de OP-CYB-001. Esta precisión desarrolla la regla del Acta 001, §9, sin renombrar códigos históricos.

| Flujo de §8 | Requisitos de OP-CYB-001 pertinentes | Contexto y controles de OP-CYB-001 pertinentes | Alcance y condición conservados |
|---|---|---|---|
| Datos artificiales de prueba | RS04/RS06; REQ-CYB-001/005 | EP15; C02/C06/C15/C16 | Se conserva el banco como objeto de preparación. Deben examinarse procedencia, contenido y metadatos; la exclusión de datos reales debe comprobarse y no deducirse de su etiqueta. |
| Ingesta clínica | RS01/RS02/RS04/RS06/RS07; REQ-CYB-001/003/005/006 | EP01/EP02/EP07/EP15; C01/C02/C03/C05/C06/C11/C14/C15/C16 | Aplicación de obligaciones de soporte, sin constituir conocimiento clínico. Original, transformación, permiso, cobertura y contenido sin autoridad se examinan separadamente. Las necesidades clínicas competen a su dominio. |
| Consulta por persona | RS01–RS05/RS07/RS08; REQ-CYB-001/004/007/009 | EP01/EP05/EP06/EP07/EP15/EP17; C02/C07/C08–C13/C15/C16 | Identidad, permiso, finalidad, destinatario y tiempo. C08–C12 conservan sus condiciones particulares: una consulta no exige por sí sola un plan operativo ni una ventana de intervención. |
| Inferencia local | RS04/RS06/RS08; REQ-CYB-001/003/009 | EP15/EP16/EP18; C02/C06/C13/C14/C15/C16 | Fuentes y reglas admitidas, aislamiento del caso, admisión y explicación. La correspondencia no acredita aptitud de un modelo ni permite promoción autónoma de conocimiento. |
| API externa | RS02/RS04/RS05/RS06/RS07; REQ-CYB-001/002/003/006/007 | EP07/EP12/EP15/EP16; C02/C05/C11/C14/C15/C16 | Opción condicionada de §8, sin conector habilitado. La petición humana, el permiso de salida, la admisión de la respuesta y el régimen del tercero son objetos distintos. |
| Consulta estadística federada | RS04/RS06/RS07; REQ-CYB-005/006/008 | EP12/EP15/EP17; C05/C06/C11/C15/C16 | Opción condicionada a una necesidad justificada, no constituida. Cobertura, sensibilidad, magnitudes y revelación acumulada requieren contrato; la necesidad no decide la aplicabilidad jurídica. |
| Entrenamiento federado | RS05; REQ-CYB-007, para separar consejo y autoridad | EP16; C14/C15/C16 | Capacidad excluida de la realización actual por §§7–8 y estudio, §6. La correspondencia fija esa frontera; no inicia diseño o entrenamiento. |
| Frame, documento o exportación | RS04/RS07/RS08; REQ-CYB-009 | EP15/EP18; C05/C11/C13/C15/C16 | Generación, presentación y difusión tienen condiciones propias. Se conserva el estatuto de informe si falta arquitectura que legitime un Frame; la transformación no elimina restricciones. |
| Telemetría CYB | RS01–RS04/RS06/RS07/RS09–RS12; REQ-CYB-001/002/004/005/006/008 | EP03/EP04/EP05/EP06/EP07/EP08/EP09/EP10/EP11/EP12/EP13/EP14/EP15/EP18; C01/C02/C05/C06/C07/C11/C15/C16/C17 | Se conservan los grupos diferenciados de §11.2 y la cobertura del caso. La presión del universo no acredita una plataforma ni una investigación forense general. |
| Registros, respaldos e historia | RS03/RS04/RS06/RS09–RS12; REQ-CYB-001/004/005/008 | EP03/EP07/EP08/EP09/EP10/EP11/EP15/EP18; C02/C04/C06/C07/C13/C14/C15/C16/C17 | Original, vista, obligación, custodia, tiempos y restauración. Los plazos y reglas del flujo siguen sin constituirse; no hay oráculo material de conservación acreditado. |

La aplicación por flujo se apoya asimismo en las seis correspondencias expresas del acta de relevo, §12.4: procedencia y cobertura (C01/C02/C03/C05/C06; REQ-CYB-001/003/005/006; RS06/RS07/RS12); instrucción, dato y facultad (C02/C11/C14; REQ-CYB-007; RS01–RS03/RS05); explicación (C13; REQ-CYB-009; RS08); versión e historia (C07/C14/C17; REQ-CYB-001/004; RS09–RS12); destinatario y salidas (C05/C11/C13/C15/C16; RS04/RS07); y revisión y obligaciones (C08–C13/C17; REQ-CYB-007/009; RS08/RS11). Se conservan sus condiciones; no se exige activar cada control en todos los flujos.

REQ-CYB-005 requiere sensibilidad propia del criterio y del observador. REQ-CYB-008 exige conservar literal, tipo, unidad, rango y exactitud cuando intervengan magnitudes, también en tiempos y cuotas. Sus referencias en la tabla no acreditan esas comprobaciones ni agotan su aplicabilidad transversal.

La tabla mantiene las sedes funcionales de §8 y sus brechas, sin presentar como localizados componentes e interfaces todavía no constituidos. Los flujos comunes siguen bajo estudio en S32; la API externa y la consulta federada mantienen su selección condicionada, y el entrenamiento federado permanece excluido de la realización actual. No se transforma una opción en obligación de construirla. Para cerrar el alcance de §5 habrá que resolver expresamente la inclusión o exclusión de las opciones aún condicionadas y enlazar cada realización necesaria diferida con su seguimiento y condición de habilitación.

### 11.6. Resultado del examen adversarial y disposición del parche

| Extremo examinado | Decisión incorporada |
|---|---|
| Fuente constitutiva de C17 | Confirmada en el expediente v0.4, §8, con desarrollo en §§2–5. |
| Correspondencia conjunta de RS09–RS12 | Precisada por requisito; no se sustituye cada grupo por una lista común de elementos EP. |
| Admisión y autorización | Sustituida la denominación «dos autorizaciones» por la distinción de §11.3; conservadas las condiciones de adquisición de C02. |
| C16 y datos artificiales o federación | Retirada toda conclusión automática por nombre del banco o conveniencia del flujo; se exige el contexto definido por C16. |
| RS05 y revisión de actos ya ejecutados | Aceptada con la condición de C10 y la definición de P32 preservadas. |
| Correspondencia de controles, contexto y requisitos | Desarrollada en §11.5 con las fuentes de §§5–6 y §12.4 del acta; aplicación documental separada de ejecución. |
| Códigos de aceptación y parámetros | Conservados y calificados por ámbito, tipo, fuente y corte, conforme al Acta 001, §9. |
| Revisión 2 y campos registrales vacíos del candidato | No incorporados. La revisión 2 efectiva sigue siendo RETP-2026-248; esta incorporación ocupa la revisión 3, con fecha efectiva y RETP-2026-249. |

**Dictamen:** revisión sustantiva documental ejecutada; correspondencia aceptada con las precisiones incorporadas en este apartado. La aceptación comprende las atribuciones y distinciones aquí examinadas. No valida contratos inexistentes, todos los requisitos de privacidad del sistema ni la ejecución de los diez pares. El parche depositado conserva sus bytes; la versión incorporada es este apartado y su actualización registral concordante.

### 11.7. Estado, evidencia y continuación

Se cotejan las fuentes de estado en el corte receptor: [Sucesos SV](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e2cd67c5b11c8382ab32532e5c66a63ab2e73eda/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md) para S32, S22 y S26, en ejecución, y S29, finalizado en su alcance histórico; y [ESTADO_WORKFLOW.json](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e2cd67c5b11c8382ab32532e5c66a63ab2e73eda/docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/ESTADO_WORKFLOW.json), blob `958c0738f60c8cac49bc613b17133fabd1c207a0`, para BIS-00/01, finalizado; BIS-02/03/04, en ejecución; BIS-05/06/07/08, pendiente. El estudio BIS-03, §7, y este parte delimitan el recuento: cero ensayos de privacidad ejecutados. Las remisiones del workflow no sustituyen a los asientos vigentes de Sucesos.

La conservación de los documentos previos y la concordancia registral se comprueban con [el cotejo Rust de esta incorporación](COTEJO_REVISION_SUSTANTIVA_S32.rs) y [su resultado](COTEJO_REVISION_SUSTANTIVA_S32_SALIDA.txt). La identidad de las fuentes permite reconstruir qué justificó la revisión; el cotejo mecánico acredita edición e integridad, no corrección semántica por sí solo. Rust 1.98.0 y su biblioteca estándar nativa bastan para este auxiliar; no requiere dependencias externas de Cargo. El conector GitHub transporta los documentos; Git comprueba sus identidades. No se ejecutan campañas SV, Q1/Q2 ni E1–E16.

**Siguiente actuación de S32:** constituir el contrato de los trayectos comunes de consulta, autorización, salida y persistencia, con componente e interfaz identificados, permisos y condiciones de aplicabilidad, plazos y restauración, resultados esperados y observador. Cada prueba futura conservará el enlace al requisito y apartado que la motiva, contrato y versión, entradas, resultado esperado, aceptación y resultado efectivo, conforme a la recepción RETP-2026-248. Las opciones condicionadas se resolverán expresamente antes del cierre documental de §5. Esta incorporación termina su revisión de correspondencia; S32 y BIS-03 permanecen en ejecución.

<a id="s32-recepcion-h1-2026-09-17"></a>

## 12. Recepción y revisión documental de TLC-S32-02 v0.1 + H1 · 17/09/2026

**Seguimiento:** S32, revisión 4, en ejecución. **Referencia:** RETP-2026-250. **Corte receptor:** `03578e3c3919d38ed1ae1dbc686d36ea9147c0b6`. Se han consultado íntegramente los Pilares de 05/09/2026, el Acta de perfiles y ensamblaje de 06/09/2026 y el Acta de transición desde OP-IMM-001 de 03/09/2026, con su secuencia y recepciones hasta §30, conforme a AGENTS.

### 12.1. Objeto y procedencia

Se recibe como propuesta documental el paquete H1 del depósito privado `SV-sala-de-maquinas`, commit `75f5e440b9dd8461889ec4083df9763231284d06`. Se ha comprobado el acceso desde la conexión receptora, su visibilidad privada y el árbol publicado, que contiene 34 archivos. El cotejo con el sistema local de producción de esos 34 archivos consta en el informe de entrega; esta recepción no ha accedido directamente a aquel sistema.

Los siete archivos de `TLC-S32-02/originales/` conservan los mismos identificadores de blob que la carpeta de procedencia en `SVperitus-dataset`, commit `bba2d3ae24cdc20e33b90375f295916928011985`. Se ha contrastado esa igualdad entre ambos árboles inmutables. La propuesta no sustituye esos originales.

| Objeto recibido | Blob Git |
|---|---|
| [Contrato candidato H1](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/75f5e440b9dd8461889ec4083df9763231284d06/TLC-S32-02/propuesta/CONTRATO_CANDIDATO_TLC_S32_02_v0.1.md) | `9d9f96a84961a28d2e319ba41d47934a41f87d72` |
| [Casos previstos H1](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/75f5e440b9dd8461889ec4083df9763231284d06/TLC-S32-02/propuesta/CASOS_PREVISTOS_TLC_S32_02_v0.1.md) | `779708a8f9da852c4675e961c2c2c24ea475b4cc` |
| [Auxiliar Rust](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/75f5e440b9dd8461889ec4083df9763231284d06/TLC-S32-02/propuesta/COTEJO_ENTREGA.rs) | `34306db98c93aa5de52c03b4aa5178c6dd83154b` |
| [Referencias del cotejo](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/75f5e440b9dd8461889ec4083df9763231284d06/TLC-S32-02/REFERENCIAS_H1.tsv) | `e963ea3646005f5b22b0388d724392958dd49f3e` |
| [Nota de entrega](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/75f5e440b9dd8461889ec4083df9763231284d06/TLC-S32-02/propuesta/NOTA_ENTREGA.md) | `6324faf5bf57331d1b4c0c547298bc3a0ec18b8e` |
| [Diferencias respecto del original](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/75f5e440b9dd8461889ec4083df9763231284d06/TLC-S32-02/PROPUESTA_H1.diff) | `536a7827862a4560699e16315c92b26868c9238f` |
| [Registro de ejecución instrumental](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/75f5e440b9dd8461889ec4083df9763231284d06/REGISTRO_CONTINUIDAD_2026-09-17.txt) | `d29bba7d17b1f656c626838ff614a22f9a9d3de6` |

Los enlaces del depósito requieren acceso autorizado. Su carácter privado se conserva; esta recepción pública no reproduce sus registros íntegros ni sus datos de configuración local. Las huellas delimitan los objetos examinados, sin acreditar por sí solas su corrección semántica.

### 12.2. Examen de las seis correcciones propuestas

| Extremo | Resultado de la lectura receptora |
|---|---|
| Recuento | Los encabezados identifican 19 casos: cinco positivos —TLC-01, 03, 05, 07 y 13— y catorce negativos. Los diecinueve conservan el estado «no ejecutado: implementación pendiente». |
| Precondiciones de los positivos | La configuración común §1.1 explicita autoridad sintética separada de la petición, delegación, identidad, sesiones, vigencia, cuota, contexto y observación. Es una especificación de ensayo; no acredita instituciones ni componentes reales. Persiste la precisión temporal de §12.4. |
| Egreso prohibido, TLC-08 | Se amplía el criterio a errores, cabeceras, URL, depuración, registros y respuesta, con contenido prohibido y transformaciones. La cobertura insuficiente se considera no concluyente. El observador material sigue sin constituirse. |
| Restauración y supresión, TLC-11 | Se separan los brazos con y sin supresión, con permisos suficientes, control previo y cuotas propias. Una denegación ordinaria deja de bastar como evidencia de supresión. Un bloqueo por continuidad conserva exclusivamente ese alcance. |
| Auxiliar de cotejo | Distingue identidad y estructura. Seis referencias fijadas fuera del directorio examinado se incorporan al compilar. La identidad se compara mediante Git; las comprobaciones estructurales no equivalen a interpretación completa del contrato. |
| Fallo de evidencia posterior a la salida | El contrato §9.4 distingue entrega confirmada, efecto indeterminado y ausencia de efecto acreditada; prohíbe presumir reversión o reintento automático. Exige evidencia durable previa cuando proceda, conforme a R2-0 §§10–12 y su recepción R06. No acredita implementación de esa persistencia. |

Se reconoce el tratamiento documental de las seis objeciones. Esta conclusión no constituye aceptación integral del candidato ni habilitación del trayecto.

### 12.3. Evidencia instrumental y límites

El registro recibido contiene una compilación final del auxiliar con Rust 1.98.0, destino Windows MSVC, terminación 0 y salida de error vacía. En la serie final, los modos `identidad`, `estructura` y `todo` aceptan el paquete previsto, con terminación 0. Las cinco mutaciones previstas se rechazan en modo `todo`, con terminación 1:

- cambio de identidad de la nota manteniendo su tamaño;
- resumen de recuento incompatible con los encabezados;
- promoción indebida de un caso a «superado»;
- ausencia del bloque TLC-19;
- duplicación del campo de ejecución.

Las cuatro mutaciones estructurales se rechazan también en modo `estructura`. La modificación de fecha que sólo afecta a identidad pasa ese modo, resultado coherente con la separación declarada. No se describen los cinco rechazos como ensayos de privacidad ni como prueba exhaustiva de sensibilidad del auxiliar.

El registro conserva intentos anteriores, incluido un rechazo inicial de clasificación de TLC-15, y diferencia la recompilación final. Las salidas históricas `COTEJO_ENTREGA_SALIDA.txt` y `ENTORNO_MINIMO_SALIDA.txt` copiadas en la propuesta pertenecen al paquete original ejecutado en Linux. No constituyen salidas del auxiliar H1 en Windows.

La verificación receptora consiste en lectura del código, las referencias y los registros, recuento de casos y contraste de metadatos Git. No se ha repetido aquí la compilación ni la ejecución del auxiliar. La confianza en su fuente, sus referencias y el Git invocado es externa al propio cotejo. No se acredita una auditoría criptográfica independiente.

### 12.4. Precisiones pendientes antes de la aceptación integral

**H1-01 · Dependencia durable en la configuración temporal.** El contrato §9.4 exige evidencia suficiente antes del efecto. La configuración común sólo enumera admisión en tick 3, efecto en 4, salida en 5 y persistencia en 6. Esa secuencia describe el cierre posterior, pero no fija el estado durable previo, su confirmación ni el criterio que impediría el efecto si faltase. Antes de materializar o aceptar íntegramente el banco se debe explicitar esa precondición en las entradas y el oráculo, o declarar expresamente qué capacidad se simula y el alcance limitado de tal simulación. Deben quedar distinguibles fallo previo al efecto, fallo posterior a entrega y resultado indeterminado; no se amplía retrospectivamente el recuento de diecinueve casos.

**H1-02 · Referencias y sede del derivado.** Los enlaces relativos del contrato §1 a TLC-S32-01 v0.1 y v0.2 se han heredado de la carpeta de origen y no resuelven dentro del árbol H1 publicado. El §13 conserva además la descripción del depósito original. La siguiente revisión debe fijar enlaces a los commits de procedencia y distinguir de forma inequívoca la sede del original y la del derivado. Se conserva H1 sin alteraciones; no se corrigen silenciosamente los objetos recibidos.

Estas precisiones se siguen dentro de S32. La falta de un observador ejecutable, los controles institucionales y las interfaces pendientes de los apartados 5 y 8 mantienen sus efectos propios, aunque se subsanen las referencias.

### 12.5. Disposición y siguiente actuación

**Dictamen:** recepción documental comprobada; seis correcciones examinadas; aceptación integral de H1 pendiente de H1-01/H1-02 y del contraste contractual que corresponda al alcance a constituir. No se aplica el diff al Lenguaje ni se incorpora código de la propuesta mediante esta recepción.

Se mantienen pendientes E1–E4 del trayecto, ligadura de autorización con EP07, sesiones, cuotas, contexto C16 del uso, plazos reales, observadores y realizaciones R2/R3. API externa y consulta federada conservan su selección condicionada; el entrenamiento federado permanece excluido. Los nombres de etapa E1–E4 de este contrato no designan los casos E1–E16 de cualificación de leyenda.

**Recuento material:** cero ensayos de privacidad ejecutados; ningún cierre de S32, BIS-03, S22 o S26. Q1/Q2 y E1–E16 conservan su estado. La preparación del entorno receptor se sigue separadamente en S33, conforme al [Acta 001, §10](ACTA_001_CONTINUIDAD_Y_RUMBO_2026_09_15.md#continuidad-entornos-2026-09-17).

La siguiente revisión debe conservar las fuentes recibidas, presentar diferencias explícitas para H1-01/H1-02 y volver a fijar las referencias del auxiliar antes de recompilarlo. La subsanación documental no inicia por sí sola una campaña de privacidad.

<a id="s32-recepcion-h2-2026-09-17"></a>

## 13. Recepción de H2 y subsanación documental de H1-01/H1-02 · 17/09/2026

**Seguimiento:** S32, revisión 5, en ejecución. **Referencia:** RETP-2026-254. **Unidad receptora:** W-S32. **Corte canónico de entrada:** `79ee51dd659e6b2906cb662ffcab6b4d9cd6f40b`. Se conservan íntegros los apartados anteriores y su alcance histórico.

### 13.1. Objetos recibidos y preservación

Se recibe la [entrega H2](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/a47d53a703994acfee08f2b6fd4d1d63f04341d0/respuestas-ejecucion/TLC-S32-02-H2/entrega-01/RESPUESTA.md), publicada en `a47d53a703994acfee08f2b6fd4d1d63f04341d0`, con productos fijados en `6d616d5f6e7c167413003ce74e04c0cbf66a32ed`. La sede privada conserva acceso restringido.

| Objeto H2 | Blob Git |
|---|---|
| Contrato candidato | `b70e4a0c6b6b0fc241744be90da542bd7a419d4b` |
| Diecinueve casos previstos | `3fd9ab8195221f72371c5fe513ee8c6897ae46ed` |
| Auxiliar Rust sin modificar | `8e658bd8c4ac879d72bbf45ffb1af735bdfb05a9` |

El [informe receptor y sus evidencias](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/1a3e197b584eeb9f5269ce5a9449c857e20d7493/watson-herramientas/evidencias/receptor-h2-20260917/README.md) conservan las identidades completas. Se verificaron los 23 objetos enumerados en el manifiesto, además de la identidad del propio manifiesto. La comparación de árboles desde `bd99f96b34dca442db7999d94bed695377c0ec82` registra 89 de los 91 blobs anteriores idénticos, dos índices modificados, 24 altas y ninguna baja. Originales, H1, encargos e históricos mantienen sus blobs. No se afirma acceso al disco productor ni descarga de los 115 contenidos del árbol final.

### 13.2. Examen de los reparos

**H1-01: subsanado documentalmente.** El contrato §9.4 y las entradas y oráculos de los casos §§1.1–1.2 distinguen preparación, persistencia, confirmación durable y revalidación antes de la lectura protegida; efecto, entrega y cierre posterior E4. La confirmación en memoria no basta. La secuencia temporal no acredita por sí sola causalidad. El modelo de fallo, la independencia necesaria frente a recuperación y el observador material permanecen por constituir. Ausencia de prueba, entrega confirmada y resultado indeterminado conservan consecuencias diferentes, conforme a R2-0 §§10–12 y sus recepciones R02/R06. No se infiere reversión ni reintento automático.

**H1-02: subsanado documentalmente.** Los enlaces de TLC-S32-01 v0.1 y v0.2 se fijan al dataset `bba2d3ae24cdc20e33b90375f295916928011985`; se comprobaron por lectura y blobs `d0d25ba1f953a751d3b346d29015f5f7e7e4b500` y `6f2ba710da9ffdb1b2dee84b0df0b006111eae0f`. Se distinguen original, derivado, base semántica histórica y cortes posteriores de revisión.

**Dictamen:** aceptación de la subsanación de ambos reparos en el alcance documental examinado. H2 conserva estatuto de contrato candidato; esta recepción no constituye aceptación integral del trayecto, congelación de interfaz ni incorporación de código al núcleo.

### 13.3. Reproducción instrumental receptora

Rust 1.98.0, Linux x86-64, auxiliar productor sin modificar y referencias externas originales. Compilación con retorno 0. Se reconstruyeron en Rust las cinco mutaciones prescritas y se cotejaron las 46 entradas congeladas antes y después de ejecutar. Las 18 combinaciones de seis variantes por tres modos concuerdan con el plan previo: base 0/0/0; M1 1/0/1; M2–M5 1/1/1 para identidad/estructura/todo. Se inspeccionaron los diagnósticos y corresponden a los defectos documentales previstos.

Las huellas acreditan identidad respecto de entradas fijadas; no son firma independiente ni demostración semántica. El adaptador receptor usa Rust y herramientas de identidad Git/sha256sum. No se ejecutó PowerShell ni se acredita identidad entre entornos Windows y Linux. La prueba demuestra la aptitud de esta instancia receptora para este auxiliar; no disponibilidad permanente.

### 13.4. Estado y continuación

Diecinueve casos previstos: cinco positivos y catorce negativos; **ninguno ejecutado**. Durabilidad especificada, no implementada ni acreditada. No se ejecutan Q1/Q2, E1–E16 ni campañas de privacidad. S32 y BIS-03 permanecen abiertos; S22 y S26 no reciben nuevos ensayos ni cambian de estado. S34 conserva su recepción instrumental propia pendiente.

Siguiente actuación propuesta: matriz de cobertura residual de S32 sobre los diez flujos de §8 y la condición de cierre de §5. Cada obligación debe identificar componente, contrato, prueba exigida, situación, seguimiento concreto y condición de habilitación. Las opciones API externa y consulta federada requieren decisión expresa; entrenamiento federado sigue excluido. Ningún aplazamiento se considerará resuelto sólo por llamarlo «pendiente».

El encargo se preparará en la sede de encargos y permanecerá pendiente de autorización humana de su revisión exacta. La respuesta se publicará por separado, conservando la copia local. Esta recepción no ordena su ejecución ni cierra (p1+p3)-Bis.

<a id="s32-recepcion-cobertura-2026-09-17"></a>

## 14. Recepción de S32-COBERTURA-01/v1 · 17/09/2026

**Seguimiento:** S32, revisión 6, en ejecución. **Referencia:** RETP-2026-255. **Unidad receptora:** W-S32. **Entrada canónica:** `e1ab93d0de3df170a674f5a7887a3c20e96c6816`. Se conservan íntegros los apartados anteriores.

### 14.1. Objeto y criterio

Se examina la [respuesta entrega-01](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/8bb4d41fd4138c7f1a6c246ac952d5004ffd7479/respuestas-ejecucion/S32-COBERTURA-01/entrega-01/RESPUESTA.md) y los [cuatro productos](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/tree/c1bbc5cf7049913fafa72feea9a8cfb8d0d4bbc7/S32-COBERTURA-01/v1), frente al [encargo v1](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/e61b99dda6c2a1a62c8abf9ce40d160ab3c4f6ca/encargos-ejecucion/S32-COBERTURA-01/v1/ENCARGO.md). Son referencias privadas; esta recepción no cambia su visibilidad.

La ficha e historial registran autorización de la revisión exacta y entrega por UE-LOCAL-CODEX-WINDOWS. La comunicación humana recibida aporta la entrega para revisión. No se atribuye observación directa de la sesión o del disco productor.

**Dictamen técnico receptor:** conforme al objeto documental del encargo. Se recibe la matriz como referencia de cobertura residual y el inventario de decisiones y carencias como base de continuación. No se identifica un reparo que exija rehacer esta entrega. La conformidad no adopta las alternativas reservadas a la Dirección, no constituye los contratos faltantes y no declara cerrado S32.

### 14.2. Identidad y preservación comprobadas

| Producto | Bytes UTF-8 | SHA-256 |
|---|---:|---|
| MATRIZ_COBERTURA.md | 32398 | `cea2f205fa1ace4177a426ef7d105c6d1e6f28c51db57c1bc5dd7f0d419c4c23` |
| DECISIONES_Y_PENDIENTES.md | 29080 | `e9d5821941ccb5dd8a9df28f42df76f11f433721aa825739057778078ebcf3e6` |
| FUENTES.tsv | 8443 | `75abb8eebf36d75accadeab463266c5c2ac4dc7a6f5002515b25634a0bdd6803` |
| MANIFIESTO_SHA256.tsv | 289 | `3ddcfdfa0d762b0ade38f997d451198480adebaffeae92952e2c3d5673cc4b3b` |

Se recalcularon tamaños, SHA-256 y blobs Git de los cuatro contenidos recuperados; las tres entradas del manifiesto coinciden y sus cuatro blobs son los fijados por los árboles de productos y entrega. La huella del manifiesto se registra aquí, fuera de él. Las 34 referencias con blob declarado en FUENTES.tsv resuelven al objeto indicado en su repositorio, commit y ruta; las dos remisiones sin comprobación declarada mantienen ese límite. Identidad de fuente no significa relectura semántica íntegra de sus 34 contenidos.

Comparación privada `e61b99d…` → `8bb4d41…`: 124 blobs iniciales; 120 idénticos, cuatro modificaciones administrativas y cinco altas; ninguna eliminación. Las modificaciones corresponden exclusivamente a ficha, historial e índices de encargos/respuestas. Las altas son los cuatro productos y la respuesta. Se conservan, entre otros, el cuerpo autorizado, H1/H2, herramientas y registros anteriores. Los cuatro productos mantienen identidad entre `c1bbc5c…` y `8bb4d41…`. Se comprueban árboles y contenidos de esta entrega, no todos los archivos del equipo productor.

Cotejo administrativo de identidad mediante las funciones SHA-256/SHA-1 de Node.js y los árboles Git recuperados; sin auxiliar experimental ni ejecución SV. No se repite la compilación H2 ni se atribuyen aquí sus 18 comprobaciones como nuevas.

### 14.3. Revisión sustantiva y límites

Se releen AGENTS y los tres rectores completos —Pilares, perfiles y transición §§1–30— del corte canónico de entrada. Se contrastan el encargo, los productos, las condiciones de S32 §§4–5/8/11.5/13, Acta 001 §§1–4/10, los casos H2, la continuidad F01/F02, los registros vigentes pertinentes y el procedimiento S36. La correspondencia CYB se coteja con la ya recibida en §11.5; no se afirma una nueva auditoría integral del dominio.

| Extremo examinado | Resultado receptor |
|---|---|
| Perímetro de la matriz | Conserva diez flujos y doce columnas; las seis puertas comunes y las tablas TLC/CYB completan la misma correspondencia. |
| Diecinueve TLC | Coinciden identificadores, cinco positivos, catorce negativos y límites del trayecto local. Ninguno ejecutado. P05 carece de TLC en H2; TLC-09 no acredita federación ni presupuesto de privacidad. |
| Evidencia durable | Se conserva la recepción documental de H1-01/H1-02. Compromiso y confirmación previos al efecto permanecen distintos de cierre E4; no se acredita realización durable. |
| Observación | Una denegación por falta de plataforma no prueba la causa negativa específica. TLC-11 requiere controles recuperables y comparación causal; TLC-08/14 exigen cobertura de contenido y canales, no sólo un marcador. |
| Seguimientos A–I | Son agrupaciones internas de brechas propuestas. «Seguimiento concreto por constituir» declara una carencia: no satisface §5, no asigna una unidad ni autoriza trabajo. |
| Sedes anteriores | F01/F02 conserva su alcance de referencia/consulta exacta y su pendiente material. S28 está finalizado documentalmente según Sucesos; su cierre no constituye agentes. S22 y S26 no reciben nuevos encargos por esta matriz. |
| Opciones y autoridad | API externa y consulta estadística federada conservan alternativas separadas pendientes de decisión. Entrenamiento federado permanece excluido. Roles propuestos no sustituyen autoridades institucionales reales. |
| Condiciones de cierre | Los siete pasos de Decisiones §4 ordenan la resolución; no son siete logros consumados ni nueve campañas abiertas por A–I. |

No se han ejecutado pruebas de privacidad, Q1/Q2, E1–E16 ni campañas R2/R3. La recepción es documental y no incluye actualización legislativa ni certificación jurídica. No se declara ausencia exhaustiva de defectos fuera del alcance examinado.

### 14.4. Disposición de continuidad

La tarea S32-COBERTURA-01 queda técnicamente recibida en su objeto documental. La Dirección conserva la decisión sobre las alternativas y el encargo siguiente. S32 continúa en ejecución y BIS-03 abierto; H2 sigue candidato, sin aceptación integral ni habilitación. Se mantienen las 19 pruebas previstas sin ejecutar y la durabilidad sin implementación acreditada.

El siguiente paso es fijar, con decisión expresa y motivación, el perímetro que deberán cubrir los contratos: modalidades general/particular/investigación y, por separado, API externa y consulta estadística federada. Para cada opción: retención condicionada con obligaciones y seguimiento, o exclusión motivada del alcance actual con control del límite y posibilidad de reconsideración futura. No se adopta ninguna alternativa en esta recepción. La exclusión del entrenamiento federado permanece vigente.

Después corresponde un encargo acotado a contratos y asignación de las obligaciones retenidas, aprovechando las sedes existentes. Cada realización diferida necesaria debe identificar responsable competente, producto, dependencia, seguimiento y condición de habilitación; no basta renombrar A–I como tareas. No se abre por defecto H3 ni se encarga simultáneamente todo el inventario. La publicación del futuro encargo no autoriza su ejecución: rige S36 y la aprobación humana de su revisión exacta.

Se conserva el orden de continuidad comunicado: concluir (p1+p3)-Bis; revisar el encaje y lo pendiente de incorporación al núcleo desde el 15/09, con revisión experimental reforzada antes del retorno; recuperar el punto documentado de la secuencia e Inmunología; abordar posteriormente agentes, interfaz experta e IA cuando correspondan sus contratos. Esta recepción no activa esos frentes. La unidad documental pública mantiene su ámbito separado y no recibe encargos mediante este dictamen.

Los registros canónicos y sus índices se actualizan conjuntamente. Los espejos históricos conservan su corte; no se afirma sincronización nueva ni se duplican paquetes o informes para la revisión humana.

<a id="s32-recepcion-aux-duckdb-ncbi-2026-09-17"></a>

## 15. Recepción instrumental del auxiliar y alcance por dominios · 17/09/2026

Fecha de registro: 2026-09-17T11:54:22Z. Unidad W-S32; S32 revisión 7; RETP-2026-256.

Se recibe AUX-DUCKDB-NCBI-01/v1 como preparación instrumental recuperable en el perfil Linux observado. D01–D05 aportan siete subcasos conformes y D06 acredita recuperación y recompilación propias con Rust 1.98.0, dependencias vendorizadas y --locked --offline. Las huellas de los ZIP y las 4192 entradas del manifiesto interior concuerdan. Se conserva la incidencia de extracción y su suplemento documentado, sin modificar originales ni reconstruir registros. La evidencia detallada permanece en [la recepción privada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/bab99267318baccd1f8bbc951ff09d2505f72b43/watson-herramientas/evidencias/receptor-aux-duckdb-ncbi-20260917/RECEPCION_LINUX.md); su acceso requiere autorización al repositorio.

Esta aceptación no habilita el auxiliar en producción ni acredita confinamiento completo. La FFI del auxiliar comparte proceso con la biblioteca nativa y exige límites materiales adicionales. La recuperación Linux no requirió .NET; su uso observado corresponde al empaquetado. La supervisión, la identidad de dependencias, la política del transporte externo y los límites de recursos/salida requieren contrato, realización y pruebas antes de integrar. La recepción privada conserva las observaciones técnicas y cuatro pendientes locales de contención, sin constituir nuevos sucesos ni órdenes de ejecución.

La Dirección precisa que el acceso externo y la consulta federada deben contemplar inmunología y ciberseguridad, conforme al currículo vigente de Universo1 y a futuras ampliaciones. NCBI es una fuente biomédica particular; no define la arquitectura general. Se requiere un contrato común y perfiles de autorización por dominio, finalidad y expediente. En auditoría de ciberseguridad deberán delimitarse titularidad, datos autorizados, destinos, operaciones, minimización, retención y trazabilidad depurada antes de cualquier salida. Las consultas y los agregados también pueden revelar información: su comunicación requiere evaluación específica. No se habilita el envío de datos sensibles, secretos o evidencias de terceros a servicios externos, modelos remotos o sedes públicas por la sola disponibilidad del auxiliar. Las pruebas posteriores se delimitarán con datos sintéticos o expresamente autorizados. Esta precisión es requisito pendiente, no control ya implementado ni auditoría integral del currículo.

Windows y NCBI conservan evidencia productora con atribución expresa; no se han repetido en esta recepción. La clave NCBI no se utilizó ni validó. Los diecinueve TLC siguen sin ejecutar, H2 conserva estatuto candidato y la durabilidad sigue sin implementación acreditada. S32 continúa en ejecución y BIS-03 abierto; S22, S26, S34 y otros frentes conservan sus estados. API externa y consulta estadística federada mantienen decisiones y condiciones propias; la exclusión del entrenamiento federado continúa vigente. No se acredita integración Qwen, navegador o WASI.

El siguiente encargo deberá acotar los contratos y realizaciones necesarias sobre estas sedes y reservar la ejecución a la autorización humana de su revisión exacta, conforme a S36. La autorización de adoptar DuckDB como auxiliar no equivale a autorizar todos los flujos o fuentes de datos. Se preservan originales, antecedentes, copia local de trabajo y paquetes privados publicados; no se exige descarga duplicada ni se afirma persistencia de la instancia receptora. Los registros e índices canónicos se actualizan conjuntamente; los espejos históricos conservan su corte.
