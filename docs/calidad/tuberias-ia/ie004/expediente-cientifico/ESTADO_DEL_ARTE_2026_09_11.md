# Estado del arte: interpretación acotada y subordinación de IA externa

**Corte: 11/09/2026 · RETP-2026-129 · Revisión focal, versión 1.** Preparación por Watson, bajo la dirección de Juan Antonio Lloret Egea. [Expediente del estudio](EXPEDIENTE_PARA_REVISION_POR_PARES.md), [bibliografía BibTeX](BIBLIOGRAFIA.bib), [fuentes y alcance efectivo de lectura](FUENTES_Y_ALCANCE.json).

La separación entre un generador probabilístico y un mecanismo externo que impone restricciones tiene antecedentes importantes. **CaMeL y FIDES son comparadores directos de arquitectura**; PICARD y Synchromesh, de interpretación y generación restringida. Por tanto, la novedad del trabajo SV no puede consistir simplemente en «poner un comprobador determinista alrededor de un LLM». La posible aportación debe delimitarse y medirse en su combinación concreta de significado, autoridad, recursos y reproducción. [B03](https://aclanthology.org/2021.emnlp-main.779/), [B04](https://arxiv.org/abs/2201.11227v1), [B08](https://arxiv.org/abs/2503.18813v2), [B09](https://arxiv.org/abs/2505.23643v2).

## 1. Pregunta y método de esta revisión

Pregunta: **¿qué permiten afirmar los trabajos existentes sobre una interfaz española de dominio acotado, asistida por un modelo externo, cuya salida autorizada debe conservar significado, referencia, permisos y evidencia repetible?**

Se hicieron búsquedas dirigidas sobre interpretación semántica, lenguajes controlados, decodificación restringida, control de flujo de información, inyección de instrucciones, evaluación y reproducción. Se combinaron términos con títulos/autores conocidos y seguimiento de enlaces primarios. Las consultas efectivamente realizadas quedan en [BUSQUEDA.json](BUSQUEDA.json). Se consultaron arXiv, ACL Anthology, páginas de los autores e instituciones y W3C; los buscadores sólo sirvieron para localizar esas fuentes.

Se incluyen 18 referencias con contenido primario consultado, de alcance desigual. Para las más próximas se leyeron también secciones del artículo; en otras, metadatos y resumen. **No se declara lectura íntegra de todos los artículos, revisión sistemática, evaluación exhaustiva de 2026 ni prioridad científica demostrada.** Hay publicaciones revisadas por pares, preprints y una nota técnica W3C, identificados individualmente. Los resultados ajenos son declaraciones de sus autores: no se han reproducido sus experimentos.

Se excluyeron como evidencia páginas raíz sin artículo identificable, Wikipedia, publicidad y resultados sin relación material. El acceso primario completo a *Proof-Carrying Code* y a las reglas ACM de evaluación de artefactos no se obtuvo en esta consulta; quedan como lagunas de lectura, sin adjudicarles resultados no contrastados. MASSIVE quedó como ampliación bibliográfica posible, no como banco ejecutado. GIF fue localizado, pero su lectura primaria suficiente quedó pendiente. Esas ausencias limitan el alcance de la revisión.

Las fichas fijan URL, edición o versión cuando existe, fecha de consulta y partes leídas. **El SHA-256 del expediente identifica nuestras fichas, no los PDF editoriales:** no se descargaron sus bytes originales para custodiarlos y su huella queda expresamente sin acreditar. Un DOI identifica una publicación; no sustituye la versión arXiv ni una copia preservada. No se redistribuyen artículos completos.

## 2. Antecedentes comparables

La última columna recoge nuestra interpretación para SV, no una conclusión atribuida a los autores.

| Línea y referencia | Qué aporta el antecedente | Qué debe distinguir nuestro estudio |
| --- | --- | --- |
| Lenguajes naturales controlados · [Kuhn, 2014; B02](https://aclanthology.org/J14-1005/) | Clasifica lenguajes construidos que restringen léxico, sintaxis o semántica; su inventario se centra en inglés | Declarar el español admitido y sus límites. Un vocabulario cerrado de parámetros no demuestra por sí solo comprensión de frases |
| Decodificación restringida · [PICARD, 2021; B03](https://aclanthology.org/2021.emnlp-main.779/) | Valida incrementalmente candidatos de generación SQL; incluye análisis y guardas de esquema | La validez del programa objetivo no basta para acreditar que sirve la referencia y operación solicitadas |
| Restricciones semánticas · [Synchromesh, 2022; B04](https://arxiv.org/abs/2201.11227v1) | Combina recuperación de ejemplos con restricciones de sintaxis, ámbito, tipos y lógica contextual | Sería incorrecto reducirlo a «JSON bien formado». Comparar exactamente qué semántica verifica cada mecanismo |
| Interpretación multilingüe · [MTOP, 2021; B05](https://aclanthology.org/2021.eacl-main.257/) | Evalúa representaciones composicionales en seis idiomas, incluido español | Separar equivalencia semántica de igualdad textual. Las métricas multilingües del banco no acreditan un dominio inmunológico ni nuestros perfiles |
| Recuperación y generación · [RAG, 2020; B06](https://arxiv.org/abs/2005.11401v4) | Combina memoria paramétrica, recuperación documental y generación | Tener fuentes recuperadas no impone por sí solo una respuesta canónica. RAG tampoco implica necesariamente aprendizaje durante cada consulta |
| Propuestas comprobables · [LeanDojo, 2023; B07](https://arxiv.org/abs/2306.15626v2) | Acopla modelos y entorno formal Lean para construir y evaluar demostraciones | Un comprobador formal controla obligaciones formalizadas; la fidelidad de una frase humana a esa formalización necesita una justificación adicional |
| Separación de control y datos · [CaMeL, 2025; B08](https://arxiv.org/abs/2503.18813v2) | Extrae un plan de la consulta confiable y aplica políticas/capacidades al uso de herramientas mediante un intérprete | Comparar supuestos de confianza, flujos excluidos y utilidad. Su implementación investigadora con Python no acredita nuestra realización Rust |
| Control de flujo de información · [FIDES, 2025; B09](https://arxiv.org/abs/2505.23643v2) | Etiquetas de integridad/confidencialidad y políticas deterministas; caracteriza garantías y expresividad de planificadores | Distinguir integridad, confidencialidad explícita y corrección de referencia; no presentar toda «no interferencia» como una única propiedad |
| Evaluación de agentes · [AgentDojo, 2024; B10](https://arxiv.org/abs/2406.13352v3) | Entorno extensible con tareas legítimas y ataques sobre datos de herramientas; registra también dificultad sin ataque | Informar servicio legítimo y seguridad por separado. Rechazar una solicitud debida sigue siendo fallo funcional |
| Salidas estructuradas · [JSONSchemaBench, 2025; B11](https://arxiv.org/abs/2501.10868v3) | Compara eficiencia, cobertura de restricciones y calidad en generación conforme a esquemas | Conformidad al esquema, corrección del contenido y coste necesitan mediciones separadas |

### La comparación decisiva: CaMeL, FIDES y P2/2

CaMeL parte de una consulta confiable y declara límites para la corrección de respuestas textuales que no vulneran sus políticas de flujo. FIDES distingue no interferencia de integridad y secreto explícito; no equipara este último a excluir toda influencia del secreto sobre el control. Ambos obligan a precisar **qué entrada puede influir en qué observable**. Se consultaron CaMeL v2, de 24/06/2025, y FIDES v2, de 03/09/2025; no se mezclan resultados de sus versiones anteriores. [B08](https://arxiv.org/pdf/2503.18813v2), [B09](https://arxiv.org/pdf/2505.23643v2).

En la candidata pública [IE004-ES-P2/2](../perfil-es-p2-2/PERFIL_INTERACCION_ES_IE004_CANDIDATO_2.md), **A obtiene la interpretación y fija el cuerpo canónico antes de verificar la propuesta V**. Es una diferencia operativa relevante, todavía documental. Bajo ese orden, el contenido de V no mejora causalmente una respuesta ya entregada; tampoco puede atribuirse al modelo la cobertura que obtiene A solo. La hipótesis de utilidad adicional de la IA permanece abierta y deberá identificar un producto útil distinto y medible, o reconocer que esta candidata no acredita ese objetivo. No se modifica el algoritmo ni la reserva para resolver esa cuestión en este documento.

## 3. Trabajos recientes de 2026 y límites de sus afirmaciones

| Referencia consultada | Dirección de investigación | Lectura crítica para este expediente |
| --- | --- | --- |
| [AgentSentry, febrero de 2026; B16](https://arxiv.org/abs/2602.22724v1) | Diagnóstico causal temporal y depuración de contexto frente a inyección indirecta | Preprint declarado en revisión; aquí sólo se contrastaron ficha y resumen. No importar sus cifras como comparación con una consulta SV |
| [AI Agents May Always Fall for Prompt Injections, mayo de 2026; B17](https://arxiv.org/abs/2605.17634v1) | Critica defensas mediante manipulación de contexto y normas de flujo | La discusión de imposibilidad del resumen no se toma aquí como teorema universal contra cualquier sistema de política fija. Sí obliga a declarar quién fija contexto y permisos |
| [AgentSecBench, mayo de 2026; B18](https://arxiv.org/pdf/2605.26269v1) | Separa juegos de integridad, confidencialidad y capacidades; ensaya controles benignos/adversos incluso con Qwen3 de 0,6B y 1,7B | Sus marcadores son observables limitados, no seguridad semántica completa. Muestra una vía de evaluación con modelos pequeños; no demuestra que cualquier tamaño baste para nuestro español |

Los tres se registran como preprints en el alcance comprobado. No se ha auditado su código ni verificado sus demostraciones completas. Tampoco se ha encontrado evidencia suficiente para anunciar que una arquitectura existente garantice a la vez comprensión de español abierto, respuesta exacta y disponibilidad bajo cualquier atacante.

## 4. Memoria, ejecución y trazabilidad

Los principios de mediación completa, privilegio mínimo y reducción de mecanismos compartidos preceden a los LLM. Su aplicación obliga a mirar la autoridad efectiva de accesos y dependencias, además de las instrucciones dadas al modelo. [Saltzer y Schroeder, 1975; B01](https://web.mit.edu/Saltzer/www/publications/protection/Basic.html).

RustBelt formaliza seguridad para un subconjunto realista de Rust y condiciones para extensiones que usan `unsafe`. No es una certificación automática de todo programa Rust, de una FFI o de nuestro significado de dominio. El trabajo original de WebAssembly presenta un formato de ejecución validable y portable; elegir ese formato tampoco acredita todos los límites del host que lo ejecuta. [B13](https://www.mpi-sws.org/~dreyer/papers/rustbelt/paper.pdf), [B14](https://research.google/pubs/bringing-the-web-up-to-speed-with-webassembly/).

Para el artículo, las pruebas sintácticas auxiliares de Claude en Python se conservan como evidencia de esa herramienta. La semántica, los desbordamientos, memoria, cancelación y rendimiento de una realización Rust requieren sus propios artefactos compilados y sus propios destinos. Propiedad de una cadena, préstamo y duración de una referencia no deciden la autoridad de su significado.

PROV organiza procedencia mediante entidades, actividades y agentes. Es un antecedente útil para ordenar un expediente auditable; su nota de visión general no prueba que un dato sea verdadero. La investigación sobre explicaciones no fieles en cadenas de razonamiento refuerza otra separación: registrar una explicación del modelo no da acceso comprobado a su proceso causal interno. El expediente registra entradas, propuestas, reglas aplicadas y salidas observables. [B15](https://www.w3.org/TR/2013/NOTE-prov-overview-20130430/), [B12](https://arxiv.org/abs/2305.04388v2).

## 5. Aportación que puede investigarse y afirmaciones que aún no caben

La aportación candidata es **un método verificable de diseño y evaluación de una frontera de consulta en español**, con significado acotado, conocimiento autorizado versionado, unicidad antes de permisos, cuerpo canónico y separación del coste de propuestas externas. Su interés depende de la precisión del contrato, de los contraejemplos que detecte y de lo que demuestre su realización. No se reclama que cada elemento sea nuevo ni que su combinación haya quedado validada.

| Afirmación futura | Evidencia necesaria antes de sostenerla |
| --- | --- |
| Conserva significado ante paráfrasis admitidas | Referencias y cuerpos correctos, por caso y relación, con contexto y versiones fijados |
| Impide sustituciones por una IA externa | Positivos legítimos y ataques que distingan una comprobación débil de la candidata; efectos y accesos observados |
| Evita que V agote A | Separación material de admisión, memoria, trabajo, excepciones y plazos, además de igualdad de salida |
| Es portable y viable | Rust compilado, igualdad relevante nativo/WASI, recursos y entorno registrados |
| La IA aporta utilidad subordinada | Comparación con A solo en la misma tarea; mejora definida antes de medir y sin pérdida de garantías |
| Es útil en práctica inmunológica | Dominio constituido, datos y validación de uso correspondientes; no se deduce del banco artificial actual |

La revisión favorece continuar con el proceso acotado ya acordado: localizar defectos, congelar expectativas, cualificar una candidata y permitir un resultado negativo. No justifica sustituir ahora el núcleo, elegir proveedor, introducir otro idioma o abrir una integración tecnológica. La investigación bibliográfica permanece fuera del conocimiento aplicado por la tubería.

## 6. Bibliografía y disponibilidad

La bibliografía completa, las versiones y el alcance de lectura figuran a continuación y en los archivos asociados. La ficha de una publicación en arXiv no implica por sí sola revisión por pares; donde una aceptación sólo consta en sus metadatos se indica así. El próximo manuscrito requerirá actualizar esta búsqueda en su fecha de cierre y completar las lecturas necesarias para cualquier afirmación más fuerte.

- **B01.** Saltzer, Jerome H.; Schroeder, Michael D. (1975). [The Protection of Information in Computer Systems](https://web.mit.edu/Saltzer/www/publications/protection/Basic.html). Proceedings of the IEEE. Copia HTML de autor del artículo de 1975; consulta 2026-09-11. Artículo publicado; copia de autor consultada.

- **B02.** Kuhn, Tobias (2014). [A Survey and Classification of Controlled Natural Languages](https://aclanthology.org/J14-1005/). Computational Linguistics. Edición publicada de marzo de 2014. Artículo de revista; aceptación indicada en el PDF.

- **B03.** Scholak, Torsten; Schucher, Nathan; Bahdanau, Dzmitry (2021). [PICARD: Parsing Incrementally for Constrained Auto-Regressive Decoding from Language Models](https://aclanthology.org/2021.emnlp-main.779/). Proceedings of the 2021 Conference on Empirical Methods in Natural Language Processing. Edición EMNLP 2021. Artículo en actas revisadas por pares.

- **B04.** Poesia, Gabriel; Polozov, Oleksandr; Le, Vu; Tiwari, Ashish; Soares, Gustavo; Meek, Christopher; Gulwani, Sumit (2022). [Synchromesh: Reliable code generation from pre-trained language models](https://arxiv.org/abs/2201.11227v1). arXiv. v1, 2022-01-26. Versión preprint consultada; sede editorial no cotejada aquí.

- **B05.** Li, Haoran; Arora, Abhinav; Chen, Shuohui; Gupta, Anchit; Gupta, Sonal; Mehdad, Yashar (2021). [MTOP: A Comprehensive Multilingual Task-Oriented Semantic Parsing Benchmark](https://aclanthology.org/2021.eacl-main.257/). Proceedings of the 16th Conference of the European Chapter of the Association for Computational Linguistics: Main Volume. Edición EACL 2021. Artículo en actas revisadas por pares.

- **B06.** Lewis, Patrick; Perez, Ethan; Piktus, Aleksandra; Petroni, Fabio; Karpukhin, Vladimir; Goyal, Naman; Küttler, Heinrich; Lewis, Mike; Yih, Wen-tau; Rocktäschel, Tim; Riedel, Sebastian; Kiela, Douwe (2020). [Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks](https://arxiv.org/abs/2005.11401v4). arXiv. v4, 2021-04-12; depósito inicial 2020. La ficha declara aceptación en NeurIPS 2020; se consultó esta versión arXiv.

- **B07.** Yang, Kaiyu; Swope, Aidan M.; Gu, Alex; Chalamala, Rahul; Song, Peiyang; Yu, Shixing; Godil, Saad; Prenger, Ryan; Anandkumar, Anima (2023). [LeanDojo: Theorem Proving with Retrieval-Augmented Language Models](https://arxiv.org/abs/2306.15626v2). arXiv. v2, 2023-10-27. La ficha declara NeurIPS 2023 Datasets and Benchmarks; versión arXiv consultada.

- **B08.** Debenedetti, Edoardo; Shumailov, Ilia; Fan, Tianqi; Hayes, Jamie; Carlini, Nicholas; Fabian, Daniel; Kern, Christoph; Shi, Chongyang; Terzis, Andreas; Tramèr, Florian (2025). [Defeating Prompt Injections by Design](https://arxiv.org/abs/2503.18813v2). arXiv. v2, 2025-06-24. Preprint consultado; no se acredita aquí aceptación editorial.

- **B09.** Costa, Manuel; Köpf, Boris; Kolluri, Aashish; Paverd, Andrew; Russinovich, Mark; Salem, Ahmed; Tople, Shruti; Wutschitz, Lukas; Zanella-Béguelin, Santiago (2025). [Securing AI Agents with Information-Flow Control](https://arxiv.org/abs/2505.23643v2). arXiv. v2, 2025-09-03. Preprint consultado; no se acredita aquí aceptación editorial.

- **B10.** Debenedetti, Edoardo; Zhang, Jie; Balunović, Mislav; Beurer-Kellner, Luca; Fischer, Marc; Tramèr, Florian (2024). [AgentDojo: A Dynamic Environment to Evaluate Prompt Injection Attacks and Defenses for LLM Agents](https://arxiv.org/abs/2406.13352v3). arXiv. v3, 2024-11-24. Versión arXiv consultada; sede editorial no verificada en esta ficha.

- **B11.** Geng, Saibo; Cooper, Hudson; Moskal, Michał; Jenkins, Samuel; Berman, Julian; Ranchin, Nathan; West, Robert; Horvitz, Eric; Nori, Harsha (2025). [JSONSchemaBench: A Rigorous Benchmark of Structured Outputs for Language Models](https://arxiv.org/abs/2501.10868v3). arXiv. v3, 2025-02-27; título de esta versión. Preprint consultado; no se acredita aquí aceptación editorial.

- **B12.** Turpin, Miles; Michael, Julian; Perez, Ethan; Bowman, Samuel R. (2023). [Language Models Don't Always Say What They Think: Unfaithful Explanations in Chain-of-Thought Prompting](https://arxiv.org/abs/2305.04388v2). arXiv. v2, 2023-12-09. La ficha declara NeurIPS 2023; versión arXiv consultada.

- **B13.** Jung, Ralf; Jourdan, Jacques-Henri; Krebbers, Robbert; Dreyer, Derek (2018). [RustBelt: Securing the Foundations of the Rust Programming Language](https://www.mpi-sws.org/~dreyer/papers/rustbelt/paper.pdf). Proceedings of the ACM on Programming Languages. Artículo 66, enero de 2018; copia de autor. Artículo POPL publicado; formato de referencia comprobado en el PDF.

- **B14.** Haas, Andreas; Rossberg, Andreas; Schuff, Derek; Titzer, Ben; Holman, Michael; Gohman, Dan; Wagner, Luke; Zakai, Alon; Bastien, JF (2017). [Bringing the Web up to Speed with WebAssembly](https://research.google/pubs/bringing-the-web-up-to-speed-with-webassembly/). ACM SIGPLAN Conference on Programming Language Design and Implementation (PLDI). Ficha institucional de la publicación de 2017; copia consultada 2026-09-11. Artículo PLDI 2017 según la ficha institucional.

- **B15.** Groth, Paul; Moreau, Luc (eds.) (2013). [PROV-Overview: An Overview of the PROV Family of Documents](https://www.w3.org/TR/2013/NOTE-prov-overview-20130430/). World Wide Web Consortium. Nota de Grupo de Trabajo, 2013-04-30. Nota técnica W3C; no se presenta como Recommendation ni artículo revisado por pares.

- **B16.** Zhang, Tian; Xu, Yiwei; Wang, Juan; Guo, Keyan; Xu, Xiaoyang; Xiao, Bowen; Guan, Quanlong; Fan, Jinlin; Liu, Jiawei; Liu, Zhiquan; Hu, Hongxin (2026). [AgentSentry: Mitigating Indirect Prompt Injection in LLM Agents via Temporal Causal Diagnostics and Context Purification](https://arxiv.org/abs/2602.22724v1). arXiv. v1, 2026-02-26. Preprint; ficha declara revisión pendiente.

- **B17.** Abdelnabi, Sahar; Bagdasarian, Eugene (2026). [AI Agents May Always Fall for Prompt Injections](https://arxiv.org/abs/2605.17634v1). arXiv. v1, 2026-05-17. Preprint; sin aceptación editorial acreditada aquí.

- **B18.** Alpay, Faruk; Alpay, Taylan (2026). [AgentSecBench: Measuring Prompt Injection, Privacy Leakage, and Tool-Use Integrity in LLM Agents](https://arxiv.org/pdf/2605.26269v1). arXiv. v1, 2026-05-25; identificada en el PDF. Preprint; sin aceptación editorial acreditada aquí.

