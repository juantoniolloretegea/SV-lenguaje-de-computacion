# Contrato de diagnóstico estructurado, procedencia y presentación ES/EN

**Fecha:** 9 de septiembre de 2026.

**Registro:** RETP-2026-109.

**Mandato:** continuación autorizada por Juan Antonio Lloret Egea tras revisar el idioma de los diagnósticos, el ensamblaje y sus posibles consecuencias de seguridad.

**Estado:** CONTRATO_PREVIO_CONSTITUIDO_EN_CANDIDATA; REALIZACION_PENDIENTE.

**Sede:** Lenguaje SV, fila 9; PT01/PT02/PT03/PT04/PT12/PT14, DFL-001 y obligación lingüística DFL-011.

## 1. Corte, fuentes y objeto

El corte examinado es `26ebc9f139397b086ce630df358d6b0fb11d997b`, árbol `b40b886aa35c4410dfcd8733f39df5cadeea3bdd`, cabeza de la [PR #88](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/88), en borrador y sin fusionar. Su base es `main@66967a80a40f4e2781ef983bd725690db54c25c5`. Los resultados anteriores conservan sus propias identidades.

Se han leído AGENTS.md, los [Pilares completos](PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md), el [acta de perfiles completa](ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md), la [transición y sus relevos](../dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md), el [acta del español](ACTA_DE_USO_DEL_ESPANOL_EN_TODOS_LOS_REPOSITORIOS_SV_2026_09_07.md), los [perfiles fuente ES/EN](../../ESPECIFICACION_NORMATIVA_PERFILES_FUENTE_SVP_ES_EN_v1_2026_08_29.md), el [catálogo diagnóstico efectivo](../referencia/ERRORES_CANONICOS_SV_v0_3.md), DFL-001 y RETP-107/108. El [contrato SEC.0-D](../arquitectura/CONTRATO_ABSTRACTO_DE_DIAGNOSTICO_Y_FALLO_CERRADO_SEC0_D_2026_08_21.md) conserva la separación entre resultado técnico, autoridad y valor de dominio.

Se fija la estructura exigible para que el autor reciba una explicación en el idioma de su fuente y pueda localizar el defecto sin alterar la identidad diagnóstica, el juicio ni el código original. La seguridad se recibe como condición de diseño de esta frontera. No se abre una campaña general de ataque, no se acredita seguridad material ni se incorpora conocimiento profesional IMM/CYB.

## 2. Defecto medido y límite de la evidencia

Sobre el corte indicado, `codominio K = {};` bajo ES y `codomain K = {};` bajo EN producen el mismo diagnóstico: `InvalidProgram("E004 (InvalidCodomain): codomain K vacío")`. La falta del punto y coma final produce `Frontend(UnexpectedEnd)` en ambos perfiles. Dos declaraciones de K, una EN y otra ES, producen `InvalidProgram("identificador duplicado: K")`, sin identificar los dos archivos ni sus posiciones. El mismo fallo sintáctico en la primera o en la segunda unidad tampoco queda atribuido por el error devuelto.

Una grafía exclusiva del otro perfil puede exponer `__SVP_FOREIGN_SURFACE__`. Una entrada nativa no UTF-8 devuelve un fallo de lectura con retorno 2 y puede mezclar prosa española con texto de la biblioteca de sistema. No es un rechazo semántico con retorno 1. El centinela es una representación interna inadecuada para explicar el defecto; su aparición no prueba por sí sola divulgación de información sensible.

La realización usa `CompileError::InvalidProgram(String)` y variantes de `FrontendError`, presentadas mediante `Debug`. El perfil selecciona el análisis de superficie, pero no la prosa diagnóstica. La interfaz traduce sus rótulos y muestra el error recibido; esa traducción no localiza el diagnóstico. El ensamblaje conserva unidades de entrada y una identidad conjunta, pero el error devuelto no incorpora el vínculo estructurado necesario con sus contextos.

La caracterización reproducible se conserva en [evidencias/RETP-109](evidencias/RETP-109/custodia.json): trece casos sintéticos, ejecutados en nativo y con el módulo WebAssembly mediante Node. Proceden del artefacto `10116699662` del [flujo de la cabeza examinada](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34383121897). Esta repetición no recompila Rust, no es una ejecución local en navegador real y no prueba toda la cobertura diagnóstica. Sus resultados describen la carencia anterior; no acreditan su reparación.

El ZIP descargado tiene 3419689 bytes y SHA-256 recalculado `91a535e098f59b91f03f220438b712952fd1d120a5ad02200e7f4370b256e408`, coincidente con los metadatos del artefacto. Para repetir, descargar ese artefacto exacto, comprobar la huella y extraer `rust/target/release/sv-native`, `rust/target/release/examples/assembly_probe` y `rust/target/wasm32-unknown-unknown/release/sv_wasm.wasm`, conservando sus rutas y habilitando la ejecución de los dos binarios nativos. El script comprueba sus tres huellas antes de ejecutarlos. Con Node 24.19.0, desde la raíz y ajustando únicamente los dos directorios locales:

```sh
node docs/calidad/evidencias/RETP-109/sondar.mjs /ruta/extraida/rust/target /ruta/temporal > /ruta/resultados.json
```

Se conservan [los casos](evidencias/RETP-109/casos.json), [el reproductor](evidencias/RETP-109/sondar.mjs), [las salidas íntegras](evidencias/RETP-109/resultados.json) y [sus huellas](evidencias/RETP-109/huellas.sha256). Los esperados se fijaron a partir de la medición previa: son una caracterización de compatibilidad, no un oráculo independiente de la reparación futura. El texto de sistema y su ruta temporal se custodian sin sustituirlos; ese texto puede variar entre sistemas. La disponibilidad del ZIP depende de la retención del artefacto.

## 3. Contenido y autoridad de un diagnóstico

Se distinguen los siguientes componentes obligatorios en la realización que se ofrezca. Son obligaciones de representación; esta tabla no publica una nueva API ni un esquema de transporte ejecutable.

| Componente | Obligación |
|---|---|
| Origen y fase | Diferenciar Lenguaje SV, herramientas Rust y fallo de lectura, transporte o anfitrión. Un código de rustc no se incorpora por coincidencia numérica al catálogo SV. |
| Decisión técnica | Conservar el rechazo o fallo original fuera de la prosa. Una presentación localizada no crea admisión, permiso, autoridad, resultado de dominio ni Tri.U. |
| Identidad diagnóstica | Conservar código efectivo y nombre canónico cuando exista correspondencia constituida. La ausencia de código catalogado se declara; no se asigna un E... por semejanza textual. |
| Causa discriminante | Conservar la infracción concreta mediante una representación cerrada y versionada emitida en el punto de comprobación. Compartir E115 no puede borrar la distinción entre claves repetidas, ausentes y ajenas. |
| Entidades relacionadas | Conservar tipos canónicos, nombres originales y función de cada referencia en el defecto. Un nombre no acredita identidad de instancia ni autoridad. |
| Contextos fuente | Ligar cada ubicación a su unidad, perfil y bytes exactos, diferenciando contexto principal y referencias relacionadas cuando esa distinción esté acreditada. |
| Explicaciones | Producir textos ES/EN mediante plantillas revisadas, versionadas y deterministas, con parámetros tipados. No se consulta un modelo ni un servicio de traducción durante la ejecución. |
| Estado de presentación | Hacer visible una explicación o un contexto que no pueda presentarse. Esa carencia no reemplaza ni oculta la decisión técnica. |

La relación entre código, causa, parámetros y mensajes se establece desde las reglas constituidas y sus emisores. Queda excluido reconstruirla interpretando `Debug`, expresiones regulares sobre la prosa o identificadores elegidos por el usuario. Una estructura rellenada después por conjetura no satisface este contrato.

Se conservarán el código y el tipo IR como identidades canónicas. Puede mostrarse además su denominación explicativa española o la forma fuente constituida, sin sustituir aquellas identidades. Los nombres de objetos y cadenas del dominio no se traducen ni se emplean para deducir el idioma.

## 4. Idioma y atribución en el ensamblaje

| Entrada | Presentación exigida |
|---|---|
| Unidad SVP-ES | Explicación en español y contexto de sus bytes originales. |
| Unidad SVP-EN | Explicación en inglés y contexto de sus bytes originales. |
| Ensamblaje sólo ES o sólo EN | Explicación en el idioma común, conservando la identidad separada de todas las unidades relacionadas. |
| Ensamblaje ES+EN | Explicaciones en ambos idiomas para el diagnóstico del conjunto; cada contexto conserva su archivo, perfil y fragmento originales. Un solo defecto mantiene una sola identidad diagnóstica. |
| Perfil todavía no establecido o no admitido | Diagnóstico técnico explícito con explicación ES/EN. No se infiere un perfil a partir del texto, del archivo ni del idioma de la interfaz. |

La entrada histórica sin selector conserva EN como perfil de compatibilidad; ese comportamiento ya está constituido y no es detección automática. El idioma de navegación no modifica el perfil ni el juicio. Presentar ambas explicaciones en un ensamblaje mixto no atribuye el defecto a ambas unidades cuando sólo una lo contiene.

Para localizar un defecto se necesita, al menos, índice de unidad dentro de la entrada ordenada, nombre recibido, perfil explícito, huella de los bytes originales y uno o más intervalos pertinentes. Dos unidades con el mismo nombre de archivo no deben confundirse. Un defecto sintáctico puede existir antes de constituir un objeto; uno relacional puede afectar a varios. No se presupone un objeto único propietario de cada error.

Los intervalos autoritativos se expresan sobre bytes originales, desde cero y con extremo final excluido. El fin de archivo puede señalarse mediante el intervalo vacío en la longitud de la fuente. Las cifras deben preservarse exactamente en el transporte. Líneas y columnas son vistas derivadas: la realización documentará su convención antes de ofrecerlas y comprobará UTF-8 multibyte, CRLF, LF y tabulaciones. No se fabrican coordenadas cuando falte su procedencia ni se reemplaza UTF-8 inválido para simular una fuente válida.

La fuente íntegra permanece inalterada. Los escapes o recortes visibles deben distinguirse de sus bytes originales; un fragmento omitido por límite de presentación no desaparece silenciosamente ni cambia la huella. La explicación no reconstruye el código del usuario desde la IR canónica.

## 5. Frontera de seguridad y relación con R1

Las carencias medidas permiten confusión, pero no demuestran un permiso indebido. Las rutas examinadas conservan rechazo mediante `Err`, retorno nativo y señal de error WebAssembly, independientemente del texto. La interfaz examinada inserta la salida como texto. No se declara probada una elusión de R1, una inyección de registros ni divulgación sensible.

El diseño deberá satisfacer estas condiciones:

- Ningún consumidor puede obtener admisión, autoridad, permiso, revocación o decisión de reintento interpretando una palabra del mensaje localizado.
- El cambio, ausencia o corrupción de una plantilla no transforma el resultado técnico. Un fallo de presentación queda separado y visible; nunca produce una salida de dominio.
- Fragmentos, nombres y textos externos son datos. Se delimitan y escapan según el destino, sin interpretarlos como HTML, instrucciones, secuencias de control del terminal o registros independientes.
- Se presenta sólo el contexto necesario al receptor autorizado; no se copian por defecto fuentes completas, secretos, rutas internas ni objetos protegidos. La vista pública no sustituye la custodia del original.
- Una huella identifica bytes; no autentica al emisor ni acredita facultades. Serializar un nombre de referencia protegida no transporta una capacidad R1.
- La decisión y su vinculación con objeto y contexto permanecen en las vías de control correspondientes. No existe conversión de una explicación o un diagnóstico en permiso.

La evaluación de explotación material y de costes pertenece a su alcance y sede posteriores. Como referencias de clase se conservan [CWE-117](https://cwe.mitre.org/data/definitions/117.html), sobre neutralización de registros, y [CWE-209](https://cwe.mitre.org/data/definitions/209.html), sobre información sensible en mensajes. No se asignan como vulnerabilidades demostradas del SV.

El enlace profesional pendiente de [RETP-108, §8](CONTRATO_DE_CONSUMO_DOCUMENTAL_CYB_2026_09_09.md#8-retp-108-frontera-entre-declaración-documental-y-autoridad) conserva su contrato de recepción de referentes y premisa externa. Este trabajo diagnóstico lo precede por mandato del Director; no modifica el R1 ya acreditado ni convierte su corrección en autorización institucional.

## 6. Sede de la realización y compatibilidad

La pérdida se localiza en los errores y su procedencia durante el análisis y la validación, y en la presentación de las salidas. Traducir únicamente la página del catálogo no basta: el contraejemplo del ensamblaje no contiene los archivos que esa página necesitaría señalar. Mantener sólo una tabla externa de nombres tampoco basta: no distingue dos declaraciones homónimas ni un fallo anterior a la declaración.

La realización empezará por una estructura diagnóstica y un vínculo de procedencia conservado desde el análisis, separado de la semántica de la IR. No se modifica la gramática ni la IR canónica para resolver por anticipación esta necesidad. Si la implementación demuestra insuficiencia de esa sede, conservará un contraejemplo discriminante antes de proponer otra. No se da por resuelto K2 ni se introducen nuevas capacidades profesionales.

Antes del cambio funcional se inventariarán los puntos de emisión alcanzables por las entradas públicas afectadas, sus causas, código catalogado o ausencia explícita, parámetros y contextos. Ese inventario impedirá que la localización se limite a los ejemplos aquí medidos. Los 51 códigos catalogados no equivalen a 51 emisores presentes ni a cobertura completa.

La migración será aditiva en la identidad y trazabilidad: añade información estructurada y explicaciones sin renumerar códigos ni reescribir resultados históricos. No obliga a mantener para siempre toda cadena de presentación. El cambio observable de CLI/ABI, envoltura, serialización o formato diagnóstico debe declararse y versionarse antes de sustituirlo. No se añade una API como atajo para mantener dos juicios distintos.

El oráculo actual comprueba retorno 1, salida normal vacía, envoltura de rechazo y subcadena esperada; no exige siempre igualdad de todo el mensaje. Se conservarán las 106 fuentes inválidas y sus obligaciones, junto con los positivos y las campañas aplicables. La migración fijará por caso la identidad y causa esperadas, sus ubicaciones y los textos ES/EN revisados. Los nuevos esperados no se generarán copiando automáticamente la salida de la realización.

Los oráculos del resultado técnico, de procedencia y de presentación se comprobarán por separado. La paridad se exige entre destinos para una misma entrada y política de presentación. Entre fuentes ES y EN equivalentes se exige igualdad del juicio, identidad canónica y causa; no igualdad de prosa ni de huellas de fuentes distintas.

## 7. Condiciones de aceptación de la realización

Los identificadores DG siguientes nombran obligaciones de esta matriz; no son códigos diagnósticos del catálogo. Su estado inicial es PENDIENTE_DE_REALIZACION_Y_VERIFICACION. La caracterización de §2 no los cierra.

| ID | Control positivo | Contraejemplo o control discriminante |
|---|---|---|
| DG01 | Fuentes ES/EN equivalentes conservan admisión e IR semántica. | Una traducción de presentación no admite una fuente inválida ni modifica un literal de dominio. |
| DG02 | E004 conserva identidad con explicación ES o EN según perfil. | Cambiar el idioma de la interfaz no cambia el perfil fuente ni el juicio. |
| DG03 | Las causas de E115 permanecen diferenciadas y sus textos explican cada una. | Una causa de ausencia presentada como repetición se detecta aunque conserve E115. |
| DG04 | Fin de archivo se explica y localiza en la unidad correcta. | El mismo fallo en A y en B debe devolver procedencias distintas y correctas. |
| DG05 | Una grafía admitida se reconoce bajo su perfil. | La grafía exclusiva del otro perfil se rechaza con contexto original, sin centinela interno como explicación. |
| DG06 | Ensamblaje ES/EN conforme conserva sus dos unidades. | Una colisión enlaza ambas declaraciones exactas y ofrece ambas explicaciones; un fallo local no acusa a la unidad sana. |
| DG07 | Referencias válidas entre unidades se resuelven en ambos órdenes admitidos. | Un nombre de archivo compartido no permite confundir índices, huellas o ubicaciones. |
| DG08 | Los intervalos recuperan exactamente el fragmento UTF-8 esperado. | Multibyte, EOF, CRLF, LF y tabulaciones no desplazan ni inventan la ubicación. |
| DG09 | Error de lectura o perfil no admitido conserva su capa técnica. | UTF-8 inválido no se convierte en rechazo semántico, fuente reparada o Tri.U. |
| DG10 | Plantillas completas conservan identidad, parámetros y resultado. | Falta de plantilla, parámetro o idioma no oculta el rechazo ni produce éxito. |
| DG11 | Fragmentos se presentan como datos delimitados. | Texto con apariencia de HTML, instrucciones o controles no se interpreta ni fabrica otros registros. |
| DG12 | Se conserva sólo el contexto necesario a la vista correspondiente. | Una vista de error no expone por defecto fuentes completas, secretos ni capacidades protegidas. |
| DG13 | Un consumidor utiliza la decisión técnica y las referencias comprobadas. | Prosa que diga lo contrario, o un identificador aportado por el usuario, no altera admisión ni concede un permiso. |
| DG14 | Mismos casos, perfiles y formato producen resultados pertinentes concordantes en nativo, WASI y navegador. | Una desviación en causa, contexto o texto localizado se detecta; no se acredita una vía por el verde de otra. |

DG11–DG13 se acotan a la frontera diagnóstica que se materialice y a consumidores de prueba explícitos. No sustituyen una campaña general R3/R4. Antes de ejecutar cada grupo se comprometerán sus casos concretos, esperados y límite; una condición sólo podrá cerrarse con evidencia de su implementación y destino.

## 8. Relevo y condición de terminación

Este incremento termina con el contrato, la caracterización de la carencia y la continuidad RETP/deuda concordantes. Su resultado no es un catálogo ya localizado. El siguiente incremento realizará el inventario de emisores, la representación diagnóstica, la procedencia y las plantillas, con los oráculos comprometidos y migración versionada de las interfaces afectadas.

Antes de proseguir el enlace con R1 o aprobar la candidata, el Lenguaje deberá recibir una realización verificada de este contrato en el alcance ofrecido, o una nueva decisión explícita del Director sobre ese alcance. No se presume otra autorización de cierre por la mera publicación de este documento.

DFL-001 conserva las carencias de concordancia y cobertura no resueltas. DFL-011 y el acta del español permanecen vigentes; no se aplaza la corrección de texto nuevo a la revisión integral final. La anotación histórica del observador G/H se conserva. Fila 9 continúa abierta; los dominios permanecen en pausa y PR #88 continúa como candidata sin fusionar.

## 9. RETP-110 — Entrega visible y aplicación a los dominios

**Precisión del Director, 09/09/2026.** Corte leído: 4276c79be20a7bbf46ddd9440b5a83361bd03dd0, árbol 91728c4f605604f01a6ee6fc1cca957c0935463d. Se conserva el contrato previo y la carencia caracterizada; esta sucesión precisa el lugar de entrega observable y la relación con los perfiles de dominio. No afirma implementación ni despliegue.

### 9.1. Comprobación en el playground de producción

La localización deberá poder observarse en el [entorno público de producción](https://lenguaje-sv.itvia.online/) cuando se publique su realización. Se distinguen dos momentos para evitar una dependencia circular: DG01–DG14 y la interfaz candidata se verifican antes de aprobar la integración; la entrega visible se comprueba después del despliegue de la versión admitida. La fusión de una PR no demuestra por sí sola qué versión sirve la web.

La comprobación posterior conservará URL, fecha, corte publicado e identidad de los artefactos realmente cargados —módulo WebAssembly, interfaz y catálogo o plantillas— y su correspondencia con la realización probada. El usuario deberá poder consultar la versión y obtener en el propio playground explicaciones ES, EN y ambas en ensamblaje mixto, con código y causa invariantes y contexto original atribuido. Cambiar sólo los rótulos o servir una interfaz nueva con un módulo anterior no satisface la entrega. Los fallos de sintaxis anteriores a un objeto también deben ser observables.

Se reutilizarán los casos de DG02/DG04/DG05/DG06 y los controles conformes pertinentes. El cambio del idioma de navegación no debe sustituir el perfil fuente. Se comprobará que un artefacto anterior o un conjunto de versiones discordantes no se acredita como la nueva entrega; se identificará y corregirá la discrepancia antes de declararla disponible. La evidencia de navegador sobre la candidata y la del sitio desplegado conservan sus cortes y alcances separados. Esta precisión no ordena publicar ahora una realización pendiente ni convierte el playground en una operación clínica o profesional CYB.

### 9.2. Independencia del idioma y del perfil de dominio

El [acta de perfiles, §§3–8](ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md) distingue perfil fuente, constitución de dominio, contrato del agente y soporte tecnológico. Inmunología y Ciberseguridad Inteligente reciben la misma obligación diagnóstica del Lenguaje cuando utilicen las operaciones realizadas: el idioma determina la explicación; el dominio determina qué constitución y reglas se han recibido. Ningún dominio queda ligado por defecto a ES o EN. El ensamblaje ES+EN no constituye una composición IMM+CYB.

La localización no modifica parámetros, células, asignaciones, cobertura, facultades, reglas, fuentes ni versiones de un dominio. Los identificadores, referencias, unidades de medida, códigos externos y literales profesionales se preservan como datos originales. La traducción o adaptación de un contenido profesional necesita su propia constitución y evidencia; no se realiza como efecto de traducir la interfaz o un diagnóstico del Lenguaje. Un término profesional que se ofrezca como explicación propia requiere una correspondencia aprobada por la sede competente.

Sí hay una obligación de trazabilidad al diagnosticar contratos de dominio: conservar la identidad y versión exactas y las referencias a objeto, parámetro, regla u operación efectivamente involucrados, cuando el comprobador disponga de esa ligadura. El idioma, nombre de archivo o nombre nominal de un agente no pueden suplirla. Un error previo a constituir el dominio no recibe una atribución inventada. La falta de la ligadura necesaria se hace explícita, sin presentar como comprobado un significado profesional que el Lenguaje no haya recibido. Un fallo de compilación sigue siendo tal; no se convierte en diagnóstico médico, valoración de seguridad ni resultado válido del dominio.

Para verificar esta aplicación se comprometerá una matriz de casos recibidos IMM/CYB × ES/EN en el alcance representable. Se conservarán idénticas las referencias profesionales y la decisión técnica al variar únicamente la superficie admitida y la explicación. El control discriminante sustituirá o confundirá una referencia o versión: si se afirma comprobar esa ligadura, el caso debe detectarse por su causa; si falta representación o comprobador, se registra la insuficiencia y no se atribuye una cobertura inexistente. El caso de nombres coincidentes entre dominios no autoriza equivalencias. Esta obligación desarrolla DG01/DG07/DG13 y el inventario de emisores; no declara nuevas operaciones ni una composición multidominio disponible.

**Estado:** precisión contractual en candidata; comprobaciones de realización, recepción de ligaduras pertinentes y entrega visible pendientes. No se modifican las constituciones ni los repositorios IMM/CYB, no se fabrica arquitectura de agente y no se presupone un cambio de gramática o IR. Cualquier insuficiencia representacional sigue el contraejemplo discriminante del acta de perfiles §7.
