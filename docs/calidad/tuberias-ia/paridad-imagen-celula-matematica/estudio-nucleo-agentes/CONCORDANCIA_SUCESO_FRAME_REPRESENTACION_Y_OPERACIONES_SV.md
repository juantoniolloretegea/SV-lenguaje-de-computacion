# Suceso, frame, representación y operaciones de una biblioteca SV

**Expediente:** S22 · (p1+p3)-Bis · BIS-01.  
**Estatuto:** contraste documental y propuesta de delimitación para BIS-02; no constituye una revisión aprobada de la semántica, de la IR ni del núcleo.  
**Fecha:** 13 de septiembre de 2026.

## 1. Objeto y conclusión del contraste

La palabra «suceso» aparece en la doctrina algebraica, en la descripción de actividades humanas o técnicas y en los registros del proyecto. Esa coincidencia léxica no basta para atribuir a todos esos usos un único tipo formal. Del mismo modo, «frame» se emplea en la evaluación de una arquitectura y en la discusión de la paridad entre una célula matemática y su representación gráfica. La correspondencia entre estas acepciones debe declararse antes de convertirlas en tipos o interfaces de programación.

La revisión confirma una distinción ya documentada: el Documento III define el frame canónico como resultado de una evaluación espacial y distingue ese resultado de los sucesos instanciados que motivan una reevaluación. El Documento V conserva esa separación y añade la consulta trazable. La especificación metodológica subordinada sitúa las lecturas geométricas después del resultado estructural. Por tanto, existe fundamento para alinear las acepciones sin hacer de toda actividad registrada una transición algebraica ni de todo frame un archivo de imagen.

El requisito planteado por el autor —pueden existir sucesos sin frame, pero no frames sin suceso— se recibe para su formalización. La formulación propuesta es **vinculación constitutiva y trazable entre el frame y su evaluación o reevaluación**, con los sucesos pertinentes cuando corresponda. No se da por demostrada una identidad de tipos entre frame y suceso del horizonte. El régimen de constitución del frame inicial requiere una casilla explícita: no se fabricará una transición anterior únicamente para satisfacer una cadena de registro.

## 1.1. Precisión del autor: el hecho, la prosa, la matemática y la imagen

El autor precisa durante el contraste: **«El suceso es el hecho; la prosa, la matemática y la imagen son formas de representarlo».** Esta formulación incluye expresamente la representación matemática. Según el contrato y la finalidad, la documentación puede emplear prosa, representación matemática, imagen o una combinación de ellas. La diferencia se sitúa en la necesidad y el contrato de representación, no en que existan hechos de menor o mayor realidad por disponer de imagen.

Se adopta esta acepción general como criterio de trabajo recibido del autor. Se distinguen el hecho y sus formas de representación: discursiva, matemática y visual, con sus respectivos contratos. El registro conserva evidencia de la actividad o del objeto representado. Un hecho no deja de existir por carecer de dibujo; tampoco se acredita un hecho únicamente porque exista una narración o una imagen que lo afirme. Sus consecuencias deberán identificarse con su alcance y respaldo; no se presume que todas sean conocidas o que produzcan cambios paramétricos relevantes para una arquitectura concreta.

En particular, «la IA emitió el consejo X» puede ser un hecho registrado sin que por ello quede acreditada la verdad del contenido de X. La tubería deberá preservar esa diferencia entre la ocurrencia de una afirmación y la justificación de lo afirmado.

Dentro del SV, el álgebra establece las restricciones de la representación matemática y de su correspondencia gráfica. La fidelidad al hecho representado requiere, además, respaldo en la captura, admisibilidad, transducción y trazabilidad que correspondan. La coherencia interna de una representación no acredita por sí sola la verdad de una afirmación sobre el mundo. Las distintas representaciones deben declarar su alcance; no se presume identidad de contenido ni reversibilidad entre prosa, matemática e imagen.

La analogía de fotografiar un volcán o una escena explica la utilidad de complementar la prosa. En la paridad SV, el polígono es específicamente una representación del estado matemático bajo un convenio declarado. Una fotografía externa puede ser evidencia de entrada, pero no se convierte automáticamente en ese polígono ni en un frame canónico.

Esta precisión permite conservar un concepto general de suceso y, a la vez, delimitar su admisión al horizonte formal de una arquitectura. La expresión «suceso del horizonte» añade condiciones contractuales a la acepción general; no convierte la mera ausencia de admisión en inexistencia del hecho.

## 2. Fuentes y rango

Se han contrastado las siguientes piezas en el corte `b8fd32978292d25adf9b87cf71e409005dce642c` de `SV-matematica-semantica`:

- [Documento III, §§3.2–3.7, 5 y 6](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/documentos/composicion/III_horizonte_sucesos_reevaluacion_discreta.md): horizonte, sucesos instanciados, frame, transición, trayectoria y regímenes de reevaluación.
- [Documento V, §§2, 4–6](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/documentos/composicion/V_invariantes_agentes_operador_consulta.md): herencia doctrinal, invariantes, especialización y consulta.
- [Especificación metodológica, §§2 y 7–10](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/especificaciones/proposiciones/ESPECIFICACION_METODOLOGICA_SOBRE_SUCESOS_REEVALUACION_Y_FRAMES_EN_EL_SISTEMA_VECTORIAL_SV.md): resultado estructural y lecturas auxiliares. Es una pieza subordinada de encuadre.
- Nota de precisión *Suceso local, suceso envolvente y reevaluación situacional*, §§4–6 y 13: la pertenencia a un horizonte competente requiere declaración; una circunstancia externa no adquiere automáticamente potestad de reevaluación. La nota declara que no impone una modificación inmediata de la IR.

Se contrasta asimismo la IR canónica 0.3, especialmente su adenda §6.6, y el anexo SEC5, §§8–9, sobre el estatuto del Frame y la vinculación probatoria externa. Las identidades de los archivos examinados constan en [el manifiesto](FUENTES_CONCORDANCIA_SUCESO_FRAME.json). Los Fundamentos conservan su primacía. Este informe no armoniza silenciosamente variantes de notación ni convierte el estado histórico de los dominios del Documento V en una decisión actual de incorporación al Lenguaje.

## 3. Concordancia de acepciones

| Acepción | Estatuto y criterio | Relación con el frame |
|---|---|---|
| Actividad o hecho técnico registrado | Lectura, recepción, revisión, propuesta o ejecución, descrita por su contrato de trazabilidad. Su registro es evidencia de la actividad; no es la actividad misma. | Puede existir sin constituir ni producir un frame. |
| Tipo de suceso del horizonte | Tipo relevante declarado para una arquitectura. En la IR: `Horizon.events : [EventType]`. | Declara qué puede ser pertinente para reevaluar; no representa una secuencia de frames observados. |
| Suceso instanciado | Incidencia, ausencia verificada o indeterminación de un tipo pertinente, según protocolo. En la IR: pares de `TransitionData.events`. | Integra el dato que motiva una reevaluación; no es su resultado. |
| Frame canónico del Documento III | Evaluación completa de la arquitectura: Sₙ = ℰ(𝒜, Pₙ). El frame pasado es inmutable. | Es el resultado estructural; debe distinguirse de la transición y de la historia completa. |
| Paridad celular discutida en Bis | Correspondencia tipada entre estado matemático de una célula y representación gráfica bajo contrato. | Su relación con el frame de arquitectura y los objetos de IR debe precisarse; no se presume equivalencia por compartir nombre. |
| Representación poligonal | Lectura gráfica subordinada al objeto algebraico, a su orden paramétrico y a su convenio declarado. | Representa el estado referenciado; no lo constituye ni lo reemplaza. |
| Consulta formal del Documento V | Operación bajo firma y alcance, con respuesta, justificación reconstruible y metadatos. | Puede leer el estado o comparar frames sin reescribir la trayectoria. La consulta no exige por sí sola crear un frame nuevo. |

La asociación es contractual y puede cruzar estas filas. Por ejemplo, una lectura de archivo puede registrarse como actividad técnica y, si satisface además el contrato de entrada de un horizonte declarado, aportar evidencia para un suceso formal. Esa segunda condición debe verificarse; no se obtiene del verbo «leer».

## 4. Existencia, correspondencia gráfica y presentación

Para desarrollar la paridad `(frmat, frvis)` deben distinguirse tres obligaciones:

1. **Correspondencia definida:** qué estado matemático, constitución, orden de parámetros y convenio de representación corresponden a la parte visual.
2. **Representación materializada:** generación de un artefacto gráfico concreto a partir de esa correspondencia.
3. **Representación presentada o consumida:** uso efectivo del artefacto por una GUI, por una persona o por un componente de visión.

La primera obligación protege la paridad. Las otras dos dependen del contrato de uso que se constituya. El funcionamiento sin interfaz gráfica no debe confundirse con permiso para perder la referencia matemática. Una imagen antigua tampoco puede presentarse como representación de un estado nuevo.

Esta separación se propone para conciliar el par tipado con los usos sin presentación visual. Queda pendiente decidir si `frvis` designará una descripción verificable de representación o un artefacto ya materializado; no se modifica esa definición mediante este informe.

Los parámetros nucleares de conocimiento y los singulares de decisión pueden compartir el molde gráfico sin compartir significado. No se restringe la representación a decisiones ni se impone a cada actividad técnica. Los radios, colores o convenciones históricos no redefinen el álgebra.

## 5. Ejemplo de frontera en la tubería documental

La cadena «obtener un documento → revisarlo → presentar un consejo documental» puede dejar evidencia de fuente, versión, actividad y resultado sin crear un frame. Tampoco toda salida textual de una IA constituye el operador formal de consulta del Documento V.

Si el proceso incorpora valores a parámetros declarados, evalúa una célula o una composición y presenta una conclusión como decisión formal SV, debe justificar la entrada, la evaluación, el resultado y su vínculo con el frame correspondiente. La consulta de un frame ya constituido no equivale a una nueva reevaluación. La presentación poligonal será exigible en los usos que así lo declaren.

Debe fijarse expresamente el alcance de «decisión»: el requisito del autor se dirige a decisiones formalizadas en SV. No se extiende por analogía a cada bifurcación interna de un programa. Una compuerta de autorización modelada como célula sí pertenece al examen algebraico, aunque su dominio sea la propia tubería de IA.

El fallo de renderizado, la ausencia de archivo o una referencia gráfica incorrecta son incidencias técnicas. No se convierten automáticamente en `U`. Tampoco se inventarán coordenadas para obtener un polígono aparentemente completo.

## 6. Biblioteca SV: primitivas, funciones, métodos y macros

La clasificación semántica de una operación y su mecanismo de implementación son dos decisiones diferentes.

| Concepto | Criterio de admisión |
|---|---|
| Primitiva del SV | Su estatuto proviene del fundamento o de una ampliación formal constituida; debe tener dominio, codominio y leyes declarados. |
| Operación derivada | Se construye a partir de operaciones admitidas y conserva sus condiciones e invariantes. Puede formar parte de una biblioteca base. |
| Función Rust | Forma de implementación apropiada cuando la operación no requiere un receptor privilegiado o cuando conviene expresar explícitamente todos sus argumentos. |
| Método Rust | Operación con receptor cuya elección debe concordar con el contrato; la notación de llamada no crea ni prueba una relación algebraica. |
| Macro Rust | Mecanismo de expansión de código, justificable por una necesidad concreta de declaración o generación. No concede estatuto de primitiva ni sustituye la validación. |

Rust dispone de funciones, métodos y macros; las macros expanden código durante la compilación ([Rust Reference](https://doc.rust-lang.org/reference/macros.html)). La preferencia de trabajo es comenzar por funciones y métodos tipados y justificar las macros mediante una necesidad observada. Toda vía de construcción, incluida una expansión de macro, debe desembocar en las mismas restricciones de admisión del SV.

Una biblioteca base del SV puede organizar estas operaciones sin incorporar al núcleo parámetros médicos ni el vocabulario particular de un agente. Que una operación exista como función de Rust no la convierte automáticamente en una construcción del lenguaje fuente SV. Esta correspondencia corresponde al estudio de semántica e IR.

Los comentarios de implementación y documentación ES/EN siguen la política ya publicada. No activan por sí mismos perfiles lingüísticos de la DSL.

## 7. Trabajo que se incorpora al workflow

**BIS-01:** completar la concordancia de acepciones, fuentes, versiones y estatutos; distinguir lectura completa de piezas de lectura selectiva de secciones.

**BIS-02:** formular contratos y ejemplos adversariales para: constitución inicial; actividad técnica sin frame; ingreso legítimo al horizonte; reevaluación con frame; consulta de un frame existente; paridad celular y frame de arquitectura; representación definida, materializada y presentada; consejo documental y decisión formal SV.

**BIS-03:** determinar la sede de cada obligación en semántica, IR, núcleo, biblioteca base o contrato de dominio. Preservar la separación del anexo SEC5 entre Frame y traza probatoria completa; cualquier necesidad nueva de campos se deberá justificar, no añadir por comodidad.

**Banco de contraste:** incluir inmunología y ciberseguridad con el mismo criterio transversal. Los antecedentes pulmonar e IMMUNO-2 continúan separados de los dominios vigentes. La célula `(9,3)` conserva su condición mínima, sin transformarse en tamaño predeterminado.

**Catálogo posterior:** recoger únicamente causas delimitadas y comprobadas. Se proponen como familias a examinar la confusión de estatutos, la entrada sin horizonte competente, la falta de correspondencia gráfica, la representación desactualizada y la afirmación de decisión formal sin respaldo. Estas denominaciones no son todavía códigos normativos ni errores implementados.

## 8. Estado de cierre

La precisión del autor sobre el hecho factual y sus tres formas de representación —prosa, matemática e imagen— queda incorporada como criterio de trabajo. El contraste permite avanzar con una distinción fundada entre sucesos, reevaluaciones, frames, consultas y representaciones. Permanecen por constituir los contratos de integración del Bis, especialmente la inicialización, la relación entre frame celular y frame arquitectónico y el estatuto material de `frvis`.

S22 y BIS-01 continúan en ejecución. No se declara probado un runtime, no se modifica la IR ni se añade una primitiva. La revisión documental orienta el trabajo posterior y evita que una elección de programación resuelva implícitamente una cuestión semántica.
