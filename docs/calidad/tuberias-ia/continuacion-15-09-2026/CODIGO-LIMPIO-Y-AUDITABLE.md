# Código limpio y auditable

Norma del Sistema Vectorial SV · Versión 1 · 18 de octubre de 2026.

**Carácter obligatorio desde esta fecha.** Se aplica al código Rust final del SV, a sus bibliotecas propias, ejecutables, adaptadores, persistencia y comprobaciones. La entrada en vigor no certifica retrospectivamente todos los componentes existentes. Cada entrega debe identificar su alcance comprobado y las desviaciones pendientes; ninguna excepción tácita se considera conformidad.

## 1. Autoridad, contratos y organización

Cada módulo declara finalidad, entradas no fiables, salidas, efectos, límites y dependencias. Las decisiones científicas, los permisos humanos y los resultados instrumentales tienen tipos y recorridos diferentes. Un error de red, capacidad o almacenamiento nunca se transforma silenciosamente en 0, 1 o U, ni en un dictamen de aptitud. Se conserva una sola terna vigente y un solo contrato de polígono, aplicado a la dimensión de la célula.

El dominio es titular de una asignación vigente: dominio → nodo → modelo. Compartir modelo o infraestructura no comparte datos, permisos, aptitud ni conversaciones. Las herramientas auxiliares de traducción quedan fuera de los nodos de inferencia experta. La preparación no habilita producción.

Una regla debe tener una única realización competente, reutilizada por sus entradas. Se evitan servicios, adaptadores o verificadores paralelos que reinterpreten la misma decisión. Las representaciones de interfaz son proyecciones de reglas Rust y no autoridades alternativas. Los programas auxiliares de edición o construcción no deciden la semántica del SV.

## 2. Estilo, legibilidad y contratos de programación

Se utiliza la edición Rust declarada y una versión mínima comprobada. `rustfmt` normaliza los archivos propios. Clippy se aplica con sus reglas recomendadas y una selección justificada de reglas adicionales; no se activa indiscriminadamente toda la categoría `restriction`. Cada excepción se limita a su expresión o módulo y explica su necesidad. No se silencian advertencias globalmente para obtener una comprobación aparentemente limpia.

Los nombres siguen las convenciones Rust: módulos, funciones y campos en `snake_case`, tipos en `UpperCamelCase`, constantes en `SCREAMING_SNAKE_CASE`. Los nombres describen significado y unidades. Las identidades persistentes existentes no se renombran por estilo sin migración explícita. No se utiliza la oscuridad de nombres como protección.

Los comentarios necesarios expresan **por qué existe una condición**, qué frontera protege y qué compatibilidad conserva. Las fronteras, invariantes, formatos persistentes y operaciones privilegiadas tienen explicación breve en español e inglés, con el mismo significado. No se traduce mecánicamente cada línea ni se duplica la documentación del lenguaje. La documentación extensa se enlaza desde el código.

Se prefieren tipos y enumeraciones que impidan estados contradictorios; `Option` representa ausencia, no cero. Los resultados recuperables se devuelven mediante `Result`, con causas estables y ayuda comprensible. Se restringen `unwrap`, `expect` y `panic` en recorridos que dependen de datos o recursos externos; su uso excepcional requiere una invariancia demostrada. Los errores no publican claves, contenidos, rutas privadas ni respuestas completas de terceros.

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

**Política vigente: sólo conexión; ningún contenido a proveedores de IA.** Quedan excluidos documentos, preguntas, respuestas, instrucciones, soluciones, conversaciones, datos personales y pruebas sintéticas de inferencia. Los permisos históricos de suministro no levantan esta prohibición.

`aislamiento_ia.rs` realiza el control de salida del Gestor hacia proveedores de IA. No sustituye la instrucción ni el contrato de frontera del núcleo. No se añade una FFI para denominar aislamiento a una interfaz entre lenguajes. La comprobación se solicita voluntariamente, deriva una ruta de catálogo del destino configurado y permite GET sin cuerpo. Sólo emplea los datos técnicos y la credencial necesarios para autenticar ese destino. La vinculación de cuenta mantiene sus intercambios de autorización fijos, sin contenido documental.

Los contadores remotos de tokens también son destinatarios de contenido cuando reciben el texto que han de medir. Quedan bloqueados aunque no generen respuestas o no facturen inferencia. Una medición anterior conserva su evidencia y autorización histórica, pero no habilita un nuevo envío. La interfaz debe declarar esta regla de forma visible, explicar qué comprueba una conexión y agrupar las fuentes de respaldo por su función.

La salida no admite instrucciones, cabeceras libres, redirecciones, reintentos, herramientas ni peticiones derivadas de respuestas. Se limita tiempo, destino público y tamaño de respuesta. La interpretación del catálogo sólo comprueba la identidad declarada del modelo; cualquier instrucción incrustada se trata como datos inertes. La presencia en el catálogo no acredita inferencia, capacidad máxima, versión efectiva, saldo ni aptitud.

El punto histórico de envío de contenido rechaza antes de acceder a la credencial, realizar DNS o abrir una conexión. La interfaz de exámenes debe mostrar el impedimento. No existe una preferencia, variable de entorno o casilla que lo suprima. Una futura modificación de esta política requiere decisión expresa, cambio identificado, revisión y nuevas pruebas; nunca se deduce de un permiso antiguo.

La garantía técnica queda delimitada al ejecutable y configuración comprobados. No borra datos entregados anteriormente, no gobierna otros programas de la institución ni prueba qué retiene un proveedor sobre los datos mínimos de autenticación. Las conexiones de identidad, correo, custodia y accesibilidad se inventarían por separado como tratamientos y destinatarios; no se ocultan bajo la denominación «conexión de IA».

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
