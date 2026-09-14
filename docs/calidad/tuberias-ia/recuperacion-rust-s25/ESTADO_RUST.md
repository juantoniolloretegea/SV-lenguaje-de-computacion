# Estado observado de Rust y del entorno nativo

Fecha UTC: 2026-09-14T10:43:49Z. Arquitectura: x86_64. Unidad: Watson / W-S26.

**Comprobación instrumental: CONFORME PARA LAS HERRAMIENTAS NATIVAS DECLARADAS.** Este registro es una observación fechada, no un monitor en directo ni garantía de persistencia después de reemplazar el entorno.

| Componente | Ruta | Observación |
| --- | --- | --- |
| rustc | /opt/sv-rust-1.98.0/bin/rustc | rustc 1.98.0 (88d9e12ae 2026-08-18) |
| cargo | /opt/sv-rust-1.98.0/bin/cargo | cargo 1.98.0 (797e8a9bc 2026-08-05) |
| rustfmt | /opt/sv-rust-1.98.0/bin/rustfmt | rustfmt 1.9.0-stable (88d9e12ae1 2026-08-18) |
| clippy-driver | /opt/sv-rust-1.98.0/bin/clippy-driver | clippy 0.1.98 (88d9e12ae1 2026-08-18) |
| Gestor rustup | /opt/sv-cargo/bin/rustup | rustup 1.29.1 (d95a37b6a 2026-08-13)info: This is the version for the rustup toolchain manager, not the rustc compiler. info: the currently active `rustc` version is `rustc 1.98.0 (88d9e12ae 2026-08-18)`  |
| Toolchain enlazado sv-1.98.0 | /opt/sv-rust-1.98.0 | Destino real cotejado con la instalación directa |
| Biblioteca estándar nativa | /opt/sv-rust-1.98.0/lib/rustlib/x86_64-unknown-linux-gnu/lib | Directorio observado; componentes abajo |
| cc | /usr/bin/cc | Localizado en PATH |
| gcc | /usr/bin/gcc | Localizado en PATH |
| ld | /usr/bin/ld | Localizado en PATH |
| ar | /usr/bin/ar | Localizado en PATH |
| pkg-config | — | No localizado en PATH |
| rustup | /opt/sv-cargo/bin/rustup | Localizado en PATH |
| dotnet | — | No localizado en PATH |

Distribución observada:

```text
PRETTY_NAME="Ubuntu 24.04.3 LTS"
NAME="Ubuntu"
VERSION_ID="24.04"
VERSION="24.04.3 LTS (Noble Numbat)"
VERSION_CODENAME=noble
ID=ubuntu
ID_LIKE=debian
HOME_URL="https://www.ubuntu.com/"
SUPPORT_URL="https://help.ubuntu.com/"
BUG_REPORT_URL="https://bugs.launchpad.net/ubuntu/"
PRIVACY_POLICY_URL="https://www.ubuntu.com/legal/terms-and-policies/privacy-policy"
UBUNTU_CODENAME=noble
LOGO=ubuntu-logo
```

Componentes declarados por la instalación:

```text
rustc
cargo
rust-std-x86_64-unknown-linux-gnu
rustfmt-preview
clippy-preview
```

El banco S26-F01/F02 LIG compiló y ejecutó diez casos en debug y diez en release con entradas EN/ES: [recepción](../../riesgos-materiales-s26/r06/RESULTADOS_F01_F02_LIGADURAS.md). Este inventario no repite esas pruebas ni acredita dependencias de etapas futuras.

[Obligación operativa, acceso y rustup](OBLIGACION_RUST_Y_PROVISION_DEL_ENTORNO_S29.md). [Fuente Rust de esta consulta](estado_rust.rs). El programa sólo lee archivos del sistema y consulta versiones/localización; declara los programas auxiliares invocados (date y uname). No usa Python ni red.
