# S29 · Verificación de la recepción operativa

La incorporación se realiza después de publicar la recepción LIG: Lenguaje `2ebd2215a803bcfb31b1425cba940bd2df19c56a`, laboratorio `32ab2bc5b7f279a03b078eea165c180fa7b20d60`. S26 revisión 25 y S28 revisión 2 se conservan. S29 revisión 0 registra la obligación y el acceso; S24 revisión 1 recibe C#/.NET sin iniciar GUI.

Comandos ejecutados desde la raíz del Lenguaje:

```text
/opt/sv-rust-1.98.0/bin/rustc --edition=2021 docs/calidad/tuberias-ia/recuperacion-rust-s25/estado_rust.rs -o /workspace/scratch/570d4b947ea6/estado-rust
/workspace/scratch/570d4b947ea6/estado-rust docs/calidad/tuberias-ia/recuperacion-rust-s25/ESTADO_RUST.md
/opt/sv-rust-1.98.0/bin/rustc --edition=2021 docs/calidad/tuberias-ia/recuperacion-rust-s25/registrar_s29.rs -o /workspace/scratch/570d4b947ea6/registrar-s29
```

Los tres comandos concluyeron con retorno 0. La consulta produjo el estado de 2026-09-14T10:23:40Z. La compilación del registrador produjo una advertencia unused_mut en la clausura administrativa set, sin error. El registrador se ejecutó sobre el checkout, con la fecha UTC incorporada en las filas de Sucesos y una carpeta nueva de copia previa fuera del repositorio; retorno 0 y salida:

```text
S29 revision 0 finalizado; S24 revision 1 pendiente; 30 sucesos concordantes; historial conservado.
```

El programa verifica las correspondencias CSV/Markdown/última revisión, rechaza un alta S29 existente, conserva el prefijo histórico y relee lo escrito. No accede a RETP. Su fuente, el estado observado y los cambios quedan en las sedes de Calidad y laboratorio. No se reejecutó el banco LIG ni se instaló rustup o .NET.

Identidades SHA-256:

```text
b5a6e4c703c5b21c98e858c639b6b03211ce7afc6e722d646f857b03e76753bd  docs/calidad/tuberias-ia/recuperacion-rust-s25/estado_rust.rs
a61b312c74528fc2131f3636306e45d61d24b6a80bbff3410f34dbefc6ee3d1b  docs/calidad/tuberias-ia/recuperacion-rust-s25/registrar_s29.rs
fa1d5c853cbee79b3a5482f32bbd6fb1ad32cea76d0fb23560bd4c9b82648f9a  docs/calidad/tuberias-ia/recuperacion-rust-s25/ESTADO_RUST.md
```

Se utiliza el conector GitHub para publicar sobre las ramas existentes, con cotejo completo de árboles. La coordinación de herramientas del entorno usa JavaScript; esta circunstancia se declara en la obligación y no se presenta como una migración de esa infraestructura a Rust. No se ejecuta Python en S29. El inventario enumera también las herramientas de sistema que consulta.
