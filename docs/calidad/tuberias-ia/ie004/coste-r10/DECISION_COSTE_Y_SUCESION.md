# Decisión causal sobre coste y sucesión del calendario

**RETP-2026-132 · 11/09/2026.** Corte de entrada: Lenguaje `af38e499b46efc05ff4bc98ae2bc66d004ec8b3f`; laboratorio `854c8c332f38843e4acf333d62b98f1aadbd7ec6`. Autorización humana posterior a RETP-131: «De acuerdo, sigamos con el coste y el paso siguiente».

## Objeto acotado

Una corrección causal del recorrido sintáctico, conservando exactamente el léxico, G01–G25 y los 72 controles públicos de RETP-130. La alternativa se llama **IE004-R10-MEMO/1**. No se presenta como realización conforme al calendario obligatorio de IE004-ES-P2/2 ni como corrector semántico completo. No se modifican /2, el compromiso, las preguntas, las notas ni el oráculo reservado. No se usa conocimiento de esos archivos.

## Causa y cota condicionada

La sonda anterior cobra una visita por cada alternativa exterior de G01–G25, incluso sin hijos compatibles. Son 66 alternativas. Para n tokens visita `(n+1)(n+2)/2` intervalos, incluidos los vacíos. Si se produce alguna inserción, su cierre exige al menos un segundo barrido completo. Sólo esas visitas cuestan:

`2 × 66 × (n+1)(n+2)/2`.

Para 127 tokens: 1.089.792; para 128: 1.106.820. Superan el millón antes de añadir las particiones internas. Por tanto, acelerar el código conservando **este cobro exhaustivo** no puede resolver esos dos casos dentro de su cuenta. C02 demuestra además un agotamiento observado a 17 tokens por la acumulación de barridos y búsqueda interna.

Esta cota corresponde al instrumento /1, con sus visitas cobradas. /2 habla de intentos y combinaciones; no se eleva esta codificación particular a prueba de que toda interpretación posible de su contabilidad sea imposible. Sí exige abandonar o precisar expresamente una realización que ya ha fallado. Cambiar sólo el nombre de las unidades o no cobrar trabajo repetido no sería una corrección causal.

## Cambio material propuesto

El grafo de referencias entre G01–G25 es acíclico. El compilador experimental lo comprueba. Comprueba también que el cuerpo de cada repetición tiene longitud mínima positiva. Se puede evaluar por dependencia y posición, guardando cada resultado `(expresión, posición inicial)` una sola vez. Cada resultado conserva **todos** los extremos alcanzados y todas las familias de hijos; nunca el primero que acierta.

Las repeticiones sólo recursan desde una posición a otra estrictamente mayor. Se conserva un bosque compacto de derivaciones: los subárboles comunes comparten almacenamiento; las alternativas conservan identidad, extremos e hijos. No se expanden todos los árboles completos ni se elige por política. Una raíz sintáctica de G10 puede seguir produciendo varios significados: la unicidad sintáctica no decide la semántica de /2 §5.3.

La compilación de EBNF es explícita: secuencia a pares, alternativa a unión, opcional a unión con vacío, repetición a vacío o hijo positivo seguido de repetición. Cada expresión guarda su regla de origen. Los terminales y referencias se copian del instrumento anterior sin editar la gramática. El original y los préstamos UTF-8 conservan el mismo código léxico y de primitivas.

## Argumento de equivalencia sintáctica y sus límites

Por inducción estructural, un terminal aporta su intervalo exacto; una alternativa reúne ambas relaciones; una secuencia compone todos los extremos compatibles; un opcional añade vacío. Para la repetición se usa inducción sobre los tokens restantes: el cuerpo consume al menos uno y se conservan todas las particiones con ese cuerpo. La inducción sobre el DAG de referencias concluye la igualdad de la relación de derivabilidad. Memorizar una relación completa y de sólo lectura no cambia esa relación. Ninguna poda depende del contexto, permisos, un hash, una propuesta externa o un caso Lxx/Rxx.

Es un argumento revisable acompañado de código y corpus, no una demostración mecanizada ni una prueba de equivalencia de semántica, diagnósticos de recursos u orden de trazas. Una recursión entre reglas o una repetición anulable haría inaplicable este mecanismo y provoca error de constitución en el instrumento. No se resuelve escondiendo el ciclo. El conjunto de derivaciones sólo se usa si la evaluación termina íntegra dentro de sus cuotas.

## Contabilidad y almacenamiento de la alternativa

Se limita a un millón de **unidades experimentales**, con cargos previos por evaluación, consulta a memo (incluidos aciertos repetidos), byte comparado, combinación de hijos examinada, comparación para deduplicar familias, inserción de familia e inserción de nodo. Una evaluación fallida consume su trabajo. Estas unidades **no son intercambiables** con los intentos de /1 ni constituyen una nueva definición silenciosa de A en /2. Se publican desglosadas; no se calcula una mejora porcentual comparando ambas cuentas.

La arena sintáctica tiene un máximo candidato de 32 MiB de capacidades contabilizadas. Incluye la tabla triangular de referencias, los estados de memo, los vectores de resultados, los nodos y familias. Antes de ampliar un vector se comprueba también la coexistencia de la capacidad anterior y la nueva solicitada. La asignación es fallible. Se registra capacidad real y pico solicitado. El contexto, el original, la normalización léxica, gramática fija, pila, runtime y serialización quedan fuera de esa cifra y deben medirse aparte: no es RSS ni la arena semántica completa de P3. Los nodos del bosque no se equiparan a los 16.384 significados parciales del contrato A, todavía no implementados.

La gramática compilada se acota a 512 expresiones; se rechaza el exceso. Todos los índices exteriores y productos de dimensiones se comprueban antes de reservar. La tabla triangular usa u32 como identificador interno con conversión comprobada, y u64 para tamaños/cargos; NULO es el centinela u32::MAX. Las vidas de los slices no sustituyen la identidad de solicitud ni la separación A/V.

## Contraste fijado antes de ejecutar

Una compilación preparatoria sin ejecución comprueba la construcción de la fuente. Después se fijan las huellas y se ejecuta una vez en cada configuración Rust 1.98.0: nativo/WASI, debug/release, mismos flags de RETP-131. Máximo 30 s por proceso de prueba, sin reintentos; compilación acotada y resultado de cada intento conservado. No se lanza una enumeración general ni se solicita otra captura de modelo.

Los mismos 72 esperados permanecen: 48 históricos, 18 contrastes, 127/128/129 tokens y 8191/8192/8193 bytes. Debe coincidir el desenlace de todos los controles; en particular, C02/127/128 deben terminar con DERIVA. Las 21 primitivas siguen ejecutándose. Se compara además el bosque completo emitido por Rust entre destinos/configuraciones, excluyendo medidas de memoria y cabeceras de plataforma. No se infiere paridad semántica por esa identidad.

El lanzador Node sólo compila, captura, compara y calcula huellas de bytes de evidencia; no ejecuta reglas gramaticales ni decide significados. No se ejecuta Python. Los tiempos de un único proceso incluyen arranque y exportación de bosque: no son un benchmark, una media clínica ni una comparación justa de velocidad con /1, que exportaba menos datos. P5 conserva su diseño pareado futuro.

## Consecuencia sobre P3 y siguiente paso

Si la alternativa falla, se conserva el contraejemplo y se cierra este intento. Si pasa, habilita trabajar en una **sucesora explícita del calendario**, con evaluación semántica tipada del bosque que conserve /2 §5, antes de congelar el corrector y el esquema V. Los barridos de /2 §6.3, su orden de trazas y la imputación A deben revisarse expresamente; un resultado de esta sonda no los sustituye automáticamente.

La reserva conserva su identidad /2. Cualquier uso para una sucesora necesita una declaración de compatibilidad que mantenga preguntas/contextos, versiones de base/política, significado y esperados, y documente el cambio de calendario/recursos. Si cambia un significado o el ámbito, se devuelve al custodio la decisión sobre esa reserva; no se reetiqueta su hash. Esta pieza no autoriza leer el oráculo, pedir una segunda reserva ni iniciar la captura de Grok.

El próximo objeto material, si el control termina, es el **evaluador semántico de A sobre bosque completo**, con restricciones por campo, productos de G10, exclusiones parciales, demandas contextuales y deduplicación de significado antes de permisos. Su primera prueba utilizará exclusivamente los antecedentes públicos. Transporte A/V, certificado V, límites materiales P4 y viabilidad P5 mantienen sus puertas. El beneficio útil del agente sigue sin demostrarse: el análisis A no espera a V.

Rectoras consultadas completas en esta continuidad: AGENTS, Pilares, perfiles/ensamblaje, transición §§1–30 y arquitectura; perfiles /2, workflow RETP-123, guía RETP-127 y contrato RETP-128. Se cotejan sus identidades contra el corte vigente. Esta decisión experimental no cambia núcleo, DSL/IR, constitución de dominio, catálogo/localización ni fila 9.
