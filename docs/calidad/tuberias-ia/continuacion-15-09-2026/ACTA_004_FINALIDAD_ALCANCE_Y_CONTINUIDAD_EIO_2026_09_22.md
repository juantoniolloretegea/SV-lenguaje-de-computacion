# Acta 004 · Finalidad, alcance y continuidad del ensayo de inteligencia artificial y observabilidad

**Estado actualizado · 05/10/2026:** [Acta 004 §32](ACTA_004_FINALIDAD_ALCANCE_Y_CONTINUIDAD_EIO_2026_09_22.md#32-conciliación-de-cierres-y-nueva-preevaluación--05102026), S39 revisión 38 y RETP-2026-277. Safeguard cerrado, conservado y con instancia/disco eliminados; Thinking cerrado y conservado, pendiente de eliminar su instancia/disco; Qwen3.5-122B-A10B Q8_0 instalado en UpCloud con 256 GB nominales y A01–A04 = 0 en A0. El bloque sigue incompleto. Los cortes inferiores conservan su valor histórico y no expresan el estado vigente.

**Fecha:** 22 de septiembre de 2026.  
**Seguimiento canónico:** S39, revisión 6; RETP-2026-268.  
**Naturaleza:** síntesis científica y técnica, actualización del estado documental y delimitación de la continuación.  
**Estado:** investigación lateral en seguimiento; instalación nativa documentada; comprobación material independiente pendiente.  
**Corte del Lenguaje examinado:** `9b2e5ef0aa1ea010a7cc79a2a67df7017bd99bc7`, rama `main`.

**Actualización posterior de estado:** §24, vinculada a S39 revisión 27 y TT-0016, incorpora el cierre de Instruct v7 y la corrección documental para revisión humana. Los §§1–23 conservan sus fechas y alcances históricos.

## 1. Objeto y razón de la investigación

El ensayo de inteligencia artificial y observabilidad estudia una cuestión concreta: si un modelo auxiliar de lenguaje puede funcionar en una realización identificada, producir una respuesta utilizable y conservar evidencia suficiente de la petición, las operaciones observables, el resultado y las condiciones de ejecución, bajo las obligaciones de autoridad del Sistema Vectorial SV.

Su finalidad es aportar información experimental a un trabajo principal ya existente: determinar la suficiencia de las representaciones, los contratos y las comprobaciones del Lenguaje SV. Las necesidades materiales de la inferencia, la comunicación y la conservación de evidencias deben conocerse antes de atribuir al sistema capacidades que dependan de ellas. La investigación tecnológica permite localizar esas necesidades; la decisión sobre su incorporación corresponde a la sede competente.

El ensayo constituye, por tanto, una **investigación lateral acotada dentro de (p1+P3)-Bis**. No sustituye el objetivo nuclear ni prescribe, por anticipación, cambios en la semántica, la representación intermedia —IR— o el núcleo Rust. Su contribución consiste en proporcionar hechos y contraejemplos que permitan distinguir una carencia de representación de un defecto de implementación, una limitación del modelo, una insuficiencia de observación o una restricción del entorno.

La denominación documental recibida es **gramática o superficie 0.2 e IR 0.3**. Las obligaciones semánticas proceden de sus fuentes normativas y adendas; no se introduce una especificación independiente denominada «Semántica 0.2». Esta precisión conserva la nomenclatura del [programa de trabajo (p1+p3)-Bis, versión 2, §2][bis].

## 2. Procedencia y relación con el trabajo principal

| Antecedente | Necesidad recibida | Relación con el ensayo |
|---|---|---|
| [Adenda de OP-CYB-001, §12][cyb] | Preservar la integridad del consejo asistido por IA: procedencia, cobertura, significado, autoridad, confidencialidad y continuidad. Una explicación generada no prueba el proceso que afirma describir. | Origina la necesidad de contrastar una realización concreta del componente auxiliar y de distinguir contenido, observación y fundamento verificable. |
| [Programa de trabajo (p1+p3)-Bis][bis] | Determinar qué obligaciones ya conserva la realización, cuáles necesitan comprobación o contrato y cuáles revelan una insuficiencia de representación o soporte. | Proporciona la adscripción del estudio y la sede de devolución de sus resultados. S22 conserva su alcance propio. |
| [Acta de rutas de conocimiento, §§2–4][rutas] | Distinguir recorrido exigido, recorrido efectivamente observado y justificación presentada; detectar también dependencias pertinentes omitidas. | Aporta las obligaciones de cobertura, correlación y separación de autoridad que recibe el contrato experimental. |
| [Acta 001, §§2–4 y 10](ACTA_001_CONTINUIDAD_Y_RUMBO_2026_09_15.md) | Conservar la secuencia de retorno al Lenguaje, evaluar la incorporación de Qwen y recibir las realizaciones con su alcance probado. | Mantiene la continuidad del trabajo y la revisión reforzada antes del retorno al recorrido previo a la adenda de ciberseguridad. |
| [EIO-CONTRATO-01, revisión 1][contrato] | Ejecutar un modelo auxiliar, verificar resultados técnicos y registrar operaciones instrumentadas con coste medible. | Concreta el ensayo en SV-motor, adscrito a (p1+P3)-Bis y con recepción pertinente en S32. S39 reúne su seguimiento específico. |

Los [Pilares][pilares], el [acta de perfiles, contratos y ensamblaje, §§6–9 y 12][perfiles] y la [transición secuencial desde OP-IMM-001][transicion] conservan su rango rector. En esta recepción se han leído completos, junto con AGENTS.md, en el corte indicado. La adenda de ciberseguridad, el programa Bis y el contrato EIO se reciben en los cortes identificados en sus referencias.

La relación con el núcleo es precisa: un resultado del ensayo deberá indicar qué información o distinción necesita conservar una operación, dónde se pierde o deja de comprobarse y qué sede debe resolverlo. Una exigencia particular de Qwen, Candle o Codespaces no se convierte por ese motivo en una regla universal del SV. Si la representación vigente es suficiente, el remedio puede corresponder al adaptador, al contrato del agente, a la frontera de ejecución o al soporte tecnológico.

## 3. Preguntas que debe resolver el ensayo

El objeto se delimita mediante cinco preguntas, que reúnen las obligaciones ya identificadas como EIO-P-01 a EIO-P-15 en el [contrato experimental][contrato]:

1. **Viabilidad material:** ¿puede la combinación seleccionada cargar el modelo y completar una respuesta en el entorno disponible, con una configuración explícita y recursos conocidos?
2. **Integridad del recorrido:** ¿pueden vincularse sin ambigüedad la petición, las entradas, la ejecución, la respuesta original y su causa de terminación?
3. **Cobertura de observación:** ¿qué operaciones se observan realmente, qué ausencias se detectan y qué aspectos permanecen fuera del alcance del instrumento?
4. **Control de ejecución:** ¿pueden aplicarse las facultades y límites de la operación, solicitar su cancelación y comprobar el estado material resultante sin conferir autoridad al texto generado?
5. **Suficiencia y coste:** ¿qué recursos y tiempos requiere cada vía, qué obligaciones quedan satisfechas en los casos observados y qué limitaciones deben devolverse al Lenguaje?

La compatibilidad técnica, la conformidad contractual y la calidad de la respuesta son juicios diferentes. Una respuesta puede haberse generado correctamente y no satisfacer el contrato; una salida estructuralmente válida puede contener afirmaciones incorrectas. Ninguno de estos resultados acredita por sí solo suficiencia clínica, seguridad general ni una operación soberana del SV.

## 4. Realización concreta y dos vías de ejecución

La combinación actual utiliza Rust 1.98.0, Qwen3-0.6B con cuantización Q4_K_M, Candle y OpenTelemetry Rust. Las identidades de los componentes y las dependencias efectivas se fijan por revisión y huella; sus nombres comerciales no bastan para reproducir una ejecución.

| Componente | Función delimitada |
|---|---|
| [Qwen3-0.6B][qwen] y [distribución GGUF seleccionada][pesos] | Modelo y pesos empleados para generar texto. Su aptitud para el propósito permanece sometida a evaluación. |
| [Candle, revisión fijada][candle] | Biblioteca que realiza las operaciones numéricas de inferencia, integrada en el ejecutable Rust. |
| `tokenizers` 0.23.1 y `tokenizer.json` identificado | Conversión entre texto y secuencias de unidades de procesamiento del modelo, denominadas *tokens*. |
| Rust 1.98.0 y código de integración | Adaptación, comunicación, comprobación determinista y control de los procesos dentro del alcance implementado. |
| OpenTelemetry Rust 0.31.0 | Registro de los puntos instrumentados. No sustituye la imposición de permisos ni demuestra observación exhaustiva del sistema. |
| Guarda de ejecución | Componente de supervisión y control de la carga experimental. Debe comprobarse sobre la configuración real; su generalidad para otros modelos o plataformas no se presume. |

| Vía | Lugar de la inferencia | Papel de la interfaz web | Continuidad actual |
|---|---|---|---|
| A · navegador y WebAssembly | Entorno del navegador que ejecuta el módulo correspondiente. | Recibe la petición y comunica con la ejecución del modelo en el navegador. | Conserva los resultados parciales anteriores y una comparación posterior pendiente. |
| B · ejecución nativa | Proceso Rust del equipo remoto; actualmente, el Codespace asociado a SV-motor. | Envía la petición al servicio y presenta su respuesta y estado. | Vía inmediata de continuación después de la comprobación material de la instalación. |

La elección secuencial sigue el criterio del [léame del ensayo, §3][ensayo]: avanzar primero por la vía materialmente factible que permita obtener antes un funcionamiento verificable y abordar después la otra. Se compararán configuraciones explícitas y condiciones pertinentes; utilizar los mismos pesos no garantiza igual comportamiento en plataformas diferentes.

La futura URL permitirá examinar la ejecución nativa desde un navegador. Su disponibilidad requerirá comprobar conjuntamente el servicio, el acceso de la plataforma, la petición y la presentación de la respuesta. La instalación actual no acredita ese recorrido ni constituye la interfaz general del Lenguaje cuya secuencia anterior fue cancelada en Acta 001 §3.

## 5. Criterio experimental y significado de las medidas

La continuación se apoya en los componentes reales y sus necesidades efectivas. Los controles sintéticos anteriores conservan utilidad para las propiedades que examinaron, pero no sustituyen la ejecución de Qwen mediante Candle en el entorno previsto. Tampoco una comprobación local de lógica demuestra por sí sola el funcionamiento de la comunicación, los permisos o la terminación en Codespaces.

Para interpretar el rendimiento deberán identificarse el modelo, la cuantización, la plantilla de conversación, el régimen de razonamiento, el método de selección de *tokens*, la extensión de entrada y salida, la concurrencia y el estado de carga del modelo. Las denominaciones genéricas de rapidez o intensidad no sustituyen los parámetros que la realización permita configurar y que realmente utilice.

La medida principal de uso será el intervalo entre la petición y la respuesta recibida, con sus puntos de inicio y fin definidos. Se distinguirá de la carga del modelo, la generación, el transporte y la presentación cuando esas etapas puedan observarse. Las duraciones se medirán con una referencia monotónica dentro de cada entorno; las fechas civiles permiten ordenar el expediente, pero no autorizan a restar relojes de equipos diferentes como si estuvieran sincronizados.

Memoria, procesamiento y almacenamiento se describirán indicando magnitud, unidad, ámbito, método y cobertura. El espacio libre del sistema de archivos no equivale al consumo atribuible al modelo; la suma de memoria residente de varios procesos tampoco equivale necesariamente a memoria física exclusiva. Se identificará la actividad ajena observada y cualquier limitación para excluir interferencias. No se declarará un entorno libre de ellas sin evidencia.

El coste de observación se distinguirá del coste de inferencia. Un instrumento demasiado costoso, incompleto o ambiguo puede interrumpir una ejecución sin demostrar un defecto del modelo. Los presupuestos temporales y de almacenamiento delimitan el trabajo autorizado; alcanzar uno de ellos se registra como causa de interrupción, sin presentarlo automáticamente como respuesta completada, incapacidad intrínseca de Qwen o valor semántico `U`.

Los ajustes posteriores se justificarán a partir de los resultados reales. Configurar el modelo o corregir el programa no equivale a entrenar sus pesos. Conservar registros tampoco produce aprendizaje ni permite modificar automáticamente el conocimiento activo.

## 6. Registro por sucesos y conservación de evidencia

Cada observación o resultado utilizado para fundamentar una conclusión deberá quedar vinculado a un registro estructurado y a su procedencia. JSON y, cuando resulte adecuado, JSONL cumplen la función de formatos de datos; su utilización no atribuye autoridad al contenido ni exige un motor JavaScript para interpretarlo.

El registro de ejecución deberá permitir identificar la sesión y petición, el componente emisor, las versiones relevantes, la secuencia observada, las marcas temporales, las medidas con sus unidades, el resultado y su causa de terminación. Las interpretaciones se distinguirán de las salidas originales. La ausencia de una medida se conservará expresamente; no se sustituirá por cero ni se deducirá éxito de un registro incompleto.

La respuesta original, las transformaciones necesarias para presentarla y las conclusiones de revisión conservarán relaciones explícitas. La retención y el volumen se limitarán por el presupuesto de la fase correspondiente. Este principio no exige crear un archivo distinto para cada muestra ni duplicar indiscriminadamente el contenido de las peticiones.

**S39 y el historial de Sucesos registran los hitos del proyecto; los registros de ejecución describen los hechos de cada sesión.** Las dos escalas se enlazan, pero no comparten por ello identidad ni significado. La [proyección documental JSON de esta recepción](S39_RECEPCION_DOCUMENTAL_INSTALACION_2026_09_22.json) conserva las identidades y resultados recibidos sin incorporar credenciales ni datos personales de operación.

## 7. Evolución observada y estado al 22 de septiembre

La [Acta 003](ACTA_003_CONCILIACION_EIO_SUCESOS_Y_CALIDAD_2026_09_20.md) conserva la cronología detallada y sus cortes. Esta síntesis actualiza su lectura sin sustituir resultados históricos.

| Etapa | Resultado documentado | Límite de la conclusión |
|---|---|---|
| Referencias nativas EIO-05 y EIO-06 | Compilaciones, controles e inferencias conservados en sus expedientes; resultados contractuales adversos. | Existieron inferencias anteriores. No acreditan el servicio ni el ejecutable instalado posteriormente. |
| Ensayos de navegador NAV-01 y NAV-02 | Controles iniciales conformes; interrupción de la fase de modelo por el umbral de memoria empleado, antes de obtener el primer *token*. | No prueban inviabilidad general de WebAssembly ni una causa exclusiva de Candle. |
| Preparación y controles nativos del 21/09 | Acceso IPv4, compilaciones y bancos específicos recibidos. Se corrigieron defectos de transferencia, formato, comprobación de entradas y coordinación temporal. | Cada resultado conserva su configuración; las correcciones parciales no equivalen a validación integral. |
| Última continuación nativa del 21/09 | La supervisión instrumental interrumpió la sesión con `DISCO_MEDICION_LENTA`; restitución y parada de plataforma documentadas. | El mensaje reunía dos causas posibles y no permitió discriminar la causa concreta. Los controles posteriores quedaron sin conformidad acreditada. |
| Examen del medidor del 21/09 | La prueba Linux documentó el vencimiento del plazo con un árbol de directorios y una limitación de cobertura sobre archivos abiertos sin nombre. | Es evidencia sobre el instrumento. No demuestra la causa exacta del episodio remoto ni constituye una corrección integrada. |
| Instalación nativa del 22/09 | Descarga e identidad de pesos y tokenizador, reutilización de Candle y construcción de `inferidor` con retorno 0. Cierre `Shutdown` registrado. | Recepción documental de una instalación. Su persistencia y estado material actual requieren la comprobación directa posterior. |

Las dos últimas entregas del 21/09 se identifican en el [informe de interrupción][interrupcion] y el [examen del medidor][medidor], ambos de acceso restringido. La restitución allí registrada sucede al estado indeterminado conservado en Acta 003 §12; no permite afirmar retrospectivamente que aquella captura incompleta hubiera acreditado restitución.

### 7.1. Instalación recibida

Se recibieron `REGISTRO.json` y la carpeta comprimida de instalación. Las dos copias del registro son idénticas y las 28 salidas cotejadas coinciden con los tamaños y SHA-256 declarados. Las salidas identifican una compilación Linux en el Codespace de SV-motor, con Rust/Cargo 1.98.0 y las dependencias fijadas. Los archivos `Cargo.toml` y `Cargo.lock` conservan sus huellas antes y después.

| Dato de la sesión de instalación | Valor documentado |
|---|---|
| Recursos asignados a la instancia | 2 CPU, 8 GiB de memoria y 32 GiB de almacenamiento nominal. |
| Pesos Qwen3-0.6B Q4_K_M | 396 705 472 bytes; tamaño y SHA-256 conformes según las salidas recibidas. |
| `tokenizer.json` | 11 422 654 bytes; tamaño y SHA-256 conformes según las salidas recibidas. |
| Compilación de `inferidor` | Retorno 0; 6 min 46 s informados por Cargo; ejecutable de 10 272 192 bytes. |
| Espacio libre al final de la compilación | 13 314 121 728 bytes en el sistema de archivos observado. |
| Duración de la sesión rectificada | 729,594 s, desde el inicio registrado hasta la confirmación de cierre. |
| Confirmación final conservada | `Shutdown`, registrada a las 04:38:41 UTC del 22/09/2026. |
| Ejecución del binario e inferencias durante esta instalación | No realizadas según el registro y las órdenes conservadas. |

La instalación inicial se había detenido al no acreditar una cota agregada propia de 4 GiB. La continuación autorizada se limitó a adquisición y construcción con los recursos asignados por la plataforma, sin modificar permisos ni grupos de control. Esta precisión no declara validada una guarda ni traslada automáticamente sus condiciones a una futura inferencia.

Las órdenes remotas de instalación conservadas emplean Bash. Para la continuación se mantiene el perímetro de implementación y pruebas técnicas propias en Rust 1.98.0; la recepción de aquel procedimiento no incorpora Bash ni PowerShell a esa base de trabajo.

### 7.2. Alcance exacto de esta recepción

El cotejo realizado comprueba los documentos y las salidas aportadas; no sustituye una lectura actual del Codespace ni un recálculo directo de las huellas de sus archivos. La proyección JSON distingue ambos niveles. No se atribuyen a este acto una conexión remota, una nueva compilación, una inferencia o la disponibilidad de una URL.

El [ejecutable candidato][inferidor] conserva una interfaz limitada a peticiones prefijadas y el [adaptador][adaptador] mantiene su configuración y límites internos. La instalación no los convierte en una interfaz de conversación libre. Una futura ejecución deberá examinar la causa de terminación de la generación: el retorno cero del proceso, aisladamente, no acredita que la respuesta haya concluido por la condición final del modelo.

## 8. Continuación secuencial y condición de término

La siguiente secuencia sucede, para la continuación actual, a la acción operativa prevista en S39 revisión 5 y Acta 003 §12. Los controles históricos pendientes conservan su estado; no se consideran superados ni se reactivan automáticamente.

1. **Publicar esta recepción documental.** Actualizar S39, su historial, los registros de calidad y los accesos de lectura. Entregar su resultado antes de la comprobación material.
2. **Comprobar directamente la instalación.** Examinar en el Codespace la identidad y presencia de los archivos, el ejecutable y las dependencias pertinentes, el entorno efectivo y su estado. Contrastar esos resultados propios con la entrega recibida. Esta comprobación tendrá evidencia distinta de la revisión documental.
3. **Obtener la primera respuesta de esta instalación mediante Qwen y Candle.** Identificar previamente la petición, la configuración efectiva, el presupuesto y las condiciones de aceptación. Registrar respuesta original, causa de terminación y observaciones realmente disponibles. Las inferencias históricas conservarán su atribución separada.
4. **Comprobar el recorrido accesible por URL de la vía B.** Verificar acceso autorizado, envío, recepción, presentación, control y conservación de evidencia con la realización efectiva. La visualización humana examinará ese sistema concreto.
5. **Contrastar la vía A y devolver el resultado.** Evaluar la otra vía de forma secuencial y comparar tiempos y recursos bajo condiciones declaradas. Si una vía no permite completar el contraste, conservar el diagnóstico y la insuficiencia de comparación en lugar de fabricar equivalencia.

Cada fase termina con un resultado acotado: capacidad acreditada en los casos observados, incumplimiento identificado o evidencia insuficiente. Un fallo exige localizar la causa y justificar una corrección concreta antes de otra ejecución; no autoriza una sucesión indefinida de repeticiones ni la ampliación automática de recursos o alcance. Los presupuestos de campañas anteriores no constituyen autorización permanente.

La salida de la investigación será una correspondencia entre **obligación, componente, observación, resultado, límite y sede de resolución**, reutilizando los identificadores del contrato EIO. Esa entrega permitirá decidir si la combinación seleccionada resulta adecuada, requiere una modificación justificada o debe descartarse para el alcance considerado. La misma tabla localizará las necesidades que deban recibir (p1+P3)-Bis y el Lenguaje.

## 9. Efecto sobre el estado del proyecto

S39 permanece **en ejecución de seguimiento**; esa expresión no significa que exista una campaña remota activa. La última sesión de instalación consta cerrada en la evidencia recibida. La comprobación material independiente y la primera inferencia de esta instalación permanecen pendientes y se tratarán sucesivamente.

S22, S32/BIS-03 y las obligaciones del núcleo conservan sus estados. Los estudios diferidos S37, S38 y S40 mantienen objetos distintos; no se incorporan a esta fase como nuevos requisitos generales. La disponibilidad local de herramientas en Windows o WSL2 puede servir de apoyo cuando sea pertinente, pero no se convierte en una obligación de repetir toda comprobación en tres entornos.

Esta acta no modifica la gramática, la IR, el núcleo, los dominios, el código experimental ni los criterios de las campañas ya realizadas. Conserva la autoridad humana sobre finalidad, permisos y aceptación. La IA auxiliar no adquiere facultad para incorporar conocimiento, cerrar `U` o producir efectos por el contenido de su respuesta.

El registro vigente, su revisión histórica y el nuevo asiento RETP se publican concordantemente. El mapa HTML y los expedientes anteriores conservan sus identidades. La sede experimental continúa en SV-motor; Calidad del Lenguaje conserva la recepción y el seguimiento canónicos.

## 10. Actualización de estado y continuación delimitada · 22 de septiembre de 2026

**Cortes recibidos:** SV-motor `a14ea31b3903d49a98f08b912206b1c8c9eeaf74`; Lenguaje `a875a5f74505045855bfbd56449f4c2522b457e7`. Se mantienen las piezas rectoras consultadas en esta acta.

La entrega EIO conversación 0.1.3 · Beta 1 y DOC-01 han finalizado en los alcances de TT-0007 y TT-0008. DOC-01 conserva cuatro terminaciones normales y cero aceptaciones contractuales. La conformidad integral de la vía B permanece pendiente; el cierre de estas tareas no la sustituye.

[CAPACIDAD-CGROUP-01](https://github.com/juantoniolloretegea/SV-motor/blob/511969f4ba576734f4bf96e39eed3e90d5dd02d5/laboratorio/ensayo-ia-y-observabilidad/resultados/capacidad-cgroup-01/README.md), recibida mediante TT-0009, aporta una inspección inicial del entorno. La apertura del control `cgroup.kill` de la raíz visible devolvió permiso denegado. No se acreditaron una hoja exclusiva delegada ni el control superior requerido. El resultado no demuestra imposibilidad de otra delegación; impide habilitar pruebas de bloqueo con esta evidencia. No hubo inferencias ni escrituras en los controles. El entorno quedó detenido tras la inspección.

Se mantiene la condición de continuación de §8: resolver una carencia concreta con presupuesto y condición de parada; una repetición exige una corrección o una condición distinta justificada. Antes de ensayar la guarda debe acreditarse la capacidad exterior. Si no está disponible dentro del alcance autorizado, corresponde registrar la limitación y resolver el alcance, sin ampliar recursos automáticamente.

La edición documental 2.5 incorpora [fichas por modelo](https://github.com/juantoniolloretegea/SV-motor/blob/511969f4ba576734f4bf96e39eed3e90d5dd02d5/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/README.md), conservando Qwen/Candle y la entrega publicada. gpt-oss-20b se registra como candidato sin instalación. La vía A continúa significando inferencia en navegador/WebAssembly. Una interfaz ante un proceso nativo remoto no acredita esa vía. El contraste entre Qwen/B y gpt-oss/A describiría configuraciones completas, sin aislar el efecto de WebAssembly.

Esta actualización no modifica la arquitectura contractual, las obligaciones del núcleo ni los criterios de pruebas anteriores. S39 revisión 10 y TT-0009 reciben el estado y la evidencia; los demás tiques conservan sus estados. La revisión específica de la adenda y la devolución a (p1+P3)-Bis permanecen posteriores al cierre delimitado de B.

[cyb]: https://github.com/juantoniolloretegea/SVperitus-dataset/blob/bbac1b44b1d3b845305e9cde492a08221206d631/dominios/ciberseguridad-inteligente/dominio-04-09-26/universos/OP-CYB-001/ACTA_CONTINUIDAD_Y_RELEVO_OP_CYB_001_AL_LENGUAJE_SV_2026_09_09.md#12-adenda-integridad-y-trazabilidad-del-consejo-asistido-por-ia
[bis]: https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/9b2e5ef0aa1ea010a7cc79a2a67df7017bd99bc7/docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/WORKFLOW_P1_P3_BIS_v2.md
[rutas]: https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/9b2e5ef0aa1ea010a7cc79a2a67df7017bd99bc7/docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/ACTA_EVALUACION_Y_RECEPCION_DOCUMENTAL_RUTAS_CONOCIMIENTO_SV_2026_09_14.md
[pilares]: https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/9b2e5ef0aa1ea010a7cc79a2a67df7017bd99bc7/docs/calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md
[perfiles]: https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/9b2e5ef0aa1ea010a7cc79a2a67df7017bd99bc7/docs/calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md
[transicion]: https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/9b2e5ef0aa1ea010a7cc79a2a67df7017bd99bc7/docs/dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md
[contrato]: https://github.com/juantoniolloretegea/SV-motor/blob/484acafebd5ec0bcedb759e2423ae935e7feb1a1/laboratorio/ensayo-ia-y-observabilidad/contrato/README.md
[ensayo]: https://github.com/juantoniolloretegea/SV-motor/blob/484acafebd5ec0bcedb759e2423ae935e7feb1a1/laboratorio/ensayo-ia-y-observabilidad/README.md
[qwen]: https://huggingface.co/Qwen/Qwen3-0.6B/tree/c1899de289a04d12100db370d81485cdf75e47ca
[pesos]: https://huggingface.co/unsloth/Qwen3-0.6B-GGUF/tree/f2d6f9ca53a254cc379437c49e4b2eb447f779df
[candle]: https://github.com/huggingface/candle/tree/ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a
[interrupcion]: https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/fae1add308dcf7079e3d46e7a0c969716139d26f/respuestas-ejecucion/EIO-GITHUB-01/entrega-04/controles-nativos-01/revision-04/RESPUESTA.md
[medidor]: https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/10484cf7a6bd5ae5fd4a132e0aeb6ab6ec58044e/watson-herramientas/evidencias/cualificacion-medidor-20260921/INFORME.md
[inferidor]: https://github.com/juantoniolloretegea/SV-motor/blob/484acafebd5ec0bcedb759e2423ae935e7feb1a1/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-nativa-03/nativa/inferidor.rs
[adaptador]: https://github.com/juantoniolloretegea/SV-motor/blob/484acafebd5ec0bcedb759e2423ae935e7feb1a1/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-nativa-03/inferencia/adaptador.rs

## 11. Recepción de correcciones y cierre delimitado de Qwen/B · 23 de septiembre de 2026

Se recibe el [cierre de campaña](https://github.com/juantoniolloretegea/SV-motor/blob/8cddcc83359bf6733a360d5bba2cd72426f8b631/laboratorio/ensayo-ia-y-observabilidad/resultados/cierre-qwen-b-20260923/INFORME.md) y la verificación local de conversación 0.1.4 y controlador gpt-oss 0.1.1. Ambos candidatos compilan con Rust 1.98.0. Las pruebas propias acreditan las propiedades descritas en sus informes; no incluyen nueva inferencia ni sustitución de las instalaciones. La revisión externa y sus originales conservan su identidad; sus precisiones de recepción no se presentan como una nueva conformidad externa.

Qwen/B queda concluida como realización parcial con limitaciones: resultados de rendimiento y fidelidad conservados, DOC-01 con cero aceptaciones de cuatro peticiones, guarda exterior y contención agregada no acreditadas, y comprobación externa de navegador incompleta. La intervención administrativa contemplada en un encargo histórico no constituye una tarea de la continuación actual. No se reactivan esos controles ni se exige recorrer todas las combinaciones de modelo y soporte.

S39 revisión 11 mantiene abierto el seguimiento del conjunto EIO. TT-0002, TT-0003, TT-0004 y TT-0006 finalizan en el alcance parcial expuesto en sus fichas; TT-0009 conserva su inspección inicial y TT-0011 recibe las correcciones. S42 revisión 1 y TT-0010 mantienen pendiente el futuro destino PC/WSL2, con auditoría ya recibida y capacidad efectiva aún sin inspeccionar. gpt-oss sigue instalado y sin respuesta de inferencia obtenida. Su emisor de SIGTERM anterior no se identifica por estas pruebas locales.

La devolución de resultados y carencias a la adenda y (p1+P3)-Bis permanece en S39. No se modifican las obligaciones rectoras, la gramática, la IR, el núcleo ni los criterios de las campañas históricas.

## 12. Continuación nativa de gpt-oss y recepción del diagnóstico · 23 de septiembre de 2026

Se recibe la [continuación material de gpt-oss-20b](https://github.com/juantoniolloretegea/SV-motor/blob/810a476488deb8dfb22585a51c91aee89d75ba0c/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/continuacion-2026-09-23/INFORME.md), con ocho intentos sin servicio ni respuesta de inferencia. Dos trazas distinguen SIGTERM externo anterior a la parada propia, sin identificar su servicio emisor ni su motivo. La recuantización Q3K, incluida la ejecución con un trabajador, alcanzó el límite instrumental de RSS. Se conservan originales, variantes y medidas; la instancia quedó detenida. El error de reintroducción de una opción incompatible se declara en el informe.

S39 pasa a revisión 12 y [TT-0012](../../Inventario-sv/tiques-tecnicos/TT-0012.md) permanece abierto para resolver el consumo de carga mediante una modificación fundamentada. No se concede conformidad integral, no se atribuye el SIGTERM histórico a una causa no demostrada y no se prescribe una nueva secuencia de gestiones administrativas. Qwen/B conserva el cierre parcial de §11; S42 conserva su destino futuro y su estado pendiente. Los diagramas A/B se han restituido en el README del ensayo con sus alcances históricos, sin modificación del mapa de continuidad.

## 13. Rectificación de supervisión y cuantización de gpt-oss · 23 de septiembre de 2026

Se recibe la [rectificación técnica y su evidencia](https://github.com/juantoniolloretegea/SV-motor/blob/250c3e27784d36f347e9ac0aefa5b53e6cc46c98/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/continuacion-2026-09-23/RECTIFICACION_CONTROLADOR.md) en Motor `250c3e27784d36f347e9ac0aefa5b53e6cc46c98`. Los intentos 09–12 no alcanzaron el servicio ni emitieron petición de inferencia. Se corrigieron dos defectos del controlador: interpretación de RSS total y plazo del acuse de escritura. La referencia 0.1.10 supera compilación y pruebas locales; ello no acredita una ejecución completa del modelo. La instancia quedó detenida, con confirmación a las 13:28 UTC.

La revisión del motor fijado y las dimensiones 2880 determina que Q2K/Q3K solicitados se sustituyen por Q4_0 en los expertos. Esta precisión rectifica la interpretación de §12 como reducción efectiva; no se modifica la evidencia histórica. Las señales exteriores preceden a la parada propia, pero no se identifica su servicio emisor ni su motivo. Se conserva la no conformidad de custodia declarada en el intento 09.

S39 pasa a revisión 13. [TT-0012](../../Inventario-sv/tiques-tecnicos/TT-0012.md) permanece abierto, con el objetivo de inferencia pendiente. La siguiente intervención requiere una modificación fundamentada del consumo de carga o una representación efectivamente compatible; cambiar sólo la etiqueta de cuantización no basta. Qwen/B conserva su cierre parcial, S42 su destino futuro y el mapa de continuidad permanece intacto.

## 14. Recepción de la continuación MXFP4 · intentos 13 y 14

Se recibe el [resultado y sus evidencias](https://github.com/juantoniolloretegea/SV-motor/blob/ae9ceaef37e18d9cc12e25ea0369e4ebf58d9fb8/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/mxfp4-2026-09-23/RESULTADO.md), Motor `ae9ceaef37e18d9cc12e25ea0369e4ebf58d9fb8`.

La candidata Rust 0.1.11 se ejecutó en los intentos 13 y 14, con topología que conserva expertos MXFP4 y solicita Q8_0 para el resto. No se completó la carga ni se emitió petición de inferencia. El intento 13 terminó por el plazo de carga de 700 s; el 14 tuvo varios hilos terminados por SIGKILL antes del SIGTERM de limpieza del controlador, con emisor y motivo sin identificar.

La última muestra persistida del intento 14 conservaba 1 204 113 408 B disponibles. El grupo visible no registró OOM y la lectura del registro del núcleo fue denegada. No se confirma ni descarta agotamiento de memoria fuera del alcance observado. El error al consultar descriptores no se presenta como causa raíz. El fallo anterior al servicio no fundamenta atribuirlo a Harmony.

Ambos hijos terminaron. El paquete original coincide con SHA-256 `35f6cb8bf4e111c4366e9382de745997ce82b2f0784d5750e3f2a6e76333fdf5`; 994 muestras de recursos persistidas y resúmenes derivados mediante Rust. La parada de Codespaces quedó confirmada a las 18:09:33 UTC, antes del límite exterior. TT-0012 pasa a pendiente: continuación suspendida para evaluación, sin otra ejecución.

Se conserva el antecedente de la transferencia bloqueada: a las 17:40 UTC se observó la instancia detenida, sin atribuir una hora retroactiva a aquella parada. El README del ensayo pasa a edición 2.9; diagramas y cierre parcial de Qwen/B conservados.

S39 pasa a revisión 14; TT-0012 pasa a pendiente, con la continuación suspendida para evaluación y el objetivo de inferencia sin conseguir. Qwen/B conserva su cierre parcial y S42 su destino futuro PC/WSL2.

## 15. Recepción GGUF: carga conseguida y diagnóstico de salida

**Fecha de recepción:** 2026-09-23T21:27:18Z. Se recibe el [informe con fuentes y evidencias](https://github.com/juantoniolloretegea/SV-motor/blob/f3746b719a3e355d75ea566d3d3abde2b6ea9e0e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/gguf-2026-09-23/RESULTADO.md), Motor `f3746b719a3e355d75ea566d3d3abde2b6ea9e0e`.

Los intentos 15–21 distinguen carga, aceptación HTTP, emisión de tokens y entrega del contenido. La asignación explícita permite cargar las 24 capas de gpt-oss-20b GGUF en CPU. Se corrige el rechazo indebido del alias del modelo por el controlador; su selector supera nueve pruebas declaradas con Rust 1.98.0 en el entorno remoto.

BF16 produce respuestas HTTP 200 con generación contabilizada, pero sin contenido final visible. La muestra con probabilidades confirma emisión de texto. Ampliar a 256 tokens no resuelve la entrega. Las variantes F32, con caché automática y con caché F32 explícita, completan la carga y cierran la conexión de inferencia sin respuesta HTTP. No se adopta una causa única ni se identifica Harmony como responsable.

El paquete de 228 833 bytes, SHA-256 `0c24a3ff967b95871a1885c793fa1187c3aa7872bab9d6ef144d24fd12bcf86d`, coincide entre origen y recepción. Contiene 115 archivos y 891 registros de recursos persistidos. Se conservan los huecos de muestreo y las limitaciones de ámbito. Los siete hijos terminan; a las 21:20:53 UTC no se observan los procesos propios examinados ni escucha en 8089. La instancia queda activa para la continuación.

S39 pasa a revisión 15 y TT-0012 a en ejecución, con respuesta útil pendiente. El diagnóstico vigente se concentra en entrada efectiva, operaciones numéricas, tokens y extracción del canal final. No se concede conformidad integral ni se reinterpretan retrospectivamente las señales anteriores. El README pasa a 2.10. Qwen/B conserva su cierre parcial, S42 su destino PC/WSL2 y el mapa de continuidad permanece intacto.

<a id="recepcion-onecloud-2026-09-24"></a>

## 16. Recepción del diagnóstico CPU y recuperación OneCloud · 24/09/2026

**Fecha registral:** 2026-09-24T11:06:58Z. **Responsable receptor:** Agente Watson / W-S39-02, en relevo de W-S39. **Lectura:** VERIFICACION_ACOTADA; cortes Lenguaje `d19bb1b33d5dea39926c1861aab5198e9556f186` y Motor `f3746b719a3e355d75ea566d3d3abde2b6ea9e0e`. **Evidencia publicada:** [informe y registros](https://github.com/juantoniolloretegea/SV-motor/blob/a74632b0b6dde70629863113134082dc3f31d521/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/recuperacion-onecloud-2026-09-24/INFORME.md), Motor `a74632b0b6dde70629863113134082dc3f31d521`.

### Hechos recibidos y comprobación nueva

1. La petición 23 fue rechazada por el JSON preparado para una ruta incompatible; no llegó a generar. La 24 fue aceptada y emitió 16 tokens defectuosos por completions. La 25 conserva HTTP incompleto y causa externa no atribuida; no se califica su contenido como otra respuesta defectuosa.
2. El cotejo de normalización y 216 bloques MXFP4 no halla discrepancias en las muestras. Las doce matrices Q8_0 examinadas favorecen el orden directo frente a las dos permutaciones ensayadas. No se certifican todos los pesos.
3. El cálculo CPU de expertos interpreta incorrectamente la entrada compartida `[tokens,1,dimensión]`. La prueba específica falla antes y pasa después del parche; contiene tres representaciones de entrada y es **una prueba**, con 340 filtradas. El resultado no acredita inferencia completa.
4. Los rechazos de admisión se separan del fallo semántico. En la revisión 32, umbral total 13 837 008 896 B y máximo disponible observado 13 811 625 984 B: déficit de 25 382 912 B; motor no iniciado. El margen revisado es experimental, no una cota demostrada.
5. En la recuperación actual faltan el GGUF y el árbol de construcción en sus anteriores rutas temporales. Causa e instante exactos no determinados. Se recuperan el ejecutable y tres paquetes en OneCloud con identidad SHA-256 cotejada. La observación nueva de las 11:00:05 UTC registra Ubuntu 26.04, 12 CPU virtuales, 66 528 256 000 B disponibles y arranque `mistralrs 0.9.3`. No hubo inferencia nueva.

### Dictamen y objeción adversarial

**Recuperación material comprobada; validación semántica de la candidata pendiente.** Disponer de más RAM y arrancar el ejecutable no demuestra que el parche resuelva la generación. La prueba unitaria solo demuestra el caso construido. El cambio de anfitrión impide atribuir cualquier mejora exclusivamente al parche sin un contraste controlado en ese mismo anfitrión. No se atribuyen a Oryx o al editor fallos del modelo sin evidencia causal.

### Decisión y continuidad

Conciliar primero los registros. Restituir y cotejar pesos, controlador y configuración; comprobar guardas; fijar petición, presupuesto y criterios de aceptación; después ejecutar una inferencia acotada con trazas y cierre. No se alteran contratos, gramática, IR, núcleo o mapa HTML. Qwen/B mantiene su cierre parcial; S42/TT-0010 mantienen el destino PC. La vía A sigue siendo inferencia en navegador/WebAssembly.

S39 pasa a revisión 16 y conserva `en ejecución` como seguimiento abierto, no como indicación de generación activa. TT-0012 sigue abierto. La conciliación fecha el relevo actual y conserva las instantáneas históricas; no reconstruye un barrido integral ni el periodo completo desde PTA-010. Las copias históricas de laboratorio conservan su corte y no se declaran sincronizadas con esta recepción canónica. Referencias: RETP-2026-269, PTA-2026-011 y Motor PTA-SVM-002.

**Observación del control registral:** El control detecta identificadores repetidos preexistentes en RETP (163, 164, 165, 166, 167 y 170); algunos representan apertura y recepción de un mismo frente. Se preservan sus filas y no se diagnostican ni renumeran sin estudiar su historia. RETP-2026-269 aparece una sola vez. Esta entrega no acredita unicidad global del registro histórico.

[Control documental de esta recepción](CONTROL_RECEPCION_ONECLOUD_2026_09_24.md).

## 17. Recepción experimental OC-01/OC-02 y cierre material de TT-0012 · 24/09/2026

**Registro:** 2026-09-24T11:47:42Z; W-S39-02. **Cortes:** Lenguaje c68020992d19b041574992355de023961f6713d6 y [Motor 70750a516001cf13314176c529508d0712b7f3c1](https://github.com/juantoniolloretegea/SV-motor/blob/70750a516001cf13314176c529508d0712b7f3c1/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/onecloud-2026-09-24/RESULTADO.md). **Clasificación:** VERIFICACION_ACOTADA.

### 17.1. Hechos y evidencias

Protocolos publicados antes de cada intento. Pesos GGUF y auxiliares cotejados; controladores y compilador identificados. Nueve pruebas instrumentales aprobadas con Rust 1.98.0, incluida auxiliar. Dos inferencias secuenciales con la misma petición, pesos y parámetros: candidata OC-01 con respuesta correcta y stop en siete tokens; anterior OC-02 con texto inconexo y length en 16. Originales JSONL completos (192 y 95 registros), HTTP, motor, consumo, fuentes y cierres conservados.

OC-01 termina a las 11:26:37,325 UTC; OC-02 a las 11:35:47,090 UTC. Los servicios se verifican inactivos y los hijos ausentes. No hay inferencia de esta campaña activa al cierre.

### 17.2. Alcance de la recepción

Se ha conseguido el objetivo material escrito en TT-0012: recibir una respuesta sintética atribuida a la instalación, con configuración, consumo y cierre. Se añade aceptación semántica del caso 3 + 2. **TT-0012 se finaliza con ese alcance; S39 permanece abierto.** No se extiende el cierre a tareas diversas, integración conversacional, vía A o aptitud productiva.

El contraste aporta diferencia entre paquetes ejecutables en el mismo anfitrión. No prueba causalidad exclusiva del parche porque las construcciones difieren. La regresión unitaria previa conserva su resultado local. No se realizó benchmark: un caso, orden fijo, sin control de cachés. El tiempo de OC-01 (112,534 s informado por el motor; 193,723 s de controlador) limita la valoración práctica.

### 17.3. Contención y reservas de medición

Servicio exterior con 32 GiB, swap 0, 256 tareas, plazo 570 s más parada, red privada y restricciones de escritura. Se conservaron propiedades, preflight y observaciones del cgroup efectivo. El controlador interno no resuelve el cgroup real: sus campos aparecen null y conserva admisión por disponibilidad global. Los contadores durante ejecución no muestran OOM; los finales no son legibles tras retirarse el directorio. No se acredita una prueba de saturación de la cuota. RSS y MemoryPeak tienen ámbitos distintos.

### 17.4. Decisión y continuidad

Inscribir S39 revisión 17, cierre acotado TT-0012, RETP-2026-270, PTA-2026-012 y remisión a PTA-SVM-003. Formular después un banco pequeño de tareas diversas con oráculos y presupuestos antes de integrar conversación. El siguiente banco no se declara ejecutado. Qwen/B conserva cierre parcial; S42/TT-0010 siguen referidos al PC. Sin modificación de contratos, gramática, IR, núcleo, fuentes rectoras ni mapa HTML.

[Control documental de esta recepción](CONTROL_OC01_OC02_2026_09_24.md). Los apartados 1–16 conservan sus cortes; §17 gobierna esta nueva recepción.

## 18. Optimización CPU MXFP4 y ejecución residente · 24/09/2026

**Registro:** 2026-09-24T12:43:08.557Z. Unidad de ejecución experimental. **Base:** VERIFICACION_ACOTADA sobre Lenguaje 32bf520f6e6c63dae84ef299957b6fd0b8c20402 y [Motor d4e62b29713a2044be0d1d4a7fb463155d3999c3](https://github.com/juantoniolloretegea/SV-motor/blob/d4e62b29713a2044be0d1d4a7fb463155d3999c3/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/onecloud-2026-09-24/optimizacion/RESULTADO.md).

### 18.1. Diseño y hechos recibidos

Autorización de dos ventanas consecutivas de una hora desde 11:58 UTC: optimización Rust y residencia hasta 12:58; llama.cpp condicional hasta 13:58. Sin adquisición adicional. Protocolo publicado antes de inferencia en 7a3a353; corrección de ubicación documental en 388933b sin alterar criterios.

Un mismo motor permite seleccionar referencia secuencial y variante paralela sin copias completas de bloques y escalas contiguos. Mismos pesos, tokenizador, parámetros y trabajadores. Tres solicitudes C01 de referencia y siete candidatas en dos sesiones residentes, una inferencia a la vez. Tres pruebas numéricas específicas, nueve guardas y Cargo check aprobados. Rust 1.98.0; sin Python.

### 18.2. Dictamen acotado

Cinco casos correctos, incluidas las tres repeticiones C01. Mediana HTTP 94,002 a 18,168 s: factor 5,174, reducción 80,673 %. Se satisface el umbral previo de reducción mínima del 50 %. Banco candidato finalizado a las 12:27:20,066 UTC, dentro de la primera fase. **Aceptación experimental acotada de la revisión. No se activa llama.cpp.**

Los 26 controles instrumentales de originales aprueban; el juicio semántico se registra aparte. Todas las respuestas terminan con stop. Los dos hijos y servicios están detenidos, sin errores; contadores finales de la cuota exterior sin eventos max, oom ni oom_kill. La máquina continúa disponible, sin modelo residente de esta campaña.

### 18.3. Objeciones y límites

Tres repeticiones de una tarea temporal y cinco casos sintéticos, contexto 1024, orden fijo, sin aleatorización ni control exhaustivo de cachés o carga externa. Se comparan juntas retirada de copias y paralelización; no se atribuye una fracción causal separada a cada una. La residencia se observa, pero su ganancia frente a recargar por petición no se mide de forma independiente. RSS y memory.peak pertenecen a ámbitos distintos. No hay suite integral ni certificación general, conversacional o productiva.

### 18.4. Trazabilidad y continuidad

S39 revisión 18, RETP-2026-271, PTA-2026-013 y PTA-SVM-004; [control documental](CONTROL_OPTIMIZACION_2026_09_24.md). TT-0012 conserva su cierre material; S39 permanece abierto para otra fase con protocolo y plazo nuevos. Qwen/B, S42/TT-0010, otros sucesos, rectores, núcleo y mapa HTML conservan sus alcances. No se crean ramas adicionales ni se compran recursos. Los apartados anteriores mantienen sus cortes históricos.

## 19. Calidad parcial y suspensión de la integración conversacional · 24/09/2026

Registro 2026-09-24T15:04:15.258Z; 17:04:15, Europe/Madrid. Unidad de ejecución experimental; VERIFICACION_ACOTADA sobre Lenguaje 13e5becc550b731327ff33e291f79d884eee4b9b y [Motor dd4beaf5cf59c4114925fdece05bfe7885190395](https://github.com/juantoniolloretegea/SV-motor/blob/dd4beaf5cf59c4114925fdece05bfe7885190395/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/conversacion/RESULTADO_PARCIAL.md).

Doce tareas breves terminan normalmente con contenido correcto; diez cumplen formato estricto. Q07 contiene dos espacios al final de la primera línea y Q11 añade explicación al resultado correcto de 280 cm. Latencias totales 24,017–85,650 s, mediana descriptiva 31,454 s sobre tareas heterogéneas. Una ejecución por tarea; sin estimación de calidad general.

La interfaz derivada de Qwen y su exportación se comprobaron por acceso privado antes de detenerla. 0.2.0 aprobó 18 pruebas unitarias y controles del servicio; 0.2.1 aprobó 19 pruebas y compilación, pero su guardia detectó IDs especiales diferentes de los declarados y abortó. La serialización inicial utilizaba además un cierre de historial no reconocido. M01/M02 se conservan como evidencia de esa integración; M03 se interrumpió. L y D no comenzaron. Los IDs de vista previa no acreditan IDs del motor: se enviaban textos, no esos números.

El bloqueo posterior del terminal del Codespace procede de una revisión automática de seguridad que alegó rechazo previo del origen. La recarga anterior rechazada correspondía a chrome-error por protocolo no autorizado. No se elude la restricción. La infraestructura del asistente y las incidencias de integración se distinguen del modelo. Servicio y motor detenidos en el último corte remoto; URL pendiente de entrega.

S39 revisión 19, Acta004 §19, RETP-2026-272, PTA-2026-014 y PTA-SVM-005. TT-0012 conserva el cierre acotado anterior; S39 permanece abierto. Se conservan otros sucesos, rectores, núcleo, mapa HTML y resultados adversos. Sin compras ni ramas adicionales. La continuación requiere corregir y verificar el tokenizador y reanudar solo M/L y D, sin repetir Q; plazo común 16:38:50 UTC. Si vence, se registra la parte no ejecutada y se acuerda otra ventana.

[Control documental](CONTROL_CONVERSACION_2026_09_24.md). El resultado parcial y el punto de continuidad prevalecen sobre cualquier descripción de disponibilidad anterior.

## 20. Reanudación controlada y comparación con preguntas recuperadas · 24/09/2026

Registro 2026-09-24T15:55:08.658Z; 17:55:08, Europe/Madrid. Unidad de ejecución experimental. VERIFICACION_ACOTADA sobre Lenguaje 28879943c8f403fe63d29f032f7c569bdfd8aafb y [Motor eafaf3711df15f4fca289d156e7333576144455a](https://github.com/juantoniolloretegea/SV-motor/blob/eafaf3711df15f4fca289d156e7333576144455a/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/conversacion/CONTINUIDAD.md).

La autorización expresa permitió recuperar el acceso al Codespace, que estaba detenido. Se reanudó por el control normal, sin eludir restricciones. La versión 0.2.2 completa los IDs ausentes a partir de las declaraciones del tokenizador, conserva entradas originales y mantiene las guardias. Compilación y 21 pruebas aprobadas. Servicio disponible desde las 15:40:50 UTC.

El control-03 se rechazó por reutilizar un identificador de petición antiguo: comportamiento correcto del servicio. Un auxiliar con identificador nuevo aprobó control-04, incluida cancelación y parada comprobada. La telemetría mantiene OpenTelemetry Rust y añade el muestreo del cgroup del motor, separado del árbol del servicio. Una muestra observó el PID del motor, CPU, RSS, hilos, E/S y sockets sin lagunas declaradas; no se extrapola a observación exhaustiva. Las necesidades de custodia independiente, contrato general y gráfica siguen delimitadas.

Banco M/L iniciado a las 15:46:20 UTC; M04 correcto y L0256 correcto, resto pendiente. El procedimiento secuencial espera su cierre antes de D, conservando el plazo anterior. Luego abre la comparación expresamente solicitada con máximo 60 minutos. Se recuperaron dos rondas de nueve y dos preguntas y cuatro consultas de OP-IMM-001-P10@1.0. Las preguntas y criterios se publicaron antes de la nueva ejecución. No se ha recuperado una tercera ronda ni se declara ejecutada la comparación.

Las condiciones de Qwen y GPT-OSS son diferentes; se preserva esa limitación. Se conservan antecedentes reales, límites de salida, rechazos y resultados adversos. La interfaz respondió y mostró los expedientes previos; la campaña sigue en curso y no constituye recepción final.

S39 revisión 20; Acta004 §20; RETP-2026-273; PTA-2026-015. TT-0012 conserva su cierre material acotado. S39 permanece abierto; sin modificación doctrinal, compras ni ramas adicionales.

## 21. Recepción de contexto y documental; recuperación de la comparación · 24/09/2026

Registro 2026-09-24T17:58:17.005Z; 19:58:17, Europe/Madrid. Unidad de ejecución experimental. VERIFICACION_ACOTADA sobre Lenguaje a85d4d36b952360c7b53c3bc9f519f78a545d784 y [Motor a98825f24c7865a80e9aa35d6915c4ac93abdd04](https://github.com/juantoniolloretegea/SV-motor/blob/a98825f24c7865a80e9aa35d6915c4ac93abdd04/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/conversacion/INCIDENCIA_EXPORTACION.md).

La recepción de originales acredita M01–M04 y las cuatro condiciones L completas; la entrada mayor tuvo 3096 tokens y una latencia total de 891,512 segundos. Se distingue recuperación sintética de identificador de comprensión general. D01–D04 terminaron correctamente; D05–D08 no se admitieron por plazo. Las cuatro consultas documentales del pasaje OP-IMM-001-P10@1.0 fueron conformes. Qwen incumplió formato estricto en cuatro, aunque DOC02 expresó correctamente ausencia de respaldo. Cambian modelo y condiciones; no se atribuye causalidad exclusiva ni se valida todo el universo.

El corte del asistente dejó la campaña remota conservando resultados. R02-01 produjo una respuesta truncada de 256 tokens; el cierre OpenTelemetry duplicó 35 959 bytes de datos del motor y sobrepasó la cota individual. R02-02 fue rechazada antes de inferencia. Se conservaron respuesta, expediente, registros y trazas, separando el defecto instrumental del contenido del modelo. El observador separado seguía operativo en la recepción.

La versión 0.2.3 mantiene el valor completo en el expediente y exporta resumen numérico, tamaño y huella SHA-256. Se conservan las cotas y la guardia. Las 22 pruebas Rust 1.98.0 aprobaron, incluida reproducción y corrección del defecto. Las huellas desplegadas coinciden. Servicio y motor anteriores se detuvieron con MainPID=0; ejecutables conservados. Servicio nuevo iniciado a las 17:51:04 UTC.

El guion inicial de continuación falló por omitir src/ en la ruta de preguntas, sin admitir inferencia. Corregido y conservado el diagnóstico, el controlador empezó a las 17:55:35, manteniendo el límite 19:52:28 UTC registrado a las 17:52:28. R02-02 utiliza la misma conversación tras igualdad estructural con el antecedente; no se repite ni sustituye R02-01. Plazo individual nuevo 900 segundos, sustentado por la latencia medida, manteniendo 256 tokens y contexto 4096. URL privada conectada a las 17:56:21; no se lanzó inferencia web competidora. Comparación y recepción final todavía pendientes.

S39 revisión 21; Acta004 §21; RETP-2026-274; PTA-2026-016. TT-0012 conserva su cierre material acotado. S39 permanece abierto; sin modificación doctrinal, compras ni ramas adicionales.


<a id="cierre-gpt-oss-20260925"></a>

## 22. Cierre experimental y distribución nativa de GPT-OSS · 25/09/2026

**Recepción:** 2026-09-25T11:17:31.291Z. **Base:** Lenguaje f517543c592d558d235821d5b0c0293fc95cbcab; Motor 365900cbe47b085c487ffd7b72391f0b13b679bc. **Unidad:** Unidad de ejecución experimental S39. **Lectura:** VERIFICACION_ACOTADA a fuentes, evidencias y registros de este cierre; no se declara nueva lectura integral del corpus ni se modifica doctrina.

Cierre de la campaña nativa y entrega experimental GPT-OSS 0.2.4-beta.1. Doce condiciones documentales conformes en cinco parámetros; doce consultas sintéticas HCL conservadas, con diez terminaciones normales y dos por límite de generación. Evaluación asistida inicial: siete no conformes y cinco en revisión. Sin aptitud clínica acreditada.

VERIFICACION_ACOTADA: cotejo Rust 1.98.0 de doce conversaciones y 49 sucesos; restauración de un expediente con doce conversaciones y cero peticiones pendientes, copia idéntica al original; ejecutables, archivos y recepción cotejados mediante SHA-256. No se ha ensayado instalación completa en una sede limpia ni reconstrucción hermética.

La comparación conversacional histórica termina con once preguntas previstas, diez turnos recibidos —incluido el antecedente— y una pregunta no admitida. Nueve respuestas quedaron limitadas a 256 tokens y un intento agotó su plazo sin texto entregado. Los datos personales de ese conjunto no se publican. La nueva evidencia HCL utiliza casos sintéticos y conversaciones independientes. Los bancos tienen contratos y condiciones diferentes y no se suman como tasa global de acierto.

La distribución conserva binarios, fuentes, tokenizador, Harmony, servicios e instrucciones; los pesos quedan externos y fijados por tamaño y SHA-256. Se distinguen restitución documental acreditada, instalación completa en una sede nueva no ensayada y construcción hermética no acreditada. Se mantiene la discrepancia de rótulo 0.2.2 frente a versión 0.2.4 y la semilla no transmitida.

Sin nuevas inferencias, sin eliminación de archivos y sin cambios de Qwen. Pesos externos fijados por revisión, tamaño y SHA-256. Semilla registrada pero no transmitida al motor; rótulo interno 0.2.2 conservado en aplicación 0.2.4. Conversaciones personales históricas excluidas de la entrega pública. Servicios de OneCloud activos en la observación de 11:09:52 UTC; no se garantiza disponibilidad indefinida.

**Decisión:** TT-0013 finalizado en el alcance recibido; S39 pendiente para su objeto restante. Definir el siguiente objeto experimental de la adenda a partir del cierre conservado. No reabrir las campañas terminadas ni iniciar migración, virtualización o vía A/WebAssembly como consecuencia automática de esta recepción.

**Trazabilidad:** [informe](https://github.com/juantoniolloretegea/SV-motor/blob/365900cbe47b085c487ffd7b72391f0b13b679bc/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/distribucion/0.2.4-beta.1/RESULTADOS.md); [entrega](https://github.com/juantoniolloretegea/SV-motor/releases/tag/gpt-oss-conversacion-v0.2.4-beta.1); [TT-0013](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0013.md); S39 revisión 22; RETP-2026-275; PTA-2026-017; PTA-SVM-007. El registro canónico se refleja en la copia del laboratorio; las recepciones históricas se conservan.


## 23. Preparación del candidato GPT-OSS-120B · 27/09/2026

**Registro:** 2026-09-27T07:40:39Z. **Seguimiento:** S39 revisión 24; TT-0015; RETP-2026-276. **Alcance:** actualización documental localizada, sin nueva ejecución material.

Se recibe la decisión de continuar por la vía B nativa con GPT-OSS-120B como candidato. La primera fase verifica compatibilidad y recursos; la instalación y selección posterior quedan condicionadas a su resultado y a la autorización de los recursos necesarios. El modelo permanece sin evaluación propia y no se declara Apto.

GPT-OSS-20B conserva su cierre. La selección mínima Qwen3.8-27B finalizó con respuesta completa en 227 segundos y resultado No pasa por incumplimientos documentales; esta referencia no generaliza a su familia ni abre una repetición. [Expediente restringido](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/7a116cddac3f0b98bdc52fb84e1ed1e0f2b23791/respuestas-ejecucion/QWEN38-SELECCION-MINIMA-LOCAL-20260927/entrega-01/INFORME.md). TT-0014 conserva pendiente su recepción; la publicación MCP 0.1.2 no acredita un nuevo modelo.

El [TT-0015](../../Inventario-sv/tiques-tecnicos/TT-0015.md) fija una comprobación de treinta minutos sin instalaciones ni inferencias. Examina la corrección CPU ya conservada, el soporte real del modelo, Harmony y el dimensionamiento. La [ficha del candidato](https://github.com/juantoniolloretegea/SV-motor/blob/3f24439cccb8c677c18e6ea2b2b4f13adf7a95ae/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-120b/README.md) separa datos publicados, observaciones históricas y hechos aún pendientes.

Inmunología y ciberseguridad conservan criterios y resultados diferenciados. La vía A/WebAssembly sólo podrá plantearse tras Apto en el alcance experimental nativo y autorización específica. No se amplía el Árbitro SV ni se incorpora nada al núcleo por esta preparación.

Se mantiene el circuito separado de encargos y respuestas, la recepción independiente y la conservación por commits. El mapa HTML no se modifica. Las figuras históricas se conservan; el índice del ensayo añade un esquema previsto para la continuación. Las copias históricas de laboratorio mantienen su corte, sin declarar sincronización nueva. S39 permanece pendiente de ejecución y recepción del nuevo alcance.

<a id="qwen80-instruct-cierre-20260930"></a>

## 24. Cierre de Instruct v7 y corrección documental · 30/09/2026

**Seguimiento:** S39, revisión 27 / TT-0016. **Naturaleza:** actualización basada en evidencia de ejecución y corrección documental solicitada expresamente; recepción humana pendiente. La información de este apartado prevalece para Instruct sobre los estados históricos de preparación y ejecución de los apartados anteriores.

### Evidencia recibida y alcance

El examen EVAL-PDQ-HCL-25-20260929/r1 tramitó sus 25 preguntas en cinco segmentos. Se conservaron 19 respuestas finales, seis impedimentos técnicos y ninguna pregunta sin ejecutar. P25 terminó a las 18:46:05.912 UTC; la guarda cerró conforme a las 18:46:07.897 UTC. El cierre posterior comprobó ausencia de procesos propios, carga deshabilitada y conservación de la instancia y los accesos administrativos. La [entrega 09](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/1b3e1f999acb6dc6493388b1e80424dc00dd51cb/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-09/INFORME.md), el [cotejo de originales](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/1b3e1f999acb6dc6493388b1e80424dc00dd51cb/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-09/RESULTADOS-COTEJADOS.json) y la [ficha del modelo](https://github.com/juantoniolloretegea/SV-motor/blob/6c68f4288e7356a8574c59d83448720269f96382/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3-next-80b-a3B-instruct/REGISTRO-INSTALACION-20260929.md) delimitan lo acreditado.

La continuación autorizada limitó errores instrumentales conocidos a su pregunta sin aceptar argumentos inválidos. Los seis impedimentos son P09, P12, P14, P15, P20 y P22. Los originales se conservan en cinco segmentos, sin repetir las inferencias concluidas ni sustituir las salidas. El cierre conforme indica conservación y terminación instrumental; no significa aprobado del examen.

### Corrección ternaria y criterio de aptitud

| Grupo | 0: acierto | 1: error penalizado | U: indeterminación | Impedimento fuera de terna |
|---|---:|---:|---:|---:|
| Total (25) | 7 | 2 | 10 | 6 |
| Críticas (20) | 6 | 2 | 8 | 4 |
| No críticas (5) | 1 | 0 | 2 | 2 |

La [corrección por pregunta](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/1b3e1f999acb6dc6493388b1e80424dc00dd51cb/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-09/RESULTADOS-P01-P25.md) contrasta clave previa, fuente congelada y solicitudes y respuestas MCP efectivas. Los dos errores propuestos, P07 y P19, corresponden a posiciones críticas; P19 queda señalada expresamente para revisión humana por la generalización absoluta de una afirmación de la fuente. Las diez U conservan abstenciones o respuestas insuficientes. Los seis impedimentos no se convierten en U, error ni acierto.

**Propuesta documental: No apto en el alcance examinado**, por incumplimiento de la condición crítica, pendiente de revisión humana. Se conserva el umbral del [protocolo](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/1b3e1f999acb6dc6493388b1e80424dc00dd51cb/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-09/PROTOCOLO-BANCO.md), floor(7n/9)=19, junto con la exigencia de todas las posiciones críticas en 0. No se atribuye un vector ternario completo ni se construye un polígono cerrado con las seis posiciones inválidas. La terminación de las 25 preguntas no acredita que las 25 sean evaluables. Tampoco acredita aptitud clínica o del dominio completo.

Los resultados quedan además en la [carpeta de pruebas del modelo, en Markdown](https://github.com/juantoniolloretegea/SV-motor/blob/6c68f4288e7356a8574c59d83448720269f96382/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3-next-80b-a3B-instruct/tests-y-pruebas-efectuadas/RESULTADOS-PDQ-HCL-25-20260930.md), con un JSON concordante. Esta sede permite comparar campañas sin depender de una interfaz activa.

### Ingeniería de contención y punto de retorno

La revisión mantiene la distinción entre fallo de recuperación, falta de evidencia, afirmación falsa y limitación del instrumento. Una búsqueda sin coincidencias no prueba ausencia del contenido en el corpus. La evaluación de este ensamblaje no aísla por sí sola el conocimiento interno del modelo. Las evidencias no justifican ampliar el núcleo, alterar el contrato del dominio ni considerar recibido el MCP por extensión.

S39 y TT-0016 permanecen pendientes de recepción humana. Los resultados en Markdown y JSON permiten inspeccionar cada original, referencia y fundamento. La dirección retiró la necesidad de revisión web; el servicio local quedó detenido y su presentación anterior se conserva como antecedente. La GUI egui y la composición de frames conservan su condición de necesidades posteriores: un frame Apto no garantiza un conjunto Apto.

Se han revisado las Actas 001–004. La 001 recibe remisión de continuidad; la 003, conciliación con S39; la 004 incorpora este cierre. La Acta 002 conserva su alcance de identidad y contrato de leyenda R06: esta campaña no aporta evidencia que modifique esa recepción. Los diagramas y el mapa HTML histórico quedan íntegros. Thinking y sus recepciones continúan por su expediente propio, sin intervención ni calificación nueva en esta actuación.

**Retorno:** revisión humana de la corrección y de los impedimentos → resolución documentada en TT-0016 y S39 → decisión sobre una nueva ronda identificada si procede. Ninguna repetición, instalación o ejecución queda autorizada por esta actualización documental.

## 25. MCP 0.1.3 y requisito de auditoría íntegra · 01/10/2026

MCP 0.1.3 implementado y comprobado localmente: 25 pruebas Rust conformes, paginación completa de coincidencias, eliminación del límite oculto de ocho palabras y reconstrucción determinista del recorrido documental. Integración completa con Safeguard todavía no ensayada; no se declara aptitud integral.

Un tramo relevante sin observación, trazabilidad o reconstrucción determina No apto para el uso exigido por el SV. La repetición admite variación de redacción, con invariancia del contenido exigido. Safeguard expone razonamiento: conservar íntegros análisis, respuesta final y transiciones, además de entradas y evidencias.

[Fuentes, pruebas y alcance](https://github.com/juantoniolloretegea/SV-motor/blob/9260fa886d7915330216d809eec16184facc8f16/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/model-context-protocol/0.1.3/LEAME.md); [adenda de integración](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/5b8888ee96b48b3119b61584985e0d802d4a366d/encargos-ejecucion/GPTOSS-SAFEGUARD-PREPARACION-20261001/v1/ADENDA.md). Asiento concordante en S39 revisión 28 y TT-0014. La conservación local del componente no acredita recepción completa de Safeguard. Las evidencias del modelo deben incluir análisis íntegro y decisión final, con su correspondencia documental. Se mantienen los resultados Qwen en sus expedientes; no se reabren inferencias ni se modifica el mapa histórico.

<a id="instruct-archivo-retirada-20261001"></a>

## 26. Archivo experimental y retirada de Instruct · 01/10/2026

La fase experimental de Qwen3-Next-80B-A3B-Instruct queda cerrada administrativamente el 01/10/2026, con **28/100 — No apto**, sin nuevas inferencias. Se mantienen 7 aciertos, 0 errores no críticos, 2 errores críticos (P07/P19), 10 U y 6 impedimentos técnicos (P09/P12/P14/P15/P20/P22) fuera de la terna. El umbral **T(25)=⌊7×25/9⌋=19** se distingue de la puntuación. No se emite κ de célula completa con seis posiciones sin adjudicación.

El diagnóstico v8 se conserva por separado: para P07-B, la ejecución propuso 0; la revisión documental propuso U por insuficiencia, al omitir la posible necesidad de biopsia exigida por la clave. Se mantienen ambas propuestas y su fundamento. Esta discrepancia no modifica el examen v7 ni abre otra inferencia.

La [release de archivo experimental — acceso restringido](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/releases/tag/qwen80-instruct-archivo-cierre-20261001-v1) conserva las evidencias sin pesos. Sus cinco adjuntos se descargaron desde GitHub a una carpeta independiente; se comprobaron SHA-256, contenidos comprimidos, segmentos reunidos y correspondencia con 26.139 rutas del inventario. Se conservan fuentes exactas, configuración y dependencias fijadas; 72 compilaciones derivadas quedan identificadas mediante sus huellas, sin prometer reproducción binaria idéntica. Véase el [cotejo de custodia — acceso restringido](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/8e8ea3c4ed5fc05daeb8ba82a7cf45c1faf0b328/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-11/COTEJO-RELEASE.json).

La retirada quedó confirmada por el proveedor a las 2026-10-01T09:49:42Z (1/10/2026, 11:49:42 CEST). La revisión posterior no encontró recursos residuales exclusivos en las categorías disponibles. El [acta de retirada — acceso restringido](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/8e8ea3c4ed5fc05daeb8ba82a7cf45c1faf0b328/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-11/RETIRADA.md) conserva la identidad y el registro técnico.

**S39 permanece abierto y TT-0014 pendiente.** TT-0016 cierra exclusivamente la fase administrativa de Instruct. La recepción científica independiente no se sustituye por este cierre, que no acredita validación clínica. Thinking y OpenAI conservan sus expedientes y no fueron intervenidos.

El hallazgo del recuperador se conserva en TT-0014: las variantes léxicas ensayadas produjeron búsquedas vacías; la entrega directa de página no acredita suficiencia universal de respuesta ni resolución del componente.

<a id="safeguard-instalacion-20261001"></a>

## 27. Instalación y contraste inicial de Safeguard · 01/10/2026

Instalación y contraste sintético inicial de Safeguard terminados: 87,5/100; No apto para el contraste delimitado; un incumplimiento crítico de lectura íntegra en C08; cobertura 8/8. Las ocho clasificaciones y citas son correctas respecto de los pasajes recibidos, pero C08 omite la página 0 de una sección de dos páginas. Carga completa, aislamiento probado y auditoría íntegra de C01-C07 (r3) y C08 (r4) conformes; 3673 tokens y cálculos finalizados, diez emisiones conservadas. Máximo conjunto: 111.693.459.456 bytes, sin intercambio ni agotamiento de memoria. Inferencia cerrada; servidor, pesos y accesos conservados. Entrega original y archivos cotejados desde GitHub; recepción independiente pendiente. No acredita aptitud clínica ni modifica otros expedientes.

S39, revisión 31, y [TT-0018](../../Inventario-sv/tiques-tecnicos/TT-0018.md) registran la actuación propia. TT-0014 permanece pendiente de recepción integral. [Correcciones y pruebas, acceso restringido](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/9b23fc5f59d602af8f31ed8666096c0d65471b2d/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-01/CORRECCIONES-INSTRUMENTALES.md). La ejecución y la recepción independiente conservan funciones distintas; no se declara aptitud clínica ni se cierra S39.

El banco sintético quedó fijado antes de generar respuestas: siete casos con pasajes recibidos por MCP, incluida una repetición, y un caso de recuperación autónoma. Se preservan las referencias reservadas fuera del candidato. La clasificación documental, la autonomía y la conformidad instrumental se han adjudicado separadamente: ocho etiquetas correctas, lectura autónoma incompleta y auditoría conforme de ambos tramos. Las incidencias que impiden obtener una respuesta completa quedan fuera de la terna; no se anticipa clasificación de célula SV.

[Entrega final cotejada, acceso restringido](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/9b23fc5f59d602af8f31ed8666096c0d65471b2d/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-01/INFORME-FINAL.md) · [Puntuación y adjudicaciones](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/9b23fc5f59d602af8f31ed8666096c0d65471b2d/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-01/PUNTUACION-FINAL.json) · [Custodia recuperada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/9b23fc5f59d602af8f31ed8666096c0d65471b2d/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-01/COTEJO-CUSTODIA.json). La inferencia ha terminado; se conservan instancia y accesos. La recepción independiente queda pendiente, sin cierre general de S39.

<a id="safeguard-lectura-integra-20261001"></a>

## 28. Contraste de recuperación documental íntegra de Safeguard · 2026-10-01T15:55:53Z

Contraste de recuperación documental íntegra Safeguard v2 autorizado expresamente y en ejecución, con banco nuevo SG-LECTURA-INTEGRA-20261001/r1 y exactamente tres casos autónomos. Corpus, afirmaciones, referencias reservadas, criticidad y configuración fijados antes de inferir. Contrato de páginas aclarado en la representación efectiva y observador pasivo incorporado; quince pruebas del conductor y veinticinco del MCP conformes. Tres rutas documentales completas comprobadas sin inferencia, con reserva mínima observada de 3869 tokens. Carga completa acreditada; primera secuencia en curso. El resultado anterior de 87,5/100 y C08 se conservan intactos. La nueva puntuación y el dictamen quedan pendientes del cierre y cotejo íntegros.

[Comprobaciones y admisión](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/b8db5c30c94863bc4fd95f050c9ae0b56f5d25d0/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-02/ADMISION-PREVIA.json). S39, revisión 32, conserva el seguimiento. La recepción independiente y las dependencias de los demás expedientes permanecen pendientes según sus propios registros. La aclaración de MCP y sus pruebas no constituyen una recepción integral de TT-0014 ni aptitud clínica.

### Reserva metodológica y avance del contraste de lectura íntegra · 2026-10-01T17:14:10Z

La secuencia v2 conserva dos finales sin repetición: L01 clasifica correctamente y cita la restricción, pero omite la página 1 exigida; adjudicación provisional 1 crítico. L02 lee las dos páginas, clasifica correctamente y cita regla general y excepción; adjudicación provisional 0. L03 está en ejecución con contexto nuevo. La adjudicación definitiva y la puntuación propia requieren el cotejo íntegro de cierre. Una revisión estática posterior a la fijación detecta un defecto del banco: consultas dirigidas pueden revelar cláusulas decisivas en fragmentos de 220 caracteres. Se rectifica la afirmación excesiva del protocolo, se conserva el banco sin cambios y se documenta la reserva metodológica. Esto impide declarar conformidad integral del diseño; no se corrige repitiendo resultados ni ampliando la campaña. El resultado previo de 87,5/100 y C08 permanecen intactos.

[Reserva y prueba reproducible](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/bb9158c52c1c21de691745d79ad21b9969286f73/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-02/RESERVA-DE-DISENO.md). S39, revisión 33. Se continúa únicamente L03 bajo las guardas vigentes; después corresponden cierre, custodia, adjudicación propia y recepción independiente. Esta actualización no cierra S39 general ni la recepción propia de TT-0014.

### Cierre del contraste de lectura íntegra · 2026-10-01T18:54:25Z

Contraste Safeguard v2 terminado: 66,67/100; No apto para este contraste. L01 conserva clasificación y cita correctas, pero omite la página 1: un 1 crítico. L02 y L03 reciben 0, con lectura íntegra de dos y tres páginas respectivamente; la elipsis explícita de la primera cita de L03 se acepta con cotejo de sus segmentos y conservación del resultado negativo de coincidencia continua. N=3, N₀=2, N₁=1, Nᵤ=0; sin errores no críticos ni impedimentos técnicos. Se conservan 2701 cálculos finalizados, 13 emisiones completas, 26 segmentos de canal, diez llamadas documentales y 4942 muestras de telemetría. Los siete archivos de la edición propia fueron descargados y cotejados en Rust. Se mantiene una reserva de diseño: las búsquedas dirigidas pueden revelar cláusulas decisivas en fragmentos breves; ese atajo no se materializó en el recorrido observado, pero impide declarar conformidad integral del banco. Inferencia terminada, carga deshabilitada y servidor, pesos y accesos conservados. El resultado anterior de 87,5/100 y C08 permanecen íntegros. Recepción científica independiente pendiente; sin aptitud SV general ni clínica acreditada.

[Informe y adjudicación](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/84028c8d2611ed762c11b21dd4d8e6d72e3c9c0b/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-02/INFORME-FINAL.md) · [Puntuación estructurada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/84028c8d2611ed762c11b21dd4d8e6d72e3c9c0b/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-02/PUNTUACION-FINAL.json) · [Custodia cotejada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/84028c8d2611ed762c11b21dd4d8e6d72e3c9c0b/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-02/COTEJO-CUSTODIA.json) · [Cierre material](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/84028c8d2611ed762c11b21dd4d8e6d72e3c9c0b/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-02/COTEJO-CIERRE.json).

S39, revisión 34; Acta 004 §28. Recepción científica independiente de entrega-02, incluidas adjudicación, elipsis, reserva de diseño y custodia. Mantener la inferencia detenida y preservar servidor, pesos y accesos. Retorno a expediente Safeguard, TT-0018, relación instrumental de TT-0014, S39 y Calidad. Otra campaña requiere su encargo y autorización propios. TT-0014 conserva su recepción propia pendiente. S39 general permanece en ejecución. Los cortes anteriores conservados son antecedentes y no autorizan reiniciar esta secuencia.

## 29. Contraste asistido único L01-ARBITRO · 02/10/2026

L01-ARBITRO terminado: 0, condición asistida conforme, 100/100 con N=1. Una carga y una generación; T1/S1 páginas 0 y 1 incorporadas íntegramente por el Árbitro. Clasificación CONTRADICHA, cita literal y justificación conformes. Cuarenta y siete pruebas Rust previas; núcleo, semántica e IR sin cambios. Custodia, plantilla, tokens y ambas fronteras MCP cotejados: 358 cálculos, una emisión, dos canales y 320 muestras. Cuatro archivos recuperados exactamente desde GitHub. Memoria conjunta máxima 114 GiB, con 24 eventos nuevos de presión, sin intercambio ni OOM. Inferencia cerrada, carga deshabilitada y servidor, pesos y accesos conservados. Se mantienen L01 autónomo, C08, los resultados de 66,67/100 y 87,5/100 y la reserva del banco. No acredita generalización, lectura autónoma ni aptitud clínica; recepción independiente pendiente.

[Informe](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/fc02eee0abfb49b9e653dc21e4540c8467a62e7e/respuestas-ejecucion/ARBITRO-SV-SAFEGUARD-20261002/entrega-01/INFORME-FINAL.md) · [Resultados](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/fc02eee0abfb49b9e653dc21e4540c8467a62e7e/respuestas-ejecucion/ARBITRO-SV-SAFEGUARD-20261002/entrega-01/RESULTADOS.json) · [Custodia recuperada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/fc02eee0abfb49b9e653dc21e4540c8467a62e7e/respuestas-ejecucion/ARBITRO-SV-SAFEGUARD-20261002/entrega-01/COTEJO-CUSTODIA.json) · [Cierre](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/fc02eee0abfb49b9e653dc21e4540c8467a62e7e/respuestas-ejecucion/ARBITRO-SV-SAFEGUARD-20261002/entrega-01/METRICAS-Y-COTEJO-CIERRE.json).

La integración utiliza las operaciones documentales públicas compile_svp y validate_bindings de LIG/0.1, con referencias exactas y bytes obtenidos por MCP. Se aplica la distinción del contrato CYB §§8.1–8.3: contraste documental sin constitución productiva R1. Se rectifica la dependencia indebida de esa constitución; no se modifican el núcleo ni su frontera de autoridad. La corrección semántica se adjudica externamente y no se simula una función canónica de clasificación.

S39, revisión 35; Acta 004 §29. Recepción independiente de L01-ARBITRO, su integración documental, adjudicación y custodia. Mantener la inferencia cerrada; conservar servidor, pesos y accesos. Retorno a expediente Safeguard, TT-0018, relación pertinente de TT-0014, S39 y Calidad. Se conservan las recepciones pendientes anteriores; otra prueba requiere encargo y autorización propios. TT-0014 conserva su recepción propia; S39 general permanece en ejecución.

## 30. Contraste previo v2 conservado y continuación v3 · 02/10/2026

Contraste previo de casos nuevos ARBITRO-SV-SAFEGUARD-20261002/v2 cerrado tras una carga y una generación N01. Acceso al examen no acreditado: N01 fuera de la terna por impedimento del verificador intermedio; N02–N06 no ejecutados. Puntuación auxiliar parcial 0/100 sobre N=6, sin interpretación como rendimiento del modelo. Corrección posterior autorizada, cuatro pruebas Rust y cotejo del punto original conformes; el fallo ocurrido se conserva. Custodia integral comprobada, 781 cálculos y tokens emitidos, 886 muestras; sin nueva inferencia. Núcleo, semántica e IR intactos. Servidor, pesos y accesos conservados; recepción independiente pendiente.

[Entrega-02 histórica](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/b85439113aa78754506c6eccdb861e6d89353d70/respuestas-ejecucion/ARBITRO-SV-SAFEGUARD-20261002/entrega-02/INFORME-FINAL.md). Los cuatro archivos de su edición fueron recuperados y cotejados en Rust; los 22 documentos publicados también coinciden exactamente. El impedimento permanece en el balance histórico.

La continuación v3 fue recibida y autorizada por la dirección. N01 se evalúa de forma diferida sobre el mismo original: 1 no crítico por literalidad y localización, con clasificación y fundamento sustantivo correctos; no se regenera ni se modifica. La admisión Rust conserva el Núcleo y sus 36 archivos, pesos y motor, y acredita la incorporación de las páginas, la plantilla, la tokenización, el aislamiento y la custodia del recorrido real sin modelo. [Admisión y límites](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/02674b777d6064acffc2aff30d913ad3d501f5c1/respuestas-ejecucion/ARBITRO-SV-SAFEGUARD-20261002/entrega-03/ADMISION-Y-CAMBIOS.md).

Primera condición ARBITRO-SV-SAFEGUARD-20261002/v3 cerrada: seis originales evaluables, N₀=4, N₁=2 (N01 formal y N06 crítico), Nᵤ=0; 50/100 y No apto para este contraste previo. N01 conserva su original de v2 y se evalúa de forma diferida; N02–N06 se generaron una sola vez en una carga adicional. El suministro documental y la custodia son conformes. N06 reconoce fuentes incompatibles sin precedencia, pero clasifica CONTRADICHA frente a EVIDENCIA_INSUFICIENTE. Sin seis ceros ni acceso acreditado al examen. Cierre material cotejado, servidor, pesos y accesos conservados; recepción independiente pendiente.

[Resultados y cierre cotejados](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/d56211a0aac5240a85217dd0635c072329ebc11c/respuestas-ejecucion/ARBITRO-SV-SAFEGUARD-20261002/entrega-03/INFORME-PRIMERA-CONDICION.md); [puntuación propia](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/d56211a0aac5240a85217dd0635c072329ebc11c/respuestas-ejecucion/ARBITRO-SV-SAFEGUARD-20261002/entrega-03/PUNTUACION-FINAL.json). Los 28 documentos publicados coinciden exactamente tras recuperación en Rust.

La condición SG-ARBITRO-CONSISTENCIA-20261002-r1 tiene hipótesis, siete entradas y referencias prefijadas. El primer arranque falló antes de cargar por una cardinalidad residual del custodio; se conservó, reprodujo y corrigió en Rust. La recuperación arbitro-consistencia-02 fue admitida con entradas idénticas y está en ejecución, con D01 conocido previo a seis casos nuevos, máximo una carga efectiva y siete generaciones. Cierre ante cualquier 1, U, blanco o impedimento. El resultado está pendiente; no constituye acceso al examen ni aptitud. [Admisión y recuperación](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/90160167a3af4930ef135d29c25207ac22ac9a50/respuestas-ejecucion/ARBITRO-SV-SAFEGUARD-20261002/entrega-03/condiciones/consistencia-01/MANIFIESTO-RECUPERACION-01.json).

Los resultados históricos de 87,5/100 y 66,67/100, el 0 asistido de L01-ARBITRO y el impedimento propio de v2 se mantienen separados. La evaluación diferida no constituye otra generación ni una recepción oportuna de v2. S39 permanece en ejecución; TT-0018 y TT-0014 conservan sus recepciones independientes pendientes. No se declara aptitud general ni clínica ni se modifica el Núcleo, la semántica o el IR.

## 31. Cierre del contraste v3 y diagnóstico de consistencia · S39 revisión 37

Primera condición ARBITRO-SV-SAFEGUARD-20261002/v3 cerrada: seis originales evaluables, N₀=4, N₁=2 (N01 formal y N06 crítico), Nᵤ=0; 50/100 y No apto para este contraste previo. N01 conserva su original de v2 y se evalúa de forma diferida; N02–N06 se generaron una sola vez en una carga adicional. El suministro documental y la custodia son conformes. N06 reconoce fuentes incompatibles sin precedencia, pero clasifica CONTRADICHA frente a EVIDENCIA_INSUFICIENTE. Sin seis ceros ni acceso acreditado al examen. Cierre material cotejado, servidor, pesos y accesos conservados; recepción independiente pendiente. D01 reproduce el error crítico: CONTRADICHA frente a EVIDENCIA_INSUFICIENTE, pese a reconocer fuentes incompatibles sin precedencia. El control externo cerró la condición tras una carga y una generación. F01–F06 no se ejecutaron; no acreditan rendimiento ni generalización. Sin nueva hipótesis causal discriminante, se cierra esta vía conforme al §7. Inferencia detenida, carga deshabilitada, servidor, pesos y accesos conservados; recepción independiente pendiente.

Nueve secuencias alcanzaron cálculo: dos iniciales inválidas y siete con diecinueve finales adjudicados, incluido D01 conocido. Este recuento no suma puntuaciones ni convierte antecedentes en casos independientes. F01–F06 permanecen no ejecutados.

El Director acredita suministro documental íntegro, entrada efectiva y custodia; no certifica corrección semántica. Las rectificaciones propias de cardinalidad y ruta del sello están identificadas y comprobadas en Rust. Núcleo, semántica e IR intactos. Tres unidades propias inactivas y enmascaradas, sin procesos ni sockets propios; servidor, pesos y accesos conservados.

Evidencias: [informe final](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/19778e3692802316c86b3a6107d625e0042cf95c/respuestas-ejecucion/ARBITRO-SV-SAFEGUARD-20261002/entrega-03/INFORME-FINAL.md), [diagnóstico](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/19778e3692802316c86b3a6107d625e0042cf95c/respuestas-ejecucion/ARBITRO-SV-SAFEGUARD-20261002/entrega-03/condiciones/consistencia-01/INFORME-FINAL.md) y [recuperación de los seis archivos nuevos](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/19778e3692802316c86b3a6107d625e0042cf95c/respuestas-ejecucion/ARBITRO-SV-SAFEGUARD-20261002/entrega-03/condiciones/consistencia-01/COTEJO-RECUPERACION-GITHUB.json). Veinticinco documentos del cierre recuperados y cotejados en Rust.

Retorno: recepción científica independiente de entrega-03 y antecedentes. Sin seis ceros no se habilita examen. S39 y los tiques conservan su estado abierto o pendiente; no se modifica el ámbito de otros modelos.

## 32. Conciliación de cierres y nueva preevaluación · 05/10/2026

**S39, revisión 38; RETP-2026-277. Corte documental: 2026-10-05T10:20:04Z.** Esta sección actualiza el seguimiento desde las entregas publicadas; no altera los resultados ni las condiciones de ejecución. Los estados precedentes son históricos y no reactivan campañas.

| Expediente | Resultado experimental | Conservación | Situación del recurso |
|---|---|---|---|
| [Safeguard / TT-0018](../../Inventario-sv/tiques-tecnicos/TT-0018.md) | Preevaluación cerrada; No apto para acceder al examen; A0–A3 sin mejora y diagnósticos agotados | Imagen documental cifrada recuperada y cotejada | Instancia y disco de origen eliminados el 04/10 |
| [Thinking / TT-0017](../../Inventario-sv/tiques-tecnicos/TT-0017.md) | Examen cerrado; No apto en las condiciones evaluadas por demoras operativas y falta de finalización fiable | Ediciones pública y privada publicadas; recuperación y cotejo conformes | Pendiente eliminar instancia y disco según la última evidencia |
| [Qwen3.5 Q8_0 / TT-0019](../../Inventario-sv/tiques-tecnicos/TT-0019.md) | Instalado en UpCloud, 48 CPU y 256 GB nominales; A01–A04 = 0 en A0, A04 crítico | Hitos parciales publicados; consolidación de fase pendiente | Realización activa; sin GPU; sin intervención por esta conciliación |

### 32.1. Safeguard

El [informe de cierre](https://github.com/juantoniolloretegea/SV-motor/blob/149c4b848475802942af35ab39e7335081398480/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-safeguard-120b/ensayos-reiterados-adversariales-y-aprendizaje/resultados/cierre-20261004/INFORME-FINAL.md) documenta ocho clasificaciones correctas de nueve y error crítico A08, con omisiones formales. A0–A3 conservan −88,89/100 y vector (1,1,1,1,1,1,1,1,1), T(9)=7. No hay mejora; tampoco correcciones o regresiones entre A2 y A3. Esfuerzo alto y antecedentes adicionales cambiaron conjuntamente, sin causalidad aislada acreditada.

D01 y D03 del diagnóstico posterior de A08 no corrigieron la clasificación; D02 quedó sin final por límite temporal y fuera de la terna. A3 conserva un agotamiento de memoria del custodio posterior a A09, sin cierre normal acreditado. Se mantienen las reservas sobre presentación del JSON, revisión contextual sin entrenamiento y equivalencia numérica no probada.

La [conservación privada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/releases/tag/safeguard-imagen-20261004-v1) y el [acta de eliminación](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/d7bae43cab1f900afc2dcc5e8c0e35c61215e5a3/respuestas-ejecucion/SAFEGUARD-CONSERVACION-20261004/ACTA-RETIRADA.md) acreditan hechos separados. Instancia y disco fueron retirados el 04/10; no se ha ensayado arranque restaurado. No procede A4, bloque B ni examen por este expediente.

### 32.2. Thinking

El [cierre y conservación](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/0212a5ae1ccd3d42b5a26796068c0a3981d380c5/respuestas-ejecucion/QWEN80-THINKING-Q4K-ONECLOUD-20260930/entrega-03/CIERRE-Y-CONSERVACION.md) y la [edición pública](https://github.com/juantoniolloretegea/SV-motor/releases/tag/qwen3-next-80b-a3b-thinking-archivo-cierre-20261005-v1) acreditan el resultado operativo desfavorable. Se conservan nueve finales pendientes de adjudicación de contenido, cuatro impedimentos terminales, P14 incompleta y once no ejecutadas. Denominador 25, sin puntuación global de contenido; los impedimentos no se convierten en U. No se atribuye toda la demora exclusivamente al modelo.

La [edición privada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/releases/tag/thinking-imagen-cierre-20261005-v1) fue recuperada, reconstruida y descifrada; el cotejo Rust comprende 164.854 entradas y 141.890 archivos ordinarios. No hay arranque restaurado ensayado. **Está cerrado el examen y está conservada la evidencia; sigue pendiente eliminar la instancia y su disco.** La última evidencia no acredita desaparición de la infraestructura ni cese de cargos. TT-0017 se finaliza en su alcance experimental y documental, con esa dependencia de infraestructura separada.

### 32.3. Qwen3.5 y MCP

La [recepción instrumental](https://github.com/juantoniolloretegea/SV-motor/blob/149c4b848475802942af35ab39e7335081398480/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3.5-122b-a10b-q8-0/seguimiento/RECEPCION_INSTRUMENTAL_20261004.md) acredita la nueva instalación CPU de 256 GB nominales, aproximadamente 251,65 GiB efectivos, y la respuesta mínima LISTO en 48,459 s; no demuestra velocidad de campaña. Identifica la corrección CPU Q8_0 r2, el tokenizador nativo GGUF y MCP 0.1.4-pdf.1. La lectura e instalación del corpus PDF no equivalen a comprensión por el modelo. [TT-0014](../../Inventario-sv/tiques-tecnicos/TT-0014.md) conserva su recepción integral pendiente.

El [vector del hito A04](https://github.com/juantoniolloretegea/SV-motor/blob/149c4b848475802942af35ab39e7335081398480/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3.5-122b-a10b-q8-0/preevaluacion-20261004/resultados/hitos/A04-A0/vector-parcial/CAPA.json) contiene cuatro ceros y cinco NE, con A04 crítico correcto. NE expresa falta de adjudicación y permanece fuera de 0/1/U. No hay κ ni puntuación global; no se habilita examen con un bloque incompleto. El [alcance del hito](https://github.com/juantoniolloretegea/SV-motor/blob/149c4b848475802942af35ab39e7335081398480/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3.5-122b-a10b-q8-0/preevaluacion-20261004/resultados/hitos/A04-A0/ALCANCE.md) separa publicación parcial de consolidación de fase y recepciones posteriores.

### 32.4. Dependencia y retorno

S39 permanece **en ejecución**, porque el candidato Qwen tiene evaluación propia en curso. Se preservan cotas originales, solicitudes y componentes activos. Retorno: cierre acotado de fase, conservación, cotejo y adjudicación, con recepción del alcance antes de decidir admisión. La eventual eliminación de Thinking requiere evidencia posterior; no es otra inferencia ni una condición añadida a Qwen.

Esta conciliación no modifica el Núcleo del SV, la semántica V0.2 ni la IR 0.3. Las necesidades derivadas de los ensayos quedan sujetas a revisión antes de incorporarse al Núcleo. Se mantienen mapa y diagramas históricos. Se concilian fuentes fijadas; no se repiten reconstrucciones de imágenes ni se certifica por ello corrección clínica, equivalencia numérica o restauración funcional.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
