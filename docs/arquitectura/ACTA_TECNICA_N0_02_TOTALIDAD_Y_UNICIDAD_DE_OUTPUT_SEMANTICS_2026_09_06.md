# Acta técnica N0-02: totalidad y unicidad de la relación CellSpec–OutputSemantics–Codomain

**Fecha:** 6 de septiembre de 2026

**Registro:** RETP-2026-079

**Corte de entrada:** `ed61af2fb80641866356a7138cc87763eab005d9`, integración de [PR #65](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/65) tras la reparación de oráculos

**Expediente:** [historial de la candidata N0-02](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commits/n0-02-output-semantics-20260906), con su PR y controles asociados

**Frente:** fila 3, K1; cierre incremental N0-02

**Estado:** cierre relacional delimitado; promoción condicionada a los controles del candidato exacto

## 1. Fuentes y problema comprobado

El corte de entrada incorpora N0-01 y los oráculos de RETP-078. Se reciben los [Pilares](../calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md), el [acta de perfiles y ensamblaje](../calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md) y el [acta de transición completa, §§12–19](../dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md), cuya identidad respecto del corte ya leído se ha comprobado. Se contrastan J1.1 de [IR v0.2](../../IR_CANONICA_BIENFORMACION_SV_v0_2.md), su sucesión [IR v0.3](../../IR_CANONICA_BIENFORMACION_SV_v0_3.md), la [radiografía N0-02/N0-03](./N0_RADIOGRAFIA_DE_OBJETOS_INVARIANTES_Y_ORACULOS_DEL_NUCLEO_SV_2026_09_04.md) y el [acta de oráculos](../calidad/ACTA_TECNICA_REPARACION_DE_ORACULOS_2026_09_06.md).

J1.1 exige que la semántica documente la interpretación de cada elemento del codominio. Ambos validadores comprobaban la existencia y el tipo de las referencias, pero admitían una semántica vacía, incompleta, con claves ajenas o repetidas. Los cuatro nuevos negativos reproducen esa admisión en ambos emisores del corte de entrada. En el caso repetido, Python perdía una entrada al construir un mapa; Rust emitía miembros homónimos. Que ambos procesos terminaran correctamente no constituía conformidad.

## 2. Decisión normativa y sede

Se constituye **J-K1**, en [IR v0.3 §6.2](../../IR_CANONICA_BIENFORMACION_SV_v0_3.md#relacion-n0-02): para cada `CellSpec`, las claves de su `OutputSemantics` deben corresponder exactamente a los miembros de su `Codomain`, con una sola entrada por símbolo.

| Supuesto | Decisión |
|---|---|
| Semántica vacía sobre codominio no vacío | Rechazo |
| Miembro del codominio sin interpretación | Rechazo |
| Clave semántica ajena al codominio | Rechazo |
| Clave repetida, con texto igual o distinto | Rechazo |
| Símbolos distintos con el mismo texto descriptivo | Admisible por J-K1 |
| Claves y miembros declarados en órdenes distintos | Admisible por J-K1; no se reordena el AST ni la IR |
| Referencias adelantadas o entre unidades ensambladas | Se resuelven sobre el programa completo antes de comprobar cada relación |
| Semántica compartida por varias celdas | Cada relación se comprueba por separado |

El Lenguaje verifica la relación declarada; no atribuye significado a los textos, no selecciona un dominio, no completa interpretaciones y no obtiene claves de otra semántica. La sede es la validación de `CellSpec`, donde existen ambas referencias. No se añade una referencia de codominio a `OutputSemantics`.

El diagnóstico nuevo es **E115 — InvalidOutputSemantics**, fase `validate`, capa efectiva 1 (Estado). Identifica la celda, semántica y codominio; enumera claves repetidas, ausentes y ajenas. La ordenación de esas listas sirve únicamente al diagnóstico determinista. `E102` conserva el rechazo de referencia semántica ausente o de tipo incorrecto. El [catálogo efectivo v0.3](../referencia/ERRORES_CANONICOS_SV_v0_3.md) pasa de 50 a 51 códigos, sin reutilizar identificadores históricos.

Esta precisión refuerza la admisión: las entradas que violan J-K1 dejan de aceptarse. Conserva Gramática 0.2, esquema IR 0.3 y serializador 0.1.0, sin alterar la forma de salida de programas conformes. La representación existente permite expresar la obligación; no se justifica otro tipo de IR ni una ampliación de superficie por este hallazgo.

## 3. Realización y oráculos

La referencia [Python](../../src/svp_validator.py) comprueba multiplicidad y cobertura en `_validate_cellspec`, después de resolver los tipos y antes del descenso que construiría el mapa. [Rust](../../rust/sv_core/src/wellformed.rs) aplica el mismo juicio al programa completo en `validate_object`. Sus rutas públicas de compilación EN, ES y ensamblaje ya atraviesan esa validación; no se introduce otra ruta paralela.

El corpus pasa de **80 a 85 casos: 13 válidos y 72 inválidos**. Se añaden:

- [`output_semantics_vacia.svp`](../../tests/conformance/invalid/output_semantics_vacia.svp).
- [`output_semantics_clave_ausente.svp`](../../tests/conformance/invalid/output_semantics_clave_ausente.svp).
- [`output_semantics_clave_ajena.svp`](../../tests/conformance/invalid/output_semantics_clave_ajena.svp).
- [`output_semantics_clave_repetida.svp`](../../tests/conformance/invalid/output_semantics_clave_repetida.svp).
- [`output_semantics_total_permutada.svp`](../../tests/conformance/valid/output_semantics_total_permutada.svp), con orden independiente y textos compartidos, y su [esperado explícito](../../tests/conformance/valid/output_semantics_total_permutada.expected.json).

El esperado nuevo se declara desde los campos normativos y la huella de la fuente; no se genera desde la salida del compilador. Los doce esperados anteriores conservan su identidad Git. Los cuatro negativos exigen retorno 1, ausencia de IR y E115; no basta un fallo de proceso.

Las [cinco pruebas Python](../../tests/test_output_semantics_totality.py) verifican, además, conservación del AST tras aceptación o rechazo, referencias adelantadas, semántica compartida y conservación de E102. Las [seis pruebas Rust](../../rust/sv_core/tests/output_semantics_totality.rs) ejercen ES/EN, preservación de secuencias y textos, ensamblaje con referencias cruzadas en ambos órdenes de unidades, rechazo de las cuatro clases de defecto y comprobación separada de cada celda. Los subcasos no se suman al corpus de conformidad.

## 4. Sucesión explícita del banco de sensibilidad

[`run_oracle_sensitivity.py`](../../tests/run_oracle_sensitivity.py) conserva las recetas anteriores y pasa al esquema `sv-oracle-sensitivity-v2`:

| Sonda | Resultado exigido tras N0-02 | Estado |
|---|---|---|
| `control_valid` | Admisión y equivalencia estructural | Control conservado |
| `semantics_duplicate` | E115 en ambos emisores y ausencia de IR | Cierre de la relación inválida |
| `semantics_unbound_duplicate` | Se reproduce la pérdida de un miembro en Python y el homónimo en Rust | Testigo residual N0-03 |
| `crlf` | Se detecta la alteración de la identidad de fuente Python | DFL-008 abierta |
| `string_crlf` | Se detecta también la alteración del literal | DFL-008 abierta |

La sonda no enlazada se obtiene de la misma entrada duplicada retirando sólo la declaración `CellSpec`. Su admisión observada no se eleva a una regla normativa: sirve para refutar un cierre global prematuro. El resultado del banco es **un control conforme, un rechazo N0-02 y tres divergencias abiertas detectadas**. No se suman esas sondas a la conformidad. El acta RETP-078 conserva su resultado histórico y remite expresamente a esta sucesión.

## 5. Verificación y condición de promoción

La ejecución local con Rust/cargo 1.98.0 acredita:

| Comprobación | Resultado |
|---|---|
| Conformidad Python y equivalencia R0-7 | 85/85: 13 positivos y 72 rechazos controlados |
| Nuevas pruebas N0-02 | Python 5/5; Rust 6/6 |
| Pruebas Rust anteriores | Internas 210/210; N0-01 3/3; dominios cerrados 5/5; adaptador 2/2; documentales 17/17 |
| Observador, CLI, resistencia SEC-0 y E006 | 16/16; 3/3; 3/3; 4/4 |
| Sensibilidad | Control, rechazo relacional y tres divergencias detectadas según §4 |
| Recepción frente a la base | Las doce salidas de Python y las doce de Rust conservan sus bytes, cada una frente a su propia vía en la base; los cuatro negativos nuevos pasan de admisión a E115 en ambas vías |

La comparación entre esperado, Python y Rust conserva el contrato de pares JSON ordenados de RETP-078; no demuestra igualdad literal entre emisores. La conservación literal frente a la base se comprueba separadamente con `stdout` en bytes sobre los mismos archivos y nombres. No se modifica ningún esperado anterior para acomodar una salida.

Comandos centrales reproducibles desde la raíz:

```bash
python tests/run_conformance.py
python -m unittest discover -s tests -p 'test_output_semantics_totality.py' -v
python -m unittest discover -s tests -p 'test_oracle_support.py' -v
cargo test --manifest-path rust/Cargo.toml --workspace
cargo build --manifest-path rust/Cargo.toml -p sv_native
python tests/r0_7_equivalence.py --rust-bin rust/target/debug/sv-native
python tests/run_oracle_sensitivity.py --rust-bin rust/target/debug/sv-native --output-dir artifacts/oracle-sensitivity
```

La promoción exige Conformidad SVP, R0 Rust, R0-8 Baseline nativa y R0 WASM de tres vías correctos sobre la candidata exacta, con comprobación del árbol de integración frente a la base vigente. WASI y navegador deben recibir el corpus 13/72; las seis sondas DG-01/02/03 de navegador conservan su alcance anterior. Las pruebas bilingües específicas de N0-02 pertenecen aquí a Rust nativo; no se trasladan como evidencia directa de esas sondas en navegador. Las identidades de cabeza, ejecuciones e integración se conservan en el expediente asociado a la rama. Los resultados de PR #65 no acreditan esta candidata.

El índice tabular RETP recupera las filas 075 y 076 que faltaban en su presentación; sus asientos detallados y filas CSV se conservan íntegros. El nuevo asiento es 079.

## 6. Perfiles tecnológicos, límites y relevo

PT04 recibe la identidad E115 y la separación entre rechazo y fallo técnico; PT13 recibe la paridad exigible del mismo programa; PT14 recibe corpus, versiones, entorno y límites. PT01/PT02 conservan identidad de fuentes y salidas en el alcance medido. Los perfiles fuente SVP-EN/SVP-ES mantienen sus contratos; Python conserva su superficie EN, sin atribuirle ensamblaje bilingüe. No aplica contenido de un dominio IMM/CYB a este invariante intrínseco.

Se recibe del [registro experimental 016](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/374a10b73041b5a4ba74909e50df98732aaff735/laboratorio-de-infraestructura-SV/registros/016-VIGILANCIA_Y_COSTE_2026_09_06.md) la separación entre validez e identidad. El [registro 018](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/374a10b73041b5a4ba74909e50df98732aaff735/laboratorio-de-infraestructura-SV/registros/018-COMPARACION_DOTNET_FFI_WASM_2026_09_06.md) conserva sus 79 programas y condiciones .NET/FFI/WASM; no se reinterpreta como ensayo de los 85 casos ni de ensamblaje. No se promueve una nueva realización tecnológica ni se repite esa campaña sin brecha pertinente. La exigencia de prueba previa en laboratorio se conserva para las realizaciones que se pretendan promover bajo el contrato operacional de frontera.

El cierre se limita a **N0-02** y es efectivo al integrarse el expediente con las comprobaciones indicadas. **Sigue N0-03**, dentro de la fila 3: ausencia global de miembros JSON homónimos y estabilidad de la proyección admitida, con atención expresa a declaraciones sin vínculo con `CellSpec`. Se conservan DFL-001 (concordancia diagnóstica general) y DFL-008 (entrada CRLF), además del resto de K1/K1-T. Esta unidad no declara cerrados K1, F, álgebra, K2, núcleo ni R2/R3/R4, y no publica un nuevo artefacto del entorno web.
