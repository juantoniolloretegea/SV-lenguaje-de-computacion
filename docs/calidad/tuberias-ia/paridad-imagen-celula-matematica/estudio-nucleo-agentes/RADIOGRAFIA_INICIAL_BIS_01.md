# Radiografía inicial de (p1+p3)-Bis

**13 de septiembre de 2026 · S22 · BIS-01 en ejecución**

## Resultado y perímetro

El primer contraste permite preparar el contrato y las pruebas sin presuponer una sustitución general del núcleo. Se han identificado representaciones y controles existentes que deben conservarse y una diferencia de alcance entre el Frame de IR 0.3 y la pareja matemática/visual propuesta. La suficiencia integrada permanece pendiente.

Corte público: `e7370d1ad3e75d829b5a032e156d7691aa5a94fd`. Corte de laboratorio: `457f9ffffb2e669933805aad55ea1c239bdeabaf`. Las rutas y blobs recuperados figuran en [FUENTES_CORTE.json](FUENTES_CORTE.json). Se consultaron AGENTS, Pilares y el acta de perfiles; la lectura completa de la transición y los fundamentos efectuada en S20 se reutiliza con identidad cotejada. Se volvió a examinar expresamente el capítulo 7 de los fundamentos para evaluar la propuesta sobre métodos.

La revisión de realización es estática: tipos y construcción de `ir.rs`, definición, validación y accesos de `frame.rs`, representación de `nat.rs`, tramos de análisis de perfiles y CellSpec de `frontend.rs`, validaciones de dimensión y longitud de `wellformed.rs`, pruebas declaradas en `cell_geometry_native.rs` y documentación inicial de `lib.rs`. Se examinaron las disposiciones pertinentes de perfiles SVP y de IR 0.3. El manifiesto incluye archivos recuperados para el trabajo posterior; no atribuye lectura integral a todos ellos.

## Hallazgos y decisiones de estudio

| Obligación | Evidencia inspeccionada | Resultado estático | Trabajo necesario |
| --- | --- | --- | --- |
| BIS-O01/O02: dimensión y longitud | `frontend.rs`: análisis de CellSpec; `wellformed.rs`: CellSpec, CellState y CoupledState; `nat.rs` | El frontend deriva n de b; la validación comprueba el mínimo y las longitudes. Nat utiliza representación decimal canónica. | Distinguir expresividad matemática y soporte material; constituir los tamaños admitidos. No se acredita E003 como protección ejecutable por este hallazgo. |
| BIS-O02/O05: almacenamiento y admisión | `ir.rs`: IrProgram, IrObject, IrObjectKind y módulo construction | Existen Vec internos, campos privados y accesos mediante referencias compartidas. Los constructores de programa y objetos quedan en ámbito de crate. | Evaluar arrays frente a almacenamiento encapsulado por obligación y coste. La presencia de Vec no demuestra por sí sola crecimiento externo de un estado admitido. |
| BIS-O04/O06: significado de Frame | IR 0.3 §4 y `frame.rs` | El Frame existente reúne referencias de arquitectura, estados y resultados con cierre estructural y causal. Sus campos y constructor restringen la construcción directa. | Precisar cómo se relaciona ese marco con la pareja de una célula y su imagen. No renombrar, reemplazar ni equiparar ambos objetos por coincidencia nominal. |
| BIS-O06: composición y métodos | Fundamentos §§7.2–7.12; objetos CompositionGraph, SemanticRelation, Pattern, Connector; operación Compose en `ir.rs` | La definición algebraica exige composición tipada. La IR distingue estructura y operación declarada. | Fijar contrato, receptor, entradas, resultado, efectos y errores de cada método. La existencia de una declaración en IR no acredita toda su ejecución material. |
| BIS-O07: U y fallo | Tipos de estados y errores de cierre en `frame.rs`; restricciones rectoras | La API examinada contiene causas específicas de cierre inválido. | Mantener fallos técnicos y rechazos separados de la indeterminación válida en toda nueva frontera. |
| BIS-O11: idioma | Especificación SVP-ES/SVP-EN §§6–7; frontend y documentación Rust | Los comentarios quedan fuera de la traducción del perfil fuente. Los perfiles y comentarios del compilador tienen sedes distintas. | Aplicar la política ES/EN a documentación Rust; conservar pruebas, identificadores y diagnósticos canónicos. |
| BIS-O01/O10: tamaños en pruebas | `tests/cell_geometry_native.rs` | Hay cuatro pruebas declaradas: b inferior al mínimo; derivación para b=3 y b=4; longitudes 8/10 incompatibles con b=3; aceptación de 16 coordenadas con b=4. | Reejecutarlas en entorno identificado y ampliar con dimensiones constituidas, composiciones y presupuesto. Su lectura no acredita ejecución ni suficiencia de cargas mayores. |
| BIS-O03/O08/O09/O12: recorrido completo | Requisitos V2 y antecedentes S20; recuperación de fuentes para contraste | Esta radiografía no acredita el recorrido integrado de identidad, imagen consumida, explicación y defensa de autoridad. | Descomponer el recorrido en casos y evidencias antes de cerrar paridad o capacidad de una IA. |

Las referencias de código de la tabla corresponden a `rust/sv_core/src/`, salvo la ruta explícita de pruebas bajo `rust/sv_core/`. Para inspeccionar el corte exacto: [fuentes Rust](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/tree/e7370d1ad3e75d829b5a032e156d7691aa5a94fd/rust/sv_core), [IR 0.3](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e7370d1ad3e75d829b5a032e156d7691aa5a94fd/IR_CANONICA_BIENFORMACION_SV_v0_3.md) y [perfiles fuente](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e7370d1ad3e75d829b5a032e156d7691aa5a94fd/ESPECIFICACION_NORMATIVA_PERFILES_FUENTE_SVP_ES_EN_v1_2026_08_29.md).

## Evaluación de la propuesta sobre métodos

La propuesta es adecuada como forma de implementación. La composición exige conservar también sus participantes, vínculos y contrato como estructura inspeccionable. La célula puede ofrecer métodos correspondientes a sus operaciones; una relación o arquitectura puede ser el receptor apropiado para operaciones multicelulares. El contrato determina esa elección y el tipo de resultado.

No se introduce una operación binaria universal: los fundamentos distinguen dominancia, compuerta, serie, supervisión, enriquecimiento y construcción de sistema. La API no puede borrar esas diferencias ni inventar propiedades algebraicas. El [workflow §5.1](WORKFLOW_P1_P3_BIS_v1.md#51-operaciones-de-composición-expresadas-mediante-métodos) recoge la evaluación como criterio para BIS-02 y BIS-03, sin declarar métodos nuevos implementados.

## Documentación y perfiles

Se adopta una política de comentarios y documentación Rust en español e inglés para las unidades que se introduzcan o revisen. Los perfiles fuente SVP se mantienen independientes. Los ejemplos de DSL conservarán su perfil declarado; los ejemplos ejecutables y compile_fail de rustdoc conservarán su finalidad de prueba. El contenido de los comentarios puede alterar la identidad de los archivos aunque no altere el significado del programa. Esta entrega no traduce masivamente el código existente.

## Comprobaciones realizadas y límites

Se recuperaron fuentes de cortes identificados y se cotejaron sus identidades Git. Se revisaron estáticamente las piezas enumeradas. Las consultas `rustc --version` y `cargo --version` devolvieron comando no encontrado en este entorno. No se ejecutaron nuevas pruebas Rust, rustdoc, WASM ni inferencia de IA. No se atribuyen resultados de ejecución a la lectura de archivos de pruebas.

No se emite todavía dictamen global sobre semántica V0.2: debe fijarse su fuente normativa exacta y sus adendas, distinguiéndolas de la gramática V0.2 y de IR 0.3. Tampoco se constituye una lista finita de N a partir de ejemplos de tamaños.

## Continuación concreta

Completar la correspondencia documental de semántica y las obligaciones aún no recorridas; después preparar el contrato candidato y el banco previo BIS-02, incorporando la distinción entre estructura de composición y métodos de construcción, validación o evaluación. La realización del banco requiere un entorno Rust identificado. Cada carencia debe tener un caso discriminante y una sede justificada antes de modificar semántica, IR o núcleo. El retorno al catálogo corresponde a BIS-08.
