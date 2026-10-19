# Agradecimientos

El Lenguaje de Computación del Sistema Vectorial SV y sus herramientas asociadas se apoyan en proyectos, bibliotecas y tecnologías que contribuyen a su desarrollo, comprobación, gestión documental y representación. Agradecemos el trabajo de las comunidades y equipos que los crean, mantienen y documentan. Este reconocimiento comprende también las herramientas utilizadas en pruebas técnicas de inferencia, sin atribuirles por ello una función activa en todas las instalaciones.

La relación se presenta en **orden alfabético**, sin clasificación de importancia. Cada entrada indica el ámbito de su aportación:

- **[anyhow](https://github.com/dtolnay/anyhow)**: tratamiento y descripción de errores en herramientas auxiliares y adaptadores de inferencia en Rust.
- **[Axum](https://github.com/tokio-rs/axum)**: organización de las rutas y atención de las solicitudes del servicio web del Gestor.
- **[base64](https://github.com/marshallpierce/rust-base64)**: codificación de datos binarios en los formatos de intercambio que la requieren.
- **[Candle](https://github.com/huggingface/candle)**: bibliotecas de Hugging Face para cálculo tensorial y ejecución de modelos en Rust, utilizadas por la traducción auxiliar local.
- **[Cargo](https://doc.rust-lang.org/cargo/)**: organización de los componentes Rust, sus dependencias, construcción y pruebas.
- **[Clippy](https://doc.rust-lang.org/clippy/)**: análisis estático y ayuda para revisar la claridad y calidad del código Rust.
- **[Cloudflare](https://developers.cloudflare.com/)**: servicios de publicación y protección del alojamiento web institucional. Su uso corresponde a la instalación que los configure.
- **[cookie](https://github.com/SergioBenitez/cookie-rs)**: construcción e interpretación de cookies HTTP en el servicio web.
- **[egui y eframe](https://github.com/emilk/egui)**: interfaces y visualizadores en Rust para presentar los resultados del SV.
- **[Fontations y Skrifa](https://github.com/googlefonts/fontations)**: lectura de fuentes tipográficas y sus métricas en el tratamiento documental de PDF.
- **[Git](https://git-scm.com/)**: identificación de revisiones y conservación de la evolución del código y los documentos.
- **[GitHub](https://docs.github.com/en/actions)**: alojamiento de las fuentes y ejecución de las comprobaciones automatizadas declaradas por el proyecto.
- **[Harmony, de OpenAI](https://github.com/openai/harmony)**: formato de mensajes y representación de conversaciones utilizado en las pruebas de integración de modelos de la familia gpt-oss. Es un formato de intercambio, no un motor de inferencia.
- **[Hayro](https://github.com/LaurenzV/hayro)**: interpretación y representación gráfica de documentos PDF en Rust.
- **[Helsinki-NLP y OPUS-MT](https://huggingface.co/Helsinki-NLP/opus-mt-en-es)**: modelo de traducción inglés-español utilizado como herramienta auxiliar local del Gestor. Su función es apoyar la lectura; no sustituye el original ni realiza el consejo experto de un dominio.
- **[html-to-markdown](https://github.com/xberg-io/html-to-markdown)**: conversión de contenido HTML a Markdown en el tratamiento documental.
- **[hyper y hyper-util](https://github.com/hyperium/hyper)**: transporte HTTP del servicio, junto con los componentes del mismo ecosistema para cuerpos de mensajes y fechas HTTP.
- **[image](https://github.com/image-rs/image)**: lectura y tratamiento de imágenes en los componentes documentales y visuales.
- **[jsonwebtoken](https://github.com/Keats/jsonwebtoken)**: interpretación y verificación de credenciales estructuradas JWT en los recorridos de autenticación que las utilizan.
- **[lopdf](https://github.com/J-F-Liu/lopdf)**: lectura y análisis de la estructura de documentos PDF.
- **[mdBook](https://rust-lang.github.io/mdBook/)**: construcción y navegación de los libros de documentación.
- **[Mermaid](https://github.com/mermaid-js/mermaid)**: descripción textual y representación de diagramas de arquitectura, organización y procesos en la documentación.
- **[MiniJinja](https://github.com/mitsuhiko/minijinja)**: aplicación de plantillas de conversación en las pruebas de integración de motores de inferencia.
- **[mistral.rs](https://github.com/EricLBuehler/mistral.rs)**: motor de inferencia en Rust utilizado en pruebas técnicas de integración y observabilidad. Esta mención corresponde al proyecto de software, no a una recomendación de modelos de Mistral AI.
- **[OpenAI](https://openai.com/)**: tecnología de inteligencia artificial utilizada como apoyo al análisis, la programación y la revisión documental; también modelos abiertos empleados en pruebas de integración.
- **[pdf-extract](https://github.com/jrmuizel/pdf-extract)**: extracción de texto de documentos PDF, base de la adaptación documental utilizada por el SV.
- **[pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark)**: interpretación de Markdown para su presentación documental.
- **[rand](https://github.com/rust-random/rand)**: generación de valores aleatorios en los mecanismos que la requieren.
- **[redb](https://github.com/cberner/redb)**: base de datos integrada en Rust utilizada para los registros persistentes del Gestor y componentes asociados.
- **[Reqwest](https://github.com/seanmonstar/reqwest)**: cliente HTTP para las comunicaciones expresamente habilitadas por la aplicación.
- **[Rust](https://rust-lang.org/)**: lenguaje, compilador y biblioteca estándar sobre los que se realiza el Lenguaje de Computación SV.
- **[RustCrypto](https://github.com/RustCrypto)**: bibliotecas de resumen criptográfico y autenticación de mensajes, como SHA-2 y HMAC, y utilidades como zeroize para el borrado de valores sensibles en memoria.
- **[rustfmt](https://github.com/rust-lang/rustfmt)**: presentación uniforme de las fuentes Rust.
- **[rustix](https://github.com/bytecodealliance/rustix)**: acceso desde Rust a funciones del sistema operativo utilizadas por los controles de procesos.
- **[Rustls](https://github.com/rustls/rustls)**: protección TLS de comunicaciones, con las bibliotecas criptográficas y de comprobación de certificados que incorpora la configuración utilizada.
- **[rustup](https://rust-lang.github.io/rustup/)**: gestión de las versiones de Rust y de sus componentes de desarrollo.
- **[Serde y serde_json](https://serde.rs/)**: lectura y escritura de datos estructurados en componentes, herramientas documentales y visualizadores asociados.
- **[sysinfo](https://github.com/GuillaumeGomez/sysinfo)**: obtención de información del sistema y de sus recursos para la telemetría y las comprobaciones de hardware.
- **[Tokenizers](https://github.com/huggingface/tokenizers)**: bibliotecas de Hugging Face para convertir texto y secuencias de tokens en las herramientas de traducción e integración de modelos.
- **[Tokio](https://github.com/tokio-rs/tokio)**: ejecución de tareas asíncronas y coordinación de operaciones de entrada y salida en servicios y herramientas de inferencia.
- **[Tower](https://github.com/tower-rs/tower)**: utilidades para comprobar los servicios HTTP del Gestor.
- **[unicode-normalization](https://github.com/unicode-rs/unicode-normalization)**: normalización Unicode en la extracción y el tratamiento de texto documental.
- **[UserWay](https://userway.org/)**: herramienta complementaria de accesibilidad incorporada a la web institucional; no sustituye la accesibilidad propia de la interfaz.
- **[wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/)**: comunicación entre WebAssembly y JavaScript en los visualizadores web asociados.
- **[zip](https://github.com/zip-rs/zip2)**: lectura y preparación de archivos ZIP, incluidas las copias descargables previstas por el Gestor.

Cada aportación se reconoce en su ámbito. La relación no sustituye el inventario de dependencias y versiones de cada distribución, ni atribuye a sus responsables participación en las decisiones, certificación o aval del SV. Se reconoce asimismo el trabajo de quienes mantienen las dependencias indirectas de estos proyectos. Sus licencias y atribuciones se conservan en las fuentes y distribuciones correspondientes; la licencia de este documento no las modifica.

La relación se revisará cuando cambien las herramientas utilizadas. Su presentación en el [Manual del Lenguaje de Computación](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/agradecimientos.html) mantiene el mismo contenido y orden.

Revisión: 19 de octubre de 2026.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
