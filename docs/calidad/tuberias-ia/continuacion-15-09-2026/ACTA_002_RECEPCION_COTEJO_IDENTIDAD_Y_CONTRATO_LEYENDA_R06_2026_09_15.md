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
