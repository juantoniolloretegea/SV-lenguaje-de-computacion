# Parte de trabajo S31: corrección acotada y trazabilidad documental

**Fecha de apertura:** 15 de septiembre de 2026.  
**Estado inicial:** pendiente.  
**Registro canónico:** [S31](../../Inventario-sv/sucesos/SUCESOS_SV.md#s31--corrección-acotada-de-recuentos-identidad-y-condiciones-de-reproducibilidad).  
**Corte de entrada del Lenguaje:** `465e3dfb18b8b0775a3ee2ffffda423acfef851b`.

## Objeto y antecedentes

Delimitar y seguir la precisión de los recuentos del mapa, la comprobación previa de identidad de los archivos utilizados, la recuperación acotada de dos archivos pendientes y la diferenciación documental de los lotes público y reservado P3. El alta precede a la ejecución de este parte. Los cotejos preliminares ya realizados constituyen antecedentes; no se atribuyen a una ejecución posterior ni se presentan como nuevos.

Rigen el registro Sucesos SV y la continuidad establecida por el Acta 001 de esta sede. El Acta 002 mantiene el contrato de leyenda R06 como candidato. Este parte no modifica la secuencia general ni acredita capacidades adicionales.

## Unidades de trabajo

| Unidad | Trabajo delimitado | Evidencia de terminación | Estado al alta |
| --- | --- | --- | --- |
| C01 | Precisar 761 referencias pendientes globales, 691 atribuidas a 32 nodos y 70 sin atribución; distinguir 101 enlaces al laboratorio entre 329 enlaces totales de cualquier medida de volumen de evidencia. | Nota de precisión con fuente y alcance de cada recuento; conservación del HTML y del paquete histórico. | pendiente |
| C02 | Cotejar en Rust 1.98.0 la identidad de cada archivo utilizado en esta intervención antes de su análisis: repositorio, corte, ruta, tamaño y blob esperado. | Relación de entradas y resultados; una copia discordante no se utiliza. Una huella SHA-256 calculada durante la intervención no se presenta como compromiso previo. | pendiente |
| C03 | Intentar recuperar exclusivamente los dos archivos pendientes identificados abajo por el acceso autorizado disponible. | Resultado por archivo: recuperado y cotejado, o recuperación no lograda con causa concreta y condición de continuación. No ejecutar sus contenidos ni repetir campañas. | pendiente |
| C04 | Contrastar exclusivamente la documentación pública sobre la coincidencia de identificadores del lote sintético y P3, y el sentido de los valores vigente:false. | Conclusión acotada: correspondencia con controles negativos o defecto documentado; condición previa al uso de P3. No consultar ni alterar material reservado. | pendiente |
| C05 | Consolidar resultados en este parte mediante una sección añadida y actualizar CSV, Markdown, historial y entrada de continuidad. | Referencias a cortes y evidencias, actuaciones concluidas, limitaciones restantes y relevo al contrato de leyenda R06. | pendiente |

## Archivos pendientes de recuperación

Repositorio de procedencia: `SV-matematica-semantica-cuaternaria`. Corte histórico: `86441ad4d375e31737dfcead0b1fd9cd52161883`. La presencia de los blobs en el corte se cotejará antes de utilizar los contenidos.

1. `laboratorio/tareas-watson/tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores/s6-trazabilidad-total/custodia/CONTROLES_CUALIFICACION.zip`; blob esperado `ffe5c4a9b86b26c2409abe4239e6355f7a3bef19`.
2. `laboratorio/tareas-watson/tuberias-ia/paridad-imagen-celula-matematica/soporte/v2/frame-original.png`; blob esperado `f123822692430d79b0b5b44d2aa4ffca4c238c94`.

La recuperación no equivale a cualificación del instrumento ni autoriza difundir contenidos privados. Si las vías disponibles no permiten obtener un archivo, se conserva el pendiente sin abrir una investigación de infraestructura.

## Exclusiones

No se modifica la presentación del HTML ni se regenera su paquete histórico. No se crean ramas ni se reorganizan directorios. No se renumeran actas, sucesos o registros técnicos. No se implementa el reconocedor de leyenda, se repiten campañas, se abren reservas, se renombran lotes, se modifican oráculos ni se implementan adaptadores P3. No se alteran los estados de S22 o S26 ni sus condiciones de cierre.

## Condiciones de suspensión

Una discrepancia real de identidad, una mezcla efectiva de material público y reservado o una evidencia que contradiga una aceptación vigente suspende la unidad afectada y exige documentar el hallazgo antes de ampliar la intervención. Una limitación de acceso ya conocida no amplía el alcance.

## Seguimiento y cierre

La apertura se registra como S31, revisión 0, en estado pendiente, con fechas de inicio y fin vacías. Antes de comenzar C01–C04 se añadirá la revisión de inicio, en ejecución, con su fecha efectiva. Toda actualización conservará las instantáneas anteriores del historial.

Al terminar se añadirá a este mismo parte una sección de cierre con una fila por C01–C05, evidencia, resultado y pendientes derivados. Se actualizará S31 a finalizado con fecha efectiva únicamente cuando haya concluido el alcance delimitado. Si un archivo no se recupera, la finalización del intento se distinguirá de la recuperación pendiente; no se atribuirá conformidad material.

Una interrupción antes de completar el alcance se registrará como pendiente, con motivo y siguiente acción. El cierre identificará las copias efectivamente actualizadas y las que conserven un corte anterior. Tras concluir esta intervención se retomará la subsanación del contrato candidato de leyenda R06. La numeración histórica se conserva íntegra.


## Inicio de ejecución · 15/09/2026, 07:57:42 UTC

S31 pasa a en ejecución, revisión 1. Se mantiene íntegro el alcance C01–C05 del alta publicada en `22233612e8f641d885f731e9ed46f1433ead1cb2`. Las unidades se resolverán mediante resultados separados y verificables. Esta anotación registra el inicio, sin anticipar resultados ni recuperación de archivos.


## Cierre de la intervención acotada · 2026-09-15T08:07:22Z

**Resultado:** finalizado en el alcance de C01–C05. El PNG pendiente se ha recuperado y cotejado; la recuperación material del ZIP S6 permanece pendiente. No se amplía la aceptación histórica de ningún instrumento ni se declara compatibilidad integral de P3.

| Unidad | Resultado final | Evidencia y límite |
| --- | --- | --- |
| C01 | Precisión documental concluida. | 761 referencias pendientes globales = 691 atribuidas a 32 nodos + 70 sin atribución a nodo, localizadas en 12 documentos. 101 de 329 enlaces apuntan al repositorio del laboratorio. Son recuentos de referencias y enlaces, no de experimentos ni de volumen de evidencia. |
| C02 | Identidad previa comprobada para las entradas utilizadas. | Rust 1.98.0 coteja el blob del HTML antes de leer su contenido, el PNG contra el árbol histórico y once archivos públicos contra el corte de entrada. Los miembros del ZIP embebido proceden del HTML identificado y se comprueban mediante CRC. No se ha producido una discrepancia de identidad en esas entradas. |
| C03 | Intento acotado concluido; un archivo recuperado y otro pendiente. | PNG: 255245 bytes, blob concordante. ZIP S6: 1811714 bytes declarados en el árbol; no se obtuvieron bytes completos utilizables mediante las vías examinadas. No se ejecutaron contenidos ni campañas. |
| C04 | Diferenciación documental y condición previa de P3 precisadas. | El lote sintético es público y conserva su identidad propia. Los tres valores históricos de vigencia negativa corresponden a esperados públicos. El cuarto del sucesor S2 tiene declaración causal separada. Se conservan la reserva cerrada y la decisión de asociación pendiente. |
| C05 | Parte de cierre y seguimiento consolidados. | Fuente completo, manifiesto de entradas y salida del cotejo adjuntos en esta sede; CSV, Markdown, historial y Léame primero actualizados. S31 finalizado, revisión 2. S22 y S26 conservan su estado y sus condiciones. |

### Precisión del mapa y su paquete histórico

HTML: 3703604 bytes; blob `24e25dbf1255ddf25d5f1e7eefa8d89d4b12ed3e`; SHA-256 `bf57da72c7d535aa4b258b2c80232bc8954f91eb28aa4917739c3db4de7404c0`. ZIP embebido: 2616129 bytes; SHA-256 `6e4eec4bdae325eeba222635ee5954e0edddc0c5c2e6fae70a47a6bc6bd11e04`. Ambos se conservan sin modificación.

La frase del README histórico que reúne los 32 nodos y las 761 apariciones debe leerse con esta precisión: **el total global es 761; el subconjunto atribuido a los 32 nodos suma 691; las otras 70 referencias se conservan en el listado global sin atribución a nodo**. El listado del cotejo identifica los 12 documentos correspondientes. No se trasladan esas referencias a nodos por inferencia.

Los 101 enlaces al laboratorio representan aproximadamente el 30,7 % de los 329 enlaces. Esa proporción no mide volumen, relevancia ni suficiencia de evidencia. Los 228 enlaces restantes no se declaran verificados por ser públicos. No se han vuelto a cotejar los 6085 archivos de la auditoría histórica.

El JSON de la página y `salida/NODOS_VERIFICADOS.json` del ZIP **no son idénticos campo a campo**. La salida contiene 230 enlaces; la página incorpora 99 adicionales distribuidos en 68 nodos y no retira ninguno. Los demás campos compartidos cotejados concuerdan, y los 144 ámbitos `paths` de la página concuerdan con `entrada/NODOS.json`. Esta distinción precisa la composición de la página; no altera los recuentos ni los dictámenes de recuperación. No se afirma que los 99 destinos adicionales hayan sido descargados.

### Recuperación material acotada

El árbol histórico del laboratorio identifica los dos archivos y sus tamaños. La lectura del PNG mediante transporte base64 produjo bytes completos que fueron decodificados y cotejados en Rust antes de utilizarse.

- `frame-original.png`: 255245 bytes; blob `f123822692430d79b0b5b44d2aa4ffca4c238c94`; SHA-256 observado `4ce891a55f68884b5dd7652b7cfb39cde3c745977a4cb4e8936b99269e0d66af`. Se acredita identidad de bytes, no contenido visual, semántica, cualificación de captor ni suficiencia de soporte.
- `CONTROLES_CUALIFICACION.zip`: blob esperado `ffe5c4a9b86b26c2409abe4239e6355f7a3bef19`, tamaño declarado 1811714 bytes. La lectura textual y la lectura de blob no admitieron la codificación binaria; la solicitud de contenido base64 devolvió metadatos con contenido vacío. **No se calcula ni se atribuye una SHA-256 observada del ZIP.** La recuperación queda pendiente de una entrega completa obtenida con acceso autorizado y cotejada con ese blob antes de usarla.

Las fuentes privadas conservan su ubicación y acceso; este cierre no publica sus contenidos ni crea una sede pública sustitutiva. La coincidencia del PNG no recalifica automáticamente el nodo histórico BIS-PLAN ni el mapa congelado.

### Identidad de lotes y vigencia pública

Corte documental: Lenguaje `46a620864e77543140e3d3290be7c33bea71766b`.

El lote sintético público contiene 24 posiciones bajo `IE004-P3-A/1`, con identificadores `P3-01…P3-24`; tiene 7773 bytes y SHA-256 `88bda419f98d75c10be7cf91acc849e852904495a9e6deb4403a8429a1c2990a`. Su procedencia lo identifica como reutilización de diez entradas públicas. El compromiso de la reserva identifica otro archivo, de 6871 bytes, con SHA-256 `65ac9f7a8c08528009a473065de3bdf989d7852cb15561036c9bba61c79780d7`. No se ha leído ese archivo reservado ni comparado sus preguntas.

La versión de formato y los identificadores de posición **no bastan para identificar el lote**. Toda referencia de seguimiento o admisión debe incluir su naturaleza pública o reservada, procedencia, corte, ruta y huella previamente fijada. El contrato público del adaptador ya exige cotejar todos los bytes con la huella confiable antes de decodificar; la huella suministrada por una entrada no confiable no acredita esa confianza. No se renombra ni altera retrospectivamente ningún lote. La diferenciación nominal pendiente se conserva para la preparación de P3.

En el montaje público original, `P3-04`, `P3-14` y `P3-24` heredan A04 y `vigente:false`; esos valores coinciden con los cuerpos esperados públicos. No constituyen por sí mismos un defecto. El montaje sucesor S2 añade `P3-11` con vigencia negativa, declarado en `DECLARACION_VIGENCIA.json` como cambio sintético de true a false y documentado en el resultado causal de S2. No se utiliza el esperado histórico positivo de P3-11 como esperado de esa sucesión.

La decisión preparada conserva `PREPARADA_NO_EMITIDA` y `captura_habilitada:false`. La asociación /2–/3, el alcance de las expectativas y la admisión de auxiliares siguen sujetos a sus condiciones previas. Este cotejo documental no abre P3 ni determina su compatibilidad integral.

### Evidencia reproducible y alcance del cierre

- [Fuente completo del cotejo](COTEJO_S31.rs): 41999 bytes; SHA-256 `faa9f6c36ea5b7953bf1fa4462322eef02ea5674ba1be4e882a5fa033fef3669`.
- [Manifiesto de las once entradas públicas](ENTRADAS_S31.tsv): 1478 bytes; SHA-256 `01ddf0e6dd9ad1dc7df1cf14e4cb442071dd5253e09f2135c7689cac06ee4fe5`.
- [Salida íntegra del cotejo](COTEJO_S31_SALIDA.txt): 4424 bytes; SHA-256 `0c6889ab294c899e285d4ee45240475c08c80961d7e555647df62e84d51c6e25`.

El fuente reúne los lectores y funciones de huella del paquete histórico, y las comprobaciones acotadas de S31. Se compiló y ejecutó con Rust 1.98.0; `RC_COMPILE=0`, `RC_RUN=0`. La dependencia de descompresión es flate2 1.1.9, construida con el Cargo.lock y las dependencias locales del paquete embebido, en modo locked/offline. No se presenta como validación criptográfica independiente. No se ejecutaron los binarios de los expedientes ni las campañas históricas.

Para reproducir, obtener el HTML por su corte/blob, las once entradas públicas desde el corte indicado y el PNG por acceso autorizado desde su corte histórico. El TSV relaciona nombres auxiliares con rutas canónicas y blobs. Guardar las entradas con esos nombres, el PNG codificado en base64 como `sv_s31_frame.b64` y el HTML como `MAPA.html`, junto al manifiesto. Compilar el fuente con Rust 1.98.0, enlazando la biblioteca flate2 construida con las dependencias fijadas; ejecutar `cotejo_s31 DIRECTORIO_ENTRADAS`. El programa coteja identidades antes de analizar y genera copias auxiliares de cuatro miembros del ZIP. No requiere el ZIP S6, cuya recuperación no se acredita.

El registro actualizado corresponde a la sede canónica del Lenguaje; las copias de laboratorio y los paquetes históricos mantienen sus cortes anteriores, expresamente identificados. No se afirma sincronización de esos espejos. No se adoptó una nueva realización material ni se renumeraron registros técnicos.

**Relevo:** retomar la subsanación del contrato candidato de leyenda R06. La recuperación del ZIP S6 y las condiciones previas de P3 quedan identificadas y no se convierten en una reapertura general de campañas.


## Recepción posterior de la evidencia S6 · 15 de septiembre de 2026

**Naturaleza:** incorporación de evidencia al seguimiento C03 de S31, que conserva el estado finalizado y la fecha de cierre de la revisión 2. No constituye una reanudación de C01–C05 ni una nueva campaña. El cierre publicado en `4e8ab90885625fabac76addc8d3eba450dafbe2d` se conserva íntegro: la ausencia de bytes completos descrita allí corresponde a aquel corte.

### Identidad recibida y comprobada

La entrega completa recibida por canal privado se cotejó antes de descomprimirla con el tamaño y el blob Git previamente fijados:

- Procedencia: `SV-matematica-semantica-cuaternaria`, corte `86441ad4d375e31737dfcead0b1fd9cd52161883`.
- Ruta: `laboratorio/tareas-watson/tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores/s6-trazabilidad-total/custodia/CONTROLES_CUALIFICACION.zip`.
- Tamaño observado: **1811714 bytes**, coincidente.
- Blob Git observado: `ffe5c4a9b86b26c2409abe4239e6355f7a3bef19`, coincidente.
- SHA-256 observada en esta recepción: `48e2c1e5acc384687dcd346613a945bf18798a105c72b94b6adb72be23115e1a`. No se atribuye esta medición al cierre anterior.

### Comprobación y resultado

Rust 1.98.0 (`88d9e12ae 2026-08-18`), con flate2 1.1.9 y las dependencias fijadas del cotejo S31. Compilación y ejecución con código 0. Se comprobaron las cabeceras locales y centrales, límites, nombres únicos, correspondencia exacta de entradas, descompresión en memoria y CRC de **487/487 entradas**. Tamaño descomprimido total: **13216586 bytes**.

El archivo contiene `controles/0000.json` a `controles/0481.json` y cinco auxiliares: `CUALIFICACION_COMPLETA.json`, `CLI-referencia.json`, `CLI-fallo.json`, `INCIDENCIA_PREPARACION.txt` y `cualificar.py`. No se escribieron los miembros en disco, no se ejecutaron programas contenidos y no se repitieron los 482 controles. La integridad del contenedor no acredita por sí sola la corrección funcional de esos controles.

**Se resuelve el pendiente de recuperación e identidad del ZIP S6 señalado en C03.** El PNG ya recibido conserva su comprobación anterior. Esta recepción no amplía la cualificación histórica del instrumento, no altera P3 ni modifica el rumbo de S22/S26. El siguiente trabajo sigue siendo la subsanación del contrato candidato de leyenda R06.

### Evidencia de la recepción

- [Fuente completo](COTEJO_S31_RECEPCION_ZIP_S6.rs): 10738 bytes; SHA-256 `1464bda40b9a0038a5d950ff61d16b1e177d6b98a02bb6d407226335ad19efb8`.
- [Salida y comandos de reproducción](COTEJO_S31_RECEPCION_ZIP_S6_SALIDA.txt): 719 bytes; SHA-256 `af1dfa7172a42a3ad725dcb8c2a09c04c85f236f069386bf9f3a2eaa5c3694a8`.

Las funciones de huella proceden del cotejo S31; no se presenta una validación criptográfica independiente. El ZIP conserva su sede privada: esta incorporación pública contiene únicamente el cotejador, sus resultados de integridad y la referencia de procedencia. La reproducción requiere acceso autorizado al archivo original. El código reutiliza las dependencias fijadas en las instrucciones de reproducción anteriores.

Se añade la revisión 3 al historial de S31, conservando las revisiones 0–2. Los demás sucesos, el HTML, los lotes y los registros RETP no cambian. Los espejos históricos conservan sus cortes; esta recepción se registra en la sede canónica del Lenguaje.
