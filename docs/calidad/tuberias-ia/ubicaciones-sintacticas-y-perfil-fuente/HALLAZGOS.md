# Hallazgos y límites

1. RETP-158 perdía la posición de errores sintácticos distintos de EOF y de caracteres léxicos inválidos. Las primitivas incluidas conservan ahora el intervalo en su punto de rechazo.
2. Las grafías extranjeras de los puntos incluidos compartían causas genéricas. La clasificación léxica y el contexto del emisor permiten distinguirlas sin cambiar admisión ni interpretar el mensaje.
3. La incorporación de una variante pública hace observable la sucesión experimental /1 → /2. Se declara expresamente y se conservan dos versiones de los esperados afectados.
4. Los controles comprueban que una grafía contextual utilizada válidamente como identificador, un literal o un comentario no se convierte en error por su idioma aparente.
5. Los emisores compuestos pendientes conservan rango ausente. Dos controles ES/EN comprueban esa distinción; no se declara cobertura universal.
6. La base sin causa nueva y un mutante con intervalos desplazados terminan con código 101 en el comprobador, por las discrepancias previstas. Son controles de sensibilidad exigidos, conservados por separado; no fallos inesperados de la candidata.

No se observaron fallos inesperados en la cualificación inicial ni en el complemento. Las mediciones instrumentales no acreditan rendimiento relativo ni viabilidad de servicio. La aceptación de este subconjunto no cierra el contrato diagnóstico global.
