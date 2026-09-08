# Construcción de la candidata Rust de fila 7

Este paquete conserva fuentes y entradas del commit identificado en `CORTE_GIT.txt`. No contiene el editor web ni los conductores Python/JavaScript de la auditoría completa. La licencia y los manifiestos Cargo se conservan literalmente.

Desde el directorio `fuentes`, con Rust/Cargo 1.98.0, GCC/libc para el destino GNU/Linux y GNU Coreutils:

```sh
sha256sum --check INVENTARIO.sha256
cargo +1.98.0 build --manifest-path rust/Cargo.toml --workspace --release --offline
cargo +1.98.0 test --manifest-path rust/Cargo.toml --workspace --offline
rust/target/release/sv-native --profile es tests/row7_bindings/base.es.svp
```

Las sondas de ligaduras y transporte documental se ejecutan con:

```sh
cargo +1.98.0 run --manifest-path rust/Cargo.toml -p sv_core --example binding_probe --release --offline
cargo +1.98.0 run --manifest-path rust/Cargo.toml -p sv_core --example gh_binding_probe --release --offline
rustc +1.98.0 --edition=2021 tests/row7_gh/generar_entradas.rs -o rust/target/gh-generate
rust/target/gh-generate --check
```

LIG/0.1 es una API Rust tipada del núcleo; no es una sintaxis de entrada añadida a la CLI. Las sondas son pruebas. La auditoría documental externa y su recálculo de huellas siguen requiriendo Node y están en el repositorio completo. Los resultados de las sondas no se certifican por sí solos.

La prueba de CI registra la imagen Ubuntu 24.04, Rust 1.98.0 y paquetes del entorno, comprueba ausencia de intérpretes Python/Node y ejecuta la construcción con red deshabilitada y Cargo offline. Docker aísla ese ensayo; no es necesario para ejecutar `sv-native` ni constituye una elección de plataforma final. La prueba no acredita Q0 ni las doce SP integradas. La auditoría externa fuerte examina toda la candidata antes de entregar a CYB.
