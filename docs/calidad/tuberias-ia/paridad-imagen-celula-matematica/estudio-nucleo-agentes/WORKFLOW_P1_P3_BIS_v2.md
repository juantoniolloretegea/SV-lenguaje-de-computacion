# Workflow de trabajo de (p1+p3)-Bis

**Versión 2 · 13 de septiembre de 2026 · S22 · Juan Antonio Lloret Egea y Watson**

**Definición del workflow cerrada por instrucción humana. Ejecución en curso.** Esta revisión sucede a la V1, conservada como antecedente. [Copia PDF](WORKFLOW_P1_P3_BIS_v2.pdf).

## 1. Finalidad y alcance

Determinar qué obligaciones de la célula matemática y de su paridad visual conserva la realización vigente, qué necesita un contrato adicional y qué exige modificar semántica, IR, validación o soporte. El resultado debe permitir continuar hacia el catálogo de errores con causas delimitadas y evidencia suficiente. El nombre workflow designa esta secuencia de trabajo; no prescribe una automatización de GitHub Actions.

La [explicación V2](../CELULA_IMAGEN_Y_AGENTES_EN_EL_SV_P1_P3_BIS_2026_09_13_V2.md) conserva el acuerdo conceptual. Este workflow lo convierte en objetos de contraste, entregables y condiciones de salida. Sus fases tienen estado propio: aprobar el plan no equivale a ejecutar sus pruebas ni a cerrar (p1+p3)-Bis.

El trabajo se custodia en el estudio núcleo–agentes, con copia en el laboratorio privado, continuidad en Sucesos SV y RETP, y entrada desde Léame primero. Los originales y las revisiones previas permanecen conservados. Las unidades de dominio constituyen sus células y asignaciones; este estudio no redistribuye inventarios ni decide el tamaño de células clínicas o de ciberseguridad.

## 2. Puerta de entrada y fuentes rectoras

Corte de esta revisión: Lenguaje `b43dce153ee061e298f1c730f4f5ed075c84f3c7`, rama main; laboratorio `9ad7729bd69c26d4e52a6630f815b646a277466f`, rama lab/playground-sv-permanente. El corte inicial V1 permanece identificado en el manifiesto histórico. Las fuentes recuperadas se identifican por ruta y blob en [FUENTES_CORTE.json](FUENTES_CORTE.json); recuperar un archivo no equivale a leerlo íntegramente ni a ejecutarlo. La extensión efectiva de lectura se declara en cada informe.

Antes de cada incremento se compara el corte de entrada con la rama vigente. Se consultan AGENTS, los Pilares, el acta de perfiles y ensamblaje, la transición secuencial y el relevo aplicable. Las lecturas anteriores pueden reutilizarse cuando su identidad y alcance sigan cubriendo la tarea, sin presentar resultados heredados como nuevos.

Referencias principales: [Pilares](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e7370d1ad3e75d829b5a032e156d7691aa5a94fd/docs/calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md), [perfiles, contratos y ensamblaje](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e7370d1ad3e75d829b5a032e156d7691aa5a94fd/docs/calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md), [transición secuencial](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e7370d1ad3e75d829b5a032e156d7691aa5a94fd/docs/dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md), [IR 0.3](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e7370d1ad3e75d829b5a032e156d7691aa5a94fd/IR_CANONICA_BIENFORMACION_SV_v0_3.md) y sus definiciones heredadas. La denominación vigente, recuperada del acta S13 / RETP-182, es gramática/superficie 0.2, IR 0.3 y serializador 0.1.0. Las obligaciones semánticas se leen en Frontera normativa v0, IR 0.3 y las partes heredadas no sustituidas de IR 0.2, junto con los rectores y adendas aplicables. Se rectifica el pendiente nominal de V1: no se busca ni se inventa una especificación independiente titulada «Semántica V0.2». Véase [la radiografía de ejecución](RESULTADO_BIS_01_Y_APERTURA_BIS_02.md).

## 3. Secuencia, productos y criterios de salida

| Etapa | Trabajo | Evidencia y condición de salida |
| --- | --- | --- |
| BIS-00. Fijación | Identificar ramas, rectores, versiones, entrada de lectura, realizaciones y capacidad local de prueba. | Manifiesto de fuentes y estado recuperable. Ausencias declaradas. |
| BIS-01. Radiografía | Recorrer las obligaciones desde definición hasta Rust y consumidores. Distinguir el Frame vigente de la pareja matemática/visual, estados de células, instancias y composiciones. Examinar perfiles y documentación. | Matriz obligación → fuente → representación → comprobación → consumidor → cobertura o carencia. Las ausencias se delimitan al perímetro inspeccionado. |
| BIS-02. Contrato y banco previo | Definir identidad, constitución, N, posiciones, versión, estado, representación, transformación, consumo y operaciones. Identificar el conjunto de tamaños desde constituciones competentes. Comprometer positivos, negativos y esperados antes de ejecutar. | Contrato candidato y banco con fuentes y oráculos independientes. Toda magnitud no constituida queda pendiente; no se inventa un máximo para desbloquear la fase. |
| BIS-03. Decisión de sede | Contrastar cada obligación con frontera semántica y adendas vigentes, IR 0.3, núcleo, representación gráfica, frontera, dominio y agente. Resolver la relación con los tipos vigentes. | Decisión por obligación: ya preservada; falta comprobación; insuficiencia representacional; responsabilidad externa; exclusión expresa. Toda ampliación necesita un caso discriminante y compatibilidad definida. |
| BIS-04. Realización acotada | Implementar en laboratorio el mínimo cambio justificado. Estudiar estructuras, constructores y almacenamiento fijo. Añadir documentación ES/EN al código afectado conforme a la política enlazada. | Fuente identificada, API, encapsulación, cambios de versión si proceden y pruebas de construcción válida e inválida. Ningún nombre nuevo acredita autoridad o capacidad por sí solo. |
| BIS-05. Paridad y recorrido | Ensayar constitución → estado exacto → representación → imagen consumida → explicación. Incluir revisión de estado, transformación y composición. | Preservación posicional, semántica y de procedencia en el alcance acordado; rechazo de sustituciones y desajustes. Distinguir imagen pendiente, producida y efectivamente consumida. |
| BIS-06. Falsación y coste | Atacar las invariantes una por una y medir los recursos relevantes sobre tamaños mayores y composiciones. Reutilizar pruebas previas sólo donde correspondan. | Controles sensibles a alteraciones, resultados desfavorables conservados, latencia/CPU/memoria con entorno y alcance. Viabilidad sólo frente a un presupuesto constituido. |
| BIS-07. Integración y dictamen | Ejecutar las puertas requeridas para el incremento, incluidos perfiles y destinos materiales afectados. Verificar documentación, compatibilidad y ausencia de cambios ajenos. | Dictamen que enumere obligaciones acreditadas, exclusiones y pendientes. Publicación coherente de especificación, código, pruebas y registros cuando haya promoción. |
| BIS-08. Retorno al catálogo | Entregar las causas comprobadas y su concordancia con los diagnósticos existentes. | Acta de retorno y matriz de causas con emisor, etapa, condición, evidencia, precedencia y efecto. El catálogo recibe lo demostrado; no convierte propuestas en garantías. |

BIS-01 puede revelar una carencia que se estudie con una sonda acotada antes de diseñar la solución. Su plan y esperados se fijan previamente. Las fases posteriores no se cierran por acumulación de documentos. Una obligación necesaria y no satisfecha impide admitir la operación correspondiente; una exclusión futura debe ser expresa y compatible con el alcance que se cierra.

## 4. Obligaciones mínimas del contrato candidato

| ID de estudio | Obligación |
| --- | --- |
| BIS-O01 | Preservar Σ={0,1,U}, b ≥ 3, n=b² y el vector plano ordenado. |
| BIS-O02 | Mantener N fijo tras la admisión; declarar tamaños soportados por versión y condiciones de ampliación. |
| BIS-O03 | Conservar identidad y versión de constitución, posiciones, parámetros y procedencia. |
| BIS-O04 | Vincular frmat y frvis a la misma instancia y estado, con la transformación de representación declarada. |
| BIS-O05 | Impedir que entradas o modificaciones no validadas adquieran reconocimiento como objeto constituido. |
| BIS-O06 | Distinguir estados, resultados, marcos de arquitectura y composiciones; preservar sus contratos y la información que requieran sus operaciones. |
| BIS-O07 | Mantener separados Tri.U, rechazo, falta de materialización, indisponibilidad y fallo técnico. |
| BIS-O08 | Identificar el artefacto efectivamente mostrado o consumido por la IA; acreditar cada capacidad NLP/visión en su tarea. |
| BIS-O09 | Conservar límites de autoridad y resistencia frente a instrucciones incrustadas, sustitución de evidencia y explicación sin respaldo. |
| BIS-O10 | Acotar coordenadas y recursos totales: células, composiciones, texto, imágenes, concurrencia y transporte. |
| BIS-O11 | Preservar independencia de perfiles fuente, documentación del código y presentación localizada, con identidad de fuentes y diagnósticos. |
| BIS-O12 | Asegurar trazabilidad desde obligación hasta caso, resultado, límite, versión y eventual causa del catálogo. |

Estos identificadores sirven para el estudio. No son tipos de IR, códigos canónicos de error ni parámetros de dominio.

## 5. Clases, objetos y almacenamiento en Rust

La expresión clase se utiliza como analogía de diseño. En Rust se estudiarán tipos mediante struct y enum, constructores y métodos en impl, y traits cuando su contrato sea necesario. No se presupone una jerarquía de herencia ni se introduce el par conceptual en una tupla pública cuyos componentes puedan sustituirse libremente.

El candidato debe distinguir descripción recibida, validación y objeto admitido. Los constructores y mutaciones autorizadas deben conservar la constitución; las referencias de lectura no deben ofrecer una vía de redimensionamiento o sustitución del estado admitido. Los accesos internos y fronteras de importación requieren examen propio.

[Tri; N] es la opción preferente de estudio para dimensiones disponibles estáticamente. Deben compararse su selección por tamaño, huella de código, copias y disposición de memoria con alternativas de longitud fijada tras validación. Un array puede alojarse dentro de un objeto en memoria dinámica; longitud fija no significa obligatoriamente pila. Un Vec encapsulado puede servir como almacenamiento interno o de recepción sin exponer crecimiento al consumidor. La elección se justifica por invariantes y costes, no sólo por el nombre del contenedor. [Rust: array](https://doc.rust-lang.org/std/primitive.array.html) · [Rust: Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html).

El número de posiciones declarado en ejecución no se convierte por sí solo en un parámetro constante del tipo compilado. Debe definirse la selección o admisión para cada estrategia. No se sustituirán globalmente Vec y String: distintas colecciones tienen cardinalidades y funciones diferentes.

SV(9,3) conserva su condición de mínimo. El banco incluirá dimensiones mayores y composiciones; sus fixtures sintéticas no constituyen células de un dominio real. No se enumeran los 3ⁿ estados para sostener un frame.

### 5.1. Operaciones de composición expresadas mediante métodos

La propuesta de implementar las operaciones mediante métodos de Rust es pertinente. La composición conserva, además, una representación explícita de participantes, vínculos, roles y contrato. El método construye, comprueba o evalúa esa estructura según la operación constituida. Esta distinción permite inspeccionar y auditar la arquitectura sin identificarla exclusivamente con una llamada de código.

Los fundamentos (§§7.2–7.12) establecen una familia de operadores tipados y excluyen una operación binaria universal para cualquier par de células. Por ello, la interfaz debe conservar las diferencias entre dominancia homogénea, compuerta jerárquica, composición en serie, supervisión, enriquecimiento y construcción de sistema. La forma de invocación no introduce asociatividad, conmutatividad ni intercambiabilidad de roles.

| Objeto de la operación | Sede que debe evaluarse en Rust | Obligación de diseño |
| --- | --- | --- |
| Evaluación de una célula constituida | Método de la célula o del evaluador que dispone de su contrato | Entrada y codominio explícitos; criterio normativo identificado. |
| Relación entre células | Método de una relación o composición validada; método de célula sólo si el contrato lo justifica | Compatibilidad semántica, roles, orientación y participantes identificados. |
| Construcción de arquitectura | Función asociada de construcción y métodos de validación de la arquitectura | Representar la estructura y sus enlaces; no reducirla a un resultado ternario. |
| Enriquecimiento o supervisión | Método del objeto competente según la operación | Diferenciar información adicional, resultado y autoridad de paso o veto. |

En la terminología de Rust, un método tiene un receptor `self`, `&self` u otra forma admitida. Un constructor asociado sin receptor es una función asociada. La tabla propone sedes para el contraste; no declara nuevas API existentes. [Referencia de Rust: elementos asociados](https://doc.rust-lang.org/reference/items/associated-items.html#methods).

Cada operación candidata debe declarar receptor, entradas, salida, precondiciones, efectos, contexto, versión del contrato y causas de rechazo. Una salida puede ser una evaluación, un estado o una arquitectura, según su definición; no tiene por qué ser siempre `Self`. La indeterminación matemática válida se mantiene separada del error de construcción o ejecución. La encapsulación debe impedir cambios que eludan la validación; usar `&self` por sí solo no demuestra pureza.

Los traits se introducirán cuando expresen un contrato necesario. Su extensibilidad no debe permitir sustituir las leyes soberanas mediante una implementación arbitraria. Las extensiones de dominio conservarán su vía declarada y validada. BIS-02 fijará estas obligaciones y BIS-03 justificará la sede de cada operación antes de modificar el núcleo.

## 6. Documentación del código como obligación transversal

Se aplica [Documentación Rust ES/EN](DOCUMENTACION_RUST_ES_EN_v1.md). Cada incremento explica contratos, invariantes, límites, errores y decisiones relevantes en ambos idiomas. Se revisan conjuntamente traducción y código. La documentación debe describir el comportamiento existente o identificar expresamente una obligación pendiente.

La revisión es progresiva por unidades coherentes: entrada y perfiles; IR y construcción; validación de células y Frame; resolución y composición; serialización y consumidores. Se aprovecha el incremento que examine cada pieza. No se exige traducir masivamente todo el repositorio antes de contrastar (p1+p3)-Bis, ni se permite aplazar la documentación del código nuevo o modificado hasta el final.

## 7. Prueba, recursos y retorno

El banco debe incluir, según la obligación, dimensión inferior al mínimo, longitud incompatible, tamaño no soportado, ausencia de constitución, orden permutado, radios o leyenda alterados, imagen de otra instancia o de un estado anterior, composición incompatible, falta de evidencia y fallo del renderizador. Se conserva la precedencia de rechazo: un caso que falla antes no acredita una comprobación posterior.

Se medirá sólo lo necesario para la decisión: almacenamiento por estado y conjunto, asignaciones o copias cuando condicionen el diseño, tiempo y memoria de validación/renderizado, coste de imágenes y texto, y límites bajo concurrencia. No se deduce una ventaja de rendimiento de la seguridad de tipos. El coste de inferencia de una IA se registra separadamente del coste del núcleo y del renderizador.

Si falta un compilador o destino exigido, se registra la limitación y se continúa con lo que pueda comprobarse sin suplantar esa ejecución. No se usa Python como sustituto del backend soberano ni se convierte un resultado estático en prueba nativa/WASM.

Para retornar al catálogo, cada causa propuesta debe distinguir regla algebraica, contrato inválido, no admisión de soporte, pérdida de representación, fallo material e indeterminación válida. Se comprueba primero si existe un diagnóstico aplicable; no se crean códigos por analogía nominal. El retorno conserva el estado de todas las obligaciones y la sede de las que continúen fuera del alcance.

## 8. Continuidad operativa

[ESTADO_WORKFLOW.json](ESTADO_WORKFLOW.json) conserva el estado por etapa. Sucesos SV registra la actividad y el historial de revisiones. Léame primero remite a la explicación vigente, este workflow y el último informe. Cada avance declara corte, objeto terminado, comprobaciones realizadas, límites y siguiente acción concreta. El cierre de una etapa no cierra por arrastre las siguientes ni predetermina la secuencia futura de agentes.

## 9. Ampliación documental: ejemplos de composición y biblioteca base

La [revisión de IMMUNO-2 y del expediente pulmonar](CONTRASTE_ANTECEDENTES_COMPOSICION_Y_BIBLIOTECA_SV.md) incorpora a BIS-01 ejemplos de serie con puente, compuerta heterogénea, supervisión y resultados enriquecidos. Son antecedentes de estudio; no modifican ni se incorporan al dominio vigente.

BIS-02 debe explicitar dirección, compatibilidad semántica, identidad de participantes, dimensiones independientes y codominio del resultado. BIS-03 distinguirá primitivas justificadas, operaciones derivadas de una biblioteca base y contratos particulares. La existencia de sv_core como crate biblioteca no acredita la suficiencia de todas esas operaciones. El contraste no médico de ciberseguridad se conserva como requisito antes de concluir generalidad.

## 10. Concordancia de suceso, frame, representación y operaciones

La [concordancia documental](CONCORDANCIA_SUCESO_FRAME_REPRESENTACION_Y_OPERACIONES_SV.md), S22 / RETP-2026-198, contrasta los Documentos III y V, la especificación metodológica de frames, la nota de suceso local/envolvente, la IR 0.3 y el anexo SEC5. Recibe la precisión del autor: suceso como hecho factual; prosa, matemática e imagen como formas de representación según necesidad. Distingue actividad técnica, tipo del horizonte, suceso instanciado, resultado de reevaluación, consulta y representación. Registrar que una IA emitió una afirmación no acredita su contenido.

BIS-02 debe fijar la constitución inicial y la relación entre frame celular y frame de arquitectura, así como distinguir correspondencia gráfica, generación y presentación. El requisito humano de vinculación entre frame y suceso queda recibido sin identificar automáticamente ambos tipos. BIS-03 evaluará primitivas y operaciones derivadas por separado de funciones, métodos y macros Rust. Se conservarán los casos técnicos sin frame y la frontera de la decisión formal SV. No se modifica la semántica, la IR, el núcleo ni la explicación V2 mientras sus contratos de integración sigan pendientes.

### 10.1. Precisión expresa del autor · RETP-2026-199

El suceso es el hecho; la prosa, la matemática y la imagen son formas de representarlo. Se corrige la formulación abreviada que omitía la matemática. BIS-02 debe declarar alcance, fidelidad y correspondencia de las representaciones, sin presumir equivalencia ni reversibilidad universales. El álgebra rige la representación matemática SV y su paridad gráfica; la fidelidad al hecho requiere el respaldo de entrada y trazabilidad correspondiente.

## 11. Premisas de cierre de la definición

El cierre humano comprende el plan de investigación y realización. Las cuestiones de implementación siguientes son objetos del plan, con su etapa y evidencia exigida; su resolución no se presume por aprobar el workflow.

1. **Hecho y representación.** El suceso es el hecho; la prosa, la matemática y la imagen son formas de representarlo. Cada representación declara alcance y respaldo. Registrar una afirmación no acredita su contenido.
2. **Autoridad algebraica.** La célula es un vector plano ordenado de N=b² coordenadas, b≥3, sobre Σ. Los 3ᴺ estados posibles no son una obligación de almacenamiento. Radios y convenios gráficos se subordinan a los fundamentos.
3. **Constitución competente.** La unidad de dominio fija células, tamaños, número, asignación, orden y relaciones. Las funciones de conocimiento nuclear y decisión singular comparten molde sin intercambiar sus significados.
4. **Admisión de tamaño.** N permanece fijo en un objeto admitido. El soporte se declara por versión; ampliarlo exige revisión de compatibilidad semántica, IR y realización, cuyo resultado puede ser ausencia de cambio o cambio justificado. No se establece un máximo universal ni se adopta la célula mínima (9,3) por defecto.
5. **Paridad.** La pareja matemática/visual conserva instancia, estado, constitución, orden y convenio. Se distinguen correspondencia definida, imagen generada y presentación o consumo efectivos. La materialización exacta de frvis se decide en BIS-02/03 con casos, sin imponer imágenes a cada actividad técnica.
6. **Suceso y frame.** Se exige respaldo y trazabilidad de la constitución o reevaluación del frame. Se diferencian tipo del horizonte, instancia, dato de transición y resultado. El frame inicial y el vínculo entre célula y arquitectura reciben contratos explícitos. No se inventa una transición anterior ni se introduce tiempo como primitivo.
7. **Operaciones y biblioteca.** Primitivas, operaciones derivadas y contratos particulares se delimitan antes de elegir funciones, métodos o macros Rust. Los métodos no crean relaciones algebraicas. La biblioteca base conserva las obligaciones soberanas del núcleo; no las desplaza a código de dominio sin control.
8. **Admisión y encapsulación.** Se estudian arrays [Tri; N] como opción preferente para N estático, junto con alternativas encapsuladas cuando estén justificadas. El contenedor no sustituye la validación de tamaños y contratos. No habrá sustitución masiva de Vec por analogía.
9. **Auditoría y actuación.** Se admite diversidad de estrategias compatibles con el contrato. La justificación debe ser contrastable con fuentes y operaciones pertinentes; los registros deben acreditar las actuaciones relevantes. No se confunde una explicación con prueba de que ese fue el proceso efectivamente ejecutado.
10. **Soberanía y documentación.** La IA presta apoyo bajo autoridad humana y límites verificables. Comentarios y documentación del Rust afectado se mantienen en español e inglés, independientes de los perfiles fuente de la DSL.
11. **Contraste transversal.** Inmunología y ciberseguridad son contrastes de generalidad. IMMUNO-2 histórico y el expediente pulmonar aportan antecedentes, sin incorporarse a los universos vigentes. No se fija ahora una secuencia futura de agentes.
12. **Salida.** El catálogo recibe causas comprobadas tras el dictamen de integración. La representación de una obligación, la validación local y su ejecución efectiva conservan estados separados.

## 12. Arranque material de la ejecución

La [radiografía BIS-01](RESULTADO_BIS_01_Y_APERTURA_BIS_02.md) registra la matriz de doce obligaciones, sus fuentes, realizaciones, consumidores y carencias. El cierre de BIS-01 es documental y estático, delimitado al corte inspeccionado. No acredita suficiencia integral ni ejecución Rust.

BIS-02 comienza con [contrato candidato](CONTRATO_CANDIDATO_BIS_02_v0_1.md) y [banco de contraste previo](BANCO_PREVIO_BIS_02_v0_1.json). El banco declara controles positivos y negativos, fuentes, precondiciones y oráculos requeridos; no se presenta como corpus ejecutado. Deben materializarse los fixtures y comprometerse los esperados definitivos antes de la realización correspondiente. Las incógnitas se conservan en el contrato y no se resuelven mediante relleno o convención automática.
