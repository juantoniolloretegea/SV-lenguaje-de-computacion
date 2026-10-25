# Diccionario del Lenguaje de Computación

**Enciclopedia temática del Sistema Vectorial SV · Volumen I · Primera edición, 25 de octubre de 2026.**

Este diccionario explica el vocabulario del Lenguaje de Computación del SV y relaciona sus términos con las especificaciones que los delimitan. Es una referencia explicativa: no introduce instrucciones, modifica contratos ni acredita capacidades de ejecución. Las versiones de las fuentes y las condiciones de cada distribución conservan su autoridad.

Las equivalencias inglesas facilitan la consulta de bibliografía y especificaciones. La procedencia de una sigla se identifica cuando está documentada; no se atribuyen etimologías históricas sin fuente. Los conceptos físicos, químicos y matemáticos se remiten al tercer volumen cuando dispongan de una definición recibida.

## Consulta temática

| Materia | Términos relacionados |
| --- | --- |
| Escritura e interpretación | [Archivo SVP](#archivo-svp), [DSL](#dsl), [gramática](#gramatica), [perfil lingüístico](#perfil), [representación intermedia](#ir), [semántica](#semantica). |
| Validez y ejecución | [Admisión técnica](#admision), [contrato](#contrato), [diagnóstico](#diagnostico), [invariante](#invariante), [núcleo](#nucleo). |
| Gobierno y recursos | [Árbitro–Director](#director), [Explorador Tensorial](#explorador), [Selector](#selector), [tubería de IA](#tuberia). |
| Evidencia y presentación | [Comprobación](#comprobacion), [frame](#frame), [huella](#huella), [observabilidad](#observabilidad), [trazabilidad](#trazabilidad). |

## Términos

Las entradas están ordenadas alfabéticamente. Cada desplegable conserva una dirección propia para facilitar su cita.

### Admisión técnica {#admision}

Cumplimiento de las condiciones exigidas para aceptar una entrada en una operación determinada. En el Lenguaje SV puede comprender la forma del texto, los tipos y las restricciones de su contrato. Su alcance depende de las comprobaciones efectivamente realizadas.

**Distinción:** admitir una representación no demuestra que su operación se haya ejecutado ni que exista autorización institucional para utilizarla. Véase [Conformidad y diagnósticos](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/conformidad-diagnosticos.html).

**Para continuar la lectura:** [Contrato de una operación](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#contrato) · [Autorización de uso](https://documentos-sv.itvia.online/enciclopedia/ia.html#autorizacion). Comparar las condiciones técnicas con la decisión institucional. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Árbitro–Director {#director}

Componente al que se atribuye el gobierno de una operación: comprueba condiciones, entradas, permisos y límites, y relaciona su ejecución con la evidencia correspondiente. Sus facultades concretas dependen de los contratos implementados y recibidos.

**Relación:** el Selector informa la elección del destino de cálculo a partir de los recursos observados. La presencia de un dispositivo no autoriza su uso. Véase [Árbitro–Director](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/arbitro-director.html).

**Para continuar la lectura:** [Selector de recursos de cálculo](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#selector) · [IA gobernada](https://documentos-sv.itvia.online/enciclopedia/ia.html#gobernada). Relacionar la elección de recursos con las facultades de gobierno. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Archivo SVP {#archivo-svp}

Documento textual que expresa declaraciones y operaciones del Lenguaje de Computación SV conforme a un perfil lingüístico y una gramática identificados. El análisis produce una representación estructurada o un diagnóstico.

**Distinción:** un archivo admitido no equivale a una operación ejecutada. La extensión del archivo tampoco acredita por sí sola su validez. Véase [Ejemplos por construcción](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/ejemplos-svp.html).

**Para continuar la lectura:** [Gramática superficial](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#gramatica) · [Representación intermedia · IR](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#ir). Seguir la relación entre escritura y representación, sin presumir ejecución. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Comprobación {#comprobacion}

Contraste de una condición con criterios y evidencias identificados. Debe declarar objeto, versión, procedimiento, resultado y límites. Una inspección del texto, una prueba ejecutada y una recepción favorable son actos distintos.

**Uso:** «comprobado» siempre requiere precisar qué se comprobó. La ausencia de un fallo observado no acredita propiedades que no fueron examinadas. Véase [Desarrollo y validación del núcleo](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/desarrollo-validacion-nucleo.html).

**Para continuar la lectura:** [Evidencia de evaluación](https://documentos-sv.itvia.online/enciclopedia/ia.html#evidencia) · [Trazabilidad](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#trazabilidad). Examinar qué fundamenta una afirmación y cómo se reconstruye. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Contrato de una operación {#contrato}

Especificación de las entradas, resultados, relaciones, restricciones y condiciones de validez de una operación. Aquí se utiliza en sentido formal y computacional; no designa por sí mismo un acuerdo jurídico.

**En el SV:** una interfaz o un modelo de IA no pueden ampliar las facultades definidas por el contrato. Su aceptación técnica y la autorización institucional se comprueban separadamente. Véase [Arquitectura y entornos de ejecución](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/arquitectura-ejecucion.html).

**Para continuar la lectura:** [Invariante](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#invariante) · [Admisión técnica](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#admision). Identificar qué debe conservarse y qué condiciones permiten admitir una entrada. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Diagnóstico técnico {#diagnostico}

Información que identifica un rechazo, una limitación o una incidencia del procesamiento. Explica qué condición impide continuar o interpretar válidamente el resultado.

**Distinción:** un fallo de ejecución no se transforma automáticamente en un valor U, un dictamen científico ni un diagnóstico clínico. Véase [Conformidad y diagnósticos](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/conformidad-diagnosticos.html).

**Para continuar la lectura:** [Dictamen de evaluación](https://documentos-sv.itvia.online/enciclopedia/ia.html#dictamen) · [Comprobación](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#comprobacion). Distinguir una incidencia técnica de una conclusión de evaluación. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### DSL · Lenguaje específico de dominio {#dsl}

Sigla inglesa de *domain-specific language*. Designa un lenguaje especializado en expresar problemas y operaciones de un ámbito delimitado, frente al propósito general de otros lenguajes. La especialización exige reglas explícitas; no basta una colección informal de instrucciones.

**En el SV:** identifica su lenguaje propio de construcción y representación. Su implementación en Rust se distingue de los programas escritos en SVP. Véanse [Ámbito y organización](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/constructor.html) y la publicación metodológica [L-04](#publicaciones).

**Para continuar la lectura:** [Archivo SVP](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#archivo-svp) · [Agente Especializado del SV](https://documentos-sv.itvia.online/enciclopedia/ia.html#agente). Relacionar el lenguaje especializado con los documentos y construcciones que expresa. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Explorador Tensorial del SV {#explorador}

Componente destinado a identificar recursos y capacidades observables e informar de ellos al Selector. El inventario conserva procedencia, disponibilidad y límites de observación.

**Distinción:** detectar un acelerador no acredita que una operación concreta pueda ejecutarse en él ni que resulte más rápida. Véase [Explorador Tensorial del SV](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/explorador-tensorial-sv.html).

**Para continuar la lectura:** [Selector de recursos de cálculo](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#selector) · [Observabilidad](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#observabilidad). Diferenciar la detección de capacidades de la selección fundamentada. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Frame · Representación estructurada de un suceso {#frame}

Denominación formal utilizada para una representación estructurada de un suceso y de las relaciones requeridas por su contrato. La explicación española acompaña al término de la especificación; no sustituye sus campos ni su identidad.

**Distinción:** no es una captura de pantalla ni una descripción narrativa libre. Su validez exige las relaciones establecidas en la fuente correspondiente. Véase [Parámetros, recorridos y evidencia](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/parametros-rutas.html).

**Para continuar la lectura:** [Contrato de una operación](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#contrato) · [Trazabilidad](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#trazabilidad). Comprender las relaciones exigidas al registro y su reconstrucción posterior. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Gramática superficial {#gramatica}

Conjunto de reglas que determina cómo se escriben las construcciones del lenguaje. Su versión delimita las formas admitidas y sus restricciones expresas.

**Distinción:** la corrección sintáctica no acredita por sí sola la validez semántica, la autorización ni la ejecución. Las precisiones de una versión posterior prevalecen únicamente en lo que sustituyen. Véase [Terminología y referencias del lenguaje](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/referencia-terminologia.html).

**Para continuar la lectura:** [Semántica del lenguaje](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#semantica) · [Representación intermedia · IR](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#ir). Distinguir la forma de escritura, su significado y su representación. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Huella de contenido · SHA-256 {#huella}

Valor calculado mediante una función criptográfica sobre los bytes de un contenido. Permite cotejar la identidad de las copias con una referencia identificada, bajo las propiedades del algoritmo y las condiciones del cotejo.

**Distinción:** la coincidencia de huellas no acredita autoría, permiso de uso, corrección científica ni autenticidad de una referencia no fiable. Véase [Verificación documental](https://documentos-sv.itvia.online/documentacion/reglas-usos-descripciones-documentales/verificacion.html).

**Para continuar la lectura:** [Evidencia de evaluación](https://documentos-sv.itvia.online/enciclopedia/ia.html#evidencia) · [Trazabilidad](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#trazabilidad). Separar identidad de contenido, validez de la evidencia y reconstrucción documental. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Invariante {#invariante}

Propiedad que debe conservarse en las transformaciones u operaciones comprendidas en una especificación. Su contenido y alcance deben estar expresados; la palabra no sustituye la formulación ni la comprobación.

**En el SV:** la construcción de un agente no puede introducir excepciones tácitas a las invariantes matemáticas o semánticas. La demostración denominada seudo-producción tampoco permite modificar el núcleo. Véase [Agentes Especializados y frontera](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/autoridad.html#agentes-especializados-y-frontera).

**Para continuar la lectura:** [Semántica del lenguaje](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#semantica) · [Aislamiento de la IA](https://documentos-sv.itvia.online/enciclopedia/ia.html#aislamiento). Relacionar las propiedades exigidas con las protecciones permanentes. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Núcleo del Lenguaje SV {#nucleo}

Componente que realiza el análisis, las comprobaciones y la representación estructurada del lenguaje, con las funciones implementadas en la revisión correspondiente. Gobierna los contratos semánticos que le competen.

**Límite:** ninguna IA recibe acceso directo ni autoridad para modificarlo. Una descripción arquitectónica no acredita la integración material completa de todas sus protecciones. Véanse [Arquitectura](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/arquitectura-ejecucion.html) y [Desarrollo y validación](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/desarrollo-validacion-nucleo.html).

**Para continuar la lectura:** [Tubería de IA](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#tuberia) · [Aislamiento de la IA](https://documentos-sv.itvia.online/enciclopedia/ia.html#aislamiento). Examinar por qué una aportación de IA no confiere acceso ni autoridad sobre el núcleo. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Observabilidad {#observabilidad}

Capacidad de obtener evidencia sobre actuaciones, resultados, tiempos y recursos mediante puntos de observación identificados. Su cobertura depende de lo que se mida efectivamente.

**Distinción:** observar una petición y su respuesta no permite conocer todo el procesamiento interno de un proveedor. Debe indicarse qué parte permanece fuera de la observación. Véase [Observabilidad y medidas](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/observabilidad.html).

**Para continuar la lectura:** [Trazabilidad](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#trazabilidad) · [Evidencia de evaluación](https://documentos-sv.itvia.online/enciclopedia/ia.html#evidencia). Precisar qué se observa, qué puede reconstruirse y qué queda sin comprobar. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Perfil lingüístico SVP-ES o SVP-EN {#perfil}

Selección explícita de la forma lingüística con la que se expresa un programa SVP. No se deduce del idioma de la interfaz.

**Uso:** una instrucción del perfil inglés conserva su nombre formal cuando se explica su sintaxis; la explicación española precisa su significado. Cambiar la presentación no modifica automáticamente el programa. Véase [Playground del Lenguaje SV](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/playground-sv.html).

**Para continuar la lectura:** [Archivo SVP](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#archivo-svp) · [Gramática superficial](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#gramatica). Situar el idioma del programa dentro de sus reglas de escritura. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Representación intermedia · IR {#ir}

Sigla inglesa de *intermediate representation*. Estructura que conserva los objetos, operaciones y relaciones obtenidos al interpretar el texto y aplicar las comprobaciones correspondientes. En el SV se somete a reglas de bienformación y a una versión identificada.

**Distinción:** representar una operación no equivale a ejecutarla. La interfaz no puede completar relaciones faltantes atribuyéndoles una semántica no recibida. Véase [Terminología y referencias del lenguaje](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/referencia-terminologia.html).

**Para continuar la lectura:** [Semántica del lenguaje](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#semantica) · [Contrato de una operación](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#contrato). Reconocer las relaciones formales que la representación debe respetar. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Selector de recursos de cálculo {#selector}

Componente que elige un destino compatible y autorizado para una carga, utilizando capacidades y medidas disponibles. La denominación completa del SV identifica los dispositivos CPU, GPU, TPU y NPU considerados por su especificación.

**Límite:** no se presume una prioridad universal de un tipo de dispositivo. La elección debe fundamentarse; puede no existir un destino admisible. Véase [Selector CPU/GPU/TPU/NPU](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/selector-arbitro-director.html).

**Para continuar la lectura:** [Explorador Tensorial del SV](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#explorador) · [Árbitro–Director](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#director). Relacionar la información sobre recursos con la decisión autorizada de utilización. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Semántica del lenguaje {#semantica}

Significado atribuido a las construcciones y operaciones por sus definiciones y reglas. Se distingue de la sintaxis, que determina su forma de escritura.

**En el SV:** la presentación visual, el proveedor y el nodo de ejecución no instituyen una semántica alternativa. Una explicación en lenguaje natural debe remitir al contrato que precisa el significado. Véase [Construcciones e instrucciones propias](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/construcciones-y-componentes.html).

**Para continuar la lectura:** [Gramática superficial](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#gramatica) · [Invariante](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#invariante). Comparar forma, significado y propiedades que no pueden alterarse tácitamente. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Trazabilidad {#trazabilidad}

Posibilidad de reconstruir relaciones pertinentes entre fuentes, versiones, actuaciones y resultados mediante registros identificados. Exige conservar el fundamento y las condiciones de lo que se pretende reconstruir.

**Distinción:** disponer de registros no acredita cobertura completa ni autoriza su difusión. Deben declararse las observaciones ausentes y las condiciones de acceso. Véase [Observabilidad y medidas](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/observabilidad.html).

**Para continuar la lectura:** [Frame · Representación estructurada de un suceso](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#frame) · [Evidencia de evaluación](https://documentos-sv.itvia.online/enciclopedia/ia.html#evidencia). Relacionar los registros estructurados con el fundamento de las conclusiones. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

### Tubería de IA {#tuberia}

Denominación específica del SV para el recorrido controlado de recepción y admisión de aportaciones de un modelo. Comprende contratos de frontera y comprobaciones; no designa una conexión directa entre la IA y el núcleo.

**Límite permanente:** las aportaciones se reciben como datos no fiables, sin autoridad para ejecutar órdenes o modificar el sistema. El monitor informativo explica este requisito; su presencia no demuestra la implementación integrada del monitor operativo. Véase [Agentes Especializados y frontera](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/autoridad.html#agentes-especializados-y-frontera).

**Para continuar la lectura:** [Aislamiento de la IA](https://documentos-sv.itvia.online/enciclopedia/ia.html#aislamiento) · [Admisión técnica](https://documentos-sv.itvia.online/enciclopedia/lenguaje.html#admision). Distinguir la separación material de las comprobaciones aplicadas a los datos recibidos. [Explorar otros recorridos](https://documentos-sv.itvia.online/enciclopedia/recorridos.html).

## Publicaciones

Los enlaces de esta sección se abren en una pestaña nueva. Los libros del SV fundamentan su uso específico; la bibliografía externa aporta contexto metodológico y no acredita conformidad del SV. La ausencia de DOI identificado se declara sin inventar un identificador.

| Ref. | Publicación y enlace | Identificación y relación |
| --- | --- | --- |
| L-01 | [Lenguaje de Computación SV: fundamentos, arquitectura y referencia](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/index.html) | Edición consultada el 25/10/2026. Sin DOI identificado. Gramática, IR, componentes, comprobaciones y límites. |
| L-02 | [Dominios y universos del Sistema Vectorial SV - Definiciones y parámetros · V.03](https://documentos-sv.itvia.online/dominios/universos/index.html) | Sin DOI identificado. Alcance de los agentes, Universos y relaciones documentadas. |
| L-03 | [Validación, autorización y puesta en Producción de modelos de Inteligencia Artificial](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/index.html) | Sin DOI identificado. Frontera de IA y condiciones de autorización. |
| L-04 | [When and how to develop domain-specific languages](https://doi.org/10.1145/1118890.1118892) | ACM Computing Surveys, 37(4), 316–344, 2005. DOI: 10.1145/1118890.1118892. Contexto metodológico de los lenguajes específicos de dominio. |

## Casos de uso o ejemplos específicos

### Interpretar un archivo sin atribuirle una ejecución

Una persona presenta un archivo SVP con perfil identificado. El comprobador examina su forma y las relaciones requeridas. Si produce una IR válida, se ha acreditado ese recorrido de interpretación. Para afirmar que una operación fue ejecutada hacen falta además su implementación disponible, la autorización pertinente y la evidencia de ejecución. Intervienen [archivo SVP](#archivo-svp), [admisión](#admision), [IR](#ir) y [comprobación](#comprobacion).

### Elegir recursos con evidencia

Un inventario identifica CPU y GPU. El Explorador informa de lo observado y el Selector examina compatibilidad, autorización y medidas pertinentes. La mera presencia de la GPU no basta para seleccionarla. Si no existen medidas suficientes, se declara esa limitación. Este ejemplo explica responsabilidades; no atribuye un rendimiento ni una elección a un equipo concreto.

### Rechazar una instrucción ajena al contrato

Una respuesta del modelo contiene texto que pretende cambiar permisos. Su recepción no le otorga autoridad. La tubería mantiene el contenido fuera del núcleo y la operación competente sólo puede admitir datos conformes a su contrato. El ejemplo expone el requisito de aislamiento; su acreditación exige pruebas materiales de la integración utilizada.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
