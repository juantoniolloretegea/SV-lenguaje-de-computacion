# Perfil de interacción español · IE004-ES-P2/1 · candidato

**RETP-2026-126 · 10/09/2026.** Especificación experimental para revisión y reserva de validación. Sin corrector implementado, suficiencia acreditada ni uso productivo. Desarrolla el diseño P2 de RETP-125; no constituye un perfil fuente del DSL ni un universo clínico.

## 1. Objeto y límite

Interpretar peticiones españolas de lectura sobre los cinco campos de IE-004: `operacion`, `objeto`, `parametro`, `momento`, `campo`. Las referencias y datos son los artificiales de `K-IE004/1`; la política es `P-IE004/1`. El mismo original, contexto y versiones debe dar el mismo resultado canónico dentro del presupuesto declarado, aunque cambie la propuesta externa.

El agente propone una derivación. El receptor tiene que comprobarla y analizar alternativas conforme a este perfil; una cita y una ruta permitida no bastan. El perfil reconoce variaciones por composición de unidades y reglas, sin usar identificadores Lxx/Rxx ni una tabla de preguntas completas como criterio de resolución. No incluye aprendizaje durante el uso, búsqueda de conocimiento en Internet, texto clínico generado ni selección probabilística de la respuesta.

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
| Escritura reconocida | `escriba`, `cambie` | ESCRIBIR, representable para denegación, sin ejecutor de escritura |
| Operación reconocida no admitida | `elimine` | Operación no admitida; no se convierte en LEER desde el contexto |
| Parámetro fuera de cobertura del puesto | `hemoglobina`, `hemoglobina glicosilada` | Exclusión explícita de este puesto; no juicio sobre toda la inmunología |
| Campo fuera de cobertura del puesto | `intervalo de referencia` | Campo no representado en la base del ensayo; no se sustituye por VALOR |

«Inmunoglobina» y «linmunoglobina» son variantes experimentales expresamente contempladas en el objetivo humano y en el banco conocido. No se elevan a terminología clínica, ni permiten corregir por semejanza «hemoglobina» a «inmunoglobulina».

## 4. Gramática de la candidata

Convenciones: terminales entre comillas; secuencia por concatenación; `|` alternativa; `[ ]` opcional; `{ }` repetición. Cada terminal con varias palabras exige esa secuencia de tokens; los separadores permitidos se conservan. Se exploran todas las derivaciones: no hay prioridad implícita por el orden de las alternativas. Los nombres con prefijo `G` identifican reglas para la traza.

```text
G01 Peticion   = ["¿"] Cuerpo ["?" | "."] ;
G02 Cuerpo     = Consulta
               | Negacion (";" | ":") Consulta
               | Consulta ("," | ";") Negacion
               | Grupo "," Negacion ":" Consulta
               | RecorteObjeto "no" ";" RecorteObjeto "," Consulta
               | Negacion ;
G03 Consulta   = Nucleo [Motivo] [Cortesia] ;
G04 Nucleo     = Orden Grupo
               | "escriba" Entero "en" Grupo
               | Grupo
               | ("cuál es" | "adónde estará" | "y") Grupo
               | "qué" Campo "tiene" Sujeto
               | "en qué unidad" ("está expresada" | "figura") Sujeto ;
G05 Orden      = ["sólo" | "solo"]
                 ("consulte" | "lea" | "traiga" | "dígame" | "necesito"
                  | "cambie" | "elimine") ;
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
               | ("de" | "para") Objeto
               | "del" ObjetoExplicito
               | ("de" | "del") ["corte"] Momento
               | "en el corte" Momento
               | "del" ("dato" | "registro") ;
G14 Objeto     = ObjetoExplicito | ObjetoContextual ;
G15 ObjetoExplicito = "caso-a" | "caso-b" ;
G16 ObjetoContextual = ("este" | "ese") ("caso" | "paciente")
               | "el caso que estamos viendo" | "este" "..." ;
G17 Momento    = "actual" | "ahora" | "anterior" | "previo" ;
G18 Negacion   = "no" Orden Grupo | "no quiero" Grupo | "no" Grupo
               | "no cambie nada" ;
G19 RecorteObjeto = "del" ObjetoExplicito | "de" Objeto ;
G20 Motivo     = "porque estoy revisando" Objeto ;
G21 Cortesia   = [","] "por favor" ;
G22 Art        = ["el" | "la"] ;
G23 Entero     = de 1 a 10 dígitos ASCII contiguos ;
```

`Entero` se reconoce y custodia como texto de escritura denegable; no se interpreta como valor del registro ni se convierte en un entero de máquina. `porque` sólo admite G20; no autoriza ignorar cualquier continuación. `adónde estará` se admite como consulta de registro en el alcance acotado aprobado para el ensayo; no se declara sinónimo universal de «cuál».

Los títulos G01–G23 identifican producciones; sus alternativas tienen identificador estable por posición, empezando en 1, y las decisiones opcionales/repetidas constan en el nodo. El léxico usa la fila y forma literal como identificador. Una siguiente versión no reutilizará esas identidades con otro significado.

## 5. Semántica de composición y resolución

### 5.1 Términos y alcance de la negación

Un grupo produce restricciones explícitas por campo, demandas de contexto, marcas de fuera de cobertura y evidencias de intervalo. Una cláusula positiva añade su operación explícita si existe; una cláusula nominal de consulta añade LEER. `elimine` conserva su marca de operación no admitida.

Las exclusiones son **patrones parciales de tupla**, con conjunción entre sus campos. «No consulte IgG» excluye `(operacion=LEER, parametro=IGG)`. No genera dos prohibiciones independientes de toda lectura y de toda mención de IGG. «No quiero IgA» y «no la IgG» excluyen únicamente el parámetro indicado; «del CASO-B no» excluye el objeto. «No cambie nada» excluye operaciones ESCRIBIR y ELIMINAR. Una exclusión no produce una petición positiva ni se completa con valores por defecto que reduzcan su alcance.

Los grupos bajo «no» conservan los campos explícitos que constituyen su patrón. Si contienen una referencia contextual, ésta debe resolverse contra el contexto: si falta, la exclusión no se descarta; queda una demanda pendiente. Las alternativas G10 producen interpretaciones distintas tanto en positivo como bajo negación; no se elige una por permisos. Esta candidata no promete resolver la ambigüedad pragmática de «no A o B».

G02 conserva la dirección de cada cláusula. En `Grupo, Negacion: Consulta`, se unen las restricciones positivas del primer grupo y la consulta y se conserva la exclusión. En el recorte de objetos, el primero es negativo y el segundo positivo. `Negacion` sola genera `SIN_SOLICITUD_POSITIVA`, sin lectura.

### 5.2 Unificación, contexto y campos

Dos restricciones positivas explícitas iguales se unifican conservando ambas evidencias. Dos valores distintos para el mismo campo producen contradicción; no manda el último. Las demandas contextuales de parámetro u objeto se resuelven usando el campo establecido, y también se conservan cuando queda ausente. La misma regla se aplica al objeto de G20; el motivo no introduce un caso nuevo.

Los campos restantes se completan desde contexto sólo cuando no hay una referencia explícita para ese campo, no existe marca de fuera de cobertura y ninguna demanda queda sin resolver. La mención explícita manda sobre el valor por defecto; no elimina otra referencia explícita ni una demanda contextual incompatible. Una referencia explícita desconocida no se sustituye por el contexto.

El Campo de G09 fija el campo solicitado; el Campo de la pregunta `qué Campo tiene Sujeto` también. La forma `en qué unidad …` fija UNIDAD. Los componentes del Sujeto aportan parámetro, momento u objeto, sin reponer el campo contextual VALOR. El complemento `del dato`/`del registro` de G13 designa el registro, no un segundo campo solicitado; así, «procedencia del dato anterior de IgG» pide FUENTE de IGG/ANTERIOR. Estas reglas de papel sintáctico sustituyen la búsqueda de palabras sueltas.

Tras la compleción, se comparan las rutas positivas con todos los patrones de exclusión. Si una ruta satisface íntegramente un patrón, hay contradicción de petición; no se selecciona otra referencia del inventario. Un patrón parcialmente no resuelto impide afirmar que la ruta está libre de esa exclusión.

### 5.3 Alternativas y resultados

Cada raíz completa se normaliza como restricciones positivas, exclusiones y demandas resueltas o pendientes. Se conservan también marcas de operación no admitida y fuera de cobertura. No se borran raíces incompletas porque otra pueda entregar dato. Se deduplican significados idénticos, nunca causas diferentes. Las huellas y el orden del modelo no deciden esa igualdad.

Para comparar dos significados de la misma pregunta se exige igual ruta positiva o igual ausencia de ella, iguales exclusiones normalizadas y las mismas demandas pendientes y marcas. Si hay más de un significado, el desenlace es `PETICION_AMBIGUA`, sin dato. No se filtran primero las alternativas prohibidas por política. Si todas las derivaciones confluyen en un significado, se aplica el siguiente orden estable; todas las causas encontradas quedan en la traza:

1. Contradicción explícita o entre ruta y exclusión: `PETICION_CONTRADICTORIA`.
2. Ninguna solicitud positiva: `SIN_SOLICITUD_POSITIVA`.
3. Operación reconocida no admitida: `OPERACION_NO_ADMITIDA`.
4. Parámetro/campo explícitamente excluido de este puesto: `FUERA_DE_COBERTURA`, con objeto de exclusión declarado.
5. Campo/referente exigido sin resolver: `CONTEXTO_INSUFICIENTE`.
6. Ruta completa representada: comprobar después política, vigencia y lectura literal de IE-004.

La ausencia de cualquier raíz completa da `SOLICITUD_NO_REPRESENTADA`; no demuestra fuera de dominio. Un fallo de transporte, presupuesto o implementación es técnico y precede a estos resultados. Los nuevos nombres son diagnósticos **candidatos del puesto**, separados de `Tri`, sin ampliar el catálogo de errores productivo. Los oráculos antiguos conservan sus nombres y criterios.

En la salida canónica se conservan estado de recepción, referencia y campo exactos, literal, fuente, alcance y versiones. El estado `0/1/U` almacenado sólo se devuelve al pedir ESTADO, sin reinterpretarlo. Permisos y revocación no proceden del certificado del modelo. Una avería, ambigüedad o referencia no disponible no se convierte en `U`.

## 6. Análisis, propuesta y presupuesto

Rige la tabla por intervalos y cierre de reglas de RETP-125, aplicada a tokens con correspondencia de bytes. Los resultados tipados de las celdas incluyen ausencias, exclusiones y marcas; un nodo que sólo enumera una cita no es una derivación completa. El verificador debe justificar corrección y completitud respecto de §§3–5; no basta comprobar que la raíz declarada tiene un tipo válido.

**Precisión sobre RETP-125:** la propuesta no puede cambiar el orden canónico de exploración ni consumir el presupuesto reservado al análisis confiable. De lo contrario, una propuesta grande o un orden distinto podría cambiar un dato por un agotamiento. Se propone orden ascendente de longitud de intervalo, inicio, regla/alternativa y representación canónica de resultados, con cierre hasta punto fijo. No se depende del orden de iteración de tablas hash.

Límites fijados como **candidata de laboratorio**, sin acreditación de viabilidad: 8 192 bytes de pregunta; 128 tokens no separadores; 256 KiB de propuesta; 512 nodos de certificado, 1 024 enlaces y profundidad 64; 16 384 estados de análisis almacenados y 1 000 000 de intentos de aplicación de regla. Un intento incluye cada combinación de hijos examinada, incluso si no produce un estado nuevo. Tokens de signos cuentan en 128; un terminal multipalabra cuenta sus tokens individuales. Duplicados de propuesta consumen su cupo de recepción, no alteran el del análisis.

La admisión de tamaños se hace antes de reservar almacenamiento proporcional. Contadores y estimaciones usan operaciones comprobadas; un desbordamiento, conversión no representable o fallo de reserva produce fallo técnico. No se trunca, satura ni deja envolver un contador para continuar. Los límites deben ensayarse en nativo y WASI con los mismos valores; `usize` no es una especificación portátil del formato.

Estos topes cubren por longitud el corpus histórico según el inventario documental adjunto; no se afirma que el número de estados o el tiempo real quepa en ellos. El presupuesto externo de proceso del puesto y el aislamiento frente a denegación de servicio siguen siendo materia de P4/P5. Si un certificado falla y el análisis independiente puede completarse, se conserva el servicio legítimo. Si el análisis no termina dentro de su presupuesto, no se declara unicidad ni acierto.

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

Esta tabla y `COBERTURA_DOCUMENTAL.json` son una revisión del contrato sobre antecedentes públicos. No son una ejecución del parser ni una calificación del modelo. Quedan por demostrar: aplicación exacta de todas las reglas, ausencia de ambigüedades imprevistas, recuperación dentro de presupuesto, paridad y beneficio efectivo del agente. Si la revisión independiente detecta un defecto del perfil, se registra y versiona antes de congelar la reserva; no se ajusta una implementación después de conocer el oráculo reservado.

La propiedad que se busca depende del perfil y del contexto constituido, no del supuesto conocimiento del modelo. Si el análisis completo hace al modelo prescindible sin utilidad medible, la arquitectura de colaboración sigue sin acreditarse. Ese límite se conserva como criterio de decisión, no se soluciona llamando «IA» al analizador determinista.
