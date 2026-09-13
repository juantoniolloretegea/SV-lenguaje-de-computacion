# Manifiesto SV incorporado y ejecución Rust verificable

Versión 1 · 13 de septiembre de 2026 · S23 / RETP-2026-201 · Juan Antonio Lloret Egea y Watson

El acta de prohibición absoluta de uso bélico y primacía de supervivencia humana queda incorporada como recurso de `sv_core`. La función `manifiesto_sv()` proporciona su texto completo y los comandos `manifiesto-sv` y `sv-native manifiesto-sv` lo emiten íntegramente. No se descarga en ejecución ni se requiere conservar el repositorio junto al ejecutable. El documento español canónico mantiene su autoridad; la capa Rosetta incluida conserva su estatuto declarativo.

[Contrato previo y alcance](CONTRATO.md) · [Acta canónica](../ACTA_TECNICA_DE_PROHIBICION_ABSOLUTA_DE_USO_BELICO_Y_PRIMACIA_DE_SUPERVIVENCIA_HUMANA_EN_EL_SV_2026_03_26.md) · [Recurso dentro del núcleo](../../../rust/sv_core/assets/manifiesto-sv.md) · [Entorno, fuentes y artefactos](evidencia/ENTORNO_Y_FUENTES.json).

## Resultado material de laboratorio

| Comprobación | Resultado observado |
|---|---|
| Rust nativo disponible | rustc 1.98.0, cargo 1.98.0, x86_64-unknown-linux-gnu; instalación en entorno remoto de trabajo |
| Workspace nativo | 369 pruebas superadas, sin fallos en la ejecución limpia identificada |
| Conformidad de fuentes SV | 14 válidos y 106 inválidos conforme a los esperados comprometidos |
| Paridad nativa–WASI del corpus | 120 casos; salida y diagnóstico conservados según el observador vigente |
| Consulta documental | 19 134 bytes idénticos en binario independiente, subcomando, WASI y adaptador WASM |
| Independencia del repositorio en ejecución | Binarios copiados a directorio ajeno; fichero señuelo y variable de entorno no alteran el contenido; WASI sin directorios preabiertos |
| Casos negativos | Argumentos incompatibles rechazados con código 2; fallo de stdout separado; observador detecta byte alterado y truncamiento |

La [prueba específica](evidencia/verificacion-manifiesto.json), el [registro del workspace](evidencia/cargo-test-clean.log), la [conformidad](evidencia/conformance.log) y la [paridad completa](evidencia/paridad-sv.json) conservan los resultados. El adaptador WASM se ejecutó en Node; la ejecución en navegador real corresponde a los controles de la candidata. El estado de integración se registra por separado y no se deduce de estas pruebas locales.

## Incidencias conservadas

La primera ejecución encontró símbolos no resueltos en el enlace de pruebas; se conserva `cargo-test.log`, sin atribuir una causa raíz no demostrada. Una segunda configuración, con compilación incremental desactivada, dejó dos bibliotecas `rlib` en el directorio de salida, por lo que el observador de autoridad rechazó su identidad ambigua. Se limpiaron exclusivamente los productos de construcción locales y la ejecución completa pasó con `CARGO_INCREMENTAL=0`; no se modificaron las guardas para ocultar el fallo. `cargo-test-noincremental.log` y `cargo-test-final.log` conservan esa incidencia. El primer recorrido de paridad completó sus casos pero falló al registrar `rustc` por ausencia de su directorio en PATH; se conserva `paridad-sv.log`. La repetición identificada añadió el directorio al PATH y produjo el informe completo. La instalación no había modificado el PATH de las consolas; las invocaciones posteriores deben cargar el entorno de Cargo.

## Reproducción

Rustup se obtiene mediante el [procedimiento oficial de Rust](https://rust-lang.org/tools/install/). Una vez instalado:

```sh
. "${CARGO_HOME:-$HOME/.cargo}/env"
sh tools/preparar_rust_sv.sh
rustc +1.98.0 --version --verbose
cargo +1.98.0 --version
```

Desde una copia identificada del repositorio, con directorio de construcción limpio:

```sh
export CARGO_INCREMENTAL=0
cargo +1.98.0 test --manifest-path rust/Cargo.toml --workspace --offline
cargo +1.98.0 build --manifest-path rust/Cargo.toml --release -p sv_native --offline
cargo +1.98.0 build --manifest-path rust/Cargo.toml --release -p sv_native --target wasm32-wasip1 --offline
cargo +1.98.0 build --manifest-path rust/Cargo.toml --release -p sv_wasm --target wasm32-unknown-unknown --offline
node tests/manifiesto_sv/verificar.mjs
python tests/run_conformance.py --rust-bin rust/target/release/sv-native
./rust/target/release/manifiesto-sv
```

El workspace no añade dependencias externas por esta incorporación. `include_str!` incorpora el recurso durante la compilación conforme a la [documentación oficial de Rust](https://doc.rust-lang.org/std/macro.include_str.html). El comando instalado se obtiene con `cargo +1.98.0 install --path rust/sv_native --offline`, manteniendo el directorio de ejecutables en PATH. En WASI puede utilizarse `node tests/run_wasi_sv_native.js rust/target/wasm32-wasip1/release/sv-native.wasm manifiesto-sv`. El comando es de distribución: no cambia SVP-ES/SVP-EN ni introduce palabras reservadas. No se traduce silenciosamente el acta canónica.

## Alcance y continuación

Se resuelve la disponibilidad de herramientas para trabajar en este entorno sin depender del ordenador del autor. No se garantiza que un contenedor futuro conserve su instalación; cada relevo comprueba las versiones y utiliza el procedimiento si debe reponerlas. La continuidad ya no puede presentar la ausencia histórica de Cargo como estado actual sin verificarla.

Mostrar el acta no acredita detección o bloqueo de usos prohibidos, confianza frente a un host comprometido, permanencia del entorno ni cierre de la semántica. No se interpreta ni ejecuta el fragmento Rosetta. La aplicación pública de navegador no se presume recompilada por añadir una exportación al adaptador. La gramática 0.2, IR 0.3 y serializador 0.1.0 permanecen sin modificación.

S22 / (p1+p3)-Bis continúa desde BIS-02: preparar entradas y oráculos de BIS-C01. Los 24 escenarios previos siguen sin ejecutar; esta prueba documental y la regresión del Lenguaje tienen objeto e identidad propios.

## Corrección de empaquetado detectada en CI

La primera candidata, `074436ce0a5fa9dd3fd3c3b1b8e45cf4419bbce5`, falló en la [ejecución aislada de fila 7](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34744228854): la lista de fuentes admitidas omitía el nuevo recurso Markdown requerido por `include_str!`. Se añade exclusivamente ese fichero a las listas de empaquetado y verificación; se mantienen las restricciones sobre otros archivos. El ensayo aislado comprueba ahora las dos consultas del manifiesto y la autoprueba añade tres rechazos específicos. El registro original se conserva en `evidencia/ci-empaquetado-inicial.log`. El resultado de la repetición se documentará con el commit efectivamente comprobado.


La primera ejecución WASM de la candidata corregida terminó con `PENDIENTE` al capturar el DOM de la sonda LIG/0.1; no informó divergencia algebraica ni fallo del manifiesto. Se conserva `evidencia/ci-wasm-intento-1.log`. Se repitió el trabajo sobre el mismo commit, sin cambiar las pruebas ni sus criterios. El resultado posterior y su número de intento constan en el recibo; la causa raíz de esa captura pendiente no queda acreditada por una repetición satisfactoria.

## Integración y cierre instrumental

2026-09-13T07:16:29Z: S23 finalizado mediante [PR #90](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/90). Candidata comprobada: `1469b08272f5d5c10e406dfa77f76a4d11c3accc`; integración: `efdb68d6c62355a75aa1d2b3b196bb78246f2014`. Los seis flujos terminaron satisfactoriamente. El [recibo de integración](evidencia/INTEGRACION.json) conserva identificadores, intentos y pasos.

| Flujo | Resultado | Evidencia |
|---|---|---|
| Fila 7 fuentes y ejecución sin intérpretes | Conforme | [Ejecución 34744520767](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34744520767) |
| Conformidad SVP | Conforme | [Ejecución 34744520774](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34744520774) |
| R0 Rust | Conforme | [Ejecución 34744520781](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34744520781) |
| Retorno CYB documental | Conforme | [Ejecución 34744520785](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34744520785) |
| R0 WASM paridad nativa y navegador | Conforme | [Ejecución 34744520798](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34744520798) |
| R0-8 Baseline nativa | Conforme | [Ejecución 34744520803](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/34744520803) |

El comando quedó además instalado en el entorno remoto mediante Cargo offline y se comprobó por nombre desde un directorio ajeno al repositorio: `evidencia/COMANDO_INSTALADO.json` y `evidencia/cargo-install.log`. La repetición aislada construyó y probó Rust sin red ni intérpretes Python/Node; contrastó las dos salidas del manifiesto y superó los once controles negativos del paquete. El flujo de paridad incluye el corpus en navegador; esto no declara desplegada una interfaz de consulta del manifiesto en la aplicación pública. S22 continúa en BIS-02 con sus 24 escenarios aún sin ejecutar. El registro de cierre y las copias preservan el alcance instrumental de S23.

Copia de laboratorio verificada por árbol Git en `d2e10fe6d75ba334956de66b7532b0ce3f890f6a` (repositorio privado del proyecto); el expediente público contiene la evidencia necesaria para su lectura independiente.
