# Acta técnica N0-03: unicidad de miembros y estabilidad de la proyección JSON

**Fecha:** 6 de septiembre de 2026

**Registro:** RETP-2026-080

**Corte de entrada:** `016b2f4d1f896dcbd4e8d177e8db4e736e2cd571`, integración de [PR #66 / N0-02](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/66).

**Expediente:** [historial de la candidata N0-03](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commits/n0-03-proyeccion-json-20260906), con su PR y controles asociados.

**Frente y estado:** fila 3 de K1; cierre de N0-03 condicionado a la promoción del candidato exacto.

## 1. Fuente y defecto recibido

Se reciben los [Pilares](../calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md), el [acta de perfiles](../calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md), la [secuencia completa con relevo §20](../dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md) y el [acta N0-02](./ACTA_TECNICA_N0_02_TOTALIDAD_Y_UNICIDAD_DE_OUTPUT_SEMANTICS_2026_09_06.md). Se comprueba su identidad respecto de las piezas ya leídas en el corte anterior. La [radiografía N0-03](./N0_RADIOGRAFIA_DE_OBJETOS_INVARIANTES_Y_ORACULOS_DEL_NUCLEO_SV_2026_09_04.md) exige ausencia de miembros JSON homónimos y estabilidad de la proyección admitida.

N0-02 cerró cada relación `CellSpec–OutputSemantics–Codomain`. El testigo residual `semantics_unbound_duplicate` quitaba sólo la celda del programa duplicado: ambos emisores volvían a admitirlo; Python perdía un miembro al construir el mapa y Rust conservaba dos nombres homónimos. La equivalencia de retornos de proceso no acreditaba integridad de representación. Se reproduce esa admisión sobre el corte de entrada antes de comprobar su rechazo en la candidata.

## 2. Inventario de las sedes de emisión

Se revisan íntegramente el [descenso Python](../../src/svp_ir.py), su [serializador](../../src/svp_serialize.py), la [proyección Rust](../../rust/sv_core/src/equivalence.rs) y las guardas aplicables de ambos validadores.

| Sede del esquema vigente | Riesgo y guarda |
|---|---|
| `OutputSemantics.mappings` | Pares con claves declaradas: Python formaba un mapa y Rust concatenaba los miembros. N0-03 exige unicidad en toda declaración, antes de exponer la proyección. |
| `Connector.mapping` | Segundo mapa con claves variables. Ambos validadores ya rechazan repeticiones y cobertura incorrecta; se conserva la guarda y se añade un contraejemplo explícito al corpus. |
| Tablas de admisibilidad, sucesos, parámetros y demás secuencias | Se emiten como filas o listas con campos fijos. Sus elementos no se convierten en nombres de miembros JSON; sus otras obligaciones siguen en sus sedes. |
| Cabeceras, objetos, operaciones y campos opcionales | Nombres fijos y distintos dentro de cada objeto del esquema. Los valores textuales se escapan; no se concatenan como nuevos nombres de campos. |

En Rust, `IrProgram` y sus objetos conservan campos privados y acceso público de lectura. Las rutas `compile_svp`, `compile_svp_profile` y `compile_svp_assembly` atraviesan la validación completa. La afirmación de admisión no incluye objetos fabricados fuera de ese contrato. Python recibe igualmente un AST validado antes del descenso; invocar directamente un constructor interno del flujo con datos no validados no acredita admisión.

La obligación general se fundamenta en este esquema cerrado, la guarda de sus dos mapas variables y el escape de valores. Las pruebas ejercen los casos enumerados; no se presentan como demostración formal mecanizada de todos los programas posibles.

## 3. Decisión normativa y realización

Se constituye **J-J0** en [IR v0.3 §6.3](../../IR_CANONICA_BIENFORMACION_SV_v0_3.md#proyeccion-n0-03). Toda `OutputSemantics` tiene claves únicas, con independencia de su uso. La comparación es local a cada mapa, sin deduplicación, fusión ni normalización de nombres. Distintos objetos pueden compartir claves y textos. No se exige cobertura de un codominio no referenciado; la forma vacía no enlazada conserva su admisibilidad estructural previa, sin acreditar suficiencia operacional.

La misma infracción de multiplicidad conserva **E115 — InvalidOutputSemantics**. Se amplía explícitamente la sede de comprobación y se precisa su texto general Python; el [catálogo efectivo §2.3](../referencia/ERRORES_CANONICOS_SV_v0_3.md) permanece en **51 códigos**. Sin celda vinculante, el detalle identifica sólo la semántica y sus claves repetidas. E102 conserva el rechazo de referencia semántica ausente o de tipo incorrecto.

La comprobación complementaria recorre las declaraciones después de las validaciones existentes. Así se conservan los rechazos relacionales N0-02 y su precedencia, también con referencias adelantadas o entre unidades. El rechazo ocurre antes del descenso a mapas en Python y antes de devolver un programa admitido en Rust. No se modifica la implementación de los emisores ni se introduce una segunda ruta de serialización productiva.

Gramática 0.2, esquema IR 0.3 y serializador 0.1.0 conservan sus versiones. La entrada con claves repetidas no enlazadas deja de admitirse. El texto general Python de E115 cambia deliberadamente para describir ambos alcances; código, nombre, capa, fase y detalles relacionales se conservan. Esta actualización textual no se presenta como igualdad literal con el diagnóstico anterior.

## 4. Oráculos y corpus

El [observador](../../tests/oracle_support.py) añade `assert_json_roundtrip`: lectura del JSON con pares ordenados, escritura del valor y nueva lectura. Comprueba miembros, valores, tipos, orden y tokens numéricos, sin estrechar naturales ni convertirlos a coma flotante. Rechaza homónimos antes de perderlos en mapas, incluidos nombres iguales después de decodificar escapes. La escritura pertenece al banco de pruebas y no define otro serializador de IR. Se permiten sólo las diferencias de presentación y escapes equivalentes ya previstas por RETP-078.

La comprobación entra en la conformidad directa y en `assert_success`, utilizado por R0-7, WASI y la preparación del manifiesto de navegador. El destino navegador compara el documento con los bytes nativos previamente comprobados. Ese contraste no equivale a ejecutar dentro del navegador el mismo lector Python.

El corpus pasa de **85 a 88 programas: 14 válidos y 74 inválidos**:

- [`output_semantics_sin_celda_repetida.svp`](../../tests/conformance/invalid/output_semantics_sin_celda_repetida.svp): E115 para duplicación no enlazada, incluso con textos iguales.
- [`connector_clave_repetida.svp`](../../tests/conformance/invalid/connector_clave_repetida.svp): control E007 de una protección anterior; no se presenta como corrección nueva de Connector.
- [`output_semantics_independientes.svp`](../../tests/conformance/valid/output_semantics_independientes.svp): claves locales, textos compartidos, mapa vacío, ambas familias de mapas y texto con LF, tabulación, barra inversa y Unicode. Su [esperado](../../tests/conformance/valid/output_semantics_independientes.expected.json) se declara desde el esquema y la huella de fuente, sin generarlo desde el compilador.

Los trece esperados anteriores conservan su identidad. Las [tres pruebas Python N0-03](../../tests/test_json_projection.py) verifican el rechazo antes de descender a mapas, la ausencia de mutación y la conservación de cada miembro de los mapas del corpus desde el AST. Las [cinco pruebas Rust](../../rust/sv_core/tests/json_projection.rs) ejercen ES/EN, claves compartidas entre objetos, preservación del programa, ensamblaje en ambos órdenes y la guarda de Connector. Las [pruebas del observador](../../tests/test_oracle_support.py) pasan de 16 a 19 e incluyen un número de 5000 dígitos y homónimos expresados mediante escapes equivalentes. Estas subdivisiones no se suman al corpus de conformidad.

## 5. Sensibilidad, resultados y promoción

[`run_oracle_sensitivity.py`](../../tests/run_oracle_sensitivity.py) pasa al esquema `sv-oracle-sensitivity-v3` sin modificar ninguna de las cinco entradas de v2. La sonda no enlazada ahora exige E115; no se elimina ni se acepta cualquier cambio de comportamiento. Resultado: **un control conforme, dos rechazos N0-02/N0-03 y dos divergencias CRLF abiertas detectadas**. Las dos sondas de DFL-008 conservan sus pérdidas observadas y no se incluyen en el corpus conforme.

Verificación local con Rust/cargo 1.98.0:

| Comprobación | Resultado |
|---|---|
| Conformidad y R0-7 | 88/88: 14 positivos y 74 rechazos controlados |
| N0-03 Python / Rust | 3/3 y 5/5 |
| Observador | 19/19 |
| Rust anterior | 210 internas; 3 N0-01; 6 N0-02; 5 dominios cerrados; 2 adaptador; 17 documentales, todas correctas |
| Python N0-02, CLI, SEC-0 y E006 | 5/5; 3/3; 3/3; 4/4 |
| Recepción de positivos | Trece salidas por emisor conservan exactamente sus bytes frente a la base, cada una en su propia vía |
| Recepción de negativos | Los 72 rechazos anteriores conservan retorno e identidad; Rust conserva sus bytes diagnósticos. Python sólo cambia el texto general E115 en sus cuatro casos, conservando los demás bytes y los detalles relacionales |

Comandos centrales reproducibles:

```bash
python tests/run_conformance.py
python -m unittest discover -s tests -p 'test*.py' -v
cargo test --manifest-path rust/Cargo.toml --workspace
cargo build --manifest-path rust/Cargo.toml -p sv_native
python tests/r0_7_equivalence.py --rust-bin rust/target/debug/sv-native
python tests/run_oracle_sensitivity.py --rust-bin rust/target/debug/sv-native --output-dir artifacts/oracle-sensitivity
```

La promoción exige Conformidad SVP, R0 Rust, R0-8 Baseline nativa y R0 WASM de tres vías correctos sobre la candidata exacta, con identidad del árbol de integración y base vigente. WASI y navegador reciben 14/74; las seis sondas DG-01/02/03 conservan su alcance. Las pruebas específicas ES/EN y de ensamblaje N0-03 pertenecen a Rust nativo; no se atribuyen como pruebas directas de esas variantes en navegador. La cabeza, ejecuciones y commit de integración quedan identificados en el expediente enlazado. Los verdes de PR #66 no se transfieren.

## 6. Perfiles, laboratorio y siguiente paso

PT04 recibe rechazo controlado e identidad diagnóstica; PT13, paridad de la proyección; PT14, corpus, entorno y límites. PT01/PT02 conservan las identidades de fuentes y artefactos en el alcance comprobado. La corrección es intrínseca al Lenguaje: no aplica contenido IMM/CYB ni se constituye un contrato de dominio o agente. La representación existente basta para la obligación y no justifica añadir un tipo de perfil tecnológico a la IR.

Se reutilizan los oráculos de RETP-078/079 y la distinción validez/identidad del [registro de laboratorio 016](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/374a10b73041b5a4ba74909e50df98732aaff735/laboratorio-de-infraestructura-SV/registros/016-VIGILANCIA_Y_COSTE_2026_09_06.md). El [registro 018](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/374a10b73041b5a4ba74909e50df98732aaff735/laboratorio-de-infraestructura-SV/registros/018-COMPARACION_DOTNET_FFI_WASM_2026_09_06.md) conserva sus 79 programas y condiciones; no acredita retrospectivamente el corpus de 88 ni su ensamblaje. La prueba previa de las realizaciones tecnológicas sigue bajo el contrato de frontera, sin repetir una campaña por esta actualización ni promover una plataforma nueva.

El cierre es efectivo al integrarse el expediente con las comprobaciones señaladas. **Continúa N0-04:** `Horizon.architecture` debe denotar una referencia real a `CompositionGraph`, conforme a la radiografía y la secuencia. DFL-008, la concordancia diagnóstica general DFL-001 y el resto de K1/K1-T permanecen abiertos. N0-03 no completa un serializador canónico Rust, no importa IR desde JSON, no prueba suficiencia operacional global, no consolida el núcleo ni cierra R2/R3/R4. El entorno web publicado mantiene su artefacto anterior.
