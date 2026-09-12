# Plan previo: ubicaciones sintácticas y perfil fuente

Cortes: Lenguaje 03d00131fb0df234101f01c9ca5dfa5f22886ceb; laboratorio ce202d9420adb2c6189fce557ad74e2a276c3a0c, rama lab/playground-sv-permanente. Base experimental: cápsula RETP-158, verificada contra los 49 archivos del árbol publicado.

Objeto: ampliar el paso 6 del plan de integración. Conservar el intervalo original en los emisores básicos del analizador y distinguir la grafía incompatible con el perfil cuando ese emisor compruebe una forma constitutiva. La causa se obtiene de la clasificación léxica ya existente y del contexto del emisor; nunca del mensaje ni del centinela. Una grafía contextual empleada legítimamente como identificador sigue admitida. Los datos y comentarios no se reclasifican como instrucciones.

Se fija CASOS_ESPERADOS.json antes de modificar la candidata. Cada intervalo se obtiene del fragmento seleccionado expresamente sobre la fuente original, sin consultar la salida del analizador. Las entradas inválidas se ejercitan además en ambas posiciones de un ensamblaje mixto con nombres de archivo iguales y una unidad sana. Se comparan los resultados heredados con la base, incluyendo los 120 casos canónicos (14 válidos, 106 inválidos). Se conservan las 239 pruebas unitarias por modo y los casos focales previos; dos esperados de grafía extranjera se sustituyen explícitamente en la batería sucesora, conservando la anterior.

Contrato experimental COMPILER-DIAGNOSTICS/2: se añade CD.FOREIGN_SURFACE y mensajes estáticos ES/EN. Se conservan códigos SV, decisión técnica e IR. La actualización diagnóstica es observable en la API detallada; la API heredada mantiene su resultado. No se promueve código productivo.

Emisores incluidos: despacho inicial, peek_word, take_raw_word, take_dispatch_word, take_word, take_text, take_nat, word, sym, arrow y etiqueta individual take_tri. Permanecen fuera los emisores compuestos de campos opcionales, reglas de admisibilidad y validadores restantes. Un emisor no migrado no recibe por conjetura la posición del último elemento consumido.

Ejecución: rustc 1.98.0 nativo; debug y release; tres repeticiones de casos focales. Límite instrumental de 120 segundos por proceso y 2 MiB por flujo. Se conservan comandos, binarios, salidas, códigos, tiempo, CPU y máximo acumulado de memoria de hijos; este último no se presenta como consumo por caso ni como incremento atribuible al cambio. Fallos y correcciones permanecen registrados. Python sólo transporta y observa; no actúa como compilador ni autoridad semántica SV.

Fuentes rectoras: AGENTS, Pilares, perfiles y ensamblaje, transición completa y relevos, contrato diagnóstico RETP-109/110 y workflow V2. Continúan DFL-001/011, emisores restantes, presentación final, WASI/navegador y recepción profesional. P3 reservado, P4/P5 y el encargo externo conservan sus condiciones. Este incremento no abre la reserva ni ejecuta proveedores externos.
