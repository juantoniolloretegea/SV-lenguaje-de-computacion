# Acta técnica de ordenación de continuidad semántica y arquitectónica del Lenguaje SV

**Fecha de fijación:** 16/08/2026  
**Hora de fijación (Europe/Madrid):** 19:27  
**Revisión lingüística pública:** 18/08/2026, sin modificación de alcance, jerarquía ni decisiones  
**Naturaleza:** acta pública de gobierno técnico y continuidad  
**Ámbito:** `SV-lenguaje-de-computacion`  
**Estado:** vigente tras aprobación humana  
**Destinatario operativo principal:** futuras unidades y responsables técnicos que deban continuar el Lenguaje SV  
**Acta privada local:** existe y queda custodiada de forma local

## 1. Objeto

La presente acta ordena los frentes semánticos, matemáticos, de seguridad y de continuidad que deberán comparecer antes de cualquier nueva expansión material del Lenguaje SV.

Su finalidad es impedir cuatro errores de continuidad:

1. perder desarrollos doctrinales o matemáticos ya existentes por quedar fuera del repositorio técnico inmediato;
2. convertir una publicación, colección, prototipo o realización aplicada en semántica ejecutiva por mera proximidad temática;
3. continuar IR, DSL, bibliotecas, agentes o infraestructura de ejecución sobre distinciones todavía no suficientemente reconciliadas;
4. bloquear indefinidamente el Lenguaje por pretender completar de antemano toda matemática o toda colección lateral.

Esta acta es autocontenida y debe poder gobernar una futura reentrada. Existe además un acta privada local destinada a restauración de contexto cuando esté disponible. No es normativa y no sustituye esta acta ni a la doctrina superior.

## 2. Jerarquía aplicable

Se mantiene la jerarquía vigente:

- `SV-matematica-semantica` conserva la primacía doctrinal y matemática;
- la especificación operativa del Lenguaje queda subordinada a esa doctrina;
- analizador sintáctico, descenso a IR, validador, ejecutores de prueba, interfaz de línea de órdenes, `Playground`, bibliotecas e infraestructura de ejecución son realizaciones técnicas subordinadas;
- ninguna capa inferior corrige silenciosamente a una superior;
- el humano fija objetivos, límites y autorizaciones; el álgebra gobierna; la IA permanece subordinada.

La dirección legítima de descenso continúa siendo:

`doctrina → especificación → lenguaje → pruebas → evidencia`.

Una publicación o realización posterior puede ejercer presión legítima sobre el Lenguaje, pero no se convierte por ese solo hecho en palabra reservada, nodo IR, diagnóstico, capacidad de entorno de ejecución o contrato de infraestructura ejecutiva.

## 3. Estado técnico recibido y pausa de control

El tablero vigente del frente final mantiene:

- `FFL-A` — contrato diagnóstico — formalmente abierto;
- `FFL-B` — cadena de implementación — abierto;
- `FFL-C` — batería de pruebas y evidencia — abierto;
- `FFL-D` — documentación pública — abierto;
- `FFL-E` — ABI semántico-diagnóstico — abierto.

La etapa frontal existente analiza, valida, desciende a IR y serializa. Esa capacidad no equivale todavía a ejecución operacional general. La infraestructura soberana de ejecución en Rust permanece como objetivo futuro y no se abre por esta acta.

Hasta este punto, `FFL-A` era el primer frente cerrable por prioridad técnica. A partir de la presente acta se introduce una **pausa preventiva adicional**: una vez incorporada esta acta, no se realizará nueva modificación material del Lenguaje SV hasta completar el punto de control doctrinal y matemático definido en las secciones 7 y 8.

Esta pausa:

- no cancela `FFL-A`;
- no declara incorrecta la IR vigente;
- no convierte las aperturas matemáticas en errores del compilador;
- no prejuzga que todas las colecciones matemáticas deban completarse antes de reabrir el Lenguaje.

Su objeto es evitar que una corrección o extensión técnica cristalice prematuramente contratos sobre fundamentos que se han decidido revisar primero.

## 4. Seis bloques de continuidad

Los errores y diagnósticos no constituyen un séptimo bloque. Forman una capa transversal que deberá expresar de forma estable los incumplimientos de los seis frentes siguientes.

### 4.1. Bloque 1 — Suceso y dinámica eventiva

Comprende el sustrato semántico constituido alrededor de suceso, ocurrencia, activación, admisibilidad, horizonte, `Frame`, trayectoria, identidad, encadenamiento, ciclos, acumulación, cambio de régimen, preservación y `U`.

La familia VII, la Colección I y desarrollos matemáticos posteriores aportan una base extensa y materialmente relevante. Sin embargo, no debe confundirse el desarrollo del **cálculo del suceso** con el cierre de toda su **dinámica general**. Persisten elementos que requieren cierre o reconciliación antes de una semántica ejecutiva general del suceso, especialmente la relación canónica entre tipo de suceso, ocurrencia, activador, dato de transición, `Frame` y trayectoria, así como el estatuto de la composición parcial cuando resulte realmente necesaria.

Esta acta no declara que exista una gran teoría perdida ni obliga a crear estructuras algebraicas adicionales. Obliga a comprobar qué aperturas siguen vivas y a no fijar en código una respuesta antes de resolverlas.

### 4.2. Bloque 2 — Interfaces y transducción entre planos

Comprende las fronteras humano/SV, dominio/representación, conectores, transductores, representación poligonal y transferencias entre componentes.

Una interfaz transporta, traduce o hace visible bajo un contrato declarado. No crea autoridad, verdad o cierre por vías implícitas. Toda transducción deberá declarar dominio, codominio, pérdidas, condiciones de admisión y retorno cuando proceda.

Los Transductores de matemáticas pueden tener materialización reutilizable dentro del bloque de bibliotecas, pero su contrato de frontera pertenece a este bloque.

### 4.3. Bloque 3 — Tesauro, bibliotecas y tronco matemático

El **Tesauro de matemáticas del Sistema Vectorial SV** se recibe como estructura troncal de ordenación del cuerpo matemático. Su función es reunir, precisar, relacionar y hacer legibles las matemáticas existentes; no crear matemáticas para completar categorías.

`Matemáticas del Potencial SV` se recibe como colección matemática especializada adscrita al Tesauro y coordinada con los Transductores de matemáticas. Su propio ámbito depende de objetos generales como suceso admisible, dominio, `Frame`, trayectoria, identidad, residual, `U` y retorno. Por ello no podrá utilizarse para corregir silenciosamente la matemática general del suceso.

La futura biblioteca estándar y las bibliotecas científicas, gráficas o de otros dominios deberán separar:

- núcleo semántico indispensable del Lenguaje;
- biblioteca matemática central y transversal;
- bibliotecas especializadas.

Que una matemática figure en el Tesauro no implica que deba convertirse en primitiva de SVP.

### 4.4. Bloque 4 — IA propia del ecosistema SV

Incluye el motor IA/Inteligencia Lógica, NLP y Banco de Idiomas.

La IA puede proponer observables, traducciones, recomendaciones o estructuras auxiliares, pero no sustituye al álgebra ni al experto soberano. Los documentos NLP y los desarrollos posteriores constituyen continuidad legítima, no autorización automática para modificar DSL, IR o entorno de ejecución.

La reconciliación interna de NLP, Banco de Idiomas y motor deberá realizarse antes de una integración productiva de agentes o capacidades de IA en el Lenguaje.

### 4.5. Bloque 5 — Frontera de seguridad frente a IA y fuentes externas no confiables

Este bloque es una frontera de confianza transversal y no un subapartado del motor IA.

Existen ya realizaciones aplicadas con cuarentena, comprobación de tipo y límites, saneamiento, procedencia, control de ascenso, preservación de `U`, confirmación humana y separación entre exploración y constitución efectiva. Esa custodia no se considera todavía universalizada en todos los puntos sensibles.

Antes de habilitar ingestión de terceros, acceso en línea, herramientas, memoria externa, infraestructura ejecutable, WASM o superficies equivalentes deberá existir una frontera explícita que impida que contenido externo no confiable alcance directamente el núcleo semántico o ejecutivo.

`SEC-0` mantiene su estatuto de referencia básica de resistencia del compilador. Esta acta no declara abierta ni numera una fase posterior de seguridad; únicamente fija la obligación arquitectónica que deberá resolverse cuando el Lenguaje admita entrada externa no confiable o un entorno de ejecución abierto.

### 4.6. Bloque 6 — Programación de SVP mediante lenguaje natural

Este frente es distinto del NLP general. Su objeto es transducir intención humana a programas `.svp` bajo la gramática y los contratos vigentes.

La IA no podrá inventar palabras reservadas, tipos, operadores o semántica por generación automática. El programa resultante deberá atravesar las mismas reglas de análisis sintáctico, descenso a IR, validación, diagnóstico y evidencia que un programa escrito manualmente y conservar una traza suficiente entre intención, reglas de transducción y salida.

Este bloque no se abrirá como capacidad productiva antes de que el Lenguaje alcance estabilidad suficiente y la seguridad de la entrada correspondiente esté definida.

## 5. Diagnósticos y evidencia como capa transversal

Toda futura incorporación de alguno de los seis bloques deberá poder descender a una cadena verificable:

`fundamento o contrato → objeto o frontera técnica → regla de bienformación o ejecución → diagnóstico observable → prueba`.

No procede crear, renumerar o reutilizar códigos canónicos antes de fijar de forma inequívoca:

- qué incumplimiento representan;
- dónde es alcanzable;
- dónde se emite;
- qué componente depende de él;
- qué evidencia permite sostenerlo.

Cuando `FFL-A` se reabra deberá hacerlo contra el repositorio fresco y sin revivir contradicciones ya resueltas por memoria histórica.

## 6. Relación especial entre Suceso, Tesauro y Matemáticas del Potencial

El Tesauro es troncal como arquitectura de ordenación, pero no es una fuente creadora de matemática. `Matemáticas del Potencial SV` es especializada y depende parcialmente de la matemática general del suceso.

Por ello se fija el orden conceptual siguiente:

1. cerrar o reconciliar la matemática general que realmente permanezca abierta;
2. revisar las matemáticas especializadas contra ese cierre;
3. actualizar el Tesauro para que ordene el corpus resultante sin inventar definiciones;
4. decidir qué parte pertenece al núcleo del Lenguaje, qué parte a una biblioteca central y qué parte a bibliotecas especializadas;
5. sólo después materializar en SVP aquello que realmente deba ser ejecutable por el Lenguaje.

La colección `Matemáticas del Potencial SV` fue concebida inicialmente en siete publicaciones. La cifra editorial no se convierte por esta acta en obligación matemática. Tras el cierre de la Dinámica del Suceso deberá auditarse cada pieza prevista contra el corpus ya existente para decidir si se conserva, se reduce, se reformula, se integra o deja de ser necesaria como publicación autónoma.

## 7. Primer punto de control previo al Lenguaje: cierre o reconciliación del programa de gobierno determinista

Antes de reabrir modificaciones materiales del Lenguaje se realizará un cierre funcional del antiguo programa de gobierno determinista que dio origen a la Colección I.

Ese trabajo deberá:

- releer materialmente los tres documentos de Colección I;
- contrastarlos con la familia VII y los desarrollos posteriores sobre células, agentes, universo de sucesos, ITI, Carta Magna, frontera SV/IA e interfaces relevantes;
- determinar función por función qué objetivos del antiguo programa fueron absorbidos, superados o permanecen realmente abiertos;
- evitar reconstruir por memoria los Documentos 4–9 o convertir su numeración histórica en deuda automática.

**Lo obligatorio es el cierre o reconciliación funcional.** La forma bibliográfica final no queda impuesta por esta acta. Si la revisión demuestra contenido doctrinal autónomo suficiente podrá adoptar forma de publicación; si no, bastará la pieza doctrinal de cierre que corresponda.

## 8. Segundo punto de control previo al Lenguaje: Dinámica del Suceso

Después del cierre anterior se desarrollará una pieza matemática específica de **Dinámica del Suceso**, con título definitivo aún no fijado.

No deberá ser una reedición panorámica del corpus existente. Su primera tarea será construir una matriz de obligaciones y vecinos que incluya, como mínimo, Composición III, Transiciones de U, familia VII, Documento 1 de Colección I/HNA, Nuevas matemáticas, Conjunto matemático unificado, Potencial de un suceso, Tesauro, Transductores y los desarrollos posteriores que afecten materialmente al objeto.

La pieza deberá determinar, sin sobreconstrucción:

- qué distingue tipo de suceso, ocurrencia, activador, dato de transición, `Frame`, horizonte y trayectoria;
- qué identidad necesita el suceso en cada alcance;
- si una composición parcial de sucesos es necesaria y, sólo en ese caso, bajo qué dominio y compatibilidad;
- cómo se articulan cadenas, ciclos, acumulación, cambio de horizonte, bifurcación y preservación `append-only`;
- qué aperturas históricas están cerradas, superadas, absorbidas, resultan innecesarias o permanecen realmente vivas.

No se impondrá un suceso nulo universal, un monoide o cualquier otra estructura por obligación editorial. La estructura se fijará únicamente si el objeto matemático y su uso operativo la exigen.

## 9. Compuerta posterior: revisión de base de Matemáticas del Potencial y Tesauro

Terminados los dos cierres anteriores se realizará una auditoría acotada específica de `Matemáticas del Potencial SV` y del Tesauro.

Esa auditoría decidirá expresamente una de estas dos rutas:

- **Ruta A:** la matemática troncal queda suficientemente estabilizada y `FFL-A` puede reabrirse sin esperar al cierre completo de la colección de Potencial;
- **Ruta B:** determinadas piezas de Potencial o una actualización previa del Tesauro constituyen dependencia real para la superficie del Lenguaje que se pretende tocar y deberán cerrarse antes.

Esta decisión evita dos errores simétricos: programar demasiado pronto y bloquear el Lenguaje por una colección especializada que no sea dependencia del frente técnico inmediato.

## 10. Relación con actas vigentes

Se mantienen vigentes, sin duplicación ni reescritura silenciosa:

- el acta de 26/03/2026 sobre continuidad tras la familia VII;
- el acta de 30/03/2026 sobre Colección I, custodia estructural y NLP;
- el cierre auditado y preservación de SV-AUTH de 14/08/2026;
- la recepción doctrinal del aprendizaje trazable de 15/08/2026;
- la fijación previa al DSL del aprendizaje trazable de 16/08/2026.

AUTH y aprendizaje trazable permanecen como continuidad latente legítima y no autorizan integración automática.

## 11. Condición de reapertura del Lenguaje

No se reabrirá automáticamente el Lenguaje por el mero hecho de publicar o cerrar documentos.

Tras los puntos de control de las secciones 7–9 deberá existir un dictamen corto y explícito que responda:

1. qué cambió doctrinal o matemáticamente respecto del contrato actual;
2. qué objetos del Lenguaje quedan afectados de forma real;
3. si procede modificar IR, DSL o código, o basta con documentación, bibliotecas o contratos externos;
4. qué bloque o bloques de los seis quedan activados;
5. qué batería de pruebas, diagnóstico y evidencia serán necesarios.

Sólo entonces se decidirá si `FFL-A` recupera prioridad inmediata o si persiste una dependencia previa.

## 12. No efectos de esta acta

Esta acta:

- no modifica la doctrina del Sistema Vectorial SV;
- no modifica la IR canónica ni la gramática superficial;
- no modifica analizador sintáctico, descenso a IR, validador, ejecutores de prueba, interfaz de línea de órdenes, `Playground` o infraestructura de ejecución;
- no crea nuevos códigos de error;
- no declara cerrada la dinámica general del Suceso;
- no declara completadas las siete publicaciones de Matemáticas del Potencial;
- no convierte el Tesauro en creador de matemática;
- no universaliza la cuarentena de entradas externas;
- no abre una nueva fase de seguridad;
- no integra motor IA, NLP, Banco de Idiomas, agentes o programación natural en SVP;
- no abre Rust soberano.

## 13. Acta privada local y restauración futura

Existe un acta privada local, custodiada por el director humano.

Cuando el director humano la proporcione o autorice su lectura, una futura unidad deberá leerla **después de esta acta** y **antes de proponer una reapertura material**, y deberá contrastarla siempre con el repositorio y la doctrina frescos.

La ausencia temporal del acta privada no invalida esta acta pública ni autoriza a inferir el contenido que falte.

## 14. Cierre

**Seis bloques de continuidad:** RECIBIDOS Y ORDENADOS.  
**Modificación material inmediata del Lenguaje:** PAUSADA TRAS ESTA ACTA.  
**Primer cierre previo:** reconciliación funcional del programa de gobierno determinista.  
**Segundo cierre previo:** Dinámica del Suceso.  
**Comprobación posterior:** revisión de base de Matemáticas del Potencial y Tesauro y decisión expresa sobre dependencia real.  
**FFL-A:** no cancelado; reapertura diferida hasta el punto de control.  
**Infraestructura de ejecución/Rust:** NO ABIERTA.  
**Entrada externa no confiable:** requiere una futura frontera de custodia no eludible antes de su habilitación.  
**Integración automática de IA/NLP/bibliotecas/NL→SVP:** NO AUTORIZADA.

La continuidad queda, por tanto, gobernada por una regla única: **primero se cierra la matemática o doctrina que realmente falte; después se ordena en el Tesauro; después se materializa en bibliotecas; y sólo finalmente se incorpora al Lenguaje aquello que deba pertenecer al Lenguaje.** Ninguna especialización matemática asciende automáticamente a primitiva de SVP.
