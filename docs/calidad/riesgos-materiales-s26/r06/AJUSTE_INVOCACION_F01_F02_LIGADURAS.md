# S26-F01/F02: ajuste de invocación previo a la ejecución

**2026-09-14T09:55:10Z. Unidad: Watson / W-S26. Diez casos todavía sin ejecutar.**

Precompromiso recibido en Lenguaje `ccbb152f69fa1a954e153fbaac24e4ac337e9ef3` y laboratorio `985423ecc0f5c0127764286bf26ee503ec050c17`. El banco, su manifiesto y los oráculos se conservan literalmente. Este ajuste sólo sustituye la selección `+1.98.0`, que depende de rustup, por los ejecutables 1.98.0 instalados directamente en este entorno.

Versiones observadas: rustc 1.98.0 (88d9e12ae 2026-08-18), cargo 1.98.0 (797e8a9bc 2026-08-05), host x86_64-unknown-linux-gnu, LLVM 22.1.8. rustup no está disponible. La primera compilación de un auxiliar administrativo con `rustc +1.98.0` fue rechazada por argumentos; no compiló ni ejecutó el banco SV. Se conserva esa incidencia.

## Comandos fijados

Ejecutar desde la raíz del Lenguaje, después de publicar este ajuste en las ramas existentes. Compilación offline y bloqueada por Cargo.lock; diez casos por perfil, cada uno con entradas EN y ES. El registrador administrativo Rust invoca estos argumentos y conserva stdout, stderr, retorno y fechas; no cambia los oráculos.

```text
/opt/sv-rust-1.98.0/bin/cargo build --manifest-path rust/Cargo.toml --package sv_core --lib --locked --offline --target-dir rust/target/s26-f01-f02
/opt/sv-rust-1.98.0/bin/rustc --edition=2021 --test docs/calidad/riesgos-materiales-s26/r06/s26_f01_f02_ligaduras.rs --extern sv_core=rust/target/s26-f01-f02/debug/libsv_core.rlib -L dependency=rust/target/s26-f01-f02/debug/deps -C opt-level=0 -C debuginfo=2 -o rust/target/s26-f01-f02/s26-f01-f02-debug
rust/target/s26-f01-f02/s26-f01-f02-debug --test-threads=1 --nocapture
/opt/sv-rust-1.98.0/bin/cargo build --manifest-path rust/Cargo.toml --package sv_core --lib --release --locked --offline --target-dir rust/target/s26-f01-f02
/opt/sv-rust-1.98.0/bin/rustc --edition=2021 --test docs/calidad/riesgos-materiales-s26/r06/s26_f01_f02_ligaduras.rs --extern sv_core=rust/target/s26-f01-f02/release/libsv_core.rlib -L dependency=rust/target/s26-f01-f02/release/deps -C opt-level=3 -C debug-assertions=no -C overflow-checks=no -o rust/target/s26-f01-f02/s26-f01-f02-release
rust/target/s26-f01-f02/s26-f01-f02-release --test-threads=1 --nocapture
```

## Integridad y límites

El auxiliar R08 ha cotejado las 2999 entradas de archivo del precompromiso contra sus bytes locales y los 2996 archivos del corte fuente `fe96bea19c14ddbb80c01033b52a7e4761754732`, extraído de Git. Los descriptores proceden de árboles completos leídos por API. Los primeros descriptores administrativos contenían una línea final vacía: R08 los rechazó con PUB_CAMPO; se preservan y se generaron descriptores sin esa línea antes del cotejo conforme. No se modificó R08.

Antes de ejecutar se cotejará también el nuevo corte publicado. Después se releerán los archivos. La evidencia conservará adaptadores administrativos Rust, descriptores, incidencias, órdenes y salidas. Los hashes y el cotejo no acreditan un host independiente ni inmutabilidad física. El banco mantiene su alcance LIG/0.1: no cierra F01/F02 globales ni implementa consulta histórica por Frame.

Sucesos S26 revisión 24 recibe este ajuste y la recuperación instrumental. No se reejecutan bancos anteriores. RETP canónica, núcleo, contratos y las ramas existentes conservan su estado; la continuación material permanece abierta.
