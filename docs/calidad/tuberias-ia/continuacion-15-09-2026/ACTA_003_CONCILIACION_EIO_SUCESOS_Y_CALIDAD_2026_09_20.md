# Acta 003 · Conciliación del ensayo EIO con Sucesos y Calidad

**Fecha:** 20 de septiembre de 2026.  
**Registro:** RETP-2026-259. **Seguimiento:** S39, en ejecución; S37 y S38 permanecen pendientes.  
**Naturaleza:** conciliación documental de actividad ya iniciada; no nueva campaña ni aceptación científica.  
**Autoridad:** instrucción humana expresa de conciliar los registros tras el cotejo de su desfase.  
**Unidad receptora:** W-S37. **Momento del asiento:** 2026-09-20T18:32:14Z.

## 1. Objeto y cortes de lectura

Se incorpora al seguimiento canónico la continuidad del ensayo EIO-GITHUB-01, que ya disponía de productos, resultados y recepciones en sus depósitos. Se conserva la separación entre preparación, ejecución, entrega, recepción y aceptación. La incorporación tardía no atribuye autorización retrospectiva, no rehace los contadores de campaña ni presenta como nueva una prueba anterior.

| Sede | Corte leído | Función |
|---|---|---|
| Calidad del Lenguaje, main | 68772d8bad39730425d1b73d9c82cf687c39ef02 | Registro canónico, antecedentes, reglas y obligaciones |
| SV-motor, main | 057dba8a4a1fd2774e35f61e6135fd096679502a | README 2.3 y expedientes públicos conservados |
| SV-sala-de-maquinas, main | d416dd75962314f84ff04f4cf5974f63b468fb28 | Encargos, entregas y recepciones; acceso restringido |

En el corte del Lenguaje se leyeron AGENTS.md, los Pilares de 05/09, el acta de perfiles, contratos y ensamblaje de 06/09 y la transición secuencial desde OP-IMM-001 completa, incluidos §§12–30. Se consultaron además Acta 001, Acta 002, parte S32, procedimiento de encargos de 17/09, reglas de Sucesos y registros RETP. El [contrato EIO-CONTRATO-01/1](https://github.com/juantoniolloretegea/SV-motor/blob/057dba8a4a1fd2774e35f61e6135fd096679502a/laboratorio/ensayo-ia-y-observabilidad/contrato/README.md) identifica su recepción del acta de rutas y su adscripción a (p1+P3)-Bis.

Los adjuntos históricos de agosto no se sustituyen ni reciben un nuevo dictamen por esta conciliación. Las fuentes canónicas y los expedientes identificados delimitan este acto.

## 2. Hallazgo y criterio de conciliación

Antes de este asiento, el registro RETP en CSV y Markdown terminaba en RETP-2026-258, del 17/09. Las entradas de continuidad y el parte S32 llegaban a esa recepción. Sucesos incorporaba S37 y S38 el 20/09, pero no una entrada que reuniera la continuidad material del EIO. Existía evidencia experimental en los depósitos; faltaba su enlace suficiente desde el seguimiento canónico.

El cotejo de las 39 filas vigentes con su última revisión histórica no encontró discrepancias de campos. En Markdown, 38 entradas concordaban descontando formato; S22 conservaba un resumen abreviado del mismo resultado, omitiendo campos presentes en CSV. Se restituye su representación completa desde la fila vigente, sin cambiar dicha fila ni crear una revisión experimental.

Se asigna **S39** al seguimiento del ensayo ya iniciado. S22 conserva la cualificación de leyenda y S32 sus obligaciones de privacidad; ninguno absorbe por semejanza nominal las campañas del motor. S39 permanece subordinado a (p1+P3)-Bis y remite a S32 para las obligaciones que le correspondan. No crea una segunda serie de sucesos ni abre una línea experimental nueva.

El primer hito documental localizado es la [versión 0.1 del ensayo](https://github.com/juantoniolloretegea/SV-motor/blob/875a3df0f2e07fb3c71f98d7fd6968ccae54af24/laboratorio/ensayo-ia-y-observabilidad/README.md), cuyo commit tiene fecha 2026-09-18T10:00:48Z. Esta marca se usa como inicio documental comprobado de S39; no fecha el comienzo de trabajos previos ni una autorización. El alta y la actualización del registro tienen la fecha real de esta conciliación.

## 3. Cronología técnica recibida por referencia

Los enlaces apuntan a informes conservados en el corte público leído. Sus propios campos identifican el commit ejecutado y el run; el commit que conserva un resultado no se confunde con el que se ejecutó. Esta tabla es una síntesis documental, no una reproducción de los bancos ni un recálculo de todos los manifiestos.

| Etapa y ejecución | Fuente | Resultado y límite conservados |
|---|---|---|
| Preparación documental inicial | [README 0.1](https://github.com/juantoniolloretegea/SV-motor/blob/875a3df0f2e07fb3c71f98d7fd6968ccae54af24/laboratorio/ensayo-ia-y-observabilidad/README.md) y [candidata 6f8b86d](https://github.com/juantoniolloretegea/SV-motor/tree/6f8b86d06ad46a5d98d31cefe6076b130280ffab/laboratorio/ensayo-ia-y-observabilidad) | Preparación sin compilación ni inferencia acreditadas en ese corte. |
| Número 1, intento 1 | [Resultado de preparación v3](https://github.com/juantoniolloretegea/SV-motor/blob/057dba8a4a1fd2774e35f61e6135fd096679502a/laboratorio/ensayo-ia-y-observabilidad/resultados/ejecucion-03/RESULTADO.md) | startup_failure por YAML antes de comenzar trabajos; cuenta 1/3 conservada. La corrección no constituye una ejecución satisfactoria. |
| Número 2, intento 1 | [Segunda preparación](https://github.com/juantoniolloretegea/SV-motor/blob/057dba8a4a1fd2774e35f61e6135fd096679502a/laboratorio/ensayo-ia-y-observabilidad/resultados/ejecucion-04/RESULTADO.md) | Rust/Cargo 1.98.0; lock de 171 paquetes y tres productos recuperados con identidades cotejadas según informe. Sin compilación ni inferencia; cuenta 2/3. |
| Número 3, intento 1 | [Ensayo final del presupuesto inicial](https://github.com/juantoniolloretegea/SV-motor/blob/057dba8a4a1fd2774e35f61e6135fd096679502a/laboratorio/ensayo-ia-y-observabilidad/resultados/ejecucion-04/ensayo-final/RESULTADO.md) | Instalación remota y rechazo de redirección, código 65; sin compilación ni inferencia. Presupuesto inicial 3/3 agotado. |
| EIO-TR-01, número 4, intento 1 | [Diagnóstico de transporte](https://github.com/juantoniolloretegea/SV-motor/blob/057dba8a4a1fd2774e35f61e6135fd096679502a/laboratorio/ensayo-ia-y-observabilidad/resultados/revision-transporte/EIO-TR-01/RESULTADO.md) | Host observado us.aws.cdn.hf.co, rechazado sin seguirlo; no identifica con certeza el destino de la ejecución anterior. Presupuesto extraordinario propio. |
| EIO-05, número 5, intento 1 | [Resultado](https://github.com/juantoniolloretegea/SV-motor/blob/057dba8a4a1fd2774e35f61e6135fd096679502a/laboratorio/ensayo-ia-y-observabilidad/resultados/continuacion-05/RESULTADO.md) | Seis descargas verificadas, compilación nativa y 24/24 controles. Siete inferencias rechazadas por ESTRUCTURA; check WASM con retorno 101. Comparación on/off realizada, sin aislar coste puro ni acreditar rendimiento general. |
| EIO-06, número 6, intento 1 | [Resultado](https://github.com/juantoniolloretegea/SV-motor/blob/057dba8a4a1fd2774e35f61e6135fd096679502a/laboratorio/ensayo-ia-y-observabilidad/resultados/continuacion-06/RESULTADO.md) | Inventario de 171 paquetes conservado; nativo y 24/24. Referencia literal reproducida y variante también ESTRUCTURA; receptor 2. Check WASM 0, sin ejecución en navegador por esta campaña. |
| EIO-JSON-01, número 7, intento 1 | [Recepción](https://github.com/juantoniolloretegea/SV-motor/blob/057dba8a4a1fd2774e35f61e6135fd096679502a/laboratorio/ensayo-ia-y-observabilidad/resultados/json-01/RECEPCION.md) | 24 controles de regresión y 35 complementarios, incluidos seis de sensibilidad. Sin nueva inferencia. Reserva de identidad entre archivos efímeros y segmentos de log; no trasladar su conformidad a todas las células SV. |
| EIO-NAV-01, número 8, intento 1 | [Resultado](https://github.com/juantoniolloretegea/SV-motor/blob/057dba8a4a1fd2774e35f61e6135fd096679502a/laboratorio/ensayo-ia-y-observabilidad/resultados/navegador-01/README.md) | NAV01–NAV04 conformes en Chrome identificado; NAV05 interrumpido por RSS agregada de aproximadamente 4,35 GiB frente a 4 GiB. 39 archivos recuperados según informe; sin salida contractual y con lagunas de cierre. |
| EIO-NAV-02, número 9, intento 1 | [Resultado](https://github.com/juantoniolloretegea/SV-motor/blob/057dba8a4a1fd2774e35f61e6135fd096679502a/laboratorio/ensayo-ia-y-observabilidad/resultados/navegador-02/README.md) | NAV01–NAV04 conformes; NAV05 interrumpido por el mismo umbral. Marcas tras ModelWeights y antes del primer forward; sin primer token ni salida contractual. 49 archivos recuperados según cotejo publicado; PSS del pico incompleta y coste de escritura de Node no exportado. |
| Preparaciones nativas 01 y 02 | [Candidata 01](https://github.com/juantoniolloretegea/SV-motor/blob/057dba8a4a1fd2774e35f61e6135fd096679502a/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-nativa-01/README.md) y [candidata 02](https://github.com/juantoniolloretegea/SV-motor/blob/057dba8a4a1fd2774e35f61e6135fd096679502a/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-nativa-02/README.md) | Fuentes y bancos preparados; no compilados ni ejecutados. No heredan la conformidad del nativo EIO-05/06. |
| Recepción de candidata 02 y encargo 03 | [Recepción NAT02-A/B](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/ba87934922a2de2956c1dc27155190c6b48f4caf/respuestas-ejecucion/EIO-GITHUB-01/revision-integracion-nativa-02/RECEPCION.md) y [EIO-NAT-PREP-03](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/d416dd75962314f84ff04f4cf5974f63b468fb28/encargos-ejecucion/EIO-GITHUB-01/integracion-nativa-03/ENCARGO.md), privados | Ajustes estáticos de oráculos/recuperación y cierre ante fallo de custodia. Encargo publicado para transmisión humana, sin entrega sucesora recibida en el corte. No habilita una nueva campaña. |

La suma histórica de números de ejecución no representa una autorización continua e ilimitada. Cada continuación conserva su encargo, precompromiso y presupuesto; esta conciliación no sustituye esos documentos ni audita exhaustivamente todas las autorizaciones históricas. Las guardas cerradas constan en los cierres publicados; no se modifican ni se afirma haber observado procesos remotos durante esta revisión.

## 4. Estado común de las dos vías

El [README 2.3](https://github.com/juantoniolloretegea/SV-motor/blob/057dba8a4a1fd2774e35f61e6135fd096679502a/laboratorio/ensayo-ia-y-observabilidad/README.md) reúne la explicación pública y conserva la procedencia desde 0.1. La vía A ejecuta Candle y el modelo en WASM dentro del navegador; la vía B prepara un servicio y procesos nativos con interfaz web. Ambas persiguen un funcionamiento verificable con las mismas exigencias aplicables; se trabaja primero la más factible y rápida que las conserve, después la otra, de forma secuencial.

Ninguna acredita todavía una aplicación completa de extremo a extremo. Las inferencias nativas anteriores produjeron resultados contractuales adversos; las de navegador quedaron interrumpidas; el servicio nativo posterior continúa como candidata sin comprobación ejecutable. No hay URL operativa acreditada por esta recepción.

La interrupción RSS demuestra la observación y actuación registradas, no un límite intrínseco de WASM, una fuga probada ni una causa exclusiva de Candle. La suma RSS no equivale a memoria física exclusiva y el umbral por muestreo no es una cota dura.

La integridad JSON se refiere a sus objetos, oráculos y mutaciones concretas. No constituye una célula SV ni acredita de manera general identidad de parámetro, valor y posición. Se mantienen los invariantes del Lenguaje y la separación entre fallo técnico y U.

## 5. Seguridad, observación y asuntos diferidos

- **Seguridad pasiva:** perímetros, permisos, separación y restricciones de acceso; la presencia de documentación o de una sandbox no demuestra todas las propiedades del conjunto.
- **Seguridad activa:** rechazo, revocación, parada y tratamiento de fallos; su aceptación depende de controles materiales identificados.
- **Observabilidad:** OpenTelemetry y medidas exteriores aportan señales con cobertura declarada; no confieren autoridad ni sustituyen la imposición de controles.

[S37](../../Inventario-sv/sucesos/SUCESOS_SV.md#s37) conserva el estudio diferido del contraste de propuestas de varias IA bajo decisión humana. [S38](../../Inventario-sv/sucesos/SUCESOS_SV.md#s38) conserva el estudio diferido de encapsulación y coexistencia. Sus altas del 20/09 están en CSV, Markdown e historial. Esta acta y RETP-259 les proporcionan remisión de continuidad en Calidad sin atribuirles estudio, implementación o pruebas realizadas.

S32/BIS-03, S22, S26, dominios y deuda nuclear mantienen sus estados. La evidencia EIO puede informar una recepción posterior específica de sus obligaciones; no las cierra por agregación. Los asuntos internos ajenos al experimento quedan fuera del asiento.

## 6. Continuación delimitada

1. Recibir la corrección NAT02-A/NAT02-B del encargo 03 cuando exista entrega identificada; distinguir inspección estática de prueba reproducida.
2. Preparar y decidir la habilitación remota de compilación y bancos sintéticos sin modelo. Cuota disponible, gasto adicional cero y contención exterior deben comprobarse en la fase aplicable; no se presumen.
3. Antes de ejecutar testigos que bloqueen o consuman recursos, acreditar la guarda exterior y sus permisos. La ausencia de esa capacidad impide habilitar esa fase; no autoriza elevar privilegios ni rebajar cotas.
4. Sólo tras recibir los resultados pertinentes, decidir una primera inferencia del servicio nativo y su interfaz accesible.
5. Consolidar resultados EIO-P-01 a EIO-P-15, reservas y conclusión limitada; remitir el resultado a (p1+P3)-Bis. La vía pendiente conserva sus comprobaciones propias.

Este orden describe la continuidad del expediente. No inicia Actions, Codespaces, compilación, inferencia, instalaciones, gastos ni un servicio; tampoco modifica guardas, workflows, contratos o criterios de aceptación.

## 7. Comprobación documental y conservación

El cambio reúne: alta S39 en CSV/Markdown/historial; representación completa de S22 sin alterar su fila; RETP-259 en CSV/Markdown; esta acta; remisiones en Acta 001, inicio, Léame primero y acceso al mapa. Se contrastan los campos vigentes con el historial, la conservación de los registros anteriores y los enlaces locales de las incorporaciones. La relectura remota de la publicación se realiza antes de comunicar su cierre.

Comprobación previa a publicación: 40 filas vigentes concordantes entre CSV, Markdown y última revisión del historial; 154 instantáneas históricas, con las 153 anteriores intactas; las 39 filas anteriores del CSV no cambian. RETP-259 conserva sus 14 campos en CSV y su desarrollo en Markdown. Los enlaces relativos incorporados resuelven dentro del árbol documental.

Se conservan íntegros los asientos anteriores de RETP y del historial de Sucesos. El mapa HTML mantiene su blob y su condición de instantánea histórica conforme a Acta 001 §10.1. Las copias históricas del laboratorio mantienen su corte; no se declaran espejos actualizados ni se modifican por esta conciliación, de acuerdo con esa disposición posterior.

Esta recepción acredita organización y correspondencia documental en los cortes citados. No recalcula todos los archivos experimentales, no repite los ensayos y no constituye aceptación científica, promoción productiva ni auditoría exhaustiva de plataforma.


<a id="recepcion-nat03"></a>

## 8. Continuidad: recepción preparatoria de NAT03 · 20/09/2026

**RETP-2026-263; S39 revisión 1.** La [recepción de NAT03](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/41341f0f5c362a0f649206b8c70bca6586e3e295/respuestas-ejecucion/EIO-GITHUB-01/revision-integracion-nativa-03/RECEPCION.md) actualiza el pendiente de §§3 y 6 para la candidata pública 484acaf y entrega privada 1c55fe5. NAT02-A/B se reciben favorablemente en fuentes: oráculos específicos y recuperación acotada; tratamiento del fallo de registro de waitpid con no admisión y diagnóstico. Compilación, pruebas y viabilidad material siguen pendientes.

El receptor cotejó 44 archivos públicos, 242446 bytes, por tamaño y SHA-256, y las dos entradas del manifiesto privado. Las 26 copias declaradas literales conservan sus blobs de base. Los textos se recuperaron por el conector GitHub; no se acredita independencia de plataforma ni funcionamiento. Los resultados históricos y las reservas B01–B08 mantienen su alcance.

Siguiente: [propuesta EIO-NAT-PREP-04](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/41341f0f5c362a0f649206b8c70bca6586e3e295/encargos-ejecucion/EIO-GITHUB-01/integracion-nativa-04/ENCARGO.md), comprobación previa de cuota, barrera de gasto cero y datos de entorno mediante lecturas disponibles. No permite crear o arrancar infraestructura. La fase material deberá concretarse y decidirse con esas evidencias; los testigos peligrosos requieren además contención exterior comprobada. Inferencia y URL operativa continúan posteriores a la recepción de las pruebas pertinentes.

La continuidad queda concordante en Sucesos CSV/Markdown/historial y RETP CSV/Markdown. No cambia el rumbo rector, la arquitectura, los otros sucesos ni el mapa histórico; no es otra acta de arquitectura ni una nueva campaña.


<a id="recepcion-m0"></a>

## 9. Recepción de NAT04 y cierre de inspecciones M0 · 21/09/2026

**RETP-2026-264; S39 revisión 2.** Asiento de recepción: 2026-09-21T02:35:11Z. Cortes leídos: Lenguaje main 6f3b38b654cb6df280e72f325fcfc5aa2b3b6c9d; SV-motor main 484acafebd5ec0bcedb759e2423ae935e7feb1a1; depósito privado main 493be00dde6d84899738a5dd6cc2aa78283dcf6f. Se consultaron AGENTS, Pilares completos, acta de perfiles completa, transición secuencial completa (§§1–30), reglas de Sucesos, Acta 001 §10, esta acta y registros aplicables. No se modifica la doctrina ni la arquitectura.

Esta recepción actualiza la continuidad de §§6 y 8: NAT04 ya fue entregada; dos inspecciones materiales posteriores quedaron incompletas y sus instancias fueron eliminadas. Los textos precedentes mantienen su corte histórico.

| Etapa | Fuente fijada | Resultado recibido y límite |
|---|---|---|
| NAT04 | [Informe](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/400976d807bfa5df5a40644c25912ec158eee7df/respuestas-ejecucion/EIO-GITHUB-01/entrega-04/integracion-nativa-04/RESPUESTA.md) | Comprobación documental y de cuenta; barrera de gasto cero observada. Sin infraestructura creada por NAT04. Imagen, herramientas y supervisión no acreditadas. La exigencia posterior de saldo numérico fue precisada por los encargos M0, sin tratar una cuota nominal como saldo leído. |
| M0-01 | [Cierre](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/eeaef4ed035e44bb0595bb6e70a58517451137fe/respuestas-ejecucion/EIO-GITHUB-01/entrega-04/inspeccion-nativa-m0-01/CIERRE.md) | Una solicitud consumida. Parada observada a 103,446 s; terminal bloqueado por confianza no resuelta, cero órdenes remotas. Eliminación con pérdida autorizada de cambios desconocidos. |
| M0-02 | [Evidencia](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/0fad8c0c44f8b3dd9b940860c923deb40bcfe151/respuestas-ejecucion/EIO-GITHUB-01/entrega-04/inspeccion-nativa-m0-02/EVIDENCIAS.md) y [cierre](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/493be00dde6d84899738a5dd6cc2aa78283dcf6f/respuestas-ejecucion/EIO-GITHUB-01/entrega-04/inspeccion-nativa-m0-02/CIERRE.md) | Una solicitud consumida. Parada observada a 174,321 s. Inserción accidental en el editor de ajustes por destino de entrada no comprobado, sin guardado acreditado. Confianza no aceptada y cero órdenes remotas. Eliminación con pérdida autorizada; All 0 documentado. No prueba incompatibilidad del entorno. |

Las observaciones proceden de informes y transcripciones exteriores. El receptor no ha repetido las interacciones ni recuperado los discos eliminados. La captura de la incidencia M0-02 permanece referida al registro de herramientas de la sesión; no se publica un archivo de imagen independiente. No se declara conservación íntegra de evidencia potencial ni cotejo original/copia de aquellos cambios.

El cierre M0-02 registra consumo acumulado del periodo y 0 USD facturados mostrados en cómputo y almacenamiento. Esa lectura, con redondeo y posible demora, no equivale a auditoría final ni permite atribuir todas las unidades a un solo intento. El tiempo hasta parada y la retención posterior son magnitudes distintas. El descuento operativo conservador es 104 + 175 = 279 s; quedan como máximo 2121 s (35 min 21 s) del presupuesto propuesto de 40 minutos, sin autorización de consumo por esta recepción.

### 9.1. Revisión adversarial del procedimiento

Se separan dos causas: M0-01 encontró una decisión de confianza no suficientemente explicitada; M0-02 incluyó esa decisión pero falló en la interacción con el editor antes de aplicarla. Ninguna inspección llegó a medir el entorno. Los cierres no se convierten en aceptación de CPU, memoria, herramientas o contención.

La consulta de configuración no requería editar JSON. La [propuesta M0-03](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/3cf1d2625033206cab66975de669e64d2285e543/encargos-ejecucion/EIO-GITHUB-01/inspeccion-nativa-m0-03/ENCARGO.md) sustituye para el nuevo intento el recorrido abierto por un procedimiento completo: consulta gráfica, identificación del destino antes de introducir texto, ausencia de uso de paleta para ajustes, confianza exclusiva de carpeta y captura exterior. Conserva presupuesto, tiempo, parada y decisión específica ante originales únicos no recuperados. No exige auditar exhaustivamente componentes integrados de la plataforma durante cinco minutos ni permite eludir controles de confianza.

La revisión es favorable a someter esa propuesta a decisión; no demuestra que la interacción futura vaya a completarse. No se crea otra instancia, no se cambia infraestructura ni se repite una campaña por esta preparación. No se habilitan compilación, pruebas o inferencia. La guarda exterior continúa pendiente de acreditación antes de las operaciones que la necesiten.

### 9.2. Conciliación y continuación

S39 conserva su identidad y estado en ejecución de seguimiento; no designa procesos remotos activos. Su fila CSV, representación Markdown y revisión 2 del historial incorporan esta recepción. RETP-264 se añade en CSV y Markdown. Las 41 filas vigentes restantes, las 158 instantáneas históricas previas y los asientos RETP anteriores se conservan. No se abre un nuevo suceso ni se implanta S41.

Se reutiliza esta acta como sede receptora, sin nueva acta de arquitectura ni actualización del mapa histórico. S22, S26, S32/BIS-03, S37/S38/S40/S41 conservan sus alcances. Las fuentes experimentales y los resultados anteriores no se modifican. Siguiente: decisión sobre M0-03; después, recibir sus lecturas y determinar si procede otra habilitación material. No se promete aptitud de Codespaces ni URL operativa.


<a id="recepcion-preparacion-ssh"></a>

## 10. Preparación material y cierre del intento SSH · 21/09/2026

**RETP-2026-265; S39 revisión 3.** Asiento receptor: 2026-09-21T07:06:43Z. Relevo de seguimiento de W-S37 a W-S39; se conserva la atribución histórica de las ejecuciones. Cortes de lectura: Lenguaje `6cf8abfc008ea5f6edb0821223a03812bbb0d2bb`, SV-motor `484acafebd5ec0bcedb759e2423ae935e7feb1a1` y depósito privado `4f46b7d118258136baecd9c657ad94358d7e4b77`. Se consultaron AGENTS, los Pilares y perfiles completos, la transición secuencial §§1–30, Acta 001, las entradas de continuidad y los registros aplicables.

Esta recepción sucede a §9: M0-03 ya dispone de respuesta y de continuaciones posteriores. La propuesta anterior y sus saldos teóricos conservan su fecha; no describen la disponibilidad actual ni autorizan otra ejecución.

| Etapa y evidencia fijada | Resultado recibido y límite |
|---|---|
| [M0-03: inspección inicial](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/4f46b7d118258136baecd9c657ad94358d7e4b77/respuestas-ejecucion/EIO-GITHUB-01/entrega-04/inspeccion-nativa-m0-03/RESPUESTA.md) | Instancia creada; inspección incompleta, sin órdenes remotas. Parada observada a 243,200 s. Instancia conservada, sin atribuir identidad a sus cambios desconocidos. |
| [Ventana técnica posterior](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/4f46b7d118258136baecd9c657ad94358d7e4b77/respuestas-ejecucion/EIO-GITHUB-01/entrega-04/inspeccion-nativa-m0-03/ventana-tecnica-20260921-035810/RESPUESTA.md) | Acceso previamente habilitado por intervención humana; Rust/Cargo 1.98.0 identificados y HEAD de NAT03 leído. Prueba aislada de grupo de control, terminación del hijo y comprobación de vaciado. Parada observada a 792,718 s. Guarda integral, vigilancia bajo carga y compilación no realizadas. El informe conserva diez evidencias cotejadas por el productor. |
| [Preparación de la guarda](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/4f46b7d118258136baecd9c657ad94358d7e4b77/respuestas-ejecucion/EIO-GITHUB-01/entrega-04/inspeccion-nativa-m0-03/ventana-tecnica-20260921-035810/guarda-rust-preparacion-v1/LEAME.md) | Fuentes Rust, receptor y procedimientos preparados; paquete de referencia d9fa1b55ca5986f40ea21e6d3d16edaacf05bc55. La preparación no acredita compilación ni validación dinámica. El antecedente Python permanece identificado como desviación documentada, sin ejecutarlo ni incorporarlo como sustituto de la guarda. |
| [Integración y último cierre](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/4f46b7d118258136baecd9c657ad94358d7e4b77/respuestas-ejecucion/EIO-GITHUB-01/entrega-04/inspeccion-nativa-m0-03/ventana-tecnica-20260921-035810/integracion-gh-2.101.0/LEAME.md) | GitHub CLI 2.101.0 y clientes existentes identificados; identidad dedicada cifrada y habilitada según registros de la entrega. Una conexión SSH rechazada antes de obtener una sesión de órdenes. Sin transferencia, comprobación del canal, compilación, pruebas, inferencia ni modelos; sin reintento. |

### 10.1. Resultado de conexión y alcance de la observación

El [error conservado](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/4f46b7d118258136baecd9c657ad94358d7e4b77/respuestas-ejecucion/EIO-GITHUB-01/entrega-04/inspeccion-nativa-m0-03/ventana-tecnica-20260921-035810/integracion-gh-2.101.0/preparacion-autorizada/conexion-permisos.stderr.txt) registra `Connection refused` durante el intercambio inicial y terminación SSH 255. El informe indica retorno 1 de GitHub CLI y salida estándar vacía. La causa permanece sin determinar: no se atribuye a credenciales, permisos, servidor o túnel por ese mensaje.

La [observación exterior del cierre](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/4f46b7d118258136baecd9c657ad94358d7e4b77/respuestas-ejecucion/EIO-GITHUB-01/entrega-04/inspeccion-nativa-m0-03/ventana-tecnica-20260921-035810/integracion-gh-2.101.0/preparacion-autorizada/CIERRE_OBSERVADO.json) registra un intento, una solicitud de parada, cero transferencias y cero compilaciones. El [resultado de parada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/4f46b7d118258136baecd9c657ad94358d7e4b77/respuestas-ejecucion/EIO-GITHUB-01/entrega-04/inspeccion-nativa-m0-03/ventana-tecnica-20260921-035810/integracion-gh-2.101.0/preparacion-autorizada/parada/RESULTADO.txt) confirma `Shutdown`. Inicio del intervalo: 06:38:22.5847943 UTC; consulta que devolvió el estado detenido iniciada a 06:39:11.8669709 UTC; observación exterior posterior a 06:40:08.3278053 UTC. Los 105,750 s constituyen una cota exterior conservadora, no tiempo facturable ni instante exacto de parada.

El receptor coteja esos registros publicados con el informe; no ha vuelto a conectar ni observado directamente el estado actual de la plataforma. La preparación SSH pudo producir efectos previos al rechazo; no se afirma ausencia absoluta de cambios remotos. No se recalculan los 53 archivos comunicados por la entrega ni se presenta ese cotejo como reproducción receptora. Tampoco se conoce la duración total de la intervención manual anterior o el coste individual de las ventanas.

### 10.2. Continuación y correspondencia documental

El siguiente trabajo es delimitar el diagnóstico técnico con las evidencias y herramientas existentes e identificar la observación mínima que falta. La última ventana está consumida y cerrada; un diagnóstico remoto requiere un encargo posterior. No procede repetir instalaciones, generar otra identidad o modificar permisos basándose únicamente en el rechazo registrado.

La acreditación del canal y de los controles aplicables precede a la compilación acotada de la guarda y NAT03. Los bancos sintéticos, la activación de inferencia y la descarga de modelos mantienen sus fases y condiciones propias. Las compilaciones nativas históricas EIO-05/06 no acreditan esta candidata.

Se actualizan la fila S39, su representación Markdown y la revisión 3 del historial, junto con RETP-265 y las remisiones de entrada. Se conservan las otras 41 filas, las 159 instantáneas históricas anteriores y todos los asientos RETP precedentes. S39 sigue en ejecución de seguimiento. Esta recepción no modifica código, contratos, infraestructura, permisos, presupuesto, otros sucesos o mapa histórico, ni incorpora una nueva autorización material.
