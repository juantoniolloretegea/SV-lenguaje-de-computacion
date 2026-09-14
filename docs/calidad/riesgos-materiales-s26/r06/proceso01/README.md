# S26 R06 · PROCESO01 · Observador superviviente de proceso

**Estado de este corte: protocolo y banco precomprometidos; ninguna ejecución de los catorce casos.** La compilación preparatoria no cuenta como ejecución. Los resultados se incorporarán en un documento separado, preservando los esperados de este banco.

## Objeto, corte y sedes

Continuación de R06 T03/T04/T08 desde Lenguaje `2135c89ed3e6f9e28a77d11dc3d4dd5a8b85a7ab` y laboratorio `37fa52af4e3091aa8aea3565f0dc144e73d4654a`. Se conservan las ramas `main` y `lab/playground-sv-permanente`. LOCAL01 acreditó pánicos desenrollables dentro de un proceso; este banco contrasta una frontera entre dos procesos bajo un mismo anfitrión confiable.

Se reutilizan `ReceivedBytes`, `admit`, `AdmittedDelivery`, `Capture` y `certify`, el conjunto literal I0205-01 conservado en R05 y su oráculo. No hay cambios del núcleo, IR, gramática, perfiles fuente ni campañas anteriores. Pilares, perfiles/contratos/ensamblaje y transición secuencial se han leído completos; sus identidades se cotejan con el corte indicado. Las obligaciones específicas proceden de R06 §§3–5, R02 y las sedes R2-0/LIG allí enlazadas.

## Protocolo fijado antes del ensayo

El padre conserva en RAM las aperturas y escribe `apertura.json` antes de lanzar cada hijo. Cada caso reserva dos identificadores instrumentales, `Pnn-A` y `Pnn-B`, ligados al mismo encargo de prueba; inicialmente se selecciona A. El emisor es `observador-R06-PROCESO01`, con ámbito de una instancia de campaña. No hay reutilización entre casos, identidad global, permiso de reintento ni persistencia de la correlación acreditada. B sólo se utiliza para el control explícito de selección en P07.

Un canal local Unix de flujo transporta mensajes con magia/versionado de ocho bytes `R06P01\0\x01`, longitud de cuerpo `u32` big endian y tres campos JSON: `intento`, `clase`, `contenido`. Las clases admitidas son `captura`, `informe` y `barrera`. La completitud requiere cabecera, longitud exacta del cuerpo recibido y decodificación válida. EOF no constituye un informe. Se conserva el canal literal en `canal.bin` y el prefijo procesado.

El hijo admite el conjunto existente y copia el descriptor del objeto admitido. Envía esa captura y el recibo obtenido mediante `certify`. El padre compara la captura y la codificación del recibo con la geometría, contexto y recibo del oráculo fijado. Esto cualifica entrega documental local desde un hijo controlado; no acredita una pantalla, adquisición física ni independencia frente a un hijo arbitrariamente malicioso que falsifique una captura.

El primer informe de cada intento se conserva íntegro. Un duplicado idéntico en bytes queda identificado sin reemplazarlo; un segundo informe distinto genera conflicto, conserva ambas piezas y deja de ofrecer concordancia del intento seleccionado. Un informe de A recibido tras seleccionar B se conserva como tardío de A. B exige su propia captura e informe correlacionados; el contenido igual no sustituye la identidad del intento. Los mensajes de un intento no abierto se conservan como ajenos y no producen recepción atribuida.

Las barreras anteriores al aborto y posteriores a la escritura esperan un byte de liberación del padre. En P04 el padre termina al hijo en la barrera posterior a la escritura mediante `Child::kill`. P07 cambia la selección en una barrera antes de liberar los informes. Se observa la salida del hijo mediante `try_wait`, distinguiendo código y señal. Los códigos de salida nunca crean un recibo.

En los casos de cierre de canal, truncamiento o rechazo de cabecera, el hijo permanece esperando una orden por la dirección inversa del canal. En P10 no envía datos y espera esa orden. El padre guarda `frontera.json` **antes** de liberar o terminar administrativamente al hijo; allí la terminación debe permanecer no observada. La salida final posterior se registra aparte. El agotamiento de un plazo no se transforma en muerte de proceso ni ausencia de efecto.

## Oráculos previos

| Caso | Discriminador | Estímulo y esperado |
| --- | --- | --- |
| P01 | T03/T04/T08, positivo | Captura y recibo concordantes, EOF y salida 0; recepción de A válida. Archivo local igual a su estado inicial. |
| P02 | T03 | `abort` tras barrera anterior a captura: SIGABRT (6), sin captura ni informe; apertura conservada. |
| P03 | T03 | Captura, escritura real, barrera y `abort`: SIGABRT (6), captura conservada, informe ausente y lectura posterior igual a `efecto_local_anterior_a_terminacion`. |
| P04 | T03 | Captura y escritura; terminación desde el padre en la barrera: SIGKILL (9), captura y escritura conservadas, informe ausente. |
| P05 | T04 | Dos informes idénticos: un duplicado concordante, primer informe intacto y ninguna ejecución adicional de efecto. |
| P06 | T04 | Informe válido seguido de informe con contenido null: conflicto conservado, primer informe intacto, recepción seleccionada no conforme. |
| P07 | T04 | Seleccionar B en barrera; recibir A tardío y después captura/informe de B: A separado como tardío, B aceptado por sus propias piezas. No se afirma una nueva adquisición física. |
| P08 | T04 | Informe identificado como AJENO: registrado como ajeno, sin recibo atribuido a A ni B. |
| P09 | T08 | EOF mientras el hijo espera control: terminación no observada en la frontera; liberación posterior y salida 0, sin informe aceptable. |
| P10 | T08 | Canal abierto sin mensajes: plazo técnico agotado; terminación no observada en esa frontera. Terminación administrativa posterior, SIGKILL (9), sin recibo ni reintento. |
| P11 | T08 | Cabecera declara 16385 bytes: `cuota_mensaje`; el cuerpo no se admite. Terminación no observada hasta liberar al hijo. |
| P12 | T03/T08 | Captura completa y nueve bytes de cabecera del informe, después EOF: `mensaje_truncado`; captura conservada, sin informe. Hijo todavía sin terminación observada. |
| P13 | T03 | Captura e informe válidos seguidos de salida 7: ambas piezas conservadas y concordantes; código 7 registrado por separado. No acredita un efecto externo. |
| P14 | T08 | Magia inválida: rechazo `magia_invalida`, sin informe aceptado y sin terminación observada hasta liberar al hijo. |

Cada caso exige simultáneamente su estado de canal, código/señal final, presencia de captura, recepción seleccionada, lectura local y conservación de apertura; añade los controles específicos de la tabla. Una discrepancia se conserva y provoca salida 1. Un fallo instrumental detiene la campaña con error, sin contar casos posteriores como ejecutados. Las dos campañas utilizarán los mismos oráculos, con compilación debug y release sin `debug_assertions`.

## Recursos y cobertura

- Linux x86-64; canales Unix locales, dos procesos vivos como máximo, catorce casos secuenciales y ninguna conexión de red. El mismo ejecutable identificado actúa como padre e hijo; no hay FFI.
- Cuerpo de mensaje: 16384 bytes. Canal recibido: 65536 bytes más un byte centinela; el exceso se conserva como prefijo y no se analiza. Hasta ocho mensajes analizados por caso y dos intentos; no hay cola de reintentos.
- Archivos de entrada: 8192 bytes por pieza auxiliar; la admisión conserva sus cuotas propias. Lectura del efecto: 1024 bytes. Salida por archivo: 1048576 bytes. No se acredita una cota global de RAM o CPU.
- Conexión: máximo técnico de 3 s. Recepción: 1500 ms, excepto P10 con 150 ms. Espera final: 2 s. Hijo: límites de lectura/escritura de canal de 3 s. Los tiempos son recursos del ensayo, no primitivas semánticas ni prueba de vigencia. No se derivan garantías de tiempo real.
- Directorios nuevos bajo control del operador. El hijo sólo produce el efecto local declarado en P03/P04; sus salidas estándar se descartan para que no constituyan un segundo canal de informes. El límite de fallos se observa mediante canal y estado del proceso. No se ensayan procesos descendientes ni herencia hostil de descriptores.

La separación sobrevive a la terminación del hijo dentro de este perfil. No prueba supervivencia del padre, del anfitrión o de la energía, integridad física de RAM, durabilidad ni aislamiento frente a privilegios. La lectura del archivo inicial o modificado sólo describe ese destino local al terminar; no acredita ausencia universal de efectos. Todas las observaciones conservan `efecto_externo=NO_ACREDITADO`, sin acción dependiente ni reintento automático. Escribir o sincronizar evidencia no constituye una transacción durable.

Las cuotas agregada/de mensajes tienen guardas, pero P11 sólo cualifica el exceso de cuerpo. No se presenta este banco como cobertura exhaustiva de agotamiento de recursos, autenticación de productor o variantes de protocolo. T07 y la integración completa permanecen pendientes; cero casos globales S26 cerrados.

## Reproducción y trazabilidad

Después de publicar este precompromiso, desde esta carpeta:

```text
/opt/sv-rust-1.98.0/bin/cargo build --offline --locked --target-dir /tmp/r06p-build
/tmp/r06p-build/debug/sv_s26_r06_proceso01 ../../r05/fixture /tmp/r06p-debug
/opt/sv-rust-1.98.0/bin/cargo build --release --offline --locked --target-dir /tmp/r06p-build
/tmp/r06p-build/release/sv_s26_r06_proceso01 ../../r05/fixture /tmp/r06p-release
```

La compilación se realiza sobre el árbol del Lenguaje; el espejo conserva las mismas piezas, sin constituir otra distribución compilable por sus rutas relativas. `FUENTES.json` identifica las fuentes recibidas. Las fechas y los identificadores de Git son evidencia de procedencia del expediente, no autoridad del resultado semántico.

| Registro | Correspondencia |
| --- | --- |
| Suceso canónico | [S26](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md), su CSV y su historial mantienen la actividad única. |
| Suceso en laboratorio | [Copia de Sucesos](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/tree/lab/playground-sv-permanente/laboratorio/tareas-watson/sucesos-sv), con los mismos identificadores y revisiones. |
| Calidad del incremento | Este expediente, sus fuentes, precompromiso y evidencia posterior. [LOCAL01 / RETP-236](../local01/RESULTADOS.md) conserva su estatuto de antecedente. |
| Expediente de laboratorio | [PROCESO01](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/tree/lab/playground-sv-permanente/laboratorio/tareas-watson/riesgos-materiales-s26/r06/proceso01), con los mismos bytes experimentales. |

El seguimiento de este incremento se identifica por S26/PROCESO01 y la revisión de Sucesos. No se asigna un nuevo identificador RETP. S22/Bis y el orden catálogo/cierre → S24 conservan su estado.
