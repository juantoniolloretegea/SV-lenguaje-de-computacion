# Acta 004 · Finalidad, alcance y continuidad del ensayo de inteligencia artificial y observabilidad

**Fecha:** 22 de septiembre de 2026.  
**Seguimiento canónico:** S39, revisión 6; RETP-2026-268.  
**Naturaleza:** síntesis científica y técnica, actualización del estado documental y delimitación de la continuación.  
**Estado:** investigación lateral en seguimiento; instalación nativa documentada; comprobación material independiente pendiente.  
**Corte del Lenguaje examinado:** `9b2e5ef0aa1ea010a7cc79a2a67df7017bd99bc7`, rama `main`.

**Actualización posterior de estado:** §10, vinculada a S39 revisión 10. Los §§1–9 conservan el corte de recepción original.

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
