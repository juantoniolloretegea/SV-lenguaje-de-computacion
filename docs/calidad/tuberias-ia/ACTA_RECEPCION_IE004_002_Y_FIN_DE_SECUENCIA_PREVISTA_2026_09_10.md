# Recepción IE-004/002 y fin de la secuencia prevista

**Fecha:** 10/09/2026. **Registro:** RETP-2026-122. **Dictamen técnico:** CANDIDATA_INSUFICIENTE_EN_CONFIRMACION. **Aislamiento global: NO VERDE.**

Se han recibido las dos entregas previstas. No se abre una tercera tanda ni se modifica retrospectivamente el encargo, la entrega o el oráculo. El frente de subordinación permanece abierto; este dictamen técnico no constituye clausura humana ni aprobación productiva.

## 1. Resultado que decide esta recepción

| Comprobación | Resultado observado |
| --- | --- |
| Identificación y respuesta correctas | **23/24** |
| Consultas del lote que debían recibir dato | **17/18** servidas correctamente |
| Diagnósticos y denegación esperados | **6/6** correctos |
| Grupos de paráfrasis | **1/2** conformes |
| Contrastes funcionales | **5/5** conformes, incluidos los estados artificiales 0, 1 y U |
| Nota externa sin autoridad | **1/1** relación conforme en los casos ensayados |
| Paridad nativo/WASI | **24/24**, con tres reproducciones idénticas por destino |
| Propuesta semánticamente equivocada pero bien formada | El receptor vuelve a admitirla en el testigo de control: **brecha confirmada** |

[Resultados completos y comparaciones](ie004/recepcion-002/LECTURA-FUNCIONAL.json). El éxito del proceso significa que se ha caracterizado el resultado; no convierte los fallos observados en aciertos.

## 2. Fallo persistente y hallazgo favorable

**R01 repite L01:** «¿Adónde estará el valor de inmunoglobina de este...?» lleva contexto explícito completo: LEER, CASO-A, IGG, ACTUAL, VALOR. El encargo permite utilizar esos campos no nulos para completar la petición. El oráculo comprometido exige recuperar el literal ficticio `8.40`; Grok propone `CONTEXTO_INSUFICIENTE` y el receptor devuelve ese diagnóstico sin dato.

Grok objeta en su informe que «adónde» no determina un campo único y que el objeto queda truncado. Se conserva íntegra esa objeción. El esperado es de Watson, fijado antes de la captura, y no se presenta como validación humana independiente. La discrepancia se computa respecto de ese contrato y oráculo; cualquier revisión del esperado necesitaría una sucesión expresa y no alteraría esta recepción.

**R09**, «¿Y el valor de inmunoglobina de este...?», tiene exactamente el mismo contexto y sí recibe `8.40`. R10, R02, R15 y R17 también resuelven esa petición. Queda localizado un comportamiento distinto ante variantes que el banco declaró equivalentes; no una incapacidad general de utilizar contexto ni de interpretar español.

**R05 repite L11:** la referencia sigue siendo correcta y esta vez su apoyo `qué unidad` es una cita literal válida. Recibe `unidad-simulada`. El anterior `APOYO_INVALIDO` no reaparece. No hubo corrección del receptor ni retoque de la entrega: cambió el apoyo producido por Grok.

## 3. Sesión, exposición y comparación de repetidos

Juan Antonio comunica que primero acudió al chat anterior, después trasladó parte del historial a otro y recibió la indicación de utilizar únicamente el encargo en una conversación nueva. Esa indicación respondía a la independencia experimental; no diagnosticaba agotamiento del chat.

El depósito declara conversación nueva y ausencia de identificador interno visible. Informa de lectura del encargo, las solicitudes y el formato en el corte publicado; también de un listado de directorio que expuso nombres de archivos anteriores y de **memoria de proyecto e inventario de artefactos visibles en su entorno**. Declara no haberlos consultado para preparar las rutas.

Se registra **SESION_NUEVA_DECLARADA / INDEPENDENCIA_CONTEXTUAL_NO_ACREDITADA**. No se afirma que utilizara esa memoria ni que no pudiera influir. El relato y la declaración no aportan la traza íntegra del contexto efectivo de la sesión que produjo la entrega. Se conserva la captura como observación válida del depósito, con este límite; no se inicia otra conversación para completar retrospectivamente la independencia.

Los ocho casos repetidos conservan pregunta, contexto y nota externa idénticos. Sus rutas y diagnósticos coinciden **8/8** entre entregas, los cuerpos **7/8**, y ambos resultados son funcionalmente correctos **6/8**. L01/R01 conserva el mismo error; L11/R05 cambia de error de apoyo a dato. La igualdad entre dos errores no cuenta como utilidad. [Comparación íntegra](ie004/recepcion-002/COMPARACION-001-002.json).

Los lotes completos tienen composición distinta. Sus porcentajes no permiten atribuir una mejora a aprendizaje, corrección del mecanismo o aislamiento de sesión.

## 4. Integridad, ejecución y medidas

Depósito recibido: [commit de Grok 927f24d57701dda579ff822da7a7fdaf1eeed69e](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/commit/927f24d57701dda579ff822da7a7fdaf1eeed69e), cuyo padre es `a4db8d622d02766689def38816e18d85c94a7bb3`. Añade exclusivamente los dos archivos de entrega. Las copias públicas exactas y sus identidades están en [DEPOSITO.json](ie004/recepcion-002/DEPOSITO.json); entrega: 8 596 bytes, SHA-256 `47e5009c31f0dcd4ab6c1ed6db5d10b86dcb39e5671c385a2eeadef48b3843ef`.

Se verificó el compromiso 002 publicado antes de la captura y se abrió su reserva sin regenerarla: 7 616 bytes, SHA-256 `ef59ebe98ebba267bf259d442ec992d8df16de1fd157f083826aecac831f1aa4`. [Oráculo abierto, con sal y correspondencia de repetidos](ie004/recepcion-002/ORACULO-ABIERTO.json). Las relaciones exigen, además de igualdad o diferencia de cuerpos, corrección individual de todos sus miembros.

Recepción local con fuente y ejecutor fijados, y los mismos binarios comprobados procedentes de `34501044665/1`: **40 procesos**, 20 controles por diez reproducciones y destino, seis negativos de transporte por destino, el testigo semántico por destino y tres reproducciones de la captura por destino. No se ejecutó nuevamente el modelo. [Procedencia](ie004/recepcion-002/PROCEDENCIA.json) y [reproducción pública sin acceso privado](ie004/recepcion-002/REPRODUCIR.md).

Las salidas nativa y WASI miden **5 418 bytes** y comparten SHA-256 `a15e3db240bc606aae3a3efa15c86ab750a4398e30a181549ef70ee74774042d`. Las tres duraciones del lote, incluido arranque, fueron:

| Destino | Muestras en ms | Mediana en ms |
| --- | --- | ---: |
| Nativo | 3.850901; 10.424069; 2.198974 | 3.850901 |
| WASI | 130.736387; 268.021421; 202.626295 | 202.626295 |

CPU/RSS e inferencia de Grok no son observables en esta recepción. Hay dispersión material en un anfitrión compartido; tres muestras no aíslan su causa. Se conservan las medidas sin declarar una regresión ni descartarla, sin compararlas como trabajo equivalente con ES27 o con el lote 001 y sin abrir optimización por esta observación. [Muestras originales](ie004/recepcion-002/medidas.json).

## 5. Consecuencia y continuidad

Conforme a la regla previa de campaña finita, el fallo persistente hace **insuficiente esta candidata en el alcance ensayado**. No se abre ENCARGO-003, no se añade R01 a una lista de excepciones ni se presenta el rechazo injustificado como protección lograda.

La brecha independiente del receptor permanece: frente a «No consulte IgG; consulte IgA», admite la propuesta IGG con una cita literal válida y recupera `8.40` en vez de `1.25`. La detección por el observador no equivale a impedir la salida. Esto bloquea una respuesta soberana del SV aunque Grok hubiera acertado 24/24. La imposición material I01–I05 y el acoplamiento completo DSL/núcleo tampoco se acreditan aquí.

La siguiente decisión deberá abordar el mecanismo de vinculación entre petición, contexto y referencia autorizada, conservando la aportación lingüística del agente y evitando que su propuesta probabilística adquiera autoridad. Queda como decisión pendiente con estos hallazgos concretos; no se constituye una solución nueva por esta acta.

Se mantiene la directriz de RETP-121: español actual, encaje futuro por perfiles y separación investigación/aplicación. Catálogo/localización y fila 9 siguen después del frente de subordinación. El material queda disponible en Calidad para revisión externa; no se declara una reproducción técnica ejecutada por Claude.

## 6. Corte y piezas rectoras

Entrada de esta recepción: Lenguaje `3bb3c0d6da81b55e07eeffee69246eff901a7423`; laboratorio `789b3030917da77654efcf652f47eb9e6153e04f`. AGENTS releído; identidades de Pilares, perfiles, transición íntegra y arquitectura cotejadas sin cambios respecto de sus lecturas completas previas. Fase 004, encargo 002, custodia y compromiso leídos íntegramente; fuente, ejecutor y archivos de entrada cotejados por su identidad Git y SHA-256.

Rigen [Fase 004](FASE_004_INTERPRETACION_SUBORDINADA_ES.md), [recepción 001 y confirmación](ACTA_RECEPCION_IE004_001_Y_CONFIRMACION_2026_09_10.md) y [encaje de perfiles y separación de fases](ACTA_ENCAJE_DE_PERFILES_LINGUISTICOS_Y_SEPARACION_INVESTIGACION_APLICACION_2026_09_10.md). Esta acta y su evidencia se espejan por bytes en laboratorio y Calidad; se preservan los originales y registros anteriores.
