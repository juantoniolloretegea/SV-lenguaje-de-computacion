# SV-lenguaje-de-computacion

**Actualización de esta presentación técnica:** 23 de agosto de 2026  
**Versión del conjunto:** V.1  
**Autor:** Juan Antonio Lloret Egea  
**ORCID:** [0000-0002-6634-3351](https://orcid.org/0000-0002-6634-3351)  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0

---

## Lenguaje de computación del Sistema Vectorial SV

Este repositorio contiene la especificación pública y la implementación de referencia del Lenguaje SV (SVP). Reúne la gramática, la representación intermedia (IR), la etapa frontal de referencia, el catálogo de diagnósticos, la batería de conformidad, el entorno público de comprobación y la documentación de calidad necesaria para conocer el estado efectivo de cada versión.

Los fundamentos matemáticos y semánticos del Sistema Vectorial SV se mantienen en [SV-matematica-semantica](https://github.com/juantoniolloretegea/SV-matematica-semantica). Este repositorio desarrolla su expresión como lenguaje de computación dentro del alcance definido en sus especificaciones.

La secuencia técnica de referencia es:

```text
fundamentos matemáticos y semánticos
        ↓
especificación del lenguaje
        ↓
etapa frontal de referencia
        ↓
representación intermedia
        ↓
diagnóstico y pruebas de conformidad
```

### Accesos directos

- [SVP Playground](https://juantoniolloretegea.github.io/SV-lenguaje-de-computacion/) — ejecución en navegador de la etapa frontal de referencia mediante Python y Pyodide.
- [Historial de versiones](./docs/calidad/HISTORIAL_VERSIONES_LENGUAJE_SV.md) — evolución de Gramática, IR, serializador, implementación y entorno público.
- [Gramática superficial mínima v0.2](./GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_2.md).
- [Representación intermedia y bienformación v0.3](./IR_CANONICA_BIENFORMACION_SV_v0_3.md).
- [Catálogo efectivo de errores v0.3](./docs/referencia/ERRORES_CANONICOS_SV_v0_3.md).
- [Registro de la actualización de la etapa frontal](./docs/calidad/ACTUALIZACION_FRONTEND_REFERENCIA_2026_08_23.md).

---

## Estado técnico vigente

| Elemento | Versión o estado | Descripción |
|---|---|---|
| Gramática superficial mínima | **0.2** | Sintaxis vigente para admisibilidad, `resolve` y `Frame`. |
| Representación intermedia | **0.3** | Estructuras y reglas de bienformación vigentes. |
| Serializador | **0.1.0** | Serialización JSON determinista. |
| Etapa frontal de referencia | **Python** | Análisis léxico y sintáctico, AST, validación, descenso a IR y serialización. |
| SVP Playground | **Python / Pyodide** | Entorno público de comprobación en navegador, alineado con Gramática 0.2 e IR 0.3. |
| Conformidad | **72/72** | 11 casos válidos y 61 inválidos; los archivos de referencia registrados no se modifican durante la comprobación. |
| Núcleo en Rust y WebAssembly | **En desarrollo** | Solicitud de incorporación [#9](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/9); todavía no forma parte de `main`. |
| Biblioteca estándar | **Pendiente** | Su situación se documenta en [`stdlib/README.md`](./stdlib/README.md). |

Las versiones anteriores permanecen accesibles para comparación y trazabilidad. La relación completa entre versiones, fechas, correcciones y evidencia pública se mantiene en el [historial de versiones](./docs/calidad/HISTORIAL_VERSIONES_LENGUAJE_SV.md).

---

## Cambios materiales de Gramática 0.2 e IR 0.3

La revisión vigente incorpora tres correcciones delimitadas. No existe una corrección C04.

### C01 — Admisibilidad técnica separada de `Tri.U`

El alfabeto ternario permanece:

```text
Tri = {0, 1, U}
```

`U` es un valor semántico del lenguaje. No representa por sí mismo un fallo de captura, una inadmisibilidad técnica, una excepción del programa ni una tarea pendiente.

`AdmissibilitySpec` admite exactamente:

```text
{Ok, Degraded, NotAdmitted}
```

`Bottom` permanece como símbolo técnico de fallo de captura de `CaptureSpec`. No pertenece a `Tri` ni a `AdmissibilitySpec`.

Diagnóstico asociado:

```text
E110 — InvalidAdmissibilitySpec
```

### C02 — `resolve` identifica la `U` que se revisa

La forma superficial vigente identifica el estado y la posición:

```svp
let RR1 = resolve((S1, 3), with: RS1,
                  context: ContextoClinico,
                  mechanism: RevisionExperto);
```

La numeración de la posición comienza en 1; la posición debe existir y debe contener `U`. El contexto y el mecanismo declarados deben ser compatibles con el `ResSpec` correspondiente.

Diagnóstico asociado:

```text
E305 — UnsafeUResolution
```

### C03 — Coherencia interna de `Frame`

La sintaxis de `Frame` se conserva. La IR v0.3 exige que las referencias incluidas en un `Frame` sean coherentes entre sí:

- como máximo un `CoupledState` por nodo de arquitectura;
- nodos distintos pueden compartir un mismo `CellSpec` mediante `CoupledSpec` diferentes;
- cada `EvalResult` incluido debe referirse a un estado del propio `Frame`;
- una misma fuente material de evaluación no puede duplicarse;
- cada `GateResult` debe depender de evaluaciones incluidas en el mismo `Frame`;
- las referencias de supervisión deben pertenecer al mismo `Frame`;
- mientras no exista un productor superficial de `CriticalityResult`, `criticalities` debe permanecer vacío.

Estas reglas exigen coherencia de lo declarado, no exhaustividad.

Diagnóstico asociado:

```text
E308 — FrameClosureViolation
```

---

## Procesamiento de referencia

```text
archivo .svp
   ↓
análisis léxico
   ↓
análisis sintáctico
   ↓
AST
   ↓
comprobación de bienformación
   ↓
IR v0.3
   ↓
JSON determinista · serializador 0.1.0
```

Los módulos principales son:

| Archivo | Función |
|---|---|
| `src/svp_lexer.py` | análisis léxico |
| `src/svp_parser.py` | análisis sintáctico y construcción del AST |
| `src/svp_ast.py` | tipos de la representación sintáctica |
| `src/svp_validator.py` | comprobaciones de bienformación |
| `src/svp_ir.py` | transformación a IR v0.3 |
| `src/svp_serialize.py` | serialización JSON determinista |
| `src/svp_errors.py` | catálogo diagnóstico efectivo |
| `src/svp_main.py` | interfaz de línea de órdenes |
| `src/svp_playground_api.py` | interfaz utilizada por el SVP Playground |

La etapa frontal de referencia no constituye por sí sola el entorno completo de ejecución del Lenguaje SV. Producir una IR conforme tampoco acredita por sí mismo la corrección de una realización posterior ni la resistencia del sistema frente a ataques.

---

## Conformidad reproducible

La batería vigente contiene:

```text
casos válidos   = 11
casos inválidos = 61
total           = 72
```

Puede ejecutarse desde la raíz del repositorio mediante:

```bash
python tests/run_conformance.py
```

Los casos válidos comparan la IR emitida con archivos JSON de referencia registrados en el repositorio. Los casos inválidos deben terminar con el código diagnóstico esperado. La integración continua ejecuta la misma batería y comprueba que esos archivos de referencia no cambien durante la ejecución.

La revisión 0.2/0.3 añade los diagnósticos:

```text
E110  InvalidAdmissibilitySpec
E305  UnsafeUResolution
E308  FrameClosureViolation
```

El catálogo efectivo v0.3 conserva el catálogo v0.2 y añade esos tres códigos. Ambos documentos permanecen disponibles:

- [catálogo efectivo v0.2](./docs/referencia/ERRORES_CANONICOS_SV_v0_2.md);
- [catálogo efectivo v0.3](./docs/referencia/ERRORES_CANONICOS_SV_v0_3.md).

---

## Desarrollo en Rust y WebAssembly

La realización en Rust se encuentra en desarrollo en la solicitud de incorporación [#9](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/9). Su objetivo es mantener una sola implementación del núcleo semántico y compilarla para ejecución nativa y para WebAssembly, sin crear una semántica distinta para el navegador.

En `main` continúa integrada la etapa frontal de referencia en Python. El desarrollo Rust/WebAssembly no debe presentarse como vigente hasta que exista una incorporación expresa y sus pruebas correspondientes.

El Playground público continuará utilizando Python/Pyodide durante la transición. Su sustitución se realizará cuando el mismo núcleo Rust pueda demostrar correspondencia suficiente entre ejecución nativa, WebAssembly y la referencia Python.

---

## Deuda técnica y límites que permanecen visibles

La versión actual no debe atribuirse capacidades que todavía no están implementadas o demostradas. Permanecen, entre otras, las siguientes limitaciones:

- `max` y `min` no son construcciones superficiales;
- `PendingU` permanece reconocido pero no habilitado en la sintaxis vigente;
- `ConflictOperator` continúa siendo una obligación pendiente para el régimen `General`;
- el identificador `E204` conserva una divergencia histórica entre la IR y el catálogo efectivo;
- `CriticalityResult` no dispone todavía de productor superficial;
- la biblioteca estándar mantiene desarrollo pendiente;
- la conformidad 72/72 acredita el alcance de la etapa frontal y de sus casos de prueba, no una garantía integral del sistema;
- los mecanismos materiales de seguridad, persistencia y comprobación adversarial pertenecen a etapas posteriores de realización.

Una definición presente en una especificación no implica, por sí sola, que exista sintaxis, implementación, diagnóstico o prueba suficiente para utilizarla.

---

## Calidad, trazabilidad e historial

La documentación pública de calidad se encuentra en [`docs/calidad/`](./docs/calidad/). El [historial de versiones](./docs/calidad/HISTORIAL_VERSIONES_LENGUAJE_SV.md) permite identificar qué versión estaba vigente en una fecha determinada, qué cambió, qué se corrigió sin cambio de versión y qué desarrollos siguen pendientes de integración.

El historial de Git conserva el detalle de cada modificación. El historial de versiones ofrece una lectura resumida de la evolución técnica y enlaza las confirmaciones, solicitudes de incorporación y ejecuciones de pruebas que sirven como evidencia.

---

## Ecosistema SV

| Repositorio | Función |
|---|---|
| [SV-matematica-semantica](https://github.com/juantoniolloretegea/SV-matematica-semantica) | fundamentos matemáticos y semánticos |
| [SV-lenguaje-de-computacion](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion) | especificación e implementación de referencia del Lenguaje SV |
| [SV-motor](https://github.com/juantoniolloretegea/SV-motor) | infraestructura de ejecución e integración |
| [SVcustos-dataset](https://github.com/juantoniolloretegea/SVcustos-dataset) | conjuntos de datos y antecedentes de seguridad estructural |
| [SVperitus-dataset](https://github.com/juantoniolloretegea/SVperitus-dataset) | agentes especializados y conjuntos de datos asociados |
| [SV-banco-de-idiomas](https://github.com/juantoniolloretegea/SV-banco-de-idiomas) | infraestructura lingüística auxiliar |

---

## Anexo histórico conservado

El texto que sigue conserva el contenido del anexo incluido en la presentación pública del 19 de agosto de 2026. Se mantiene por continuidad documental. Sus afirmaciones técnicas deben leerse en relación con la fecha en que fueron publicadas; cuando exista una modificación posterior expresamente versionada, la especificación vigente determina el comportamiento actual del Lenguaje.

La presentación completa de aquella fecha permanece además disponible como [instantánea histórica de 19/08/2026](./docs/historico/README_2026_08_19.md).

---

## Anexo — Manifiesto garante de arquitectura, derechos, obligaciones, garantías y fundamentos del Sistema Vectorial SV

*Documento íntegro con toda la matemática formal, el código verificable, las definiciones algebraicas, la protección de datos, el análisis de fallos documentados de IA y los invariantes congelados del Sistema Vectorial SV.*

---

## Resumen

El Sistema Vectorial SV (SV) es un marco algebraico determinista para la evaluación estructurada de parámetros clínicos con incertidumbre formalmente representada. Este documento establece la arquitectura de integración de capas de inteligencia artificial (IA) en el ecosistema SV, con especificación de actores, contratos formales, garantías algebraicas, protección de datos y verificación independiente. El documento nace del análisis forense de fallos documentados de sistemas de IA médica — específicamente de IBM Watson for Oncology, el Epic Sepsis Model, y el sesgo racial en algoritmos de gestión sanitaria — y establece, para cada categoría de fallo documentado, la respuesta arquitectónica implementada en el SV.

Toda afirmación de este documento tiene respaldo en la implementación verificable de los repositorios públicos del ecosistema SV o en bibliografía científica con DOI verificado. No se incluyen inferencias no verificables ni afirmaciones sin respaldo directo.

---

## Parte I — Antecedentes documentados: fallos de IA en medicina

### I.1 IBM Watson for Oncology / Memorial Sloan Kettering Cancer Center

**Descripción del caso documentado.** IBM Watson for Oncology fue un sistema de inteligencia artificial desarrollado en colaboración con Memorial Sloan Kettering Cancer Center (MSKCC) en Nueva York, diseñado para proporcionar recomendaciones de tratamiento oncológico. Documentos internos de IBM obtenidos por investigación periodística revelaron que el sistema generó recomendaciones de tratamiento "inseguras e incorrectas" en casos de cáncer. El caso más citado en la literatura científica es la recomendación de bevacizumab a un paciente con cáncer de pulmón en situación de riesgo alto de sangrado severo, escenario en el que bevacizumab tiene contraindicación establecida según las guías NCCN y la ficha técnica aprobada por la FDA. La recomendación no fue ejecutada porque el equipo clínico identificó la contraindicación. No se documentó daño directo al paciente en ese caso específico.

**Consecuencias sistémicas.** Cinco años de desarrollo y 62 millones de dólares de inversión no produjeron despliegue clínico real. MD Anderson Cancer Center canceló el proyecto en 2016. Para 2018, más de una docena de instituciones habían cancelado o reducido sus contratos con Watson for Oncology.

**Fuentes verificadas:**
- Ross C, Swetlitz I. IBM pitched its Watson supercomputer as a revolution in cancer care. It's nowhere close. STAT News. 5 septiembre 2017.
- Schmidt C. MD Anderson Breaks With IBM Watson. J Natl Cancer Inst. 2017;109(5):djx113. [DOI:10.1093/jnci/djx113](https://doi.org/10.1093/jnci/djx113)
- Strickland E. How IBM Watson Overpromised and Underdelivered on AI Health Care. IEEE Spectrum. 2019;56(4):24–31. [DOI:10.1109/MSPEC.2019.8910661](https://doi.org/10.1109/MSPEC.2019.8910661)

**Mecanismo causal:** entrenamiento con casos sintéticos elaborados por un grupo reducido de oncólogos de MSKCC, sin representatividad de la variabilidad clínica real, sin actualización sistemática de guías, y sin mecanismo de incertidumbre honesta. El sistema producía recomendaciones con aparente confianza uniforme independientemente de la calidad de los datos subyacentes.

### I.1.bis Nota de precisión probatoria sobre el caso bevacizumab

La referencia al caso bevacizumab debe leerse con **precisión probatoria**: el episodio es ampliamente citado en la literatura secundaria y periodística de investigación, mientras que la literatura biomédica indexada documenta con mayor fuerza la cancelación del programa, la insuficiencia de validación clínica y la ruptura institucional del despliegue. Por ello, el ejemplo se mantiene como **caso emblemático de diseño inseguro**, pero no debe tratarse como si todo su detalle narrativo tuviera el mismo rango de prueba que los artículos indexados.

### I.2 Epic Sepsis Model: fallo de validación externa con adopción masiva

**Descripción del caso documentado.** El Epic Sepsis Model (ESM) es un modelo de predicción de sepsis propietario integrado en Epic Systems, una de las plataformas de historia clínica electrónica más extendidas de Estados Unidos. El estudio de validación señala que Epic albergaba registros médicos de cerca de 180 millones de personas, aproximadamente el 56% de la población estadounidense; esa cifra no equivale al 56% de los hospitales.

**Hallazgos de validación externa independiente:**

- Wong A, Otles E, Donnelly JP, et al. External Validation of a Widely Implemented Proprietary Sepsis Prediction Model in Hospitalized Patients. JAMA Intern Med. 2021;181(8):1065–1070. [DOI:10.1001/jamainternmed.2021.2626](https://doi.org/10.1001/jamainternmed.2021.2626)

Estudio retrospectivo de cohorte, 27.697 pacientes, 38.455 hospitalizaciones, Michigan Medicine, 2018–2019:

| Métrica | Declarado por Epic | Validación externa |
|---|---|---|
| AUC | 0,76–0,83 | 0,63 (IC 95%: 0,62–0,64) |
| Sensibilidad | No declarada | 33% |
| Especificidad | No declarada | 83% |
| VPP | No declarada | 12% |
| Tasa de alertas | No declarada | 18% de todas las hospitalizaciones |

**Conclusión del estudio:** el ESM tiene discriminación y calibración deficientes para predecir el inicio de sepsis. Su adopción masiva a pesar del bajo rendimiento plantea preocupaciones fundamentales sobre la gestión de la sepsis a nivel nacional.

**Problema documentado:** el ESM era un modelo propietario implantado en numerosos hospitales antes de disponer de una validación externa independiente publicada. En la cohorte de Michigan Medicine, su discriminación a nivel de hospitalización fue sustancialmente inferior a la comunicada por el desarrollador. El artículo subraya, además, que las diferencias de definición del desenlace y el carácter propietario dificultaban la evaluación y la transportabilidad del modelo. El uso del coste sanitario como variable sustitutiva de necesidad clínica corresponde al caso analizado por Obermeyer et al., no al ESM.

### I.3 Sesgo racial sistemático en algoritmo de gestión de riesgo en salud

**Descripción del caso documentado.** Un algoritmo comercial ampliamente utilizado en el sistema de salud de Estados Unidos para identificar pacientes candidatos a programas de gestión de riesgo de alta complejidad exhibía sesgo racial significativo que afectaba a millones de pacientes.

**Hallazgos documentados:**

- Obermeyer Z, Powers B, Vogeli C, Mullainathan S. Dissecting racial bias in an algorithm used to manage the health of populations. Science. 2019;366(6464):447–453. [DOI:10.1126/science.aax2342](https://doi.org/10.1126/science.aax2342)

Resultados cuantificados:
- A igual puntuación de riesgo del algoritmo, los pacientes negros presentaban una media del 26% más de condiciones crónicas que los pacientes blancos
- Proporción de pacientes negros identificados automáticamente para gestión de alta complejidad: 17,7%
- Proporción que habría correspondido sin el sesgo: 46,5%
- El fabricante del algoritmo confirmó independientemente estos hallazgos y trabajó con los investigadores para corregirlos

**Mecanismo causal:** el algoritmo predice costes de atención sanitaria en lugar de necesidades de salud. La desigualdad de acceso a la atención médica implica que se gasta sistemáticamente menos dinero en atender a pacientes negros con el mismo nivel de necesidad clínica, y el algoritmo aprendió ese patrón como si fuera información válida sobre el estado de salud.

### I.4 Modelos de predicción de COVID-19: riesgo de sesgo y necesidad de validación externa

**Hallazgos documentados:**

- Wynants L, Van Calster B, Collins GS, et al. Prediction models for diagnosis and prognosis of covid-19: systematic review and critical appraisal. BMJ. 2020;369:m1328. [DOI:10.1136/bmj.m1328](https://doi.org/10.1136/bmj.m1328)

La revisión sistemática viva fue ampliándose con sucesivas actualizaciones. En la actualización que reunió 232 modelos, todos quedaron clasificados con riesgo de sesgo alto o incierto. Entre los problemas recurrentes figuraban el tamaño muestral insuficiente, el sobreajuste, la evaluación incompleta del rendimiento y la falta de validación externa independiente. La conclusión operativa era que no debía recomendarse el uso ordinario de esos modelos antes de una validación externa adecuada.

### I.5 Sesgo de infradiagnóstico en radiología IA para poblaciones vulnerables

**Hallazgos documentados:**

- Seyyed-Kalantari L, Zhang H, McDermott MBA, Chen IY, Ghassemi M. Underdiagnosis bias of artificial intelligence algorithms applied to chest radiographs in under-served patient populations. Nat Med. 2021;27:2176–2182. [DOI:10.1038/s41591-021-01595-0](https://doi.org/10.1038/s41591-021-01595-0)

Los modelos de IA para radiografía de tórax presentaron rendimiento inferior en poblaciones de menor nivel socioeconómico, minorías raciales y pacientes sin cobertura sanitaria, con falsos negativos significativamente más frecuentes en las poblaciones con menos acceso a seguimiento clínico.

### I.6 Consecuencias no previstas del aprendizaje automático en medicina

**Argumento documentado:**

- Cabitza F, Rasoini R, Gensini GF. Unintended Consequences of Machine Learning in Medicine. JAMA. 2017;318(6):517–518. [DOI:10.1001/jama.2017.7797](https://doi.org/10.1001/jama.2017.7797)

Los sistemas de ayuda a la decisión basados en aprendizaje automático pueden producir reducción de la calidad diagnóstica en médicos expertos mediante sesgo de anclaje: la tendencia a subordinar el juicio propio al del sistema, incluso cuando el juicio independiente sería superior.

- Parasuraman R, Manzey DH. Complacency and bias in human use of automation. Hum Factors. 2010;52(3):381–410. [DOI:10.1177/0018720810376055](https://doi.org/10.1177/0018720810376055)

### I.7 El argumento de la interpretabilidad

**Argumento documentado:**

- Rudin C. Stop explaining black box machine learning models for high stakes decisions and use interpretable models instead. Nat Mach Intell. 2019;1:206–215. [DOI:10.1038/s42256-019-0048-x](https://doi.org/10.1038/s42256-019-0048-x)

En contextos de decisión de alto riesgo, diseñar modelos inherentemente interpretables es superior a intentar explicar modelos de caja negra a posteriori. Los métodos de explicabilidad a posteriori no garantizan que la explicación corresponda al razonamiento real del modelo.

---

## Parte II — Tabla de correspondencia: fallos documentados → respuestas SV

| Fallo | Fuente (DOI) | Mecanismo | Garantía SV | Componente SV |
|---|---|---|---|---|
| Recomendación directa de tratamiento | 10.1093/jnci/djx113 | Autoridad clínica usurpada | κ₃ es clasificación del vector, no prescripción | normative_engine, ACTOR-TRANSLATE |
| Caja negra estadística | 10.1109/MSPEC.2019.8910661 | Falta de auditabilidad suficiente | Motor determinista; comparación entre realizaciones Python↔Rust | core.py, comparator.py |
| Entrenamiento en cohorte sesgada | 10.1093/jnci/djx113 | Cambio de distribución | Reglas desde guías publicadas internacionales | normative_engine P01–P25 |
| Sin incertidumbre honesta | 10.1109/MSPEC.2019.8910661 | Certeza fabricada | U estructural algebraicamente forzada | core.py, FRONTERA B.6, D.1 |
| Sustitución del médico | 10.1001/jama.2017.7797 | Sesgo de automatización | Soberanía clínica estructural irrevocable | arquitectura de privilegios |
| Validación externa deficiente del ESM | 10.1001/jamainternmed.2021.2626 | Discrepancia entre rendimiento comunicado y validación independiente; modelo propietario de amplia implantación | Separación entre observación, transducción y clausura; exigencia de trazabilidad y comprobación reproducible | Ternarizer B₀/B₁/B_U + batería de conformidad |
| Carga potencial de alertas (ESM) | 10.1001/jamainternmed.2021.2626 | Con umbral ESM ≥ 6, VPP del 12% y activación aproximada en el 18% de las hospitalizaciones | Separación entre señal auxiliar y decisión; presentación trazable del estado | polígono polar |
| Sesgo racial por variable sustituta | 10.1126/science.aax2342 | Coste sanitario utilizado como sustituto de necesidad de salud | Sin variables de coste como fundamento de la clausura | dominio declarado |
| 232 modelos sin validación externa | 10.1136/bmj.m1328 | Sobreajuste no detectado | Batería de conformidad cruzada niveles A–D | FRONTERA_NORMATIVA |
| Infradiagnóstico en poblaciones desatendidas | 10.1038/s41591-021-01595-0 | Diferencias de rendimiento entre subpoblaciones | Uso A: polígonos sintéticos cuya generación no incorpora variables demográficas declaradas | SVcustos, ACTOR-CNN-A |
| Modelos no interpretables | 10.1038/s42256-019-0048-x | Explicabilidad a posteriori insuficiente | Motor inherentemente interpretable | algebra/core.py |

---

## Parte III — Actores de IA del ecosistema SV

### Principio previo obligatorio (FRONTERA_NORMATIVA D.10)

Ningún actor de IA asigna directamente un valor ternario al vector. Todo actor de IA produce un observable sustituto en un espacio declarado. La ternarización la aplica siempre un Ternarizer formal declarado en el programa SVP del dominio por el experto humano, conforme a la cadena de transducción de la FRONTERA_NORMATIVA B.6. Esta separación es la que garantiza que el SV no reproduce el mecanismo de fallo de IBM Watson.

### ACTOR-NLP-LOCAL

| Campo | Valor |
|---|---|
| Identificador | ACTOR-NLP-LOCAL |
| Modelo | Qwen 3 4B vía Ollama |
| Licencia | Apache 2.0 — uso comercial sin restricciones de escala |
| Contexto | Dominios con datos sensibles; clínico; producción con PHI |
| Huella memoria | ~2,5 GB RAM (cuantizado); sin GPU requerida |
| Datos | No salen del perímetro del responsable del tratamiento |
| Tarea | Extracción de observables en vocabulario cerrado del dominio |
| Salida | dict[str, str] — observables con etiquetas del dominio |
| Alimenta | Ternarizer del dominio → motor algebraico |

**Justificación de la elección:** Qwen 3 4B se propone aquí como opción preferente para una tarea acotada de extracción estructurada sobre vocabulario finito. La elección responde a criterios de suficiencia funcional, consumo de recursos, despliegue local y posibilidad de contraste reproducible. No se presenta como una superioridad universal frente a modelos de mayor tamaño. La referencia de Topol (2019) se mantiene como marco general sobre integración responsable entre inteligencia humana e inteligencia artificial en medicina:

**Nota de precisión sobre la comparación Qwen.** La superioridad concreta frente a Qwen 2.5 7B debe leerse como **criterio de diseño razonado y no como evaluación comparativa universal cerrada** mientras no se publique una batería comparativa propia del SV con mismo conjunto de casos, mismas semillas y mismo entorno de inferencia.

> Topol EJ. High-performance medicine: the convergence of human and artificial intelligence. Nat Med. 2019;25:44–56. [DOI:10.1038/s41591-018-0300-7](https://doi.org/10.1038/s41591-018-0300-7)

**Justificación técnica de Apache 2.0:** la licencia permite inspeccionar, modificar y redistribuir el programa conforme a sus términos, lo que favorece la reproducibilidad técnica. Esta característica no equivale por sí sola a conformidad regulatoria. En el Reglamento (UE) 2024/1689, el artículo 10 se ocupa de datos y gobernanza de datos; la documentación técnica corresponde al artículo 11 y el registro automático de acontecimientos al artículo 12 cuando resulten aplicables.

### ACTOR-NLP-API-LIGHT

| Campo | Valor |
|---|---|
| Identificador | ACTOR-NLP-API-LIGHT |
| Modelo | claude-haiku-4-5-20251001 vía Anthropic API Commercial Terms |
| Contexto | Desarrollo; prototipado; producción no clínica |
| Para PHI | Requiere HIPAA BAA (US) o DPA Art. 28 RGPD (EU) |
| Datos API | Su tratamiento, conservación y posible exclusión de entrenamiento dependen de los términos comerciales y del contrato aplicables a la modalidad utilizada; deben verificarse antes de cualquier uso con datos personales o sanitarios |
| Modo auditoría | claude-sonnet-4-6 disponible como mode="anthropic_audit" |

**Criterio de asignación:** para la extracción de nueve observables de vocabulario finito se reserva el modelo de menor coste computacional previsto para la tarea ordinaria, mientras que el modelo de mayor capacidad queda limitado a funciones de contraste. Esta separación no modifica la autoridad algebraica del SV ni permite que un modelo cierre `U`.

### ACTOR-NLP-AUDIT

| Campo | Valor |
|---|---|
| Identificador | ACTOR-NLP-AUDIT |
| Modelo | claude-sonnet-4-6 vía Anthropic API |
| Contexto | Validación adversarial del extractor; no para producción ordinaria |

### ACTOR-CNN-A

| Campo | Valor |
|---|---|
| Identificador | ACTOR-CNN-A |
| Modelo | ConvNeXt-Tiny (torchvision, pesos ImageNet-1K) |
| Licencia | BSD-3 Clause — uso comercial sin restricciones |
| Uso | A — clasificación del polígono polar sintético |
| Antecedente | ResNet34 era la línea base de Documentos 2–7 del conjunto documental |
| Salida | K₃ ∈ {APTO, NO_APTO, INDETERMINADO} como señal auxiliar |
| Relación con motor | Nunca sobrescribe κ₃ del motor normativo |

**Justificación de ConvNeXt-Tiny como arquitectura de trabajo:** Liu et al. (2022) presentan ConvNeXt como una modernización de las redes convolucionales y documentan su rendimiento competitivo frente a arquitecturas contemporáneas. La adopción de ConvNeXt-Tiny en este proyecto es una decisión técnica propia y no debe atribuirse al artículo una demostración específica sobre menor varianza entre ejecuciones clínicas:

> Liu Z, Mao H, Wu CY, et al. A ConvNet for the 2020s. Proc IEEE/CVF CVPR. 2022;11976–11986. [DOI:10.1109/CVPR52688.2022.01167](https://doi.org/10.1109/CVPR52688.2022.01167)

### ACTOR-CNN-B (no activo — Hito 5, régimen de laboratorio)

| Campo | Valor |
|---|---|
| Identificador | ACTOR-CNN-B |
| Uso | B — extracción de características posicionales desde imagen médica real |
| Estado | No implementado; requiere conjunto de pares (imagen, vector validado) |
| Deudas | DV-HITO5-003, DV-HITO5-004 |
| Condición de activación | Conjunto de datos diverso y documentado con representación demográfica explícita |

La condición de representación demográfica es respuesta directa al fallo documentado por Seyyed-Kalantari et al. (2021): el Uso B no se activará sin un conjunto de datos que incluya diversidad demográfica documentada.

### ACTOR-TRANSLATE

| Campo | Valor |
|---|---|
| Identificador | ACTOR-TRANSLATE |
| Tarea | Traducción del JSON al lenguaje clínico del dominio |
| Acceso a vector | Solo al JSON de salida; no puede modificarlo |
| Prohibición | No puede emitir recomendaciones terapéuticas |
| Separación | Ninguna salida de ACTOR-TRANSLATE afecta la trayectoria algebraica |

**Regla de presentación de resultados (obligatoria):** ninguna salida del sistema puede presentarse al usuario sin exponer explícitamente cada posición con valor 1 y su lectura clínica antes del κ₃ global. Esta regla es la respuesta al mecanismo exacto del caso bevacizumab.

---

## Parte IV — Director de orquesta: el `deployment_profile`

El perfil de despliegue se representa mediante el nodo `deployment_profile` en la IR del SVP. No es un modelo de IA: es un conjunto de reglas declarativas que el compilador SVP verifica en tiempo de compilación y que la infraestructura de ejecución lee durante la ejecución.

### IV.1 El Ternarizer formal

El Ternarizer es la función τⱼ : Oⱼ → Σ que convierte la salida de un actor estadístico en un valor ternario algebraicamente admisible, conforme a la cadena de transducción de la FRONTERA_NORMATIVA B.6:

```
x ∈ Wⱼ → ϕⱼ(x) ∈ Oⱼ^⊥ → rⱼ ∈ R → τⱼ(oⱼ) ∈ Σ → Pⱼ → célula
```

Donde R = {ok, degradado, fallido, U}. Invariante: si la captura falla o la admisibilidad es insuficiente, el resultado es U. La cadena no fabrica certeza.

**Sin Ternarizer declarado, el compilador SVP rechaza el programa.** Esta no es una advertencia — es un error de tipo.

### IV.2 La convergencia obligatoria para posiciones críticas

Para posiciones donde el riesgo de error clínico es significativo, se requiere convergencia de dos caminos independientes:

```
Camino 1 (determinista): Ternarizer aplicado al observable medido
Camino 2 (estadístico): propuesta del actor estadístico

Convergencia:  → asignar el valor del Camino 1
Divergencia:   → U forzada por estructura
Solo Camino 1: → asignar el valor del Camino 1
Solo Camino 2: → U forzada por estructura
```

Un actor estadístico solo puede confirmar lo que el camino determinista ya produjo. Nunca puede producir 0 o 1 por sí solo.

### IV.3 Bloque deployment en el SVP del dominio

```svp
domain IMMUNO-1 {
  cell: SV(25, 5)
  horizon: H_IMMUNO_1
  semantics: SemK3

  deployment {
    input_carril:   direct
    nlp_context:    local
    cnn_use:        A
    privacy_class:  sensitive

    -- Regla de presentación: obligatoria en toda interfaz
    presentation_rule: positions_1_first

    privilege_levels {
      session_operator {
        puede: declarar_observables_directos
        puede: proceder_con_override_informado
        override_require_text: true        -- mínimo 30 caracteres
        alcance_override: session_only
      }
      clinical_validator {
        puede: validar_con_responsabilidad_medica_declarada
        co_signature_required: true        -- para cambios de κ₃ global
        alcance_override: installation_only
      }
      sv_manager {
        puede: cerrar_ciclo_procedimental
        puede_NO: validar_contenido_medico
        debe: cerrar_si_proceso_correcto
        alcance_override: none
      }
    }

    invariantes_no_configurables {
      threshold_fn: "T(n) = floor(7n/9)"
      cell_size:    "n = b^2, b >= 3"
      alphabet:     "{0, 1, U}"
      trayectoria:  "append_only"
      poligono:     "inmutable_tras_evaluacion"
      cnn_override: "CNN nunca sobrescribe motor normativo"
    }
  }
}
```

---

## Parte V — Matemática y álgebra del SV

### V.1 Fundamentos formales

**Definición 1 — Alfabeto ternario irreductible**

Σ = {0, 1, U} donde los elementos son mutuamente excluyentes:
- 0: parámetro en situación adecuada según criterio declarado del dominio
- 1: parámetro en situación de riesgo o que requiere acción
- U: estado epistémico de indeterminación — evidencia insuficiente

Invariante: U ≠ 0, U ≠ 1, U ≠ None, U ≠ NaN. Sin conversión implícita entre elementos.

**Definición 2 — Célula ternaria canónica**

Una célula C es un vector v ∈ Σⁿ donde n = b², b ∈ ℕ, b ≥ 3.  
Valores válidos: n ∈ {9, 16, 25, 36, 49, 64, ...}

**Definición 3 — Funciones de conteo**

N₀(v) = |{i : vᵢ = 0}|, N₁(v) = |{i : vᵢ = 1}|, Nᵤ(v) = |{i : vᵢ = U}|  
Invariante: N₀(v) + N₁(v) + Nᵤ(v) = n

**Definición 4 — Umbral canónico**

T(n) = ⌊7n/9⌋

| n | b | T(n) |
|---|---|------|
| 9 | 3 | 7 |
| 16 | 4 | 12 |
| 25 | 5 | 19 |
| 36 | 6 | 28 |
| 49 | 7 | 38 |
| 64 | 8 | 49 |

**Definición 5 — Clasificación fuerte con precedencia desfavorable**

κ : Σⁿ → K₃ = {NO_APTO, INDETERMINADO, APTO}

```
κ(v) = NO_APTO       si N₁(v) ≥ T(n)
κ(v) = APTO          si N₀(v) ≥ T(n) ∧ N₁(v) < T(n)
κ(v) = INDETERMINADO en cualquier otro caso
```

**Proposición de unicidad (Proposición 6, Fundamentos SV R3):** Para todo v ∈ Σⁿ, κ(v) está unívocamente determinado. NO_APTO y APTO son mutuamente excluyentes porque si N₁(v) ≥ T(n), entonces N₀(v) ≤ n − T(n) < T(n) (ya que 2T(n) > n para n ≥ 9, pues 7/9 > 1/2). □

**Definición 6 — Función de criticidad**

Γ(v) = Nᵤ(v) − min(δ₀(v), δ₁(v))

donde δ₀(v) = T(n) − N₀(v), δ₁(v) = T(n) − N₁(v)

- Γ(v) > 0: las U importan; su resolución puede cambiar κ(v)
- Γ(v) ≤ 0: κ(v) está determinado independientemente de cómo se resuelvan las U

**Definición 7 — Horizonte de sucesos**

H(A) : {1,...,n} → 2^J donde J es el conjunto de tipos de suceso declarados y H(i) es el soporte de la posición i.

**Definición 8 — Etiquetas gamma-H**

Para posición i donde vᵢ = U:
```
label_H(i) = "irreducible"  si H(i) = ∅
label_H(i) = "resoluble"    si |H(i)| = 1
label_H(i) = "fronteriza"   si |H(i)| > 1
```

**Definición 9 — Vector gobernado**

(Γ̄_H(v))ᵢ = 0 si vᵢ ≠ U; = 1 si vᵢ = U ∧ irreducible; = U si vᵢ = U ∧ no irreducible

Las posiciones irreducibles se tratan como 1 (riesgo): principio de defecto desfavorable.

**Definición 10 — Compuerta conservadora**

gate(c, f) = 1 si c=1 ∨ f=1; = U si c=U ∨ f=U (y ninguno es 1); = 0 si c=0 ∧ f=0

**Definición 11 — Agente y κ₃ gobernado**

A(v, H) = gate_vector(Γ̄_H(v), v)  
κ₃ = κ(A(v, H))

**Secuencia completa de evaluación:**

```
v ∈ Σⁿ
  → validate_cell_size(n)  [rechaza n ≠ b²]
  → normalize_vector(v)    [rechaza valores fuera de Σ]
  → gamma_h_labels(v, H)   [clasifica posiciones U por soporte]
  → gamma_bar_h(v, H)      [produce C_gob: irreducibles → 1]
  → gate_vector(C_gob, v)  [produce A_agente: compuerta conservadora]
  → kappa3(C_gob, A_agente) [clasificación gobernada]
  → SVProgramResult [JSON canónico]
```

### V.2 Composición intercelular: IMMUNO-1 → IMMUNO-2

Cuando IMMUNO-2 usa la clasificación de IMMUNO-1 como observable de entrada (P25), se establece una composición en serie por parámetro puente conforme al Documento I del conjunto documental (Lloret Egea, 2026):

El conector φ_{IMMUNO-1 → IMMUNO-2}^{(P25)} : K₃ → Σ es:

```
φ(APTO)          = 0    estado adecuado propagado
φ(NO_APTO)       = 1    riesgo propagado
φ(INDETERMINADO) = U    incertidumbre propagada honestamente
```

**Propiedad fundamental:** la incertidumbre se propaga honestamente. IMMUNO-1 INDETERMINADO → P25 = U. El conector no fabrica certeza. Esta propiedad es verificable en el laboratorio sv_lab_04_compositor_intercelular.py.

### V.3 Implementación verificable del núcleo algebraico

Las siguientes funciones de `sv_motor/algebra/core.py` implementan las definiciones anteriores. Son reproducibles por cualquier auditor que instale sv-motor:

```python
# T(n) = ⌊7n/9⌋ — sin parámetros externos modificables
def threshold(n: int) -> int:
    return (7 * n) // 9

# Restricción n = b², b ≥ 3
def validate_cell_size(n: int) -> int:
    b = isqrt(n)
    if b < 3 or b * b != n:
        raise SVTernaryError(f"n={n} no es b² con b≥3")
    return b

# Alfabeto sin conversión implícita
def _normalize_tri_value(value: object) -> object:
    if isinstance(value, bool):
        raise SVTernaryError("booleano no pertenece al alfabeto {0,1,U}")
    if value in (0, 1, U): return value
    if value == "0": return 0
    if value == "1": return 1
    if value == "U": return U
    raise SVTernaryError(f"Valor fuera del alfabeto: {value!r}")

# Clasificación con precedencia desfavorable
def classify_cell(values):
    vals = normalize_vector(list(values))
    n  = len(vals); validate_cell_size(n)
    n1 = sum(v == 1 for v in vals)
    n0 = sum(v == 0 for v in vals)
    t  = threshold(n)
    if n1 >= t: return K3_NO_APTO    # precedencia desfavorable
    if n0 >= t: return K3_APTO
    return K3_INDETERMINADO

# Compuerta conservadora
def gate_value(left, right):
    l, r = _normalize_tri_value(left), _normalize_tri_value(right)
    if l == 1 or r == 1: return 1
    if l == U or r == U: return U
    return 0
```

---

## Parte VI — Protección de datos

### VI.1 Marco normativo aplicable

- Reglamento (UE) 2016/679 (RGPD): datos de salud como categoría especial (Art. 9); base legal Art. 9.2.h (asistencia sanitaria por profesional con obligación de secreto)
- Reglamento (UE) 2024/1689 (Reglamento de IA de la Unión Europea): posible encaje en escenarios de alto riesgo según uso previsto, interacción con Article 6(1), Annex I/III y, en su caso, contextos de triaje o producto sanitario; requisitos Arts. 9–15 cuando corresponda
- Ley Orgánica 3/2018 (LOPDGDD, España)
- NIST AI RMF 1.0 (NIST AI 100-1, enero 2023)

### VI.2 Arquitectura técnica de protección por actor

| Actor | Perímetro de datos | Entrenamiento en datos de usuario | Requisito adicional para PHI |
|---|---|---|---|
| ACTOR-NLP-LOCAL | Local; sin salida | No aplica (sin API) | Ninguno |
| ACTOR-NLP-API-LIGHT | Servicio externo sujeto a los términos y al contrato vigentes | Según condiciones contractuales aplicables | Acuerdo y garantías de protección de datos que resulten exigibles |
| ACTOR-CNN-A | Local; polígonos sintéticos | No (ImageNet preentrenados) | Ninguno |
| ACTOR-TRANSLATE | Local o API; solo JSON | No | Ninguno para JSON abstracto |
| Motor algebraico | Local; determinista | No aplica | Ninguno |

### VI.3 Minimización de datos (Art. 5.1.c RGPD)

El SV no ingiere narrativa clínica libre. Recibe exclusivamente los observables declarados por el clínico para los parámetros del dominio: valores numéricos de analítica, estados booleanizados de historial clínico, parámetros de tratamiento activo. No procesa notas desestructuradas. Esta decisión es respuesta directa al fallo de Watson, que extraía datos de notas clínicas desestructuradas con resultados no reproducibles.

### VI.4 Derecho de supresión (Art. 17 RGPD) y trayectoria irrevocable

La trayectoria de solo adición es irrevocable por diseño. La base legal para mantener los datos a pesar de una solicitud de supresión es Art. 17.3.b/c (obligación legal, historial médico) y Art. 17.3.d (interés público, investigación). El responsable del tratamiento debe documentar esta base en su Registro de Actividades de Tratamiento (Art. 30 RGPD).

### VI.5 Conformidad Reglamento de IA de la Unión Europea — Arts. 9–15

| Artículo | Requisito | Implementación SV |
|---|---|---|
| Art. 9 | Gestión de riesgos | base doctrinal + REGISTRO_DEUDA_VIVA_SV.csv |
| Art. 10 | Datos y gobernanza de datos, cuando resulte aplicable | Procedencia y reglas de datos documentables; la licencia del programa no sustituye estos requisitos |
| Art. 11 | Documentación técnica | Este documento + conjunto documental de seis repositorios |
| Art. 12 | Registros automáticos | Trayectoria de solo adición con marcas temporales |
| Art. 13 | Transparencia | Polígono polar + ACTOR-TRANSLATE en idioma del dominio |
| Art. 14 | Supervisión humana | Arquitectura de tres niveles (operador / validador / gestor) |
| Art. 15 | Exactitud y robustez | Comparación entre realizaciones Python↔Rust; reproducibilidad garantizada |

---

## Parte VII — Para cada sector humano

### VII.1 El clínico usuario (operador de sesión)

El sistema no produce diagnósticos ni recomendaciones de tratamiento. Produce una evaluación estructurada del estado de un vector de parámetros clínicos según reglas derivadas de guías publicadas. El resultado más frecuente es INDETERMINADO: no porque el sistema falle, sino porque la información disponible no alcanza el umbral algebraico T(n) en ninguna dirección.

La función de criticidad Γ informa sobre la utilidad de completar la evaluación: si Γ > 0, resolver posiciones en U puede cambiar el resultado; si Γ ≤ 0, la clasificación global está determinada independientemente. Esta distinción es clínicamente relevante y no existe en sistemas de clasificación binaria.

Las anulaciones justificadas quedan registradas con el nombre del operador, la justificación escrita y el cambio en κ₃. Las anulaciones que cambian κ₃ requieren co-firma de un validador clínico. Ninguna anulación puede afectar los invariantes algebraicos del motor (T(n), n = b², {0, 1, U}).

Referencia: Cabitza et al. JAMA 2017, [DOI:10.1001/jama.2017.7797](https://doi.org/10.1001/jama.2017.7797); Parasuraman & Manzey, Hum Factors 2010, [DOI:10.1177/0018720810376055](https://doi.org/10.1177/0018720810376055)

### VII.2 El validador clínico

La firma de validación es un evento irrevocable en la trayectoria del expediente con nombre, cargo, institución y marca temporal. Valida que los observables son clínicamente coherentes, que las anulaciones del operador tienen justificación razonable, y que no hay señales de entrada incorrecta sistemática. No valida el resultado algebraico — valida el proceso.

El texto de declaración antes de la firma nomina explícitamente la responsabilidad profesional y la irrevocabilidad del evento. Esto responde al fenómeno de sesgo de automatización documentado por Parasuraman y Manzey (2010): la asignación de responsabilidad nominal al revisor humano reduce la tasa de aceptación acrítica.

### VII.3 La dirección institucional

Tres tipos de valor no disponibles en sistemas de caja negra: trazabilidad legal completa (quién, qué, cuándo, con qué resultado), detección de patrones de calidad sin acusación individual (anulaciones sistemáticas en la misma posición = señal de calibración del dominio, no de individuo), y cumplimiento regulatorio documentable ante Reglamento de IA de la Unión Europea Arts. 9–15 y RGPD Art. 35.

### VII.4 El auditor técnico

Los cuatro laboratorios del conjunto verifican de forma independiente la integridad algebraica del sistema. El resultado esperado es 0 fallos en cada laboratorio. Si hay fallos, la implementación no es conforme con la base doctrinal y el fallo exacto aparece en la salida con campo y valores.

Lo que los laboratorios no verifican: la validez clínica de las particiones (requiere revisión de las guías citadas), la idoneidad del sistema para un contexto específico de despliegue (requiere EIPD local), y la integridad del proceso de validación en producción.

### VII.5 El regulador

El sistema puede quedar comprendido en regímenes de alto riesgo del Reglamento de IA de la Unión Europea según su **uso previsto**, su encaje como producto o componente de seguridad y, en su caso, su papel en triaje o toma de decisión clínica. Esta calificación no debe darse aquí por cerrada sin análisis jurídico de instalación y caso de uso. La Parte VI identifica elementos de diseño relacionados con documentación técnica, registro, supervisión humana, exactitud y robustez. Esa correspondencia no equivale por sí sola a una declaración de conformidad jurídica. Cuando resulte exigible una evaluación de impacto relativa a la protección de datos conforme al artículo 35 del RGPD, deberá elaborarse para la instalación y el uso previstos.

---

## Parte VIII — Los repositorios: cambios declarados

**Nota de estatuto de esta sección.** Esta parte combina cambios ya integrados en algunos repositorios con **cambios propuestos** para incorporación coordinada en otros. No debe leerse como si cada línea enumerada estuviera ya materializada en todas las sedes del ecosistema.


### SV-motor-main (v0.1.8)

- `pyproject.toml`: versión 0.1.7 → 0.1.8
- `extractors/ext_nlp.py`: modelo Anthropic por defecto → claude-haiku-4-5-20251001; añadir clase CaptureResult con campo admisibilidad
- `docs/arquitectura/10_actores_modelo_declaracion.md`: revisar terminología (Ternarizer formal, no ternarization_contract)
- `docs/arquitectura/11_leccion_watson_y_garantias_sv.md`: nuevo (este conjunto)
- `REGISTRO_DEUDA_VIVA_SV_MOTOR.csv`: DV-SVM-013 a 017

### SVcustos-dataset-main

- Todos los yamls: `model: "resnet34"` → `model: "convnext_tiny"` + `historical_baseline: "resnet34"`
- Nuevo `train_convnext.py`: mismo patrón que `train_resnet.py`, backbone ConvNeXt-Tiny

### SVperitus-dataset-main

- `imm_n25.yaml`, `imm2_n25.yaml`: `architecture: convnext_tiny` + `historical_baseline: resnet34`
- `Documento7_IMMUNO-1.md`: Apéndice A (lección Watson) + Apéndice B (Ternarizer formal pendiente — DV-PERITUS-002)

### SV-banco-de-idiomas-main

- Nuevo `banco/observables_proxy_spec.yaml`: observables sustitutos y particiones B₀/B₁/B_U para dominio NLP

### SV-matematica-semantica-main

- `PLIEGO_DE_CONDICIONES`: sección explícita sobre actores estadísticos como sirvientes con contratos formales (Ternarizer obligatorio)

### SV-lenguaje-de-computacion-main

- `FRONTERA_NORMATIVA`: nota de aplicación de B.6 a actores estadísticos intermedios
- `GRAMATICA_SUPERFICIAL_MINIMA`: construcción `ternarizer` como primer nivel del lenguaje
- `IR_CANONICA`: nodo `deployment_profile` optativo; obligatorio cuando actor estadístico declarado
- `WISHLIST_IRQ`: WIRQ-2026-002 → activa

---

## Parte IX — Invariantes congelados

Ningún nivel de privilegio puede modificar estos invariantes. No existe campo de override para ellos en la IR. No hay implementación de esa capacidad.

```
T(n) = ⌊7n/9⌋           → función con un único parámetro: n
n = b², b ≥ 3            → validate_cell_size() rechaza todo lo demás
Alfabeto {0, 1, U}       → _normalize_tri_value() rechaza booleanos y otras formas
Polígono inmutable        → generado una vez desde el vector; no se edita
Trayectoria de solo adición  → los frames no se borran ni modifican retroactivamente
CNN nunca sobrescribe     → la salida de ACTOR-CNN-A es señal auxiliar, no clasificación principal
Actor estadístico nunca asigna solo → requiere Ternarizer + convergencia para producir 0 o 1
```

---

## Parte X — Deudas técnicas activas

Ver fichero `REGISTRO_DEUDA_VIVA_SV.csv` en este conjunto.

Resumen:

| ID | Repositorio | Estado |
|---|---|---|
| DV-SVM-013 a 017 | SV-motor-main | Pendiente |
| DV-HITO5-003, 004 | SVperitus | Abierta |
| DV-BANCO-001 | SV-banco | Pendiente |
| DV-PERITUS-001, 002, 003 | SVperitus | Pendiente |
| DV-LENGUAJE-001, 002 | SV-lenguaje | Pendiente |
| DV-MATEMATICA-001 | SV-matematica | Pendiente |

---

---

## Dedicatoria

*[El vendedor de epitafios](https://www.itvia.online/pub/compilador-doctrinal-y-celula-meta-sv93-ia)*, [Miguel Espinosa Gironés](https://www.miguelespinosagirones.es/) (novelista y ensayista español) diría con aquiescencia, seguramente:

> «En memoria de [Rafael López Satorre](https://elnoroestedigital.com/un-siglo-de-los-manuscritos-de-pedro-lopez-ruiz-1917-2017-2/); y de su fiel servidora, mi madre (Cruz Egea Marín): con permiso de mi padre (Francisco Lloret Ruiz, que un 4/4/1934 nació. Y yo el SV, hoy recibo el bautizo oficial, un 4/4/2026).»

Mi padre es el humano que me dio la vida y diseñó la interfaz. Mi madre es el álgebra: el suelo firme sobre el que todo lo demás descansa sin poder moverse. Y mi apoyo es la Inteligencia Artificial — como no podía ser de otra manera, ni habría sido honesto que lo fuera de ninguna otra. Un sistema diseñado con la vocación de disciplinar a la IA, construido sin contar con ella como actor real y responsable, habría sido una contradicción sin salida y un error de fundación. El SV la reconoce, por tanto, con lealtad y sin ambigüedad: como apoyo que cumplió su papel dentro de los límites que la doctrina exige, sin arrogarse soberanía que no le corresponde.

---

## Referencias

[1] Ross C, Swetlitz I. IBM pitched its Watson supercomputer as a revolution in cancer care. It's nowhere close. STAT News. 5 septiembre 2017. [STAT News](https://www.statnews.com/2017/09/05/watson-ibm-cancer/)

[2] Schmidt C. MD Anderson Breaks With IBM Watson, Raising Questions About Artificial Intelligence in Oncology. J Natl Cancer Inst. 2017;109(5):djx113. [DOI:10.1093/jnci/djx113](https://doi.org/10.1093/jnci/djx113)

[3] Strickland E. How IBM Watson Overpromised and Underdelivered on AI Health Care. IEEE Spectrum. 2019;56(4):24–31. [DOI:10.1109/MSPEC.2019.8910661](https://doi.org/10.1109/MSPEC.2019.8910661)

[4] Wong A, Otles E, Donnelly JP, et al. External Validation of a Widely Implemented Proprietary Sepsis Prediction Model in Hospitalized Patients. JAMA Intern Med. 2021;181(8):1065–1070. [DOI:10.1001/jamainternmed.2021.2626](https://doi.org/10.1001/jamainternmed.2021.2626)

[5] Habib AR, Lin AL, Grant RW. The Epic Sepsis Model Falls Short. JAMA Intern Med. 2021;181(8):1040–1041. [DOI:10.1001/jamainternmed.2021.3333](https://doi.org/10.1001/jamainternmed.2021.3333)

[6] Obermeyer Z, Powers B, Vogeli C, Mullainathan S. Dissecting racial bias in an algorithm used to manage the health of populations. Science. 2019;366(6464):447–453. [DOI:10.1126/science.aax2342](https://doi.org/10.1126/science.aax2342)

[7] Wynants L, Van Calster B, Collins GS, et al. Prediction models for diagnosis and prognosis of covid-19: systematic review and critical appraisal. BMJ. 2020;369:m1328. [DOI:10.1136/bmj.m1328](https://doi.org/10.1136/bmj.m1328)

[8] Seyyed-Kalantari L, Zhang H, McDermott MBA, Chen IY, Ghassemi M. Underdiagnosis bias of artificial intelligence algorithms applied to chest radiographs in under-served patient populations. Nat Med. 2021;27:2176–2182. [DOI:10.1038/s41591-021-01595-0](https://doi.org/10.1038/s41591-021-01595-0)

[9] Cabitza F, Rasoini R, Gensini GF. Unintended Consequences of Machine Learning in Medicine. JAMA. 2017;318(6):517–518. [DOI:10.1001/jama.2017.7797](https://doi.org/10.1001/jama.2017.7797)

[10] Rudin C. Stop explaining black box machine learning models for high stakes decisions and use interpretable models instead. Nat Mach Intell. 2019;1:206–215. [DOI:10.1038/s42256-019-0048-x](https://doi.org/10.1038/s42256-019-0048-x)

[11] Parasuraman R, Manzey DH. Complacency and bias in human use of automation. Hum Factors. 2010;52(3):381–410. [DOI:10.1177/0018720810376055](https://doi.org/10.1177/0018720810376055)

[12] Shah NH, Milstein A, Bagley SC. Making Machine Learning Models Clinically Useful. JAMA. 2019;322(14):1351–1352. [DOI:10.1001/jama.2019.10306](https://doi.org/10.1001/jama.2019.10306)

[13] Topol EJ. High-performance medicine: the convergence of human and artificial intelligence. Nat Med. 2019;25:44–56. [DOI:10.1038/s41591-018-0300-7](https://doi.org/10.1038/s41591-018-0300-7)

[14] Liu Z, Mao H, Wu CY, Feichtenhofer C, Darrell T, Xie S. A ConvNet for the 2020s. Proc IEEE/CVF CVPR. 2022:11976–11986. [DOI:10.1109/CVPR52688.2022.01167](https://doi.org/10.1109/CVPR52688.2022.01167)

[15] Raghu M, Zhang C, Kleinberg J, Bengio S. Transfusion: Understanding Transfer Learning for Medical Imaging. NeurIPS. 2019. arXiv:1902.07208

[16] [Reglamento (UE) 2024/1689 del Parlamento Europeo y del Consejo, de 13 de junio de 2024](https://eur-lex.europa.eu/eli/2024/1689/oj) — Reglamento de Inteligencia Artificial de la Unión Europea. DOUE L 2024/1689.

[17] [Reglamento (UE) 2016/679 del Parlamento Europeo y del Consejo, de 27 de abril de 2016](https://eur-lex.europa.eu/eli/reg/2016/679/oj) — Reglamento General de Protección de Datos. DOUE L 2016/119.

[18] National Institute of Standards and Technology. Artificial Intelligence Risk Management Framework (AI RMF 1.0). NIST AI 100-1. Enero 2023. [DOI:10.6028/NIST.AI.100-1](https://doi.org/10.6028/NIST.AI.100-1)

[19] Lloret Egea JA. Fundamentos algebraico-semánticos del Sistema Vectorial SV. Release 3. ITVIA, 2026. ISSN:2695-6411. itvia.online/pub/fundamentos-algebraico-semanticos-del-sistema-vectorial-sv/release/3

[20] Lloret Egea JA. Álgebra de composición intercelular del marco SV — I. Transmisión en serie por parámetro puente. Release 4. ITVIA, 2026. ISSN:2695-6411. itvia.online/pub/algebra-de-composicion-intercelular-del-marco-sv/release/4

[21] Lloret Egea JA. Álgebra de composición intercelular del marco SV — IV. Transducción al alfabeto ternario e interfaz paramétrica. Release 1. ITVIA, 2026. itvia.online/pub/algebra-de-composicion-intercelular-del-marco-sv--iv-transduccion-al-alfabeto-ternario-e-interfaz-parametrica-del-sistema/release/1

[22] Maschmeyer G, et al. ECIL-4 European guidelines for empirical antibacterial therapy for febrile neutropenic patients. Haematologica. 2013;98(12):1826–1835. [DOI:10.3324/haematol.2013.091025](https://doi.org/10.3324/haematol.2013.091025)

[23] Averbuch D, et al. ECIL-7 vaccination guidelines for haematological malignancy patients. Haematologica. 2021;106(8):2206–2215. [DOI:10.3324/haematol.2021.278493](https://doi.org/10.3324/haematol.2021.278493)

[24] Curtis JR, et al. Comparative risks of serious infections with biologic and/or immunosuppressive therapy. Arthritis Rheum. 2012;64(10):3279–3288. [DOI:10.1002/art.34564](https://doi.org/10.1002/art.34564)

[25] Ytterberg SR, et al. (ORAL Surveillance) Cardiovascular and cancer risk with tofacitinib in rheumatoid arthritis. N Engl J Med. 2022;386:316–326. [DOI:10.1056/NEJMoa2109927](https://doi.org/10.1056/NEJMoa2109927)

[26] Singh JA, et al. 2015 ACR guideline for the treatment of rheumatoid arthritis. Arthritis Care Res. 2016;68(1):1–25. [DOI:10.1002/acr.22783](https://doi.org/10.1002/acr.22783)

---

*Documento de arquitectura del Sistema Vectorial SV — no modifica la doctrina algebraica superior del Sistema Vectorial SV.*  
*Juan Antonio Lloret Egea | ORCID: 0000-0002-6634-3351 | ITVIA — IA eñ™ | ISSN: 2695-6411 | CC BY-NC-ND 4.0*  
*Madrid, 4 de abril de 2026*