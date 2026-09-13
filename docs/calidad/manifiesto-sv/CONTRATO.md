# Consulta incorporada del manifiesto SV y ejecución reproducible

Versión 1 · 13 de septiembre de 2026 · Juan Antonio Lloret Egea y Watson

## Objeto y contrato previo al ensayo

Sobre Lenguaje `8893707154e5fd6e623904a71345268339eb6550`, se incorpora una consulta documental de solo lectura. El comando independiente `manifiesto-sv` y `sv-native manifiesto-sv` deben devolver exactamente los 19 134 bytes UTF-8 del acta de 26/03/2026, SHA-256 `31ae77a963718ac72bbb74a4aac4fd06d788117167162835e0652f9fb2a60056`. Se conserva el texto completo, incluida la capa Rosetta subordinada; no se ejecuta esa capa ni se altera su estatuto histórico.

El fichero se incluye dentro de `sv_core` mediante `include_str!` durante la compilación. Los adaptadores consumen esa misma fuente. La consulta no efectúa peticiones de red ni abre el documento durante la ejecución. La ausencia del recurso al construir debe impedir la compilación; su alteración respecto del acta canónica debe impedir la conformidad. La consulta no añade palabras reservadas, objetos de IR ni decisiones algebraicas. No acredita por sí misma detección o bloqueo de usos prohibidos: esas obligaciones conservan el estatuto del acta.

## Superficies

- API Rust: `sv_core::manifiesto_sv() -> &'static str`.
- CLI: `manifiesto-sv` sin argumentos, o `sv-native manifiesto-sv` sin argumentos adicionales; salida íntegra a stdout, código 0. Argumentos incompatibles: código 2 y stdout vacío. Fallo de escritura: código 2; nunca `Tri.U`.
- WASI: los mismos binarios con argumentos equivalentes, sin directorios preabiertos para esta consulta.
- Adaptador WASM: `sv_manifiesto_sv() -> u64` usa el contrato existente de posición/longitud del búfer de salida. Se copia inmediatamente la salida antes de otra llamada que reutilice el búfer; no adquiere autoridad semántica.

El texto español es canónico conforme al §7 del acta. Los perfiles fuente SVP-ES/SVP-EN seleccionan sintaxis de programas y no traducen este documento. Los comentarios nuevos explican en español e inglés finalidad, restricciones y contrato. `manifiesto-sv` designa un comando de distribución, no una instrucción añadida a `.svp`. Un archivo de ese nombre sin extensión se compila por su ruta explícita `./manifiesto-sv` si fuera necesario.

## Entorno de ejecución

Se instala Rust 1.98.0 mediante rustup oficial en el entorno de trabajo remoto, separado del ordenador del autor. Es la referencia ya fijada por los flujos vigentes; no se sustituye por el canal mutable stable del Playground. Se registran `rustc --version --verbose`, `cargo --version`, destinos, host, fuente y resultados. El Playground sirve para ejemplos aislados; no acredita por sí solo el workspace completo ni su corpus. La instalación de este entorno es recuperable, pero su permanencia entre sesiones no se presume: el procedimiento conservado permite reponerla.

## Condiciones de aceptación y alcance

1. Identidad literal entre acta canónica, copia incorporada, CLI independiente y subcomando, WASI y adaptador WASM.
2. Ejecución del binario copiado a un directorio aislado sin fuentes y del módulo WASI sin acceso a ficheros; salida ajena a variables de entorno o ficheros señuelo.
3. Rechazo de argumentos adicionales y fallo de stdout identificable; el observador debe detectar al menos alteración de un byte y truncamiento del documento.
4. Conservación de las pruebas del workspace, conformidad SV y paridad exigible. El manifiesto no se presenta como nuevo resultado del banco BIS-02.
5. Construcción reproducible mediante comandos conservados; registro de dependencias reales y separación entre infraestructura disponible y garantías no ensayadas.

La normativa se ha leído en AGENTS, Pilares, acta de perfiles y contratos y acta de transición completa, además del acta objeto de incorporación. El informe de resultados distinguirá preparación, ejecución observada y limitaciones. La secuencia (p1+p3)-Bis continúa desde BIS-02 una vez resuelto este requisito instrumental.
