# S26 R06 · INTEGRADO01 · Recepción del recorrido documental

**14 de septiembre de 2026. Unidad: Watson / W-S26. Resultado: 17/17 casos conformes en debug y 17/17 en release; ambas campañas con salida 0.**

El [banco previo](README.md) se ejecutó después de publicar el precompromiso en [Lenguaje, bc2dd7360f555c51464d3c9c185404bd4e368ac6](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/bc2dd7360f555c51464d3c9c185404bd4e368ac6) y [laboratorio, 4669517b9acc3953b104a978dd8bc2b4a7cbf591](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/commit/4669517b9acc3953b104a978dd8bc2b4a7cbf591). Los dos árboles completos se cotejaron con el auxiliar Rust R08 y se releyeron las ramas antes de ejecutar.

Se consultaron completos en esta continuidad Pilares, el acta de perfiles/contratos/ensamblaje y el acta de transición secuencial al corte de entrada fbc91824bdf57901c0e24236296f09b82fc661bd. [FUENTES.json](FUENTES.json) conserva sus rutas y blobs junto con las restantes entradas. Los siete archivos del banco y las 112 fuentes cotejadas permanecieron intactos durante ambas campañas; los oráculos no cambiaron tras observar resultados. [EJECUCIONES.json](EJECUCIONES.json) conserva comandos, fechas, entorno, códigos, huellas de ejecutables, controles por caso y cotejo posterior.

## Resultado material

| Casos | Observación recibida en ambos perfiles | Alcance |
| --- | --- | --- |
| I01–I02 | Recorrido completo y sustitución real de ruta por B después de admitir A; el consumidor recibió A y el padre aceptó su recibo literal. | Contenido admitido ligado al consumidor con el perfil mínimo declarado. |
| I03–I04 | Modificación del mismo objeto y A–B–A; captura y escritura anteriores conservadas tras SIGABRT 6 o SIGKILL 9. Las muestras acreditan B y su restitución en I04. | Terminación sin informe no borra hechos previos ni acredita reversión. |
| I05–I06 | Estado alterado antes de recibir: rechazo I03 transmitido antes del aborto. Alteración después de la copia: consumo A válido, informe truncado, hijo vivo en la frontera y salida 0 posterior. | Recepción, admisión, canal y terminación conservan sus diferencias. |
| I07–I08 | Escrituras reales durante lectura, con restitución A antes de admitir. Cambiar un byte pendiente produjo M01; cambiar uno ya leído dejó la copia A. I08 conservó informes contradictorios sin aceptación. | Los extremos A iguales no acreditan ausencia de mutación intermedia. |
| I09 | Bytes A con captura r2 para admisión r1: D04 local y captura_no_concordante en el padre. Fragmentos y bytes locales conservados. | Igualdad de contenido no sustituye referentes concordantes. |
| I10 | Informe A tardío conservado separado de B. B realizó su propia recepción, admisión, consumo y recibo tras la restitución programada de la entrada. | Correlación de dos intentos documentales fijados desde la apertura; sin reintento de un efecto incierto. |
| I11 e I15 | Sustitución por bytes iguales bajo dos perfiles previamente fijados: continuidad no ofrecida impidió ejecutar el consumidor en I11; el perfil mínimo admitió I15 y conservó un duplicado concordante. | No se rebajó el perfil solicitado para fabricar aceptación. |
| I12–I13 | Pánico local observado con captura y escritura previas conservadas, aunque el hijo salió 0. En I13 el plazo agotó con hijo vivo; SIGKILL 9 fue limpieza posterior y quedó separado. | Pánico del recorrido, silencio de canal y muerte del proceso no se confunden. |
| I14 | Captura A conservada e informe AJENO sin atribución al intento activo. | Un informe ajeno no habilita aceptación. |
| I16–I17 | Recibo y rechazo adversarial conservados en ambos órdenes, primer informe intacto y conflicto con aceptación bloqueada. | El I03 inyectado es una declaración adversarial; no se presenta como rechazo material de admit. |

Cada perfil observó dos SIGABRT, dos SIGKILL y un pánico capturado dentro del consumidor. Las señales esperadas constituyen estímulos del banco; la conformidad corresponde a los oráculos declarados. Las campañas no son mediciones de rendimiento ni de consumo global de RAM/CPU.

## Evidencia conservada

[EVIDENCIA.tar.gz](EVIDENCIA.tar.gz): **391144 bytes**, SHA-256 **bb88a56ae15a1f3aa546d837d24f5b55197a575fa1c02020413225676a56d38e**.

Contiene ambas campañas completas: apertura y plan, entradas y bytes recibidos/admitidos/consumidos cuando existen, muestras del padre, canal binario, frontera antes de liberar o terminar al hijo, observación final, efecto releído, stderr del hijo y resultados. Incluye salidas del banco, logs de construcción y manifiesto de la publicación previa. Las ausencias de piezas en casos rechazados se conservan como tales.

## Disposición y relevo

Queda contrastada la composición documental declarada entre apertura, recepción/admisión, mutación, consumidor LOCAL01 y observador de proceso. No debe repetirse LOCAL01, PROCESO03, T0701 o INTEGRADO01 por un mero cambio de registro. Una ampliación deberá identificar una frontera nueva y precomprometer su realización; un cambio de código requerirá recepción propia. PROCESO01 impedido y PROCESO02 incompleto permanecen en el historial.

La relación siguiente orienta la continuación; no convierte estos ensayos parciales en cierres de los doce casos globales:

| Caso global S26 | Aporte de INTEGRADO01 | Obligación que sigue pendiente |
| --- | --- | --- |
| F01 | I09 rechaza una captura con revisión incompatible. | Consulta exacta entre ocurrencias de igual valor y referencias distintas, con respuesta efectiva del consumidor. |
| F02 | Se preserva el encargo del fixture. | Resolución posicional/instancia/constitución frente a primera coincidencia, ambigüedad y ausencia. |
| F03 | Aperturas de intentos instrumentales conservadas. | Identidad de adquisición frente a recepción; dos IDs de intento no la constituyen. |
| F04 | I10 separa orden de llegada y selección activa. | Historia fijada, caché y proyección abierta en la realización que corresponda. |
| F05 | I05–I09 separan piezas copiadas, mutaciones y referentes. | Frontera constituida de confirmación y publicación de estado; el recibo documental no es commit. |
| F06 | Se observa la interrupción del hijo y un efecto local anterior. | Backend durable, fallo de escritura/commit y recuperación bajo política declarada. |
| F07 | Ningún contraste específico. | Fuente autoritativa y cobertura de índice; ausencia con cobertura suficiente. |
| F08 | Ningún contraste de recuperación. | Continuidad externa suficiente frente a restauración de copia antigua. |
| F09 | I10 conserva A tardío y entrega B mediante recorrido propio. | Captura de presentación y acciones de una realización consumidora futura; no hay GUI en este banco. |
| F10 | Bytes y referentes del consumidor documental se cotejan. | Correspondencia de posición, negación, leyenda, precisión y unidad en el perfil de representación elegido. |
| F11 | No se reintenta un efecto incierto ni se infiere reversión. | Autoridad, compromiso, despacho y observación del destino con las carreras pertinentes. |
| F12 | Truncamiento, rechazo, plazo, pánico y señal quedan explícitos. | Cobertura del consumidor/destino elegido y agotamiento de recursos declarado; no se acredita OOM ni host hostil. |

**Siguiente incremento: concretar S26-F01/F02 en las sedes existentes de referencias y ligaduras**, con dos ocurrencias legítimas de igual valor, consulta exacta y negativos de sustitución/ambigüedad. Primero deberán cotejarse sus contratos y realizaciones actuales; después fijar fixtures y oráculos Rust antes de ejecutar. No se inventará una identidad de adquisición ni una consulta histórica que el contrato no ofrezca.

Sucesos recibe la revisión 22 con enlace a esta evidencia y al espejo de laboratorio. Para seguimiento se usan [CSV vigente](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.csv), [Markdown vigente](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md) e historial; a9b99ea identifica un corte histórico inmutable. W-S26 se mantiene durante esta intervención.

Las pruebas se ejecutaron en Rust. RETP canónica permaneció fuera de escritura; se avanzan las dos ramas existentes sin force. No se modifica el núcleo, no se decide por dominio o agente ni se abre BD, GUI, host productivo o efecto externo. Padre e hijo comparten un anfitrión confiable: no se acredita integridad física de RAM, durabilidad, supervivencia del host ni protección frente a su compromiso. **S26/Bis permanecen abiertos; S24 sigue pendiente. R06 no cierra R0.**
