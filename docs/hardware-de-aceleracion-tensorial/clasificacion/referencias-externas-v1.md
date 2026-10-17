# Referencias externas de comparación

Versión 1 · 17 de octubre de 2026.

El SV incorpora dos perspectivas independientes a sus páginas públicas: [clasificación de modelos](https://administracion.itvia.online/benchmark) y [clasificación de hardware](https://administracion.itvia.online/benchmark/hardware). No se suman sus puntuaciones ni se utilizan para modificar el polígono, la aptitud o la recomendación del SV. La inclusión de un modelo en una fuente externa no lo incorpora al grupo de candidatos del SV.

## Modelos: Arena Text

Se consulta el conjunto público [leaderboard-dataset de Arena/LMArena](https://huggingface.co/datasets/lmarena-ai/leaderboard-dataset), configuración `text`, edición `latest`, categoría `overall`. Se presentan las veinte entradas iniciales del bloque general, con posición, puntuación, límites del intervalo, votos y proveedor. Se conservan empates y nombres de versión. Las cifras visibles se redondean para facilitar la lectura; la fuente conserva su precisión.

Arena mide preferencias entre respuestas. No constituye una certificación de exactitud, seguridad o aptitud documental conforme al SV. Sus condiciones y versiones pueden diferir de las del examen propio. La [explicación del editor](https://arena.ai/blog/arena-leaderboard-dataset) y la clasificación completa permanecen accesibles. Atribución: Arena/LMArena; licencia CC BY 4.0. La presentación del SV traduce etiquetas y selecciona el bloque indicado, sin atribuirse las mediciones.

## Hardware: MLPerf Inference

La edición se descubre a partir del enlace de resultados publicado por [MLCommons](https://mlcommons.org/benchmarks/inference-datacenter/). La consulta fija la revisión de su archivo `summary.json`. No se elige una edición por encontrar un repositorio más reciente: un repositorio preliminar no prueba publicación de resultados.

Los grupos separan carga, escenario, división, tipo de sistema, disponibilidad, unidad y condición de resultado medido o derivado según el editor. Dentro de cada grupo se ordena la magnitud comunicada: capacidad por segundo descendente o latencia ascendente. Se conservan número de aceleradores y nodos, CPU, precisión, sistema y enlace al expediente del proveedor. Las configuraciones pueden diferir incluso dentro del grupo; no se construye una puntuación por chip ni una recomendación universal.

Los resultados derivados permanecen identificados y separados. Se conservan también los sistemas que sólo declaran CPU. No se convierten potencia nominal, presencia de un chip o publicidad del fabricante en consumo energético o rendimiento observado. La página distingue estos resultados ajenos de las comprobaciones físicas documentadas por el SV. Atribución: MLCommons, MLPerf Inference; licencia Apache-2.0, disponible íntegra en la presentación.

## Actualización y límites

- Al abrir la página, si han transcurrido seis horas desde el intento anterior, comienza una consulta compartida y acotada. Tras un fallo, el reintento queda disponible a partir de una hora. No se realizan consultas en ausencia de visitas.
- La página informa de la edición, última consulta satisfactoria, último intento, actualización en curso e incidencias. Consultar hoy una fuente antigua no la convierte en una edición nueva.
- Un fallo conserva la última copia válida y la marca como pendiente de actualización. Al reiniciar el servicio se obtiene una copia nueva; no se presenta una copia incorporada al programa como si fuera actual.
- Se limita el tiempo total, el tamaño de cada respuesta y los destinos. Las redirecciones no se siguen. Se rechazan cambios de formato, fechas o bloques discordantes, intervalos inválidos, unidades desconocidas y rutas improcedentes.
- Los datos externos se tratan como texto no fiable. Sus contenidos no se ejecutan ni deciden qué direcciones consultar. No se envían documentos, credenciales, cuentas ni resultados privados. No se habilitan herramientas ni Internet a los modelos.
- Las páginas mantienen explicaciones en español e inglés, controles utilizables mediante teclado y atribución de terceros. Los enlaces de consulta pertenecen al usuario y no al candidato examinado.

Este documento complementa el [criterio propio de hardware](criterio-v1.md); no modifica la doctrina, los contratos ni los resultados científicos del SV. Las fuentes originales conservan su autoridad y sus licencias.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
