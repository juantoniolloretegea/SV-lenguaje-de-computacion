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
