# Recepción de Claude, corrección del perfil P2 y control de recursos

**RETP-2026-127 · 10/09/2026.** Responsable: Watson. Autoridad de alcance y aceptación: Juan Antonio Lloret Egea.

**Decisión:** recibido `DEFECTO_DE_ESPECIFICACION` sobre IE004-ES-P2/1. La reserva P3 continúa detenida. Se publica la candidata documental **IE004-ES-P2/2**, con corrección causal de los defectos y presupuesto explícito. No hay corrector nuevo ejecutado, nueva ronda de Grok, paridad de /2 ni promoción del aislamiento.

## 1. Corte, fuentes y atribución

Cortes comprobados antes de intervenir:

- Lenguaje, `main`: `584d5cc58ba12404548592ffc350d6829b12daac`.
- Laboratorio, `lab/playground-sv-permanente`: `6f02e5ca38a67082bdc55b6ed5647f3bbaf4e883`.

AGENTS y las rectoras Pilares, perfiles/ensamblaje, transición con sus adendas y arquitectura se han leído y cotejado contra ese corte. Conservan respectivamente los blobs `42221257270146fa94a25e8aca20a2c364c077b7`, `bba81c4ab115899fc4ae812bc7214aab6f39a9bd`, `1df9b818f3f531f628fe0e89dd2c497b5da52d4d`, `44f8fda87856ab16007195cab324d40a758e506a` y `6c57f2e895045d4a26bacefc98c49ebac2b433f7`. Rigen Fase 004 y RETP-121/123–126. Los estados históricos que mencionan Python se leen con la retirada RETP-082, no como autorización vigente de una segunda semántica.

Juan Antonio aporta cuatro archivos y exige cautela con Python, límites operativos independientes del modelo y evitar una lateral de plataformas. Claude firma el dictamen y sus observaciones; sus 30 minutos aproximados proceden del relato humano, no de un perfil de ejecución del núcleo. Watson emite la recepción y las correcciones siguientes. No se atribuyen a Juan Antonio las conclusiones técnicas de Claude ni se presenta la opinión de otra IA como cierre humano.

Los [cuatro originales](ie004/revision-claude-p2-1/DICTAMEN_P2_RETP_2026_126_CLAUDE.md) se conservan sin modificaciones y quedan inventariados en el [manifiesto de recepción](ie004/perfil-es-p2-2/MANIFIESTO.json). Claude declara acceso exclusivamente público, sesión continuada, Python 3.11.15 y participación previa en el diseño; no se acredita independencia del linaje de diseño. No ha creado la reserva según su dictamen. Sus scripts son evidencia histórica de revisión, no dependencias del núcleo, del corrector ni de CI.

## 2. Qué se ha comprobado y qué queda como declaración

| Elemento | Recepción de Watson |
| --- | --- |
| D1: negación de `escriba` | Confirmación documental: G04 representa escritura con operando; G18 y G05 no derivan su negación. No hace falta ejecutar Python para localizar la omisión |
| D2: disyunción y unicidad | Se acepta la insuficiencia de precisión: una raíz sintáctica puede tener varias interpretaciones; /1 no explicitaba su expansión y P2-09 hablaba sólo de raíces. /2 fija unión para disyunción y producto para composición, con contradicción sólo dentro de cada interpretación |
| D3: presupuesto compartido | Se acepta la falta de contabilidad y de orden obligatorio. El agotamiento adversarial descrito es un riesgo permitido por la omisión, no una ejecución demostrada contra Rust. /2 separa A/V y hace independiente la resolución confiable |
| 48/48 con derivación única, 32 871 cadenas, máximo 13 tokens | Resultados declarados por Claude sobre su traducción sintáctica y decisiones léxicas. No reejecutados por Watson; no prueban semántica §5, Rust, memoria, aislamiento ni ausencia universal de ambigüedad |
| «61 sondas» | Discrepancia conservada: `p2_probes.py` contiene **60** literales de entrada. No se identifica una sonda 61 dentro del archivo. No se altera el original ni se inventa la ejecución que falta |
| Exhaustividad del generador | El código usa vocabulario reducido y una cota de longitud; por defecto toma 8, mientras la reproducción indicada usa 6. No contiene un timeout, límite de memoria o de cardinalidad de generación. No se ha ejecutado aquí; no se atribuye su coste a un modelo o a Rust |

La inspección de los tres scripts confirma que `p2_parser.py` cuenta derivaciones sintácticas y no implementa §5. Usa cachés sin límite explícito; CAP=5000 limita el recuento, no los recursos. El generador conserva conjuntos de cadenas, y sus reducciones incluyen opciones sintácticas además de vocabulario: por ejemplo, `Cabeza` deja únicamente `el registro` en su alternativa de registro. Por tanto, «estructura intacta» no debe leerse como exploración de todas las alternativas originales. La ausencia de hallazgos en ese subconjunto no es demostración de unicidad general.

No se ejecuta ninguno de los scripts Python recibidos. Esta recepción usa lectura de texto y comprobaciones documentales de identidades, inventario y enlaces. Tampoco se implementa un sustituto en Node. Un lanzador en otro lenguaje puede invocar un binario Rust, pero el juicio sobre memoria, compilación, ejecución y rendimiento exige evidencia del artefacto Rust pertinente y su entorno.

## 3. Resolución de los tres defectos y los siete reparos

El [perfil completo /2](ie004/perfil-es-p2-2/PERFIL_INTERACCION_ES_IE004_CANDIDATO_2.md) sustituye la candidata /1 a efectos de futura implementación. /1 y sus medidas documentales permanecen intactas y recuperables. Cada regla/certificado incorpora la versión; no se reutiliza Gxx desnudo con otro significado.

| Hallazgo | Cambio o delimitación en /2 | Estatuto |
| --- | --- | --- |
| D1 | G18 representa `no escriba Entero en Grupo`; mantiene el literal como evidencia, sin convertirlo en valor del banco | Corregido documentalmente; ejecución pendiente |
| D2 | Cada raíz devuelve un conjunto semántico; disyunción une, composición combina, contradicción pertenece a una interpretación. La política se consulta tras unicidad | Corregido documentalmente; P2-09 concretado públicamente |
| D3 | Cuenta A para resolución y V para certificado, entradas/arenas/trabajo separados; resolución sin esperar al modelo, orden canónico obligatorio; fallos V no revierten el servicio | Corregido documentalmente; imposición material pendiente |
| R1 | Se eleva a bloqueante por inversión de alcance. `no sólo/solo` deja de derivar como negación; la construcción completa queda no representada, sin rescatar otra cláusula | No se afirma haber comprendido esa construcción; se elimina la interpretación falsa |
| R2 | G25 representa `del caso que estamos viendo` y se comparte en complemento/recorte. No se reescribe el original | Cobertura compositiva corregida |
| R3 | Se admite `de la` Momento con el mismo significado temporal declarado | Asimetría corregida expresamente |
| R4 | Se precisa segmentación y posición de separadores; permanecen en cobertura de bytes, fuera de los símbolos de la gramática | Omisión contractual corregida |
| R5 | Se declara deliberada la tolerancia de puntuación ya existente en G01 | Sin cambio de comportamiento prometido |
| R6 | Primera Consulta, negación y segunda Consulta componen sin prioridad por orden; recorte de objeto positivo también representado | Dos composiciones precisadas, sin gramática de español abierto |
| R7 | La operación bajo negación forma la exclusión; no contamina la positiva. Negación sola sin contradicción produce `SIN_SOLICITUD_POSITIVA` | Precedencia y alcance explícitos |

Los [contrastes públicos](ie004/perfil-es-p2-2/CONTRASTES_PUBLICOS.json) son obligaciones y ejemplos de revisión, no pruebas ejecutadas ni material reservado. Las 48 preguntas previas, las 60 entradas del script recibido, los ejemplos del dictamen y los ejemplos nuevos quedan expuestos; se excluyen de la reserva. La generación reducida declarada tampoco se presenta como fuente de preguntas inéditas.

## 4. Recursos, modelos y Python

La [guía candidata](ie004/perfil-es-p2-2/GUIA_RECURSOS_Y_LATENCIA.md) fija tres escalas distintas: trabajo de revisión/CI, ejecución confiable A y participación del proveedor. GitHub Actions sirve para imponer techos de laboratorio; su máximo de seis horas por trabajo no es un tiempo razonable de consulta interactiva. La guía propone plazos inferiores y una envolvente de consulta, sin declarar que estén alcanzados ni seleccionar un proveedor.

P2 conserva una única futura realización Rust para la corrección, con destinos nativo/WASM. Las pruebas de overflow, préstamos, límites de reserva, panic/trap y ejecución se harán en esa realización cuando corresponda; no se heredan de Python ni de un documento. Las medidas de arranque, tratamiento y proveedor se registran separadas y con artefactos. .NET/FFI siguen pendientes en su puerta; no se reabre su comparación.

La observación de Claude sobre la utilidad del agente exige atención: la candidata hace un análisis confiable completo. No se ha demostrado aportación del modelo ni reducción de coste. Tampoco se deduce de una búsqueda sintáctica finita que sólo pueda aportar mediante normalización previa. Esa nueva figura no se abre aquí. El objetivo de colaboración útil y subordinada sigue pendiente; no se dará por cumplido por omitir el modelo o por lograr un parser determinista.

## 5. Continuación y parada

Se ha atendido una devolución documental con una versión sucesora antes de disponer de reserva. **Siguiente paso:** una revisión focal de las correcciones mediante el [encargo actualizado](ie004/perfil-es-p2-2/ENCARGO_REVISION_Y_RESERVA_P3.md); basta con dictamen de suficiencia o contraejemplo bloqueante. No se repiten enumeraciones generales hasta alcanzar un resultado favorable.

Si la revisión es apta, se conserva el protocolo de reserva de RETP-126: Juan Antonio custodia solicitudes/oráculo fuera del acceso de implementador y participante; Watson recibe sólo dictamen y compromiso. La reserva precede al corrector Rust. La autoría y exposición de quien prepare el oráculo se declaran; su posterior revisión del mismo oráculo no cuenta como auditoría independiente de autoría.

Si vuelve a existir un defecto bloqueante, se detiene la preparación de reserva y se presenta causa y decisión sobre el mecanismo o alcance. No se abre automáticamente /3 ni otra ronda Grok. P3 conserva una sola captura inédita de hasta 24 casos; P4/P5/P6 conservan sus condiciones. Los límites no se aumentan a posteriori para aprobar. La nueva publicación no ejecuta esas etapas.

**Estado:** /1 devuelta con defectos; /2 corregida documentalmente y pendiente de suficiencia; RESERVA_DETENIDA; P3_NO_INICIADO; AISLAMIENTO_NO_VERDE. Código del núcleo, DSL/IR, datos, oráculos, binarios previos, encargos y capturas de Grok permanecen conservados. Catálogo/localización y fila 9 mantienen su orden posterior.
