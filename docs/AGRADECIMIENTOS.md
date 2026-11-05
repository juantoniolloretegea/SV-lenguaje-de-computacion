# Agradecimientos

El Lenguaje de Computación del Sistema Vectorial SV y sus herramientas asociadas se apoyan en proyectos, bibliotecas y tecnologías que contribuyen a su desarrollo, comprobación, gestión documental y representación. Agradecemos el trabajo de las comunidades y equipos que los crean, mantienen y documentan. Este reconocimiento comprende también las herramientas utilizadas en pruebas técnicas de inferencia, sin atribuirles por ello una función activa en todas las instalaciones.

La relación se presenta en **orden alfabético**, sin clasificación de importancia. Cada entrada indica el ámbito de su aportación:

- **[Alpine Linux](https://alpinelinux.org/)**: sistema invitado de las redes virtuales de ensayo. Se ha utilizado la edición 3.24.3; las licencias de sus componentes se conservan individualmente.
- **[anyhow](https://github.com/dtolnay/anyhow)**: tratamiento y descripción de errores en herramientas auxiliares y adaptadores de inferencia en Rust.
- **[Apache OpenDAL](https://opendal.apache.org/)**: abstracción de almacenamiento incorporada a Rustic para sus transportes. Su presencia no acredita que un destino remoto esté configurado o recibido para las copias del SV.
- **[Axum](https://github.com/tokio-rs/axum)**: organización de las rutas y atención de las solicitudes del servicio web del Gestor.
- **[base64](https://github.com/marshallpierce/rust-base64)**: codificación de datos binarios en los formatos de intercambio que la requieren.
- **[bubblewrap](https://github.com/containers/bubblewrap)**: separación de archivos, procesos y redes mediante espacios de nombres Linux para comprobar recintos de ensayo. Se ha utilizado la versión 0.11.1; licencia GNU LGPL versión 2 o posterior. La cobertura se limita a la configuración y a las comprobaciones recibidas; no constituye una garantía universal de aislamiento.
- **[BusyBox](https://busybox.net/)**: utilidades del sistema invitado para comprobar servicios y conectividad con datos sintéticos en las redes de ensayo; se conservan sus condiciones GNU GPL v2.
- **[Candle](https://github.com/huggingface/candle)**: bibliotecas de Hugging Face para cálculo tensorial y ejecución de modelos en Rust, utilizadas por la traducción auxiliar local y por motores de inferencia empleados en ensayos.
- **[Cargo](https://doc.rust-lang.org/cargo/)**: organización de los componentes Rust, sus dependencias, construcción y pruebas.
- **[cargo-audit y RustSec](https://rustsec.org/)**: contraste de dependencias con avisos públicos de seguridad y conservación del resultado de la revisión.
- **[Chrono](https://github.com/chronotope/chrono)**: fechas y marcas temporales en la instrumentación auxiliar de ensayos; su uso no sustituye el reloj monotónico que mide duraciones.
- **[Clippy](https://doc.rust-lang.org/clippy/)**: análisis estático y ayuda para revisar la claridad y calidad del código Rust.
- **[Cloudflare](https://developers.cloudflare.com/)**: servicios de publicación y protección del alojamiento web institucional. Su uso corresponde a la instalación que los configure.
- **[cookie](https://github.com/SergioBenitez/cookie-rs)**: construcción e interpretación de cookies HTTP en el servicio web.
- **[dataview](https://github.com/CasualX/dataview)**: tipos y vistas binarias utilizados por pelite para interpretar los bytes del ejecutable adquirido. Versión 1.1.0; licencia MIT.
- **[egui y eframe](https://github.com/emilk/egui)**: interfaces y visualizadores en Rust para presentar los resultados del SV.
- **[flate2](https://github.com/rust-lang/flate2-rs)**: lectura en flujo de archivos comprimidos para comprobar la recuperación de las fuentes, sin extraer rutas al sistema.
- **[Fontations y Skrifa](https://github.com/googlefonts/fontations)**: lectura de fuentes tipográficas y sus métricas en el tratamiento documental de PDF.
- **[Git](https://git-scm.com/)**: identificación de revisiones y conservación de la evolución del código y los documentos.
- **[GitHub](https://github.com/)**: alojamiento de fuentes y documentación, identificación de revisiones y servicios de comprobación automatizada en los repositorios que los tengan habilitados.
- **[Google Drive](https://workspace.google.com/products/drive/)**: almacenamiento remoto de copias cifradas y secuenciales de los recursos del SV incluidos en el catálogo autorizado. La custodia automática y la recuperación con cotejo de integridad están comprobadas; la copia de bases y su política específica de conservación siguen pendientes de recepción.
- **[Harmony, de OpenAI](https://github.com/openai/harmony)**: formato de mensajes y representación de conversaciones utilizado en las pruebas de integración de modelos de la familia gpt-oss. Es un formato de intercambio, no un motor de inferencia.
- **[Hayro](https://github.com/LaurenzV/hayro)**: interpretación y representación gráfica de documentos PDF en Rust.
- **[Helsinki-NLP y OPUS-MT](https://huggingface.co/Helsinki-NLP/opus-mt-en-es)**: modelo de traducción inglés-español utilizado como herramienta auxiliar local del Gestor. Su función es apoyar la lectura; no sustituye el original ni realiza el consejo experto de un dominio.
- **[html-to-markdown](https://github.com/xberg-io/html-to-markdown)**: conversión de contenido HTML a Markdown en el tratamiento documental.
- **[hyper y hyper-util](https://github.com/hyperium/hyper)**: transporte HTTP del servicio, junto con los componentes del mismo ecosistema para cuerpos de mensajes y fechas HTTP.
- **[image](https://github.com/image-rs/image)**: lectura y tratamiento de imágenes en los componentes documentales y visuales.
- **[Intel VTune Profiler](https://www.intel.com/content/www/us/en/developer/tools/oneapi/vtune-profiler.html)**: análisis instrumental del soporte del ensayo; las capturas recibidas conservan sus límites y no acreditan por sí solas la observación continua de todos los contadores.
- **[intptr](https://github.com/CasualX/intptr)**: representación de direcciones tipadas requerida por pelite. Versión 1.0.1; licencia MIT. Esta dependencia no concede acceso a la memoria de otros procesos.
- **[Jiff](https://github.com/BurntSushi/jiff)**: tratamiento de fechas, horas y zonas horarias en el Gestor; su uso instrumental no modifica las primitivas del SV.
- **[jq](https://jqlang.org/)**: consulta y transformación auxiliar de registros en notación de objetos de JavaScript (JSON). Versión incorporada: 1.8.1; licencia MIT para el programa principal, con avisos específicos de documentación y componentes.
- **[jsonwebtoken](https://github.com/Keats/jsonwebtoken)**: interpretación y verificación de credenciales estructuradas JWT en los recorridos de autenticación que las utilizan.
- **[Libre Hardware Monitor](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor)**: lectura de sensores del equipo de ensayo; cada señal se interpreta según su disponibilidad y procedencia comprobadas.
- **[Linux](https://www.kernel.org/)**: núcleo del sistema operativo utilizado en entornos de construcción, comprobación y ejecución del lenguaje y de sus adaptadores.
- **[lopdf](https://github.com/J-F-Liu/lopdf)**: lectura y análisis de la estructura de documentos PDF.
- **[MariaDB (cliente)](https://mariadb.org/)**: cliente de acceso a bases de datos en entornos de ensayo autorizados. Versión incorporada: 11.8.6; licencia GNU GPL versión 2 y avisos de sus componentes. La disponibilidad del cliente no acredita instalación de un servidor.
- **[mdBook](https://rust-lang.github.io/mdBook/)**: construcción y navegación de los libros de documentación.
- **[Mermaid](https://github.com/mermaid-js/mermaid)**: descripción textual y representación de diagramas de arquitectura, organización y procesos en la documentación.
- **[MiniJinja](https://github.com/mitsuhiko/minijinja)**: aplicación de plantillas de conversación en las pruebas de integración de motores de inferencia.
- **[mistral.rs](https://github.com/EricLBuehler/mistral.rs)**: motor de inferencia en Rust utilizado en pruebas técnicas de integración y observabilidad. Esta mención corresponde al proyecto de software, no a una recomendación de modelos de Mistral AI.
- **[netstat2](https://github.com/ohadravid/netstat2-rs)**: observación de conexiones TCP y UDP asociadas al proceso en la instrumentación de pruebas.
- **[Nmap](https://nmap.org/)**: instrumento incorporado para descubrimiento de recursos y comprobación de servicios en ámbitos autorizados. Versión 7.98; sus condiciones de licencia incluyen aclaraciones, excepciones y límites específicos de integración y redistribución. No se describe como una licencia GPL sin matices ni se presume autorización para incorporarlo a una distribución del SV.
- **[OpenAI](https://openai.com/)**: tecnología de inteligencia artificial utilizada como apoyo al análisis, la programación y la revisión documental; también modelos abiertos empleados en pruebas de integración.
- **[OpenTelemetry Rust](https://github.com/open-telemetry/opentelemetry-rust)**: instrumentación y correlación de operaciones en los ensayos de inferencia y en el acceso documental.
- **[PawnIO](https://github.com/namazso/PawnIO)**: controlador utilizado por la lectura de sensores del soporte Windows; su reconocimiento no atribuye autoridad decisoria sobre el SV.
- **[pdf-extract](https://github.com/jrmuizel/pdf-extract)**: extracción de texto de documentos PDF, base de la adaptación documental utilizada por el SV.
- **[pelite](https://github.com/CasualX/pelite)**: lectura de cabeceras y recursos de versión del formato ejecutable portátil (PE), a partir de los bytes adquiridos del archivo. Se ha incorporado la revisión [798566be15b5f84bb10f0894a723dcb6b25fd24d](https://github.com/CasualX/pelite/tree/798566be15b5f84bb10f0894a723dcb6b25fd24d), correspondiente al manifiesto 0.10.0; licencia MIT. La lectura no ejecuta el archivo ni certifica su procedencia, integridad editorial o inocuidad.
- **[PHP](https://www.php.net/)**: entorno de aplicaciones de prueba, separado del gobierno del SV realizado en Rust. Versión incorporada: 8.5.4; se conservan PHP License 3.01, Zend Engine License 2.00 y los avisos de sus componentes.
- **[pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark)**: interpretación de Markdown para su presentación documental.
- **[QEMU](https://www.qemu.org/)**: emulación de equipos y enlaces virtuales aislados en las comprobaciones locales. Se ha utilizado la versión 10.2.1; su licencia principal es GNU GPL v2, con las excepciones y licencias de componentes recogidas por el proyecto.
- **[rand](https://github.com/rust-random/rand)**: generación de valores aleatorios en los mecanismos que la requieren.
- **[raw-cpuid](https://github.com/gz/rust-cpuid)**: consulta desde Rust de la identificación, topología y geometría de caché declaradas por la unidad central de procesamiento. No equivale a medir su actividad ni su tiempo efectivo de habilitación.
- **[redb](https://github.com/cberner/redb)**: base de datos integrada en Rust utilizada para los registros persistentes del Gestor y componentes asociados.
- **[Reqwest](https://github.com/seanmonstar/reqwest)**: cliente HTTP para las comunicaciones expresamente habilitadas por la aplicación.
- **[Rust](https://rust-lang.org/)**: lenguaje, compilador y biblioteca estándar sobre los que se realiza el Lenguaje de Computación SV.
- **[RustCrypto](https://github.com/RustCrypto)**: bibliotecas de resumen criptográfico y autenticación de mensajes, como SHA-2 y HMAC, y utilidades como zeroize para el borrado de valores sensibles en memoria.
- **[rustfmt](https://github.com/rust-lang/rustfmt)**: presentación uniforme de las fuentes Rust.
- **[Rustic](https://github.com/rustic-rs/rustic)**: herramienta en Rust utilizada para el cifrado, la deduplicación y la recuperación de las copias del catálogo autorizado. Su transporte a Google Drive está incorporado a la custodia automática del SV, con restitución e integridad comprobadas para esos recursos; no acredita por sí solo la copia o recuperación funcional de las bases.
- **[Rustinel](https://github.com/Karib0u/rustinel)**: recolector nativo de sucesos y salud de observación en Windows, utilizado en comprobaciones pasivas delimitadas. Versión 1.10.0; licencia Apache-2.0. La recepción de datos no equivale a prevención activa ni a observación completa del equipo.
- **[rustix](https://github.com/bytecodealliance/rustix)**: acceso desde Rust a funciones del sistema operativo utilizadas por los controles de procesos.
- **[Rustls](https://github.com/rustls/rustls)**: protección TLS de comunicaciones, con las bibliotecas criptográficas y de comprobación de certificados que incorpora la configuración utilizada.
- **[rustup](https://rust-lang.github.io/rustup/)**: gestión de las versiones de Rust y de sus componentes de desarrollo.
- **[semver](https://github.com/dtolnay/semver)**: comparación explícita de versiones y delimitación de series compatibles.
- **[Serde y serde_json](https://serde.rs/)**: lectura y escritura de datos estructurados en componentes, herramientas documentales y visualizadores asociados.
- **[sysinfo](https://github.com/GuillaumeGomez/sysinfo)**: obtención de información del sistema y de sus recursos para la telemetría y las comprobaciones de hardware.
- **[tar-rs](https://github.com/alexcrichton/tar-rs)**: lectura delimitada de archivos de recuperación y cotejo de los contenidos inventariados, sin ejecutar ni instalar sus entradas.
- **[tcpdump](https://www.tcpdump.org/)**: instrumento incorporado para captura e inspección de tráfico autorizado. Versión 4.99.6; licencia BSD de tres cláusulas como principal, conservando los avisos específicos de otros componentes.
- **[Tokenizers](https://github.com/huggingface/tokenizers)**: bibliotecas de Hugging Face para convertir texto y secuencias de tokens en las herramientas de traducción e integración de modelos.
- **[Tokio](https://github.com/tokio-rs/tokio)**: ejecución de tareas asíncronas y coordinación de operaciones de entrada y salida en servicios, herramientas de inferencia y comprobaciones concurrentes de catálogos.
- **[toml_edit](https://github.com/toml-rs/toml)**: actualización acotada de los manifiestos de dependencias conservando su estructura.
- **[Tower](https://github.com/tower-rs/tower)**: utilidades para comprobar los servicios HTTP del Gestor.
- **[TShark](https://www.wireshark.org/)**: analizador de protocolos y capturas del proyecto Wireshark. Versión incorporada: 4.6.4; licencia GNU GPL versión 2 o posterior como principal, con avisos específicos de sus componentes.
- **[ttf-parser](https://github.com/harfbuzz/ttf-parser)**: lectura de estructuras y métricas de fuentes tipográficas en el tratamiento documental.
- **[Ubuntu](https://ubuntu.com/)**: entorno Linux de construcción y ejecución de instrumentos y máquinas virtuales. Sus paquetes y bibliotecas conservan sus licencias propias.
- **[unicode-normalization](https://github.com/unicode-rs/unicode-normalization)**: normalización Unicode en la extracción y el tratamiento de texto documental.
- **[UserWay](https://userway.org/)**: herramienta complementaria de accesibilidad incorporada a la web institucional; no sustituye la accesibilidad propia de la interfaz.
- **[Vector](https://vector.dev/)**: adquisición de métricas de procesador, memoria y otros recursos del anfitrión, utilizada por el panel de supervisión y recibida en ciclos delimitados de observación en Windows. Versión 0.61.0; licencia Mozilla Public License 2.0. La cobertura corresponde a las fuentes y configuraciones comprobadas; no acredita por sí sola supervisión permanente, observación de otros equipos ni detección completa de incidentes.
- **[WASI](https://wasi.dev/)**: interfaces del sistema para comprobar la ejecución WebAssembly fuera del navegador, dentro de las capacidades concedidas por el entorno.
- **[wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/)**: comunicación entre WebAssembly y JavaScript en los visualizadores web asociados.
- **[Wasmtime](https://github.com/bytecodealliance/wasmtime)**: entorno de ejecución utilizado para comprobar componentes WebAssembly con WASI. Puede emplearse en equipos locales y servidores.
- **[WebAssembly](https://webassembly.org/)**: formato de ejecución que permite distribuir componentes Rust del lenguaje en el navegador y en otros entornos compatibles.
- **[Windows Performance Recorder](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/windows-performance-recorder)**: captura de sucesos del sistema y las aplicaciones mediante el seguimiento de eventos de Windows (ETW), utilizada como evidencia instrumental del soporte.
- **[zip](https://github.com/zip-rs/zip2)**: lectura y preparación de archivos ZIP, incluidas las copias descargables previstas por el Gestor.

Cada aportación se reconoce en su ámbito. La relación no sustituye el inventario de dependencias y versiones de cada distribución, ni atribuye a sus responsables participación en las decisiones, certificación o aval del SV. Se reconoce asimismo el trabajo de quienes mantienen las dependencias indirectas de estos proyectos. Sus licencias y atribuciones se conservan en las fuentes y distribuciones correspondientes; la licencia de este documento no las modifica.

La relación se revisará cuando cambien las herramientas utilizadas, conforme a la [política de mantenimiento de dependencias y publicación de versiones](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/POLITICA-DEPENDENCIAS-Y-VERSIONES.md). La [relación de referencia en GitHub](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/AGRADECIMIENTOS.md) mantiene el mismo contenido y orden.

## Identificación y alcance del reconocimiento

Esta página ofrece metadatos públicos que describen la misma relación de proyectos y sus enlaces oficiales. Facilitan su identificación por buscadores; no garantizan su indexación ni notifican automáticamente a los responsables de los proyectos. El reconocimiento no establece patrocinio, colaboración contractual ni aval de sus titulares.

El [inventario acotado de la instrumentación del ensayo práctico](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/estandar-comparativo-o-benchmark/ediciones/ciberseguridad/eet-cyb16-practica-20261027/practicas/COMPONENTES-Y-AGRADECIMIENTOS.md) identifica versiones, licencias y funciones efectivamente utilizadas. Las referencias de arquitectura estudiadas y no incorporadas se indican por separado.

Revisión: 5 de noviembre de 2026.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

