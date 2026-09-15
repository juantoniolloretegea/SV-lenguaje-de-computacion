# Acta 002 · Recepción del cotejo de identidad y del contrato candidato de contenido de leyenda R06

**Fecha:** 15 de septiembre de 2026. **Seguimiento:** S22; recepción documental con reparos. **Estado:** contrato candidato, sin autorización de implementación ni de campaña.

## 1. Objeto y cortes

Se reciben las aportaciones relativas a la identidad de los testigos raster y al contrato LEYENDA-CONTENIDO/2. La recepción distingue la igualdad de bytes, el diseño del observador y la comprobación del contenido visible.

- Lenguaje: `e9e4a359bd3d54e2e397c4747f11b7c69d6fbb2f`.
- Depósito documental: `SVperitus-dataset@18e7178e6ec7efbb10863f4d081422f94ef163c2`, rama `dominio-inmunologia`; antecedente `ed6ade5e2285ecf3d058c20c9e074feb7a5066b6`.
- Entradas experimentales de origen: laboratorio `86441ad4d375e31737dfcead0b1fd9cd52161883`, RETP-241.

Se han consultado AGENTS.md, los Pilares de 05/09, el acta de perfiles, contratos y ensamblaje de 06/09, el acta de transición desde OP-IMM-001 con sus relevos, el Acta 001 y el registro vigente de Sucesos en el corte del Lenguaje. Los antecedentes conservan sus fechas y alcances. La ubicación del depósito no modifica el dominio de Inmunología.

## 2. Evidencia recibida y verificación

El [fuente completo del cotejo auxiliar](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/18e7178e6ec7efbb10863f4d081422f94ef163c2/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/cotejo_identidad_sha256.rs) contiene lectura de ocho entradas, cálculo de SHA-256, comparaciones de bytes y salida no nula ante discrepancias. El [registro de la ejecución 02](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/18e7178e6ec7efbb10863f4d081422f94ef163c2/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/COTEJO_IDENTIDAD_REEJECUCION_02.txt) declara compilación y ejecución con Rust 1.98.0, retorno 0, tres vectores concordantes y ocho huellas coincidentes con sus referencias. Declara asimismo igualdad entre R01 y la muestra PNG, igualdad entre la entrada SVG y su muestra, y diferencias entre R01 y R06.

En esta revisión se ha calculado en Rust 1.98.0 la huella del fuente recuperado: 9053 bytes y SHA-256 `6698383e5e768cf32da6b0d79f283c550810cc6c0c428f6bfc816679ec6098df`, coincidente con el registro recibido. Este cotejo puntual reutiliza la función SHA-256 del fuente; no constituye validación criptográfica independiente del algoritmo ni reproducción de las ocho identidades de entrada.

La comparación entre los dos cortes del depósito identifica exclusivamente tres archivos añadidos, sin modificación de los cuatro archivos anteriores. La revisión recibe la ejecución 02 como evidencia declarada y no le atribuye una repetición independiente completa. Las huellas R06 siguen sin constar en el manifiesto histórico IDENTIDADES.txt; se conserva esa distinción de procedencia.

## 3. Avance del contrato candidato

[LEYENDA-CONTENIDO/2](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/18e7178e6ec7efbb10863f4d081422f94ef163c2/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/CONTRATO_CANDIDATO_LEYENDA_CONTENIDO_2.md) añade medidas de residuo absoluto y relativo, separa cualificación de evaluación, especifica un positivo reservado E1 y casos adversariales, distingue fallos de decodificación e identifica dependencias compartidas.

Son avances de especificación. Los parámetros no están cualificados, E1 no tiene testigo material, el reconocedor no está implementado y no se ha ejecutado su evaluación.

## 4. Reparos que impiden aprobar el contrato para su implementación

| Identificador local | Observación documental | Corrección exigida |
|---|---|---|
| LC2-01 | La máscara explicada se dilata un píxel y admite umbrales de residuo. Una marca dentro de esa máscara puede quedar sin residuo; una marca exterior puede quedar bajo ambos umbrales. Sin embargo, E2 prescribe rechazo para un trazo o glifo ajeno sin delimitar esas condiciones. | Definir qué tolerancia pertenece al perfil, cómo se distingue de una adición inadmisible y qué esperados corresponden a las fronteras de tolerancia. Concordar propiedad, fórmula y casos; no declarar demostrado un falso positivo experimental. |
| LC2-02 | El paso C.7 acepta una plantilla de separador o un hueco de tinta no superior a w_sep. No delimita de forma inequívoca cuándo el separador puede faltar ni la geometría admisible de ese hueco. | Fijar un único régimen de separadores y sus condiciones medibles, con esperados para ausencia, sustitución y contacto de cláusulas. La revisión no autoriza cambiar silenciosamente el perfil. |
| LC2-03 | El fondo blanco es condición de FUERA_DE_PERFIL en A, pero el algoritmo C pasa de la decodificación a la banda sin una comprobación explícita del fondo. | Definir la región y el criterio de fondo, e insertar su comprobación después de decodificar y antes de decidir sobre la leyenda. Precisar el tratamiento de transparencia y su relación con el mapa de tinta. |

Los identificadores de esta tabla son locales a esta acta; no amplían el catálogo de diagnósticos del Lenguaje.

## 5. Resultado y continuidad

Se recibe el cotejo auxiliar y la nueva especificación como documentación trazable. LEYENDA-CONTENIDO/2 permanece candidata con los reparos de §4. La recepción no acredita lectura semántica de R06, paridad visual completa, protección del captor ni cierre de Bis.

**Siguiente acción:** corregir exclusivamente la especificación indicada, conservando las versiones anteriores. La materialización de E1, la cualificación de parámetros y la implementación requieren el encargo posterior correspondiente. No repetir RETP-241 ni las campañas SVG por esta recepción.

S22 y S26 mantienen su estado en ejecución y sus pendientes. Se conserva la cancelación de la secuencia automática de GUI, así como la revisión reforzada previa al retorno indicado en el Acta 001. No se activan rutas de Inmunología, Qwen ni la batería CYB.

Esta acta registra aportaciones técnicas y límites de evidencia; no incorpora incidencias de coordinación. No adopta una nueva decisión de diseño material ni altera los asientos RETP históricos.

## 6. Recepción de LEYENDA-CONTENIDO/3 · 15 de septiembre de 2026

**Corte receptor:** Lenguaje `5e6edf78d8714bbbd57253991e48ba993e79e7a8`. **Depósito examinado:** `SVperitus-dataset@b73c2b28f6a8899b2024eb3f9dee975bc8037016`, antecedente `18e7178e6ec7efbb10863f4d081422f94ef163c2`. Se añade esta recepción a la presente acta sin modificar las secciones anteriores ni su numeración.

### 6.1. Identidad y alcance del depósito

Los adjuntos recibidos se cotejaron en Rust 1.98.0 con los blobs del directorio remoto fijado:

| Archivo | Bytes | Blob Git | SHA-256 |
|---|---:|---|---|
| CONTRATO_CANDIDATO_LEYENDA_CONTENIDO_3.md | 11201 | `3da8e1d6e03a9f09ba4ffdbb0e36a5de7b920fa1` | `9e9937dd11ab74c4ccab2b02f958668daa9677344bce3de5a19ac2d95e858105` |
| SUBSANACION_ENTREGA_03.md | 2940 | `6c6e67d42d6fae857a6fe1789772a682a602d7b3` | `88ca1bb77524fcf09842082a47db9839dd4417c7dd61a037e0aadfb276ff5e3c` |

La [comparación de los cortes](https://github.com/juantoniolloretegea/SVperitus-dataset/compare/18e7178e6ec7efbb10863f4d081422f94ef163c2...b73c2b28f6a8899b2024eb3f9dee975bc8037016) muestra exclusivamente estos dos archivos añadidos. Los siete anteriores se conservan. La evidencia acredita identidad de los documentos, no ejecución de un reconocedor.

### 6.2. Dictamen documental de los reparos

**LC2-02:** se recibe la subsanación del régimen de separadores en alcance documental: dos separadores obligatorios, orden y distancias especificados, con esperados para ausencia, sustitución y contacto. La cadena del [SVG histórico](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e9e4a359bd3d54e2e397c4747f11b7c69d6fbb2f/docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis04-extension-perfiles-recursos-v0_1/MUESTRA_SVG_PRODUCIDA.svg) contiene los dos separadores. No se acredita cualificación numérica ni comportamiento material.

**LC2-01 permanece pendiente.** La sección B presenta tres dificultades concretas:

1. `H_T` se define como vecindad-4 de `M_T` menos `M_T`. Por construcción, todo píxel de `H_T` es 4-adyacente a `M_T`. La condición posterior de una marca contenida en ese anillo pero no 4-adyacente a `M_T`, utilizada para justificar E2b, no puede satisfacerse bajo esas mismas definiciones. Debe distinguirse, si corresponde, la adyacencia a la máscara geométrica de la adyacencia a tinta observada, sin tratarlas como equivalentes.
2. Se definen componentes de `M_B`, pero para el polvo se usan componentes del residuo `M_B` menos las máscaras. La primera prohibición se refiere a cualquier componente grande no contenida en las máscaras: una componente de tinta aceptada unida a un píxel de halo puede cumplirla. La tolerancia de halo y esa prohibición pueden asignar decisiones opuestas. Debe fijarse el conjunto exacto sobre el que se calcula cada componente y la precedencia.
3. El contrato usa `S`, `R_abs` y `R_rel` sin recuperar sus fórmulas de la versión /2. También remite a la tabla de independencia anterior sin incorporarla. No cumple la exigencia de texto autónomo. Deben explicitarse las fórmulas, sus dominios, las máscaras y el tratamiento del halo en cada medida.

**LC2-03 permanece pendiente.** La sección C.4 introduce una etapa de fondo, pero:

- Declara `F=L\\B` y después solo impone clasificación alfa/RGB en todo el lienzo y un recuento en el marco. Cualquier píxel opaco no blanco se clasifica como tinta; así, el criterio no impone por sí mismo blancura en el resto de la región F. Debe especificarse qué región se exige blanca y cómo se distingue de las zonas autorizadas de dibujo, sin recurrir al color observado para justificar la propia región.
- Acepta transparencia total como blanco mientras declara un perfil opaco heredado. Esa composición necesita una regla explícita y fundamentada en el perfil. Ocho bits de profundidad no equivalen por sí solos a opacidad. Si el perfil exige fondo opaco, no procede ampliarlo silenciosamente.
- E15/E16 deben cubrir inequívocamente las decisiones adoptadas para transparencia y fondo, preservando la precedencia de fallos de decodificación.

Son reparos de especificación obtenidos por lectura y razonamiento sobre las definiciones. No se presentan como falsos positivos ni rechazos observados en una campaña.

### 6.3. Evidencia del cotejo receptor

- [Fuente completo](COTEJO_RECEPCION_LEYENDA_03.rs): 7960 bytes; SHA-256 `0375a16d8e904b738e0f297ad1fa951b1e00c477d73db6a469296892fb237cd1`.
- [Comandos y salida](COTEJO_RECEPCION_LEYENDA_03_SALIDA.txt): 586 bytes; SHA-256 `7152a1427240a200a57cc52e5798317134b010e00113faddb23ce3baddb4c887`.

Compilación y ejecución con Rust 1.98.0, retornos 0. Se reutilizan funciones históricas de huella; no se presenta validación criptográfica independiente. El cotejo receptor no acredita retrospectivamente el código o los comandos de otra ejecución que no se hayan entregado.

### 6.4. Continuidad y conservación de evidencia

LEYENDA-CONTENIDO/3 permanece candidata, sin autorización de implementación. La siguiente subsanación debe limitarse a LC2-01, LC2-03 y la autonomía del documento, conservando la solución documental de LC2-02. E1 permanece sin testigo material; no se cualifican parámetros ni se repiten campañas. S22 y S26 conservan el estado en ejecución.

Las entregas posteriores deberán conservar en una sede expresamente autorizada el texto íntegro, el código auxiliar realmente utilizado, las versiones y comandos, las salidas completas y un manifiesto de tamaños, huellas y procedencia. Los resultados históricos irrecuperables no se reconstruyen ni se presentan como originales. Los contenidos reservados mantienen su custodia privada; una referencia a una sede temporal no constituye conservación verificable.

Esta recepción actualiza S22 y su historial, así como el Léame primero. No altera S31, RETP, HTML, reservas, código productivo ni el rumbo vigente.

## 7. Recepción de LEYENDA-CONTENIDO/4 · 15 de septiembre de 2026

**Corte receptor:** Lenguaje `4ab5c0f3b13cecaaf7eba45298d9aa5f1a709dba`. **Depósito examinado:** `SVperitus-dataset@ea4982398a42eba362f7bc7e9e00f5268f3058c9`, antecedente `b73c2b28f6a8899b2024eb3f9dee975bc8037016`. Esta recepción conserva íntegramente las secciones anteriores.

### 7.1. Recepción e identidad

Se recuperaron del corte fijado las cinco piezas siguientes. Sus bytes se cotejaron en Rust 1.98.0 con tamaños y blobs del directorio remoto y con las cuatro huellas comunicadas. La huella del manifiesto es una observación de esta recepción.

| Archivo | Bytes | Blob Git | SHA-256 |
|---|---:|---|---|
| [CONTRATO_CANDIDATO_LEYENDA_CONTENIDO_4.md](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/ea4982398a42eba362f7bc7e9e00f5268f3058c9/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/CONTRATO_CANDIDATO_LEYENDA_CONTENIDO_4.md) | 12404 | `519bb717b5362b900f275d0aad151a7b67bc9b2e` | `a5026829c381c2e0b47c89e78310887677fd79b8089fffcd78df10bc11e9b03d` |
| [SUBSANACION_ENTREGA_04.md](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/ea4982398a42eba362f7bc7e9e00f5268f3058c9/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/SUBSANACION_ENTREGA_04.md) | 3422 | `718799889b7a1aeff6510b73a0fa03ed025f082f` | `a5543add6e674dcc5e1f129e38d4141769a48b64c38ac98ce8c13fdd0e45bcb1` |
| [COTEJO_ENTREGA_04.rs](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/ea4982398a42eba362f7bc7e9e00f5268f3058c9/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/COTEJO_ENTREGA_04.rs) | 4274 | `879e044649cbad9d0b06dca87967e00c0e41ea3e` | `7bc2345df02b992bd16b24a522f001d70be166959f3986a1b5392dc6916e72df` |
| [COTEJO_ENTREGA_04_SALIDA.txt](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/ea4982398a42eba362f7bc7e9e00f5268f3058c9/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/COTEJO_ENTREGA_04_SALIDA.txt) | 1096 | `27293f4e778ade4b0583dc1b59c52cd64b2946f5` | `67e379d09c31bcb99409592c9ad8e7434268e40d6687d51aa524551e853abc40` |
| [MANIFIESTO_ENTREGA_04.tsv](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/ea4982398a42eba362f7bc7e9e00f5268f3058c9/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/MANIFIESTO_ENTREGA_04.tsv) | 1537 | `8c2302d032d0d460e091a70b09751d9e7c197f8b` | `dab91aaf18e8885ce86f0a8ffe12b78c389f52bb12ca0d089bfddb130ba230be` |

La [comparación completa de cortes](https://github.com/juantoniolloretegea/SVperitus-dataset/compare/b73c2b28f6a8899b2024eb3f9dee975bc8037016...ea4982398a42eba362f7bc7e9e00f5268f3058c9) identifica exclusivamente esas cinco incorporaciones. El cotejo de ambos directorios confirma que los nueve archivos anteriores conservan tamaño y blob. No se recibió una segunda serie de adjuntos en esta recepción: se acredita la identidad del depósito recuperado, sin atribuir una comparación con adjuntos ausentes.

### 7.2. Dictamen documental

- **LC2-02:** se conserva la subsanación documental ya recibida: dos separadores obligatorios y condiciones de orden y separación.
- **LC2-03:** se recibe su subsanación documental. G, B y W son regiones geométricas; la blancura se comprueba en W; cualquier alfa distinta de 255 queda fuera del perfil antes del control de fondo. Las decisiones de transparencia y fondo se subordinan a la precedencia explícita de A. No se ha comprobado materialmente el alfa de los testigos históricos.
- **LC2-01:** quedan subsanadas las tres dificultades de §6.2 sobre distinción de máscaras y tinta, componentes calculadas exclusivamente en el residuo y restitución de fórmulas. E2b ya no exige una condición geométrica imposible. Permanece una precisión de cierre en E2c: sus condiciones enumeradas no excluyen la plantilla no asignada que B.6.2 considera adición inadmisible. La tabla no debe permitir que la tolerancia residual baste para declarar conformidad. Debe expresar que concurren las condiciones generales de perfil, tinta suficiente, asignación inequívoca de las tres parejas del convenio y dos separadores, y que no se cumple ninguna causa de B.6, incluida B.6.2. Es una insuficiencia de la condición escrita, no un falso positivo observado.
- **Autonomía:** las fórmulas y la tabla de independencia están incorporadas en /4.

El alcance continúa siendo documental. No se acreditan parámetros cualificados, implementación, independencia material del códec ni un segundo testigo E1.

### 7.3. Alcance de la evidencia de ejecución y procedencia

El fuente `COTEJO_ENTREGA_04.rs` comprueba dos vectores y calcula huellas de los archivos recibidos como argumentos. No compara esas huellas con referencias esperadas. Por tanto, su retorno 0 por sí solo no acredita igualdad entre dos descargas ni concordancia con un manifiesto. Las concordancias de esta recepción se sostienen en el cotejo receptor explícito de §7.4.

La salida depositada consigna `CMD=/tmp/cotejo_entrega_04 CONTRATO SUBSANACION RS`, mientras las líneas ARCHIVO contienen rutas absolutas distintas. El fuente imprime literalmente cada argumento. La línea CMD, tal como está escrita, no es la invocación literal que produciría esas rutas. Debe identificarse como abreviada si ese fue su uso; si no se conserva la invocación original, debe declararse esa ausencia. No se reconstruirá una ejecución histórica ni se repetirá únicamente para reemplazar su registro.

La tabla de evidencia histórica de SUBSANACION_ENTREGA_04 presenta la fila R01/R06 con campos desplazados: el lugar del commit contiene «miembros citados» y el de ruta contiene huellas abreviadas. El manifiesto conserva las huellas completas, pero no identifica la ruta completa del paquete y sus miembros. La adenda deberá completar sede, commit íntegro, ruta del paquete, miembro y huella de cada testigo a partir de la custodia ya fijada, sin publicar sus contenidos. También deberá enlazar por corte y ruta completos el cotejo receptor /3, ya conservado en este repositorio; no es necesario duplicarlo.

### 7.4. Evidencia receptora conservada

- [Fuente receptor íntegro](COTEJO_RECEPCION_LEYENDA_04.rs): 8744 bytes; SHA-256 `1e23c6d8e2585f889cd6d58544a071cc936593bda55141ef98bf35ee49700735`.
- [Versión, comandos y salida receptora](COTEJO_RECEPCION_LEYENDA_04_SALIDA.txt): 1784 bytes; SHA-256 `4588d724f9a81f194d93e7a6f616b4e1374efc5ce6818a4c2413c9148b3a2bf3`.

Compilación y ejecución con Rust 1.98.0, retornos 0; cinco identidades concordantes. Se reutilizan funciones históricas de huella: no se presenta validación criptográfica independiente. Este cotejo no ejecuta el reconocedor ni valida retrospectivamente una ejecución distinta.

### 7.5. Cierre acotado y siguiente entrega

Se requiere únicamente una adenda documental vinculada a /4, con prioridad expresa para la precisión de E2c, la calificación fiel del registro de ejecución y las referencias completas de custodia. No se exige reescribir el contrato como /5 ni repetir comprobaciones ya acreditadas.

La siguiente entrega se limita a dos archivos nuevos en el mismo directorio autorizado: `ADENDA_CIERRE_DOCUMENTAL_ENTREGA_04.md` y `MANIFIESTO_ADENDA_ENTREGA_04.tsv`. El manifiesto consignará tamaño y huella de la adenda y referencias completas a las evidencias ya depositadas; no necesita incluir su propia huella. Se conservarán los catorce archivos existentes. Toda herramienta nueva que efectivamente se utilice deberá quedar conservada en sede autorizada; no se necesita crear una herramienta para esta adenda.

S22 permanece en ejecución y el contrato candidato. Esta recepción no modifica S26, S31, RETP, HTML, reservas ni la numeración de actas. No autoriza implementación, síntesis de plantillas, campañas, ajuste de parámetros ni materialización de E1. Solo una discrepancia material nueva, sustentada con evidencia, justifica ampliar este alcance.

## 8. Control de continuidad documental · 15 de septiembre de 2026

**Corte examinado:** Lenguaje `146cb30e3cece036947081453d7e0982ca03a1cb`. **Objeto:** conservación de la sede de continuación y su enlace con el registro vigente. Rigen AGENTS.md en su alcance aplicable, las reglas de Sucesos SV, S29, el Acta 001 y las recepciones anteriores de esta acta. No se interviene en código productivo, contratos de dominio, semántica ni IR.

### 8.1. Comprobaciones efectuadas

Control auxiliar escrito y ejecutado en Rust 1.98.0, sin dependencias externas:

- **21 identidades concordantes:** los 17 archivos de la sede de continuación, incluido el HTML, y los cuatro documentos de registro y continuidad relacionados. Tamaño, blob Git y SHA-256 observados constan en la salida. No se detecta una ausencia o alteración respecto de ese corte.
- **32 sucesos y 125 instantáneas:** serie S0–S31 sin identificadores duplicados ni saltos; revisiones consecutivas por suceso; cada fila vigente coincide íntegramente con su última instantánea; campos del Markdown concordantes con el CSV.
- **92 enlaces Markdown relativos:** destinos existentes en el árbol del corte. Se decodifican los caracteres escapados de las rutas. No se comprueban las anclas de sección. Los 17 enlaces externos encontrados no se descargaron en este control.
- **Cinco fuentes históricas de cotejo:** Rust con `forbid(unsafe_code)`, módulos de apoyo incluidos y sin invocaciones de subprocesos en el código revisado. Los tres cotejos de contenedores utilizan flate2 según lo declarado; su presencia no es una dependencia oculta. No se ejecutan programas Python del material custodiado.

Estas comprobaciones no son una auditoría de todos los repositorios, de todas las herramientas pasadas ni de la disponibilidad futura de los servicios. No se presume sincronización de las copias históricas del laboratorio.

### 8.2. Mejoras documentales

El índice inicial enlazaba únicamente el Acta 001 y el mapa. Se incorporan accesos al Acta 002, al parte S31, a los registros y a cada fuente y salida conservadas. Se añade una remisión en el Acta 001 a la recuperación posterior de sus dos archivos pendientes y se destaca el estado vigente en el Léame, conservando las anotaciones históricas.

Se precisa el alcance de los registros de ejecución: los resúmenes de resultados o comandos parametrizados no se consideran transcripciones literales completas. En particular, la recepción del ZIP S6 conserva una orden con marcadores de biblioteca y dependencias; el complemento de contenido conserva resultados y retornos, con instrucciones de reproducción en el parte. No se inventan invocaciones históricas para completar esas piezas.

No se ha identificado texto nuevo que amplíe una cualificación material o autorice una campaña. Los errores y limitaciones propios de las herramientas de asistencia no se incorporan como sucesos del proyecto. Se documentan las propiedades observables de las evidencias y sus límites.

### 8.3. Evidencia del control

- [Fuente](CONTROL_CONTINUIDAD.rs): 12121 bytes; SHA-256 `dfbf33b4e02539854efb41cf4ddaf45f3084cd971450f6346b19df466b417c97`.
- [Entradas](ENTRADAS_CONTROL_CONTINUIDAD.tsv): 3777 bytes; SHA-256 `1bab8d56cbbaf298358adf11c4f1ad1e72d00635c5bea45ee3eb194ec39f58bb`.
- [Comandos y salida](CONTROL_CONTINUIDAD_SALIDA.txt): 5720 bytes; SHA-256 `d94de0b2353263a6e93dbf3066f88bf12f78068167c431c82bb046875912430a`.

La primera columna del manifiesto describe rutas de ejecución, no sedes de custodia. La segunda identifica la ruta canónica dentro del corte citado; junto al tamaño y al blob permite recuperar la entrada aunque desaparezca la copia local. Las instrucciones de reproducción figuran en el [índice](inicio.md). No se publica contenido reservado.

### 8.4. Continuidad

Control documental concluido en su alcance. Se añade una instantánea de S22; S22 y S26 permanecen en ejecución, S31 finalizado. La adenda limitada de §7 sigue siendo el siguiente paso. No se renumeran actas, se borran antecedentes ni se modifican el HTML, las reservas o el código productivo. Las nuevas verificaciones se realizan en Rust; un uso excepcional de Python debe ser inocuo y estar justificado antes de ejecutarse.

## 9. Recepción de la adenda y cierre de la subsanación documental · 15 de septiembre de 2026

**Corte receptor:** Lenguaje `57cb5166bc4c6f2dc8d4e698e3e2990c29196723`. **Depósito recibido:** `SVperitus-dataset@23fcae974badc52360493e62346dda21a5976a78`, rama `dominio-inmunologia`, directorio `dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/`. Antecedente del depósito: `ea4982398a42eba362f7bc7e9e00f5268f3058c9`.

### 9.1. Conservación e identidad

| Archivo depositado | Bytes | Blob Git | SHA-256 |
|---|---:|---|---|
| [Adenda](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/23fcae974badc52360493e62346dda21a5976a78/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/ADENDA_CIERRE_DOCUMENTAL_ENTREGA_04.md) | 4381 | `59d01d5b6cd6957c1e4ff93ea31572f356600482` | `b54525863ba58c7c5c997ba469c9d3b54131cb10cd035c65067c684a11f8aba1` |
| [Manifiesto](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/23fcae974badc52360493e62346dda21a5976a78/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/MANIFIESTO_ADENDA_ENTREGA_04.tsv) | 2215 | `6622ccfdf7b996c1b8e4dca65b19d92353e0cb82` | `e7d863935229f3855ba17c2e9b432c3afade80808bf5ed7b97ec08212ca97f0b` |

Los archivos se recuperaron del repositorio y se cotejaron en Rust 1.98.0. La huella del manifiesto es observación de esta recepción, no declaración previa del emisor. La [comparación de cortes](https://github.com/juantoniolloretegea/SVperitus-dataset/compare/ea4982398a42eba362f7bc7e9e00f5268f3058c9...23fcae974badc52360493e62346dda21a5976a78) presenta únicamente esas dos incorporaciones. Los directorios de ambos cortes confirman que los catorce archivos anteriores conservan tamaño y blob. La sede contiene dieciséis archivos.

### 9.2. Recepción de los tres puntos delimitados

1. **E2c:** se recibe la exigencia conjunta de las condiciones generales del contrato y ausencia de todas las causas de B.6, incluida B.6.2. Los umbrales residuales no bastan para emitir conformidad. La adenda declara prioridad sobre los pasajes que precisa. Con esta recepción se concluye la subsanación documental acotada de LC2-01; LC2-02 y LC2-03 conservan las subsanaciones ya recibidas.
2. **Registro:** se recibe la declaración expresa de que la línea CMD era abreviada y la invocación original completa no se conserva. No se reconstruye ni se sustituye por una ejecución presentada como histórica. La conservación del registro y sus huellas no transforma su contenido declarado en una observación independiente de aquella ejecución.
3. **Custodia:** quedan identificados repositorio, corte, ruta del paquete y nombres y huellas declarados de R01/R06. Los metadatos del directorio del laboratorio en `86441ad4d375e31737dfcead0b1fd9cd52161883` confirman la presencia de `EVIDENCIA_RASTER_CAPTOR.tar.gz`: 4308172 bytes y blob `814619348801577afc28b8fb5be45f73e518d46c`. No se ha vuelto a descargar ni descomprimir ese paquete en esta recepción. Las huellas de miembros se conservan con su procedencia histórica; la de R06 no se atribuye al manifiesto histórico IDENTIDADES.txt. El cotejo receptor /3 dispone de ruta y corte completos en el Lenguaje, sin duplicación de su fuente.

### 9.3. Precisión receptora del alcance de igualdad

La última frase de §2 de la adenda atribuye al cotejo receptor de Acta 002 §7.4 la igualdad entre dos descargas. **Esa atribución no se recibe.** El §7.1 y el fuente conservado acreditan la comparación de archivos recuperados con tamaños, blobs y huellas fijados; no comparan dos descargas históricas ni adjuntos ausentes.

A efectos del expediente se aplica esta redacción: **«El cotejo receptor de Acta 002 §7.4 acredita identidad de los archivos recuperados respecto de las referencias fijadas. No acredita la igualdad entre dos descargas históricas.»**

Esta precisión prevalece, en la recepción canónica, sobre aquella frase. No modifica los bytes del documento recibido ni exige reconstruir la ejecución. La adenda se conserva íntegra y la discrepancia de alcance queda resuelta expresamente en esta acta.

### 9.4. Sedes y evidencia de recepción

- **Origen documental conservado:** los dieciséis archivos del [depósito fijado](https://github.com/juantoniolloretegea/SVperitus-dataset/tree/23fcae974badc52360493e62346dda21a5976a78/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/). Cada versión conserva su ruta y su commit.
- **Recepción y seguimiento:** esta acta, el índice de continuación y Sucesos SV del Lenguaje. La referencia canónica permite recuperar las piezas de origen cuando deban incorporarse a una realización de laboratorio; no se afirma que ya se hayan copiado allí.
- **Testigos reservados:** el paquete mantiene su sede y acceso privados. La recepción pública no contiene los PNG ni el archivo comprimido.
- [Fuente del cotejo receptor](COTEJO_RECEPCION_ADENDA_04.rs): 8293 bytes; SHA-256 `f8c4b6560d85de750d9b1ac251a814a5c8b7dc0c5f5a03568403dfd50891caa9`.
- [Versión, comandos y salida](COTEJO_RECEPCION_ADENDA_04_SALIDA.txt): 1333 bytes; SHA-256 `7123c807cba9ee3b146462eff0897fea35aace6e77ba47d2fd8e0a6d194810ea`.

Compilación y ejecución con Rust 1.98.0, retornos 0. Se reutilizan las funciones históricas de identidad; no se presenta validación criptográfica independiente. No se ejecutó Python ni contenido de los paquetes.

### 9.5. Estado y relevo

**Subsanación documental acotada: finalizada.** La referencia de trabajo es LEYENDA-CONTENIDO/4 más su adenda, con la precisión receptora de §9.3. No se requiere otra entrega de corrección para esta ronda.

El contrato permanece candidato. Continúan sin acreditarse la implementación del reconocedor, la cualificación de parámetros, E1 material y la independencia experimental del códec. S22 y S26 permanecen en ejecución; S31 conserva su cierre. Antes de cualquier trabajo material debe delimitarse su encargo, entradas, oráculos, recursos y condiciones de aceptación conforme al rumbo vigente. Esta recepción no autoriza por sí sola implementación ni campañas.

## 10. Recepción de realización-leyenda-01: entrega incompleta

Fecha de recepción: 2026-09-15. Corte de origen: `5e3c29bf85fd49ba44c43c894132217f7c585f2d`. Este apartado continúa el seguimiento material; no modifica el cierre documental acotado de §9 ni convierte el contrato candidato en instrumento cualificado.

### 10.1. Depósito y alcance del cotejo

La [comparación desde el corte de referencia](https://github.com/juantoniolloretegea/SVperitus-dataset/compare/23fcae974badc52360493e62346dda21a5976a78...5e3c29bf85fd49ba44c43c894132217f7c585f2d) contiene únicamente diecisiete incorporaciones, todas en [realizacion-leyenda-01](https://github.com/juantoniolloretegea/SVperitus-dataset/tree/5e3c29bf85fd49ba44c43c894132217f7c585f2d/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/realizacion-leyenda-01/). Los dieciséis documentos anteriores se conservan.

Se recuperaron los diecisiete archivos de texto del árbol fijado. El cotejo receptor en Rust 1.98.0 confirma tamaño y blob Git de los diecisiete y calcula sus SHA-256. Frente a las diecinueve filas del [manifiesto de origen](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/5e3c29bf85fd49ba44c43c894132217f7c585f2d/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/realizacion-leyenda-01/MANIFIESTO.tsv), catorce concuerdan, dos discrepan y tres corresponden a archivos ausentes. El manifiesto no se incluye a sí mismo.

| Archivo ausente del árbol | Bytes declarados, no observados | SHA-256 declarada, no observada |
|---|---:|---|
| src/main.rs | 5942 | `1223094facbd03baea7fa6870d49b7c672cae3f423c38e24a3e78646707da56b` |
| src/png_lectura.rs | 6392 | `4d9cbed4a374d698a97f5f6114b78272b47745f05a7e3085972146f68f57ce90` |
| src/reconocimiento.rs | 7800 | `287407359882e5e1bb3f31b8e76fda03f04e3b1618538186e63654e76a26ae35` |

`src/lib.rs` declara los módulos ausentes. No se acredita una compilación reproducible del árbol publicado. El [registro de compilación recibido](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/5e3c29bf85fd49ba44c43c894132217f7c585f2d/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/realizacion-leyenda-01/evidencias/COMPILACION.txt) conserva una compilación local declarada con retorno cero; la recepción verifica la identidad de ese registro, no reproduce ni acredita retrospectivamente aquella ejecución.

| Archivo discrepante | Bytes declarados / observados | SHA-256 observada |
|---|---:|---|
| PROTOCOLO_CUALIFICACION.md | 4407 / 4379 | `6233cde189816fcf3323ee2827a958ae6aa774b21a34bd3b5c832c202b6aefaa` |
| src/regiones.rs | 774 / 775 | `044d2953259e0ff7b4b32ff15251b634b44f8589541ee6aec6a3bc9d95b0b398` |

Las huellas declaradas y observadas completas figuran en la salida receptora. No se dispone de las variantes locales para atribuir la diferencia a tipografía, salto de línea o equivalencia de contenido. La declaración de que rige el protocolo depositado no subsana la discordancia de su manifiesto.

### 10.2. Condiciones pendientes antes de Q1/Q2

Esta lectura estática acotada no constituye auditoría integral del reconocedor ni ejecución de testigos.

1. **Precompromiso de parámetros.** El [protocolo](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/5e3c29bf85fd49ba44c43c894132217f7c585f2d/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/realizacion-leyenda-01/PROTOCOLO_CUALIFICACION.md) enumera nueve parámetros con tres valores y uno con dos: producto cartesiano de 39.366 combinaciones. El límite de 200 por tanda no determina la selección, el orden ni la condición de terminación. Debe fijarse antes de ensayar una selección finita justificada, con reglas de aceptación, desempate y parada; no se autoriza ampliar automáticamente la campaña.
2. **Atribución por puntuación.** [DECISIONES_PENDIENTES.md](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/5e3c29bf85fd49ba44c43c894132217f7c585f2d/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/realizacion-leyenda-01/DECISIONES_PENDIENTES.md) declara que el prototipo trata todo solapamiento entre máscaras aceptadas como ILEGIBLE. Esto no equivale a la atribución al mayor S y al tratamiento de empates por epsilon del contrato. Debe resolverse conforme al contrato o mantenerse bloqueada la operación afectada.
3. **Identidad de la fuente en la API.** [src/plantillas.rs](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/5e3c29bf85fd49ba44c43c894132217f7c585f2d/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/realizacion-leyenda-01/src/plantillas.rs) compara el argumento textual de huella con una constante, pero no calcula la huella de los bytes TTF que recibe. La API pública de [src/lib.rs](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/5e3c29bf85fd49ba44c43c894132217f7c585f2d/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/realizacion-leyenda-01/src/lib.rs) admite ambos argumentos por separado. No queda ligado el contenido a su identidad en esta entrada; la ausencia del CLI impide comprobar una eventual validación adicional. Debe validarse sobre los bytes o mediante una entrada cuya validación no pueda omitirse.
4. **Admisión y recursos.** [src/parametros.rs](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/5e3c29bf85fd49ba44c43c894132217f7c585f2d/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/realizacion-leyenda-01/src/parametros.rs) permite sobrescribir claves repetidas y omitir claves desconocidas. La comprobación de s_px no excluye por sí sola NaN o infinito positivo; epsilon tampoco dispone de una validación completa. Se requiere admisión inequívoca de parámetros finitos y cotas previas suficientes para rasterización, almacenamiento y búsqueda, además de la cuota del PNG. La lectura del archivo de parámetros y la API pública deben respetar esas condiciones. No se ha ejecutado un ensayo de agotamiento de recursos.
5. **Decisiones explícitas.** Separador, anclaje vertical, TTF exacto y paso espacial de B.6.2 continúan pendientes según el propio depósito. Deben resolverse con fundamento trazable antes de Q1/Q2, sin sustituir la fuente por otra de nombre o versión semejantes.
6. **Independencia.** La diferencia entre nombres de dependencias no acredita independencia del proceso de decodificación. El contraste debe identificar los componentes efectivos de ambos recorridos y limitar la conclusión a lo comprobado.

### 10.3. Subsanación acotada y conservación

El siguiente trabajo es completar la entrega: depositar los tres originales faltantes y toda evidencia auxiliar necesaria; conservar el corte actual y las variantes discrepantes disponibles; emitir un manifiesto corregido con identidades completas; documentar cualquier original que ya no pueda recuperarse sin reconstruirlo como histórico. Una repetición posterior se registrará como ejecución nueva.

Una vez completo el depósito, descargar ese corte en un directorio limpio y compilar con `cargo build --locked --release`, conservando versión, invocaciones literales, salida y retornos. Resolver las condiciones de §10.2 dentro del alcance contratado. Q1/Q2 y la materialización de E1–E16 permanecen sin ejecutar en esta recepción.

Las evidencias de trabajo deben quedar en el repositorio autorizado, enlazadas desde su índice. Los testigos y fuentes reservados conservan su sede privada. Esta recepción no exige publicar su contenido ni duplicar indiscriminadamente el expediente.

### 10.4. Evidencias receptoras y reproducción

- [Fuente Rust del cotejo](COTEJO_RECEPCION_REALIZACION_01.rs).
- [Entradas fijadas: rutas, tamaños y blobs](ENTRADAS_RECEPCION_REALIZACION_01.tsv).
- [Compilación, invocación y salida completas](COTEJO_RECEPCION_REALIZACION_01_SALIDA.txt).

Para reproducir el cotejo, recuperar los diecisiete textos del corte de §10.1 conservando sus bytes. El directorio local utilizado por este cotejador sustituye cada `/` de las rutas por `__`; esta correspondencia solo organiza las copias receptoras. Compilar el fuente con Rust 1.98.0 y pasar como argumentos ese directorio y el TSV de entradas. Las órdenes concretas ejecutadas están conservadas en la salida. Se reutilizan funciones históricas de identidad y sus vectores de control; no se afirma validación criptográfica independiente.

El retorno cero significa que el cotejo terminó y confirmó el balance 17/14/2/3, **no conformidad de la entrega**. No se ejecutó Python ni el reconocedor.

### 10.5. Estado

**Recepción examinada; entrega material incompleta; subsanación pendiente.** S22 permanece en ejecución. S26 y S31 conservan su estado. No se declara finalizado (p1+p3)-Bis ni se modifica el punto de retorno, la GUI o la numeración de actas. El origen está conservado y enlazado en SVperitus; no se presume que las piezas ausentes estén depositadas ni que se haya efectuado un traslado al laboratorio.

## 11. Seguimiento de publicación parcial: corte 29a3c551

Fecha: 2026-09-15. **Estado: subsanación pendiente; depósito aún incompleto.**

### 11.1. Observación remota y límite de la recepción

Se consultaron la [comparación fijada](https://github.com/juantoniolloretegea/SVperitus-dataset/compare/5e3c29bf85fd49ba44c43c894132217f7c585f2d...29a3c5517bac13511421c6a6470e9e0ceac3db24) y el árbol recursivo del repositorio en `29a3c5517bac13511421c6a6470e9e0ceac3db24`, cuya respuesta no estaba truncada. La comparación contiene únicamente dos cambios dentro de realizacion-leyenda-01:

| Ruta relativa | Cambio | Bytes según árbol Git | Blob según árbol Git |
|---|---|---:|---|
| src/main.rs | incorporado | 2903 | `2926a28b109640b52ebe4be864954c4f572b5d96` |
| src/lib.rs | modificado | 1569 | `4fac8f5dc9eb8a87a47aea53a3ff5ae1e5bd8aed` |

Los otros dieciséis archivos de la realización mantienen sus blobs. Esta recepción examina metadatos de depósito y diferencia entre cortes; no presenta nuevas SHA-256 calculadas ni una auditoría de los dos fuentes cambiados.

No aparecen en el árbol fijado:

- `src/png_lectura.rs`, `src/reconocimiento.rs` ni `src/sha256.rs`.
- `evidencias/originales-ausentes/` y los tres originales históricos declarados en el parte.
- `NOTA_DISCREPANCIAS_MANIFIESTO.md`.
- `insumos/RETICULA_TANDA_01.tsv`.

Tampoco están depositadas las actualizaciones anunciadas de `src/parametros.rs` y `src/plantillas.rs`: conservan los blobs del corte anterior. El protocolo, el manifiesto, las decisiones pendientes y el registro de compilación conservan igualmente sus versiones anteriores.

### 11.2. Declaraciones locales que no se reciben como verificadas

El parte de entrega declara recuperación de los tres originales, conservación de variantes, correcciones de atribución, huella TTF, admisión de parámetros y recursos, así como una retícula de 27 celdas y una compilación local satisfactoria. Esas declaraciones quedan identificadas como **información del emisor pendiente de depósito y revisión**, no como resultados receptores acreditados.

La coincidencia de las huellas históricas se comprobará al recuperar los originales efectivos. Una carpeta o una evidencia conservada exclusivamente en el entorno del emisor no constituye depósito accesible al receptor.

La publicación pendiente comprende tanto módulos como originales, variantes, protocolo y evidencias. Después será necesaria la compilación desde una descarga limpia y la revisión de §10.2. Por ello, la expresión «único trabajo material abierto» no permite cerrar la subsanación ni dar por aceptadas sus soluciones.

### 11.3. Siguiente acción y condiciones de cierre

Continuar la publicación ya autorizada en la misma sede. Depositar el conjunto completo y coherente antes de solicitar otra revisión, conservando los originales y manifiestos anteriores. Publicar un índice de los archivos nuevos y un manifiesto corregido con rutas, tamaños y huellas completas; el índice debe distinguir las versiones históricas de los fuentes de trabajo.

A continuación, recuperar el corte completo en un directorio limpio, compilar con Rust 1.98.0 mediante `cargo build --locked --release` y depositar las órdenes literales, salidas y retornos de esa ejecución nueva. La evidencia generada después de ese corte se incorporará en un commit posterior que identifique inequívocamente el corte compilado.

La retícula de 27 celdas, su justificación, su orden y su parada deben quedar depositados antes de Q1/Q2. El criterio de éxito debe distinguir ajuste de parámetros y cualificación; detenerse al primer éxito de ajuste no acredita por sí solo la cualificación. Las convenciones de implementación se revisarán contra el contrato y los insumos exactos, sin ampliar el perfil ni sustituir la fuente reservada.

**Q1/Q2 y E1–E16 permanecen sin ejecutar en esta recepción.** S22 continúa en ejecución; S26 y S31 conservan su estado. El presente apartado no ejecuta Python, compilaciones ni ensayos del reconocedor. No modifica el rumbo, la GUI ni la numeración de actas.

## 12. Recepción del código depositado en 0e36549f

Fecha: 2026-09-15. **Ausencia de fuentes de trabajo: resuelta en el corte examinado. Compilación reproducible y custodia histórica: pendientes.**

Se recuperaron los veinticinco archivos del [directorio fijado](https://github.com/juantoniolloretegea/SVperitus-dataset/tree/0e36549f4f6914b4fdb7dd36a1f5b4ba5b91c797/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/realizacion-leyenda-01/) en un directorio receptor nuevo. El cotejo Rust 1.98.0 confirma tamaño y blob Git de los veinticinco y calcula sus SHA-256. Están los diez archivos Rust de `src/`, incluidos los módulos anteriormente ausentes. La comparación desde `5e3c29bf85fd49ba44c43c894132217f7c585f2d` contiene ocho incorporaciones y cinco modificaciones, todas en esta realización. No se ha modificado el código de origen.

### 12.1. Intento de compilación desde el depósito

Se ejecutó `cargo build --locked --release` con Rust y Cargo 1.98.0 sobre las copias recuperadas y cotejadas. Cargo alcanzó la descarga de dependencias y terminó con retorno 101 al intentar obtener `fdeflate 0.3.7` de crates.io: transferencia de cero bytes durante el intervalo de quince segundos configurado, sin reintentos.

**No se acredita compilación satisfactoria ni se atribuye un defecto de compilación a los fuentes.** El impedimento observado corresponde a la obtención de una dependencia. No se sustituyeron versiones ni se modificó Cargo.lock para sortearlo. La compilación limpia sigue pendiente; puede completarse en un entorno con acceso a las dependencias exactas, identificando el corte compilado y conservando su ejecución.

### 12.2. Custodia aún pendiente

El árbol recursivo no truncado no contiene el ZIP `realizacion-leyenda-01-completo-2026-09-15.zip`, los directorios de originales históricos y variantes, ni `MANIFIESTO_CORREGIDO.tsv` o `evidencias/COMPILACION_LIMPIA.txt`. El índice de origen los menciona, pero esa mención no acredita su depósito.

Del ZIP solo se recibe la declaración de 40.888 bytes y SHA-256 `e7d04f9afdd3da95cae921cf860d7902c2b9e016160c15aa190d51796325fb81`; no se ha recibido su contenido ni un enlace de descarga. Por ello no se coteja su identidad ni se atribuye recepción a sus originales históricos. Debe depositarse en la sede ya asignada junto con el manifiesto corregido, o proporcionarse el paquete efectivo para completar esa transferencia.

El `MANIFIESTO.tsv` anterior permanece conservado como documento histórico. Las entradas y la salida receptoras de §12.3 identifican los veinticinco archivos actuales; no sustituyen el inventario pendiente de los originales históricos.

### 12.3. Evidencia reproducible

- [Fuente Rust del cotejo de identidad](COTEJO_RECEPCION_0E36549F.rs).
- [Entradas fijadas contra el árbol remoto](ENTRADAS_RECEPCION_0E36549F.tsv).
- [Salida completa: tamaños, blobs y SHA-256](COTEJO_RECEPCION_0E36549F_SALIDA.txt).
- [Registro del intento de compilación](COMPILACION_RECEPTORA_0E36549F.txt).

Cotejo ejecutado con estas órdenes; ambos procesos finalizaron con retorno cero:

```text
continuacion-auditoria/rust/bin/rustc --edition=2021 /tmp/COTEJO_RECEPCION_0E36549F.rs -o /tmp/cotejo_0e36549f
/tmp/cotejo_0e36549f /tmp/sv_leyenda_0e36549f /tmp/ENTRADAS_RECEPCION_0E36549F.tsv
```

Para reproducirlo, recuperar los archivos enumerados en el TSV del corte fijado conservando sus rutas relativas y bytes, compilar el fuente y pasar el directorio y el TSV como argumentos. Se reutilizan las funciones históricas de identidad; no se afirma una validación criptográfica independiente.

### 12.4. Estado y siguiente paso

Terminar la custodia del ZIP/originales, variantes y manifiesto; aportar compilación limpia del corte exacto con dependencias fijadas. Después corresponde revisar las soluciones de §10.2 antes de cualquier cualificación. La presencia del protocolo de 27 celdas no constituye su aprobación ni autoriza ensayarlo.

S22 permanece en ejecución. S26 y S31 conservan su estado. El contrato sigue candidato. No se ejecutaron Python, Q1/Q2, E1–E16 ni el reconocedor. No se abren otras carpetas o ramas de trabajo ni se modifica el punto de retorno.

## 13. Transferencia completada y punto de continuidad

Fecha: 2026-09-15. El ZIP aportado después de §12 ha sido recibido, cotejado y depositado íntegramente en la misma rama `dominio-inmunologia` y sede `realizacion-leyenda-01`, commit `36bd96f81af9b19669e1f902240493000ef36399`.

**Transferencia y custodia de esta entrega: completadas.** [Nota de depósito, manifiesto, ZIP, originales y pruebas](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/36bd96f81af9b19669e1f902240493000ef36399/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/realizacion-leyenda-01/DEPOSITO_RECEPCION_ZIP.md). El ZIP tiene 40.888 bytes y SHA-256 `e7d04f9afdd3da95cae921cf860d7902c2b9e016160c15aa190d51796325fb81`, concordantes con lo declarado; su blob depositado es `40c5040d3932341a32804f2829848b75e4fbda22`. Los tres originales históricos coinciden con sus huellas previas. Las comprobaciones de identidad se ejecutaron en Rust 1.98.0; unzip verificó los CRC sin errores.

Se conservan 32 archivos dentro del ZIP: 22 coinciden con el corte de trabajo, 3 difieren y 7 son adicionales. Las diferencias del protocolo, README y main.rs se explican en la nota de depósito. No se sobrescribió el código de trabajo. Se añadieron los siete originales/variantes, el ZIP, el manifiesto receptor y las evidencias; el índice se amplió para enlazarlos. Los catorce blobs modificados o incorporados fueron comprobados en el árbol remoto. No se crearon ramas ni otras sedes de trabajo.

La compilación limpia sigue pendiente por el impedimento de descarga documentado en §12.1. El paso siguiente es completar esa compilación con las dependencias exactas de Cargo.lock y revisar después las condiciones de §10.2. No debe repetirse la solicitud de los originales ni tratarse el ZIP como ausente a partir de esta recepción.

S22 permanece en ejecución: el cierre de la transferencia no cierra (p1+p3)-Bis ni cualifica el reconocedor. S26 y S31 conservan sus estados. El contrato permanece candidato; Q1/Q2 y E1–E16 no se ejecutaron. Se conserva el rumbo y el punto de retorno, sin GUI ni renumeración.

## 14. Dependencia recibida y compilación completada

Fecha: 2026-09-15. **Compilación limpia del corte publicado: completada, retorno cero.**

El paquete fdeflate 0.3.7 aportado tiene 27.188 bytes y SHA-256 `1e6853b52649d4ac5c0bd02320cddc5ba956bdb407c4b75a2c6b75bf51500f8c`, concordante con Cargo.lock. Se cotejó en Rust antes de incorporarlo a la caché. Cargo pudo descargar las restantes dependencias y ejecutar `cargo build --locked --release` sobre los archivos recuperados de `0e36549f4f6914b4fdb7dd36a1f5b4ba5b91c797`, con rustc y cargo 1.98.0 y retorno 0.

Se cotejaron otra vez los 25 archivos del corte tras compilar: tamaños y blobs intactos, incluido Cargo.lock. No se incorporaron fuentes locales distintos de los publicados. La fase de compilación comunicó 16,55 segundos, sin atribuir esa duración a la descarga y preparación completas.

[Paquete conservado, fuente del cotejo, huellas, compilación e identidad posterior](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/78d87137ee195ca81bdf4471084b7039a05f164a/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/realizacion-leyenda-01/RECEPCION_FDEFLATE_Y_COMPILACION.md). Sede y rama asignadas; commit `78d87137ee195ca81bdf4471084b7039a05f164a`. Los intentos fallidos anteriores se conservan, pero su estado pendiente queda superado por esta ejecución satisfactoria.

**Punto de continuidad:** transferencia y compilación completadas. Siguiente paso: revisión acotada de §10.2 antes de cualificar el reconocedor. Q1/Q2 y E1–E16 no se ejecutaron; no se ejecutó Python. El contrato permanece candidato. S22 sigue en ejecución y no se declara cierre de (p1+p3)-Bis; S26 y S31 conservan su estado.


## 15. Recepción R1–R4 y objeción de atribución reproducida

Fecha de recepción: 2026-09-15T18:15:23Z. Corte del Lenguaje leído: `514dc0b7583f0ea7f2f30739ccce13c3c9f51199`. Rectores consultados: Pilares, perfiles/contratos/ensamblaje, transición IMM y Actas 001–002. Esta recepción sucede al §14, que conserva el resultado del corte anterior.

**Entrega recibida:** Peritus, rama `dominio-inmunologia`, corte `1840c9a950bf046b0629fc2d476ff2fc1d8955c7`; fuentes completos en `fdc211fec991b062bc5cd8668536e0c6f7f6a76f`. [Subsanación declarada](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/1840c9a950bf046b0629fc2d476ff2fc1d8955c7/dominios/inmunologia/cambio-rumbo/05-grok-aportes/2026-09-15-leyenda-r06/realizacion-leyenda-01/SUBSANACION_R1_R4.md). La comparación desde 00a8a1ae contiene quince archivos dentro de la sede asignada; no modifica /4, la adenda ni la retícula. Entre fdc211fe y 1840c9a9 solo se añade la evidencia de descarga.

### 15.1. Comprobación material receptora

Se recuperaron por separado los doce archivos Rust de src y Cargo.toml/Cargo.lock del corte 1840c9a9, en un directorio temporal independiente. Los catorce blobs locales coinciden con GitHub. Se ejecutaron con Rust/Cargo 1.98.0 `cargo test --locked --offline` (16 pruebas, retorno 0) y `cargo build --locked --offline --release` (retorno 0), utilizando las dependencias disponibles en caché. No se incorporaron fuentes privados. Una primera copia instrumental añadió una línea final; se corrigió la transferencia, se cotejaron los catorce blobs y se repitieron ambas órdenes sobre los bytes exactos. Esa primera ejecución no fundamenta la identidad acreditada aquí.

[Órdenes, identidad y salidas receptoras](RECEPCION_1840_EVIDENCIAS.txt). El archivo de origen COMPILACION_DESCARGA_PUBLICADA.txt coteja siete fuentes y remite a una compilación anterior; esta recepción aporta ejecución propia sobre los catorce archivos recuperados. Cargo.toml declara rust-version 1.98; la versión 4 del lockfile por sí sola no demuestra que únicamente 1.98 pueda leerlo.

### 15.2. Alcance de R1–R4

| Punto | Resultado receptor |
|---|---|
| R1 | La selección del mayor S está escrita y pasan sus pruebas de dos máscaras. El cierre completo no se acepta: el empate con tres máscaras depende del orden. |
| R2 | Se observa la comprobación de metrics antes de rasterize y comprobar invocada por procesar_png. Recibido en ese alcance; no acredita recursos globales ni ensayo con TTF hostil. |
| R3 | Protocolo 02 exige Q1 y Q2 y selección posterior por §C; n_min no desempata. Pruebas unitarias repetidas. La tanda real no se ha ejecutado. El comparador usa tolerancia 1e-4: no se extiende su aceptación a tuplas arbitrarias fuera de las 27 congeladas. |
| R4 | Comprobación entre filas escrita y sus dos pruebas repetidas. No equivale a medir alineación sobre los PNG históricos. |

**Contraejemplo de R1.** Tres máscaras contienen el mismo píxel, con S = 0,95; 0,80; 0,79 y epsilon = 0,02. La función publicada atribuir devuelve Exclusiva con el orden [0,95; 0,80; 0,79], pero Empate con [0,80; 0,79; 0,95]. En el primer recorrido solo compara cada candidato con el ganador provisional y omite el empate 0,80/0,79. /4 §B.4 exige rechazar ese par solapado.

Se ha ejecutado un conductor Rust externo enlazado con la biblioteca publicada, sin modificarla: [fuente](RECEPCION_R1_ORDEN.rs), resultado en la evidencia anterior. Es una prueba instrumental de la función pública con máscaras sintéticas; no demuestra que R01 o R06 produzcan ese solapamiento ni ejecuta Q1/Q2.

### 15.3. Siguiente acción acotada

Corregir únicamente la detección de empates de R1 para comprobar los pares de máscaras aceptadas que comparten píxel con independencia de su orden; preservar la atribución al mayor S y M_expl como unión geométrica. Añadir regresión de las seis permutaciones del contraejemplo y un control sin empate. Conservar /4, adenda, retícula y parámetros. Publicar fuentes y evidencias en la sede y rama existentes; repetir pruebas y compilación del corte publicado. No ejecutar Q1/Q2 en esta corrección.

Después de recibir esa corrección, verificar disponibilidad e identidad del TTF contratado y de R01/R06 en su custodia antes de preparar la cualificación conjunta sobre las 27 celdas. Si varias celdas ganadoras conservan las cuatro claves de §C iguales, el empate residual es fracaso; no se usa n_min ni se amplía la retícula.

Permanecen sin acreditar la independencia del códec y la correspondencia de píxeles U+007C/fontdue frente a resvg. TTF exacto privado; E1 sin testigo; Q1/Q2 y E1–E16 no ejecutados. La cualificación no constituye por sí sola aceptación del método.

**Estado:** S22 en ejecución; contrato candidato; reconocedor no cualificado; (p1+p3)-Bis abierto. S32 conserva su seguimiento de privacidad y seguridad; la revisión del parte comunicada por el usuario se recibirá separadamente cuando esté disponible. Sin cambios en OP-CYB-001, GUI o numeración de actas. Espejos históricos conservan sus cortes.
