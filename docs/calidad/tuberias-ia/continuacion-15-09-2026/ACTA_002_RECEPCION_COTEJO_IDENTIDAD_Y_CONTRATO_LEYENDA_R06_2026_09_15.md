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
