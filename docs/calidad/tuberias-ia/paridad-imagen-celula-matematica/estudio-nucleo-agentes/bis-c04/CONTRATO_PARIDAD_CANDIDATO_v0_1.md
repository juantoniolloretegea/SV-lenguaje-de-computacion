# Contrato candidato de paridad posicional y convenio gráfico

**Versión 0.1 · S22 · BIS-02/C04 · 13 de septiembre de 2026 · Juan Antonio Lloret Egea y Watson**

## 1. Objeto, autoridad y alcance

Este contrato concreta la correspondencia entre el vector ternario ordenado de una célula y su poligonal polar cerrada. Se deriva de los Fundamentos algebraico-semánticos, §§2, 3 y 4.1, de la Frontera normativa v0, §3 y C.4, y de las obligaciones de constitución e identidad recibidas en el workflow y BIS-C03. Es un candidato para la decisión de sede de BIS-03; no introduce una primitiva ni modifica la IR, la gramática o el núcleo.

La denominación **BIS-C04** identifica un caso del estudio (p1+p3)-Bis. No designa una cuarta corrección de la adenda histórica C01–C03 de agosto de 2026.

El banco aísla la correspondencia geométrica de estados sintéticos previamente admitidos, bajo la precondición de identidad fijada en C03. La admisión algebraica, el soporte de tamaños, la constitución del dominio y la revisión exacta conservan sus comprobaciones propias. Preparar este banco no ejecuta ni acredita esas precondiciones.

## 2. Correspondencia normativa

Para N=b², b≥3, el estado es v=(v₁,…,vₙ), con vᵢ perteneciente a Σ={0,1,U}. La función visual ρ se fija así:

| Símbolo SV | Etiqueta de IR Rust | Radio canónico ρ | Lectura primaria de los fundamentos |
| --- | --- | --- | --- |
| 0 | `Zero` | 1 | Apto |
| 1 | `One` | 2 | No_Apto |
| U | `U` | 3 | Indeterminado |

El significado concreto de cada parámetro y su interpretación en un dominio permanecen declarados por su constitución. El radio no constituye una escala universal de gravedad; U conserva su estatuto semántico y no es el número tres.

Para la posición uno-basada i:

> θᵢ=2π(i−1)/N; Vᵢ=(ρ(vᵢ) cos θᵢ, ρ(vᵢ) sin θᵢ).

En coordenadas cartesianas habituales, el primer eje apunta a +x y el recorrido es antihorario. Se unen mediante segmentos rectos V₁→V₂→…→Vₙ→V₁. El cierre añade una arista, no una coordenada ternaria. La célula conserva N posiciones; no se almacena una supuesta posición N+1 ni se convierte en matriz b×b.

La paridad exige comprobar N, posición, parámetro asociado, símbolo, radio, ángulo, orden de vértices y conexiones. Las etiquetas de prueba P1,…,PN son identificadores posicionales sintéticos; no constituyen parámetros de inmunología, ciberseguridad ni otro dominio.

## 3. Símbolo, discriminante y radio son objetos distintos

En el corte inspeccionado, `Tri::as_u8()` devuelve 0, 1 y 2 para `Zero`, `One` y `U`. Es una codificación técnica explícita del enum. No es `VisualCode` ni el radio del polígono. Dibujar ese resultado directamente produciría radios 0, 1 y 2 e incumpliría el convenio visual.

El control visual debe aplicar la correspondencia normativa sobre el símbolo admitido y conservar el convenio exacto utilizado. No se deduce el radio de una clase de CNN, del orden de una leyenda, de un color, de un entero arbitrario ni de una conversión implícita. Que una fórmula accidental coincida hoy con la tabla no le concede autoridad para redefinirla.

## 4. Geometría exacta y soporte de dibujo

La lista ordenada de vértices polares, con N, ejes identificados y radios exactos, permite recuperar el vector: para cada posición se aplica la inversa de ρ. Ésta es la inyectividad que se invoca aquí. No se extiende a una silueta sin referencias, a un resumen por área, a una clasificación ni a una imagen rasterizada arbitraria.

Los conteos, el área o la clasificación global no prueban fidelidad posicional. Permutar dos coordenadas distintas conserva los conteos y la clasificación basada en ellos, pero cambia la asociación entre parámetro y valor. C04-20 entrega una descripción geométrica internamente coherente con ese otro vector para que el futuro observador deba cotejarla con el estado de origen, en lugar de aceptar su mera coherencia interna.

El formato experimental `BIS-C04-GEOMETRIA/0.1` describe geometría exacta de forma simbólica. Conserva el ángulo mediante el par entero `[i−1,N]`, medido en vueltas, y las coordenadas presentadas mediante coeficientes enteros de cos θᵢ, sin θᵢ y del término constante. Evita atribuir exactitud a aproximaciones decimales de seno y coseno.

Este formato fija una escritura única del banco: el numerador es i−1 y el denominador es N. Una fracción equivalente escrita de otra manera no cumple esa forma experimental; ello no significa que represente un ángulo matemáticamente distinto. Cualquier futura admisión de escrituras equivalentes debe declarar su normalización, sin reparación silenciosa.

El descriptor no es SVG, PNG, píxeles ni un tipo canónico de IR. El paso a un artefacto gráfico material requerirá su contrato de precisión, resolución, transformaciones, recorte, legibilidad y comprobación de salida. No se fija ahora una tolerancia numérica universal ni se afirma reversibilidad de la rasterización. Una proyección con pérdida debe declarar ese estatuto y no presentarse como equivalente exacta, conforme a C.4. Un archivo de metadatos correcto no demuestra por sí solo que el dibujo real sea correcto.

## 5. Transformación de presentación y permutación semántica

Cambiar la presentación exige una transformación declarada, identificada y comprobable, preservando el vínculo i↔parámetro↔símbolo. El banco utiliza sólo estas tres realizaciones matemáticas de prueba:

| Identificador | Transformación sobre el vértice cartesiano canónico | Alcance |
| --- | --- | --- |
| `identidad` | X=x; Y=y | Plano canónico. |
| `superior-horario` | X=y; Y=x | Reflexión respecto de la diagonal: primer eje arriba y recorrido horario en coordenadas cartesianas. |
| `pantalla-40` | X=160+40x; Y=160−40y | Conversión declarada a un sistema de pantalla con Y creciente hacia abajo; escala y centro sintéticos. |

Las tres son invertibles. La reflexión o el cambio de signo de un eje no son, por sí solos, cambios semánticos si conservan las correspondencias y su transformación está declarada. Aplicarlos silenciosamente bajo otra expectativa incumple el contrato. El centro 160 y la escala 40 pertenecen exclusivamente al banco, sin constituir un tamaño de imagen, una GUI o un soporte oficial.

La orientación histórica superior-horaria puede contrastarse así con los fundamentos. Esta relación matemática no acredita por sí sola la migración de un cascarón, su leyenda o su código. Los originales permanecen identificados en HALLAZGOS y la explicación V2.

Para cada vértice el banco declara:

> Xᵢ=aᵢ cos θᵢ+bᵢ sin θᵢ+cᵢ; Yᵢ=dᵢ cos θᵢ+eᵢ sin θᵢ+fᵢ.

Las listas `x_coef_cos_sin_constante` y `y_coef_cos_sin_constante` contienen respectivamente esos tres coeficientes. El radio ya está incorporado en los coeficientes de seno y coseno; no debe multiplicarse una segunda vez. El ángulo y el índice conservan su referencia canónica aunque el punto se presente transformado.

## 6. Entradas, expectativa y orden del montaje candidato

El conductor recibirá el estado exacto y la transformación solicitada desde el contexto de prueba comprometido. El descriptor presentado no elige su propio oráculo ni modifica el convenio. Las huellas del banco fijan las entradas; no conceden autoridad ni sustituyen el contraste del contenido observado.

Tras comprobar la integridad del banco, sus precondiciones y la recepción del formato, la precedencia experimental será:

1. Convenio y transformación solicitados, formato y dimensión declarada conformes.
2. Declaración de codificación compatible con la pretensión de equivalencia.
3. Leyenda completa y concordante.
4. Número de vértices, posiciones en orden y etiquetas de parámetros.
5. Símbolos cotejados con el vector independiente; radios y ángulos canónicos.
6. Coeficientes de las coordenadas presentadas acordes con la transformación solicitada.
7. Cierre y aristas exactos; sin tramos omitidos, sustituidos o interpolaciones curvas.

Los fallos previos se conservarán como tales. Un caso que ya se rechaza por radio no acredita una guarda posterior de coordenadas. Las etiquetas de salida del banco son experimentales: no añaden códigos al catálogo ni identifican E003 o E504 como guardas ejecutadas. La política global de diagnóstico se decidirá cuando corresponda.

## 7. Banco previo y criterio de terminación

El [banco previo](BANCO_PREVIO_v0_1.json) contiene veinte variantes: cinco positivas y quince negativas. Conserva cinco oráculos completos sobre 16, 25 y 49 posiciones. Los estados de 25 y 49 proceden de las fuentes sintéticas C01, copiadas sin cambios; el de 16 es idéntico al artefacto matemático C03.

Las variantes negativas cubren orden, símbolos, radios, leyenda, signo y denominador angular, transformación no concordante, cantidad de vértices, cierre, aristas, etiqueta de parámetro, declaración contradictoria de pérdida y sustitución por otro vector con los mismos conteos. C04-19 contrasta una declaración incompatible; no es todavía una prueba de pérdida efectiva en rasterización.

El banco sigue sin ejecutar. Antes de atribuirle evidencia será necesario decidir la sede, realizar el montaje, conservar sus salidas reales y comprobar la sensibilidad del observador frente a omisión de posiciones, radio directo desde el discriminante, uso automático de N=9, aceptación por conteos, cierre omitido y transformación aplicada sin declaración. Los esperados no se obtendrán del renderizador probado. La comparación de un descriptor con otro no acreditará por sí sola píxeles, legibilidad ni consumo por humano o IA.

## 8. Encaje en la realización existente y relevo

La Frontera distingue `VisualCode`, `Role` e `Influence`, aunque el corpus reutilice el símbolo ρ. La inspección del módulo `Tri` y de las variantes de IR no identifica una entrada material que imponga por sí sola este contrato gráfico completo. El nombre `Projection` de una operación existente tampoco basta para acreditar `VisualCode`, inyectividad o producción de una imagen. Se conserva esta conclusión dentro del alcance estático documentado, sin convertirla en una afirmación sobre todo software del ecosistema.

BIS-03 decidirá cómo representar e imponer las obligaciones que faltan y cómo enlazarlas con las ligaduras ya existentes y con C03. La representación auxiliar y sus bibliotecas permanecerán subordinadas al contrato algebraico. Si se incorpora código Rust, la política de comentarios ES/EN se aplicará al código afectado; no cambia los perfiles de la DSL.

El siguiente objeto es BIS-C05: identidad de los bytes efectivamente consumidos y concordancia con la revisión solicitada. S22 continúa en BIS-02; la secuencia de S24 conserva el análisis e instalación de la GUI para después del Bis, el catálogo y el cierre de fase.
