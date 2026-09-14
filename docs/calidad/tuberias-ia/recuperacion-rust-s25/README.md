# S25 · Recuperación de Rust 1.98.0 y activación del entorno

**S29 revisión 1:** [rustup 1.29.1 instalado y enlazado a Rust 1.98.0](RECEPCION_RUSTUP_S29.md). Activación: `. /opt/sv-rustup/env.sh`. Los estados anteriores se conservan como antecedentes fechados.

**Continuidad S29, 14/09/2026:** [obligación de Rust y provisión por etapa](OBLIGACION_RUST_Y_PROVISION_DEL_ENTORNO_S29.md) · [último estado observado](ESTADO_RUST.md) · [consulta escrita en Rust](estado_rust.rs). Estas referencias son la entrada instrumental vigente; S25 conserva a continuación su evidencia histórica.

Juan Antonio Lloret Egea y Watson · 13 de septiembre de 2026 · RETP-2026-217

## Resultado y alcance

Instalación autónoma Linux x86_64 en `/opt/sv-rust-1.98.0`, con rustc y Cargo 1.98.0, biblioteca estándar nativa, rustfmt y Clippy. La descarga del paquete completo y su huella oficial pudieron completarse después de fallos de conexión del proxy. No se atribuye una causa resuelta permanentemente a esa intermitencia. El ejecutable Windows aportado antes era rustup-init y no se empleó para esta instalación Linux.

Paquete: https://static.rust-lang.org/dist/rust-1.98.0-x86_64-unknown-linux-gnu.tar.gz

SHA-256 cotejada con el fichero oficial: `aa30409afa67bd1ada244cefd82c7980e6a65bc113bb978e934b2413c75e3900`.

Comando de instalación ejecutado en el turno precedente, conservado aquí como registro posterior:

```sh
bash rust-1.98.0-x86_64-unknown-linux-gnu/install.sh --prefix=/opt/sv-rust-1.98.0 --components=rustc,cargo,rust-std-x86_64-unknown-linux-gnu,rustfmt-preview,clippy-preview --disable-ldconfig
```

El alta de S25 se realiza por petición humana después de iniciada la actividad. No representa un alta previa a la instalación. S23 conserva su cierre histórico. La fecha de inicio registrada corresponde a la configuración observada de este incremento; no se inventa una hora de inicio de la descarga o instalación anterior.

## Activación y comprobación

[env.sh](env.sh) declara `SV_RUST_PREFIX` y añade `bin` a `PATH` sin duplicarlo. [configurar.py](configurar.py) lo copia a la instalación y enlaza su carga desde los archivos de inicio de Bash, conservando su contenido anterior. [CONFIGURACION.json](CONFIGURACION.json) recoge huellas antes/después. No se fijan RUSTUP_HOME ni CARGO_HOME: esta instalación no utiliza rustup y conserva la configuración de Cargo existente.

Nuevas sesiones login y carga de bashrc verificadas. Para una consola que ya estuviera abierta o un ejecutor que no cargue archivos de inicio:

```sh
. /opt/sv-rust-1.98.0/env.sh
rustc --version
cargo --version
```

El directorio global `/usr/local/bin` rechazó la creación de enlaces por permisos en el turno anterior; se emplea la activación de usuario. Persistencia verificada en nuevas sesiones del entorno actual, no garantizada tras sustitución o limpieza del contenedor. El archivo activador y los registros quedan versionados para recuperación.

Sonda Rust edición 2024: compilación y ejecución offline con salida `SV_TOOLCHAIN_OK:100`, una prueba aprobada y rechazo de una cadena asignada a u32 mediante E0308. El programa sólo comprueba instalación; no representa un contrato SV.

## Retorno nativo a Bis

Se compiló el checkout `52fc506a836821b7285834bdc0adf93e9ddc5680`, usando un directorio target nuevo:

```sh
CARGO_TARGET_DIR=/workspace/scratch/cdd3907241dd/rust-env-s25/target-native cargo build --manifest-path rust/Cargo.toml --locked --offline -p sv_native --bin sv-native
```

La primera comprobación con el target del checkout reutilizó artefactos; por ello se realizó la compilación con target nuevo. Ambos registros se conservan. La construcción nueva concluyó con 25 advertencias del núcleo; no se corrigen ni ocultan en este trabajo instrumental. [Registro completo](build-clean.stderr).

Reejecución del banco comprometido C01: **13/13 variantes conformes y 4/4 sensibilidades detectadas**. Se utilizó sin modificaciones el observador Node del repositorio, que invoca al binario Rust y compara esperados ya comprometidos; Node no implementa la semántica. [Resultado](bis-c01-ejecucion/RESULTADO.json) y [montaje/órdenes/retornos](VERIFICACION.json). El campo corte_lenguaje del resultado del observador procede del banco histórico; el corte realmente compilado es el indicado arriba y en VERIFICACION.json. No se sustituye la evidencia C01 anterior ni se suman estas repeticiones como nuevas variantes.

C02–C12: **202 filas preparadas, ninguna ejecutada**. BIS-02 sigue abierto; BIS-03 y GUI conservan su estado pendiente. El siguiente paso sustantivo es comprometer el contrato de integración y banco común C02–C05 con guardas transversales. Esta recuperación permite continuar su trabajo nativo; no constituye tamaños de soporte, autoridad ni paridad visual y no cierra el catálogo.

No se han instalado ni probado nuevos destinos WASM/WASI en esta recuperación. Tampoco se acredita conectividad general de dependencias: esta compilación usó las dependencias disponibles offline.

## Evidencia y trazabilidad

[Fuentes y corte de lectura](FUENTES.json), [script de verificación](verificar.py), [script de preparación](preparar.py), [publicación](publicar.py) y [comprobación del espejo](verificar_publicacion.py). Los scripts administrativos se conservan con sus rutas de ejecución originales; no deben ejecutarse indiscriminadamente desde otra ubicación. Sus usos de Python son configuración, registro y comparación; las sondas y el núcleo son Rust.

La huella del paquete no equivale a una firma criptográfica comprobada. El manifiesto de este expediente identifica sus archivos; el repositorio conserva contenido literal de sondas, órdenes y resultados. El paquete binario de 365 MB no se incorpora al repositorio.
