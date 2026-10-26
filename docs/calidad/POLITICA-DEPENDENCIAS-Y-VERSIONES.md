# Política de mantenimiento de dependencias y publicación de versiones

Sistema Vectorial SV · Versión 1.0 · 26 de octubre de 2026.

Esta política desarrolla la norma de [Código limpio y auditable](tuberias-ia/continuacion-15-09-2026/CODIGO-LIMPIO-Y-AUDITABLE.md). Se aplica a los paquetes Rust propios —crates—, sus dependencias directas e indirectas y las herramientas externas que intervienen en una distribución. Su publicación establece el procedimiento; no acredita que todas las dependencias existentes hayan sido actualizadas o auditadas, ni activa una vigilancia automatizada.

## 1. Inventario y estados verificables

Cada distribución identifica revisión de fuentes, versión propia, `Cargo.toml`, `Cargo.lock`, versión exacta de Rust y Cargo, edición, versión mínima de Rust, destino, características habilitadas, origen y huellas de los productos. El inventario distingue lo declarado, resuelto, construido, instalado y efectivamente utilizado. Incluye herramientas distribuidas por separado, como mdBook, Wasmtime o Rustic, y los componentes nativos de las fronteras FFI. Los agradecimientos son una relación editorial, no un inventario completo de componentes ni una prueba de seguridad.

Se conserva un inventario legible por máquinas, exportable en un formato de lista de materiales de software cuando exista un generador recibido. No se declara disponible una lista completa por el mero hecho de conservar `Cargo.lock`. Las dependencias de desarrollo y construcción también se examinan, pues pueden intervenir antes de ejecutar el producto final. No se publican secretos ni datos de las instalaciones en los inventarios públicos.

## 2. Revisión periódica y vulnerabilidades

El responsable técnico designado revisa semanalmente los avisos aplicables de [RustSec](https://rustsec.org/), los avisos oficiales de seguridad y las notas de versión de los proyectos utilizados. Realiza además una revisión mensual de mantenimiento, abandono de proyectos, compatibilidad y versiones de Rust admitidas. Un aviso recibido sobre explotación activa, exposición de datos, autenticación, integridad o recuperación se examina sin esperar a la siguiente revisión periódica. La falta de responsable o de ejecución se registra como pendiente; no se muestra un estado favorable vencido.

La revisión relaciona identificador y fecha del aviso, versiones afectadas y corregidas, componente real, características compiladas, recorridos alcanzables, consecuencia justificada, mitigación, responsable y fecha de nueva revisión. Un resultado sin coincidencias no acredita ausencia de vulnerabilidades desconocidas. La falta de información no se clasifica como riesgo menor. Las escalas externas conservan su significado y no se convierten en niveles clínicos ni en valores de la terna del SV.

`cargo-audit` permite contrastar `Cargo.lock` con RustSec; `cargo-deny` añade controles sobre licencias, procedencias y versiones. Su incorporación requiere fijar y comprobar la herramienta y sus reglas. Si no está disponible o la base de avisos no es vigente, se declara la limitación y se realiza un contraste documentado con las fuentes oficiales; no se simula una comprobación favorable. Una excepción tiene alcance, fundamento, responsable, medida compensatoria y vencimiento. No se ignora globalmente un aviso para superar una comprobación.

Si no existe corrección, se determina una mitigación verificable: restringir la función afectada, aislar el recorrido o suspender su uso según la consecuencia. La decisión sobre disponibilidad clínica corresponde al responsable del dominio con la información técnica necesaria; una instalación técnicamente correcta no habilita por sí sola un uso médico.

## 3. Selección de cambios y compatibilidad

Las actualizaciones se preparan en un entorno separado, por componente o conjunto coherente, con comparación de notas de versión, licencias, características, dependencias indirectas, límites de recursos y compatibilidad. No se ejecuta una actualización general sin alcance definido ni se sustituye una versión por `latest` en una instalación recibida.

La [compatibilidad semántica de Cargo](https://doc.rust-lang.org/cargo/reference/semver.html) orienta la evaluación, pero no garantiza equivalencia funcional, rendimiento ni seguridad. En versiones `0.x`, un cambio del segundo número puede ser incompatible; un parche también necesita comprobación. La versión propia distingue corrección compatible, ampliación compatible y ruptura del contrato; el estado experimental y las incompatibilidades de una serie `0.x` se explicitan.

Una dependencia no autoriza modificar gramática, representación intermedia, invariantes, rutas críticas ni contratos del núcleo. Si la actualización exige ese cambio, se separa y somete a su autoridad competente. Tampoco habilita comunicaciones con modelos, complementos, MCP, módulos ajenos ni nuevas facultades institucionales.

## 4. Condiciones de recepción

Antes de aceptar un cambio se requieren las comprobaciones pertinentes sobre la revisión exacta:

1. Construcción con resolución fijada (`--locked`), versión y destinos declarados; comprobación de formato con rustfmt y análisis con Clippy, sin silenciamientos generales.
2. Pruebas del comportamiento afectado, entradas inválidas, errores y límites. Cuando corresponda, autorización y revocación, separación de dominios, concurrencia, interrupción, aislamiento de comunicaciones y fronteras FFI.
3. Contraste de avisos de seguridad y licencias, cambios directos e indirectos y excepciones pendientes. Se conserva la fecha de la base consultada.
4. Medidas comparables de tiempo, memoria, entrada/salida y almacenamiento cuando el cambio pueda afectar a la respuesta del servicio. Una actualización no justifica eliminar controles para recuperar velocidad.
5. Compatibilidad de formatos persistentes, documentos, visualizadores y datos anteriores. Las migraciones requieren copia coherente y restauración aislada comprobada. Volver al ejecutable anterior no basta si los datos ya cambiaron de formato.
6. Revisión competente y recepción con alcance y límites. Para un dominio clínico se conserva la autorización institucional y profesional; la recepción técnica no la sustituye.

Las pruebas no realizadas permanecen identificadas. `cargo check`, `cargo build`, `cargo test`, rustfmt y Clippy aportan evidencias distintas. No se repiten baterías costosas sin un cambio o incertidumbre que lo justifique. Los ensayos respetan los límites económicos, de privacidad y de ejecución vigentes.

## 5. Publicación, incorporación y reversión

Cada versión publicada reúne notas del cambio, compatibilidad, incidencias conocidas, dependencias y herramientas fijadas, licencias y atribuciones, instrucciones de construcción y recuperación, huellas de los archivos y evidencia de las comprobaciones. Las etiquetas identifican una revisión inmutable; una corrección posterior produce otra versión, sin sustituir silenciosamente un archivo ya entregado. Una firma sólo se declara verificada cuando exista evidencia de su comprobación.

Se distinguen preparación, recepción, publicación e instalación. Publicar una versión no la instala ni autoriza producción. La entrega conserva la versión anterior necesaria para una reversión recibida y establece cómo detener el proceso si falla una comprobación. Una restauración no repite comunicaciones, notificaciones ni otras operaciones externas ya realizadas.

La política no autoriza por sí sola publicar paquetes en crates.io, abrir ramas, activar actualizaciones automáticas, aceptar licencias nuevas, contratar servicios ni consumir recursos adicionales. Las automatizaciones sólo se habilitan con alcance y permisos recibidos; su existencia no sustituye la evidencia de ejecución. Se actualizan la documentación competente y los agradecimientos cuando cambia la aportación real de un proyecto.

## 6. Registro mínimo y consecuencias de omisión

El registro de mantenimiento contiene componente, versiones anterior y propuesta, fuente del cambio, aviso de seguridad si existe, alcance afectado, decisión y responsable, comprobaciones y resultados, exclusiones, revisión publicada, instalaciones afectadas y próxima revisión. Los sucesos remiten a la evidencia, sin duplicarla en cada registro.

| Omisión | Consecuencia potencial justificada | Control y evidencia exigibles |
| --- | --- | --- |
| No revisar avisos vigentes | Mantener un recorrido afectado por una vulnerabilidad conocida. | Fecha de revisión, correspondencia con la versión utilizada y resolución documentada. |
| Actualizar sin conocer características o dependencias indirectas | Añadir comportamientos, comunicaciones o licencias no examinados. | Comparación de resolución, características, orígenes y recorridos. |
| Confundir compilación con recepción | Incorporar errores funcionales o de autorización no detectados por el compilador. | Resultados de las pruebas pertinentes y límites expresos. |
| Omitir migración y recuperación | Impedir la apertura de datos anteriores o perder coherencia al revertir. | Restitución aislada, compatibilidad y procedimiento de reversión. |
| Alterar contratos del núcleo por conveniencia de una dependencia | Introducir una semántica no autorizada. | Revisión contra las fuentes rectoras y separación del cambio que requiere decisión. |

Se aplica la norma [Consecuencias de la omisión: fuentes y delimitación](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/estandar-comparativo-o-benchmark/CONSECUENCIAS-Y-FUENTES.md). No se atribuye conformidad a las partes no examinadas.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
