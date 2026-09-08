# Dependencias de verificación y entradas documentales G/H

**RETP-2026-101 · 08/09/2026. Entrada exacta:** `ccc007578e23df3a304208733687824e2f241438`, PR #81 candidata. Sucede a RETP-100 para la preparación del ensayo; no modifica su contrato LIG/0.1 ni sus resultados históricos. Pilares RETP-073, perfiles RETP-075, secuencia RETP-076/082 y sus sucesiones RETP-096…100 conservan su autoridad.

## 1. Mandato y dependencia actual

El Director exige una entrega final de la DSL construible y ejecutable sin Python ni generación imprescindible mediante Python. La revisión integral del español conserva su momento previo al cierre y su acta permanece intacta. Una herramienta auxiliar no obtiene autorización de permanencia en la entrega final por estar situada en `tests/`.

El corte de entrada contiene 19 ficheros Python, todos bajo `tests/`. La construcción ordinaria de los tres componentes Cargo no los invoca ni declara Python/Node como dependencia; la verificación completa de CI sí requiere ambos. No se ha demostrado aún una distribución final autónoma en un entorno limpio. No se confunde la retirada del compilador Python con la retirada de todos los comprobadores Python.

## 2. Incremento autorizado y preservación exigida

Se retiran del camino activo `tests/row7_gh/generar_entradas.py` y `tests/row7_bindings/hash_reference.py`; Git conserva sus bytes históricos. La generación G/H se sustituye por una herramienta Rust autónoma compilada directamente con Rust 1.98.0, sin importar `sv_core` ni dependencias Cargo. Sus huellas documentales se calculan con la herramienta externa `sha256sum` de GNU Coreutils, ya presente en el entorno de referencia. Esa dependencia de preparación se declara expresamente; no se enlaza con el compilador SV ni con su módulo WebAssembly.

La comprobación del vector fijo LIG reutiliza el codificador JavaScript externo de RETP-100 y la misma constante previamente fijada en RETP-098. No se obtiene un nuevo esperado desde Rust ni se declara una segunda realización semántica de SV. Node permanece como dependencia de la auditoría externa; no se presenta este incremento como retirada completa de Python o Node.

Las 48 entradas mantienen todos sus campos y bytes de datos, incluidas definición, procedencia, información lateral y huellas. Sólo cambia el comentario de procedencia de `entradas.rs`. El testigo G/H, su inventario, los resultados esperados, el vector fijo LIG y los casos del corpus no se regeneran ni se ajustan. La comparación de la generación con el fichero comprometido acredita regenerabilidad, no corrección de la regla: ésta sigue contrastada por las pruebas Rust de transporte y el observador externo independiente.

## 3. Subconjunto JSON del ensayo documental

La preparación y el observador aplican, mediante implementaciones separadas, este subconjunto antes de convertir números a una representación de máquina: JSON UTF-8 estricto, cadenas de valores escalares Unicode, booleanos, null, listas ordenadas y objetos con miembros únicos en orden de declaración. Los números deben ser tokens decimales canónicos no negativos entre 0 y 9007199254740991. Se rechazan fracciones, exponentes, signos y enteros superiores; no se redondean ni se convierten en cadenas.

Se rechazan también claves de objeto que JavaScript trataría como índices de array: enteros canónicos entre 0 y 4294967294. Así no se acepta una reordenación silenciosa. Se rechazan miembros repetidos, incluso con nombres equivalentes tras descodificar escapes, y sustitutos Unicode aislados. Un lector inválido no puede repararse con caracteres de sustitución.

Este subconjunto sólo delimita los testigos documentales del ensayo G/H. No restringe `Nat` de SV, no define los tipos numéricos de Inmunología/CYB y no autoriza a recibir datos de dominios futuros. Los testigos actuales siguen fijados por su SHA-256. Una ampliación necesita contrato y testigos propios; sus números no se adaptarán al lector por conveniencia.

## 4. Controles previos al resultado

El generador y el observador deben admitir los bytes G/H fijados y los controles de Unicode, objetos/listas, cero y extremo numérico permitido. Deben rechazar con causa identificada: `1.0`, `1e0`, `-0`, `9007199254740992`, `9007199254740990.1`, miembro repetido, clave índice, sustituto aislado y UTF-8 inválido. Las comprobaciones de número deben preceder a cualquier conversión numérica; `Number.isSafeInteger` posterior no satisface esta obligación.

La generación exacta debe conservar las 48 entradas del corte precedente salvo su comentario. El valor del vector fijo LIG seguirá siendo `beff99707d648d591714e431b462bf61546da931771a6168cd80b1c8037f7dc0`. Se conservan los 28 ataques del observador de RETP-100, sus causas, el control reserializado y el contraste literal nativo/WASI/navegador. No se eliminan puertas por retirar sus conductores anteriores.

## 5. Reproducción y continuidad

Las siguientes órdenes se ejecutan desde la raíz del repositorio y sustituyen la invocación Python histórica del §6 de GH-LIG. Requieren Rust 1.98.0, GNU Coreutils y Node para la auditoría externa:

```sh
mkdir -p rust/target
rustc +1.98.0 --edition=2021 tests/row7_gh/generar_entradas.rs -o rust/target/gh-generate
rust/target/gh-generate --check
rustc +1.98.0 --edition=2021 --test tests/row7_gh/generar_entradas.rs -o rust/target/gh-input-tests
rust/target/gh-input-tests
node --test tests/row7_gh/documentary_json.test.mjs
node tests/row7_gh/contract_hash.mjs --referencia
```

La herramienta sólo escribe `entradas.rs` si se invoca sin `--check`; CI emplea siempre la comprobación sin escritura. El lector Rust reside en `documentary_json.rs`; el lector externo en `documentary_json.mjs`. Los controles de cada lector tienen expectativas fijadas por §3–4. Las comprobaciones se ejecutan sobre una candidata propia apilada sobre PR #81; no se modifica aquella cabeza durante su contraste externo.

El inventario residual previsto es 17 ficheros Python activos. Su retirada completa, la construcción sin intérpretes en entorno limpio, las pruebas de geometría ampliada y la concordancia de errores de frontera permanecen pendientes; no quedan satisfechas por este incremento. No se alteran núcleo, gramática, IR, distribución web ni repositorios de dominio. La fila 7 sigue abierta y no se promueve ninguna candidata.

## 6. Resultado del incremento

El contrato previo quedó confirmado en `9f1f39328e363c92d17d2aaf3fb0e51efff23930`, antes de la realización `a20afe9d76da173295f5c123614189b31d705974`, árbol `0cff4a5b0b907f1105e0519793fb9a56e2e6497d`. La PR #82 permanece en borrador sobre #81. CI comprobó la fusión virtual `3a09cbdc7ffcadc69edd46af5297e381223c641d`, cuyo árbol es idéntico al material citado; no es una promoción a main.

Los cuatro flujos son conformes: R0 Rust `34209375380`, Conformidad SVP `34209375465`, R0-8 `34209375394` y paridad nativa/WASI/navegador `34209375404`. Se verifican 7/7 pruebas Rust de preparación y lectura, 6/6 pruebas JavaScript, regeneración exacta de las 48 entradas y el mismo vector LIG fijado previamente. La comprobación del espacio de trabajo conserva 211 pruebas unitarias de `sv_core` y sus suites; el corpus conserva 14 válidos y 106 inválidos. Los 43 mutantes dirigidos previos son detectados sin supervivientes ni mutaciones inválidas.

El transporte conserva 16 recuperaciones F0, 16 con información lateral declarada, 16 controles H y ocho pérdidas H. El observador recalcula las 48 huellas contractuales, rechaza sus 28 ataques por la causa prevista y admite el control reserializado. Los informes de transporte siguen siendo literalmente iguales en nativo, WASI y navegador. Los identificadores y huellas de los artefactos publicados por GitHub constan en el [asiento maestro RETP-101](./REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-101).

Se han retirado los dos archivos Python previstos. Quedan 17 archivos `.py` de pruebas, además de invocaciones y fragmentos Python en los flujos; el recuento de archivos no mide toda la dependencia ejecutiva. Node y `sha256sum` permanecen declarados. La corrección local queda verificada en candidata, con revisión externa pendiente; no acredita distribución final autónoma ni cierra fila 7, SP o Q0.
