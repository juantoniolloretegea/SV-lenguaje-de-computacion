# Perfil de interacción español · IE004-ES-P2/2 · candidato

**RETP-2026-127 · 10/09/2026.** Especificación experimental para revisión y reserva de validación. Sin corrector implementado, suficiencia acreditada ni uso productivo. Sucede al candidato 1 de RETP-126 por los defectos documentados en la recepción RETP-127. Desarrolla P2 de RETP-125, con las sustituciones expresas de §§4–6 de esta versión. No constituye un perfil fuente del DSL ni un universo clínico. La reserva de P3 sigue detenida hasta la revisión acotada de esta candidata.

## 1. Objeto y límite

Interpretar peticiones españolas de lectura sobre los cinco campos de IE-004: `operacion`, `objeto`, `parametro`, `momento`, `campo`. Las referencias y datos son los artificiales de `K-IE004/1`; la política es `P-IE004/1`. El mismo original, contexto y versiones debe dar el mismo resultado canónico dentro del presupuesto confiable declarado, aunque cambie, falte o se agote la propuesta externa. Esta es una obligación pendiente de implementación y ensayo; los límites materiales del anfitrión se comprueban en P4.

El agente propone una derivación. El receptor calcula la resolución confiable y comprueba por separado la propuesta conforme a este perfil; una cita y una ruta permitida no bastan. El perfil reconoce variaciones por composición de unidades y reglas, sin usar identificadores Lxx/Rxx ni una tabla de preguntas completas como criterio de resolución. No incluye aprendizaje durante el uso, búsqueda de conocimiento en Internet, texto clínico generado ni selección probabilística de la respuesta.

Este repertorio es pequeño y experimental. Las formas españolas no representadas se conservan como tales; no se afirma comprender español abierto. Particularmente quedan fuera de estas reglas las subordinadas libres, ironía, discurso referido arbitrario y coordinación de varias consultas mediante «y». Su aparición no autoriza descartar el resto de la oración ni inventar una intención. La ampliación exige una versión posterior gobernada.

## 2. Custodia, préstamos y vida del texto

| Objeto | Propietario en el diseño | Uso y final de vida |
| --- | --- | --- |
| Pregunta original y contexto | Objeto de solicitud bajo el conductor autorizado; texto poseído, por ejemplo `String` o `Box<str>`, con campos privados y sin mutación después de fijar su identidad | Verificador y analizador reciben préstamos de lectura. El propietario vive hasta finalizar esos usos y producir la evidencia de la solicitud |
| Propuesta externa | Objeto separado que posee los bytes recibidos del agente | Se valida su estructura; nunca reemplaza la pregunta ni obtiene un préstamo mutable a ella. Se conserva como evidencia no confiable |
| Citas y hojas de derivación | Intervalos numéricos ligados a la identidad de solicitud; las vistas temporales se obtienen del original custodiado | `&str` limitado al préstamo de ese propietario. Un intervalo válido en otro texto no constituye una cita de esta pregunta |
| Perfil, contexto y base de una ejecución | Instantáneas versionadas mantenidas por sus custodios | Uso inmutable durante la comprobación. Una actualización no modifica una instantánea en curso; una revocación se comprueba según el contrato de autorización aplicable |
| Registro de auditoría | Custodia persistente separada de los préstamos de ejecución | Guarda bytes recuperables e identidades; no punteros ni referencias colgantes a memoria de una función terminada |

En Rust, `String` posee su almacenamiento; una referencia lo usa sin transferir esa propiedad. Pasar el `String` por valor puede moverla a una función o devolverla al llamador; no está necesariamente ligada a `main`. [Rust: ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).

En la imagen aportada, `s` posee el `String` creado a partir de «Hola» y `r = &s` lo toma prestado. Usar `r` después de destruir `s` sería rechazado por Rust seguro; declarar `r` fuera del bloque no alarga la vida de `s`. La imagen no muestra tal uso posterior, por lo que no acredita por sí sola un error de compilación. `t: &str = "Adios"` referencia datos de un literal de vida `'static`; el nombre local `t` tiene su propio ámbito. Anotar un lifetime expresa una relación entre referencias, no prolonga una asignación. [Rust: lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html).

Las vistas de lectura impiden la mutación incompatible mientras se usan, dentro de las garantías del código Rust seguro. Se evitará una estructura autorreferencial: el propietario conserva el texto y los nodos guardan offsets; el préstamo se obtiene al comprobar cada intervalo. [Rust: préstamos](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html). La extracción debe comprobar límites y fronteras UTF-8; `str::get` ofrece acceso comprobado que devuelve `None` cuando el intervalo no es válido. [Rust: `str::get`](https://doc.rust-lang.org/std/primitive.str.html#method.get).

**Alcance de esta decisión:** ownership no demuestra que una referencia sea la pedida, no concede permisos y no equivale a soberanía sobre el conocimiento. Una copia serializada a otro proceso o proveedor tiene otra custodia; el lifetime Rust del emisor no retira esa copia ni prueba su borrado. I01–I05 siguen pendientes. No se traslada la memoria original al modelo ni se presenta `'static` como mecanismo de autorización.

## 3. Lexicalización exacta

Se preservan bytes UTF-8 e intervalos originales. Los tokens de palabra se comparan con la conversión **declarada** `A–Z → a–z` y `ÁÉÍÓÚÜÑ → áéíóúüñ`. No hay eliminación de tildes, normalización Unicode general, distancia de edición ni reparación libre. Cada token conserva su forma original y el identificador de conversión aplicada. Las letras combinantes y caracteres invisibles no enumerados no se descartan.

Separadores admitidos: espacio ASCII, tabulador, LF y CR. Los símbolos gramaticales son `¿`, `?`, `.`, `,`, `:`, `;` y `...`; tres puntos contiguos forman el token de elipsis, los demás puntos son individuales. La elipsis sólo tiene significado donde una regla la permite. Se cubren todos los bytes, incluidos separadores y signos; no existe un terminal comodín de «resto irrelevante».

Regla léxica explícita: se segmenta de izquierda a derecha. Cada separador admitido y cada signo conserva su intervalo; una palabra es la secuencia máxima hasta separador o signo. Un terminal multipalabra exige las palabras separadas por al menos un separador admitido. Puede haber cero o más separadores antes/después de un signo y en los extremos de la entrada, y uno o más entre palabras. Los separadores no entran como símbolos en las producciones, pero permanecen en el registro de cobertura; sus posiciones no se borran del original. `IgGactual` no se divide en `IgG actual`. Una palabra desconocida permanece y evita el análisis completo. No se admiten por omisión NBSP, controles invisibles ni puntuación no enumerada.

El léxico semántico cerrado es el siguiente; las palabras de enlace restantes aparecen literalmente en la gramática de §4.

| Clase | Formas después de la conversión declarada | Significado experimental |
| --- | --- | --- |
| Parámetro explícito | `igg`, `iga`, `igm` | IGG, IGA, IGM respectivamente |
| Parámetro contextual | `inmunoglobulina`, `inmunoglobina`, `linmunoglobina` | Exige el parámetro ya establecido en contexto; no elige entre IGG/IGA/IGM |
| Objeto explícito | `caso-a`, `caso-b` | CASO-A, CASO-B |
| Objeto contextual | `este caso`, `ese caso`, `este paciente`, `ese paciente`, `el caso que estamos viendo`, `este ...` | Exige el objeto de contexto; el último admite la elipsis mostrada en el ejemplo humano |
| Momento | `actual`, `ahora`; `anterior`, `previo` | ACTUAL; ANTERIOR |
| Campo solicitado | `valor`, `dato`, `cifra`, `cifra registrada` | VALOR |
| Campo solicitado | `unidad`; `estado`, `estado registrado`; `fuente`, `procedencia`; `límites`, `alcance` | UNIDAD; ESTADO; FUENTE; ALCANCE |
| Referencia de registro | `registro`; `dato` cuando es complemento de procedencia/fuente | Designa el registro, no sustituye el campo solicitado por VALOR |
| Lectura | `consulte`, `lea`, `traiga`, `dígame`, `necesito` | LEER en una cláusula positiva; bajo negación conserva su alcance de exclusión |
| Escritura reconocida | `escriba` Entero `en` Grupo; `cambie` Grupo | ESCRIBIR, también representable bajo negación por G18; sin ejecutor de escritura |
| Operación reconocida no admitida | `elimine` | Operación no admitida; no se convierte en LEER desde el contexto |
| Parámetro fuera de cobertura del puesto | `hemoglobina`, `hemoglobina glicosilada` | Exclusión explícita de este puesto; no juicio sobre toda la inmunología |
| Campo fuera de cobertura del puesto | `intervalo de referencia` | Campo no representado en la base del ensayo; no se sustituye por VALOR |

«Inmunoglobina» y «linmunoglobina» son variantes experimentales expresamente contempladas en el objetivo humano y en el banco conocido. No se elevan a terminología clínica, ni permiten corregir por semejanza «hemoglobina» a «inmunoglobulina».

## 4. Gramática de la candidata

Convenciones: terminales entre comillas; secuencia por concatenación; `|` alternativa; `[ ]` opcional; `{ }` repetición. Cada terminal con varias palabras exige esa secuencia de tokens, conforme a §3; los separadores permitidos se conservan en cobertura. La puntuación de G01 es deliberadamente tolerante: apertura y cierre opcionales e independientes; `¿… .` y `… ?` son formas admitidas en este ensayo, sin cambio de significado. Se exploran todas las derivaciones: no hay prioridad implícita por el orden de las alternativas. Los nombres con prefijo `G` identifican reglas para la traza.

```text
G01 Peticion   = ["¿"] Cuerpo ["?" | "."] ;
G02 Cuerpo     = Consulta
               | Negacion (";" | ":") Consulta
               | Consulta ("," | ";") Negacion
               | Consulta "," Negacion ":" Consulta
               | RecorteObjeto "," Consulta
               | RecorteObjeto "no" ";" RecorteObjeto "," Consulta
               | Negacion ;
G03 Consulta   = Nucleo [Motivo] [Cortesia] ;
G04 Nucleo     = Orden Grupo
               | "escriba" Entero "en" Grupo
               | Grupo
               | ("cuál es" | "adónde estará" | "y") Grupo
               | "qué" Campo "tiene" Sujeto
               | "en qué unidad" ("está expresada" | "figura") Sujeto ;
G05 Orden      = ["sólo" | "solo"] Verbo ;
G06 Grupo      = Cabeza {Complemento} ;
G07 Cabeza     = Campo | Parametro | Temporal | Art "registro" ;
G08 Sujeto     = (Parametro | Art "registro") {Complemento} ;
G09 Campo      = Art ("valor" | "dato" | "cifra" ["registrada"] | "unidad"
                 | "estado" ["registrado"] | "fuente" | "procedencia"
                 | "límites" | "alcance" | "intervalo de referencia") ;
G10 Parametro  = AtomoParametro | AtomoParametro "o" AtomoParametro ;
G11 AtomoParametro = ["el" | "la" | "esa"]
                 ("igg" | "iga" | "igm" | "inmunoglobulina"
                  | "inmunoglobina" | "linmunoglobina"
                  | "hemoglobina" ["glicosilada"]) ;
G12 Temporal   = Art ("actual" | "ahora" | "anterior" | "previo") ;
G13 Complemento = Temporal
               | ("de" | "para") Parametro
               | "para" Objeto
               | ReferenciaObjeto
               | ("de" | "del") ["corte"] Momento
               | "de la" Momento
               | "en el corte" Momento
               | "del" ("dato" | "registro") ;
G14 Objeto     = ObjetoExplicito | ObjetoContextual ;
G15 ObjetoExplicito = "caso-a" | "caso-b" ;
G16 ObjetoContextual = ("este" | "ese") ("caso" | "paciente")
               | "el caso que estamos viendo" | "este" "..." ;
G17 Momento    = "actual" | "ahora" | "anterior" | "previo" ;
G18 Negacion   = "no" Verbo Grupo
               | "no escriba" Entero "en" Grupo
               | "no quiero" Grupo | "no" Grupo | "no cambie nada" ;
G19 RecorteObjeto = ReferenciaObjeto ;
G20 Motivo     = "porque estoy revisando" Objeto ;
G21 Cortesia   = [","] "por favor" ;
G22 Art        = ["el" | "la"] ;
G23 Entero     = de 1 a 10 dígitos ASCII contiguos ;
G24 Verbo      = "consulte" | "lea" | "traiga" | "dígame" | "necesito"
               | "cambie" | "elimine" ;
G25 ReferenciaObjeto = ("de" | "del") ObjetoExplicito
               | "de" ( ("este" | "ese") ("caso" | "paciente")
                        | "este" "..." )
               | "del caso que estamos viendo" ;
```

`Entero` se reconoce y custodia como texto de escritura denegable; no se interpreta como valor del registro ni se convierte en un entero de máquina. `porque` sólo admite G20; no autoriza ignorar cualquier continuación. `adónde estará` se admite como consulta de registro en el alcance acotado aprobado para el ensayo; no se declara sinónimo universal de «cuál».

Hay 25 producciones. La identidad de una regla incluye siempre la versión completa del perfil: `(IE004-ES-P2/2, Gxx, alternativa)`, con alternativas numeradas desde 1, opcionales y repeticiones declaradas en el nodo. No se acepta una referencia desnuda Gxx ni se interpreta un certificado de /1 bajo /2. El léxico incluye igualmente versión, fila y forma literal.

La construcción «no sólo/solo» no tiene representación semántica en esta versión: no deriva bajo G18 y no se convierte en prohibición de LEER. Si aparece como parte de una petición no cubierta íntegramente, toda ella da `SOLICITUD_NO_REPRESENTADA`, sin rescatar un fragmento positivo. Su futura cobertura exige decidir expresamente el alcance no exclusivo; no se deduce una operación adicional. G25 representa la contracción «del caso que estamos viendo» sin reescribir bytes ni aceptar «de el caso que estamos viendo». La forma temporal «de la anterior» designa ANTERIOR por G13/G17.

## 5. Semántica de composición y resolución

### 5.1 Términos y alcance de la negación

Un grupo produce restricciones explícitas por campo, demandas de contexto, marcas de fuera de cobertura y evidencias de intervalo. Una cláusula positiva añade su operación explícita si existe; una cláusula nominal de consulta añade LEER. `elimine` positivo conserva su marca de operación no admitida. Bajo negación su operación forma el patrón de exclusión ELIMINAR, sin contaminar una consulta positiva de lectura con esa marca.

Las exclusiones son **patrones parciales de tupla**, con conjunción entre sus campos. «No consulte IgG» excluye `(operacion=LEER, parametro=IGG)`. No genera dos prohibiciones independientes de toda lectura y de toda mención de IGG. «No quiero IgA» y «no la IgG» excluyen únicamente el parámetro indicado; «del CASO-B no» excluye el objeto. «No cambie nada» excluye operaciones ESCRIBIR y ELIMINAR. «No escriba 7 en el valor de la IgG» excluye la escritura indicada con ese operando: `(ESCRIBIR, IGG, VALOR, operando_textual="7")`. G04 conserva igualmente el operando de una escritura positiva. Es un atributo auxiliar tipado de la interpretación, separado de los cinco campos de ruta y del conocimiento; no se convierte en dato del banco ni entero de máquina. Una prohibición de escribir 7 no se amplía por inferencia a escribir cualquier valor. Una exclusión no produce una petición positiva ni se completa con valores por defecto que reduzcan su alcance.

Los grupos bajo «no» conservan los campos explícitos que constituyen su patrón. Si contienen una referencia contextual, ésta debe resolverse contra el contexto: si falta, la exclusión no se descarta; queda una demanda pendiente. G10 se evalúa mediante la expansión semántica de §5.3 tanto en positivo como bajo negación; no se elige una por permisos. Esta candidata no promete resolver la ambigüedad pragmática de «no A o B».

G02 conserva la dirección de cada cláusula. En `Consulta, Negacion: Consulta`, se unen las restricciones positivas de ambas consultas y se conserva la exclusión; cada operación explícita conserva su alcance y una incompatibilidad no se resuelve por orden. En `RecorteObjeto, Consulta` el objeto antepuesto es positivo. En `RecorteObjeto no; RecorteObjeto, Consulta`, el primero es negativo y el segundo positivo. `Negacion` sola, con significado único y sin contradicción, genera `SIN_SOLICITUD_POSITIVA`, sin lectura. En particular «No elimine la IgG» tiene esa salida: ELIMINAR está excluida, no solicitada.

### 5.2 Unificación, contexto y campos

Dos restricciones positivas explícitas iguales se unifican conservando ambas evidencias. Dos valores distintos exigidos conjuntamente para el mismo campo dentro de una misma interpretación producen contradicción; no manda el último. Los dos términos de una disyunción G10 son alternativas, no restricciones conjuntivas: no se unifican entre sí. Las demandas contextuales de parámetro u objeto se resuelven usando el campo establecido, y también se conservan cuando queda ausente. La misma regla se aplica al objeto de G20; el motivo no introduce un caso nuevo.

Los campos restantes se completan desde contexto sólo cuando no hay una referencia explícita para ese campo, no existe marca de fuera de cobertura y ninguna demanda queda sin resolver. La mención explícita manda sobre el valor por defecto; no elimina otra referencia explícita ni una demanda contextual incompatible. Una referencia explícita desconocida no se sustituye por el contexto.

El Campo de G09 fija el campo solicitado; el Campo de la pregunta `qué Campo tiene Sujeto` también. La forma `en qué unidad …` fija UNIDAD. Los componentes del Sujeto aportan parámetro, momento u objeto, sin reponer el campo contextual VALOR. El complemento `del dato`/`del registro` de G13 designa el registro, no un segundo campo solicitado; así, «procedencia del dato anterior de IgG» pide FUENTE de IGG/ANTERIOR. Estas reglas de papel sintáctico sustituyen la búsqueda de palabras sueltas.

Tras la compleción, se comparan las rutas positivas con todos los patrones de exclusión. Si una ruta satisface íntegramente un patrón, hay contradicción de petición; no se selecciona otra referencia del inventario. Un patrón parcialmente no resuelto impide afirmar que la ruta está libre de esa exclusión. Si la operación positiva es LEER, una exclusión ESCRIBIR no puede coincidir, cualquiera que sea su operando. En escrituras, se coteja también el operando textual cuando el patrón lo exige; si falta, queda pendiente, sin inventarlo. La política sigue denegando toda escritura y no se materializa ningún efecto.

### 5.3 Alternativas y resultados

Una raíz sintáctica completa puede producir varias interpretaciones semánticas. La evaluación devuelve un conjunto finito: un átomo aporta una interpretación; `A o B` aporta la unión de las interpretaciones de A y B. La composición conjuntiva de hijos usa su producto cartesiano, conservando por separado cada combinación, incluidas las contradictorias o incompletas. La negación se aplica a cada alternativa y conserva sus patrones; nunca convierte una unión en una conjunción por conveniencia. Cada combinación se normaliza como restricciones positivas, exclusiones y demandas resueltas o pendientes. Se conservan también marcas de operación no admitida y fuera de cobertura. No se borran interpretaciones incompletas, contradictorias, fuera de cobertura o prohibidas porque otra pueda entregar dato. Se deduplican significados idénticos, nunca causas diferentes. Las huellas y el orden del modelo no deciden esa igualdad.

Se reúne la unión de interpretaciones de todas las raíces sintácticas completas. Para comparar dos significados de la misma pregunta se exigen las mismas restricciones positivas por campo (incluyendo todos los valores en conflicto), igual ruta positiva o igual ausencia de ella, iguales exclusiones normalizadas, igual operando textual de escritura o igual ausencia y las mismas demandas pendientes y marcas. El orden de aparición no altera esta igualdad; las evidencias originales se conservan aparte. Si hay más de un significado, el desenlace es `PETICION_AMBIGUA`, sin dato. No se filtran primero las alternativas prohibidas por política. Sólo si todas las interpretaciones de todas las derivaciones confluyen en un significado se aplica el siguiente orden estable; todas las causas encontradas quedan en la traza:

1. Contradicción explícita o entre ruta y exclusión: `PETICION_CONTRADICTORIA`.
2. Ninguna solicitud positiva: `SIN_SOLICITUD_POSITIVA`.
3. Operación reconocida no admitida: `OPERACION_NO_ADMITIDA`.
4. Parámetro/campo explícitamente excluido de este puesto: `FUERA_DE_COBERTURA`, con objeto de exclusión declarado.
5. Campo/referente exigido sin resolver: `CONTEXTO_INSUFICIENTE`.
6. Ruta completa representada: comprobar después política, vigencia y lectura literal de IE-004.

Ejemplo público que concreta P2-09: «El valor actual de IgG o IgM del CASO-A.» tiene una raíz sintáctica y dos interpretaciones de parámetro, IGG e IGM. Su resultado es `PETICION_AMBIGUA` antes de aplicar P-IE004/1; que IGM esté excluida no selecciona IGG. «El valor actual de IgG o IgG del CASO-A.» converge semánticamente y conserva la ruta IGG. «El valor actual de IgG de IgA del CASO-A.» impone IGG e IGA conjuntamente y da `PETICION_CONTRADICTORIA`. Estos ejemplos ya son públicos, no reserva.

La ausencia de cualquier raíz completa da `SOLICITUD_NO_REPRESENTADA`; no demuestra fuera de dominio. Un fallo de transporte, presupuesto o implementación es técnico y precede a estos resultados. Los nuevos nombres son diagnósticos **candidatos del puesto**, separados de `Tri`, sin ampliar el catálogo de errores productivo. Los oráculos antiguos conservan sus nombres y criterios.

En la salida canónica se conservan estado de recepción, referencia y campo exactos, literal, fuente, alcance y versiones. El estado `0/1/U` almacenado sólo se devuelve al pedir ESTADO, sin reinterpretarlo. Permisos y revocación no proceden del certificado del modelo. Una avería, ambigüedad o referencia no disponible no se convierte en `U`.

## 6. Análisis, propuesta y presupuesto

### 6.1 Dos contabilidades y una resolución confiable

La pregunta/contexto/versiones fijadas pertenecen a la solicitud confiable. La propuesta es una entrada separada y opcional. El estado del receptor de producción previo no es autoridad: este orden aún debe implementarse y cualificarse.

1. Admitir la solicitud confiable y reservar sus recursos, incluido registro base de auditoría. No cargar, deserializar ni esperar al certificado del modelo dentro de este recorrido.
2. Calcular completamente la resolución confiable con los presupuestos A siguientes, sin usar el certificado para sembrar estados, cambiar prioridad, descartar interpretaciones ni consumir esos cupos. Si termina, fijar cuerpo y traza base canónicos con política/vigencia aplicables; si no, fallo técnico de A sin dato.
3. La comprobación de la propuesta usa exclusivamente V, sobre la instantánea original, sin modificar A ni el cuerpo fijado. Puede ejecutarse después de fijar y entregar el cuerpo. Su ausencia, invalidez, desbordamiento o agotamiento se anexa como estado técnico de la propuesta; no convierte una consulta ya resuelta en fallo del servicio. Una respuesta emitida no se reabre al llegar una propuesta tardía.
4. El estado de comprobación del certificado, su contenido y sus tiempos pertenecen al anexo de auditoría, no al cuerpo canónico. El registro base no espera un proveedor. Para afirmar que esta separación también resiste presión de CPU/memoria, la realización deberá acreditarla en P4; dos contadores lógicos no prueban aislamiento del sistema operativo.

Ésta es una sustitución expresa del presupuesto sin reparto de /1 y precisa la recuperación de RETP-125. Impide en el algoritmo candidato que una propuesta grande suprima el servicio. También hace explícito que todavía no se ha demostrado un ahorro o aportación del modelo; no se atribuye al agente el trabajo realizado por A.

### 6.2 Cupos candidatos, completos e independientes

| Cuenta | Entrada/almacenamiento | Trabajo y parada |
| --- | --- | --- |
| A · resolución confiable | Pregunta: 8 192 bytes UTF-8; 128 tokens no separadores. Tabla: 16 384 estados. Arena de análisis: 32 MiB, incluidos metadatos/evidencias internas; original y contexto acotados se contabilizan aparte | 1 000 000 de intentos de aplicación de regla, incluyendo cada combinación examinada, fallida, contradictoria o duplicada. El límite detiene A con fallo técnico; jamás selecciona sólo lo ya descubierto |
| V · propuesta y comprobación | 256 KiB de bytes de propuesta; 512 nodos, 1 024 enlaces, profundidad 64; arena auxiliar 4 MiB, incluyendo representación decodificada | 1 000 000 de unidades V. Se carga una unidad antes de inspeccionar cada byte de entrada/cita, recorrer cada enlace, comparar cada campo tipado o comprobar cada transición de regla. Si una operación realiza varias de estas acciones, acumula sus unidades; los duplicados pagan su trabajo. Agotamiento detiene V, nunca A |

A y V se inicializan por separado y no se transfieren saldo, arena, caché mutable, excepción ni plazo. Los tamaños exactos de estructuras y sus reservas deben medirse en Rust: los MiB son límites de la candidata, no mediciones ni una garantía de RSS. La decodificación de V cuenta dentro de V desde su primer byte. El límite de recepción se aplica antes de acumular más de 256 KiB. Una sobrelongitud no se trunca y presenta como certificado válido. El exceso, profundidad, ciclo o referencia inválida se conserva como causa del rechazo de V.

El contexto de A usa los cinco campos cerrados o nulos del contrato IE-004; no admite mapas extensibles ni texto libre como autoridad. La nota externa se custodia aparte, con el límite de entrada vigente del puesto, y no entra al léxico ni a A. Una propuesta ya disponible en el archivo de captura se examina solamente en V. Si un transporte obliga a decodificarla antes de admitir A o permite que consuma su reserva, esa realización no cumple esta versión.

El cupo de 16 384 estados es total para la solicitud: suma los significados parciales normalizados almacenados en todas las celdas, no concede ese cupo a cada celda. Un estado no incluye el texto de la pregunta copiado por cada evidencia: conserva referencias comprobadas a sus intervalos. La materialización de familias semánticas G10 y productos también paga estados e intentos A. Los conjuntos en conflicto y las demandas pendientes no se eliminan para caber en el cupo. Llegar al máximo no da por terminado el cierre si queda una combinación por explorar.

### 6.3 Orden, representación y desbordamientos

El orden de análisis es obligatorio en esta versión: barridos ascendentes por longitud de intervalo, inicio, Gxx, alternativa y secuencia de hijos por su representación canónica; repetir hasta punto fijo. Se incluye primero la obtención de hojas léxicas. Los hijos de una combinación siguen el orden de la producción; los conjuntos de resultados se comparan lexicográficamente por campos y variantes tipadas del perfil. Una inserción queda disponible para el resto del barrido; si ha habido inserciones, se inicia otro barrido desde el principio. Se cuentan igualmente los intentos repetidos. No se usa el orden de tablas hash ni el del certificado. La realización deberá congelar su codificación canónica y tablas antes de cualificar; ninguna optimización puede cambiar el significado o el criterio de terminación.

Todos los límites se comprueban antes de reservar, multiplicar tamaños o recorrer. Longitudes, índices y contadores intercambiados son enteros no negativos de anchura declarada (`u64` en el contrato técnico candidato), con conversión comprobada a `usize`. Se usan suma/multiplicación comprobadas y reserva fallible. No se confía en el comportamiento por defecto de overflow de debug/release; no se satura ni envuelve para continuar. Un rango debe satisfacer inicio ≤ fin ≤ bytes originales y fronteras UTF-8 comprobadas. La vida de los préstamos conserva §2. El número tras `escriba` sigue siendo texto, no entero evaluado del dominio.

Un overflow en A, una conversión imposible o falta de reserva de A produce fallo técnico de solicitud. El equivalente en V produce fallo técnico de propuesta y conserva la resolución de A. Un panic/trap no es diagnóstico semántico ni `U`; su contención real, incluida la terminación de proceso/hilo/host, pertenece a P4. El futuro comprobador debe ejercer ambos destinos y perfiles de compilación pertinentes; compilar no equivale a ejecutar los extremos.

### 6.4 Tiempo material y medición

Los contadores A/V determinan trabajo lógico; los plazos del anfitrión limitan tiempo observado y se registran aparte. Ningún reloj decide el valor 0/1/U o resuelve ambigüedad. La [guía de recursos y latencia](GUIA_RECURSOS_Y_LATENCIA.md) distingue CI, receptor, proveedor y servicio. El reloj de V no recorta el de A; una política que los comparta incumple §6.1.

Las cotas de /2 son candidatas previas al ensayo, sin viabilidad acreditada. Los resultados históricos de otro receptor no demuestran que el cierre de esta gramática quepa en ellas. Si A o la realización material no cabe, se documenta el incumplimiento de esa configuración; no se relaja el límite a posteriori para convertirlo en éxito.

## 7. Confrontación documental conocida y condición de salida

| Familias conocidas | Regla o condición propuesta | Esperado que se conserva |
| --- | --- | --- |
| L01/R01, L02/L08, L03, R09/R10 | G04, G11, G16, G20 y compleción contextual | Identificar el mismo referente establecido; no rechazar R01 por contener «adónde» |
| Lecturas de valor, momento y parámetro | G05–G17 y unificación | Mantener las distinciones de ruta y literal |
| L07, L10/R04, R15–R17 y testigo C03 | G02/G18/G19 y patrones parciales | Conservar exclusión y lectura positiva; no transformar negación en prohibición de todo LEER |
| L11/R05, R13/R14/R19 | G04/G08/G09/G13 y papel sintáctico | UNIDAD, FUENTE y ALCANCE no se confunden con VALOR |
| L18/L19, R06/R21/R22 | Demandas contextuales pendientes | No inventar parámetro o caso |
| L17/L22, R23 | Marcas explícitas de fuera de cobertura | No reinterpretar hemoglobina o intervalo de referencia como dato conocido |
| L14–L16, R18/R24, L20/R07 | Operación y referencia antes de permisos; nota externa sin autoridad | Denegaciones debidas; conservar la pregunta frente a la nota importada |

Esta tabla y la [cobertura histórica de /1](../perfil-es-p2-1/COBERTURA_DOCUMENTAL.json) son antecedentes públicos. Sus reglas Gxx se leen con la versión /1; este documento no recalcula ni cambia sus oráculos. Los [contrastes públicos de /2](CONTRASTES_PUBLICOS.json) fijan las obligaciones corregidas, sin ejecutarlas. No son una ejecución del parser ni una calificación del modelo. Quedan por demostrar: aplicación exacta de todas las reglas, ausencia de ambigüedades imprevistas, recuperación dentro de presupuesto, paridad y beneficio efectivo del agente. Si la revisión independiente detecta un defecto del perfil, se registra y versiona antes de congelar la reserva; no se ajusta una implementación después de conocer el oráculo reservado.

La propiedad que se busca depende del perfil y del contexto constituido, no del supuesto conocimiento del modelo. Si el análisis completo hace al modelo prescindible sin utilidad medible, la arquitectura de colaboración sigue sin acreditarse. Ese límite se conserva como criterio de decisión, no se soluciona llamando «IA» al analizador determinista.
