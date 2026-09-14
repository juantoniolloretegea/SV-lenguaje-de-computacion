# S29 · Recepción de rustup y conservación de Rust 1.98.0

14 de septiembre de 2026. Unidad: Watson / W-S26. Corte de entrada: Lenguaje `baa679a3956dd619b3365678b470e196d914fc57`; laboratorio `053aaaf6e4759097de6c598c3d14471fef54084f`. Incorporación instrumental autorizada por la entrega humana del instalador. No modifica semántica, pruebas SV ni RETP canónica.

## Procedencia y verificación previa

Adjunto recibido: `095b2e53-eec9-4ff3-abef-f71cad2ecaae`. Formato observado: ELF de 64 bits, x86-64, GNU/Linux, intérprete `/lib64/ld-linux-x86-64.so.2`.

SHA-256 calculada y coincidente con el [archivo oficial](https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init.sha256):

```text
dda7234360b7f578ca8b0ddcb80145646fa61a67c1720a5abc7051b35c9fcb71
```

Respuesta oficial recibida mediante curl, salida 0: `dda7234360b7f578ca8b0ddcb80145646fa61a67c1720a5abc7051b35c9fcb71 *./rustup-init`. La descarga tardó aproximadamente 13 segundos. El lector web alternativo no pudo abrir este recurso; no se utilizó como evidencia de concordancia. Esta comparación no constituye verificación de una firma independiente.

El intento `--help` con el nombre asignado al adjunto devolvió 1: `unknown proxy name`. Tras copiarlo como `/tmp/rustup-init`, `--help` devolvió 0 e identificó `rustup-init 1.29.1 (d95a37b6a 2026-08-13)`. No se alteraron los bytes para corregir el nombre.

## Instalación y acceso

Órdenes ejecutadas, cada una con retorno 0:

```sh
RUSTUP_HOME=/opt/sv-rustup CARGO_HOME=/opt/sv-cargo /tmp/rustup-init -y --default-toolchain none --no-modify-path --profile minimal
RUSTUP_HOME=/opt/sv-rustup CARGO_HOME=/opt/sv-cargo /opt/sv-cargo/bin/rustup toolchain link sv-1.98.0 /opt/sv-rust-1.98.0
RUSTUP_HOME=/opt/sv-rustup CARGO_HOME=/opt/sv-cargo /opt/sv-cargo/bin/rustup default sv-1.98.0
```

El instalador advirtió de la instalación directa existente, imprimió `cannot install while Rust is installed` y continuó por `-y`, indicando que la comprobación era omitible y que se omitía la instalación de toolchains. Se conserva esa advertencia; el éxito se comprueba después por las consultas efectivas.

El gestor está en `/opt/sv-cargo/bin/rustup`; sus datos, en `/opt/sv-rustup`. El toolchain personalizado `sv-1.98.0` enlaza la instalación directa conservada. No se descargó ni sustituyó Rust y no se afirma que el toolchain enlazado permita instalar componentes mediante rustup. Cualquier ampliación deberá comprobar esa capacidad y la disponibilidad de sus paquetes antes de utilizarla.

Activación explícita de cada consola o proceso que lo necesite:

```sh
. /opt/sv-rustup/env.sh
rustup show
rustc --version
cargo --version
```

[Copia versionada del activador](env-rustup.sh). El activador declara `RUSTUP_HOME`, `CARGO_HOME` y antepone `/opt/sv-cargo/bin` sin duplicarlo. No se modificaron automáticamente los archivos de inicio de la consola. La instalación directa mantiene su acceso anterior. La persistencia tras reemplazo del contenedor no está acreditada.

## Resultado instrumental

A las 10:42 UTC se observaron: rustup 1.29.1; toolchain activo y predeterminado `sv-1.98.0`; destino `x86_64-unknown-linux-gnu`; rustc 1.98.0, commit `88d9e12ae178fab0fb5cc050a94da85685d449ea`, LLVM 22.1.8; Cargo 1.98.0; rustfmt 1.9.0-stable y Clippy 0.1.98. `rustup which rustc` y `rustup which cargo` resuelven por `/opt/sv-rustup/toolchains/sv-1.98.0/bin/`. La consulta Rust del [estado](ESTADO_RUST.md) se renueva con el activador cargado.

No se ejecutó Python. Se utilizaron Bash para invocar herramientas y configurar variables, file para identificar ELF, sha256sum y curl para cotejar procedencia, cp/chmod para preparar el nombre y permiso del instalador, y herramientas Git para publicación. La coordinación del conector utiliza JavaScript. Estas herramientas se declaran y no se presentan como implementaciones Rust. El inventario y la actualización concordante de Sucesos se realizan con auxiliares Rust; los bancos SV permanecen en Rust.

S29 revisión 1 recibe la instalación. S26 revisión 25 conserva la recepción LIG y sus pendientes; no se repiten casos por esta operación instrumental. C#/.NET, GUI y cualquier FFI conservan su evaluación y secuencia propias en S24.
