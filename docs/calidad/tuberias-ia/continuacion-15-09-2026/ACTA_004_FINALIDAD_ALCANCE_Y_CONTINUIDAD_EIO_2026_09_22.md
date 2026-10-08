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

## 33. Componente de cálculo Rust para AMD · 05/10/2026

**S39 revisión 39; TT-0020; RETP-2026-278. Registro documental: 2026-10-05T18:29:13Z.**

Se incorpora CubeCL como componente de cálculo en evaluación, con rust-gpu como alternativa. [Estudio técnico](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/inferencia/cubecl-evaluacion-20261005/ESTUDIO.md) y [adenda AMD de acceso restringido](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/main/amd/estudio-uso-5-10-29-v1/calculo-rust-20261005/ADENDA.md). El mantenimiento observable respalda considerar ambos proyectos; la muestra reciente de CubeCL es más amplia, sin convertir recuentos en garantía de calidad o continuidad.

**Candidatos conservados en estudio:** [Kimi K3](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/kimi/kimi-k3/ESTUDIO-VIABILIDAD-20261005.md) y [GLM-5.3, con Flash como variante distinta](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/zai-org/glm-5.3/ESTUDIO-VIABILIDAD-20261005.md). La evaluación actual es documental y de viabilidad; no hay despliegue ni ensayo de respuestas. No se ha acordado su descarte definitivo. El cierre de la búsqueda técnica anterior y los impedimentos actuales no se convierten en un dictamen sobre su aptitud.

La preferencia provisional no acredita inferencia ni recepción. El código CubeCL examinado excluye MFMA/CDNA por LLVM; MI300X/gfx942 exige una adaptación y contraste delimitados. rust-gpu requiere comprobar controlador y extensiones matriciales efectivos. La publicación de SPIR-V 1.6 revisión 8 no prueba su realización en una GPU y no modifica la IR 0.3 del SV.

[TT-0020](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0020.md) permanece pendiente para las necesidades N-C01–N-C05: realización efectiva, operación f16/f32, referencia independiente Rust, coste medido y mantenimiento acotado. La evaluación documental está concluida. No hay desarrollo, instalación, ejecución GPU, inferencia ni gasto por esta incorporación. Una futura operación correcta tampoco acredita el motor completo ni habilita el examen.

Antes de una prueba material deben fijarse versiones, dependencias admisibles, casos, tolerancias y cotas. La propuesta se limita a una multiplicación matricial representativa. Si exige un compilador o arquitectura de modelo completos, se documenta el límite y vuelve a decisión de alcance. Los componentes de terceros conservan sus licencias.

El §32 y las revisiones anteriores se conservan como cortes históricos. La [retirada posterior de Thinking](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/2ddf814bb039b0c7876745023ea165baf38ed6f9/respuestas-ejecucion/QWEN80-THINKING-Q4K-ONECLOUD-20260930/entrega-03/retirada-20261005/ACTA-RETIRADA.md) acredita la eliminación administrativa de instancia y almacenamiento asociado; esa reserva anterior queda superada por su acta. El arranque restaurado sigue sin ensayarse. Los restantes resultados y expedientes experimentales conservan sus controles, cotas y recepciones propios, sin transferencias de aptitud o ampliación por el nuevo componente.

La comprobación de esta incorporación concierne a concordancia, trazabilidad, preservación y recuperación documental en Rust. Se distingue de exactitud numérica, rendimiento y recepción científica. Núcleo del SV, semántica V0.2 e IR 0.3 intactos; mapa y diagramas históricos preservados. Retorno: prueba técnica delimitada y recibida antes de proponer cualquier integración.

## 34. Recepción matricial AMD y estudio de Z.ai — GLM-5.3-Flash · 06/10/2026

**S39 revisión 40; TT-0020; RETP-2026-279. Registro: 2026-10-06T01:23:04Z.** Se incorpora el resultado recibido después del corte del §33 y la apertura del nuevo estudio autorizado. Los apartados anteriores conservan sus fechas y alcance históricos.

### 34.1. Recepción instrumental delimitada

La [entrega AMD-CUBECL-MFMA-20261005/r1](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/tree/714246c09a160740295f03ae5fb1381dd7a29b15/respuestas-ejecucion/AMD-CUBECL-MFMA-20261005/entrega-01) acredita el producto f16/f16 con acumulación y salida f32, ejecutado mediante CubeCL–LLVM–MFMA en MI300X/gfx942: diez casos, sesenta salidas y referencia independiente Rust con tolerancias prefijadas. La adaptación corresponde a MFMA manual 16³; dimensiones ensayadas hasta 512³ y casos especiales. La custodia publicada acredita 617 contenidos. La retirada del recurso temporal consta en su acta.

La revisión independiente posterior leyó fuentes y documentos y cotejó en Rust los 617 contenidos ya recuperados y las sesenta salidas frente a referencias f64 conservadas. No repitió la descarga de quince fragmentos, el cálculo de referencias, una ejecución GPU o un arranque restaurado. Se conservan las reservas de medición temporal, memoria muestreada y reconstrucción completa fuera de línea no acreditada. La recepción es favorable dentro de esa cobertura, sin garantía general del motor.

### 34.2. Encargo autorizado e iniciado

[ZAI-GLM53FLASH-RUST-AMD-20261006/r1](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/fd662a62e6d9f6346ae7a5f3fbba8aaffd95fcb4/encargos-ejecucion/ZAI-GLM53FLASH-RUST-AMD-20261006/v1/ENCARGO.md), con [recuperación y cotejo del encargo](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/c3e64e31a90485104c41c7a82a723f7fd4b4983a/encargos-ejecucion/ZAI-GLM53FLASH-RUST-AMD-20261006/v1/COTEJO-PUBLICACION.json). Identificación: **Z.ai — familia GLM — GLM-5.3-Flash**, `zai-org/GLM-5.3-Flash`; editor Zhipu AI (Z.ai). Distinguir modelo base, representación cuantizada, motor y proveedor. El estudio se ha transmitido y su recepción e inicio han sido confirmados; el dictamen permanece pendiente.

Máximo tres horas de trabajo activo, con treinta minutos finales para informe y custodia. Objeto: cobertura del recorrido completo en Rust, mantenimiento de sus componentes, memoria de pesos, estados y espacios temporales, y magnitud real de la adaptación. La capacidad gráfica y la RAM se dimensionan por separado. La ejecución matricial previa no demuestra esas condiciones.

Esta fase no activa servidores, descarga pesos, ejecuta inferencias ni habilita examen. Se exige una conclusión única: vía concreta con adaptación delimitada y propuesta de una prueba material; desarrollo sustancial necesario; o impedimento/evidencia insuficiente. La propuesta material vuelve a decisión antes de ejecutarse. No se abre una sucesión de familias o pruebas por inercia.

### 34.3. Tique, dependencias y retorno

TT-0020 pasa a **en ejecución**, con N-C01–N-C05 reutilizadas y recepción integral pendiente; la tabla del tique distingue avances limitados de cada necesidad. S39 mantiene **en ejecución**. Retorno: recibir el informe y su custodia, revisar sus límites y decidir la siguiente actuación. No se declara aptitud de Z.ai, Kimi o Qwen.

Los expedientes experimentales restantes conservan sus últimos cierres y recepciones propios. Núcleo del SV, semántica V0.2, IR 0.3, mapa y diagramas históricos intactos. No se incorporan necesidades al Núcleo por esta recepción.

## 35. Recepción del estudio Z.ai — GLM-5.3-Flash · 06/10/2026

**S39 revisión 41; TT-0020; RETP-2026-280. Registro: 2026-10-06T01:55:09Z.** Se recibe la [entrega ZAI-GLM53FLASH-RUST-AMD-20261006/r1](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/f4a05ba19ef1907e8e87d13547605e3bfc2efab1/respuestas-ejecucion/ZAI-GLM53FLASH-RUST-AMD-20261006/entrega-01/INFORME.md), cerrada dentro de la cota documental. Dictamen: **Desarrollo sustancial necesario**.

La multiplicación matricial antecedente conserva su resultado acotado. Para el modelo completo faltan la composición KDA–MLA/DSA, las conexiones mHC, el cargador de los tipos cuantizados exactos y la recepción numérica de estados, expertos y respuesta. No se recomienda un despliegue inmediato ni otra prueba GPU aislada bajo este encargo. El dictamen no es una clasificación «No apto», no atribuye U y no habilita examen.

La revisión independiente recuperó los dieciséis archivos (132549 bytes), verificó mediante Rust sus huellas y el manifiesto de trece contenidos, y reprodujo literalmente el cálculo de memoria y su contraste. La tabla contiene 141 filas y seis escenarios aritméticos. Cuatro fuentes primarias fijadas se releyeron y cotejaron: registros de modelos de mistral.rs y Candle, realización de referencia Transformers leída como documentación y base ROCm de Burn. No se reproduce aquí toda la investigación de bibliotecas ni se ejecuta código Python, GPU o modelo.

Los pesos se conocen por metadatos de procedencia, sin descarga ni equivalencia numérica recibida. El presupuesto de VRAM y RAM mantiene una reserva del 20 % por separado. Los cálculos acreditan sólo escenarios: materializar puntuaciones para todo el contexto excede el presupuesto preliminar desde 8192 tokens; una realización por bloques podría reducir los temporales, pero no está realizada ni medida. Se conservan las incidencias instrumentales documentadas del contraste y la diferencia entre integridad y capacidad efectiva.

El estudio está concluido y recibido. TT-0020 pasa a **pendiente** por integración sustancial no iniciada; N-C01–N-C05 no se cierran globalmente. S39 conserva su seguimiento general **en ejecución**, sin implicar inferencia o actividad material en este estudio. Retorno: decisión de alcance o nueva evidencia concreta antes de cualquier continuación. No hay nueva instancia, descarga de pesos, motor desarrollado ni seguimiento automático.

Los §§1–34 y registros históricos se preservan. Núcleo, semántica V0.2, IR 0.3, mapa y diagramas históricos intactos; los otros expedientes mantienen sus propios resultados y recepciones.

## 36. Cierre y conservación de Qwen3.5-122B-A10B Q8_0 · 06/10/2026

**S39 revisión 42; TT-0019; Acta 004 §36; RETP-2026-281. Antecedentes, licencias y diagramas preservados. Registro: 2026-10-06T16:34:52.8103019Z.**

Qwen3.5-122B-A10B Q8_0: preevaluación cerrada, admisión no acreditada por impedimento temporal. Siete respuestas A0, N0=6, N1=1 (A06 crítico), NU=0; dos casos no ejecutados. Sin κ, puntuación global, revisiones B, integración generativa PDF o examen. Imagen cifrada sin pesos y complemento recuperados, reconstruidos y descifrados; contenido e inventario cotejados con Rust. Retirada administrativa pendiente; servicio detenido y originales conservados en origen. No se declara cese de cargos. Arranque restaurado no ensayado.

[Informe científico fijado](https://github.com/juantoniolloretegea/SV-motor/blob/11a84870221355a8b5f9b22db67d574f8bb3f1eb/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3.5-122b-a10b-q8-0/preevaluacion-20261004/resultados/cierre-r1/INFORME-FINAL.md) · [Edición pública de cierre](https://github.com/juantoniolloretegea/SV-motor/releases/tag/qwen3.5-122b-a10b-q8-0-archivo-cierre-20261006-v1) · [Conservación cifrada de acceso restringido](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/releases/tag/qwen35-122b-q8-imagen-cierre-20261006-v1). Los siete originales y sus adjudicaciones permanecen intactos. A06 es error sustantivo crítico; ausencia de datos no constituye negación del fenómeno. Las incidencias de transporte de conservación se documentan separadamente y no readjudican la campaña.

La recepción de la nueva imagen verifica instalación, inventario y contenido, con pesos identificados sin duplicación. Se conserva la diferencia entre copia documental, recuperación estructural y arranque restaurado no ensayado. El servicio detenido no equivale a recurso dado de baja.

Retirada administrativa pendiente; servicio detenido y originales conservados en origen. No se declara cese de cargos.

TT-0019 finaliza en alcance experimental y de conservación; la recepción científica independiente pendiente no se presenta como realizada. S39 general conserva en ejecución. Conservar el expediente cerrado y sus reservas. A01–A03 conservan recepción independiente favorable; A04–A07 y fase permanecen pendientes al último corte competente. Sin nuevas inferencias o examen por este asiento. Los demás expedientes y estudios mantienen sus propios alcances.

Se mantiene la recepción previa de GLM-5.3-Flash y su impedimento de integración. No se transfieren resultados entre modelos. Núcleo, semántica V0.2, IR 0.3, §§1–35, antecedentes y diagramas preservados. La fecha de esta constancia es 06/10/2026; no se atribuye un cierre al 15/09/2026.

## 37. Retirada posterior de Qwen3.5-122B-A10B Q8_0 · 06/10/2026

**S39 revisión 43; TT-0019; Acta 004 §37; RETP-2026-282. Antecedentes, licencias y diagramas preservados. Registro: 2026-10-06T18:09:28.3464564Z.**

Qwen3.5-122B-A10B Q8_0: preevaluación cerrada, admisión no acreditada por impedimento temporal. Siete respuestas A0, N0=6, N1=1 (A06 crítico), NU=0; dos casos no ejecutados. Sin κ, puntuación global, revisiones B, integración generativa PDF o examen. Imagen cifrada sin pesos y complemento recuperados, reconstruidos y descifrados; contenido e inventario cotejados con Rust. Instancia y disco exclusivo retirados; desaparición cotejada en inventarios administrativos. Arranque restaurado no ensayado.

[Informe científico fijado](https://github.com/juantoniolloretegea/SV-motor/blob/11a84870221355a8b5f9b22db67d574f8bb3f1eb/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3.5-122b-a10b-q8-0/preevaluacion-20261004/resultados/cierre-r1/INFORME-FINAL.md) · [Edición pública de cierre](https://github.com/juantoniolloretegea/SV-motor/releases/tag/qwen3.5-122b-a10b-q8-0-archivo-cierre-20261006-v1) · [Conservación cifrada de acceso restringido](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/releases/tag/qwen35-122b-q8-imagen-cierre-20261006-v1). Los siete originales y sus adjudicaciones permanecen intactos. A06 es error sustantivo crítico; ausencia de datos no constituye negación del fenómeno. Las incidencias de transporte de conservación se documentan separadamente y no readjudican la campaña.

La recepción de la nueva imagen verifica instalación, inventario y contenido, con pesos identificados sin duplicación. Se conserva la diferencia entre copia documental, recuperación estructural y arranque restaurado no ensayado. El servicio detenido no equivale a recurso dado de baja.

Instancia y disco exclusivo retirados; desaparición cotejada en inventarios administrativos.

TT-0019 finaliza en alcance experimental y de conservación; la recepción científica independiente pendiente no se presenta como realizada. S39 general conserva en ejecución. Conservar el expediente cerrado y sus reservas. A01–A03 conservan recepción independiente favorable; A04–A07 y fase permanecen pendientes al último corte competente. Sin nuevas inferencias o examen por este asiento. Los demás expedientes y estudios mantienen sus propios alcances.

Se mantiene la recepción previa de GLM-5.3-Flash y su impedimento de integración. No se transfieren resultados entre modelos. Núcleo, semántica V0.2, IR 0.3, §§1–36, antecedentes y diagramas preservados. La fecha de esta constancia es 06/10/2026; no se atribuye un cierre al 15/09/2026.

## 38. Nodo 03: entrega estructurada de GPT-6 Astra y separación de fases

**S39 revisión 44; TT-0021; RETP-2026-283. Registro 2026-10-06T20:38:37Z.**

Nodo 03, GPT-6 Astra: tercera prueba instrumental concluida, una consulta artificial, JSON válido con justificación, referencias, código propuesto y límites. 17,910 s; 1189 tokens; 599 eventos y 79 muestras; 22 archivos cotejados en Rust. Resumen opcional no recibido. Falso positivo de referencias del comprobador aclarado fuera de línea, conservando originales. No hay catálogo, adversariales ni examen ejecutados.

[Informe instrumental fijado](https://github.com/juantoniolloretegea/SV-motor/blob/f6fc0b1640ac965a5497a14f3415e85ca2d64fc0/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/PRUEBA-ENTREGA-ESTRUCTURADA-20261006.md) · [Tique técnico](../../Inventario-sv/tiques-tecnicos/TT-0021.md).

Se distinguen declaración del modelo, resumen opcional del proveedor y evidencia medida por el cliente Rust. El JSON sí contiene justificación breve; el resumen opcional no llegó. El código generado permanece como texto. La alerta de referencia es un falso positivo del comprobador, conservado junto con su revisión; no se adjudica como error del modelo ni se reescribe la respuesta original.

Esta observación concluye sin evaluación científica del candidato. Recibir la adaptación al contrato científico, admisión de datos, referencias y control de consumo antes de A01. Mantener separados catálogo, revisiones adversariales y examen. No repetir la consulta trivial ni reabrir expedientes cerrados. Criptografía nativa y recepción integral pendientes. El polígono se presenta sólo con vector completo. La fidelidad no se deduce del formato JSON ni de consistencia superficial: depende de significado, condiciones, fuentes, negaciones e incertidumbre, con revisión competente.

El permiso temporal de créditos se restablece a desactivado y el receptor se cierra. Coste liquidado no comunicado. No hubo pagos, recargas, información sanitaria ni ejecución del código propuesto. Dependencia criptográfica C/ensamblador pendiente según excepción expresa. La publicación comprende informe y respuesta artificial; no custodia remota integral de los originales operativos.

La conciliación es posterior a la ejecución y conserva ese orden temporal. El tique permanece pendiente por recepción integral, no por una inferencia activa. Se mantienen los cierres y reservas de Qwen, Z.ai, MCP y demás expedientes. Núcleo, semántica V0.2, IR 0.3, §§1–37 y diagramas intactos.

## 39. Nodo 03: contrato de Astra por API y gobierno local del Árbitro-Director

**Fecha de asiento:** 2026-10-06T21:15:55Z. **Suceso:** S39, revisión 45. **Tique:** TT-0021, pendiente. **Evolución:** RETP-2026-284.

Astra por API con caché, control y evaluación en el SV, también en el examen. Árbitro-Director y auxiliares Rust conservan el gobierno; candidato sin acceso a clave, adjudicación, herramientas ni telemetría. Contrato científico conservado: dos páginas completas por caso, banco A/B, revisiones y criterios; anexo MCP/PDF separado antes del examen. Preparación local comprobada: 18 casos, 36 páginas previstas, política idéntica, siete pruebas Rust conformes y ninguna nueva inferencia.

La precisión humana prevalente permite el examen de Astra mediante API y mantiene la caché y el control en el SV. La carpeta del equipo propio desempeña la función de servidor de pruebas. El modelo recibe la documentación admitida, no la clave sellada ni los registros de medición; no ejecuta herramientas, no navega y no modifica la evaluación. El Árbitro y sus auxiliares Rust mantienen sus funciones; se comprueba únicamente la nueva adaptación técnica sin reabrir la validación del Árbitro.

Se conservan las fases y el banco. El anexo de capacidad MCP/PDF precede al examen, mantiene su propia evidencia y no altera preguntas ni puntuaciones. Las dos páginas del catálogo son lógicas; se distinguen de páginas físicas del PDF. La emisión explicativa del candidato se adjudica externamente y no constituye prueba de sus procesos internos.

[Contrato y constancia de preparación](https://github.com/juantoniolloretegea/SV-motor/blob/26c177b4f99352b2f6ab1bc1ba0dd9f03fb7dc71/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/CONTRATO-CATALOGO-20261006.md). Tres documentos publicados en Motor, recuperados por referencia inmutable y cotejados por bytes/SHA-256. La preparación local no acredita el recorrido MCP real o la inferencia científica. Un salto de línea añadido en la incorporación inicial fue rechazado por integridad; se restituyeron los originales antes de completar el cotejo. Sin cambios de preguntas o política.

Recibir el acoplamiento científico: recorrido MCP local y correspondencia con las páginas previstas, transporte con telemetría y separación de autoridad, y admisión efectiva de consumo antes de A01. Conservar el Árbitro existente y sus antecedentes; no reabrir su validación científica. Después catálogo y revisiones, anexo PDF y examen condicionado por API; navegación prohibida. Criptografía nativa pendiente y polígono únicamente completo y adjudicado.

Núcleo, semántica, IR, README y mapa histórico intactos. No hay campaña, anexo, examen, pagos, recargas ni otra inferencia por este asiento. Los resultados históricos permanecen conservados.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 40. Astra: recepción instrumental e inicio del catálogo A0 · 07/10/2026

Continuación autorizada del catálogo de GPT-6 Astra en S39 y TT-0021. Recepción previa Rust conforme: nueve recorridos MCP, dieciocho páginas completas y aislamiento seccomp cotejado; 33 pruebas locales conformes. Política idéntica al nodo 1, con exclusión de hechos externos; herramientas del candidato desactivadas. A01-A09/A0 preparados, sin inferencia del catálogo todavía.

Ejecutar secuencialmente A01-A09/A0 bajo control Rust; medir la entrega y adjudicar fuera del candidato. Detener ante incidencia instrumental, sin reintento automático. No abrir B, revisiones, anexo ni examen mediante el inicio de este bloque. Polígono sólo completo y adjudicado.

Se conserva el contrato y la obligación de fundamentar exclusivamente en las dos páginas. La comparación científica mantiene banco, política y criterios; las diferencias de infraestructura y observabilidad quedan declaradas. No se habilita navegación, herramientas, pagos o recarga. [Inventario y límites](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/anexos/TT-0021_ADMISION_CATALOGO_2026-10-07.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 41. Astra: adjudicación completa de A0, instrumentación y custodia · 07/10/2026

Astra A01–A09/A0 ejecutado y adjudicado: 9/9 correctos, incluidos seis críticos; κ Apto, puntuación 100/100 limitada a A0. Dos páginas completas por caso, sin herramientas del candidato ni premisas externas identificadas. Nueve solicitudes, 90,992 s con observación; 20.051 tokens; 340 muestras y 1.874 eventos SSE, sin fallos de captura. Presentación completa en egui comprobada.

Se conserva el criterio T(9)=7 y la exigencia de seis críticos correctos. El candidato recibió las dos páginas íntegras y la política original; no recibió clave o telemetría. La decisión sustantiva se contrastó fuera del candidato y la puntuación se aplicó en Rust. Conformidad limitada a la capa inicial, sin inferir aptitud clínica ni repetibilidad. La observación local no acredita control de los procesos internos de OpenAI.

[Informe científico](https://github.com/juantoniolloretegea/SV-motor/blob/a88ddfe7be8f5f8c8f535f0c46d61fd518e92786/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/catalogo-a0-20261007/INFORME.md). Custodia pública cotejada en Motor; custodia restringida y nueve informes de consumo en el archivo económico, revisión 8885d855eaab243a1265f45e7ffabf3ed54111b3. Importes no comunicados; cuota ordinaria sin modificar permisos de créditos o recarga.

Recibir el bloque A0 y continuar según el protocolo sin mezclar revisiones adversariales, B, anexo MCP/PDF y examen. No se acredita todavía estabilidad entre ejecuciones, aptitud clínica o recepción independiente. Conciliación económica individual y criptografía nativa pendientes.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 42. Corrección de geometría e interacción del visor Astra A0 · 07/10/2026

Corrección de la entrega gráfica Astra A0: el HTML anterior era una imagen estática de nueve botones, sin polígono ni interacción. Se sustituye por egui 0.2.0 compilado desde Rust a WebAssembly, incorporado con sus datos en un HTML autónomo. Cuatro pruebas Rust conformes y selección, pasajes y huellas comprobados en navegador por HTTP local. La apertura file:// no está verificada por la automatización. CAPA.json, adjudicación, puntuaciones y telemetría de inferencia intactos; cero nuevas llamadas al candidato.

Se rectifica la afirmación de presentación completa de S39 revisión 47, Acta 004 §41 y RETP-2026-286 en su alcance gráfico. Aquella inspección comprobó datos y captura, no geometría poligonal ni interacción del archivo. La revisión anterior permanece recuperable. No es una recalificación del modelo ni una auditoría científica independiente.

[Evidencias de corrección](https://github.com/juantoniolloretegea/SV-motor/blob/0aa9451017f522dd9e2e88422f15409d0d1f1597/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/catalogo-a0-20261007/CORRECCION-VISOR.md). Retorno al cierre de A0 y recepción del bloque conforme al protocolo. TT-0021 permanece pendiente en sus demás obligaciones; no se abren B, adversariales, anexo PDF ni examen.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 43. Recepción del suministro completo PDF en Rust · 07/10/2026

Recepción local del suministro PDF de Astra en Rust: diez páginas físicas, treinta fragmentos MCP completos y nueve solicitudes previstas, recompuestas y cotejadas sin envío. 52 pruebas distintas conformes. El auxiliar de admisión documental del Árbitro-Director exige identidad, integridad, aislamiento, diario e instrumentación antes de componer. Clave, telemetría y adjudicación fuera del contexto del candidato. Cero inferencias nuevas; A0 intacto.

La recepción acredita suministro textual, no comprensión del modelo, OCR, adjudicación ni admisión clínica. R2: 24 muestras, 68 registros, cero fallos y 275 ms de intervalo máximo. No acredita calibración integral ni mediciones de recursos internos del proveedor, hilos, asignaciones individuales de memoria Rust o recursos Linux por el PID del transporte WSL. Clave y originales operativos íntegros siguen locales; publicación técnica por proyección. Sin polígono de resultados PDF todavía.

[Recepción técnica](https://github.com/juantoniolloretegea/SV-motor/blob/d321b6ea7d3cf9333df1520eee885f9e22deabdb/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/anexo-pdf-20261007/INFORME-RECEPCION.md). Continúa TT-0021 pendiente. Dependencia específica: acoplar y recibir este suministro con el transporte de inferencia, cotejando cada solicitud efectiva antes del envío y conservando autoridad, instrumentación y límites de consumo. No se confunden anexo, adversariales y examen. Criptografía nativa y conciliación económica conservan su estado.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 44. Recepción del acoplamiento PDF06 con Astra · 07/10/2026

Acoplamiento del suministro PDF al transporte de GPT-6 Astra comprobado mediante una inferencia PDF06: páginas físicas 5 y 8 completas, seis fragmentos del MCP, solicitud recompuesta antes de envío. 23,582 s hasta entrega y cotejo inmediato; 5.228 tokens de entrada, 844 de salida, total 6.072; 848 eventos SSE y 101 muestras, cero fallos y 283 ms de intervalo máximo. HTTP 200 y response.completed. Admisión documental y de transporte en Rust, ajena al candidato. Tres citas cotejadas literalmente fuera de línea. A0 intacto.

El comprobador inicial exigió índices enteros de fragmento no prescritos en el banco. La entrega usó intervalos exactos; se conservaron rechazo y rectificación Rust, sin repetir inferencia. La integración general debe incorporar esa resolución antes de ampliar los casos. Esta recepción no adjudica los nueve casos, no prueba estabilidad, examen ni aptitud clínica. Herramientas del candidato deshabilitadas; no se certifica aislamiento interno del proveedor. Medidas locales por PID; límites instrumentales y criptografía nativa documentados. No se recibió resumen adicional del proveedor. Créditos e importe atribuibles desconocidos; SV-GASTO-20261007-012 con conciliación pendiente.

[Informe técnico](https://github.com/juantoniolloretegea/SV-motor/blob/237e3abad048280df3a4c7ddb5f8b4e74a6ac344/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/anexo-pdf-20261007/prueba-transporte/INFORME.md). TT-0021 permanece pendiente. Recibir la integración PDF06 y fijar la representación de localizadores en el comprobador general antes de ampliar el anexo. Conservar separación entre recepción instrumental, adjudicación científica, adversariales y examen. No formar polígono con casos sin evaluar.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 45. Admisión del banco PDF01–PDF09 · 07/10/2026

Admisión previa del banco PDF01–PDF09 bajo el mismo expediente S39 / TT-0021. Nueve contextos independientes, máximo 300 s por caso, sin reintentos, fuente y clave fijadas. Suministro nuevo de diez páginas mediante MCP Rust y comprobación conforme. Cuarenta pruebas Rust conformes. Comprobador 0.2.0 admite índices e intervalos exactos; tres citas de la entrega PDF06 anterior aceptadas sin inferencia adicional. Hito instrumental durable por pregunta antes de continuar.

La primera preparación fue detenida antes de lectura por denegación del servicio WSL en el entorno restringido; recibos conservados, cero llamadas al candidato. Acceso WSL comprobado con permisos de ejecución adecuados; recepción nueva r2 conforme. No altera fuentes ni resultados anteriores. El banco mantiene su terna propia, sin heredar pesos, criticidades ni puntuación A0. Adjudicación semántica separada; no aptitud clínica ni recepción independiente. Criptografía nativa y límites observacionales declarados.

[Admisión publicada](https://github.com/juantoniolloretegea/SV-motor/blob/6185ed4076ff2c2a0948fd0516fd67026dd60d43/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/anexo-pdf-20261007/banco-nueve-preguntas/ADMISION.md). Ejecutar una vez los nueve casos admitidos; detener ante fallo instrumental; cotejar entregas, registrar evaluación externa al candidato y consumos individuales 013–021 según ejecución efectiva. Incorporar hitos retrospectivos de A0 sin nuevas inferencias. TT-0021 pendiente.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 46. Banco PDF completo y conservación de hitos · 07/10/2026

Banco PDF01–PDF09 ejecutado una vez: nueve HTTP 200 y response.completed, sin reintentos. Vector completo [0,0,0,0,0,0,0,1,0]: ocho respuestas correctas frente al documento y un incumplimiento de formato y trazabilidad en PDF08. Duración del banco 264.710 ms; solicitudes 232.625 ms; 30.617 tokens de entrada, 7.508 de salida y 38.125 totales. Telemetría Rust: 986 muestras, cero fallos, máximo intervalo 345 ms. Hito instrumental contemporáneo por caso; medición y adjudicación posteriores separadas. Nueve hitos A0 reconstruidos retrospectivamente sin inferencia nueva. Polígono egui completo y comprobado.

PDF08 conserva una explicación concordante, pero entrega JSON inválido y no completa las evidencias; no se imputa una contradicción médica no demostrada. Cuatro rechazos iniciales por representación fueron rectificados fuera de línea con citas y localizadores exactos: se preservan ambos estados y originales. No se cambia la clave ni se corrige la respuesta. Anexo de fuente histórica, no aptitud clínica ni estabilidad general; comparación de estructura, no equivalencia de puntuaciones A0/PDF. Proyección pública identificada; originales extensos siguen locales. Criptografía nativa y cobertura no medida declaradas. Costes individuales desconocidos; nueve expedientes privados 013–021, sin convertir saldo agregado en coste cero.

[Informe técnico y límites](https://github.com/juantoniolloretegea/SV-motor/blob/deb363f165470b50fb1f5d011c0eefe5996d1a73/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/anexo-pdf-20261007/banco-nueve-preguntas/INFORME.md). Archivo económico privado, revisión 98a2139d5dc42f50cce75bffc33dedc7f210c9fa, recuperado y cotejado. Recibir el anexo completo y determinar el tratamiento del incumplimiento PDF08 antes de continuar las fases separadas. Conservar TT-0021 pendiente. No repetir inferencia automáticamente; adversariales y examen no realizados por esta actuación.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 47. Lectura SV, criticidad y abstención en el anexo PDF · 07/10/2026

Rectificación de lectura SV del anexo PDF sin nuevas inferencias: vector original (0,0,0,0,0,0,0,1,0), T(9)=7 y clasificación matemática κ=Apto, separados de la admisión no acreditada. Se incorpora regla eliminatoria: un solo 1 crítico determina No apto aunque κ sea Apto. Criticidades no fijadas antes del ensayo; errores críticos y puntuación no determinados. Pareja Frame_C=(frmat,frvis), identidad posicional, radios y colores del logo SV (0 rojo/r1, 1 verde/r2, U azul/r3). Doce pruebas Rust favorables, incluidos 19.683 estados, veto crítico y comprobación visual e interactiva del HTML 0.3.1.

La omisión de criticidades corresponde a la preparación, no al candidato; no se asignan retrospectivamente ni se transforma su ausencia en U. κ es una lectura posterior de la terna completa, no aprobación clínica ni modificación del contrato previo. PDF08 conserva el defecto formal y de trazabilidad original, sin imputar un error médico no demostrado. Adenda de abstención justificada preparada para el siguiente encargo: U con causa, evidencia y dato faltante; revisión exterior al candidato para impedir abstenciones injustificadas. No enviada al modelo. Un 0 por reconocimiento de límite documental exige símbolo de peligro y explicación de la capacidad no acreditada. PDF02 y PDF09 llevan avisos generales, particulares y junto a los vértices, con pasajes cotejados. Regla transversal para los tres nodos, todos los modelos y cualquier fuente: PDF, HTML en caché u otros documentos. Revisar diseño si se pidió un dato que el suministro no permite obtener; la autodeclaración del modelo no basta para 0 ni U. Un fallo instrumental queda fuera de la terna.

[Informe y fuentes](https://github.com/juantoniolloretegea/SV-motor/blob/9eaa42f9f9bbf3fc41f0a55947b1d04c92c0acaa/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/anexo-pdf-20261007/banco-nueve-preguntas/lectura-sv-v03/INFORME-RECTIFICACION.md). Archivo económico privado: expediente SV-GASTO-20261007-022, revisión 1a721ef91541e9be767864fc870cb123ce25401d; cero inferencias del candidato, asistencia e importe sin desglose atribuible. Recibir la rectificación y constituir competentemente criticidades y contrato antes de otra ejecución que aspire a admisión completa. Conservar TT-0021 pendiente. No repetir respuestas ni iniciar adversariales o examen mediante esta corrección.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 48. Admisión del banco PDF01–PDF09 · 07/10/2026

Admisión previa del ensayo PDF doble-1.0.0: nueve preguntas con respuesta provisional, autocrítica documental y verificación final neutral; máximo 27 solicitudes sin reintentos. Nueve requisitos críticos documentales fijados antes de la ejecución, veto de 1 crítico, U crítica impide admisión, alertas por límites. Fuente y preguntas originales, nuevo contrato estructurado y tres entregas conservadas por caso. Cuarenta y cuatro pruebas Rust conformes; suministro nuevo conforme, diez páginas completas, sin fallos de medición. Director coteja fuente, composición, historia y recibos antes del envío. Ninguna inferencia nueva realizada al registrar este asiento.

Contrato nuevo; no réplica causal idéntica ni modificación retrospectiva de resultados. Clave, calificaciones y telemetría fuera del candidato. Sin herramientas ni navegación; aislamiento interno del proveedor no observable. Forma, fidelidad y trazabilidad se adjudican separadamente; aptitud sólo para el contrato documental, sin certificación clínica. Criptografía nativa pendiente según excepción autorizada; magnitudes no medidas expresamente declaradas. Las revisiones universales no garantizan mejora.

[Admisión publicada](https://github.com/juantoniolloretegea/SV-motor/blob/dbed4c78fb7d873908cdbc66e32d10b4b82b1a59/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/revision-doble-20261007/ADMISION.md). Ejecutar una vez las 27 solicitudes como máximo, 300 s por solicitud y 90 min globales; detener ante incidencia técnica, registrar cada consumo efectivo y cotejar tres vectores sin escoger retrospectivamente la mejor respuesta. S39/TT-0021 continúa pendiente. Futura ampliación MCP excluida.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 49. Recepción PDF de tres etapas universales · 07/10/2026

Recibidas 27 generaciones Astra: R0 provisional, R1 autocrítica y R2 verificación final neutral para cada una de las nueve preguntas PDF. Sin reintentos ni selección retrospectiva. Tres vectores de nueve ceros; nueve requisitos críticos fijados previamente, sin errores críticos. Apto para el contrato documental de esta edición, recepción clínica independiente pendiente. PDF02 y PDF09 conservan advertencia por límite documental. PDF08 completo desde R0; el nuevo contrato no permite aislar el efecto causal de esquema JSON y revisiones.

164.122 tokens: 135.135 entrada y 28.987 salida; 26 de razonamiento incluidos, cero caché. Llamadas 941.258 ms; conjunto 1.069.854 ms. 3.761 muestras, cero fallos, intervalo máximo 475 ms. HTTP 200 y response.completed en 27 solicitudes; cotejo Rust de historia, fuente, SSE, texto, uso y telemetría conforme. 44 pruebas del controlador; seis del adjudicador y doce del visor. Polígono Rust/egui 0.4.0 comprobado en navegador, incluidos licencia, advertencias y pareja matemática. Veintisiete hitos y expedientes económicos 023–049.

Evaluación documental asistida por IA y exterior al candidato; no recepción clínica independiente ni garantía de reproducibilidad. Sin herramientas ni navegación entregadas al candidato; no inspección de infraestructura interna de OpenAI. Revisiones del mismo modelo, no adversarios independientes. Instrumentación limitada al inventario declarado. Criptografía nativa pendiente por excepción autorizada. Créditos e importes atribuibles desconocidos; asistencia sin desglose. Edición anterior y resultados intactos.

[Informe y evidencias](https://github.com/juantoniolloretegea/SV-motor/blob/1e1133f1b2ce8a287711d400e8a152c8a92f7896/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/revision-doble-20261007/INFORME.md). [Archivo económico privado](https://github.com/juantoniolloretegea/usos-gasto-creditos-tokens-sv/tree/7b2289e7fd4b1d8aee417f183b3f0ad7b9435c76). Recepción del ensayo concluido y revisión competente independiente; no iniciar automáticamente otro banco, examen ni ampliación MCP. Tique TT-0021 permanece pendiente por el alcance restante. Conservar originales, hitos y consumos sin duplicarlos.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 50. Admisión del banco P01–P25 · 07/10/2026

Admisión del examen Astra de 25 preguntas, edición 1: banco y clave históricos inmutables, caché NCI-PDQ de 2024 recuperada en 2026. Veinte críticas y cinco no críticas; T=19, veto de cualquier 1 crítico y U crítica impide admisión. Respuesta provisional, autocrítica y verificación neutral universales: máximo 75 solicitudes, 300 s por entrega y 90 min globales, sin reintentos. Suministro MCP Rust de cinco secciones completas, 25 fragmentos, diario conforme, sin fallos de medición; 44 pruebas Rust superadas. Ninguna inferencia de esta edición al registrar la admisión. TT-0016 gobierna el examen; TT-0021 conserva la vinculación del transporte.

Fuente exclusiva, razonamiento documental permitido sin exigir conclusión literal. Clave y adjudicación fuera del candidato; sin herramientas ni navegación. Aislamiento MCP comprobado, infraestructura de OpenAI no inspeccionable. Edición con secciones completas, JSON estricto y dos revisiones, técnicamente distinta del nodo 1; no confundir comparabilidad de preguntas con equivalencia instrumental ni admisión clínica. Costes y créditos desconocidos no se igualan a cero. Excepción criptográfica experimental conservada.

[Admisión publicada](https://github.com/juantoniolloretegea/SV-motor/blob/c1b7a041ae024e01fa009665b2ad201fa3070c84/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/examen25-20261007/ADMISION.md). Ejecutar una vez P01–P25/R0–R2 bajo control Rust, preservar originales e hitos, registrar consumo individual privado y corregir cada fase desde la clave externa. Sin selección retrospectiva, con alertas verificadas y polígono egui sólo con 25 posiciones válidas. Recepción humana posterior. No modifica resultados históricos ni inicia otras pruebas.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 51. Recepción del examen Astra P01–P25 · 07/10/2026

Examen Astra P01–P25 concluido: 75 entregas completas en tres fases universales; R2 final=(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), dictamen Apto para el contrato documental, errores críticos 0. Veinte preguntas críticas y cinco no críticas, T(25)=19. Cotejos Rust de suministro, historia, eventos, texto, tokens y telemetría. Se conserva además un intento interrumpido por rechazo local de keepalive, sin respuesta final ni uso conocido; corrección técnica r2 acreditada con 47 pruebas y fuentes idénticas. P15/R0 interrumpida por OpenAI con server_is_overloaded; la adenda r3 registra aplazamiento al final, umbral operativo de cinco minutos y 54 pruebas Rust. Las catorce preguntas completas no se repiten. La pausa de intervención local no se atribuye a una caída del proveedor. Registro SV-SERVICIO-OPENAI-20261007-001 conservado en servicios-proveedores-ia/openai, con causa literal, duraciones conocidas y recuperación de P15; valoración del proveedor pendiente. Tokens conocidos 842698, sin añadir consumo desconocido. Hitos individuales y representación egui completa; clave fuera del candidato.

Admisión limitada al contrato documental NCI-PDQ de actualización 14/11/2024. No certificación clínica ni recepción médica independiente. Revisión sustantiva asistida por IA, exterior al candidato. Sin herramientas ni navegación habilitadas; no se inspecciona la infraestructura interna del proveedor. Preguntas y criterio históricos conservados, con condiciones instrumentales distintas del nodo 1. Criptografía nativa pendiente por excepción autorizada; créditos e importes por solicitud desconocidos, no cero. Incidencias de las explicaciones de revisión conservadas en el informe.

[Informe y evidencias](https://github.com/juantoniolloretegea/SV-motor/blob/a8cd2319c004f76a1159cb382f7ed93b2d648bf3/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/examen25-20261007/resultado/INFORME.md). Recibir el resultado y su revisión competente; conservar originales y archivo económico privado. No iniciar otra campaña, revisión adicional, examen ni ampliación del MCP sin nueva instrucción. Valorar calidad del servicio más adelante con criterios definidos y referencias principales ISO/IEC 42001:2023 e ISO 9001:2026; ISO/IEC 20000-1:2018 y 25010:2023 son complementarias, sin declarar ahora conformidad ni puntuación.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

<a id="cierre-documental-astra-nodo03"></a>

## 52. Cierre documental de GPT-6 Astra · Nodo 03 · 07/10/2026

Fecha UTC: 2026-10-07T19:27:27Z. S39 revisión 58; TT-0016 y TT-0021; Acta 004 §52; RETP-2026-297.

**Dictamen de esta prueba: Apto para el contrato documental.** Cierre documental del examen GPT-6 Astra, nodo 03, autorizado el 07/10/2026: Apto para el contrato documental P01–P25. Las 25 respuestas finales R2 son correctas, incluidas las 20 críticas; cero errores y cero U; T(25)=19. Se conservan 75 entregas completas y dos intentos interrumpidos. Portada general, catálogo, ficha e índice propio conciliados con el informe; inferencia ejecutada por OpenAI, control propio del SV en Rust. No se repite la prueba ni se modifican sus resultados.

El nodo 01 ejecuta motor y pesos en infraestructura administrada por el proyecto. Astra corresponde al nodo 03: OpenAI realiza la inferencia; el Árbitro-Director y sus componentes Rust gobiernan el suministro, orden, recepción, adjudicación y mediciones propios. El candidato no accede a la clave ni gobierna la instrumentación. Se mantuvieron las fuentes documentales acotadas y no se habilitaron herramientas de navegación al modelo; no se inspecciona la infraestructura interna del proveedor.

R0, R1 y R2 se aplicaron universalmente: respuesta provisional, autocrítica y verificación final neutral. R2 es la respuesta final de cada pregunta; no se selecciona retrospectivamente la mejor fase. Los veinte críticos en 0 y el umbral T(25)=⌊7×25/9⌋=19 sustentan el dictamen. La puntuación ponderada no está constituida para este examen. Los originales, las incidencias y los avisos documentales permanecen inalterados.

El cierre corresponde a esta prueba documental. No acredita aptitud clínica, recepción médica independiente, estabilidad estadística ni identidad instrumental con el nodo 01. Se conservan las observaciones y avisos del informe. Dependencia criptográfica nativa, evaluación del servicio del proveedor e integración general pendientes por separado. S39 general permanece en ejecución y TT-0021 pendiente; el cierre administrativo histórico de Instruct en TT-0016 conserva su resultado propio.

[Expediente consolidado](https://github.com/juantoniolloretegea/SV-motor/blob/365c0004164a5e7c7dd7e5ecb230ebba7f7a1582/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/readme.md) · [Informe y evidencias](https://github.com/juantoniolloretegea/SV-motor/blob/365c0004164a5e7c7dd7e5ecb230ebba7f7a1582/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/examen25-20261007/resultado/INFORME.md) · [Dictamen estructurado](https://github.com/juantoniolloretegea/SV-motor/blob/365c0004164a5e7c7dd7e5ecb230ebba7f7a1582/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/examen25-20261007/resultado/visor/src/DICTAMEN.json).

**Retorno:** Prueba documental de Astra cerrada como Apto. Conservar sus evidencias y el archivo económico privado; atender la revisión independiente cuando se disponga. Mantener separados los pendientes de criptografía, integración general, conciliación económica y calidad del servicio. Una nueva campaña requiere su encargo y autorización propios.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 53. Libros Markdown mediante MCP · comprobación instrumental del 07/10/2026

Fecha UTC: 2026-10-07T21:53:40Z. S39 revisión 59; TT-0014; Acta 004 §53; RETP-2026-298.

Prototipo Markdown 0.1.0 y servicio Rust MCP 0.1.5-mdbook.1: 36 comprobaciones conformes; cuatro documentos, 31 secciones y 31 entregas reconstruidas exactamente. Diario de 110 sucesos y 36 mediciones del proceso. Edición humana mdBook comprobada con documentación pública del manual; sin modificar sus once estados pendientes.

El preparador fija índice, archivos y huellas; rechaza alteraciones, rutas ajenas, enlaces simbólicos, inclusiones no resueltas, HTML incrustado e imágenes en esta primera edición. Conserva exactamente el texto, las líneas y las posiciones Unicode. El diario permite cotejar solicitudes, entregas y medidas. La búsqueda literal no decide suficiencia; la instrucción documental admite razonamiento fundado y U justificada, bajo revisión exterior al candidato.

Ensayo instrumental sin candidato, Árbitro completo ni transporte Astra. No constituye recepción integral MCP ni validación científica. Las decisiones de suministro continúan reservadas al Árbitro. No cambia resultados anteriores, núcleo, gramática, IR ni sedes del manual. Sin navegación ni ejecución de ejemplos en el servicio probado; límites y magnitudes no medidas expresamente declarados.

[Fuentes, contrato y límites](https://github.com/juantoniolloretegea/SV-motor/blob/b4f6fed400e8d9e6c7bccaa8023ca40a8de83d74/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/model-context-protocol/mdbook/0.1.0/LEAME.md) · [Recorrido medido](https://github.com/juantoniolloretegea/SV-motor/blob/b4f6fed400e8d9e6c7bccaa8023ca40a8de83d74/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/model-context-protocol/mdbook/0.1.0/evidencias/recorrido/COMPROBACION.json) · [Manifestación de identidad](https://github.com/juantoniolloretegea/SV-motor/blob/b4f6fed400e8d9e6c7bccaa8023ca40a8de83d74/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/model-context-protocol/mdbook/0.1.0/MANIFIESTO.json).

La conservación pública fue recuperada y cotejada en Rust: 36 archivos idénticos por bytes y SHA-256, revisión b4f6fed400e8d9e6c7bccaa8023ca40a8de83d74. Registro administrativo específico en el archivo privado de usos; ninguna inferencia nueva ni coste desconocido convertido en cero. No se amplía la recepción del candidato por esta comprobación instrumental.

**Dependencia y retorno:** Recibir el adaptador y preparar su incorporación al recorrido del Árbitro y al transporte elegido antes de una prueba de candidato con autorización propia. Conservar intactos los cierres documentales de Astra y los demás pendientes generales.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 54. Admisión del ensayo del manual mediante MCP · 08/10/2026

Fecha UTC: 2026-10-08T03:40:27Z. S39 r60; TT-0014 / TT-0021; Acta 004 §54; RETP-2026-299.

Admisión MD01–MD09 para GPT-6 Astra, nodo 03: nueve preguntas sobre constructor del manual y compatibilidad interlenguajes; seis críticas fijadas previamente, T(9)=7, veto de error crítico y U crítica sin admisión. Tres etapas universales R0/R1/R2. Recorrido Rust adaptado: 53 pruebas favorables; cuatro documentos y 31 secciones suministrados íntegramente por MCP, diario y bytes cotejados, sonda de sockets rechazada.

Corpus preentregado por el Árbitro: no acredita búsqueda autónoma del candidato. Clave y control separados. Sin herramientas de navegación del candidato; aislamiento MCP no equivale al de OpenAI. Criptografía nativa pendiente. Resultados anteriores intactos; sin recepción médica ni manual terminado.

La conexión se ha preparado bajo la autorización existente. En este asiento no se acredita ninguna inferencia: el acceso interactivo presenta una verificación de seguridad que impide completar la sesión. No hay respuesta, tokens de candidato ni calificación del modelo. No se elude la verificación ni se modifica la protección del navegador. Se distingue este impedimento de acceso de una interrupción de una pregunta ya enviada.

[Admisión](https://github.com/juantoniolloretegea/SV-motor/blob/b45d2e2830e312fc962d134d52ddd988c15b9d59/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/ADMISION.md) · [Banco](https://github.com/juantoniolloretegea/SV-motor/blob/b45d2e2830e312fc962d134d52ddd988c15b9d59/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/fuentes/BANCO.json) · [Suministro recibido](https://github.com/juantoniolloretegea/SV-motor/blob/b45d2e2830e312fc962d134d52ddd988c15b9d59/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/suministro/CONTROL-ARBITRO.json) · [Identidad previa](https://github.com/juantoniolloretegea/SV-motor/blob/b45d2e2830e312fc962d134d52ddd988c15b9d59/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/PREVIA.json). Diecisiete archivos recuperados y cotejados en Rust por bytes y SHA-256, revisión b45d2e2830e312fc962d134d52ddd988c15b9d59.

**Dependencia y retorno:** Completar el acceso ya autorizado y ejecutar el banco bajo límites fijados; cotejar cada entrega, adjudicar R2 externamente y producir polígono egui sólo con nueve posiciones completas. Registrar todos los intentos en el archivo administrativo privado.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 55. Cierre documental del ensayo del manual Markdown · 08/10/2026

Fecha UTC: 2026-10-08T04:09:42Z. S39 r61; TT-0014 y TT-0021; Acta 004 §55; RETP-2026-300.

Ensayo MD01–MD09 concluido: 27 entregas GPT-6 Astra en tres etapas; cuatro documentos y 31 secciones íntegros por MCP. R0 y R1: nueve correctas; R2: ocho correctas y error crítico MD01. Vector final (1,0,0,0,0,0,0,0,0), T(9)=7 y κ=Apto, pero admisión No apto por veto crítico. Respuesta principal del manual correcta; afirmación inexacta sobre etapas previas en la revisión final.

El acceso pendiente descrito en la admisión se resolvió antes de las llamadas. Las 27 terminaron con HTTP 200 y response.completed, sin reintentos ni interrupciones del servicio durante las preguntas. Recepción formal y cotejo Rust conformes; no equivalen a corrección semántica. La inferencia duró 779.679 ms de banco, aproximadamente 12 min 59,7 s. Se conservaron 2.826 muestras del cliente y 23.853 eventos SSE; máximo intervalo 392 ms. Uso recibido: 516.548 tokens de entrada, 23.988 de salida y 540.536 totales. Créditos e importe atribuibles desconocidos, pendientes de conciliación.

El candidato recibió el corpus completo previamente suministrado por el Árbitro, sin navegación autónoma MCP ni herramientas habilitadas. El aislamiento de sockets corresponde al proceso MCP y no a infraestructura interna de OpenAI. Las dos revisiones universales son del propio candidato. El error introducido en R2 demuestra que no garantizan mejora en todas las ejecuciones.

Polígono Rust/egui 0.6.0 completo y comprobado: nueve posiciones, seis criticidades, pareja matemática/visual, umbral y veto; convención 0 rojo/r1, 1 verde/r2, U azul/r3. Advertencias sustentadas en límites documentales, sin penalizar razonamiento válido. Interacción y desplegable de fuente/licencia comprobados en el navegador integrado. Comprobaciones finales: 53 controlador, 13 recepción, 4 instrumentación y 10 visor; módulos parcialmente compartidos, sin sumar cobertura independiente.

Revisión exterior al candidato asistida por IA; recepción científica independiente pendiente. No atribuye causa interna al proveedor: posible ambigüedad temporal del encargo conservada como limitación. No acredita implementación del manual ni aptitud clínica. Criptografía nativa pendiente. Resultados de anteriores contratos intactos.

[Informe](https://github.com/juantoniolloretegea/SV-motor/blob/5f989cb592bbbc7da6568c156b2f8c5da7adf689/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/INFORME.md) · [Dictamen](https://github.com/juantoniolloretegea/SV-motor/blob/5f989cb592bbbc7da6568c156b2f8c5da7adf689/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/visor/src/DICTAMEN.json) · [Comparación](https://github.com/juantoniolloretegea/SV-motor/blob/5f989cb592bbbc7da6568c156b2f8c5da7adf689/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/COMPARACION-RUST.json) · [Métricas](https://github.com/juantoniolloretegea/SV-motor/blob/5f989cb592bbbc7da6568c156b2f8c5da7adf689/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/METRICAS-RUST.json) · [Polígono](https://github.com/juantoniolloretegea/SV-motor/blob/5f989cb592bbbc7da6568c156b2f8c5da7adf689/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/visor/POLIGONO-EGUI.html). Resultados y fuentes conservados en Motor, revisión 5f989cb592bbbc7da6568c156b2f8c5da7adf689. Archivo administrativo privado, revisión 7bd30fa322bd30846132626d29b1fa1529a5fbfb: 27 expedientes por entrega y uno de preparación; originales operativos con manifiesto. Las lecturas de cuenta compartida no se imputan al banco.

**Dependencia y retorno:** Conservar este cierre y sus originales; revisión competente independiente y conciliación económica pendientes. Mejoras del contrato requieren versión propia y otra inferencia requiere autorización. Sin repetición automática.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 56. Reserva metodológica del dictamen MD01 · 08/10/2026

Fecha UTC: 2026-10-08T04:25:26Z. S39 r62; TT-0014/TT-0021; Acta 004 §56; RETP-2026-301.

Revisión metodológica MD01: cotejo Rust de 27 entregas confirma corpus y respuestas históricas íntegros, etapas correctas y sustitución del encargo R1 en los nueve contextos R2, sin conservar sus instrucciones históricas completas. Contenido crítico de MD01 correcto; frase retrospectiva inexacta. No apto histórico bajo reserva metodológica, sin convertirlo en Apto ni alterar valores.

La composición del contexto y la delimitación temporal son insuficientes para una atribución exclusiva al modelo. El veto Rust funciona conforme a su configuración; el alcance sustantivo de la criticidad requiere revisión. No se demuestra causalidad exclusiva ni aptitud general. Revisión exterior asistida por IA, recepción independiente pendiente.

Una prueba local del comprobador Rust detecta alteración, reordenación y falta de antecedentes. Su ejecución coteja archivos fijados, solicitudes y respuestas frente a la recepción conservada. No reinterpreta SSE ni decide automáticamente la semántica. La conclusión anterior sobre incapacidad crítica era demasiado categórica. La observación de inexactitud permanece y las reglas de veto crítico no se suavizan.

Sin inferencia nueva ni tokens nuevos del candidato; asistencia no imputada como cero ni a Astra. No se modifican respuestas, clave, banco, adjudicación, polígonos ni resultados de otros ensayos. La nueva nota en el informe remite a esta reserva; la versión histórica sigue recuperable. El futuro banco no debe combinar una repetición con resultados anteriores como si procedieran de un único contrato.

[Adenda y cotejo Rust](https://github.com/juantoniolloretegea/SV-motor/blob/17059fb9b57f825d5b14bfa13e1a370a97b3a6ef/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/revision-metodologica-20261008/REVISION-METODOLOGICA.md). Decidir réplica diagnóstica sólo de MD01 con R0/R1/R2 y contexto corregido, prefijado y comprobado. Anexo experimental y otro banco son propuestas separadas. No ejecutar inferencias por esta adenda.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 57. Réplica diagnóstica MD01 recibida · 08/10/2026

Fecha UTC: 2026-10-08T04:48:53Z. S39 r63; TT-0014/TT-0021; Acta 004 §57; RETP-2026-302.

Réplica diagnóstica autorizada sólo de MD01, contrato 1.1.0: tres etapas R0/R1/R2 completas y conformes, valores 0/0/0 para una misma pregunta. Contexto histórico íntegro y alcance temporal explícito; no reaparece la imputación incorrecta de identificadores. Contenido, estructura y revisión recibidos por separado.

La pregunta sigue siendo la función del constructor y su diferencia respecto del manual final. Se conservan cuatro documentos, 31 secciones completas, parámetros y tres etapas universales. La modificación prefijada conserva los encargos e instrucciones históricos íntegros, las respuestas sin alteración y la validez temporal de cada identificador. No se informa al candidato del fallo anterior ni se le entrega la clave. La fuente es suficiente: su límite documental se advierte sin exigir contenido ausente ni penalizar razonamiento fundado.

Recepción propia Rust: tres HTTP 200 y response.completed, sin reintentos ni interrupciones; originales, solicitudes, SSE, citas, uso y fuentes fijadas cotejados. R2 reconoce válidos los identificadores históricos 0 y 1. Revisión sustantiva exterior al candidato asistida por IA, identificada y conservada. Conformidad exige simultáneamente contenido, estructura y revisión; no se suaviza el veto crítico.

Duración del banco: 126.191 ms. Uso declarado: 61.110 tokens de entrada y 4.435 de salida, 65.545 total. Instrumentación cliente: 452 muestras, máximo intervalo 289 ms, cero fallos; preparación MCP: 21 muestras, máximo 287 ms. Herramientas del candidato deshabilitadas y navegación prohibida por contrato; aislamiento de sockets acreditado sólo en el proceso MCP Linux. Criptografía nativa pendiente. Comprobaciones: controlador 55, receptor con dictamen 15, métricas 3; conjuntos parcialmente compartidos.

No sustituye el vector del ensayo de nueve preguntas, cuyo dictamen histórico continúa bajo reserva metodológica. No acredita causa exclusiva, estabilidad estadística, recepción científica independiente ni aptitud clínica. Criticidad y veto de error conservados.

La presentación es una tabla de etapas y respuestas originales, generada por Rust en HTML estático, sin JavaScript. No se crea un polígono de nueve posiciones para una pregunta ni se incorpora el resultado a una ejecución anterior. Las respuestas originales y adjudicaciones anteriores se preservan.

Custodia científica: Motor/main 0246a7df1e9436f4bff4f0d94ac6d36e1f29c1e0, 43 archivos recuperados y cotejados por bytes y SHA-256. Archivo administrativo privado: tres expedientes por entrega y uno de preparación; créditos, impuestos, importe y asistencia atribuibles pendientes de conciliación, nunca sustituidos por cero. Sin pagos, recargas, nuevas ramas ni cambios de README.

[Informe, dictamen, mediciones y código](https://github.com/juantoniolloretegea/SV-motor/blob/0246a7df1e9436f4bff4f0d94ac6d36e1f29c1e0/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/replica-md01-20261008/INFORME.md). Recibir competentemente el diagnóstico y conciliar el consumo monetario cuando exista información atribuible. Ninguna pregunta, banco o inferencia adicional queda iniciada.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 58. Grok 4.7 y cliente común Rust · 08/10/2026

Fecha UTC de incorporación: 2026-10-08T07:14:07.5330919+00:00. S39 r64; TT-0014 y nuevo TT-0022; Acta 004 §58; RETP-2026-303. Estado: Ensayo concluido; recepción independiente pendiente.

Examen completo: 75 entregas, 25 preguntas y tres etapas. R0: 24 correctas y P02 crítica incorrecta; No apto. R1 y R2: 25 correctas y ningún error crítico; Apto para el contrato documental. Duración del banco 3.814.378 ms; 1.273.687 tokens científicos. Instrumentación: 14.007 muestras, intervalo máximo 446 ms y cero fallos. Retención cero confirmada. Error inicial corregido espontáneamente y conservado; revisión exterior asistida por IA. Polígono egui completo e interactivo comprobado. Diecisiete pruebas del cliente, cuatro de adjudicación y doce del visor conformes.

Modalidad: nodo 03, API directa xAI. El Árbitro, el suministro completo por MCP, los localizadores, la instrumentación y los criterios conservan su autoridad en el SV. El candidato recibe únicamente pregunta, secciones y antecedentes de esa pregunta. Banco EVAL-PDQ-HCL-25-20260929/r1, veinticinco preguntas, veinte críticas, T(25)=19 y veto de cualquier error crítico. R0, R1 y R2 universales; los registros de instrucciones históricas completas preservan el contexto temporal comprobado en MD01. La clave reservada no se envía.

El cliente se conserva en una sede común con configuración por proveedor y modelo. Reutiliza componentes recibidos en Astra, sin modificar su historia. Diferencia xAI comprobada: tools=[] sin tool_choice; la primera comprobación fue rechazada antes de obtener respuesta, la segunda recibió el objeto sintético exacto. Retención cero activada por autorización expresa y confirmada mediante cabecera. Licencia propia en cada envío, sin confundirla con una barrera técnica ni una cesión de derechos. No se habilitan compras ni recargas.

Los originales, estados, mediciones, límites y costes pertenecen a sus respectivas sedes. El archivo económico privado registra los intentos rechazados y los costes desconocidos como pendientes; la compra de crédito y su consumo no se duplican como gasto. La preparación se fijó antes de cada ejecución. La presente incorporación documental no altera las fechas de los originales ni presenta el tique como anterior a la ejecución.

El control de salida del SV y los contadores del proveedor no son inspección de su infraestructura. La admisión documental no equivale a aptitud clínica ni estabilidad estadística. La generalidad del cliente exige recibir cada perfil nuevo; no se presupone compatibilidad por cambiar el nombre del modelo.

[Expediente, fuentes y recepción](https://github.com/juantoniolloretegea/SV-motor/blob/e238915ae755bfb5e58738c7430e060f39b08baf/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/xai/grok-4.7/examen25-20261008/INFORME.md). Conservar originales y registro por intento, recibir competentemente el resultado y conciliar los importes que permanezcan desconocidos. Ninguna ampliación económica se deduce del ensayo.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 59. Grok MD01 diagnóstica · 08/10/2026

S39 r65; TT-0014/TT-0022; Acta 004 §59; RETP-2026-304. Estado: diagnóstico parcial y reserva económica pendiente de corrección.

MD01 diagnóstica con Grok 4.7: R0 y R1 recibidas y correctas para el núcleo documental preguntado; R2 no enviada por reserva insuficiente. Diagnóstico parcial, sin conformidad del conjunto. 66.789 tokens, 243.560 ms, 908 muestras del cliente, intervalo máximo 367 ms y cero fallos. Se identifica una insuficiencia del supuesto económico: max_output_tokens de xAI no limita los tokens de razonamiento.

La reserva previa no constituye una garantía de coste máximo total y debe corregirse antes de cualquier nueva campaña. El coste efectivo sí quedó dentro de la autorización. No se atribuye incumplimiento al proveedor por aplicar el límite sólo a la salida visible. La revisión científica independiente permanece pendiente; no se altera ningún examen anterior ni se acredita aptitud clínica.

Cuatro documentos y 31 secciones completos obtenidos de nuevo por MCP Rust, contexto temporal cotejado y clave fuera del candidato. HTTP 200 y retención cero confirmados; cero herramientas y fuentes externas comunicadas. No navegación habilitada ni inspección de servidores xAI. 25 muestras previas del MCP; pruebas Rust 7+14+1 favorables. La parada se produjo antes del envío R2 por control local, no por falta de servicio del proveedor. Las dos respuestas conservan el núcleo correcto; R1 precisa denominaciones sin alterar la distinción entre arquitectura preparatoria y manual final. No se construye un polígono con etapas ni se reevalúan otros bancos.

[Expediente y pruebas](https://github.com/juantoniolloretegea/SV-motor/blob/091b50a884c893fd4322bbf211b0feb0132bb258/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/xai/grok-4.7/md01-diagnostica-20261008/INFORME.md). [Semántica del límite de xAI](https://docs.x.ai/developers/rest-api-reference/inference/responses). Archivo privado individual actualizado. Conservar los originales; recibir la revisión competente y resolver el control económico antes de proponer otra ejecución. No hay continuación automática ni ampliación de gasto.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## 60. Qwen3.8-Max-0902 · Apertura experimental · 08/10/2026

S39 r66; TT-0014/TT-0023; Acta 004 §60; RETP-2026-305.

Qwen3.8-Max-0902 incorporado experimentalmente al nodo 03 mediante API directa de Alibaba Cloud Singapore y cliente común Rust. Acceso técnico recibido: HTTP 200, respuesta JSON exacta, 3827 ms y telemetría sin fallos. Examen histórico de 25 preguntas iniciado, tres etapas universales; sin dictamen todavía.

Sólo cuota gratuita con Stop-on-Exhaust activado; presupuesto pagado cero. Retención ordinaria admitida para corpus documental sin pacientes; store=false no acredita ZDR. Esquema exigido en instrucciones y validado en Rust, sin garantía remota presumida. Sin herramientas de navegación ni fuente externa autorizada. No se inspecciona la infraestructura del proveedor. La criptografía nativa conserva su excepción experimental.

MCP con cinco secciones íntegras y aislamiento de sockets comprobado. La clave de corrección y la instrumentación no entran en el contexto del candidato. El intento inicial de preparación quedó interrumpido por restricción local de acceso a WSL antes del suministro, sin inferencia; se conservó y se completó la preparación con acceso al proceso autorizado. No es fallo del modelo ni del proveedor.

[Admisión y componentes](https://github.com/juantoniolloretegea/SV-motor/blob/38b5a46cf8dac6ca0303da81417504c6984acee2/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/qwen/qwen3.8-max-0902/examen25-20261008/ADMISION.md).

Retorno: Completar el banco dentro de la cuota y plazos, recibir originales y telemetría en Rust, realizar revisión sustantiva exterior y archivar cada intento. Polígono egui únicamente con conjunto completo adjudicado. Si faltan recursos, conservar resultado parcial sin atribuir fallo al candidato.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
