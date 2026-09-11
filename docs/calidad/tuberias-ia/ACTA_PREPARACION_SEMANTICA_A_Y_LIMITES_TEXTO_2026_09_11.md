# Preparación de evaluación semántica y límites de texto

**RETP-2026-134 · 11/09/2026 · PREPARADO_PARA_CONTRASTE_PUBLICO.**

Se ejecuta la autorización de Juan Antonio Lloret Egea para evaluar todos los significados antes de aplicar permisos. Se incorpora su observación sobre idioma, tamaño de texto, capacidad de Vec y memoria compartida por las fases de análisis.

El objeto acotado es [IE004-ES-P2/3-COSTE/1](ie004/semantica-a/SUCESORA_CALENDARIO_Y_CONTABILIDAD.md). Conserva el léxico, G01–G25 y los criterios semánticos de /2, y declara un calendario y una contabilidad sucesores. Se fijan 61 controles semánticos y 72 regresiones sintácticas públicas, la base artificial y los esperados antes de ejecutarlos. El código devuelve todos los significados distintos, incluidos los denegables y contradictorios. Sólo después de resolver unicidad y demás impedimentos se consulta la política.

La realización conserva el texto original y offsets UTF-8, usa identificadores tipados para los campos y presta vistas de datos. Los buffers de A comparten un presupuesto agregado; cada inserción comprueba su máximo lógico antes de push. La memoria vieja y la nueva solicitada se contabilizan durante crecimientos. No se declara medida la memoria física del asignador, la pila ni una latencia de servicio.

[FIJACION_PREVIA.json](ie004/semantica-a/FIJACION_PREVIA.json) identifica las fuentes exactas, fixtures, sucesora y conductor. Hubo compilaciones preparatorias sin ejecución del binario; una invocación desde un directorio incorrecto no encontró main.rs y se corrigió el directorio. Ninguna de esas acciones es un resultado funcional. No se ejecutó Python.

Se realizará una matriz nativo/WASI, debug/release. Un fallo se conserva antes de una única corrección causal. No se ajustarán cupos ni esperados para convertir un fallo observado en éxito. La captura reservada no está habilitada; su compatibilidad con la sucesora queda pendiente de revisión y de fijar recepción A/V. P4 y P5 permanecen pendientes. El compromiso y la custodia declarada por el humano conservan su identidad.

Los documentos se espejan en Lenguaje y laboratorio. El avance registral se incorpora al RETP global en el mismo commit de Lenguaje. Se conserva la documentación del proceso para contradicción y reproducción por terceros.
