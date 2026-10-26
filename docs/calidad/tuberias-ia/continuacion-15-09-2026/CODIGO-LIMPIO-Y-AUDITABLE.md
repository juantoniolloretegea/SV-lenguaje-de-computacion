# Código limpio y auditable

Norma del Sistema Vectorial SV · Versión 1.10 · 26 de octubre de 2026. Entrada en vigor: 18 de octubre de 2026.

**Carácter obligatorio desde el 18 de octubre de 2026.** Se aplica al código Rust final del SV, a sus bibliotecas propias, ejecutables, adaptadores, persistencia y comprobaciones. La entrada en vigor no certifica retrospectivamente todos los componentes existentes. Cada entrega debe identificar su alcance comprobado y las desviaciones pendientes; ninguna excepción tácita se considera conformidad.

## 1. Autoridad, contratos y organización

Cada módulo declara finalidad, entradas no fiables, salidas, efectos, límites y dependencias. Las decisiones científicas, los permisos humanos y los resultados instrumentales tienen tipos y recorridos diferentes. Un error de red, capacidad o almacenamiento nunca se transforma silenciosamente en 0, 1 o U, ni en un dictamen de aptitud. Se conserva una sola terna vigente y un solo contrato de polígono, aplicado a la dimensión de la célula.

El dominio es titular de una asignación vigente: dominio → nodo → modelo. Compartir modelo o infraestructura no comparte datos, permisos, aptitud ni conversaciones. Las herramientas auxiliares de traducción quedan fuera de los nodos de inferencia experta. La preparación no habilita producción.

Una regla debe tener una única realización competente, reutilizada por sus entradas. Se evitan servicios, adaptadores o verificadores paralelos que reinterpreten la misma decisión. Las representaciones de interfaz son proyecciones de reglas Rust y no autoridades alternativas. Los programas auxiliares de edición o construcción no deciden la semántica del SV.

## 2. Estilo, legibilidad y contratos de programación

Se utiliza la edición Rust declarada y una versión mínima comprobada. `rustfmt` normaliza los archivos propios. Clippy se aplica con sus reglas recomendadas y una selección justificada de reglas adicionales; no se activa indiscriminadamente toda la categoría `restriction`. Cada excepción se limita a su expresión o módulo y explica su necesidad. No se silencian advertencias globalmente para obtener una comprobación aparentemente limpia.

Los nombres siguen las convenciones Rust: módulos, funciones y campos en `snake_case`, tipos en `UpperCamelCase`, constantes en `SCREAMING_SNAKE_CASE`. Los nombres describen significado y unidades. Las identidades persistentes existentes no se renombran por estilo sin migración explícita. No se utiliza la oscuridad de nombres como protección.

Los comentarios necesarios expresan **por qué existe una condición**, qué frontera protege y qué compatibilidad conserva. Las fronteras, invariantes, formatos persistentes y operaciones privilegiadas tienen explicación breve en español e inglés, con el mismo significado. No se traduce mecánicamente cada línea ni se duplica la documentación del lenguaje. La documentación extensa se enlaza desde el código.

Se prefieren tipos y enumeraciones que impidan estados contradictorios; `Option` representa ausencia, no cero. Los resultados recuperables se devuelven mediante `Result`, con causas estables y ayuda comprensible. Se restringen `unwrap`, `expect` y `panic` en recorridos que dependen de datos o recursos externos; su uso excepcional requiere una invariancia demostrada. Los errores no publican claves, contenidos, rutas privadas ni respuestas completas de terceros.

### 2.1. Autoría y licencia de cada implementación

Desde el 25 de octubre de 2026, toda implementación de código propio del SV, nueva o modificada, debe incorporar en cada archivo el aviso literal de autoría y licencia siguiente. La obligación comprende bibliotecas, ejecutables, adaptadores, interfaces, pruebas y programas auxiliares propios, sin que ello autorice lenguajes, componentes o funciones distintos de los admitidos por las normas del sistema.

> © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

El aviso se dispone como comentario válido del lenguaje, en la cabecera o al final del archivo, acompañado de `SPDX-License-Identifier: CC-BY-NC-ND-4.0` y del [texto oficial de la licencia](https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode.es). Debe conservar las instrucciones de ejecución, la codificación y el comportamiento del programa. La declaración general de `LICENSE` no sustituye esta identificación individual.

Cuando el formato no admite comentarios, o cuando el archivo es una prueba de bytes exactos, un producto generado, una fuente firmada o una evidencia histórica cuya integridad debe conservarse, el aviso se incorpora a un archivo adjunto denominado `<nombre-original>.license`. Este adjunto identifica expresamente el archivo y su huella; ambos deben conservarse y distribuirse juntos. La excepción no permite omitir la atribución ni modificar silenciosamente firmas, manifiestos o resultados anteriores.

Si los permisos del repositorio impiden modificar el archivo o situar el aviso junto a él, la declaración individual se conserva en `licencias-de-archivos/`, con la ruta completa y la huella del original. Esta excepción debe quedar identificada en la revisión; no autoriza eludir las protecciones del repositorio.

Los componentes de terceros conservan sus autores, avisos y licencias. La presente declaración se limita al código y a las aportaciones de titularidad propia; no sustituye derechos ajenos ni revoca permisos válidamente concedidos con anterioridad. Los archivos con contenido de procedencia mixta requieren delimitar esas aportaciones, sin atribuir íntegramente al SV el trabajo de terceros.

Antes de aceptar una implementación o una entrega, la revisión debe comprobar la presencia del aviso, su alcance, la conservación de las licencias de terceros y las excepciones justificadas. Los generadores y las plantillas de código propio deben conservarlo en sus productos. La ausencia de esta comprobación impide declarar cumplida esta obligación. Esta norma no acredita por sí sola que exista una comprobación automática ni que todas las ediciones históricas hayan sido revisadas.


## 3. Memoria y concurrencia

El código propio seguro declara `forbid(unsafe_code)` donde sea aplicable. Un adaptador que necesite FFI o un controlador nativo no hereda seguridad por estar invocado desde Rust: delimita punteros, tamaños, duración, propiedad, hilos, errores y liberación. Toda excepción `unsafe` exige contrato de seguridad documentado, revisión específica y pruebas; nunca se introduce para eludir una restricción.

La RAM dispone de cotas de bytes y elementos, comprobadas antes de reservar o acumular, y plazos para las operaciones bloqueantes. Un semáforo conserva su permiso durante toda la vida efectiva del recurso. Se evita mantener bloqueos de estado mientras se espera la red. Las operaciones lentas revalidan sesión, revisión y permiso del recurso en la última frontera antes de entregar o escribir.

GPU, TPU y NPU requieren reserva y liberación reales, identificación del dispositivo, memoria disponible observada y límites del controlador. Una etiqueta de dispositivo no acredita asignación ni aislamiento. La selección depende de compatibilidad y rendimiento verificado de extremo a extremo; no prioriza automáticamente un acelerador ni divide silenciosamente una tarea. Si esta versión no utiliza un dispositivo para una función, se declara no utilizado; no se le atribuyen controles que aún no existen.

Los secretos tienen vida mínima y se mantienen fuera de registros y errores. Los contenedores con borrado al liberar reducen exposición, pero no prueban la eliminación de todas las copias, volcados, paginación o memoria de dispositivos. El aislamiento de proceso, los permisos del sistema operativo y la configuración del soporte completan la protección.

## 4. Persistencia, tablas y soportes

El esquema es un producto auditable: tabla o colección, clave, campos, tipos, unidades, nulabilidad, referencia, ámbito, clasificación de datos, lectores, escritores, índices, límites, conservación, versión y migración. Debe existir correspondencia entre el esquema publicado y las definiciones utilizadas por el programa. Toda modificación explica compatibilidad, recuperación y comprobación de datos anteriores.

En el Gestor actual las tablas son `redb`, con nombres y claves tipados y valores serializados; no son sentencias SQL. La definición común está en `esquema.rs`. Los nombres se fijan en código y no proceden de formularios. Si se añade SQL, los valores se suministran mediante parámetros, y los identificadores dinámicos sólo mediante una lista permitida de nombres competentes. Cambiar nombres de campos, escapar texto indiscriminadamente o confiar en el compilador no evita por sí solo una inyección.

La autorización se comprueba por sujeto, operación, dominio, recurso, vigencia y revisión. Encontrar un duplicado no concede permiso para revelar su identidad. La protección de lectura, escritura, conversión, impresión y exportación se determina separadamente. Ningún cliente de IA recibe un identificador de instalación o credencial humana como sustituto de esta autorización.

Las transacciones conservan integridad ante concurrencia e interrupción. Un resultado de confirmación incierta no se reintenta como si nada hubiera ocurrido. Las huellas detectan discrepancias, pero no son por sí solas una firma de autoría ni resisten a quien puede reescribir simultáneamente todos los datos y huellas. La custodia remota, las copias y el control de acceso tienen contratos propios.

La conservación histórica y la capacidad activa se contabilizan separadamente. Cerrar una solicitud o carga libera la plaza que corresponda sin borrar su evidencia. Los límites incluyen personas, dominios, cantidad, bytes y frecuencia. El archivo histórico también requiere política de conservación, capacidad y recuperación; no se promete almacenamiento ilimitado.

Los accesos a almacenamiento fijo se limitan a rutas resueltas del recinto autorizado, con control de enlaces, permisos mínimos y creación segura. Se comprueba el límite antes de leer archivos completos. Las copias se cifran y custodian según el riesgo y el soporte institucional; la mera existencia de una clave API no cifra una base local. Debe ensayarse la restauración de copias y la gestión del espacio insuficiente.

## 5. Aislamiento del SV de IAs externas

**Política general: sólo conexión con proveedores de IA.** Las excepciones de evaluación se limitan a los ocho candidatos de la convocatoria EET-CYB16 y a la edición vigente de su categoría. Cada examen exige condiciones recibidas, habilitación administrativa expresa del expediente y lanzamiento del Evaluador. El recuento oficial disponible para Anthropic, Gemini, OpenAI, Kimi y Grok requiere autorización documental propia; no habilita generación. Las soluciones, conversaciones y datos personales ajenos a los documentos autorizados quedan excluidos. Los permisos históricos no habilitan una nueva entrega. Ninguna excepción habilita producción.
`aislamiento_ia.rs` realiza el control de salida del Gestor hacia proveedores de IA. No sustituye la instrucción ni el contrato de frontera del núcleo. No se añade una FFI para denominar aislamiento a una interfaz entre lenguajes. La comprobación se solicita voluntariamente, deriva una ruta de catálogo del destino configurado y permite GET sin cuerpo. Sólo emplea los datos técnicos y la credencial necesarios para autenticar ese destino. La vinculación de cuenta mantiene sus intercambios de autorización fijos, sin contenido documental.

**Excepción de inicialización del Panel del Experto, 26/10/2026.** Se autoriza un recorrido Rust específico que solicita únicamente la respuesta fija «OK, preparado» a Astra 6 mediante la cuota incluida de la cuenta ChatGPT vinculada por el Administrador. Exige identidad y configuración vigentes, reserva previa, observación del proceso y custodia verificadas; la confirmación requiere recepción final íntegra, identidad del modelo, consumo coherente y cierre de la traza. No admite contenido documental, preguntas libres, herramientas, credenciales de API de pago, créditos adicionales, sustitución del modelo ni reenvíos automáticos. El registro distingue este uso del desarrollo y de los exámenes. La caducidad o el fallo no se interpretan como conformidad. La excepción no habilita consultas, agentes para terceros, rutas críticas ni autoridad sobre el núcleo. La indicación tiene vigencia acotada y no acredita vigilancia permanente ni corte monetario institucional recibido.

Los contadores remotos de tokens reciben contenido. La excepción se realiza mediante un recorrido Rust específico, limitado a los destinos oficiales recibidos de Anthropic, Gemini, OpenAI, Kimi y Grok y a los originales verificados del expediente. Exige identidad, permiso vigente, confirmación de la entrega, correspondencia de modelo y configuración, huellas documentales y registro previo de la operación. No permite direcciones libres, archivos adicionales, soluciones ni una solicitud de generación. Conserva la respuesta y su huella, fecha, modelo y configuración; un recuento es una estimación, no consumo facturado. Un cambio documental o de configuración invalida su aplicación automática. No se repite la operación sin una nueva autorización explícita. La interfaz explica estas condiciones antes de la entrega. En Grok, el recuento sólo acredita los textos íntegros; se conserva la aceptación expresa de la envoltura no medida, sin presentarlo como recuento integral. La lista de tokens tiene recepción acotada y reserva de custodia previa a la comunicación; no se interpreta como instrucciones.

La configuración de salida no admite instrucciones arbitrarias, cabeceras libres, redirecciones, reintentos, herramientas ni peticiones derivadas de respuestas. Se limita tiempo, destino público y tamaño de respuesta. La interpretación del catálogo sólo comprueba la identidad declarada del modelo; cualquier instrucción incrustada se trata como datos inertes. La presencia en el catálogo no acredita inferencia, capacidad máxima, versión efectiva, saldo ni aptitud.

El punto general de envío de contenido continúa rechazando antes de acceder a la credencial, realizar DNS o abrir una conexión. El recorrido del examen exige un suceso administrativo que identifique expediente, responsable, fundamento y huellas del plan y los tiempos. Se revalida antes de las credenciales y de la red y durante la recepción. Retirar o volver a recibir condiciones invalida la habilitación anterior. La reserva transaccional impide repetir el intento; un corte irreversible conserva lo recibido sin reenvío ni dictamen. No existe una variable de entorno que suprima estos controles. Las respuestas se conservan como datos; no ejecutan herramientas ni conceden acceso al SV. La observación exigida condiciona el cierre y la corrección. Cualquier ampliación requiere decisión expresa, cambio identificado, revisión y nuevas pruebas. Las credenciales son propias de cada instalación, se conservan fuera de los artefactos distribuidos y no se incorporan a una release junto con sesiones, expedientes o copias institucionales.

La garantía técnica queda delimitada al ejecutable y configuración comprobados. No borra datos entregados anteriormente, no gobierna otros programas de la institución ni prueba qué retiene un proveedor sobre los datos mínimos de autenticación. Las conexiones de identidad, correo, custodia y accesibilidad se inventarían por separado como tratamientos y destinatarios; no se ocultan bajo la denominación «conexión de IA».

**Consulta delimitada del Panel del Experto, 26/10/2026.** La ampliación expresa comprende la comprobación sintética de seis referencias SV(25,5) y la investigación bibliográfica pública sobre tricoleucemia, bajo las reglas del dominio de Inmunología. Sólo se admite la cuota incluida de ChatGPT y Astra 6. El servidor decide antes del envío mediante operaciones identificadas: no admite texto libre, expedientes, datos de pacientes, herramientas administrativas, claves alternativas ni usos personales ajenos al dominio. La investigación requiere autorización por consulta; la modalidad ordinaria carece de herramientas de Internet. Reserva, aislamiento, revocación, observación, consumo, límites y custodia son obligatorios. La explicación del modelo no acredita determinismo ni observación de su razonamiento interno: se cotejan por separado fuentes, orden, valores y omisiones de la salida. Los hallazgos externos permanecen pendientes de los filtros científicos y de la aprobación competente; no se incorporan automáticamente. Las incidencias de pertinencia se notifican sin el texto rechazado. Su revisión puede corregir una regla excesiva dentro de la especialidad, nunca permitir asuntos ajenos ni reenviar por sí sola una consulta. Esta ampliación no acredita recepción clínica, clasificación general de preguntas ni acceso de terceros. Las demás restricciones permanecen vigentes.
### 5.1. Núcleo aislado y transporte por la tubería del SV

La IA no accede directamente al núcleo. Su transporte hacia él debe realizarse por la tubería del SV y someterse a la instrucción y al contrato de frontera constituidos. No se crea una vía paralela mediante llamadas directas, contenido ejecutable del modelo, memoria compartida sin contrato, escritura de tablas o modificación de permisos. La definición y semántica del núcleo tienen una única autoridad; el Gestor no las reimplementa.

La tubería transporta datos y propuestas subordinadas. El contrato determina qué petición cerrada puede admitirse, con identidad, procedencia, versión, orden, recursos, autorización, diagnóstico y evidencia. Las salidas de IA son entradas no fiables, no instrucciones con autoridad. Una respuesta no funda por sí sola una transición real, un suceso ni conocimiento admitido. El fallo técnico conserva su causa y no se convierte en un valor de la terna.

Se aplican la arquitectura de núcleo–frontera–soporte y las recepciones de la continuación del 15/09/2026, conservando sus fechas y límites. Las alternativas históricas de soporte no sustituyen la decisión vigente de realizar el SV en Rust. Antes de integrar un trayecto se identifican contrato y versión, componentes utilizados, límites materiales, observador y pruebas positivas y negativas. Una recepción documental o un control intraproceso no acredita confinamiento del conjunto. La mera presencia de una FFI tampoco lo acredita.

La versión actual del Gestor comprueba conexiones y conserva preparación y antecedentes; no habilita el circuito de consejo experto en producción ni afirma haber recibido una integración de inferencia con el núcleo. El control de salida a proveedores y la frontera del núcleo son obligaciones complementarias, con evidencias separadas. La documentación y la interfaz deben mostrar ambas con igual visibilidad, sin presentar una como realización completa de la otra.

## 6. Protección de datos desde el diseño

La institución identifica responsable, finalidades, base jurídica y destinatarios antes de habilitar un tratamiento. Para datos de salud u otras categorías especiales debe concurrir además la condición jurídica específica aplicable. Una aceptación de uso, una exención de responsabilidad o el nombre de un dominio médico no acreditan licitud.

El diseño minimiza campos, accesos, salidas, tiempos y conservación. Se ofrece información al interesado antes de recoger datos y se documentan los encargados, subencargados y transferencias aplicables. La identificación mediante un tercero y la protección perimetral no sustituyen estas obligaciones. En España se considera asimismo la LOPDGDD; la institución determina las normas sectoriales y territoriales adicionales que le correspondan.

Se mantiene un inventario de tratamientos y una matriz obligación → control → evidencia → pendiente. Se preparan procedimientos de acceso, rectificación, supresión, limitación, oposición y portabilidad cuando proceda, verificando identidad sin recopilar datos excesivos. No se declara una supresión cumplida si permanecen copias accesibles sin tratamiento justificado. La conservación probatoria se concilia con minimización y plazos aprobados, mediante archivo restringido, bloqueo o supresión según corresponda; nunca por borrado automático improvisado.

El análisis de riesgo determina medidas proporcionadas, registro de incidentes, recuperación, necesidad de evaluación de impacto y, en su caso, delegado de protección de datos. El procedimiento de brechas identifica decisión, responsables, destinatarios y plazos legales; un registro de error técnico no equivale por sí solo a una notificación cumplida. La información jurídica institucional y las evidencias de configuración deben revisarse antes de distribuir o habilitar uso real.

## 7. Identidad, claves y segundo factor

Se distinguen clave de instalación, credencial de usuario, sesión, secreto de proveedor y segundo factor. Ninguna sustituye a las demás. Las credenciales se vinculan al destino exacto y al ámbito autorizado; no se incluyen en URL, consultas, informes ni ejemplos. Se documentan alta, rotación, revocación, caducidad y recuperación.

El segundo factor se acredita mediante evidencia del proveedor de identidad o del mecanismo institucional recibido. No se infiere de una casilla, del nombre del proveedor o de la ausencia de un campo. La instalación actual comprueba las condiciones GitHub en `github.rs`; una institución distinta debe recibir su propia adaptación, sin estar obligada a reproducir GitHub o Cloudflare. El esquema público no contiene secretos ni datos personales de cuentas reales.

## 8. Recepción, pruebas y optimización

Cada cambio relaciona necesidad, suceso, tique, revisión de fuentes, contrato, implementación, pruebas positivas y negativas, resultado y límites. Se comprueban al menos autorización, revocación, caducidad, concurrencia, interrupción, restauración, tamaño extremo y separación entre dominios cuando resulten afectados. Las pruebas de aislamiento inspeccionan solicitudes construidas y demuestran rechazo de contenido antes de cualquier salida; no utilizan proveedores reales para comprobar una prohibición.

La entrega incluye formato comprobado, compilación reproducible en el entorno declarado, pruebas pertinentes, revisión de advertencias y dependencias, y verificación visual de las pantallas afectadas. Los avisos de terceros se registran sin confundirlos con fallos propios ni ocultarlos. Una herramienta no disponible deja un pendiente explícito; no se anota como prueba favorable.

La optimización parte de medidas comparables de tiempo, memoria, almacenamiento y concurrencia. Se priorizan cargas excesivas y operaciones repetidas evitables, sin debilitar permisos, conservar datos fuera de su ámbito o alterar documentos. Una caché nunca acredita autorización. No se promete rapidez obtenida suprimiendo comprobaciones necesarias.

La revisión independiente recibe fuentes, esquema, instrucciones de construcción, pruebas, resultados y limitaciones. La autoridad de recepción es distinta de quien implementa; «preparado para revisión» no significa certificado por auditores externos.

### 8.1. Mantenimiento de dependencias y publicación de versiones

La [Política de mantenimiento de dependencias y publicación de versiones](../../POLITICA-DEPENDENCIAS-Y-VERSIONES.md), versión 1.1 de 26/10/2026, desarrolla la revisión periódica de avisos, la resolución fijada de dependencias, la compatibilidad, las condiciones de recepción y la recuperación. Se aplica a las crates propias, sus dependencias directas e indirectas y las herramientas externas de la distribución. No autoriza actualizaciones indiscriminadas, cambios semánticos del núcleo ni gastos adicionales. La aplicación automática está autorizada dentro del alcance recibido, con copia cotejada, comprobaciones y registro de incorporación. Su publicación no acredita retrospectivamente la actualización o auditoría del conjunto existente.

## Referencias de aplicación

- [Arquitectura del SV: núcleo, frontera y soporte](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/40df67a40c86b405061d2b8ae40f0064d2672dc0/docs/calidad/ACTA_TECNICA_DE_ARQUITECTURA_DE_SOFTWARE_NUCLEO_FRONTERA_Y_HOST_SV_2026_09_04.md).
- [Tuberías de IA: continuación del 15/09/2026](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/tree/main/docs/calidad/tuberias-ia/continuacion-15-09-2026): contratos y recepciones con su alcance propio.
- [Cookie 0.18.2](https://docs.rs/cookie/0.18.2/cookie/): construcción y lectura de cookies; su reutilización no certifica cumplimiento jurídico.
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/): contratos y convenciones de interfaces.
- [Clippy](https://doc.rust-lang.org/clippy/): comprobaciones estáticas y selección de reglas.
- [OWASP: prevención de inyección SQL](https://cheatsheetseries.owasp.org/cheatsheets/SQL_Injection_Prevention_Cheat_Sheet.html): parametrización y listas permitidas.
- [OWASP: inyección de instrucciones en modelos](https://cheatsheetseries.owasp.org/cheatsheets/LLM_Prompt_Injection_Prevention_Cheat_Sheet.html): fronteras de confianza y contenido no fiable.
- [RGPD, texto oficial](https://eur-lex.europa.eu/eli/reg/2016/679/oj/spa): arts. 5, 6, 9, 12–22, 24–25, 28, 30, 32–37 y capítulo V.
- [AEPD: privacidad desde el diseño](https://www.aepd.es/guias/guia-privacidad-desde-diseno.pdf).
- [Ley Orgánica 3/2018, texto consolidado](https://www.boe.es/eli/es/lo/2018/12/05/3/con).

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
