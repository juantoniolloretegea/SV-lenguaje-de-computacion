# S26 R06 · PROCESO03 · Resultados del observador de proceso

**Catorce casos conformes en debug y catorce en release: 28 concordancias con los oráculos previos.** Se recibe el observador de proceso en el alcance local de T03/T04/T08. R06 permanece abierto por T07 y la integración del recorrido completo; cero casos globales S26 cerrados.

## Procedencia y orden

El [banco](README.md) quedó publicado antes de ejecutar en Lenguaje [2038f55685f8d6a406b837012a0731afcadc597e](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/2038f55685f8d6a406b837012a0731afcadc597e) y laboratorio `223bc1dedd847b93664c64018c75d7ae95699a44`. Se cotejaron los árboles completos de ambas sedes y sus referencias; no se crearon ramas. El registro conserva Pilares, perfiles/contratos/ensamblaje y transición secuencial como piezas rectoras leídas, junto con R06 y sus sedes materiales enlazadas.

Después de las campañas, las siete piezas del precompromiso y las 99 fuentes reutilizadas coinciden con sus blobs fijados. El núcleo, la admisión, el certificado y el conjunto I0205-01 no se modificaron. Los [antecedentes PROCESO01](../proceso01/RESULTADOS.md) y [PROCESO02](../proceso02/RESULTADOS.md) conservan, respectivamente, el impedimento instrumental y la campaña incompleta. Sus resultados no se suman a las 28 concordancias.

Rust/Cargo 1.98.0, LLVM 22.1.8, Linux 6.18.44 x86-64; compilaciones `--offline --locked`. Ambos perfiles usan `panic=unwind`; release desactiva `debug_assertions`. Los abortos de proceso son llamadas reales a `std::process::abort`, independientes de esa configuración de pánico. Las compilaciones terminaron con código 0 y conservan las 25 advertencias existentes de `sv_core`. Ambas campañas completas terminaron con código 0. [Comandos, configuración e identidades](EJECUCIONES.json).

## Evidencia y límites por grupo

| Casos | Resultado en ambos perfiles | Alcance demostrado |
| --- | --- | --- |
| P01 | Captura y recibo concordantes, EOF, salida 0 | Control positivo del recorrido documental, con geometría original de 6237 bytes reconstruida por concatenación exacta de fragmentos. |
| P02 | SIGABRT (6), barrera anterior a captura e informe ausente | El padre conserva la apertura y observa la terminación real del hijo. |
| P03 | SIGABRT (6) tras captura y escritura | Captura completa y escritura anterior conservadas; la nueva lectura del padre recupera el efecto local. No hubo reversión de esa escritura. |
| P04 | SIGKILL (9) desde el padre en barrera | Terminación real observada; captura y escritura anteriores se conservan, sin informe final. |
| P05 | Duplicado idéntico identificado | Primer informe preservado; no se duplica una ejecución de efecto. |
| P06 | Segundo informe contradictorio conservado | Conflicto explícito, primer informe intacto y pérdida de concordancia de la recepción seleccionada. |
| P07 | A tardío separado; B recibido con su correlación | La selección de B no reatribuye el informe de A. No acredita dos adquisiciones físicas distintas. |
| P08 | Intento AJENO conservado como tal | No produce un recibo atribuido a las aperturas A o B. |
| P09 | EOF con terminación aún no observada | El hijo sigue esperando control; la liberación y salida posteriores quedan en otra observación. |
| P10 | Plazo agotado tras barrera recibida | No se infiere terminación del hijo. La terminación administrativa posterior y SIGKILL se registran aparte, sin reintento. |
| P11 | Cabecera de 8193 bytes rechazada | Cuota del cuerpo aplicada antes de admitirlo; el hijo aún no ha terminado en la frontera observada. |
| P12 | Captura completa, informe truncado | Se conserva la captura; EOF no se transforma en informe completo ni en terminación de proceso. |
| P13 | Informe válido y salida 7 coexistentes | Resultado documental y código de salida permanecen separados. |
| P14 | Magia inválida rechazada | No hay informe aceptado; la terminación posterior no modifica retrospectivamente la recepción. |

Se observaron por perfil dos SIGABRT y dos SIGKILL, este último par compuesto por la terminación en barrera y la limpieza tras plazo. No se recuentan las operaciones de limpieza como efectos SV. `frontera.json` conserva el estado anterior a la liberación o terminación administrativa, y `observacion.json` conserva la salida final por separado.

## Conservación de la evidencia

[EVIDENCIA.tar.gz](EVIDENCIA.tar.gz): 134127 bytes; SHA-256:

`28f0b4b159928116d689957ae7b776a893cac878751a8a4d6e784d6a9988f85d`.

Incluye las dos campañas completas: apertura de cada caso, canales literales, fragmentos y capturas, instantáneas de frontera, observaciones finales, lecturas del destino local, resultados globales, registros de compilación y de ejecución y configuración. Las fuentes permanecen en el precompromiso. La documentación del perfil describe los límites de captura y conservación; no se reconstruyen piezas ausentes de los intentos anteriores.

SHA-256 de los ejecutables:

- Banco debug: `23edc05bf0363beb3d9b45997e14d62c1e6c0f706002fd99d2de581b9b0f57da`.
- Banco release: `87f04ab08b93a2c29aac1e23587a7e9b96660e73e867957b1935636a86f17ea5`.

## Frontera de confianza y alcance de RAM

El padre custodia las aperturas, fragmentos, capturas e informes en su RAM. La terminación del hijo no destruye esos objetos del padre. El hijo consume la copia del descriptor admitido; la certificación y las comparaciones conservan sus límites documentales. Las tuberías transportan copias entre procesos; el banco no altera memoria compartida del núcleo.

La única transferencia `unsafe` del montaje toma una vez la propiedad de los descriptores de las tuberías heredadas, bajo la precondición documentada. No aporta aislamiento frente al anfitrión. El padre, el hijo y el hilo lector siguen dependiendo del mismo sistema operativo y de sus permisos. Las pruebas no acreditan integridad física de RAM, inmunidad a corrupción arbitraria, supervivencia del padre, fallo de anfitrión o energía, ni protección ante procesos descendientes que retengan descriptores.

Las cuotas, los plazos técnicos y el tamaño de fragmento pertenecen al perfil local; no constituyen una cota universal de memoria/CPU ni un régimen de tiempo real. P11 cualifica exceso de cuerpo; las restantes guardas agregadas no adquieren cobertura exhaustiva por estas pruebas. La comparación de bytes tampoco acredita continuidad del soporte entre lecturas.

El destino observado es un archivo sintético local. La evidencia no demuestra un efecto externo, una transacción durable, recuperación productiva, presentación en GUI ni ejecución exactamente una vez. Todas las observaciones mantienen `efecto_externo=NO_ACREDITADO`; el montaje carece de despacho de acciones dependientes y de reintento automático. Un campo documental no se presenta como guarda de un despachador inexistente.

## Seguimiento y siguiente paso

[Sucesos de Calidad](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md) y su [copia de laboratorio](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/tree/lab/playground-sv-permanente/laboratorio/tareas-watson/sucesos-sv) reciben S26 revisión 17. [SUCESO_RECEPCION.json](SUCESO_RECEPCION.json) conserva el contenido de la recepción. El expediente experimental tiene [espejo en laboratorio](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/tree/lab/playground-sv-permanente/laboratorio/tareas-watson/riesgos-materiales-s26/r06/proceso03).

La edición administrativa está separada del banco: el ejecutable `sucesos` prepara y comprueba los tres archivos, conserva las otras filas y secciones y añade la revisión al historial. No modifica RETP; el incremento se identifica por su expediente y revisión de S26. Este editor no forma parte de los catorce casos ni se presenta como instrumento adversarialmente cualificado. El auxiliar Rust R08 coteja los descriptores completos de publicación, con sus límites de autoridad y concurrencia. Los programas de ensayo y ambos auxiliares son Rust; no se ejecutaron Python ni Java en estas campañas.

Corresponde preparar **T07** con barreras propias: modificación del mismo objeto, restitución A→B→A y dependencia procedente de otro corte, conservando la copia efectivamente consumida y sus referentes. Después queda la integración del recorrido completo. No se adelanta cierre de S26, R2, Bis, catálogo o S24; no se seleccionan base de datos, GUI ni plataforma productiva.
