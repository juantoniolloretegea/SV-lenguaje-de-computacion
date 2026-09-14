# S29 · Obligación de Rust y provisión verificable del entorno

**Instrucción recibida el 14 de septiembre de 2026. Unidad: Watson / W-S26.** Se incorpora después de publicar la recepción del banco S26-F01/F02 LIG, en Lenguaje `2ebd2215a803bcfb31b1425cba940bd2df19c56a` y laboratorio `32ab2bc5b7f279a03b078eea165c180fa7b20d60`.

## Obligación vigente

Por instrucción expresa del responsable, **Rust es obligatorio para todos los procesos del trabajo SV**, incluidos preparación, ejecución, observación, tratamiento de evidencias, registros y auxiliares. La denominación «auxiliar» no permite eludir esta obligación. Las nuevas realizaciones y la lógica propia de comprobación se escribirán y ejecutarán en Rust.

**Cada uso de Python deberá explicarse y argumentarse antes de ejecutarlo**: necesidad concreta, alternativa Rust considerada, razón de su insuficiencia o indisponibilidad, alcance de lectura/escritura y memoria, dependencias y comprobación del resultado. La comodidad no constituye justificación. Cualquier desviación debe quedar visible, con su alcance y disposición; no se autoriza una excepción tácita por llamar a un programa desde Rust.

La aplicación exige identificar también los programas, bibliotecas, compiladores, enlazadores, conectores y servicios que intervienen. Los procesos ajenos al código propio no se presentarán como escritos en Rust. El entorno de esta sesión ofrece herramientas de ejecución y el conector GitHub coordinados mediante JavaScript; Git, el enlazador del sistema y los utilitarios de identificación y conservación tienen funciones concretas declaradas. Esa infraestructura no acredita una realización íntegra en Rust ni exime de registrar su participación. La continuidad deberá conservar estas distinciones y resolver las dependencias necesarias sin sustituciones silenciosas.

La instrucción actual sustituye la formulación anterior de Rust como «primera opción» en el control de auxiliares. Los antecedentes y resultados históricos se conservan. Esta recepción no certifica que todos los auxiliares antiguos hayan sido migrados ni atribuye garantías de memoria al conjunto del host por el lenguaje elegido.

## Instalación y acceso comprobados

Entorno observado: **Ubuntu 24.04.3 LTS (Noble Numbat), x86_64**. Instalación directa, recibida en S25: `/opt/sv-rust-1.98.0`.

| Elemento | Acceso |
| --- | --- |
| Compilador | `/opt/sv-rust-1.98.0/bin/rustc` |
| Cargo | `/opt/sv-rust-1.98.0/bin/cargo` |
| Formato y Clippy | `/opt/sv-rust-1.98.0/bin/rustfmt`; `/opt/sv-rust-1.98.0/bin/clippy-driver` |
| Biblioteca estándar nativa | `/opt/sv-rust-1.98.0/lib/rustlib/x86_64-unknown-linux-gnu/lib` |
| Activación de consola | `. /opt/sv-rust-1.98.0/env.sh` |
| Estado fechado y renovable | [ESTADO_RUST.md](ESTADO_RUST.md) |
| Programa de consulta | [estado_rust.rs](estado_rust.rs) |

La consulta es de lectura y no depende de Python ni de red. Para renovarla desde la raíz del Lenguaje, compile el programa y ejecute la salida; revise después el informe antes de publicarlo:

```text
/opt/sv-rust-1.98.0/bin/rustc --edition=2021 docs/calidad/tuberias-ia/recuperacion-rust-s25/estado_rust.rs -o /tmp/sv-estado-rust
/tmp/sv-estado-rust docs/calidad/tuberias-ia/recuperacion-rust-s25/ESTADO_RUST.md
```

El registro web muestra la última comprobación publicada, con fecha. No es un monitor en tiempo real. Las versiones instaladas son Rust/Cargo 1.98.0; el banco LIG acaba de compilar y ejecutarse offline en dos perfiles. No se reejecuta ese banco para actualizar un registro instrumental.

## rustup para este Linux

La [documentación oficial de instalación manual](https://rust-lang.github.io/rustup/installation/other.html) identifica el instalador Linux correspondiente y su fichero de huella:

- [rustup-init para x86_64-unknown-linux-gnu](https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init).
- [SHA-256 oficial del instalador](https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init.sha256).
- [Página oficial de instalación](https://rust-lang.org/tools/install/).
- [Notas oficiales de versiones de Rust](https://doc.rust-lang.org/releases.html).

Para la descarga desde un ordenador Windows debe escogerse expresamente ese archivo Linux, sin extensión .exe. El usuario ha ofrecido aportar el archivo o su enlace. Su recepción verificará plataforma y huella antes de instalar. La huella no se ha descargado en esta recepción: el visor web no admite su tipo de contenido; no se inventa su valor.

Actualmente rustup no está localizado en PATH. La instalación directa de Rust funciona sin ese gestor. Incorporar rustup exigirá conservar y registrar el compilador 1.98.0 usado por los bancos, sus componentes y la precedencia en PATH; no se cambiará silenciosamente a la versión estable que exista entonces. Una actualización de herramienta deberá tener versión y comprobación propias. Los enlaces anteriores satisfacen la petición de localizar el instalador; no describen una instalación ya realizada.

## Provisión por etapa y continuidad

Antes de cada etapa se identificarán las versiones, destinos de compilación, bibliotecas, enlazadores y herramientas requeridos, con acceso reproducible. Se comprobará que el conjunto esté disponible; cualquier falta se resolverá y registrará antes de ejecutar. Queda prohibido suplir un componente ausente mediante un cambio silencioso de lenguaje, omitir la dependencia o presentar como conforme una ejecución incompleta.

La compilación nativa actual dispone de los componentes enumerados en el estado y está acreditada por la campaña LIG. `pkg-config` y `dotnet` no se localizan en PATH; este hecho no acredita ni niega su disponibilidad por rutas no inventariadas. No se declaran satisfechas dependencias de otras etapas ni de futuros destinos WASM/WASI. La evidencia y los archivos de recuperación quedan en los repositorios; la permanencia del contenedor no está bajo control del proyecto.

Por instrucción humana, **C# y .NET se provisionarán cuando corresponda la GUI**. S24 conserva el orden **Bis → catálogo de errores y cierre de fase → GUI**. La revisión de S24 recibe esa indicación y el requisito de bibliotecas y herramientas completas; las fechas de inicio y fin de GUI permanecen vacías. No se instala ni se selecciona ahora una versión o biblioteca de interfaz.

## Registro y alcance de cierre

S29 registra y finaliza esta incorporación de la obligación, la localización de herramientas, el informe renovable y los enlaces oficiales pedidos. Su cierre documental no declara instalado rustup ni completada una migración general. S24 mantiene la futura provisión de C#/.NET, y S26/Bis sus obligaciones materiales pendientes. RETP canónica permanece sin cambios y se utilizan las ramas y carpetas existentes.

La edición de Sucesos de esta incorporación se efectúa mediante [registrar_s29.rs](registrar_s29.rs), con conservación del historial, concordancia de CSV/Markdown y altas o revisiones identificadas. La publicación utiliza el conector GitHub; esa operación y el cotejo de árboles quedan explícitos. No se ejecuta Python en S29.
# Actualización instrumental S29 revisión 1

Rustup 1.29.1 ya está instalado y enlazado a Rust 1.98.0. [Recepción, procedencia, incidencias y acceso](RECEPCION_RUSTUP_S29.md). Las menciones siguientes a rustup no instalado describen el corte anterior de la revisión 0.
