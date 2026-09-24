# Registro de partes de trabajo por agente

## Finalidad

Este registro deja constancia mínima, verificable y comparativa del trabajo realizado por cada agente sobre un árbol del repositorio verificado determinado.

## Campos clave

- `Agente`: siempre en formato `Agente X`.
- `Base_Verificacion`: grado real de lectura empleado antes de actuar.
- `Actuaciones_Desde_Ultimo_Control_Secuencial`: resumen de lo ocurrido desde el último barrido formal del árbol.

## Regla operativa

- No debe declararse una lectura `VERIFICACION_INTEGRAL` si el agente no ha recorrido realmente el árbol del repositorio verificado relevante para su dictamen.
- Las actuaciones globales de fase o de registro maestro deben apoyarse en lectura `VERIFICACION_INTEGRAL`.
- Las actuaciones locales pueden apoyarse en lectura `VERIFICACION_ACOTADA`, pero deben declararlo expresamente.

## Regla de no repetición derivada de la continuidad operativa asociada a WBeta SV-UCBC8

1. Todo archivo `.zip` subido por el responsable del proyecto debe inspeccionarse realmente por dentro antes de emitir juicio, aunque no se diga expresamente, presumiendo de entrada que puede tratarse de un repositorio verificado o de un parche material relevante.
2. En caso de tensión entre nombre externo del archivo, contenido interno real del ZIP, clon local, PDF, árbol del repositorio verificado o una reconstrucción no verificada, **manda el contenido material verificable más reciente**, no una memoria referencial no verificable.
3. Ningún agente debe tratar como “crear” un archivo cuya existencia real en el árbol del repositorio verificado no haya verificado antes con lectura suficiente del árbol.
4. La lectura `VERIFICACION_INTEGRAL` obliga a revisar el objeto material subido, no sólo a reconstruir el estado por continuidad documental operativa.
5. Cuando un agente cometa una inferencia indebida o un error de lectura material, debe dejar constancia explícita para prevenir repetición por agentes posteriores.

## Preservación del histórico detallado

El detalle humano completo de `PTA-2026-001` a `PTA-2026-009` queda preservado sin modificación en:

`docs/calidad/historico/REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE_HASTA_PTA_2026_009.md`

El CSV `REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE.csv` conserva la serie tabular completa y su continuidad. Este archivo mantiene desde ahora una lectura humana compacta del tramo vivo, sin eliminar el histórico anterior.

## Tabla maestra de partes

| Parte ID | Fecha | Hora | Agente | Lectura | Alcance declarado |
|---|---|---|---|---|---|
| PTA-2026-001 | 23/03/2026 | 16:59:50 | Agente WLenguaje7 SV | VERIFICACION_INTEGRAL | Control adicional de barridos y base de verificación. |
| PTA-2026-002 | 23/03/2026 | 17:45:05 | Agente WLenguaje7 SV | VERIFICACION_INTEGRAL | Auditoría H2-A y apertura restringida de H2. |
| PTA-2026-003 | 23/03/2026 | 20:02:35 | Agente WLenguaje8 SV | VERIFICACION_INTEGRAL | Parche correctivo funcional acotado del frontend. |
| PTA-2026-004 | 24/03/2026 | 08:48:50 | Agente WBeta SV-UCBC8 | VERIFICACION_INTEGRAL | Primer paquete VII y reglas de no repetición. |
| PTA-2026-005 | 25/03/2026 | 07:15:00 | Agente WBeta SV-UCBC9 | VERIFICACION_INTEGRAL | Consolidación VII hasta VII.3 y prueba de estrés al Lenguaje. |
| PTA-2026-006 | 25/03/2026 | 21:07:32 | Agente WBeta SV-UCBC10 | VERIFICACION_INTEGRAL | Consolidación VII.4–VII.5 y continuidad hacia VII.6. |
| PTA-2026-007 | 26/03/2026 | 09:55:00 | Agente WBeta SV-UCBC11 | VERIFICACION_INTEGRAL | Sellado técnico mínimo prebackend. |
| PTA-2026-008 | 26/03/2026 | NO_CONSTA | Agente WBeta SV-UCBC12 | VERIFICACION_ACOTADA | Asentamiento UCBC12 y piloto de seguridad estructural. |
| PTA-2026-009 | 16/08/2026 | 06:03 | Agente Watson Publicaciones-Lenguaje | VERIFICACION_INTEGRAL | Aprendizaje trazable y preservación pre-DSL. |
| PTA-2026-010 | 18/08/2026 | 18:49:00 | Agente Watson Lenguaje SV | VERIFICACION_INTEGRAL | Reentrada Ruta A, auditoría FFL-A/FFL-B, microcierres E112/E113/E307, reversión E406 y reparación registral. |
| PTA-2026-011 | 24/09/2026 | 13:06:58 | Agente Watson / W-S39-02 | VERIFICACION_ACOTADA | Recepción S39, diagnóstico MXFP4 y recuperación OneCloud. |

## PTA-2026-010 — Agente Watson Lenguaje SV

- **Fecha:** 18/08/2026  
- **Hora (Europe/Madrid):** 18:49:00  
- **Lectura del árbol del repositorio verificado:** `VERIFICACION_INTEGRAL`.  
- **Alcance declarado:** reentrada controlada del Lenguaje tras Ruta A; auditoría del trabajo heredado en FFL-A/FFL-B; materialización y contraste de E112, E113 y E307; reversión íntegra del intento E406 no mínimo; reparación y sincronización del control de calidad antes de rehacer el programa operativo.  
- **Base de repositorio verificada:** `main` fresco de `SV-lenguaje-de-computacion` desde la reapertura del 18/08 hasta el estado posterior a la reparación registral; acta de reapertura; tablero FFL; IR v0.2; catálogo diagnóstico; validator; suite declarativa; matrices/crosswalk/deuda; `docs/calidad`; Dinámica del Suceso como fundamento superior ya cerrado.  
- **Actuaciones desde el último control secuencial:** desde PTA-2026-009 se cerraron las compuertas matemáticas previas, se aprobó Ruta A y otra unidad avanzó FFL-A/FFL-B. Al asumir el frente se detectó una cadena de commits válida en parte, un parche E406 incompleto/no mínimo y un desfase grave: RETP, BARR y PTA no reflejaban la actividad del 18/08.  
- **Actuaciones ejecutadas:** lectura material del repositorio fresco; auditoría de los commits heredados; ratificación de FFL-A bajo Vía B; microcierres E112/E113/E307 bajo radio corto; reversión completa del intento E406; contraste editorial y conceptual de dependencias; revisión de interfaces y precursor del Panel del Experto; reparación de RETP CSV/MD; ejecución del barrido BARR-2026-006 y alta del presente parte.  
- **Artefactos leídos:** acta de reapertura; tablero FFL; IR v0.2; `src/svp_errors.py`; `src/svp_validator.py`; `tests/run_conformance.py`; catálogo de errores; matrices/crosswalk/deuda de `docs/calidad`; registros RETP/BARR/PTA; sede doctrinal pertinente; `SVperitus-dataset` sólo para contraste prospectivo del Panel del Experto.  
- **Resultado:** FFL-A queda trazablemente cerrado bajo Vía B; FFL-B es el único frente técnico activo; E112/E113/E307 están materializados; E406 no está aplicado y sólo podrá reabrirse mediante nueva microauditoría y diff mínimo; los registros de calidad vuelven a sostener una reentrada rápida.  
- **Observaciones:** no se declara una ejecución nueva de la suite global. No se consideran el estado editorial externo de publicaciones en `U`, el saneamiento de espejos ITVIA/GitHub, las interfaces futuras, el Panel del Experto, el manual/wiki/diccionario ni `NL→SVP` como bloqueos del FFL-B inmediato.

## PTA-2026-011 — Agente Watson / W-S39-02

- **Registro:** 2026-09-24T11:06:58Z; 13:06:58, Europe/Madrid.
- **Base:** VERIFICACION_ACOTADA, limitada a S39/TT-0012 y sus registros; corte Lenguaje `d19bb1b33d5dea39926c1861aab5198e9556f186` y [recepción Motor](https://github.com/juantoniolloretegea/SV-motor/blob/a74632b0b6dde70629863113134082dc3f31d521/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/recuperacion-onecloud-2026-09-24/INFORME.md).
- **Actuación:** lectura material de partes, Sucesos, historial, RETP, tique, actas y accesos; conciliación de intentos y recuperación; inscripción del relevo y de la revisión 16; remisiones a evidencia fijada a commit.
- **Desfase recibido:** S39 ya alcanzaba revisión 15, RETP terminaba en 268 y PTA en 010. Se registra la recepción hoy; no se certifica el intervalo global entre partes ni se crean asientos ficticios.
- **Resultado:** recuperación y arranque básico comprobados; regresión CPU recibida como una prueba aprobada, 340 filtradas; inferencia completa de la candidata pendiente.
- **Límites:** no hay inferencia nueva, modificación doctrinal, auditoría global o cierre de obligaciones ajenas. Ejecuciones heredadas conservan su atribución. Continúa S39 con W-S39-02; TT-0012 permanece abierto.
- **Referencias:** RETP-2026-269; Acta 004 §16; PTA-SVM-002. S42/TT-0010 y cierre parcial Qwen/B conservados.

**Límite del control registral:** El control detecta identificadores repetidos preexistentes en RETP (163, 164, 165, 166, 167 y 170); algunos representan apertura y recepción de un mismo frente. Se preservan sus filas y no se diagnostican ni renumeran sin estudiar su historia. RETP-2026-269 aparece una sola vez. Esta entrega no acredita unicidad global del registro histórico.

## PTA-2026-012 — Agente Watson / W-S39-02

- **Registro:** 2026-09-24T11:47:42Z; 13:47:42, Europe/Madrid.
- **Base:** VERIFICACION_ACOTADA sobre Lenguaje c68020992d19b041574992355de023961f6713d6 y [resultado Motor](https://github.com/juantoniolloretegea/SV-motor/blob/70750a516001cf13314176c529508d0712b7f3c1/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/onecloud-2026-09-24/RESULTADO.md).
- **Actuación:** lectura de rectores y condición de cierre; recepción de guardas, identidades y dos inferencias; contraste con originales; actualización S39 revisión 17, TT-0012 e índice, historial, RETP, actas y accesos.
- **Resultado:** OC-01 responde correctamente al caso 3 + 2; OC-02 conserva texto inconexo del anterior. Objetivo material de TT-0012 conseguido y tique finalizado con alcance explícito; S39 sigue abierto.
- **Límites:** un caso por ejecutable, construcciones distintas, rendimiento limitado y contadores finales de cgroup no disponibles. Sin prueba de calidad general, integración ni causalidad exclusiva.
- **Trazabilidad:** RETP-2026-270; Acta004 §17; PTA-SVM-003. No se atribuyen estos resultados a partes históricos ni se declara barrido integral.

## PTA-2026-013 · Optimización CPU y ejecución residente

**Registro:** 2026-09-24T12:43:08.557Z; 14:43:08, Europe/Madrid. **Unidad:** Unidad de ejecución experimental. **Base:** VERIFICACION_ACOTADA sobre Lenguaje 32bf520 y [Motor](https://github.com/juantoniolloretegea/SV-motor/blob/d4e62b29713a2044be0d1d4a7fb463155d3999c3/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/onecloud-2026-09-24/optimizacion/RESULTADO.md).

Se reciben protocolo previo, fuentes, tres pruebas numéricas, nueve guardas, Cargo check, diez peticiones en dos sesiones y 26 controles instrumentales. Cinco casos correctos y mediana 94,002 a 18,168 s; cierre del banco a las 12:27:20,066 UTC. Se cumple el criterio de primera fase; llama.cpp no se activa.

Se concilian S39 revisión 18, historial, RETP-2026-271, Acta004 §18 y accesos. TT-0012 conserva el cierre material anterior. Sin compras, ramas adicionales ni inferencia activa. Banco pequeño, orden fijo y ausencia de aislamiento entre las dos optimizaciones; no se certifica calidad general ni integración.

## PTA-2026-014 · Calidad parcial e integración conversacional

Registro 2026-09-24T15:04:15.258Z; 17:04:15, Europe/Madrid. Unidad de ejecución experimental; VERIFICACION_ACOTADA sobre Lenguaje 13e5becc550b731327ff33e291f79d884eee4b9b y [Motor dd4beaf5cf59c4114925fdece05bfe7885190395](https://github.com/juantoniolloretegea/SV-motor/blob/dd4beaf5cf59c4114925fdece05bfe7885190395/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/conversacion/RESULTADO_PARCIAL.md).

Doce tareas breves terminan normalmente con contenido correcto; diez cumplen formato estricto. Q07 contiene dos espacios al final de la primera línea y Q11 añade explicación al resultado correcto de 280 cm. Latencias totales 24,017–85,650 s, mediana descriptiva 31,454 s sobre tareas heterogéneas. Una ejecución por tarea; sin estimación de calidad general.

La interfaz derivada de Qwen y su exportación se comprobaron por acceso privado antes de detenerla. 0.2.0 aprobó 18 pruebas unitarias y controles del servicio; 0.2.1 aprobó 19 pruebas y compilación, pero su guardia detectó IDs especiales diferentes de los declarados y abortó. La serialización inicial utilizaba además un cierre de historial no reconocido. M01/M02 se conservan como evidencia de esa integración; M03 se interrumpió. L y D no comenzaron. Los IDs de vista previa no acreditan IDs del motor: se enviaban textos, no esos números.

El bloqueo posterior del terminal del Codespace procede de una revisión automática de seguridad que alegó rechazo previo del origen. La recarga anterior rechazada correspondía a chrome-error por protocolo no autorizado. No se elude la restricción. La infraestructura del asistente y las incidencias de integración se distinguen del modelo. Servicio y motor detenidos en el último corte remoto; URL pendiente de entrega.

S39 revisión 19, Acta004 §19, RETP-2026-272, PTA-2026-014 y PTA-SVM-005. TT-0012 conserva el cierre acotado anterior; S39 permanece abierto. Se conservan otros sucesos, rectores, núcleo, mapa HTML y resultados adversos. Sin compras ni ramas adicionales. La continuación requiere corregir y verificar el tokenizador y reanudar solo M/L y D, sin repetir Q; plazo común 16:38:50 UTC. Si vence, se registra la parte no ejecutada y se acuerda otra ventana.

## PTA-2026-015 · Reanudación controlada y comparación con preguntas recuperadas

Registro 2026-09-24T15:55:08.658Z; 17:55:08, Europe/Madrid. Unidad de ejecución experimental. VERIFICACION_ACOTADA sobre Lenguaje 28879943c8f403fe63d29f032f7c569bdfd8aafb y [Motor eafaf3711df15f4fca289d156e7333576144455a](https://github.com/juantoniolloretegea/SV-motor/blob/eafaf3711df15f4fca289d156e7333576144455a/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/conversacion/CONTINUIDAD.md).

La autorización expresa permitió recuperar el acceso al Codespace, que estaba detenido. Se reanudó por el control normal, sin eludir restricciones. La versión 0.2.2 completa los IDs ausentes a partir de las declaraciones del tokenizador, conserva entradas originales y mantiene las guardias. Compilación y 21 pruebas aprobadas. Servicio disponible desde las 15:40:50 UTC.

El control-03 se rechazó por reutilizar un identificador de petición antiguo: comportamiento correcto del servicio. Un auxiliar con identificador nuevo aprobó control-04, incluida cancelación y parada comprobada. La telemetría mantiene OpenTelemetry Rust y añade el muestreo del cgroup del motor, separado del árbol del servicio. Una muestra observó el PID del motor, CPU, RSS, hilos, E/S y sockets sin lagunas declaradas; no se extrapola a observación exhaustiva. Las necesidades de custodia independiente, contrato general y gráfica siguen delimitadas.

Banco M/L iniciado a las 15:46:20 UTC; M04 correcto y L0256 correcto, resto pendiente. El procedimiento secuencial espera su cierre antes de D, conservando el plazo anterior. Luego abre la comparación expresamente solicitada con máximo 60 minutos. Se recuperaron dos rondas de nueve y dos preguntas y cuatro consultas de OP-IMM-001-P10@1.0. Las preguntas y criterios se publicaron antes de la nueva ejecución. No se ha recuperado una tercera ronda ni se declara ejecutada la comparación.

Las condiciones de Qwen y GPT-OSS son diferentes; se preserva esa limitación. Se conservan antecedentes reales, límites de salida, rechazos y resultados adversos. La interfaz respondió y mostró los expedientes previos; la campaña sigue en curso y no constituye recepción final.

S39 revisión 20; Acta004 §20; RETP-2026-273; PTA-2026-015. TT-0012 conserva su cierre material acotado. S39 permanece abierto; sin modificación doctrinal, compras ni ramas adicionales.

## PTA-2026-016 · Recepción de contexto y documental; recuperación de la comparación

Registro 2026-09-24T17:58:17.005Z; 19:58:17, Europe/Madrid. Unidad de ejecución experimental. VERIFICACION_ACOTADA sobre Lenguaje a85d4d36b952360c7b53c3bc9f519f78a545d784 y [Motor a98825f24c7865a80e9aa35d6915c4ac93abdd04](https://github.com/juantoniolloretegea/SV-motor/blob/a98825f24c7865a80e9aa35d6915c4ac93abdd04/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/conversacion/INCIDENCIA_EXPORTACION.md).

La recepción de originales acredita M01–M04 y las cuatro condiciones L completas; la entrada mayor tuvo 3096 tokens y una latencia total de 891,512 segundos. Se distingue recuperación sintética de identificador de comprensión general. D01–D04 terminaron correctamente; D05–D08 no se admitieron por plazo. Las cuatro consultas documentales del pasaje OP-IMM-001-P10@1.0 fueron conformes. Qwen incumplió formato estricto en cuatro, aunque DOC02 expresó correctamente ausencia de respaldo. Cambian modelo y condiciones; no se atribuye causalidad exclusiva ni se valida todo el universo.

El corte del asistente dejó la campaña remota conservando resultados. R02-01 produjo una respuesta truncada de 256 tokens; el cierre OpenTelemetry duplicó 35 959 bytes de datos del motor y sobrepasó la cota individual. R02-02 fue rechazada antes de inferencia. Se conservaron respuesta, expediente, registros y trazas, separando el defecto instrumental del contenido del modelo. El observador separado seguía operativo en la recepción.

La versión 0.2.3 mantiene el valor completo en el expediente y exporta resumen numérico, tamaño y huella SHA-256. Se conservan las cotas y la guardia. Las 22 pruebas Rust 1.98.0 aprobaron, incluida reproducción y corrección del defecto. Las huellas desplegadas coinciden. Servicio y motor anteriores se detuvieron con MainPID=0; ejecutables conservados. Servicio nuevo iniciado a las 17:51:04 UTC.

El guion inicial de continuación falló por omitir src/ en la ruta de preguntas, sin admitir inferencia. Corregido y conservado el diagnóstico, el controlador empezó a las 17:55:35, manteniendo el límite 19:52:28 UTC registrado a las 17:52:28. R02-02 utiliza la misma conversación tras igualdad estructural con el antecedente; no se repite ni sustituye R02-01. Plazo individual nuevo 900 segundos, sustentado por la latencia medida, manteniendo 256 tokens y contexto 4096. URL privada conectada a las 17:56:21; no se lanzó inferencia web competidora. Comparación y recepción final todavía pendientes.

S39 revisión 21; Acta004 §21; RETP-2026-274; PTA-2026-016. TT-0012 conserva su cierre material acotado. S39 permanece abierto; sin modificación doctrinal, compras ni ramas adicionales.
