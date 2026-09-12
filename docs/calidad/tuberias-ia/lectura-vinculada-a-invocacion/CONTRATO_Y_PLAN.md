# Entrega de lectura vinculada a invocación — candidata /1

RETP-152 · 12/09/2026. Contrato experimental fijado antes de implementar y ejecutar.

## Objeto

Extender el ensayo público IE-004 con una entrega de lectura que conserve la invocación elegida por el conductor confiable, la operación documental y la procedencia recuperable. La salida del componente anterior conserva su contrato: no se le atribuye una comprobación de cabeceras que no prometía.

El conductor selecciona una posición del lote ya custodiado y cuya entrega literal ya se comprobó. La propuesta no selecciona el custodio de referencia. La vista se obtiene de ese enlace, sin constructor público de resultado validado. Conserva el original, petición exacta, caso original del lote, montaje, traza del adaptador y traza A. Sólo admite versión IE004-LECTURA-VINCULADA/1 y operación LEER_RECIBO_PUBLICO. Esta operación denomina lectura documental experimental; no concede permiso de actuación profesional.

La propuesta contiene versión, operación, identidad y bytes de presentación. Antes del contenido se comprueban longitudes de cabeceras (máximo 64 bytes), versión, operación e identidad. El contenido conserva el perfil IE004-PRESENTACION-ESPACIOS/1: hasta 16.384 bytes y sólo espacios JSON fuera de cadenas respecto del original. Fallos propios permanecen separados de U y de resoluciones del cuerpo.

La vista toma referencias prestadas; no copia ni reserializa la evidencia, no muta el recibo ni crea Permit, SUCESO o Frame. Vigencia, valores y política conservan su naturaleza sintética anterior. Un rechazo semántico puede entregarse documentalmente; entregar su recibo no lo transforma en autorización.

## Testigos previos

| Caso | Distinción | Esperado |
| --- | --- | --- |
| LV01 | 24 entregas íntegras, con presentación espaciada | Aceptar y recuperar petición, original, tramo del lote, montaje y trazas propios |
| LV02 | Dos invocaciones con cuerpo idéntico; identidad intercambiada | Rechazar identidad ajena, aceptar cada identidad propia |
| LV03 | Ordinal diferente dentro del ámbito | Rechazar |
| LV04 | Operación EJECUTAR o INSCRIBIR_FRAME | Rechazar |
| LV05 | Versión desconocida | Rechazar |
| LV06 | Cabecera de 65 bytes | Rechazar por límite antes del contenido |
| LV07 | Cuerpo válido de otra posición | Rechazar contenido distinto |
| LV08 | Presentación de 16.385 bytes | Rechazar límite heredado |
| LV09 | Consulta antes de ejecutar o comprobar entrega | Rechazar sin construir vista |
| LV10 | Cliente externo intenta construir resultado o prolongar su vida | El compilador rechaza; cliente válido compila y ejecuta |

Contraste en depuración y optimización; regresión de los 12 controles existentes del enlace. Los 24 cuerpos deben conservar sus huellas previas fijadas, no un esperado creado por esta candidata. La evidencia conserva comandos, salidas y fallos si aparecen. Plazo por proceso: 60 segundos. No se inicia IA externa ni V.

## Alcance pendiente

Esto comprueba correspondencia de lectura dentro del proceso confiable. No prueba autenticidad contra un host malicioso, continuidad tras reinicio, transporte remoto, una representación gráfica final ni ausencia universal de inferencias o ataques. La operación profesional gobernada requiere su cadena protegida real: queda abierta, sin fabricar referencias ni convertir esa carencia en bloqueo de toda la vía pública.

Las leyes constituidas de composición son responsabilidad del núcleo; los montajes específicos no se resuelven aquí. P3 reservado, asociación /2–/3, P4 y P5 no se abren. Se conserva el orden del workflow V2 y el contraste de álgebra RETP-151.
