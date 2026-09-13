# Composición celular y biblioteca propia del SV: contraste de antecedentes

**13 de septiembre de 2026 · S22 · Ampliación de BIS-01**

## 1. Objeto y autoridad de las fuentes

Por indicación expresa del autor se revisan el compositor histórico IMMUNO-1/IMMUNO-2 y el expediente pulmonar aportado como ejemplos de células, relaciones y resultados. El objetivo es precisar requisitos transversales para (p1+p3)-Bis. Este contraste no incorpora parámetros, reglas clínicas ni células de estos antecedentes al dominio vigente; tampoco cambia su secuencia de trabajo.

El autor explica que la elección de ciberseguridad inteligente como segundo dominio perseguía contrastar la generalidad del Lenguaje fuera de la medicina. Se conserva esta finalidad como criterio de diseño: una sede nuclear debe justificarse por la obligación formal que preserva. La repetición de una necesidad en dos ámbitos médicos no acredita, por sí sola, suficiencia transversal.

SVperitus se inspecciona en el corte `47dc27aec9e7b517c27cfcf39b7ad1b186d36a4c`. La consulta directa de GitHub Pages no pudo recuperarse mediante el lector web; se leyó el HTML de esa aplicación y el código relacionado en el repositorio. No se declara ejecución del demostrador ni identidad comprobada entre ese corte y el despliegue servido por Pages.

El adjunto ZIP contiene 18 archivos bajo la carpeta Pulmón. Se leyeron el maestro de SV-ADC(par) v1 y el maestro global de archivos v2, sus instrucciones y las estructuras pertinentes de sus JSON; se contrastaron también las convenciones META/Γ del global v1 y el desarrollo V1 de la carpeta final III. No se atribuye lectura exhaustiva de todos los borradores auxiliares. El inventario de identidades acompaña este informe. Los rótulos de cierre contenidos en los antecedentes conservan su alcance original; no constituyen cierres del workflow actual.

## 2. Inmuno 2: relación y realización observadas

El README del compositor lo adscribe a la Fase II del mismo agente de Inmunología. El código Python histórico expresa la secuencia IMMUNO-1 → P25 → IMMUNO-2: evalúa la primera célula, transforma su clasificación en el valor puente y evalúa la segunda con ese dato. El propio archivo califica P25 de conector experimental. [README](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/47dc27aec9e7b517c27cfcf39b7ad1b186d36a4c/agentes/inmunologia/fase_2/compositor/README.md) · [compose.py](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/47dc27aec9e7b517c27cfcf39b7ad1b186d36a4c/agentes/inmunologia/fase_2/compositor/compose.py).

La aplicación HTML enlazada usa una exportación WASM `compose` y presenta dos células de 25 coordenadas y una meta-célula de 9. P25 de IMMUNO-2 está bloqueado para edición manual en ese formulario y muestra el valor calculado del puente. Los polígonos consumen los vectores devueltos por el motor. La meta-célula recibe, en este demostrador, datos construidos por una simulación de intrusión. [HTML](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/47dc27aec9e7b517c27cfcf39b7ad1b186d36a4c/aplicaciones/demo_wasm/compositor.html).

La fuente Rust del demostrador define `compose` como función libre exportada mediante wasm_bindgen; no es actualmente un método con receptor. Evalúa las dos células y la meta-célula, y devuelve resultados junto con `system_status`: valid, supervised o vetoed. Los resultados se siguen serializando cuando el estado es vetoed. Esta marca no demuestra por sí sola una barrera contra el consumo indebido. La criticidad Γ de la interfaz se calcula en JavaScript; el código lo declara expresamente. [Rust del demostrador](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/47dc27aec9e7b517c27cfcf39b7ad1b186d36a4c/entornos/rust/wasm/src/lib.rs).

La distinción anterior evita atribuir al antiguo compose.py la ejecución del demostrador WASM o confundir un indicador visual de veto con una protección ya acreditada. El antecedente aporta una relación concreta con posición de destino, transformación y procedencia; su traslado a la DSL exige conservarlas explícitamente.

## 3. Neumología: otras dimensiones y otra composición

| Pieza del expediente aportado | Estructura declarada | Enseñanza para el contraste transversal |
| --- | --- | --- |
| SV-ADC_L | 25 coordenadas, (25,5), sospecha lesional | Dimensión y función deben pertenecer a una constitución identificada. |
| SV-ADC_V | 16 coordenadas, (16,4), validez confirmatoria | Dos células relacionadas pueden tener longitudes y codominios diferentes. |
| Composición L/V | Compuerta jerárquica | La relación no queda determinada por el alfabeto ternario ni por un máximo universal. |
| SV-ADC_META | 9 coordenadas automáticas, (9,3), integridad y clasificabilidad | El mínimo tiene un uso específico; N no determina significado ni autoridad. |
| Γ_ADC-L, Γ_ADC-V, Γ_ADC-G | En el paquete de archivos v2, tuplas de estado, pivote y motivo | El enriquecimiento requiere resultado estructurado y procedencia; no se reduce necesariamente a un Tri. |
| ADC_FINAL | Tupla core, gamma, meta, next_step_domain | Un resultado de composición puede conservar varias dimensiones de información. |

Los 41 parámetros manuales de L y V permanecen distribuidos entre dos células completas. La composición no se convierte por suma en una célula de 41 coordenadas. El maestro pareado propone dos ramas de CNN con fusión tardía, una por imagen; el paquete global amplía sus objetivos. Se trata de descripción y plantillas: esta revisión no aporta ejecución de entrenamiento ni acredita capacidades de una IA.

El significado de las salidas L y V es diferente, aunque sus coordenadas utilicen 0/1/U. La meta-célula también tiene significado propio. Estas diferencias requieren contratos de interpretación y compatibilidad, además de tipos de almacenamiento y tamaño.

## 4. Versiones, representación y límites de reutilización

Los archivos llamados global_v2 conservan internamente rótulos GLOBAL v1. Además, global_v1 y el paquete de archivos v2 no usan idénticas convenciones de META y Γ: la orientación de integridad y los resultados de criticidad difieren. Se registran como convenios de antecedentes distintos, conservando identidades; no se fusionan ni se elige silenciosamente uno como norma vigente.

En el JSON global de archivos v2, el caso semilla contiene U en las coordenadas y etiquetas de ejemplo para Γ y META. Esas etiquetas no son resultados calculados por esta revisión ni constituyen un oráculo ejecutable acreditado.

El HTML usa radiusMap 0→1, 1→2, U→3, seguido de la transformación de pantalla `baseR + radio * stepR`. Esta segunda transformación debe distinguirse del radio algebraico. El expediente pareado declara orden de coordenadas y restricciones de dibujo. Una futura paridad deberá identificar convenio, transformación y consumidor exactos; el parecido entre polígonos no sustituye ese contrato.

## 5. Dictamen sobre una biblioteca propia del SV

La propuesta tiene sentido como organización de software: el SV puede ofrecer una biblioteca base de tipos y operaciones constituidas, implementada en Rust y reutilizable por sus consumidores. El Lenguaje ya contiene una crate biblioteca `sv_core`, declarada en Cargo.toml, con módulos y exportaciones. Eso acredita una forma de organización existente, no la cobertura completa de la biblioteca que se está estudiando. [Cargo.toml del Lenguaje](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/a09cfbc4f55b7d7adedbe2903a079849c83247cc/rust/sv_core/Cargo.toml).

Conviene distinguir tres sedes durante BIS-03:

| Sede conceptual | Contenido que debe justificarse |
| --- | --- |
| Núcleo semántico y validación | Invariantes del SV, tipos, representación de operaciones y comprobaciones indispensables para conservar su significado. |
| Biblioteca base del SV | Operaciones comunes construidas sobre el núcleo, interfaces de funciones o métodos y resultados tipados. Su distribución física puede compartir crate o emplear otras; esta revisión no decide esa partición. |
| Contratos y extensiones de dominio | Constitución de células, interpretación, conectores concretos, condiciones de decisión y cobertura del agente. |

Una operación declarada primitiva recibe significado directo en la semántica/IR o en su realización normativa; exige justificar esa condición. Una operación derivada puede componerse a partir de operaciones ya definidas sin crear una primitiva nueva. Que una función sea pública, interna, un método o parte de una biblioteca no decide esa cuestión semántica.

El mecanismo de transportar una salida mediante un conector puede ser común. Que en un antecedente esa salida alimente P25, o que en otro intervenga una condición confirmatoria, procede del contrato particular. El núcleo debe poder representar y comprobar la relación sin incorporar nombres de enfermedades o reglas clínicas como leyes universales.

En Rust pueden coexistir funciones libres, funciones asociadas y métodos. La elección se resolverá por receptor, estado, invariantes y tipo de resultado, preservando las distinciones algebraicas. El actual compose exportado no obliga a reproducir su forma de API en la DSL. [Rust: crates biblioteca](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) · [Rust: implementaciones](https://doc.rust-lang.org/reference/items/implementations.html).

La denominación propuesta es **biblioteca base del SV**, con un núcleo de operaciones constituidas. Llamarla biblioteca estándar del SV puede ser una decisión posterior sobre contrato público, versiones y compatibilidad. No implica modificar std de Rust ni trasladar toda extensión al núcleo de confianza.

## 6. Incorporación al workflow y siguiente contraste

BIS-01 recibe tres familias de ejemplo diferenciadas: serie con puente posicional; compuerta entre células heterogéneas; supervisión y enriquecimiento con resultados estructurados. En BIS-02 se prepararán casos de preservación de dimensiones, identidad, interpretación, dirección del conector y tipo de salida. BIS-03 determinará qué existe ya, qué es derivable y qué insuficiencia requiere modificación.

El banco deberá conservar también un contraste no médico, procedente del expediente de ciberseguridad competente, antes de concluir generalidad. No se crearán parámetros de ciberseguridad por analogía clínica. Se mantiene abierta la correspondencia con semántica V0.2 e IR 0.3 y la realización de pruebas materiales. Esta entrega aporta revisión documental y estática, sin cambio del núcleo, dominios, agentes ni antecedentes.
